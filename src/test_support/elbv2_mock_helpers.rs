//! MockClient helpers for Elastic Load Balancing v2 API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Elastic Load Balancing v2 helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait Elbv2MockHelpers {
    /// Helper to expect `describe_load_balancers`: Describes the specified load balancers or all of
    /// your load balancers.
    fn expect_describe_load_balancers(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_target_groups`: Describes the specified target groups or all of
    /// your target groups. By default, all target groups are described. Alternatively, you can
    /// specify one of the following to filter the results: the ARN of the load balancer, the names
    /// of one or more target groups, or the ARNs of one or more target groups.
    fn expect_describe_target_groups(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_target_health`: Describes the health of the specified targets or
    /// all of your targets.
    fn expect_describe_target_health(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_listeners`: Describes the specified listeners or the listeners
    /// for the specified Application Load Balancer, Network Load Balancer, or Gateway Load
    /// Balancer. You must specify either a load balancer or one or more listeners.
    fn expect_describe_listeners(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_load_balancer_attributes`: Describes the attributes for the
    /// specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. For
    /// more information, see the following: Load balancer attributes in the Application Load
    /// Balancers Guide Load balancer attributes in the Network Load Balancers Guide Load balancer
    /// attributes in the Gateway Load Balancers Guide
    fn expect_describe_load_balancer_attributes(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_load_balancer`: Deletes the specified Application Load Balancer,
    /// Network Load Balancer, or Gateway Load Balancer. Deleting a load balancer also deletes its
    /// listeners. You can't delete a load balancer if deletion protection is enabled. If the load
    /// balancer does not exist or has already been deleted, the call succeeds. Deleting a load
    /// balancer does not affect its registered targets. For example, your EC2 instances continue to
    /// run and are still registered to their target groups. If you no longer need these EC2
    /// instances, you can stop or terminate them.
    fn expect_delete_load_balancer(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_target_group`: Deletes the specified target group. You can delete a
    /// target group if it is not referenced by any actions. Deleting a target group also deletes
    /// any associated health checks. Deleting a target group does not affect its registered
    /// targets. For example, any EC2 instances continue to run until you stop or terminate them.
    fn expect_delete_target_group(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `modify_load_balancer_attributes`: Modifies the specified attributes of the
    /// specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. If any
    /// of the specified attributes can't be modified as requested, the call fails. Any existing
    /// attributes that you do not modify retain their current values.
    fn expect_modify_load_balancer_attributes(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl Elbv2MockHelpers for MockClient {
    /// Helper to expect `describe_load_balancers`: Describes the specified load balancers or all of
    /// your load balancers.
    fn expect_describe_load_balancers(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_target_groups`: Describes the specified target groups or all of
    /// your target groups. By default, all target groups are described. Alternatively, you can
    /// specify one of the following to filter the results: the ARN of the load balancer, the names
    /// of one or more target groups, or the ARNs of one or more target groups.
    fn expect_describe_target_groups(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_target_health`: Describes the health of the specified targets or
    /// all of your targets.
    fn expect_describe_target_health(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_listeners`: Describes the specified listeners or the listeners
    /// for the specified Application Load Balancer, Network Load Balancer, or Gateway Load
    /// Balancer. You must specify either a load balancer or one or more listeners.
    fn expect_describe_listeners(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_load_balancer_attributes`: Describes the attributes for the
    /// specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. For
    /// more information, see the following: Load balancer attributes in the Application Load
    /// Balancers Guide Load balancer attributes in the Network Load Balancers Guide Load balancer
    /// attributes in the Gateway Load Balancers Guide
    fn expect_describe_load_balancer_attributes(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_load_balancer`: Deletes the specified Application Load Balancer,
    /// Network Load Balancer, or Gateway Load Balancer. Deleting a load balancer also deletes its
    /// listeners. You can't delete a load balancer if deletion protection is enabled. If the load
    /// balancer does not exist or has already been deleted, the call succeeds. Deleting a load
    /// balancer does not affect its registered targets. For example, your EC2 instances continue to
    /// run and are still registered to their target groups. If you no longer need these EC2
    /// instances, you can stop or terminate them.
    fn expect_delete_load_balancer(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_target_group`: Deletes the specified target group. You can delete a
    /// target group if it is not referenced by any actions. Deleting a target group also deletes
    /// any associated health checks. Deleting a target group does not affect its registered
    /// targets. For example, any EC2 instances continue to run until you stop or terminate them.
    fn expect_delete_target_group(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `modify_load_balancer_attributes`: Modifies the specified attributes of the
    /// specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. If any
    /// of the specified attributes can't be modified as requested, the call fails. Any existing
    /// attributes that you do not modify retain their current values.
    fn expect_modify_load_balancer_attributes(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
