//! Types for the Amazon EMR API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// This input determines how the ListClusters action filters the list of clusters that it
/// returns.
///
/// **AWS API**: `emr.v1.ListClustersInput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListClustersInput {
    /// The creation date and time beginning value filter for listing clusters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,

    /// The creation date and time end value filter for listing clusters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,

    /// The cluster state filters to apply when listing clusters. Clusters that change state
    /// while this action runs may be not be returned as expected in the list of clusters.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cluster_states: Vec<String>,

    /// The pagination token that indicates the next set of results to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListClustersInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_states: vec![],
            marker: Some("test-marker".into()),
            ..Default::default()
        }
    }
}

/// This contains a ClusterSummaryList with the cluster details; for example, the cluster IDs,
/// names, and status.
///
/// **AWS API**: `emr.v1.ListClustersOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListClustersOutput {
    /// The list of clusters for the account based on the given filters.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clusters: Vec<ClusterSummary>,

    /// The pagination token that indicates the next set of results to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListClustersOutput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            clusters: vec![],
            marker: Some("test-marker".into()),
        }
    }
}

/// The summary description of the cluster.
///
/// **AWS API**: `emr.v1.ClusterSummary`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClusterSummary {
    /// The unique identifier for the cluster.
    pub id: String,

    /// The name of the cluster.
    pub name: String,

    /// The details about the current status of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<ClusterStatus>>,

    /// An approximation of the cost of the cluster, represented in m1.small/hours. This value
    /// is incremented one time for every hour an m1.small instance runs. Larger instances are
    /// weighted more, so an Amazon EC2 instance that is roughly four times more expensive would
    /// result in the normalized instance hours being incremented by four. This result is only
    /// an approximation and does not reflect the actual billing rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_instance_hours: Option<i32>,

    /// The Amazon Resource Name of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,

    /// The Amazon Resource Name (ARN) of the Outpost where the cluster is launched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
}

impl ClusterSummary {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            name: "test-name".into(),
            normalized_instance_hours: Some(100),
            cluster_arn: Some("test-cluster_arn".into()),
            outpost_arn: Some("test-outpost_arn".into()),
            ..Default::default()
        }
    }
}

/// The detailed status of the cluster.
///
/// **AWS API**: `emr.v1.ClusterStatus`
///
/// ## Coverage
/// 3 of 4 fields included.
/// Omitted fields:
/// - `ErrorDetails` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClusterStatus {
    /// The current state of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The reason for the cluster status change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<Option<ClusterStateChangeReason>>,

    /// A timeline that represents the status of a cluster over the lifetime of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<Option<ClusterTimeline>>,
}

impl ClusterStatus {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            state: Some("test-state".into()),
            ..Default::default()
        }
    }
}

/// The reason that the cluster changed to its current state.
///
/// **AWS API**: `emr.v1.ClusterStateChangeReason`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClusterStateChangeReason {
    /// The programmatic code for the state change reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// The descriptive message for the state change reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ClusterStateChangeReason {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            code: Some("test-code".into()),
            message: Some("test-message".into()),
        }
    }
}

/// Represents the timeline of the cluster's lifecycle.
///
/// **AWS API**: `emr.v1.ClusterTimeline`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClusterTimeline {
    /// The creation date and time of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,

    /// The date and time when the cluster was ready to run steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,

    /// The date and time when the cluster was terminated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
}

impl ClusterTimeline {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ..Default::default()
        }
    }
}

/// This input determines which cluster to describe.
///
/// **AWS API**: `emr.v1.DescribeClusterInput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeClusterInput {
    /// The identifier of the cluster to describe.
    pub cluster_id: String,
}

impl DescribeClusterInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_id: "test-cluster_id".into(),
        }
    }
}

/// This output contains the description of the cluster.
///
/// **AWS API**: `emr.v1.DescribeClusterOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeClusterOutput {
    /// This output contains the details for the requested cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Option<Cluster>>,
}

impl DescribeClusterOutput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ..Default::default()
        }
    }
}

/// The detailed description of the cluster.
///
/// **AWS API**: `emr.v1.Cluster`
///
/// ## Coverage
/// 15 of 36 fields included.
/// Omitted fields:
/// - `InstanceCollectionType` — not selected in manifest
/// - `LogEncryptionKmsKeyId` — not selected in manifest
/// - `RequestedAmiVersion` — not selected in manifest
/// - `RunningAmiVersion` — not selected in manifest
/// - `UnhealthyNodeReplacement` — not selected in manifest
/// - `Applications` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `Configurations` — not selected in manifest
/// - `SecurityConfiguration` — not selected in manifest
/// - `AutoScalingRole` — not selected in manifest
/// - `ScaleDownBehavior` — not selected in manifest
/// - `CustomAmiId` — not selected in manifest
/// - `EbsRootVolumeSize` — not selected in manifest
/// - `RepoUpgradeOnBoot` — not selected in manifest
/// - `KerberosAttributes` — not selected in manifest
/// - `PlacementGroups` — not selected in manifest
/// - `OSReleaseLabel` — not selected in manifest
/// - `EbsRootVolumeIops` — not selected in manifest
/// - `EbsRootVolumeThroughput` — not selected in manifest
/// - `ExtendedSupport` — not selected in manifest
/// - `MonitoringConfiguration` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cluster {
    /// The unique identifier for the cluster.
    pub id: String,

    /// The name of the cluster. This parameter can't contain the characters &lt;, &gt;, $, |,
    /// or ` (backtick).
    pub name: String,

    /// The current status details about the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<ClusterStatus>>,

    /// Provides information about the Amazon EC2 instances in a cluster grouped by category.
    /// For example, key name, subnet ID, IAM instance profile, and so on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_attributes: Option<Option<Ec2InstanceAttributes>>,

    /// The Amazon EMR release label, which determines the version of open-source application
    /// packages installed on the cluster. Release labels are in the form emr-x.x.x, where x.x.x
    /// is an Amazon EMR release version such as emr-5.14.0. For more information about Amazon
    /// EMR release versions and included application versions and features, see
    /// https://docs.aws.amazon.com/emr/latest/ReleaseGuide/. The release label applies only to
    /// Amazon EMR releases version 4.0 and later. Earlier versions use AmiVersion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,

    /// The path to the Amazon S3 location where logs for this cluster are stored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,

    /// Specifies whether the cluster should terminate after completing all steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_terminate: Option<bool>,

    /// Indicates whether Amazon EMR will lock the cluster to prevent the Amazon EC2 instances
    /// from being terminated by an API call or user intervention, or in the event of a cluster
    /// error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protected: Option<bool>,

    /// Indicates whether the cluster is visible to IAM principals in the Amazon Web Services
    /// account associated with the cluster. When true, IAM principals in the Amazon Web
    /// Services account can perform Amazon EMR cluster actions on the cluster that their IAM
    /// policies allow. When false, only the IAM principal that created the cluster and the
    /// Amazon Web Services account root user can perform Amazon EMR actions, regardless of IAM
    /// permissions policies attached to other IAM principals. The default value is true if a
    /// value is not provided when creating a cluster using the Amazon EMR API RunJobFlow
    /// command, the CLI create-cluster command, or the Amazon Web Services Management Console.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_to_all_users: Option<bool>,

    /// The IAM role that Amazon EMR assumes in order to access Amazon Web Services resources on
    /// your behalf.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,

    /// An approximation of the cost of the cluster, represented in m1.small/hours. This value
    /// is incremented one time for every hour an m1.small instance runs. Larger instances are
    /// weighted more, so an Amazon EC2 instance that is roughly four times more expensive would
    /// result in the normalized instance hours being incremented by four. This result is only
    /// an approximation and does not reflect the actual billing rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_instance_hours: Option<i32>,

    /// The DNS name of the master node. If the cluster is on a private subnet, this is the
    /// private DNS name. On a public subnet, this is the public DNS name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_public_dns_name: Option<String>,

    /// The Amazon Resource Name of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,

    /// The Amazon Resource Name (ARN) of the Outpost where the cluster is launched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,

    /// Specifies the number of steps that can be executed concurrently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_concurrency_level: Option<i32>,
}

impl Cluster {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            name: "test-name".into(),
            release_label: Some("test-release_label".into()),
            log_uri: Some("test-log_uri".into()),
            auto_terminate: Some(false),
            termination_protected: Some(false),
            visible_to_all_users: Some(false),
            service_role: Some("test-service_role".into()),
            normalized_instance_hours: Some(100),
            master_public_dns_name: Some("test-master_public_dns_name".into()),
            cluster_arn: Some("test-cluster_arn".into()),
            outpost_arn: Some("test-outpost_arn".into()),
            step_concurrency_level: Some(100),
            ..Default::default()
        }
    }
}

/// Provides information about the Amazon EC2 instances in a cluster grouped by category. For
/// example, key name, subnet ID, IAM instance profile, and so on.
///
/// **AWS API**: `emr.v1.Ec2InstanceAttributes`
///
/// ## Coverage
/// 6 of 11 fields included.
/// Omitted fields:
/// - `RequestedEc2SubnetIds` — not selected in manifest
/// - `RequestedEc2AvailabilityZones` — not selected in manifest
/// - `ServiceAccessSecurityGroup` — not selected in manifest
/// - `AdditionalMasterSecurityGroups` — not selected in manifest
/// - `AdditionalSlaveSecurityGroups` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ec2InstanceAttributes {
    /// The name of the Amazon EC2 key pair to use when connecting with SSH into the master node
    /// as a user named "hadoop".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_key_name: Option<String>,

    /// Set this parameter to the identifier of the Amazon VPC subnet where you want the cluster
    /// to launch. If you do not specify this value, and your account supports EC2-Classic, the
    /// cluster launches in EC2-Classic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_subnet_id: Option<String>,

    /// The Availability Zone in which the cluster will run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_availability_zone: Option<String>,

    /// The IAM role that was specified when the cluster was launched. The Amazon EC2 instances
    /// of the cluster assume this role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<String>,

    /// The identifier of the Amazon EC2 security group for the master node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emr_managed_master_security_group: Option<String>,

    /// The identifier of the Amazon EC2 security group for the core and task nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emr_managed_slave_security_group: Option<String>,
}

impl Ec2InstanceAttributes {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ec2_key_name: Some("test-ec2_key_name".into()),
            ec2_subnet_id: Some("test-ec2_subnet_id".into()),
            ec2_availability_zone: Some("test-ec2_availability_zone".into()),
            iam_instance_profile: Some("test-iam_instance_profile".into()),
            emr_managed_master_security_group: Some(
                "test-emr_managed_master_security_group".into(),
            ),
            emr_managed_slave_security_group: Some("test-emr_managed_slave_security_group".into()),
        }
    }
}

/// Input to the TerminateJobFlows operation.
///
/// **AWS API**: `emr.v1.TerminateJobFlowsInput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TerminateJobFlowsInput {
    /// A list of job flows to be shut down.
    #[serde(default)]
    pub job_flow_ids: Vec<String>,
}

impl TerminateJobFlowsInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            job_flow_ids: vec![],
        }
    }
}
