//! MockClient helpers for AWS Config API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS Config helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait ConfigMockHelpers {
    /// Helper to expect `select_resource_config`: Accepts a SQL SELECT command and returns matching
    /// resource configurations.
    fn expect_select_resource_config(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_configuration_recorders`: Returns the details for the specified
    /// configuration recorders.
    fn expect_describe_configuration_recorders(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_configuration_recorder_status`: Returns the current status of the
    /// specified configuration recorder.
    fn expect_describe_configuration_recorder_status(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl ConfigMockHelpers for MockClient {
    /// Helper to expect `select_resource_config`: Accepts a SQL SELECT command and returns matching
    /// resource configurations.
    fn expect_select_resource_config(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_configuration_recorders`: Returns the details for the specified
    /// configuration recorders.
    fn expect_describe_configuration_recorders(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_configuration_recorder_status`: Returns the current status of the
    /// specified configuration recorder.
    fn expect_describe_configuration_recorder_status(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
