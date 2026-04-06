//! MockClient helpers for Amazon Elastic Container Registry API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon Elastic Container Registry helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait EcrMockHelpers {
    /// Helper to expect `describe_repositories`: Describes image repositories in a registry.
    fn expect_describe_repositories(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_images`: Returns metadata about the images in a repository.
    fn expect_describe_images(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `put_lifecycle_policy`: Creates or updates the lifecycle policy for the
    /// specified repository.
    fn expect_put_lifecycle_policy(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `batch_delete_image`: Deletes a list of specified images within a
    /// repository.
    fn expect_batch_delete_image(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl EcrMockHelpers for MockClient {
    /// Helper to expect `describe_repositories`: Describes image repositories in a registry.
    fn expect_describe_repositories(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_images`: Returns metadata about the images in a repository.
    fn expect_describe_images(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `put_lifecycle_policy`: Creates or updates the lifecycle policy for the
    /// specified repository.
    fn expect_put_lifecycle_policy(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `batch_delete_image`: Deletes a list of specified images within a
    /// repository.
    fn expect_batch_delete_image(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
