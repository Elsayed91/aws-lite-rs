//! MockClient helpers for Amazon Kinesis API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon Kinesis helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait KinesisMockHelpers {
    /// Helper to expect `list_streams`: Lists the Kinesis data streams associated with the AWS
    /// account.
    fn expect_list_streams(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_stream_summary`: Provides a summarized description of the
    /// specified Kinesis data stream.
    fn expect_describe_stream_summary(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_stream`: Deletes a Kinesis data stream and all its shards and data.
    fn expect_delete_stream(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_stream_mode`: Updates the capacity mode of a Kinesis data stream
    /// (PROVISIONED or ON_DEMAND).
    fn expect_update_stream_mode(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl KinesisMockHelpers for MockClient {
    /// Helper to expect `list_streams`: Lists the Kinesis data streams associated with the AWS
    /// account.
    fn expect_list_streams(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_stream_summary`: Provides a summarized description of the
    /// specified Kinesis data stream.
    fn expect_describe_stream_summary(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_stream`: Deletes a Kinesis data stream and all its shards and data.
    fn expect_delete_stream(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `update_stream_mode`: Updates the capacity mode of a Kinesis data stream
    /// (PROVISIONED or ON_DEMAND).
    fn expect_update_stream_mode(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
