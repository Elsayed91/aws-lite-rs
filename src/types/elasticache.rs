//! Types for the Amazon ElastiCache API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Represents the input of a DescribeCacheClusters operation.
///
/// **AWS API**: `elasticache.v1.DescribeCacheClustersMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//DescribeCacheClustersMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeCacheClustersRequest {
    /// The user-supplied cluster identifier. If this parameter is specified, only information
    /// about that specific cluster is returned. This parameter isn't case sensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,

    /// The maximum number of records to include in the response. If more records exist than the
    /// specified MaxRecords value, a marker is included in the response so that the remaining
    /// results can be retrieved. Default: 100 Constraints: minimum 20; maximum 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,

    /// An optional marker returned from a prior request. Use this marker for pagination of
    /// results from this operation. If this parameter is specified, the response includes only
    /// records beyond the marker, up to the value specified by MaxRecords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// An optional flag that can be included in the DescribeCacheCluster request to retrieve
    /// information about the individual cache nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_cache_node_info: Option<bool>,

    /// An optional flag that can be included in the DescribeCacheCluster request to show only
    /// nodes (API/CLI: clusters) that are not members of a replication group. In practice, this
    /// means Memcached and single node Valkey or Redis OSS clusters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_cache_clusters_not_in_replication_groups: Option<bool>,
}

impl DescribeCacheClustersRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cache_cluster_id: Some("test-cache_cluster_id".into()),
            max_records: Some(100),
            marker: Some("test-marker".into()),
            show_cache_node_info: Some(false),
            show_cache_clusters_not_in_replication_groups: Some(false),
        }
    }
}

/// Represents the output of a DescribeCacheClusters operation.
///
/// **AWS API**: `elasticache.v1.CacheClusterMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//CacheClusterMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeCacheClustersResponse {
    /// A list of clusters. Each item in the list contains detailed information about one
    /// cluster.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cache_clusters: Vec<CacheCluster>,

    /// Provides an identifier to allow retrieval of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeCacheClustersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cache_clusters: vec![],
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains all of the attributes of a specific cluster.
///
/// **AWS API**: `elasticache.v1.CacheCluster`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//CacheCluster>
///
/// ## Coverage
/// 20 of 33 fields included.
/// Omitted fields:
/// - `ClientDownloadLandingPage` — not selected in manifest
/// - `PreferredOutpostArn` — not selected in manifest
/// - `PendingModifiedValues` — not selected in manifest
/// - `NotificationConfiguration` — not selected in manifest
/// - `CacheSecurityGroups` — not selected in manifest
/// - `CacheParameterGroup` — not selected in manifest
/// - `SecurityGroups` — not selected in manifest
/// - `AuthTokenLastModifiedDate` — not selected in manifest
/// - `ReplicationGroupLogDeliveryEnabled` — not selected in manifest
/// - `LogDeliveryConfigurations` — not selected in manifest
/// - `NetworkType` — not selected in manifest
/// - `IpDiscovery` — not selected in manifest
/// - `TransitEncryptionMode` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CacheCluster {
    /// The user-supplied identifier of the cluster. This identifier is a unique key that
    /// identifies a cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,

    /// The name of the compute and memory capacity node type for the cluster. The following
    /// node types are supported by ElastiCache. Generally speaking, the current generation
    /// types provide more memory and computational power at lower cost when compared to their
    /// equivalent previous generation counterparts. General purpose: Current generation: M7g
    /// node types: cache.m7g.large, cache.m7g.xlarge, cache.m7g.2xlarge, cache.m7g.4xlarge,
    /// cache.m7g.8xlarge, cache.m7g.12xlarge, cache.m7g.16xlarge For region availability, see
    /// Supported Node Types M6g node types (available only for Redis OSS engine version 5.0.6
    /// onward and for Memcached engine version 1.5.16 onward): cache.m6g.large,
    /// cache.m6g.xlarge, cache.m6g.2xlarge, cache.m6g.4xlarge, cache.m6g.8xlarge,
    /// cache.m6g.12xlarge, cache.m6g.16xlarge M5 node types: cache.m5.large, cache.m5.xlarge,
    /// cache.m5.2xlarge, cache.m5.4xlarge, cache.m5.12xlarge, cache.m5.24xlarge M4 node types:
    /// cache.m4.large, cache.m4.xlarge, cache.m4.2xlarge, cache.m4.4xlarge, cache.m4.10xlarge
    /// T4g node types (available only for Redis OSS engine version 5.0.6 onward and Memcached
    /// engine version 1.5.16 onward): cache.t4g.micro, cache.t4g.small, cache.t4g.medium T3
    /// node types: cache.t3.micro, cache.t3.small, cache.t3.medium T2 node types:
    /// cache.t2.micro, cache.t2.small, cache.t2.medium Previous generation: (not recommended.
    /// Existing clusters are still supported but creation of new clusters is not supported for
    /// these types.) T1 node types: cache.t1.micro M1 node types: cache.m1.small,
    /// cache.m1.medium, cache.m1.large, cache.m1.xlarge M3 node types: cache.m3.medium,
    /// cache.m3.large, cache.m3.xlarge, cache.m3.2xlarge Compute optimized: Previous
    /// generation: (not recommended. Existing clusters are still supported but creation of new
    /// clusters is not supported for these types.) C1 node types: cache.c1.xlarge Memory
    /// optimized: Current generation: R7g node types: cache.r7g.large, cache.r7g.xlarge,
    /// cache.r7g.2xlarge, cache.r7g.4xlarge, cache.r7g.8xlarge, cache.r7g.12xlarge,
    /// cache.r7g.16xlarge For region availability, see Supported Node Types R6g node types
    /// (available only for Redis OSS engine version 5.0.6 onward and for Memcached engine
    /// version 1.5.16 onward): cache.r6g.large, cache.r6g.xlarge, cache.r6g.2xlarge,
    /// cache.r6g.4xlarge, cache.r6g.8xlarge, cache.r6g.12xlarge, cache.r6g.16xlarge R5 node
    /// types: cache.r5.large, cache.r5.xlarge, cache.r5.2xlarge, cache.r5.4xlarge,
    /// cache.r5.12xlarge, cache.r5.24xlarge R4 node types: cache.r4.large, cache.r4.xlarge,
    /// cache.r4.2xlarge, cache.r4.4xlarge, cache.r4.8xlarge, cache.r4.16xlarge Previous
    /// generation: (not recommended. Existing clusters are still supported but creation of new
    /// clusters is not supported for these types.) M2 node types: cache.m2.xlarge,
    /// cache.m2.2xlarge, cache.m2.4xlarge R3 node types: cache.r3.large, cache.r3.xlarge,
    /// cache.r3.2xlarge, cache.r3.4xlarge, cache.r3.8xlarge Additional node type info All
    /// current generation instance types are created in Amazon VPC by default. Valkey or Redis
    /// OSS append-only files (AOF) are not supported for T1 or T2 instances. Valkey or Redis
    /// OSS Multi-AZ with automatic failover is not supported on T1 instances. The configuration
    /// variables appendonly and appendfsync are not supported on Valkey, or on Redis OSS
    /// version 2.8.22 and later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,

    /// The name of the cache engine (memcached or redis) to be used for this cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,

    /// The version of the cache engine that is used in this cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,

    /// The current state of this cluster, one of the following values: available, creating,
    /// deleted, deleting, incompatible-network, modifying, rebooting cluster nodes, restore-
    /// failed, or snapshotting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_status: Option<String>,

    /// The number of cache nodes in the cluster. For clusters running Valkey or Redis OSS, this
    /// value must be 1. For clusters running Memcached, this value must be between 1 and 40.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_cache_nodes: Option<i32>,

    /// The date and time when the cluster was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_create_time: Option<String>,

    /// Specifies the weekly time range during which maintenance on the cluster is performed. It
    /// is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The
    /// minimum maintenance window is a 60 minute period. Valid values for ddd are: sun mon tue
    /// wed thu fri sat Example: sun:23:00-mon:01:30
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,

    /// The name of the Availability Zone in which the cluster is located or "Multiple" if the
    /// cache nodes are located in different Availability Zones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zone: Option<String>,

    /// The name of the cache subnet group associated with the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<String>,

    /// If you are running Valkey or Redis OSS engine version 6.0 or later, set this parameter
    /// to yes if you want to opt-in to the next auto minor version upgrade campaign. This
    /// parameter is disabled for previous versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,

    /// The replication group to which this cluster belongs. If this field is empty, the cluster
    /// is not associated with any replication group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,

    /// The number of days for which ElastiCache retains automatic cluster snapshots before
    /// deleting them. For example, if you set SnapshotRetentionLimit to 5, a snapshot that was
    /// taken today is retained for 5 days before being deleted. If the value of
    /// SnapshotRetentionLimit is set to zero (0), backups are turned off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,

    /// The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of
    /// your cluster. Example: 05:00-09:00
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,

    /// A flag that enables using an AuthToken (password) when issuing Valkey or Redis OSS
    /// commands. Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_enabled: Option<bool>,

    /// A flag that enables in-transit encryption when set to true. Required: Only available
    /// when creating a replication group in an Amazon VPC using Redis OSS version 3.2.6, 4.x or
    /// later. Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,

    /// A flag that enables encryption at-rest when set to true. You cannot modify the value of
    /// AtRestEncryptionEnabled after the cluster is created. To enable at-rest encryption on a
    /// cluster you must set AtRestEncryptionEnabled to true when you create a cluster.
    /// Required: Only available when creating a replication group in an Amazon VPC using Redis
    /// OSS version 3.2.6, 4.x or later. Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<bool>,

    /// The ARN (Amazon Resource Name) of the cache cluster.
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// A list of cache nodes that are members of the cluster.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cache_nodes: Vec<CacheNode>,

    /// Represents a Memcached cluster endpoint which can be used by an application to connect
    /// to any node in the cluster. The configuration endpoint will always have .cfg in it.
    /// Example: mem-3.9dvc4r.cfg.usw2.cache.amazonaws.com:11211
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_endpoint: Option<Endpoint>,
}

impl CacheCluster {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cache_cluster_id: Some("test-cache_cluster_id".into()),
            cache_node_type: Some("test-cache_node_type".into()),
            engine: Some("test-engine".into()),
            engine_version: Some("test-engine_version".into()),
            cache_cluster_status: Some("test-cache_cluster_status".into()),
            num_cache_nodes: Some(100),
            cache_cluster_create_time: Some("test-cache_cluster_create_time".into()),
            preferred_maintenance_window: Some("test-preferred_maintenance_window".into()),
            preferred_availability_zone: Some("test-preferred_availability_zone".into()),
            cache_subnet_group_name: Some("test-cache_subnet_group_name".into()),
            auto_minor_version_upgrade: Some(false),
            replication_group_id: Some("test-replication_group_id".into()),
            snapshot_retention_limit: Some(100),
            snapshot_window: Some("test-snapshot_window".into()),
            auth_token_enabled: Some(false),
            transit_encryption_enabled: Some(false),
            at_rest_encryption_enabled: Some(false),
            arn: Some("test-arn".into()),
            cache_nodes: vec![],
            configuration_endpoint: Some(Endpoint::fixture()),
        }
    }
}

/// Represents an individual cache node within a cluster. Each cache node runs its own instance
/// of the cluster's protocol-compliant caching software
/// - either Memcached, Valkey or Redis OSS. The following node types are supported by
///   ElastiCache. Generally speaking, the current generation types provide more memory and
///   computational power at lower cost when compared to their equivalent previous generation
///   counterparts. General purpose: Current generation: M7g node types: cache.m7g.large,
///   cache.m7g.xlarge, cache.m7g.2xlarge, cache.m7g.4xlarge, cache.m7g.8xlarge,
///   cache.m7g.12xlarge, cache.m7g.16xlarge For region availability, see Supported Node Types
///   M6g node types (available only for Redis OSS engine version 5.0.6 onward and for Memcached
///   engine version 1.5.16 onward): cache.m6g.large, cache.m6g.xlarge, cache.m6g.2xlarge,
///   cache.m6g.4xlarge, cache.m6g.8xlarge, cache.m6g.12xlarge, cache.m6g.16xlarge M5 node
///   types: cache.m5.large, cache.m5.xlarge, cache.m5.2xlarge, cache.m5.4xlarge,
///   cache.m5.12xlarge, cache.m5.24xlarge M4 node types: cache.m4.large, cache.m4.xlarge,
///   cache.m4.2xlarge, cache.m4.4xlarge, cache.m4.10xlarge T4g node types (available only for
///   Redis OSS engine version 5.0.6 onward and Memcached engine version 1.5.16 onward):
///   cache.t4g.micro, cache.t4g.small, cache.t4g.medium T3 node types: cache.t3.micro,
///   cache.t3.small, cache.t3.medium T2 node types: cache.t2.micro, cache.t2.small,
///   cache.t2.medium Previous generation: (not recommended. Existing clusters are still
///   supported but creation of new clusters is not supported for these types.) T1 node types:
///   cache.t1.micro M1 node types: cache.m1.small, cache.m1.medium, cache.m1.large,
///   cache.m1.xlarge M3 node types: cache.m3.medium, cache.m3.large, cache.m3.xlarge,
///   cache.m3.2xlarge Compute optimized: Previous generation: (not recommended. Existing
///   clusters are still supported but creation of new clusters is not supported for these
///   types.) C1 node types: cache.c1.xlarge Memory optimized: Current generation: R7g node
///   types: cache.r7g.large, cache.r7g.xlarge, cache.r7g.2xlarge, cache.r7g.4xlarge,
///   cache.r7g.8xlarge, cache.r7g.12xlarge, cache.r7g.16xlarge For region availability, see
///   Supported Node Types R6g node types (available only for Redis OSS engine version 5.0.6
///   onward and for Memcached engine version 1.5.16 onward): cache.r6g.large, cache.r6g.xlarge,
///   cache.r6g.2xlarge, cache.r6g.4xlarge, cache.r6g.8xlarge, cache.r6g.12xlarge,
///   cache.r6g.16xlarge R5 node types: cache.r5.large, cache.r5.xlarge, cache.r5.2xlarge,
///   cache.r5.4xlarge, cache.r5.12xlarge, cache.r5.24xlarge R4 node types: cache.r4.large,
///   cache.r4.xlarge, cache.r4.2xlarge, cache.r4.4xlarge, cache.r4.8xlarge, cache.r4.16xlarge
///   Previous generation: (not recommended. Existing clusters are still supported but creation
///   of new clusters is not supported for these types.) M2 node types: cache.m2.xlarge,
///   cache.m2.2xlarge, cache.m2.4xlarge R3 node types: cache.r3.large, cache.r3.xlarge,
///   cache.r3.2xlarge, cache.r3.4xlarge, cache.r3.8xlarge Additional node type info All current
///   generation instance types are created in Amazon VPC by default. Valkey or Redis OSS
///   append-only files (AOF) are not supported for T1 or T2 instances. Valkey or Redis OSS
///   Multi-AZ with automatic failover is not supported on T1 instances. The configuration
///   variables appendonly and appendfsync are not supported on Valkey, or on Redis OSS version
///   2.8.22 and later.
///
/// **AWS API**: `elasticache.v1.CacheNode`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//CacheNode>
///
/// ## Coverage
/// 5 of 8 fields included.
/// Omitted fields:
/// - `SourceCacheNodeId` — not selected in manifest
/// - `CustomerAvailabilityZone` — not selected in manifest
/// - `CustomerOutpostArn` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CacheNode {
    /// The cache node identifier. A node ID is a numeric identifier (0001, 0002, etc.). The
    /// combination of cluster ID and node ID uniquely identifies every cache node used in a
    /// customer's Amazon account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_id: Option<String>,

    /// The current state of this cache node, one of the following values: available, creating,
    /// rebooting, or deleting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_status: Option<String>,

    /// The date and time when the cache node was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_create_time: Option<String>,

    /// The hostname for connecting to this cache node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,

    /// The status of the parameter group applied to this cache node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_status: Option<String>,
}

impl CacheNode {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cache_node_id: Some("test-cache_node_id".into()),
            cache_node_status: Some("test-cache_node_status".into()),
            cache_node_create_time: Some("test-cache_node_create_time".into()),
            endpoint: Some(Endpoint::fixture()),
            parameter_group_status: Some("test-parameter_group_status".into()),
        }
    }
}

/// Represents the information required for client programs to connect to a cache node. This
/// value is read-only.
///
/// **AWS API**: `elasticache.v1.Endpoint`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//Endpoint>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Endpoint {
    /// The DNS hostname of the cache node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// The port number that the cache engine is listening on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl Endpoint {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            address: Some("test-address".into()),
            port: Some(100),
        }
    }
}

/// Represents the input of a DescribeReplicationGroups operation.
///
/// **AWS API**: `elasticache.v1.DescribeReplicationGroupsMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//DescribeReplicationGroupsMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeReplicationGroupsRequest {
    /// The identifier for the replication group to be described. This parameter is not case
    /// sensitive. If you do not specify this parameter, information about all replication
    /// groups is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,

    /// The maximum number of records to include in the response. If more records exist than the
    /// specified MaxRecords value, a marker is included in the response so that the remaining
    /// results can be retrieved. Default: 100 Constraints: minimum 20; maximum 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,

    /// An optional marker returned from a prior request. Use this marker for pagination of
    /// results from this operation. If this parameter is specified, the response includes only
    /// records beyond the marker, up to the value specified by MaxRecords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeReplicationGroupsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            replication_group_id: Some("test-replication_group_id".into()),
            max_records: Some(100),
            marker: Some("test-marker".into()),
        }
    }
}

/// Represents the output of a DescribeReplicationGroups operation.
///
/// **AWS API**: `elasticache.v1.ReplicationGroupMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//ReplicationGroupMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeReplicationGroupsResponse {
    /// A list of replication groups. Each item in the list contains detailed information about
    /// one replication group.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub replication_groups: Vec<ReplicationGroup>,

    /// Provides an identifier to allow retrieval of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeReplicationGroupsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            replication_groups: vec![],
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains all of the attributes of a specific Valkey or Redis OSS replication group.
///
/// **AWS API**: `elasticache.v1.ReplicationGroup`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//ReplicationGroup>
///
/// ## Coverage
/// 19 of 32 fields included.
/// Omitted fields:
/// - `GlobalReplicationGroupInfo` — not selected in manifest
/// - `PendingModifiedValues` — not selected in manifest
/// - `SnapshottingClusterId` — not selected in manifest
/// - `AuthTokenLastModifiedDate` — not selected in manifest
/// - `MemberClustersOutpostArns` — not selected in manifest
/// - `UserGroupIds` — not selected in manifest
/// - `LogDeliveryConfigurations` — not selected in manifest
/// - `DataTiering` — not selected in manifest
/// - `AutoMinorVersionUpgrade` — not selected in manifest
/// - `NetworkType` — not selected in manifest
/// - `IpDiscovery` — not selected in manifest
/// - `TransitEncryptionMode` — not selected in manifest
/// - `ClusterMode` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationGroup {
    /// The identifier for the replication group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,

    /// The user supplied description of the replication group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The current state of this replication group
    /// - creating, available, modifying, deleting, create-failed, snapshotting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// A list of node groups in this replication group. For Valkey or Redis OSS (cluster mode
    /// disabled) replication groups, this is a single-element list. For Valkey or Redis OSS
    /// (cluster mode enabled) replication groups, the list contains an entry for each node
    /// group (shard).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_groups: Vec<NodeGroup>,

    /// The names of all the cache clusters that are part of this replication group.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub member_clusters: Vec<String>,

    /// The name of the compute and memory capacity node type for each node in the replication
    /// group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,

    /// Indicates the status of automatic failover for this Valkey or Redis OSS replication
    /// group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_failover: Option<String>,

    /// A flag indicating if you have Multi-AZ enabled to enhance fault tolerance. For more
    /// information, see Minimizing Downtime: Multi-AZ
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<String>,

    /// The configuration endpoint for this replication group. Use the configuration endpoint to
    /// connect to this replication group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_endpoint: Option<Endpoint>,

    /// The number of days for which ElastiCache retains automatic cluster snapshots before
    /// deleting them. For example, if you set SnapshotRetentionLimit to 5, a snapshot that was
    /// taken today is retained for 5 days before being deleted. If the value of
    /// SnapshotRetentionLimit is set to zero (0), backups are turned off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,

    /// The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of
    /// your node group (shard). Example: 05:00-09:00 If you do not specify this parameter,
    /// ElastiCache automatically chooses an appropriate time range. This parameter is only
    /// valid if the Engine parameter is redis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,

    /// A flag indicating whether or not this replication group is cluster enabled; i.e.,
    /// whether its data can be partitioned across multiple shards (API/CLI: node groups). Valid
    /// values: true | false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_enabled: Option<bool>,

    /// A flag that enables using an AuthToken (password) when issuing Valkey or Redis OSS
    /// commands. Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_enabled: Option<bool>,

    /// A flag that enables in-transit encryption when set to true. Required: Only available
    /// when creating a replication group in an Amazon VPC using Redis OSS version 3.2.6, 4.x or
    /// later. Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,

    /// A flag that enables encryption at-rest when set to true. You cannot modify the value of
    /// AtRestEncryptionEnabled after the cluster is created. To enable encryption at-rest on a
    /// cluster you must set AtRestEncryptionEnabled to true when you create a cluster.
    /// Required: Only available when creating a replication group in an Amazon VPC using Redis
    /// OSS version 3.2.6, 4.x or later. Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<bool>,

    /// The ID of the KMS key used to encrypt the disk in the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    /// The ARN (Amazon Resource Name) of the replication group.
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The date and time when the cluster was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_create_time: Option<String>,

    /// The engine used in a replication group. The options are redis, memcached or valkey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

impl ReplicationGroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            replication_group_id: Some("test-replication_group_id".into()),
            description: Some("test-description".into()),
            status: Some("test-status".into()),
            node_groups: vec![],
            member_clusters: vec![],
            cache_node_type: Some("test-cache_node_type".into()),
            automatic_failover: Some("test-automatic_failover".into()),
            multi_az: Some("test-multi_az".into()),
            configuration_endpoint: Some(Endpoint::fixture()),
            snapshot_retention_limit: Some(100),
            snapshot_window: Some("test-snapshot_window".into()),
            cluster_enabled: Some(false),
            auth_token_enabled: Some(false),
            transit_encryption_enabled: Some(false),
            at_rest_encryption_enabled: Some(false),
            kms_key_id: Some("test-kms_key_id".into()),
            arn: Some("test-arn".into()),
            replication_group_create_time: Some("test-replication_group_create_time".into()),
            engine: Some("test-engine".into()),
        }
    }
}

/// Represents a collection of cache nodes in a replication group. One node in the node group is
/// the read/write primary node. All the other nodes are read-only Replica nodes.
///
/// **AWS API**: `elasticache.v1.NodeGroup`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//NodeGroup>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeGroup {
    /// The identifier for the node group (shard). A Valkey or Redis OSS (cluster mode disabled)
    /// replication group contains only 1 node group; therefore, the node group ID is 0001. A
    /// Valkey or Redis OSS (cluster mode enabled) replication group contains 1 to 90 node
    /// groups numbered 0001 to 0090. Optionally, the user can provide the id for a node group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_id: Option<String>,

    /// The current state of this replication group
    /// - creating, available, modifying, deleting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The endpoint of the primary node in this node group (shard).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_endpoint: Option<Endpoint>,

    /// The endpoint of the replica nodes in this node group (shard). This value is read-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_endpoint: Option<Endpoint>,

    /// The keyspace for this node group (shard).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<String>,

    /// A list containing information about individual nodes within the node group (shard).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_group_members: Vec<NodeGroupMember>,
}

impl NodeGroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            node_group_id: Some("test-node_group_id".into()),
            status: Some("test-status".into()),
            primary_endpoint: Some(Endpoint::fixture()),
            reader_endpoint: Some(Endpoint::fixture()),
            slots: Some("test-slots".into()),
            node_group_members: vec![],
        }
    }
}

/// Represents a single node within a node group (shard).
///
/// **AWS API**: `elasticache.v1.NodeGroupMember`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//NodeGroupMember>
///
/// ## Coverage
/// 5 of 6 fields included.
/// Omitted fields:
/// - `PreferredOutpostArn` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeGroupMember {
    /// The ID of the cluster to which the node belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,

    /// The ID of the node within its cluster. A node ID is a numeric identifier (0001, 0002,
    /// etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_id: Option<String>,

    /// The information required for client programs to connect to a node for read operations.
    /// The read endpoint is only applicable on Valkey or Redis OSS (cluster mode disabled)
    /// clusters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_endpoint: Option<Endpoint>,

    /// The name of the Availability Zone in which the node is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zone: Option<String>,

    /// The role that is currently assigned to the node
    /// - primary or replica. This member is only applicable for Valkey or Redis OSS (cluster
    ///   mode disabled) replication groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_role: Option<String>,
}

impl NodeGroupMember {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cache_cluster_id: Some("test-cache_cluster_id".into()),
            cache_node_id: Some("test-cache_node_id".into()),
            read_endpoint: Some(Endpoint::fixture()),
            preferred_availability_zone: Some("test-preferred_availability_zone".into()),
            current_role: Some("test-current_role".into()),
        }
    }
}

/// Represents the input of a DeleteCacheCluster operation.
///
/// **AWS API**: `elasticache.v1.DeleteCacheClusterMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//DeleteCacheClusterMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteCacheClusterRequest {
    /// The cluster identifier for the cluster to be deleted. This parameter is not case
    /// sensitive.
    pub cache_cluster_id: String,

    /// The user-supplied name of a final cluster snapshot. This is the unique name that
    /// identifies the snapshot. ElastiCache creates the snapshot, and then deletes the cluster
    /// immediately afterward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_snapshot_identifier: Option<String>,
}

impl DeleteCacheClusterRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cache_cluster_id: "test-cache_cluster_id".into(),
            final_snapshot_identifier: Some("test-final_snapshot_identifier".into()),
        }
    }
}

///
/// **AWS API**: `elasticache.v1.DeleteCacheClusterResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//DeleteCacheClusterResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteCacheClusterResponse {
    /// The `CacheCluster` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster: Option<CacheCluster>,
}

impl DeleteCacheClusterResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cache_cluster: Some(CacheCluster::fixture()),
        }
    }
}

/// Represents the input of a DeleteReplicationGroup operation.
///
/// **AWS API**: `elasticache.v1.DeleteReplicationGroupMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//DeleteReplicationGroupMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteReplicationGroupRequest {
    /// The identifier for the cluster to be deleted. This parameter is not case sensitive.
    pub replication_group_id: String,

    /// If set to true, all of the read replicas are deleted, but the primary node is retained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_primary_cluster: Option<bool>,

    /// The name of a final node group (shard) snapshot. ElastiCache creates the snapshot from
    /// the primary node in the cluster, rather than one of the replicas; this is to ensure that
    /// it captures the freshest data. After the final snapshot is taken, the replication group
    /// is immediately deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_snapshot_identifier: Option<String>,
}

impl DeleteReplicationGroupRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            replication_group_id: "test-replication_group_id".into(),
            retain_primary_cluster: Some(false),
            final_snapshot_identifier: Some("test-final_snapshot_identifier".into()),
        }
    }
}

///
/// **AWS API**: `elasticache.v1.DeleteReplicationGroupResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//DeleteReplicationGroupResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteReplicationGroupResponse {
    /// The `ReplicationGroup` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}

impl DeleteReplicationGroupResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            replication_group: Some(ReplicationGroup::fixture()),
        }
    }
}

/// Represents the input of a CreateSnapshot operation.
///
/// **AWS API**: `elasticache.v1.CreateSnapshotMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//CreateSnapshotMessage>
///
/// ## Coverage
/// 4 of 5 fields included.
/// Omitted fields:
/// - `Tags` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSnapshotRequest {
    /// A name for the snapshot being created.
    pub snapshot_name: String,

    /// The identifier of an existing replication group. The snapshot is created from this
    /// replication group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,

    /// The identifier of an existing cluster. The snapshot is created from this cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,

    /// The ID of the KMS key used to encrypt the snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

impl CreateSnapshotRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshot_name: "test-snapshot_name".into(),
            replication_group_id: Some("test-replication_group_id".into()),
            cache_cluster_id: Some("test-cache_cluster_id".into()),
            kms_key_id: Some("test-kms_key_id".into()),
        }
    }
}

///
/// **AWS API**: `elasticache.v1.CreateSnapshotResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//CreateSnapshotResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSnapshotResponse {
    /// The `Snapshot` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

impl CreateSnapshotResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshot: Some(Snapshot::fixture()),
        }
    }
}

/// Represents a copy of an entire Valkey or Redis OSS cluster as of the time when the snapshot
/// was taken.
///
/// **AWS API**: `elasticache.v1.Snapshot`
/// **Reference**: <https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference//Snapshot>
///
/// ## Coverage
/// 18 of 28 fields included.
/// Omitted fields:
/// - `ReplicationGroupDescription` — not selected in manifest
/// - `PreferredOutpostArn` — not selected in manifest
/// - `TopicArn` — not selected in manifest
/// - `Port` — not selected in manifest
/// - `CacheParameterGroupName` — not selected in manifest
/// - `CacheSubnetGroupName` — not selected in manifest
/// - `VpcId` — not selected in manifest
/// - `AutoMinorVersionUpgrade` — not selected in manifest
/// - `NodeSnapshots` — not selected in manifest
/// - `DataTiering` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Snapshot {
    /// The name of a snapshot. For an automatic snapshot, the name is system-generated. For a
    /// manual snapshot, this is the user-provided name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,

    /// The unique identifier of the source replication group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,

    /// The user-supplied identifier of the source cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,

    /// The status of the snapshot. Valid values: creating | available | restoring | copying |
    /// deleting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_status: Option<String>,

    /// Indicates whether the snapshot is from an automatic backup (automated) or was created
    /// manually (manual).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_source: Option<String>,

    /// The name of the compute and memory capacity node type for the source cluster. The
    /// following node types are supported by ElastiCache. Generally speaking, the current
    /// generation types provide more memory and computational power at lower cost when compared
    /// to their equivalent previous generation counterparts. General purpose: Current
    /// generation: M7g node types: cache.m7g.large, cache.m7g.xlarge, cache.m7g.2xlarge,
    /// cache.m7g.4xlarge, cache.m7g.8xlarge, cache.m7g.12xlarge, cache.m7g.16xlarge For region
    /// availability, see Supported Node Types M6g node types (available only for Redis OSS
    /// engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward):
    /// cache.m6g.large, cache.m6g.xlarge, cache.m6g.2xlarge, cache.m6g.4xlarge,
    /// cache.m6g.8xlarge, cache.m6g.12xlarge, cache.m6g.16xlarge M5 node types: cache.m5.large,
    /// cache.m5.xlarge, cache.m5.2xlarge, cache.m5.4xlarge, cache.m5.12xlarge,
    /// cache.m5.24xlarge M4 node types: cache.m4.large, cache.m4.xlarge, cache.m4.2xlarge,
    /// cache.m4.4xlarge, cache.m4.10xlarge T4g node types (available only for Redis OSS engine
    /// version 5.0.6 onward and Memcached engine version 1.5.16 onward): cache.t4g.micro,
    /// cache.t4g.small, cache.t4g.medium T3 node types: cache.t3.micro, cache.t3.small,
    /// cache.t3.medium T2 node types: cache.t2.micro, cache.t2.small, cache.t2.medium Previous
    /// generation: (not recommended. Existing clusters are still supported but creation of new
    /// clusters is not supported for these types.) T1 node types: cache.t1.micro M1 node types:
    /// cache.m1.small, cache.m1.medium, cache.m1.large, cache.m1.xlarge M3 node types:
    /// cache.m3.medium, cache.m3.large, cache.m3.xlarge, cache.m3.2xlarge Compute optimized:
    /// Previous generation: (not recommended. Existing clusters are still supported but
    /// creation of new clusters is not supported for these types.) C1 node types:
    /// cache.c1.xlarge Memory optimized: Current generation: R7g node types: cache.r7g.large,
    /// cache.r7g.xlarge, cache.r7g.2xlarge, cache.r7g.4xlarge, cache.r7g.8xlarge,
    /// cache.r7g.12xlarge, cache.r7g.16xlarge For region availability, see Supported Node Types
    /// R6g node types (available only for Redis OSS engine version 5.0.6 onward and for
    /// Memcached engine version 1.5.16 onward): cache.r6g.large, cache.r6g.xlarge,
    /// cache.r6g.2xlarge, cache.r6g.4xlarge, cache.r6g.8xlarge, cache.r6g.12xlarge,
    /// cache.r6g.16xlarge R5 node types: cache.r5.large, cache.r5.xlarge, cache.r5.2xlarge,
    /// cache.r5.4xlarge, cache.r5.12xlarge, cache.r5.24xlarge R4 node types: cache.r4.large,
    /// cache.r4.xlarge, cache.r4.2xlarge, cache.r4.4xlarge, cache.r4.8xlarge, cache.r4.16xlarge
    /// Previous generation: (not recommended. Existing clusters are still supported but
    /// creation of new clusters is not supported for these types.) M2 node types:
    /// cache.m2.xlarge, cache.m2.2xlarge, cache.m2.4xlarge R3 node types: cache.r3.large,
    /// cache.r3.xlarge, cache.r3.2xlarge, cache.r3.4xlarge, cache.r3.8xlarge Additional node
    /// type info All current generation instance types are created in Amazon VPC by default.
    /// Valkey or Redis OSS append-only files (AOF) are not supported for T1 or T2 instances.
    /// Valkey or Redis OSS Multi-AZ with automatic failover is not supported on T1 instances.
    /// The configuration variables appendonly and appendfsync are not supported on Valkey, or
    /// on Redis OSS version 2.8.22 and later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,

    /// The name of the cache engine (memcached or redis) used by the source cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,

    /// The version of the cache engine version that is used by the source cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,

    /// The number of cache nodes in the source cluster. For clusters running Valkey or Redis
    /// OSS, this value must be 1. For clusters running Memcached, this value must be between 1
    /// and 40.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_cache_nodes: Option<i32>,

    /// The name of the Availability Zone in which the source cluster is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zone: Option<String>,

    /// The date and time when the source cluster was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_create_time: Option<String>,

    /// Specifies the weekly time range during which maintenance on the cluster is performed. It
    /// is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The
    /// minimum maintenance window is a 60 minute period. Valid values for ddd are: sun mon tue
    /// wed thu fri sat Example: sun:23:00-mon:01:30
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,

    /// For an automatic snapshot, the number of days for which ElastiCache retains the snapshot
    /// before deleting it. For manual snapshots, this field reflects the SnapshotRetentionLimit
    /// for the source cluster when the snapshot was created. This field is otherwise ignored:
    /// Manual snapshots do not expire, and can only be deleted using the DeleteSnapshot
    /// operation. Important If the value of SnapshotRetentionLimit is set to zero (0), backups
    /// are turned off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,

    /// The daily time range during which ElastiCache takes daily snapshots of the source
    /// cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,

    /// The number of node groups (shards) in this snapshot. When restoring from a snapshot, the
    /// number of node groups (shards) in the snapshot and in the restored replication group
    /// must be the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_node_groups: Option<i32>,

    /// Indicates the status of automatic failover for the source Valkey or Redis OSS
    /// replication group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_failover: Option<String>,

    /// The ARN (Amazon Resource Name) of the snapshot.
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The ID of the KMS key used to encrypt the snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

impl Snapshot {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshot_name: Some("test-snapshot_name".into()),
            replication_group_id: Some("test-replication_group_id".into()),
            cache_cluster_id: Some("test-cache_cluster_id".into()),
            snapshot_status: Some("test-snapshot_status".into()),
            snapshot_source: Some("test-snapshot_source".into()),
            cache_node_type: Some("test-cache_node_type".into()),
            engine: Some("test-engine".into()),
            engine_version: Some("test-engine_version".into()),
            num_cache_nodes: Some(100),
            preferred_availability_zone: Some("test-preferred_availability_zone".into()),
            cache_cluster_create_time: Some("test-cache_cluster_create_time".into()),
            preferred_maintenance_window: Some("test-preferred_maintenance_window".into()),
            snapshot_retention_limit: Some(100),
            snapshot_window: Some("test-snapshot_window".into()),
            num_node_groups: Some(100),
            automatic_failover: Some("test-automatic_failover".into()),
            arn: Some("test-arn".into()),
            kms_key_id: Some("test-kms_key_id".into()),
        }
    }
}
