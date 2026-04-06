//! MockClient helpers for Amazon CloudWatch API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon CloudWatch helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CloudwatchMockHelpers {
    /// Helper to expect `get_metric_statistics`: Gets statistics for the specified metric.
    fn expect_get_metric_statistics(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_metrics`: List the specified metrics.
    fn expect_list_metrics(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_alarms`: Retrieves the specified alarms.
    fn expect_describe_alarms(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `put_metric_alarm`: Creates or updates an alarm and associates it with the
    /// specified metric.
    fn expect_put_metric_alarm(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_alarms`: Deletes the specified alarms.
    fn expect_delete_alarms(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_metric_data`: Retrieves CloudWatch metric values for one or more
    /// metrics in a single batch request.
    fn expect_get_metric_data(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CloudwatchMockHelpers for MockClient {
    /// Helper to expect `get_metric_statistics`: Gets statistics for the specified metric.
    fn expect_get_metric_statistics(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_metrics`: List the specified metrics.
    fn expect_list_metrics(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_alarms`: Retrieves the specified alarms.
    fn expect_describe_alarms(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `put_metric_alarm`: Creates or updates an alarm and associates it with the
    /// specified metric.
    fn expect_put_metric_alarm(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_alarms`: Deletes the specified alarms.
    fn expect_delete_alarms(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_metric_data`: Retrieves CloudWatch metric values for one or more
    /// metrics in a single batch request.
    fn expect_get_metric_data(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
