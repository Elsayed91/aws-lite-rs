//! MockClient helpers for AWS Security Hub API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS Security Hub helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait SecurityhubMockHelpers {
    /// Helper to expect `describe_hub`: Returns details about the Hub resource in your account,
    /// including the HubArn and the time when you enabled Security Hub.
    fn expect_describe_hub(&mut self, hub_arn: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl SecurityhubMockHelpers for MockClient {
    /// Helper to expect `describe_hub`: Returns details about the Hub resource in your account,
    /// including the HubArn and the time when you enabled Security Hub.
    fn expect_describe_hub(&mut self, hub_arn: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = "/accounts".to_string();
        let mut __qp: Vec<String> = Vec::new();
        if !hub_arn.is_empty() {
            __qp.push(format!("HubArn={}", hub_arn));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }
}
