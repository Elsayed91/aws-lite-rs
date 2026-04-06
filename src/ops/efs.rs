//! Operation contracts for the Amazon Elastic File System API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/efs.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::efs::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon Elastic File System API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::efs::EfsClient`] instead.
pub struct EfsOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> EfsOps<'a> {
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> String {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/').to_string();
            }
        }
        "https://elasticfilesystem.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Returns the description of a specific Amazon EFS file system if either the file system
    /// CreationToken or the FileSystemId is provided. Otherwise, it returns descriptions of all
    /// file systems owned by the caller's Amazon Web Services account in the Amazon Web
    /// Services Region of the endpoint that you're calling.
    ///
    /// **AWS API**: `GET /2015-02-01/file-systems`
    ///
    /// # Query Parameters
    /// - `MaxItems` —
    /// - `Marker` —
    /// - `CreationToken` —
    /// - `FileSystemId` —
    ///
    /// # Response
    /// [`DescribeFileSystemsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_file_systems(
        &self,
        max_items: &str,
        marker: &str,
        creation_token: &str,
        file_system_id: &str,
    ) -> Result<DescribeFileSystemsResponse> {
        let url = format!("{}/2015-02-01/file-systems", self.base_url(),);
        let url = crate::append_query_params(
            url,
            &[
                ("MaxItems", max_items),
                ("Marker", marker),
                ("CreationToken", creation_token),
                ("FileSystemId", file_system_id),
            ],
        );
        let response = self.client.get_json(&url, "elasticfilesystem").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_file_systems response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_file_systems response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_file_systems() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/2015-02-01/file-systems?MaxItems=test-MaxItems&Marker=test-Marker&CreationToken=test-CreationToken&FileSystemId=test-FileSystemId")
            .returning_json(serde_json::to_value(DescribeFileSystemsResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EfsOps::new(&client);

        let result = ops
            .describe_file_systems(
                "test-MaxItems",
                "test-Marker",
                "test-CreationToken",
                "test-FileSystemId",
            )
            .await;
        assert!(result.is_ok());
    }
}
