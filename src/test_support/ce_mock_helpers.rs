//! MockClient helpers for AWS Cost Explorer API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS Cost Explorer helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CeMockHelpers {
    /// Helper to expect `get_cost_and_usage`: Retrieves cost and usage metrics for your account.
    fn expect_get_cost_and_usage(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CeMockHelpers for MockClient {
    /// Helper to expect `get_cost_and_usage`: Retrieves cost and usage metrics for your account.
    fn expect_get_cost_and_usage(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
