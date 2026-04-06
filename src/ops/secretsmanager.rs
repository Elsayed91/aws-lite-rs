//! Operation contracts for the AWS Secrets Manager API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/secretsmanager.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::secretsmanager::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the AWS Secrets Manager API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::secretsmanager::SecretsmanagerClient`] instead.
pub struct SecretsmanagerOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> SecretsmanagerOps<'a> {
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
        "https://secretsmanager.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Lists the secrets that are stored by Secrets Manager in the current account and Region.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListSecretsRequest`]
    ///
    /// # Response
    /// [`ListSecretsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_secrets(
        &self,
        body: &ListSecretsRequest,
    ) -> Result<ListSecretsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize list_secrets request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "secretsmanager",
                "secretsmanager.ListSecrets",
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
                    message: format!("Failed to read list_secrets response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_secrets response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a secret and all of its versions.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteSecretRequest`]
    ///
    /// # Response
    /// [`DeleteSecretResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_secret(
        &self,
        body: &DeleteSecretRequest,
    ) -> Result<DeleteSecretResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize delete_secret request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "secretsmanager",
                "secretsmanager.DeleteSecret",
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
                    message: format!("Failed to read delete_secret response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse delete_secret response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Configures and starts the asynchronous process of rotating the secret.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`RotateSecretRequest`]
    ///
    /// # Response
    /// [`RotateSecretResponse`]
    #[allow(dead_code)]
    pub(crate) async fn rotate_secret(
        &self,
        body: &RotateSecretRequest,
    ) -> Result<RotateSecretResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize rotate_secret request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "secretsmanager",
                "secretsmanager.RotateSecret",
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
                    message: format!("Failed to read rotate_secret response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse rotate_secret response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_secrets() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(ListSecretsResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = SecretsmanagerOps::new(&client);

        let body = ListSecretsRequest::fixture();
        let result = ops.list_secrets(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_secret() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DeleteSecretResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = SecretsmanagerOps::new(&client);

        let body = DeleteSecretRequest::fixture();
        let result = ops.delete_secret(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_rotate_secret() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(RotateSecretResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = SecretsmanagerOps::new(&client);

        let body = RotateSecretRequest::fixture();
        let result = ops.rotate_secret(&body).await;
        assert!(result.is_ok());
    }
}
