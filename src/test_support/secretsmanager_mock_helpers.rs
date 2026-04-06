//! MockClient helpers for AWS Secrets Manager API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS Secrets Manager helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait SecretsmanagerMockHelpers {
    /// Helper to expect `list_secrets`: Lists the secrets that are stored by Secrets Manager in the
    /// current account and Region.
    fn expect_list_secrets(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_secret`: Deletes a secret and all of its versions.
    fn expect_delete_secret(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `rotate_secret`: Configures and starts the asynchronous process of rotating
    /// the secret.
    fn expect_rotate_secret(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl SecretsmanagerMockHelpers for MockClient {
    /// Helper to expect `list_secrets`: Lists the secrets that are stored by Secrets Manager in the
    /// current account and Region.
    fn expect_list_secrets(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_secret`: Deletes a secret and all of its versions.
    fn expect_delete_secret(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `rotate_secret`: Configures and starts the asynchronous process of rotating
    /// the secret.
    fn expect_rotate_secret(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
