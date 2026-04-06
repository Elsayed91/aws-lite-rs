//! Types for the Amazon Redshift API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `redshift.v1.DescribeClustersMessage`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//DescribeClustersMessage>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `TagKeys` — not selected in manifest
/// - `TagValues` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeClustersRequest {
    /// The unique identifier of a cluster whose properties you are requesting. This parameter
    /// is case sensitive. The default is that all clusters defined for an account are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,

    /// The maximum number of response records to return in each call. If the number of
    /// remaining response records exceeds the specified MaxRecords value, a value is returned
    /// in a marker field of the response. You can retrieve the next set of records by retrying
    /// the command with the returned marker value. Default: 100 Constraints: minimum 20,
    /// maximum 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,

    /// An optional parameter that specifies the starting point to return a set of response
    /// records. When the results of a DescribeClusters request exceed the value specified in
    /// MaxRecords, Amazon Web Services returns a value in the Marker field of the response. You
    /// can retrieve the next set of response records by providing the returned marker value in
    /// the Marker parameter and retrying the request. Constraints: You can specify either the
    /// ClusterIdentifier parameter or the Marker parameter, but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeClustersRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_identifier: Some("test-cluster_identifier".into()),
            max_records: Some(100),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains the output from the DescribeClusters action.
///
/// **AWS API**: `redshift.v1.ClustersMessage`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//ClustersMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeClustersResponse {
    /// A list of Cluster objects, where each object describes one cluster.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clusters: Vec<Cluster>,

    /// A value that indicates the starting point for the next set of response records in a
    /// subsequent request. If a value is returned in a response, you can retrieve the next set
    /// of records by providing this returned marker value in the Marker parameter and retrying
    /// the command. If the Marker field is empty, all response records have been retrieved for
    /// the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeClustersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            clusters: vec![],
            marker: Some("test-marker".into()),
        }
    }
}

/// Describes a cluster.
///
/// **AWS API**: `redshift.v1.Cluster`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//Cluster>
///
/// ## Coverage
/// 21 of 63 fields included.
/// Omitted fields:
/// - `ModifyStatus` — not selected in manifest
/// - `DBName` — not selected in manifest
/// - `ClusterSecurityGroups` — not selected in manifest
/// - `VpcSecurityGroups` — not selected in manifest
/// - `ClusterParameterGroups` — not selected in manifest
/// - `ClusterSubnetGroupName` — not selected in manifest
/// - `PendingModifiedValues` — not selected in manifest
/// - `RestoreStatus` — not selected in manifest
/// - `DataTransferProgress` — not selected in manifest
/// - `HsmStatus` — not selected in manifest
/// - `ClusterSnapshotCopyStatus` — not selected in manifest
/// - `ClusterPublicKey` — not selected in manifest
/// - `ClusterNodes` — not selected in manifest
/// - `ElasticIpStatus` — not selected in manifest
/// - `ClusterRevisionNumber` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `IamRoles` — not selected in manifest
/// - `PendingActions` — not selected in manifest
/// - `MaintenanceTrackName` — not selected in manifest
/// - `ElasticResizeNumberOfNodeOptions` — not selected in manifest
/// - `DeferredMaintenanceWindows` — not selected in manifest
/// - `SnapshotScheduleIdentifier` — not selected in manifest
/// - `SnapshotScheduleState` — not selected in manifest
/// - `ExpectedNextSnapshotScheduleTime` — not selected in manifest
/// - `ExpectedNextSnapshotScheduleTimeStatus` — not selected in manifest
/// - `NextMaintenanceWindowStartTime` — not selected in manifest
/// - `ResizeInfo` — not selected in manifest
/// - `AvailabilityZoneRelocationStatus` — not selected in manifest
/// - `AquaConfiguration` — not selected in manifest
/// - `DefaultIamRoleArn` — not selected in manifest
/// - `ReservedNodeExchangeStatus` — not selected in manifest
/// - `CustomDomainName` — not selected in manifest
/// - `CustomDomainCertificateArn` — not selected in manifest
/// - `CustomDomainCertificateExpiryDate` — not selected in manifest
/// - `MasterPasswordSecretArn` — not selected in manifest
/// - `MasterPasswordSecretKmsKeyId` — not selected in manifest
/// - `IpAddressType` — not selected in manifest
/// - `MultiAZ` — not selected in manifest
/// - `MultiAZSecondary` — not selected in manifest
/// - `LakehouseRegistrationStatus` — not selected in manifest
/// - `CatalogArn` — not selected in manifest
/// - `ExtraComputeForAutomaticOptimization` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cluster {
    /// The unique identifier of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,

    /// The node type for the nodes in the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,

    /// The current state of the cluster. Possible values are the following: available
    /// available, prep-for-resize available, resize-cleanup cancelling-resize creating deleting
    /// final-snapshot hardware-failure incompatible-hsm incompatible-network incompatible-
    /// parameters incompatible-restore modifying paused rebooting renaming resizing rotating-
    /// keys storage-full updating-hsm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_status: Option<String>,

    /// The availability status of the cluster for queries. Possible values are the following:
    /// Available
    /// - The cluster is available for queries. Unavailable
    /// - The cluster is not available for queries. Maintenance
    /// - The cluster is intermittently available for queries due to maintenance activities.
    ///   Modifying
    /// - The cluster is intermittently available for queries due to changes that modify the
    ///   cluster. Failed
    /// - The cluster failed and is not available for queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_availability_status: Option<String>,

    /// The admin user name for the cluster. This name is used to connect to the database that
    /// is specified in the DBName parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,

    /// The connection endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,

    /// The date and time that the cluster was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,

    /// The number of compute nodes in the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,

    /// A boolean value that, if true, indicates that the cluster can be accessed from a public
    /// network. Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,

    /// A boolean value that, if true, indicates that data in the cluster is encrypted at rest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,

    /// The identifier of the VPC the cluster is in, if the cluster is in a VPC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// The name of the Availability Zone in which the cluster is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,

    /// The weekly time range, in Universal Coordinated Time (UTC), during which system
    /// maintenance can occur.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,

    /// The version ID of the Amazon Redshift engine that is running on the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,

    /// A boolean value that, if true, indicates that major version upgrades will be applied
    /// automatically to the cluster during the maintenance window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_version_upgrade: Option<bool>,

    /// The number of days that automatic cluster snapshots are retained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<i32>,

    /// The default number of days to retain a manual snapshot. If the value is -1, the snapshot
    /// is retained indefinitely. This setting doesn't change the retention period of existing
    /// snapshots. The value must be either -1 or an integer between 1 and 3,653.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,

    /// The Key Management Service (KMS) key ID of the encryption key used to encrypt data in
    /// the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    /// An option that specifies whether to create the cluster with enhanced VPC routing
    /// enabled. To create a cluster that uses enhanced VPC routing, the cluster must be in a
    /// VPC. For more information, see Enhanced VPC Routing in the Amazon Redshift Cluster
    /// Management Guide. If this option is true, enhanced VPC routing is enabled. Default:
    /// false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,

    /// The namespace Amazon Resource Name (ARN) of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_namespace_arn: Option<String>,

    /// The total storage capacity of the cluster in megabytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_storage_capacity_in_mega_bytes: Option<i64>,
}

impl Cluster {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_identifier: Some("test-cluster_identifier".into()),
            node_type: Some("test-node_type".into()),
            cluster_status: Some("test-cluster_status".into()),
            cluster_availability_status: Some("test-cluster_availability_status".into()),
            master_username: Some("test-master_username".into()),
            endpoint: Some(Endpoint::fixture()),
            cluster_create_time: Some("test-cluster_create_time".into()),
            number_of_nodes: Some(100),
            publicly_accessible: Some(false),
            encrypted: Some(false),
            vpc_id: Some("test-vpc_id".into()),
            availability_zone: Some("test-availability_zone".into()),
            preferred_maintenance_window: Some("test-preferred_maintenance_window".into()),
            cluster_version: Some("test-cluster_version".into()),
            allow_version_upgrade: Some(false),
            automated_snapshot_retention_period: Some(100),
            manual_snapshot_retention_period: Some(100),
            kms_key_id: Some("test-kms_key_id".into()),
            enhanced_vpc_routing: Some(false),
            cluster_namespace_arn: Some("test-cluster_namespace_arn".into()),
            total_storage_capacity_in_mega_bytes: Some(100),
        }
    }
}

/// Describes a connection endpoint.
///
/// **AWS API**: `redshift.v1.Endpoint`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//Endpoint>
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `VpcEndpoints` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Endpoint {
    /// The DNS address of the Cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// The port that the database engine is listening on.
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

/// Describes a pause cluster operation. For example, a scheduled action to run the PauseCluster
/// API operation.
///
/// **AWS API**: `redshift.v1.PauseClusterMessage`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//PauseClusterMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PauseClusterRequest {
    /// The identifier of the cluster to be paused.
    pub cluster_identifier: String,
}

impl PauseClusterRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_identifier: "test-cluster_identifier".into(),
        }
    }
}

///
/// **AWS API**: `redshift.v1.PauseClusterResult`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//PauseClusterResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PauseClusterResponse {
    /// The `Cluster` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

impl PauseClusterResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster: Some(Cluster::fixture()),
        }
    }
}

/// Describes a resume cluster operation. For example, a scheduled action to run the
/// ResumeCluster API operation.
///
/// **AWS API**: `redshift.v1.ResumeClusterMessage`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//ResumeClusterMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResumeClusterRequest {
    /// The identifier of the cluster to be resumed.
    pub cluster_identifier: String,
}

impl ResumeClusterRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_identifier: "test-cluster_identifier".into(),
        }
    }
}

///
/// **AWS API**: `redshift.v1.ResumeClusterResult`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//ResumeClusterResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResumeClusterResponse {
    /// The `Cluster` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

impl ResumeClusterResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster: Some(Cluster::fixture()),
        }
    }
}

/// Describes a resize cluster operation. For example, a scheduled action to run the
/// ResizeCluster API operation.
///
/// **AWS API**: `redshift.v1.ResizeClusterMessage`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//ResizeClusterMessage>
///
/// ## Coverage
/// 5 of 7 fields included.
/// Omitted fields:
/// - `ReservedNodeId` — not selected in manifest
/// - `TargetReservedNodeOfferingId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResizeClusterRequest {
    /// The unique identifier for the cluster to resize.
    pub cluster_identifier: String,

    /// The new cluster type for the specified cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,

    /// The new node type for the nodes you are adding. If not specified, the cluster's current
    /// node type is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,

    /// The new number of nodes for the cluster. If not specified, the cluster's current number
    /// of nodes is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,

    /// A boolean value indicating whether the resize operation is using the classic resize
    /// process. If you don't provide this parameter or set the value to false, the resize type
    /// is elastic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic: Option<bool>,
}

impl ResizeClusterRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_identifier: "test-cluster_identifier".into(),
            cluster_type: Some("test-cluster_type".into()),
            node_type: Some("test-node_type".into()),
            number_of_nodes: Some(100),
            classic: Some(false),
        }
    }
}

///
/// **AWS API**: `redshift.v1.ResizeClusterResult`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//ResizeClusterResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResizeClusterResponse {
    /// The `Cluster` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

impl ResizeClusterResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster: Some(Cluster::fixture()),
        }
    }
}

///
/// **AWS API**: `redshift.v1.DeleteClusterMessage`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//DeleteClusterMessage>
///
/// ## Coverage
/// 3 of 4 fields included.
/// Omitted fields:
/// - `FinalClusterSnapshotRetentionPeriod` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteClusterRequest {
    /// The identifier of the cluster to be deleted. Constraints: Must contain lowercase
    /// characters. Must contain from 1 to 63 alphanumeric characters or hyphens. First
    /// character must be a letter. Cannot end with a hyphen or contain two consecutive hyphens.
    pub cluster_identifier: String,

    /// Determines whether a final snapshot of the cluster is created before Amazon Redshift
    /// deletes the cluster. If true, a final cluster snapshot is not created. If false, a final
    /// cluster snapshot is created before the cluster is deleted. The
    /// FinalClusterSnapshotIdentifier parameter must be specified if SkipFinalClusterSnapshot
    /// is false. Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_cluster_snapshot: Option<bool>,

    /// The identifier of the final snapshot that is to be created immediately before deleting
    /// the cluster. If this parameter is provided, SkipFinalClusterSnapshot must be false.
    /// Constraints: Must be 1 to 255 alphanumeric characters. First character must be a letter.
    /// Cannot end with a hyphen or contain two consecutive hyphens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_cluster_snapshot_identifier: Option<String>,
}

impl DeleteClusterRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_identifier: "test-cluster_identifier".into(),
            skip_final_cluster_snapshot: Some(false),
            final_cluster_snapshot_identifier: Some(
                "test-final_cluster_snapshot_identifier".into(),
            ),
        }
    }
}

///
/// **AWS API**: `redshift.v1.DeleteClusterResult`
/// **Reference**: <https://docs.aws.amazon.com/redshift/latest/APIReference//DeleteClusterResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteClusterResponse {
    /// The `Cluster` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

impl DeleteClusterResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster: Some(Cluster::fixture()),
        }
    }
}
