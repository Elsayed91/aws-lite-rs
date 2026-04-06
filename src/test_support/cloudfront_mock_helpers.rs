//! MockClient helpers for Amazon CloudFront API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon CloudFront helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CloudfrontMockHelpers {
    /// Helper to expect `list_distributions`: List CloudFront distributions.
    fn expect_list_distributions(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_distribution_config`: Get the configuration for a CloudFront
    /// distribution.
    fn expect_get_distribution_config(&mut self, id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_distribution`: Update a CloudFront distribution configuration.
    fn expect_update_distribution(&mut self, id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_origin_access_control`: Creates a new origin access control in
    /// CloudFront.
    fn expect_create_origin_access_control(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_distribution`: Creates a new CloudFront distribution.
    fn expect_create_distribution(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CloudfrontMockHelpers for MockClient {
    /// Helper to expect `list_distributions`: List CloudFront distributions.
    fn expect_list_distributions(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/2020-05-31/distribution".to_string();
        self.expect_get(&path)
    }

    /// Helper to expect `get_distribution_config`: Get the configuration for a CloudFront
    /// distribution.
    fn expect_get_distribution_config(
        &mut self,
        id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/2020-05-31/distribution/{id}/config");
        self.expect_get(&path)
    }

    /// Helper to expect `update_distribution`: Update a CloudFront distribution configuration.
    fn expect_update_distribution(
        &mut self,
        id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/2020-05-31/distribution/{id}/config");
        self.expect_put(&path)
    }

    /// Helper to expect `create_origin_access_control`: Creates a new origin access control in
    /// CloudFront.
    fn expect_create_origin_access_control(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/2020-05-31/origin-access-control".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `create_distribution`: Creates a new CloudFront distribution.
    fn expect_create_distribution(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/2020-05-31/distribution".to_string();
        self.expect_post(&path)
    }
}
