//! MockClient helpers for Amazon ElastiCache API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon ElastiCache helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait ElasticacheMockHelpers {
    /// Helper to expect `describe_cache_clusters`: Returns information about all provisioned
    /// clusters if no cluster identifier is specified, or about a specific cache cluster if a
    /// cluster identifier is supplied. By default, abbreviated information about the clusters is
    /// returned. You can use the optional ShowCacheNodeInfo flag to retrieve detailed information
    /// about the cache nodes associated with the clusters. These details include the DNS address
    /// and port for the cache node endpoint. If the cluster is in the creating state, only cluster-
    /// level information is displayed until all of the nodes are successfully provisioned. If the
    /// cluster is in the deleting state, only cluster-level information is displayed. If cache
    /// nodes are currently being added to the cluster, node endpoint information and creation time
    /// for the additional nodes are not displayed until they are completely provisioned. When the
    /// cluster state is available, the cluster is ready for use. If cache nodes are currently being
    /// removed from the cluster, no endpoint information for the removed nodes is displayed.
    fn expect_describe_cache_clusters(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_replication_groups`: Returns information about a particular
    /// replication group. If no identifier is specified, DescribeReplicationGroups returns
    /// information about all replication groups. This operation is valid for Valkey or Redis OSS
    /// only.
    fn expect_describe_replication_groups(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_cache_cluster`: Deletes a previously provisioned cluster.
    /// DeleteCacheCluster deletes all associated cache nodes, node endpoints and the cluster
    /// itself. When you receive a successful response from this operation, Amazon ElastiCache
    /// immediately begins deleting the cluster; you cannot cancel or revert this operation. This
    /// operation is not valid for: Valkey or Redis OSS (cluster mode enabled) clusters Valkey or
    /// Redis OSS (cluster mode disabled) clusters A cluster that is the last read replica of a
    /// replication group A cluster that is the primary node of a replication group A node group
    /// (shard) that has Multi-AZ mode enabled A cluster from a Valkey or Redis OSS (cluster mode
    /// enabled) replication group A cluster that is not in the available state
    fn expect_delete_cache_cluster(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_replication_group`: Deletes an existing replication group. By
    /// default, this operation deletes the entire replication group, including the
    /// primary/primaries and all of the read replicas. If the replication group has only one
    /// primary, you can optionally delete only the read replicas, while retaining the primary by
    /// setting RetainPrimaryCluster=true. When you receive a successful response from this
    /// operation, Amazon ElastiCache immediately begins deleting the selected resources; you cannot
    /// cancel or revert this operation. CreateSnapshot permission is required to create a final
    /// snapshot. Without this permission, the API call will fail with an Access Denied exception.
    /// This operation is valid for Redis OSS only.
    fn expect_delete_replication_group(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_snapshot`: Creates a copy of an entire cluster or replication group
    /// at a specific moment in time. This operation is valid for Valkey or Redis OSS only.
    fn expect_create_snapshot(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl ElasticacheMockHelpers for MockClient {
    /// Helper to expect `describe_cache_clusters`: Returns information about all provisioned
    /// clusters if no cluster identifier is specified, or about a specific cache cluster if a
    /// cluster identifier is supplied. By default, abbreviated information about the clusters is
    /// returned. You can use the optional ShowCacheNodeInfo flag to retrieve detailed information
    /// about the cache nodes associated with the clusters. These details include the DNS address
    /// and port for the cache node endpoint. If the cluster is in the creating state, only cluster-
    /// level information is displayed until all of the nodes are successfully provisioned. If the
    /// cluster is in the deleting state, only cluster-level information is displayed. If cache
    /// nodes are currently being added to the cluster, node endpoint information and creation time
    /// for the additional nodes are not displayed until they are completely provisioned. When the
    /// cluster state is available, the cluster is ready for use. If cache nodes are currently being
    /// removed from the cluster, no endpoint information for the removed nodes is displayed.
    fn expect_describe_cache_clusters(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_replication_groups`: Returns information about a particular
    /// replication group. If no identifier is specified, DescribeReplicationGroups returns
    /// information about all replication groups. This operation is valid for Valkey or Redis OSS
    /// only.
    fn expect_describe_replication_groups(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_cache_cluster`: Deletes a previously provisioned cluster.
    /// DeleteCacheCluster deletes all associated cache nodes, node endpoints and the cluster
    /// itself. When you receive a successful response from this operation, Amazon ElastiCache
    /// immediately begins deleting the cluster; you cannot cancel or revert this operation. This
    /// operation is not valid for: Valkey or Redis OSS (cluster mode enabled) clusters Valkey or
    /// Redis OSS (cluster mode disabled) clusters A cluster that is the last read replica of a
    /// replication group A cluster that is the primary node of a replication group A node group
    /// (shard) that has Multi-AZ mode enabled A cluster from a Valkey or Redis OSS (cluster mode
    /// enabled) replication group A cluster that is not in the available state
    fn expect_delete_cache_cluster(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_replication_group`: Deletes an existing replication group. By
    /// default, this operation deletes the entire replication group, including the
    /// primary/primaries and all of the read replicas. If the replication group has only one
    /// primary, you can optionally delete only the read replicas, while retaining the primary by
    /// setting RetainPrimaryCluster=true. When you receive a successful response from this
    /// operation, Amazon ElastiCache immediately begins deleting the selected resources; you cannot
    /// cancel or revert this operation. CreateSnapshot permission is required to create a final
    /// snapshot. Without this permission, the API call will fail with an Access Denied exception.
    /// This operation is valid for Redis OSS only.
    fn expect_delete_replication_group(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `create_snapshot`: Creates a copy of an entire cluster or replication group
    /// at a specific moment in time. This operation is valid for Valkey or Redis OSS only.
    fn expect_create_snapshot(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
