//! Amazon ElastiCache API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::elasticache::ElasticacheOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::elasticache::ElasticacheOps,
    types::elasticache::{
        CreateSnapshotRequest, CreateSnapshotResponse, DeleteCacheClusterRequest,
        DeleteCacheClusterResponse, DeleteReplicationGroupRequest, DeleteReplicationGroupResponse,
        DescribeCacheClustersRequest, DescribeCacheClustersResponse,
        DescribeReplicationGroupsRequest, DescribeReplicationGroupsResponse,
    },
};

/// Client for the Amazon ElastiCache API
pub struct ElasticacheClient<'a> {
    ops: ElasticacheOps<'a>,
}

impl<'a> ElasticacheClient<'a> {
    /// Create a new Amazon ElastiCache API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: ElasticacheOps::new(client),
        }
    }

    /// Returns information about all provisioned clusters if no cluster identifier is specified, or about a specific cache clus
    pub async fn describe_cache_clusters(
        &self,
        body: &DescribeCacheClustersRequest,
    ) -> Result<DescribeCacheClustersResponse> {
        self.ops.describe_cache_clusters(body).await
    }

    /// Returns information about a particular replication group. If no identifier is specified, DescribeReplicationGroups retur
    pub async fn describe_replication_groups(
        &self,
        body: &DescribeReplicationGroupsRequest,
    ) -> Result<DescribeReplicationGroupsResponse> {
        self.ops.describe_replication_groups(body).await
    }

    /// Deletes a previously provisioned cluster. DeleteCacheCluster deletes all associated cache nodes, node endpoints and the
    pub async fn delete_cache_cluster(
        &self,
        body: &DeleteCacheClusterRequest,
    ) -> Result<DeleteCacheClusterResponse> {
        self.ops.delete_cache_cluster(body).await
    }

    /// Deletes an existing replication group. By default, this operation deletes the entire replication group, including the pr
    pub async fn delete_replication_group(
        &self,
        body: &DeleteReplicationGroupRequest,
    ) -> Result<DeleteReplicationGroupResponse> {
        self.ops.delete_replication_group(body).await
    }

    /// Creates a copy of an entire cluster or replication group at a specific moment in time. This operation is valid for Valke
    pub async fn create_snapshot(
        &self,
        body: &CreateSnapshotRequest,
    ) -> Result<CreateSnapshotResponse> {
        self.ops.create_snapshot(body).await
    }
}

#[cfg(test)]
mod tests {
    use crate::types::elasticache::*;

    #[tokio::test]
    async fn describe_cache_clusters_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeCacheClustersResponse><DescribeCacheClustersResult>
                <CacheClusters>
                    <CacheCluster>
                        <CacheClusterId>my-cluster</CacheClusterId>
                        <CacheNodeType>cache.t3.micro</CacheNodeType>
                        <Engine>memcached</Engine>
                        <EngineVersion>1.6.22</EngineVersion>
                        <CacheClusterStatus>available</CacheClusterStatus>
                        <NumCacheNodes>1</NumCacheNodes>
                        <ARN>arn:aws:elasticache:us-east-1:123456789012:cluster:my-cluster</ARN>
                        <CacheNodes>
                            <CacheNode>
                                <CacheNodeId>0001</CacheNodeId>
                                <CacheNodeStatus>available</CacheNodeStatus>
                                <Endpoint>
                                    <Address>my-cluster.abc.cfg.use1.cache.amazonaws.com</Address>
                                    <Port>11211</Port>
                                </Endpoint>
                            </CacheNode>
                        </CacheNodes>
                    </CacheCluster>
                </CacheClusters>
            </DescribeCacheClustersResult></DescribeCacheClustersResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elasticache()
            .describe_cache_clusters(&DescribeCacheClustersRequest {
                cache_cluster_id: Some("my-cluster".into()),
                show_cache_node_info: Some(true),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.cache_clusters.len(), 1);
        let cluster = &result.cache_clusters[0];
        assert_eq!(cluster.cache_cluster_id.as_deref(), Some("my-cluster"));
        assert_eq!(cluster.engine.as_deref(), Some("memcached"));
        assert_eq!(cluster.cache_cluster_status.as_deref(), Some("available"));
        assert_eq!(cluster.num_cache_nodes, Some(1));
        assert_eq!(cluster.cache_nodes.len(), 1);
        let node = &cluster.cache_nodes[0];
        assert_eq!(node.cache_node_id.as_deref(), Some("0001"));
        let ep = node.endpoint.as_ref().unwrap();
        assert_eq!(ep.port, Some(11211));
    }

    #[tokio::test]
    async fn describe_replication_groups_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeReplicationGroupsResponse><DescribeReplicationGroupsResult>
                <ReplicationGroups>
                    <ReplicationGroup>
                        <ReplicationGroupId>my-rg</ReplicationGroupId>
                        <Description>My replication group</Description>
                        <Status>available</Status>
                        <CacheNodeType>cache.t3.micro</CacheNodeType>
                        <Engine>redis</Engine>
                        <AtRestEncryptionEnabled>true</AtRestEncryptionEnabled>
                        <TransitEncryptionEnabled>true</TransitEncryptionEnabled>
                        <ARN>arn:aws:elasticache:us-east-1:123456789012:replicationgroup:my-rg</ARN>
                        <NodeGroups>
                            <NodeGroup>
                                <NodeGroupId>0001</NodeGroupId>
                                <Status>available</Status>
                                <PrimaryEndpoint>
                                    <Address>my-rg.abc.ng.use1.cache.amazonaws.com</Address>
                                    <Port>6379</Port>
                                </PrimaryEndpoint>
                                <NodeGroupMembers>
                                    <NodeGroupMember>
                                        <CacheClusterId>my-rg-001</CacheClusterId>
                                        <CacheNodeId>0001</CacheNodeId>
                                        <CurrentRole>primary</CurrentRole>
                                    </NodeGroupMember>
                                </NodeGroupMembers>
                            </NodeGroup>
                        </NodeGroups>
                    </ReplicationGroup>
                </ReplicationGroups>
            </DescribeReplicationGroupsResult></DescribeReplicationGroupsResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elasticache()
            .describe_replication_groups(&DescribeReplicationGroupsRequest {
                replication_group_id: Some("my-rg".into()),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.replication_groups.len(), 1);
        let rg = &result.replication_groups[0];
        assert_eq!(rg.replication_group_id.as_deref(), Some("my-rg"));
        assert_eq!(rg.status.as_deref(), Some("available"));
        assert_eq!(rg.engine.as_deref(), Some("redis"));
        assert_eq!(rg.at_rest_encryption_enabled, Some(true));
        assert_eq!(rg.transit_encryption_enabled, Some(true));
        assert_eq!(rg.node_groups.len(), 1);
        let ng = &rg.node_groups[0];
        assert_eq!(ng.node_group_id.as_deref(), Some("0001"));
        assert_eq!(ng.node_group_members.len(), 1);
        assert_eq!(
            ng.node_group_members[0].current_role.as_deref(),
            Some("primary")
        );
    }

    #[tokio::test]
    async fn delete_cache_cluster_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DeleteCacheClusterResponse><DeleteCacheClusterResult>
                <CacheCluster>
                    <CacheClusterId>my-cluster</CacheClusterId>
                    <CacheNodeType>cache.t3.micro</CacheNodeType>
                    <Engine>redis</Engine>
                    <EngineVersion>7.1.0</EngineVersion>
                    <CacheClusterStatus>deleting</CacheClusterStatus>
                    <NumCacheNodes>1</NumCacheNodes>
                    <ARN>arn:aws:elasticache:us-east-1:123456789012:cluster:my-cluster</ARN>
                </CacheCluster>
            </DeleteCacheClusterResult></DeleteCacheClusterResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elasticache()
            .delete_cache_cluster(&DeleteCacheClusterRequest {
                cache_cluster_id: "my-cluster".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        let cluster = result.cache_cluster.as_ref().unwrap();
        assert_eq!(cluster.cache_cluster_id.as_deref(), Some("my-cluster"));
        assert_eq!(cluster.cache_cluster_status.as_deref(), Some("deleting"));
        assert_eq!(cluster.engine.as_deref(), Some("redis"));
        assert_eq!(cluster.num_cache_nodes, Some(1));
    }

    #[tokio::test]
    async fn delete_replication_group_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DeleteReplicationGroupResponse><DeleteReplicationGroupResult>
                <ReplicationGroup>
                    <ReplicationGroupId>my-rg</ReplicationGroupId>
                    <Description>Test RG</Description>
                    <Status>deleting</Status>
                    <CacheNodeType>cache.t3.micro</CacheNodeType>
                    <Engine>redis</Engine>
                    <ARN>arn:aws:elasticache:us-east-1:123456789012:replicationgroup:my-rg</ARN>
                    <MemberClusters>
                        <MemberCluster>my-rg-001</MemberCluster>
                    </MemberClusters>
                </ReplicationGroup>
            </DeleteReplicationGroupResult></DeleteReplicationGroupResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elasticache()
            .delete_replication_group(&DeleteReplicationGroupRequest {
                replication_group_id: "my-rg".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        let rg = result.replication_group.as_ref().unwrap();
        assert_eq!(rg.replication_group_id.as_deref(), Some("my-rg"));
        assert_eq!(rg.status.as_deref(), Some("deleting"));
        assert_eq!(rg.engine.as_deref(), Some("redis"));
        assert_eq!(rg.member_clusters.len(), 1);
        assert_eq!(rg.member_clusters[0], "my-rg-001");
    }

    #[tokio::test]
    async fn create_snapshot_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<CreateSnapshotResponse><CreateSnapshotResult>
                <Snapshot>
                    <SnapshotName>my-snapshot</SnapshotName>
                    <CacheClusterId>my-cluster-001</CacheClusterId>
                    <SnapshotStatus>creating</SnapshotStatus>
                    <SnapshotSource>manual</SnapshotSource>
                    <CacheNodeType>cache.t3.micro</CacheNodeType>
                    <Engine>redis</Engine>
                    <EngineVersion>7.1.0</EngineVersion>
                    <NumCacheNodes>1</NumCacheNodes>
                    <ARN>arn:aws:elasticache:us-east-1:123456789012:snapshot:my-snapshot</ARN>
                </Snapshot>
            </CreateSnapshotResult></CreateSnapshotResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elasticache()
            .create_snapshot(&CreateSnapshotRequest {
                snapshot_name: "my-snapshot".into(),
                cache_cluster_id: Some("my-cluster-001".into()),
                ..Default::default()
            })
            .await
            .unwrap();
        let snap = result.snapshot.as_ref().unwrap();
        assert_eq!(snap.snapshot_name.as_deref(), Some("my-snapshot"));
        assert_eq!(snap.snapshot_status.as_deref(), Some("creating"));
        assert_eq!(snap.snapshot_source.as_deref(), Some("manual"));
        assert_eq!(snap.engine.as_deref(), Some("redis"));
        assert_eq!(snap.cache_cluster_id.as_deref(), Some("my-cluster-001"));
        assert_eq!(snap.num_cache_nodes, Some(1));
    }
}
