//! Types for the Amazon Elastic Kubernetes Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///
/// **AWS API**: `eks.v1.DescribeClusterRequest`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//DescribeClusterRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeClusterRequest {
    /// The name of your cluster.
    pub name: String,
}

impl DescribeClusterRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-describe_cluster_request".into(),
        }
    }
}

///
/// **AWS API**: `eks.v1.DescribeClusterResponse`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//DescribeClusterResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeClusterResponse {
    /// The full description of your specified cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

impl DescribeClusterResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster: Some(Cluster::fixture()),
        }
    }
}

/// An object representing an Amazon EKS cluster.
///
/// **AWS API**: `eks.v1.Cluster`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//Cluster>
///
/// ## Coverage
/// 9 of 28 fields included.
/// Omitted fields:
/// - `createdAt` — not selected in manifest
/// - `kubernetesNetworkConfig` — not selected in manifest
/// - `logging` — not selected in manifest
/// - `identity` — not selected in manifest
/// - `certificateAuthority` — not selected in manifest
/// - `clientRequestToken` — not selected in manifest
/// - `encryptionConfig` — not selected in manifest
/// - `connectorConfig` — not selected in manifest
/// - `id` — not selected in manifest
/// - `health` — not selected in manifest
/// - `outpostConfig` — not selected in manifest
/// - `accessConfig` — not selected in manifest
/// - `upgradePolicy` — not selected in manifest
/// - `zonalShiftConfig` — not selected in manifest
/// - `remoteNetworkConfig` — not selected in manifest
/// - `computeConfig` — not selected in manifest
/// - `storageConfig` — not selected in manifest
/// - `deletionProtection` — not selected in manifest
/// - `controlPlaneScalingConfig` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    /// The name of your cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The Amazon Resource Name (ARN) of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The Kubernetes server version for the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The endpoint for your Kubernetes API server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    /// The Amazon Resource Name (ARN) of the IAM role that provides permissions for the
    /// Kubernetes control plane to make calls to Amazon Web Services API operations on your
    /// behalf.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,

    /// The VPC configuration used by the cluster control plane. Amazon EKS VPC resources have
    /// specific requirements to work properly with Kubernetes. For more information, see
    /// Cluster VPC considerations and Cluster security group considerations in the Amazon EKS
    /// User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_vpc_config: Option<VpcConfigResponse>,

    /// The current status of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The platform version of your Amazon EKS cluster. For more information about clusters
    /// deployed on the Amazon Web Services Cloud, see Platform versions in the Amazon EKS User
    /// Guide . For more information about local clusters deployed on an Outpost, see Amazon EKS
    /// local cluster platform versions in the Amazon EKS User Guide .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,

    /// Metadata that assists with categorization and organization. Each tag consists of a key
    /// and an optional value. You define both. Tags don't propagate to any other cluster or
    /// Amazon Web Services resources.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

impl Cluster {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-cluster".into()),
            arn: Some("test-arn".into()),
            version: Some("test-version".into()),
            endpoint: Some("test-endpoint".into()),
            role_arn: Some("test-role_arn".into()),
            resources_vpc_config: Some(VpcConfigResponse::fixture()),
            status: Some("test-status".into()),
            platform_version: Some("test-platform_version".into()),
            tags: Default::default(),
        }
    }
}

/// An object representing an Amazon EKS cluster VPC configuration response.
///
/// **AWS API**: `eks.v1.VpcConfigResponse`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//VpcConfigResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcConfigResponse {
    /// The subnets associated with your cluster.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subnet_ids: Vec<String>,

    /// The security groups associated with the cross-account elastic network interfaces that
    /// are used to allow communication between your nodes and the Kubernetes control plane.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_group_ids: Vec<String>,

    /// The cluster security group that was created by Amazon EKS for the cluster. Managed node
    /// groups use this security group for control-plane-to-data-plane communication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group_id: Option<String>,

    /// The VPC associated with your cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// Whether the public API server endpoint is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,

    /// This parameter indicates whether the Amazon EKS private API server endpoint is enabled.
    /// If the Amazon EKS private API server endpoint is enabled, Kubernetes API requests that
    /// originate from within your cluster's VPC use the private VPC endpoint instead of
    /// traversing the internet. If this value is disabled and you have nodes or Fargate pods in
    /// the cluster, then ensure that publicAccessCidrs includes the necessary CIDR blocks for
    /// communication with the nodes or Fargate pods. For more information, see Cluster API
    /// server endpoint in the Amazon EKS User Guide .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_private_access: Option<bool>,

    /// The CIDR blocks that are allowed access to your cluster's public Kubernetes API server
    /// endpoint. Communication to the endpoint from addresses outside of the CIDR blocks that
    /// you specify is denied. The default value is 0.0.0.0/0 and additionally ::/0 for dual-
    /// stack `IPv6` clusters. If you've disabled private endpoint access, make sure that you
    /// specify the necessary CIDR blocks for every node and Fargate Pod in the cluster. For
    /// more information, see Cluster API server endpoint in the Amazon EKS User Guide . Note
    /// that the public endpoints are dual-stack for only IPv6 clusters that are made after
    /// October 2024. You can't add IPv6 CIDR blocks to IPv4 clusters or IPv6 clusters that were
    /// made before October 2024.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub public_access_cidrs: Vec<String>,
}

impl VpcConfigResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            subnet_ids: vec![],
            security_group_ids: vec![],
            cluster_security_group_id: Some("test-cluster_security_group_id".into()),
            vpc_id: Some("test-vpc_id".into()),
            endpoint_public_access: Some(false),
            endpoint_private_access: Some(false),
            public_access_cidrs: vec![],
        }
    }
}

///
/// **AWS API**: `eks.v1.ListNodegroupsRequest`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//ListNodegroupsRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListNodegroupsRequest {
    /// The name of your cluster.
    pub cluster_name: String,

    /// The maximum number of results, returned in paginated output. You receive maxResults in a
    /// single page, along with a nextToken response element. You can see the remaining results
    /// of the initial request by sending another request with the returned nextToken value.
    /// This value can be between 1 and 100. If you don't use this parameter, 100 results and a
    /// nextToken value, if applicable, are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,

    /// The nextToken value returned from a previous paginated request, where maxResults was
    /// used and the results exceeded the value of that parameter. Pagination continues from the
    /// end of the previous results that returned the nextToken value. This value is null when
    /// there are no more results to return. This token should be treated as an opaque
    /// identifier that is used only to retrieve the next items in a list and not for other
    /// programmatic purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListNodegroupsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_name: "test-cluster_name".into(),
            max_results: Some(100),
            next_token: Some("test-next_token".into()),
        }
    }
}

///
/// **AWS API**: `eks.v1.ListNodegroupsResponse`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//ListNodegroupsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListNodegroupsResponse {
    /// A list of all of the node groups associated with the specified cluster.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodegroups: Vec<String>,

    /// The nextToken value returned from a previous paginated request, where maxResults was
    /// used and the results exceeded the value of that parameter. Pagination continues from the
    /// end of the previous results that returned the nextToken value. This value is null when
    /// there are no more results to return. This token should be treated as an opaque
    /// identifier that is used only to retrieve the next items in a list and not for other
    /// programmatic purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListNodegroupsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            nodegroups: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

///
/// **AWS API**: `eks.v1.DescribeNodegroupRequest`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//DescribeNodegroupRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeNodegroupRequest {
    /// The name of your cluster.
    pub cluster_name: String,

    /// The name of the node group to describe.
    pub nodegroup_name: String,
}

impl DescribeNodegroupRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_name: "test-cluster_name".into(),
            nodegroup_name: "test-nodegroup_name".into(),
        }
    }
}

///
/// **AWS API**: `eks.v1.DescribeNodegroupResponse`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//DescribeNodegroupResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeNodegroupResponse {
    /// The full description of your node group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup: Option<Nodegroup>,
}

impl DescribeNodegroupResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            nodegroup: Some(Nodegroup::fixture()),
        }
    }
}

/// An object representing an Amazon EKS managed node group.
///
/// **AWS API**: `eks.v1.Nodegroup`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//Nodegroup>
///
/// ## Coverage
/// 17 of 24 fields included.
/// Omitted fields:
/// - `createdAt` — not selected in manifest
/// - `modifiedAt` — not selected in manifest
/// - `remoteAccess` — not selected in manifest
/// - `taints` — not selected in manifest
/// - `resources` — not selected in manifest
/// - `nodeRepairConfig` — not selected in manifest
/// - `launchTemplate` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nodegroup {
    /// The name associated with an Amazon EKS managed node group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup_name: Option<String>,

    /// The Amazon Resource Name (ARN) associated with the managed node group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup_arn: Option<String>,

    /// The name of your cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,

    /// The Kubernetes version of the managed node group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// If the node group was deployed using a launch template with a custom AMI, then this is
    /// the AMI ID that was specified in the launch template. For node groups that weren't
    /// deployed using a launch template, this is the version of the Amazon EKS optimized AMI
    /// that the node group was deployed with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,

    /// The current status of the managed node group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The capacity type of your managed node group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_type: Option<String>,

    /// The scaling configuration details for the Auto Scaling group that is associated with
    /// your node group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<NodegroupScalingConfig>,

    /// If the node group wasn't deployed with a launch template, then this is the instance type
    /// that is associated with the node group. If the node group was deployed with a launch
    /// template, then this is null.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instance_types: Vec<String>,

    /// The subnets that were specified for the Auto Scaling group that is associated with your
    /// node group.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<String>,

    /// If the node group was deployed using a launch template with a custom AMI, then this is
    /// CUSTOM. For node groups that weren't deployed using a launch template, this is the AMI
    /// type that was specified in the node group configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_type: Option<String>,

    /// The IAM role associated with your node group. The Amazon EKS node kubelet daemon makes
    /// calls to Amazon Web Services APIs on your behalf. Nodes receive permissions for these
    /// API calls through an IAM instance profile and associated policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_role: Option<String>,

    /// The Kubernetes labels applied to the nodes in the node group. Only labels that are
    /// applied with the Amazon EKS API are shown here. There may be other Kubernetes labels
    /// applied to the nodes in this group.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// If the node group wasn't deployed with a launch template, then this is the disk size in
    /// the node group configuration. If the node group was deployed with a launch template,
    /// then this is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i32>,

    /// The health status of the node group. If there are issues with your node group's health,
    /// they are listed here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<NodegroupHealth>,

    /// The node group update configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<NodegroupUpdateConfig>,

    /// Metadata that assists with categorization and organization. Each tag consists of a key
    /// and an optional value. You define both. Tags don't propagate to any other cluster or
    /// Amazon Web Services resources.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

impl Nodegroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            nodegroup_name: Some("test-nodegroup_name".into()),
            nodegroup_arn: Some("test-nodegroup_arn".into()),
            cluster_name: Some("test-cluster_name".into()),
            version: Some("test-version".into()),
            release_version: Some("test-release_version".into()),
            status: Some("test-status".into()),
            capacity_type: Some("test-capacity_type".into()),
            scaling_config: Some(NodegroupScalingConfig::fixture()),
            instance_types: vec![],
            subnets: vec![],
            ami_type: Some("test-ami_type".into()),
            node_role: Some("test-node_role".into()),
            labels: Default::default(),
            disk_size: Some(100),
            health: Some(NodegroupHealth::fixture()),
            update_config: Some(NodegroupUpdateConfig::fixture()),
            tags: Default::default(),
        }
    }
}

/// An object representing the scaling configuration details for the Auto Scaling group that is
/// associated with your node group. When creating a node group, you must specify all or none of
/// the properties. When updating a node group, you can specify any or none of the properties.
///
/// **AWS API**: `eks.v1.NodegroupScalingConfig`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//NodegroupScalingConfig>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodegroupScalingConfig {
    /// The minimum number of nodes that the managed node group can scale in to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,

    /// The maximum number of nodes that the managed node group can scale out to. For
    /// information about the maximum number that you can specify, see Amazon EKS service quotas
    /// in the Amazon EKS User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,

    /// The current number of nodes that the managed node group should maintain. If you use the
    /// Kubernetes Cluster Autoscaler, you shouldn't change the desiredSize value directly, as
    /// this can cause the Cluster Autoscaler to suddenly scale up or scale down. Whenever this
    /// parameter changes, the number of worker nodes in the node group is updated to the
    /// specified size. If this parameter is given a value that is smaller than the current
    /// number of running worker nodes, the necessary number of worker nodes are terminated to
    /// match the given value. When using CloudFormation, no action occurs if you remove this
    /// parameter from your CFN template. This parameter can be different from minSize in some
    /// cases, such as when starting with extra hosts for testing. This parameter can also be
    /// different when you want to start with an estimated number of needed hosts, but let the
    /// Cluster Autoscaler reduce the number if there are too many. When the Cluster Autoscaler
    /// is used, the desiredSize parameter is altered by the Cluster Autoscaler (but can be out-
    /// of-date for short periods of time). the Cluster Autoscaler doesn't scale a managed node
    /// group lower than minSize or higher than maxSize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_size: Option<i32>,
}

impl NodegroupScalingConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            min_size: Some(100),
            max_size: Some(100),
            desired_size: Some(100),
        }
    }
}

/// The node group update configuration. An Amazon EKS managed node group updates by replacing
/// nodes with new nodes of newer AMI versions in parallel. You choose the maximum unavailable
/// and the update strategy.
///
/// **AWS API**: `eks.v1.NodegroupUpdateConfig`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//NodegroupUpdateConfig>
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `updateStrategy` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodegroupUpdateConfig {
    /// The maximum number of nodes unavailable at once during a version update. Nodes are
    /// updated in parallel. This value or maxUnavailablePercentage is required to have a
    /// value.The maximum number is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<i32>,

    /// The maximum percentage of nodes unavailable during a version update. This percentage of
    /// nodes are updated in parallel, up to 100 nodes at once. This value or maxUnavailable is
    /// required to have a value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable_percentage: Option<i32>,
}

impl NodegroupUpdateConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            max_unavailable: Some(100),
            max_unavailable_percentage: Some(100),
        }
    }
}

/// An object representing the health status of the node group.
///
/// **AWS API**: `eks.v1.NodegroupHealth`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//NodegroupHealth>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodegroupHealth {
    /// Any issues that are associated with the node group.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<Issue>,
}

impl NodegroupHealth {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { issues: vec![] }
    }
}

/// An object representing an issue with an Amazon EKS resource.
///
/// **AWS API**: `eks.v1.Issue`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//Issue>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    /// A brief description of the error. AccessDenied: Amazon EKS or one or more of your
    /// managed nodes is failing to authenticate or authorize with your Kubernetes cluster API
    /// server. AsgInstanceLaunchFailures: Your Auto Scaling group is experiencing failures
    /// while attempting to launch instances. AutoScalingGroupNotFound: We couldn't find the
    /// Auto Scaling group associated with the managed node group. You may be able to recreate
    /// an Auto Scaling group with the same settings to recover. ClusterUnreachable: Amazon EKS
    /// or one or more of your managed nodes is unable to to communicate with your Kubernetes
    /// cluster API server. This can happen if there are network disruptions or if API servers
    /// are timing out processing requests. Ec2InstanceTypeDoesNotExist: One or more of the
    /// supplied Amazon EC2 instance types do not exist. Amazon EKS checked for the instance
    /// types that you provided in this Amazon Web Services Region, and one or more aren't
    /// available. Ec2LaunchTemplateNotFound: We couldn't find the Amazon EC2 launch template
    /// for your managed node group. You may be able to recreate a launch template with the same
    /// settings to recover. Ec2LaunchTemplateVersionMismatch: The Amazon EC2 launch template
    /// version for your managed node group does not match the version that Amazon EKS created.
    /// You may be able to revert to the version that Amazon EKS created to recover.
    /// Ec2SecurityGroupDeletionFailure: We could not delete the remote access security group
    /// for your managed node group. Remove any dependencies from the security group.
    /// Ec2SecurityGroupNotFound: We couldn't find the cluster security group for the cluster.
    /// You must recreate your cluster. Ec2SubnetInvalidConfiguration: One or more Amazon EC2
    /// subnets specified for a node group do not automatically assign public IP addresses to
    /// instances launched into it. If you want your instances to be assigned a public IP
    /// address, then you need to enable the auto-assign public IP address setting for the
    /// subnet. See Modifying the public IPv4 addressing attribute for your subnet in the Amazon
    /// VPC User Guide. IamInstanceProfileNotFound: We couldn't find the IAM instance profile
    /// for your managed node group. You may be able to recreate an instance profile with the
    /// same settings to recover. IamNodeRoleNotFound: We couldn't find the IAM role for your
    /// managed node group. You may be able to recreate an IAM role with the same settings to
    /// recover. InstanceLimitExceeded: Your Amazon Web Services account is unable to launch any
    /// more instances of the specified instance type. You may be able to request an Amazon EC2
    /// instance limit increase to recover. InsufficientFreeAddresses: One or more of the
    /// subnets associated with your managed node group does not have enough available IP
    /// addresses for new nodes. InternalFailure: These errors are usually caused by an Amazon
    /// EKS server-side issue. NodeCreationFailure: Your launched instances are unable to
    /// register with your Amazon EKS cluster. Common causes of this failure are insufficient
    /// node IAM role permissions or lack of outbound internet access for the nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// The error message associated with the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The Amazon Web Services resources that are afflicted by this issue.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_ids: Vec<String>,
}

impl Issue {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            code: Some("test-code".into()),
            message: Some("test-message".into()),
            resource_ids: vec![],
        }
    }
}

///
/// **AWS API**: `eks.v1.UpdateNodegroupConfigRequest`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//UpdateNodegroupConfigRequest>
///
/// ## Coverage
/// 5 of 8 fields included.
/// Omitted fields:
/// - `labels` — not selected in manifest
/// - `taints` — not selected in manifest
/// - `nodeRepairConfig` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNodegroupConfigRequest {
    /// The name of your cluster.
    pub cluster_name: String,

    /// The name of the managed node group to update.
    pub nodegroup_name: String,

    /// The scaling configuration details for the Auto Scaling group after the update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<NodegroupScalingConfig>,

    /// The node group update configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<NodegroupUpdateConfig>,

    /// A unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
}

impl UpdateNodegroupConfigRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_name: "test-cluster_name".into(),
            nodegroup_name: "test-nodegroup_name".into(),
            scaling_config: Some(NodegroupScalingConfig::fixture()),
            update_config: Some(NodegroupUpdateConfig::fixture()),
            client_request_token: Some("test-client_request_token".into()),
        }
    }
}

///
/// **AWS API**: `eks.v1.UpdateNodegroupConfigResponse`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//UpdateNodegroupConfigResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNodegroupConfigResponse {
    /// The `update` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

impl UpdateNodegroupConfigResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            update: Some(Update::fixture()),
        }
    }
}

/// An object representing an asynchronous update.
///
/// **AWS API**: `eks.v1.Update`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//Update>
///
/// ## Coverage
/// 4 of 6 fields included.
/// Omitted fields:
/// - `params` — not selected in manifest
/// - `createdAt` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    /// A UUID that is used to track the update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The current status of the update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The type of the update.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Any errors associated with a Failed update.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorDetail>,
}

impl Update {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            status: Some("test-status".into()),
            r#type: Some("test-type".into()),
            errors: vec![],
        }
    }
}

/// An object representing an error when an asynchronous operation fails.
///
/// **AWS API**: `eks.v1.ErrorDetail`
/// **Reference**: <https://docs.aws.amazon.com/eks/latest/APIReference//ErrorDetail>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetail {
    /// A brief description of the error. SubnetNotFound: We couldn't find one of the subnets
    /// associated with the cluster. SecurityGroupNotFound: We couldn't find one of the security
    /// groups associated with the cluster. EniLimitReached: You have reached the elastic
    /// network interface limit for your account. IpNotAvailable: A subnet associated with the
    /// cluster doesn't have any available IP addresses. AccessDenied: You don't have
    /// permissions to perform the specified operation. OperationNotPermitted: The service role
    /// associated with the cluster doesn't have the required access permissions for Amazon EKS.
    /// VpcIdNotFound: We couldn't find the VPC associated with the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,

    /// A more complete description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    /// An optional field that contains the resource IDs associated with the error.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_ids: Vec<String>,
}

impl ErrorDetail {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            error_code: Some("test-error_code".into()),
            error_message: Some("test-error_message".into()),
            resource_ids: vec![],
        }
    }
}
