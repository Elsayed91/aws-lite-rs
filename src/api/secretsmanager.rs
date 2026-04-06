//! AWS Secrets Manager API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::secretsmanager::SecretsmanagerOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::secretsmanager::SecretsmanagerOps,
    types::secretsmanager::{
        DeleteSecretRequest, DeleteSecretResponse, ListSecretsRequest, ListSecretsResponse,
        RotateSecretRequest, RotateSecretResponse,
    },
};

/// Client for the AWS Secrets Manager API
pub struct SecretsmanagerClient<'a> {
    ops: SecretsmanagerOps<'a>,
}

impl<'a> SecretsmanagerClient<'a> {
    /// Create a new AWS Secrets Manager API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: SecretsmanagerOps::new(client),
        }
    }

    /// Lists the secrets that are stored by Secrets Manager in the current account and Region.
    pub async fn list_secrets(&self, body: &ListSecretsRequest) -> Result<ListSecretsResponse> {
        self.ops.list_secrets(body).await
    }

    /// Deletes a secret and all of its versions.
    pub async fn delete_secret(&self, body: &DeleteSecretRequest) -> Result<DeleteSecretResponse> {
        self.ops.delete_secret(body).await
    }

    /// Configures and starts the asynchronous process of rotating the secret.
    pub async fn rotate_secret(&self, body: &RotateSecretRequest) -> Result<RotateSecretResponse> {
        self.ops.rotate_secret(body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn list_secrets_returns_secret_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "SecretList": [
                {
                    "ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret-AbCd",
                    "Name": "my-secret",
                    "Description": "My test secret",
                    "RotationEnabled": false,
                    "CreatedDate": 1700000000.0,
                    "LastChangedDate": 1700001000.0
                }
            ],
            "NextToken": null
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .secretsmanager()
            .list_secrets(&ListSecretsRequest::default())
            .await
            .unwrap();
        assert_eq!(result.secret_list.len(), 1);
        let entry = &result.secret_list[0];
        assert_eq!(
            entry.arn.as_deref(),
            Some("arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret-AbCd")
        );
        assert_eq!(entry.name.as_deref(), Some("my-secret"));
        assert_eq!(entry.rotation_enabled, Some(false));
        assert_eq!(entry.created_date, Some(1700000000.0));
    }

    #[tokio::test]
    async fn delete_secret_returns_arn_and_name() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret-AbCd",
            "Name": "my-secret",
            "DeletionDate": 1700086400.0
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .secretsmanager()
            .delete_secret(&DeleteSecretRequest {
                secret_id: "my-secret".to_string(),
                force_delete_without_recovery: Some(true),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(
            result.arn.as_deref(),
            Some("arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret-AbCd")
        );
        assert_eq!(result.name.as_deref(), Some("my-secret"));
        assert_eq!(result.deletion_date, Some(1700086400.0));
    }

    #[tokio::test]
    async fn rotate_secret_returns_arn_and_version() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret-AbCd",
            "Name": "my-secret",
            "VersionId": "abc-version-123"
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .secretsmanager()
            .rotate_secret(&RotateSecretRequest {
                secret_id: "my-secret".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(
            result.arn.as_deref(),
            Some("arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret-AbCd")
        );
        assert_eq!(result.name.as_deref(), Some("my-secret"));
        assert_eq!(result.version_id.as_deref(), Some("abc-version-123"));
    }
}
