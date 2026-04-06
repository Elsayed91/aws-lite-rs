//! MockClient helpers for Amazon Elastic Kubernetes Service API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon Elastic Kubernetes Service helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait EksMockHelpers {
    /// Helper to expect `describe_cluster`: Returns descriptive information about an Amazon EKS
    /// cluster.
    fn expect_describe_cluster(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_nodegroups`: Lists the managed node groups associated with the
    /// specified cluster.
    fn expect_list_nodegroups(
        &mut self,
        name: &str,
        max_results: &str,
        next_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_nodegroup`: Returns descriptive information about an Amazon EKS
    /// node group.
    fn expect_describe_nodegroup(
        &mut self,
        name: &str,
        nodegroup_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_nodegroup_config`: Updates an Amazon EKS managed node group
    /// configuration.
    fn expect_update_nodegroup_config(
        &mut self,
        name: &str,
        nodegroup_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl EksMockHelpers for MockClient {
    /// Helper to expect `describe_cluster`: Returns descriptive information about an Amazon EKS
    /// cluster.
    fn expect_describe_cluster(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/clusters/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_nodegroups`: Lists the managed node groups associated with the
    /// specified cluster.
    fn expect_list_nodegroups(
        &mut self,
        name: &str,
        max_results: &str,
        next_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/clusters/{name}/node-groups");
        let mut __qp: Vec<String> = Vec::new();
        if !max_results.is_empty() {
            __qp.push(format!("maxResults={}", max_results));
        }
        if !next_token.is_empty() {
            __qp.push(format!("nextToken={}", next_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `describe_nodegroup`: Returns descriptive information about an Amazon EKS
    /// node group.
    fn expect_describe_nodegroup(
        &mut self,
        name: &str,
        nodegroup_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/clusters/{name}/node-groups/{nodegroup_name}");
        self.expect_get(&path)
    }

    /// Helper to expect `update_nodegroup_config`: Updates an Amazon EKS managed node group
    /// configuration.
    fn expect_update_nodegroup_config(
        &mut self,
        name: &str,
        nodegroup_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/clusters/{name}/node-groups/{nodegroup_name}/update-config");
        self.expect_post(&path)
    }
}
