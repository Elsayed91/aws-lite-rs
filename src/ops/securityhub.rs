//! Operation contracts for the AWS Security Hub API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/securityhub.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::securityhub::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the AWS Security Hub API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::securityhub::SecurityhubClient`] instead.
pub struct SecurityhubOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> SecurityhubOps<'a> {
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
        "https://securityhub.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Returns details about the Hub resource in your account, including the HubArn and the
    /// time when you enabled Security Hub.
    ///
    /// **AWS API**: `GET /accounts`
    ///
    /// # Query Parameters
    /// - `HubArn` —
    ///
    /// # Response
    /// [`DescribeHubResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_hub(&self, hub_arn: &str) -> Result<DescribeHubResponse> {
        let url = format!("{}/accounts", self.base_url(),);
        let url = crate::append_query_params(url, &[("HubArn", hub_arn)]);
        let response = self.client.get_json(&url, "securityhub").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_hub response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_hub response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_hub() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/accounts?HubArn=test-HubArn")
            .returning_json(serde_json::to_value(DescribeHubResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = SecurityhubOps::new(&client);

        let result = ops.describe_hub("test-HubArn").await;
        assert!(result.is_ok());
    }
}
