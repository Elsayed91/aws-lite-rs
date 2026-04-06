//! Operation contracts for the AWS Key Management Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/kms.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::kms::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the AWS Key Management Service API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::kms::KmsClient`] instead.
pub struct KmsOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> KmsOps<'a> {
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
        "https://kms.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Lists the key ARNs of all KMS keys in the current account and region.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListKeysRequest`]
    ///
    /// # Response
    /// [`ListKeysResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_keys(&self, body: &ListKeysRequest) -> Result<ListKeysResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize list_keys request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(&url, "kms", "TrentService.ListKeys", "1.1", &body_bytes)
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_keys response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_keys response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Provides detailed information about a KMS key.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeKeyRequest`]
    ///
    /// # Response
    /// [`DescribeKeyResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_key(
        &self,
        body: &DescribeKeyRequest,
    ) -> Result<DescribeKeyResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_key request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(&url, "kms", "TrentService.DescribeKey", "1.1", &body_bytes)
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_key response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_key response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets a Boolean value that indicates whether automatic rotation of the key material is
    /// enabled for the specified KMS key.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetKeyRotationStatusRequest`]
    ///
    /// # Response
    /// [`GetKeyRotationStatusResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_key_rotation_status(
        &self,
        body: &GetKeyRotationStatusRequest,
    ) -> Result<GetKeyRotationStatusResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize get_key_rotation_status request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "kms",
                "TrentService.GetKeyRotationStatus",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_key_rotation_status response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse get_key_rotation_status response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Enables automatic rotation of the key material for the specified symmetric encryption
    /// KMS key.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`EnableKeyRotationRequest`]
    #[allow(dead_code)]
    pub(crate) async fn enable_key_rotation(&self, body: &EnableKeyRotationRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize enable_key_rotation request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "kms",
                "TrentService.EnableKeyRotation",
                "1.1",
                &body_bytes,
            )
            .await?;
        response.error_for_status("json").await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_keys() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(ListKeysResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = KmsOps::new(&client);

        let body = ListKeysRequest::fixture();
        let result = ops.list_keys(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeKeyResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = KmsOps::new(&client);

        let body = DescribeKeyRequest::fixture();
        let result = ops.describe_key(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_key_rotation_status() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(GetKeyRotationStatusResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = KmsOps::new(&client);

        let body = GetKeyRotationStatusRequest::fixture();
        let result = ops.get_key_rotation_status(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enable_key_rotation() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = KmsOps::new(&client);

        let body = EnableKeyRotationRequest::fixture();
        let result = ops.enable_key_rotation(&body).await;
        assert!(result.is_ok());
    }
}
