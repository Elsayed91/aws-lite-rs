//! MockClient helpers for Amazon CloudWatch Logs API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon CloudWatch Logs helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait LogsMockHelpers {
    /// Helper to expect `describe_log_groups`: Lists the specified log groups.
    fn expect_describe_log_groups(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_tags_for_resource`: Displays the tags associated with a CloudWatch
    /// Logs resource.
    fn expect_list_tags_for_resource(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_log_streams`: Lists the log streams for the specified log group.
    fn expect_describe_log_streams(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `put_retention_policy`: Sets the retention of the specified log group.
    fn expect_put_retention_policy(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_log_stream`: Deletes the specified log stream.
    fn expect_delete_log_stream(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_metric_filters`: Lists the specified metric filters. You can list
    /// all of the metric filters or filter the results by log name, prefix, metric name, namespace,
    /// and dimensions.
    fn expect_describe_metric_filters(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl LogsMockHelpers for MockClient {
    /// Helper to expect `describe_log_groups`: Lists the specified log groups.
    fn expect_describe_log_groups(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_tags_for_resource`: Displays the tags associated with a CloudWatch
    /// Logs resource.
    fn expect_list_tags_for_resource(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_log_streams`: Lists the log streams for the specified log group.
    fn expect_describe_log_streams(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `put_retention_policy`: Sets the retention of the specified log group.
    fn expect_put_retention_policy(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_log_stream`: Deletes the specified log stream.
    fn expect_delete_log_stream(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_metric_filters`: Lists the specified metric filters. You can list
    /// all of the metric filters or filter the results by log name, prefix, metric name, namespace,
    /// and dimensions.
    fn expect_describe_metric_filters(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
