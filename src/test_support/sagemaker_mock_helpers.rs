//! MockClient helpers for Amazon SageMaker API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon SageMaker helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait SagemakerMockHelpers {
    /// Helper to expect `list_notebook_instances`: Returns a list of the Amazon SageMaker notebook
    /// instances in the requester's account.
    fn expect_list_notebook_instances(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `stop_notebook_instance`: Terminates the ML compute instance. Before
    /// terminating the instance, SageMaker disconnects the ML storage volume from it.
    fn expect_stop_notebook_instance(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl SagemakerMockHelpers for MockClient {
    /// Helper to expect `list_notebook_instances`: Returns a list of the Amazon SageMaker notebook
    /// instances in the requester's account.
    fn expect_list_notebook_instances(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `stop_notebook_instance`: Terminates the ML compute instance. Before
    /// terminating the instance, SageMaker disconnects the ML storage volume from it.
    fn expect_stop_notebook_instance(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
