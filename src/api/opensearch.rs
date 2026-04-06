//! Amazon OpenSearch Service API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::opensearch::OpensearchOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::opensearch::OpensearchOps,
    types::opensearch::{DeleteDomainResponse, DescribeDomainResponse, ListDomainNamesResponse},
};

/// Client for the Amazon OpenSearch Service API
pub struct OpensearchClient<'a> {
    ops: OpensearchOps<'a>,
}

impl<'a> OpensearchClient<'a> {
    /// Create a new Amazon OpenSearch Service API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: OpensearchOps::new(client),
        }
    }

    /// Returns the names of all domains owned by the current user in the active Region.
    ///
    /// Pass `engine_type = ""` to list all engine types (no filter).
    pub async fn list_domain_names(&self, engine_type: &str) -> Result<ListDomainNamesResponse> {
        self.ops.list_domain_names(engine_type).await
    }

    /// Returns domain configuration information about the specified domain.
    pub async fn describe_domain(&self, domain_name: &str) -> Result<DescribeDomainResponse> {
        self.ops.describe_domain(domain_name).await
    }

    /// Deletes an OpenSearch Service domain and all of its data.
    pub async fn delete_domain(&self, domain_name: &str) -> Result<DeleteDomainResponse> {
        self.ops.delete_domain(domain_name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;
    use crate::types::opensearch::*;

    #[tokio::test]
    async fn test_list_domain_names_returns_domains() {
        let mut mock = MockClient::new();
        mock.expect_get("/2021-01-01/domain")
            .returning_json(serde_json::json!({
                "DomainNames": [
                    {"DomainName": "my-domain", "EngineType": "OpenSearch"}
                ]
            }));

        let client = AwsHttpClient::from_mock(mock);
        let result = client.opensearch().list_domain_names("").await.unwrap();

        assert_eq!(result.domain_names.len(), 1);
        assert_eq!(
            result.domain_names[0].domain_name.as_deref(),
            Some("my-domain")
        );
    }

    #[tokio::test]
    async fn test_list_domain_names_returns_empty_when_no_domains() {
        let mut mock = MockClient::new();
        mock.expect_get("/2021-01-01/domain")
            .returning_json(serde_json::json!({}));

        let client = AwsHttpClient::from_mock(mock);
        let result = client.opensearch().list_domain_names("").await.unwrap();

        assert_eq!(result.domain_names.len(), 0);
    }

    #[tokio::test]
    async fn test_describe_domain_returns_domain_status() {
        let mut mock = MockClient::new();
        mock.expect_get("/2021-01-01/opensearch/domain/test-domain")
            .returning_json(serde_json::json!({
                "DomainStatus": {
                    "DomainId": "123456789012/test-domain",
                    "DomainName": "test-domain",
                    "ARN": "arn:aws:es:us-east-1:123456789012:domain/test-domain",
                    "Created": true,
                    "Deleted": false,
                    "Processing": false,
                    "EngineVersion": "OpenSearch_2.11"
                }
            }));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .opensearch()
            .describe_domain("test-domain")
            .await
            .unwrap();

        let status = &result.domain_status;
        assert_eq!(status.domain_name, "test-domain");
        assert_eq!(status.domain_id, "123456789012/test-domain");
        assert_eq!(status.created, Some(true));
        assert_eq!(status.engine_version.as_deref(), Some("OpenSearch_2.11"));
    }

    #[tokio::test]
    async fn test_delete_domain_returns_domain_status() {
        let mut mock = MockClient::new();
        mock.expect_delete("/2021-01-01/opensearch/domain/test-domain")
            .returning_json(serde_json::json!({
                "DomainStatus": {
                    "DomainId": "123456789012/test-domain",
                    "DomainName": "test-domain",
                    "ARN": "arn:aws:es:us-east-1:123456789012:domain/test-domain",
                    "Deleted": true
                }
            }));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .opensearch()
            .delete_domain("test-domain")
            .await
            .unwrap();

        let status = result.domain_status.as_ref().unwrap();
        assert_eq!(status.domain_name, "test-domain");
        assert_eq!(status.deleted, Some(true));
    }
}
