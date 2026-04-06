//! MockClient helpers for Amazon OpenSearch Service API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon OpenSearch Service helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait OpensearchMockHelpers {
    /// Helper to expect `list_domain_names`: Returns the names of all Amazon OpenSearch Service
    /// domains owned by the current user in the active Region.
    fn expect_list_domain_names(&mut self, engine_type: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_domain`: Returns domain configuration information about the
    /// specified Amazon OpenSearch Service domain.
    fn expect_describe_domain(&mut self, domain_name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_domain`: Deletes an Amazon OpenSearch Service domain and all of its
    /// data.
    fn expect_delete_domain(&mut self, domain_name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl OpensearchMockHelpers for MockClient {
    /// Helper to expect `list_domain_names`: Returns the names of all Amazon OpenSearch Service
    /// domains owned by the current user in the active Region.
    fn expect_list_domain_names(
        &mut self,
        engine_type: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = "/2021-01-01/domain".to_string();
        let mut __qp: Vec<String> = Vec::new();
        if !engine_type.is_empty() {
            __qp.push(format!("engineType={}", engine_type));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `describe_domain`: Returns domain configuration information about the
    /// specified Amazon OpenSearch Service domain.
    fn expect_describe_domain(
        &mut self,
        domain_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/2021-01-01/opensearch/domain/{domain_name}");
        self.expect_get(&path)
    }

    /// Helper to expect `delete_domain`: Deletes an Amazon OpenSearch Service domain and all of its
    /// data.
    fn expect_delete_domain(
        &mut self,
        domain_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/2021-01-01/opensearch/domain/{domain_name}");
        self.expect_delete(&path)
    }
}
