//! MockClient helpers for Amazon Route 53 API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon Route 53 helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait Route53MockHelpers {
    /// Helper to expect `list_hosted_zones`: Retrieves a list of the public and private hosted
    /// zones associated with the current AWS account.
    fn expect_list_hosted_zones(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_resource_record_sets`: Lists the resource record sets in a specified
    /// hosted zone.
    fn expect_list_resource_record_sets(&mut self, id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_health_checks`: Retrieve a list of the health checks associated with
    /// the current AWS account.
    fn expect_list_health_checks(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_health_check_status`: Gets status of a health check based on the most
    /// recent checker observations.
    fn expect_get_health_check_status(&mut self, health_check_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_health_check`: Creates a new health check.
    fn expect_create_health_check(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_health_check`: Deletes a health check.
    fn expect_delete_health_check(&mut self, health_check_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `change_resource_record_sets`: Creates, changes, or deletes a resource
    /// record set.
    fn expect_change_resource_record_sets(&mut self, id: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl Route53MockHelpers for MockClient {
    /// Helper to expect `list_hosted_zones`: Retrieves a list of the public and private hosted
    /// zones associated with the current AWS account.
    fn expect_list_hosted_zones(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/2013-04-01/hostedzone".to_string();
        self.expect_get(&path)
    }

    /// Helper to expect `list_resource_record_sets`: Lists the resource record sets in a specified
    /// hosted zone.
    fn expect_list_resource_record_sets(
        &mut self,
        id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/2013-04-01/hostedzone/{id}/rrset");
        self.expect_get(&path)
    }

    /// Helper to expect `list_health_checks`: Retrieve a list of the health checks associated with
    /// the current AWS account.
    fn expect_list_health_checks(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/2013-04-01/healthcheck".to_string();
        self.expect_get(&path)
    }

    /// Helper to expect `get_health_check_status`: Gets status of a health check based on the most
    /// recent checker observations.
    fn expect_get_health_check_status(
        &mut self,
        health_check_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/2013-04-01/healthcheck/{health_check_id}/status");
        self.expect_get(&path)
    }

    /// Helper to expect `create_health_check`: Creates a new health check.
    fn expect_create_health_check(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/2013-04-01/healthcheck".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_health_check`: Deletes a health check.
    fn expect_delete_health_check(
        &mut self,
        health_check_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/2013-04-01/healthcheck/{health_check_id}");
        self.expect_delete(&path)
    }

    /// Helper to expect `change_resource_record_sets`: Creates, changes, or deletes a resource
    /// record set.
    fn expect_change_resource_record_sets(
        &mut self,
        id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/2013-04-01/hostedzone/{id}/rrset/");
        self.expect_post(&path)
    }
}
