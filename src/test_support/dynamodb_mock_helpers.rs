//! MockClient helpers for Amazon DynamoDB API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon DynamoDB helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait DynamodbMockHelpers {
    /// Helper to expect `list_tables`: Returns an array of table names associated with the current
    /// account and endpoint.
    fn expect_list_tables(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_table`: Returns information about the table, including the
    /// current status of the table, when it was created, the primary key schema, and any indexes on
    /// the table.
    fn expect_describe_table(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_table`: Modifies the provisioned throughput settings, global
    /// secondary indexes, or DynamoDB Streams settings for a given table.
    fn expect_update_table(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_table`: The DeleteTable operation deletes a table and all of its
    /// items.
    fn expect_delete_table(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl DynamodbMockHelpers for MockClient {
    /// Helper to expect `list_tables`: Returns an array of table names associated with the current
    /// account and endpoint.
    fn expect_list_tables(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_table`: Returns information about the table, including the
    /// current status of the table, when it was created, the primary key schema, and any indexes on
    /// the table.
    fn expect_describe_table(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `update_table`: Modifies the provisioned throughput settings, global
    /// secondary indexes, or DynamoDB Streams settings for a given table.
    fn expect_update_table(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_table`: The DeleteTable operation deletes a table and all of its
    /// items.
    fn expect_delete_table(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
