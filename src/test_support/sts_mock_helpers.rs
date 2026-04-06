//! MockClient helpers for AWS Security Token Service API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS Security Token Service helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait StsMockHelpers {
    /// Helper to expect `get_caller_identity`: Returns details about the IAM user or role whose
    /// credentials are used to call the operation.
    fn expect_get_caller_identity(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `assume_role`: Returns a set of temporary security credentials for cross-
    /// account access.
    fn expect_assume_role(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl StsMockHelpers for MockClient {
    /// Helper to expect `get_caller_identity`: Returns details about the IAM user or role whose
    /// credentials are used to call the operation.
    fn expect_get_caller_identity(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `assume_role`: Returns a set of temporary security credentials for cross-
    /// account access.
    fn expect_assume_role(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
