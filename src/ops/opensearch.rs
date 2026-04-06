//! Operation contracts for the Amazon OpenSearch Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/opensearch.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::opensearch::*;
use crate::{AwsHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Amazon OpenSearch Service API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::opensearch::OpensearchClient`] instead.
pub struct OpensearchOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> OpensearchOps<'a> {
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
        "https://es.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Returns the names of all Amazon OpenSearch Service domains owned by the current user in
    /// the active Region.
    ///
    /// **AWS API**: `GET /2021-01-01/domain`
    ///
    /// # Query Parameters
    /// - `engineType` —
    ///
    /// # Response
    /// [`ListDomainNamesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_domain_names(
        &self,
        engine_type: &str,
    ) -> Result<ListDomainNamesResponse> {
        let url = format!("{}/2021-01-01/domain", self.base_url(),);
        let url = crate::append_query_params(url, &[("engineType", engine_type)]);
        let response = self.client.get_json(&url, "es").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_domain_names response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_domain_names response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Returns domain configuration information about the specified Amazon OpenSearch Service
    /// domain.
    ///
    /// **AWS API**: `GET /2021-01-01/opensearch/domain/{DomainName}`
    ///
    /// # Path Parameters
    /// - `DomainName` —  *(required)*
    ///
    /// # Response
    /// [`DescribeDomainResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_domain(
        &self,
        domain_name: &str,
    ) -> Result<DescribeDomainResponse> {
        let url = format!(
            "{}/2021-01-01/opensearch/domain/{}",
            self.base_url(),
            encode(domain_name),
        );
        let response = self.client.get_json(&url, "es").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_domain response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_domain response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes an Amazon OpenSearch Service domain and all of its data.
    ///
    /// **AWS API**: `DELETE /2021-01-01/opensearch/domain/{DomainName}`
    ///
    /// # Path Parameters
    /// - `DomainName` —  *(required)*
    ///
    /// # Response
    /// [`DeleteDomainResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_domain(&self, domain_name: &str) -> Result<DeleteDomainResponse> {
        let url = format!(
            "{}/2021-01-01/opensearch/domain/{}",
            self.base_url(),
            encode(domain_name),
        );
        let response = self.client.delete(&url, "es").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read delete_domain response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse delete_domain response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_domain_names() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/2021-01-01/domain?engineType=test-engineType")
            .returning_json(serde_json::to_value(ListDomainNamesResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = OpensearchOps::new(&client);

        let result = ops.list_domain_names("test-engineType").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_domain() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/2021-01-01/opensearch/domain/test-DomainName")
            .returning_json(serde_json::to_value(DescribeDomainResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = OpensearchOps::new(&client);

        let result = ops.describe_domain("test-DomainName").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_domain() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/2021-01-01/opensearch/domain/test-DomainName")
            .returning_json(serde_json::to_value(DeleteDomainResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = OpensearchOps::new(&client);

        let result = ops.delete_domain("test-DomainName").await;
        assert!(result.is_ok());
    }
}
