//! MockClient helpers for AWS Key Management Service API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS Key Management Service helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait KmsMockHelpers {
    /// Helper to expect `list_keys`: Lists the key ARNs of all KMS keys in the current account and
    /// region.
    fn expect_list_keys(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_key`: Provides detailed information about a KMS key.
    fn expect_describe_key(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_key_rotation_status`: Gets a Boolean value that indicates whether
    /// automatic rotation of the key material is enabled for the specified KMS key.
    fn expect_get_key_rotation_status(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `enable_key_rotation`: Enables automatic rotation of the key material for
    /// the specified symmetric encryption KMS key.
    fn expect_enable_key_rotation(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl KmsMockHelpers for MockClient {
    /// Helper to expect `list_keys`: Lists the key ARNs of all KMS keys in the current account and
    /// region.
    fn expect_list_keys(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_key`: Provides detailed information about a KMS key.
    fn expect_describe_key(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_key_rotation_status`: Gets a Boolean value that indicates whether
    /// automatic rotation of the key material is enabled for the specified KMS key.
    fn expect_get_key_rotation_status(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `enable_key_rotation`: Enables automatic rotation of the key material for
    /// the specified symmetric encryption KMS key.
    fn expect_enable_key_rotation(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
