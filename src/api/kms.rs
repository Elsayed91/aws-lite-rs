//! AWS Key Management Service API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::kms::KmsOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::kms::KmsOps,
    types::kms::{
        DescribeKeyRequest, DescribeKeyResponse, EnableKeyRotationRequest,
        GetKeyRotationStatusRequest, GetKeyRotationStatusResponse, ListKeysRequest,
        ListKeysResponse,
    },
};

/// Client for the AWS Key Management Service API
pub struct KmsClient<'a> {
    ops: KmsOps<'a>,
}

impl<'a> KmsClient<'a> {
    /// Create a new AWS Key Management Service API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: KmsOps::new(client),
        }
    }

    /// Lists the key ARNs of all KMS keys in the current account and region.
    pub async fn list_keys(&self, body: &ListKeysRequest) -> Result<ListKeysResponse> {
        self.ops.list_keys(body).await
    }

    /// Provides detailed information about a KMS key.
    pub async fn describe_key(&self, body: &DescribeKeyRequest) -> Result<DescribeKeyResponse> {
        self.ops.describe_key(body).await
    }

    /// Gets a Boolean value that indicates whether automatic rotation of the key material is enabled for the specified KMS key.
    pub async fn get_key_rotation_status(
        &self,
        body: &GetKeyRotationStatusRequest,
    ) -> Result<GetKeyRotationStatusResponse> {
        self.ops.get_key_rotation_status(body).await
    }

    /// Enables automatic rotation of the key material for the specified symmetric encryption KMS key.
    pub async fn enable_key_rotation(&self, body: &EnableKeyRotationRequest) -> Result<()> {
        self.ops.enable_key_rotation(body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn list_keys_returns_key_entries() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "Keys": [
                {
                    "KeyId": "abc-123",
                    "KeyArn": "arn:aws:kms:us-east-1:123456789012:key/abc-123"
                },
                {
                    "KeyId": "def-456",
                    "KeyArn": "arn:aws:kms:us-east-1:123456789012:key/def-456"
                }
            ],
            "NextMarker": null,
            "Truncated": false
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .kms()
            .list_keys(&ListKeysRequest::default())
            .await
            .unwrap();
        assert_eq!(result.keys.len(), 2);
        assert_eq!(result.keys[0].key_id.as_deref(), Some("abc-123"));
        assert_eq!(
            result.keys[0].key_arn.as_deref(),
            Some("arn:aws:kms:us-east-1:123456789012:key/abc-123")
        );
        assert_eq!(result.truncated, Some(false));
    }

    #[tokio::test]
    async fn describe_key_returns_metadata() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "KeyMetadata": {
                "KeyId": "abc-123",
                "Arn": "arn:aws:kms:us-east-1:123456789012:key/abc-123",
                "CreationDate": 1700000000.0,
                "Enabled": true,
                "Description": "test key",
                "KeyUsage": "ENCRYPT_DECRYPT",
                "KeyState": "Enabled",
                "KeyManager": "CUSTOMER",
                "KeySpec": "SYMMETRIC_DEFAULT",
                "MultiRegion": false
            }
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .kms()
            .describe_key(&DescribeKeyRequest {
                key_id: "abc-123".to_string(),
            })
            .await
            .unwrap();
        let meta = result.key_metadata.unwrap();
        assert_eq!(meta.key_id.as_str(), "abc-123");
        assert_eq!(
            meta.arn.as_deref(),
            Some("arn:aws:kms:us-east-1:123456789012:key/abc-123")
        );
        assert_eq!(meta.enabled, Some(true));
        assert_eq!(meta.key_state.as_deref(), Some("Enabled"));
        assert_eq!(meta.key_manager.as_deref(), Some("CUSTOMER"));
        assert_eq!(meta.creation_date, Some(1700000000.0));
    }

    #[tokio::test]
    async fn get_key_rotation_status_returns_enabled() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "KeyRotationEnabled": true,
            "KeyId": "abc-123",
            "RotationPeriodInDays": 365,
            "NextRotationDate": 1731536000.0
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .kms()
            .get_key_rotation_status(&GetKeyRotationStatusRequest {
                key_id: "abc-123".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(result.key_rotation_enabled, Some(true));
        assert_eq!(result.key_id.as_deref(), Some("abc-123"));
        assert_eq!(result.rotation_period_in_days, Some(365));
        assert_eq!(result.next_rotation_date, Some(1731536000.0));
    }

    #[tokio::test]
    async fn enable_key_rotation_returns_unit() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({}));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .kms()
            .enable_key_rotation(&EnableKeyRotationRequest {
                key_id: "abc-123".to_string(),
                ..Default::default()
            })
            .await;
        assert!(result.is_ok(), "EnableKeyRotation should return Ok(())");
    }
}
