//! Operation contracts for the AWS IAM Access Analyzer API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/accessanalyzer.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::accessanalyzer::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the AWS IAM Access Analyzer API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::accessanalyzer::AccessanalyzerClient`] instead.
pub struct AccessanalyzerOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> AccessanalyzerOps<'a> {
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
        "https://access-analyzer.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Retrieves a list of analyzers.
    ///
    /// **AWS API**: `GET /analyzer`
    ///
    /// # Query Parameters
    /// - `nextToken` —
    /// - `maxResults` —
    /// - `type` —
    ///
    /// # Response
    /// [`ListAnalyzersResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_analyzers(
        &self,
        next_token: &str,
        max_results: &str,
        r#type: &str,
    ) -> Result<ListAnalyzersResponse> {
        let url = format!("{}/analyzer", self.base_url(),);
        let url = crate::append_query_params(
            url,
            &[
                ("nextToken", next_token),
                ("maxResults", max_results),
                ("type", r#type),
            ],
        );
        let response = self.client.get_json(&url, "access-analyzer").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_analyzers response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_analyzers response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_analyzers() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/analyzer?nextToken=test-nextToken&maxResults=test-maxResults&type=test-type",
        )
        .returning_json(serde_json::to_value(ListAnalyzersResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = AccessanalyzerOps::new(&client);

        let result = ops
            .list_analyzers("test-nextToken", "test-maxResults", "test-type")
            .await;
        assert!(result.is_ok());
    }
}
