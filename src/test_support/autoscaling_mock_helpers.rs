//! MockClient helpers for Amazon Auto Scaling API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon Auto Scaling helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait AutoscalingMockHelpers {
    /// Helper to expect `describe_auto_scaling_groups`: Describes one or more Auto Scaling groups.
    fn expect_describe_auto_scaling_groups(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_launch_configurations`: Describes one or more launch
    /// configurations.
    fn expect_describe_launch_configurations(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_auto_scaling_group`: Updates the configuration for the specified
    /// Auto Scaling group.
    fn expect_update_auto_scaling_group(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `start_instance_refresh`: Starts an instance refresh for the specified Auto
    /// Scaling group.
    fn expect_start_instance_refresh(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl AutoscalingMockHelpers for MockClient {
    /// Helper to expect `describe_auto_scaling_groups`: Describes one or more Auto Scaling groups.
    fn expect_describe_auto_scaling_groups(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_launch_configurations`: Describes one or more launch
    /// configurations.
    fn expect_describe_launch_configurations(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `update_auto_scaling_group`: Updates the configuration for the specified
    /// Auto Scaling group.
    fn expect_update_auto_scaling_group(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `start_instance_refresh`: Starts an instance refresh for the specified Auto
    /// Scaling group.
    fn expect_start_instance_refresh(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
