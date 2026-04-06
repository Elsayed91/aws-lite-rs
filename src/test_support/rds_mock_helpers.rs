//! MockClient helpers for Amazon Relational Database Service API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon Relational Database Service helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait RdsMockHelpers {
    /// Helper to expect `describe_db_instances`: Describes provisioned RDS instances.
    fn expect_describe_db_instances(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_db_snapshots`: Returns information about DB snapshots.
    fn expect_describe_db_snapshots(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_db_snapshot_attributes`: Returns a list of DB snapshot attribute
    /// names and values for a manual DB snapshot.
    fn expect_describe_db_snapshot_attributes(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `modify_db_instance`: Modifies settings for a DB instance.
    fn expect_modify_db_instance(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `stop_db_instance`: Stops an Amazon RDS DB instance.
    fn expect_stop_db_instance(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `start_db_instance`: Starts an Amazon RDS DB instance that was stopped.
    fn expect_start_db_instance(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_db_instance`: Deletes a previously provisioned DB instance.
    fn expect_delete_db_instance(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_db_snapshot`: Creates a snapshot of a DB instance.
    fn expect_create_db_snapshot(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_db_snapshot`: Deletes a DB snapshot.
    fn expect_delete_db_snapshot(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `modify_db_snapshot_attribute`: Adds an attribute and values to, or removes
    /// an attribute and values from, a manual DB snapshot.
    fn expect_modify_db_snapshot_attribute(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl RdsMockHelpers for MockClient {
    /// Helper to expect `describe_db_instances`: Describes provisioned RDS instances.
    fn expect_describe_db_instances(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_db_snapshots`: Returns information about DB snapshots.
    fn expect_describe_db_snapshots(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_db_snapshot_attributes`: Returns a list of DB snapshot attribute
    /// names and values for a manual DB snapshot.
    fn expect_describe_db_snapshot_attributes(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `modify_db_instance`: Modifies settings for a DB instance.
    fn expect_modify_db_instance(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `stop_db_instance`: Stops an Amazon RDS DB instance.
    fn expect_stop_db_instance(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `start_db_instance`: Starts an Amazon RDS DB instance that was stopped.
    fn expect_start_db_instance(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_db_instance`: Deletes a previously provisioned DB instance.
    fn expect_delete_db_instance(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `create_db_snapshot`: Creates a snapshot of a DB instance.
    fn expect_create_db_snapshot(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_db_snapshot`: Deletes a DB snapshot.
    fn expect_delete_db_snapshot(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `modify_db_snapshot_attribute`: Adds an attribute and values to, or removes
    /// an attribute and values from, a manual DB snapshot.
    fn expect_modify_db_snapshot_attribute(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
