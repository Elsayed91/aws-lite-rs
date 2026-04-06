//! MockClient helpers for AWS IAM Access Analyzer API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS IAM Access Analyzer helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait AccessanalyzerMockHelpers {
    /// Helper to expect `list_analyzers`: Retrieves a list of analyzers.
    fn expect_list_analyzers(
        &mut self,
        next_token: &str,
        max_results: &str,
        r#type: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl AccessanalyzerMockHelpers for MockClient {
    /// Helper to expect `list_analyzers`: Retrieves a list of analyzers.
    fn expect_list_analyzers(
        &mut self,
        next_token: &str,
        max_results: &str,
        r#type: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = "/analyzer".to_string();
        let mut __qp: Vec<String> = Vec::new();
        if !next_token.is_empty() {
            __qp.push(format!("nextToken={}", next_token));
        }
        if !max_results.is_empty() {
            __qp.push(format!("maxResults={}", max_results));
        }
        if !r#type.is_empty() {
            __qp.push(format!("type={}", r#type));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }
}
