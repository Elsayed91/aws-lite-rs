//! Operation contracts for the AWS Cost Explorer API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/ce.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::ce::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the AWS Cost Explorer API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::ce::CeClient`] instead.
pub struct CeOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> CeOps<'a> {
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
        "https://ce.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Retrieves cost and usage metrics for your account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetCostAndUsageRequest`]
    ///
    /// # Response
    /// [`GetCostAndUsageResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_cost_and_usage(
        &self,
        body: &GetCostAndUsageRequest,
    ) -> Result<GetCostAndUsageResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize get_cost_and_usage request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ce",
                "AWSInsightsIndexService.GetCostAndUsage",
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
                    message: format!("Failed to read get_cost_and_usage response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse get_cost_and_usage response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_cost_and_usage() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(GetCostAndUsageResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CeOps::new(&client);

        let body = GetCostAndUsageRequest::fixture();
        let result = ops.get_cost_and_usage(&body).await;
        assert!(result.is_ok());
    }
}
