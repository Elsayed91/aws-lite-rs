//! AWS Elastic File System (EFS) API client.
//!
//! Thin wrapper over generated ops.
//!
//! Needed by AWS CIS benchmark checks:
//!   - CIS 3.3.1 (aws_efs_encryption): verify that all EFS file systems have
//!     encryption at rest enabled (`Encrypted: true`).

use crate::{
    AwsHttpClient, Result,
    ops::efs::EfsOps,
    types::efs::{DescribeFileSystemsResponse, FileSystemDescription},
};

/// Client for the Amazon Elastic File System API.
pub struct EfsClient<'a> {
    ops: EfsOps<'a>,
}

impl<'a> EfsClient<'a> {
    /// Create a new EFS client.
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: EfsOps::new(client),
        }
    }

    // ── File Systems ───────────────────────────────────────────────────

    /// Return the first page of EFS file system descriptions.
    ///
    /// Pass `marker` from a previous response to paginate.
    pub async fn describe_file_systems(
        &self,
        marker: Option<&str>,
    ) -> Result<DescribeFileSystemsResponse> {
        self.ops
            .describe_file_systems("", marker.unwrap_or(""), "", "")
            .await
    }

    /// Return all EFS file systems in the current region, following pagination.
    ///
    /// CIS 3.3.1: every file system should have `Encrypted == true`.
    pub async fn list_all_file_systems(&self) -> Result<Vec<FileSystemDescription>> {
        let mut all = Vec::new();
        let mut marker: Option<String> = None;

        loop {
            let resp = self
                .ops
                .describe_file_systems("", marker.as_deref().unwrap_or(""), "", "")
                .await?;
            all.extend(resp.file_systems);
            match resp.next_marker {
                Some(m) if !m.is_empty() => marker = Some(m),
                _ => break,
            }
        }

        Ok(all)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_describe_file_systems_returns_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/2015-02-01/file-systems")
            .returning_json(json!({
                "FileSystems": [
                    {
                        "OwnerId": "123456789012",
                        "FileSystemId": "fs-0123456789abcdef0",
                        "FileSystemArn": "arn:aws:elasticfilesystem:us-east-1:123456789012:file-system/fs-0123456789abcdef0",
                        "CreationTime": "2024-01-15T10:00:00Z",
                        "LifeCycleState": "available",
                        "Name": "my-efs",
                        "NumberOfMountTargets": 2,
                        "PerformanceMode": "generalPurpose",
                        "Encrypted": true,
                        "KmsKeyId": "arn:aws:kms:us-east-1:123456789012:key/mrk-abc123",
                        "ThroughputMode": "bursting",
                        "Tags": [{"Key": "Name", "Value": "my-efs"}]
                    }
                ]
            }))
            .times(1);

        let client = crate::AwsHttpClient::from_mock(mock);
        let efs = client.efs();
        let resp = efs.describe_file_systems(None).await.unwrap();
        assert_eq!(resp.file_systems.len(), 1);
        let fs = &resp.file_systems[0];
        assert_eq!(fs.file_system_id, "fs-0123456789abcdef0");
        assert_eq!(fs.encrypted, Some(true));
        assert_eq!(fs.life_cycle_state, "available");
        assert_eq!(fs.tags.len(), 1);
        assert_eq!(fs.tags[0].key, "Name");
    }

    #[tokio::test]
    async fn test_list_all_file_systems_paginates() {
        let mut mock = crate::MockClient::new();
        // Both calls start with "/2015-02-01/file-systems" — use a sequence so the
        // second call (with Marker=page2) gets the second response.
        mock.expect_get("/2015-02-01/file-systems")
            .returning_json_sequence(vec![
                json!({
                    "FileSystems": [{
                        "OwnerId": "123456789012",
                        "FileSystemId": "fs-aaa",
                        "CreationTime": "2024-01-15T10:00:00Z",
                        "LifeCycleState": "available",
                        "NumberOfMountTargets": 0,
                        "PerformanceMode": "generalPurpose",
                        "Encrypted": false,
                        "Tags": []
                    }],
                    "NextMarker": "page2"
                }),
                json!({
                    "FileSystems": [{
                        "OwnerId": "123456789012",
                        "FileSystemId": "fs-bbb",
                        "CreationTime": "2024-01-15T11:00:00Z",
                        "LifeCycleState": "available",
                        "NumberOfMountTargets": 1,
                        "PerformanceMode": "generalPurpose",
                        "Encrypted": true,
                        "Tags": []
                    }]
                }),
            ])
            .times(2);

        let client = crate::AwsHttpClient::from_mock(mock);
        let all = client.efs().list_all_file_systems().await.unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].file_system_id, "fs-aaa");
        assert_eq!(all[0].encrypted, Some(false));
        assert_eq!(all[1].file_system_id, "fs-bbb");
        assert_eq!(all[1].encrypted, Some(true));
    }

    #[tokio::test]
    async fn test_describe_file_systems_empty() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/2015-02-01/file-systems")
            .returning_json(json!({"FileSystems": []}))
            .times(1);

        let client = crate::AwsHttpClient::from_mock(mock);
        let resp = client.efs().describe_file_systems(None).await.unwrap();
        assert!(resp.file_systems.is_empty());
    }
}
