//! Types for the Elastic Load Balancing v2 API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `elbv2.v1.DescribeLoadBalancersInput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DescribeLoadBalancersInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeLoadBalancersRequest {
    /// The Amazon Resource Names (ARN) of the load balancers. You can specify up to 20 load
    /// balancers in a single call.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_arns: Vec<String>,

    /// The names of the load balancers.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,

    /// The marker for the next set of results. (You received this marker from a previous call.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// The maximum number of results to return with this call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeLoadBalancersRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            load_balancer_arns: vec![],
            names: vec![],
            marker: Some("test-marker".into()),
            page_size: Some(100),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DescribeLoadBalancersOutput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DescribeLoadBalancersOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeLoadBalancersResponse {
    /// Information about the load balancers.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub load_balancers: Vec<LoadBalancer>,

    /// If there are additional results, this is the marker for the next set of results.
    /// Otherwise, this is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

impl DescribeLoadBalancersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            load_balancers: vec![],
            next_marker: Some("test-next_marker".into()),
        }
    }
}

/// Information about a load balancer.
///
/// **AWS API**: `elbv2.v1.LoadBalancer`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//LoadBalancer>
///
/// ## Coverage
/// 12 of 16 fields included.
/// Omitted fields:
/// - `CustomerOwnedIpv4Pool` — not selected in manifest
/// - `EnforceSecurityGroupInboundRulesOnPrivateLinkTraffic` — not selected in manifest
/// - `EnablePrefixForIpv6SourceNat` — not selected in manifest
/// - `IpamPools` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadBalancer {
    /// The Amazon Resource Name (ARN) of the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arn: Option<String>,

    /// The public DNS name of the load balancer.
    #[serde(rename = "DNSName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,

    /// The name of the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,

    /// The nodes of an Internet-facing load balancer have public IP addresses. The DNS name of
    /// an Internet-facing load balancer is publicly resolvable to the public IP addresses of
    /// the nodes. Therefore, Internet-facing load balancers can route requests from clients
    /// over the internet. The nodes of an internal load balancer have only private IP
    /// addresses. The DNS name of an internal load balancer is publicly resolvable to the
    /// private IP addresses of the nodes. Therefore, internal load balancers can route requests
    /// only from clients with access to the VPC for the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    /// The ID of the VPC for the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// The state of the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<LoadBalancerState>,

    /// The type of load balancer.
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The subnets for the load balancer.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub availability_zones: Vec<AvailabilityZone>,

    /// The IDs of the security groups for the load balancer.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_groups: Vec<String>,

    /// The type of IP addresses used for public or private connections by the subnets attached
    /// to your load balancer. [Application Load Balancers] The possible values are ipv4 (IPv4
    /// addresses), dualstack (IPv4 and IPv6 addresses), and dualstack-without-public-ipv4
    /// (public IPv6 addresses and private IPv4 and IPv6 addresses). [Network Load Balancers and
    /// Gateway Load Balancers] The possible values are ipv4 (IPv4 addresses) and dualstack
    /// (IPv4 and IPv6 addresses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,

    /// The date and time the load balancer was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    /// The ID of the Amazon Route 53 hosted zone associated with the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_id: Option<String>,
}

impl LoadBalancer {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            load_balancer_arn: Some("test-load_balancer_arn".into()),
            dns_name: Some("test-dns_name".into()),
            load_balancer_name: Some("test-load_balancer_name".into()),
            scheme: Some("test-scheme".into()),
            vpc_id: Some("test-vpc_id".into()),
            state: Some(LoadBalancerState::fixture()),
            r#type: Some("test-type".into()),
            availability_zones: vec![],
            security_groups: vec![],
            ip_address_type: Some("test-ip_address_type".into()),
            created_time: Some("test-created_time".into()),
            canonical_hosted_zone_id: Some("test-canonical_hosted_zone_id".into()),
        }
    }
}

/// Information about the state of the load balancer.
///
/// **AWS API**: `elbv2.v1.LoadBalancerState`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//LoadBalancerState>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadBalancerState {
    /// The state code. The initial state of the load balancer is provisioning. After the load
    /// balancer is fully set up and ready to route traffic, its state is active. If load
    /// balancer is routing traffic but does not have the resources it needs to scale, its state
    /// isactive_impaired. If the load balancer could not be set up, its state is failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// A description of the state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl LoadBalancerState {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            code: Some("test-code".into()),
            reason: Some("test-reason".into()),
        }
    }
}

/// Information about an Availability Zone.
///
/// **AWS API**: `elbv2.v1.AvailabilityZone`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//AvailabilityZone>
///
/// ## Coverage
/// 2 of 5 fields included.
/// Omitted fields:
/// - `OutpostId` — not selected in manifest
/// - `LoadBalancerAddresses` — not selected in manifest
/// - `SourceNatIpv6Prefixes` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvailabilityZone {
    /// The name of the Availability Zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,

    /// The ID of the subnet. You can specify one subnet per Availability Zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

impl AvailabilityZone {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            zone_name: Some("test-zone_name".into()),
            subnet_id: Some("test-subnet_id".into()),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DescribeTargetGroupsInput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DescribeTargetGroupsInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeTargetGroupsRequest {
    /// The Amazon Resource Name (ARN) of the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arn: Option<String>,

    /// The Amazon Resource Names (ARN) of the target groups.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_group_arns: Vec<String>,

    /// The names of the target groups.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,

    /// The marker for the next set of results. (You received this marker from a previous call.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// The maximum number of results to return with this call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeTargetGroupsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            load_balancer_arn: Some("test-load_balancer_arn".into()),
            target_group_arns: vec![],
            names: vec![],
            marker: Some("test-marker".into()),
            page_size: Some(100),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DescribeTargetGroupsOutput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DescribeTargetGroupsOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeTargetGroupsResponse {
    /// Information about the target groups.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_groups: Vec<TargetGroup>,

    /// If there are additional results, this is the marker for the next set of results.
    /// Otherwise, this is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

impl DescribeTargetGroupsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            target_groups: vec![],
            next_marker: Some("test-next_marker".into()),
        }
    }
}

/// Information about a target group.
///
/// **AWS API**: `elbv2.v1.TargetGroup`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//TargetGroup>
///
/// ## Coverage
/// 15 of 19 fields included.
/// Omitted fields:
/// - `Matcher` — not selected in manifest
/// - `ProtocolVersion` — not selected in manifest
/// - `IpAddressType` — not selected in manifest
/// - `TargetControlPort` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TargetGroup {
    /// The Amazon Resource Name (ARN) of the target group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,

    /// The name of the target group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_name: Option<String>,

    /// The protocol to use for routing traffic to the targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// The port on which the targets are listening. This parameter is not used if the target is
    /// a Lambda function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// The ID of the VPC for the targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// The protocol to use to connect with the target. The GENEVE, TLS, UDP, and TCP_UDP
    /// protocols are not supported for health checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_protocol: Option<String>,

    /// The port to use to connect with the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<String>,

    /// Indicates whether health checks are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_enabled: Option<bool>,

    /// The approximate amount of time, in seconds, between health checks of an individual
    /// target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval_seconds: Option<i32>,

    /// The amount of time, in seconds, during which no response means a failed health check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout_seconds: Option<i32>,

    /// The number of consecutive health checks successes required before considering an
    /// unhealthy target healthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold_count: Option<i32>,

    /// The number of consecutive health check failures required before considering the target
    /// unhealthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold_count: Option<i32>,

    /// The destination for health checks on the targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,

    /// The Amazon Resource Name (ARN) of the load balancer that routes traffic to this target
    /// group. You can use each target group with only one load balancer.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_arns: Vec<String>,

    /// The type of target that you must specify when registering targets with this target
    /// group. The possible values are instance (register targets by instance ID), ip (register
    /// targets by IP address), lambda (register a single Lambda function as a target), or alb
    /// (register a single Application Load Balancer as a target).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

impl TargetGroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            target_group_arn: Some("test-target_group_arn".into()),
            target_group_name: Some("test-target_group_name".into()),
            protocol: Some("test-protocol".into()),
            port: Some(100),
            vpc_id: Some("test-vpc_id".into()),
            health_check_protocol: Some("test-health_check_protocol".into()),
            health_check_port: Some("test-health_check_port".into()),
            health_check_enabled: Some(false),
            health_check_interval_seconds: Some(100),
            health_check_timeout_seconds: Some(100),
            healthy_threshold_count: Some(100),
            unhealthy_threshold_count: Some(100),
            health_check_path: Some("test-health_check_path".into()),
            load_balancer_arns: vec![],
            target_type: Some("test-target_type".into()),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DescribeTargetHealthInput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DescribeTargetHealthInput>
///
/// ## Coverage
/// 1 of 3 fields included.
/// Omitted fields:
/// - `Targets` — not selected in manifest
/// - `Include` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeTargetHealthRequest {
    /// The Amazon Resource Name (ARN) of the target group.
    pub target_group_arn: String,
}

impl DescribeTargetHealthRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            target_group_arn: "test-target_group_arn".into(),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DescribeTargetHealthOutput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DescribeTargetHealthOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeTargetHealthResponse {
    /// Information about the health of the targets.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_health_descriptions: Vec<TargetHealthDescription>,
}

impl DescribeTargetHealthResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            target_health_descriptions: vec![],
        }
    }
}

/// Information about the health of a target.
///
/// **AWS API**: `elbv2.v1.TargetHealthDescription`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//TargetHealthDescription>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `AnomalyDetection` — not selected in manifest
/// - `AdministrativeOverride` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TargetHealthDescription {
    /// The description of the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TargetDescription>,

    /// The port to use to connect with the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<String>,

    /// The health information for the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_health: Option<TargetHealth>,
}

impl TargetHealthDescription {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            target: Some(TargetDescription::fixture()),
            health_check_port: Some("test-health_check_port".into()),
            target_health: Some(TargetHealth::fixture()),
        }
    }
}

/// Information about a target.
///
/// **AWS API**: `elbv2.v1.TargetDescription`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//TargetDescription>
///
/// ## Coverage
/// 3 of 4 fields included.
/// Omitted fields:
/// - `QuicServerId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TargetDescription {
    /// The ID of the target. If the target type of the target group is instance, specify an
    /// instance ID. If the target type is ip, specify an IP address. If the target type is
    /// lambda, specify the ARN of the Lambda function. If the target type is alb, specify the
    /// ARN of the Application Load Balancer target.
    pub id: String,

    /// The port on which the target is listening. If the target group protocol is GENEVE, the
    /// supported port is 6081. If the target type is alb, the targeted Application Load
    /// Balancer must have at least one listener whose port matches the target group port. This
    /// parameter is not used if the target is a Lambda function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// An Availability Zone or all. This determines whether the target receives traffic from
    /// the load balancer nodes in the specified Availability Zone or from all enabled
    /// Availability Zones for the load balancer. For Application Load Balancer target groups,
    /// the specified Availability Zone value is only applicable when cross-zone load balancing
    /// is off. Otherwise the parameter is ignored and treated as all. This parameter is not
    /// supported if the target type of the target group is instance or alb. If the target type
    /// is ip and the IP address is in a subnet of the VPC for the target group, the
    /// Availability Zone is automatically detected and this parameter is optional. If the IP
    /// address is outside the VPC, this parameter is required. For Application Load Balancer
    /// target groups with cross-zone load balancing off, if the target type is ip and the IP
    /// address is outside of the VPC for the target group, this should be an Availability Zone
    /// inside the VPC for the target group. If the target type is lambda, this parameter is
    /// optional and the only supported value is all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
}

impl TargetDescription {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            port: Some(100),
            availability_zone: Some("test-availability_zone".into()),
        }
    }
}

/// Information about the current health of a target.
///
/// **AWS API**: `elbv2.v1.TargetHealth`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//TargetHealth>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TargetHealth {
    /// The state of the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The reason code. If the target state is healthy, a reason code is not provided. If the
    /// target state is initial, the reason code can be one of the following values:
    /// Elb.RegistrationInProgress
    /// - The target is in the process of being registered with the load balancer.
    ///   Elb.InitialHealthChecking
    /// - The load balancer is still sending the target the minimum number of health checks
    ///   required to determine its health status. If the target state is unhealthy, the reason
    ///   code can be one of the following values: Target.ResponseCodeMismatch
    /// - The health checks did not return an expected HTTP code. Target.Timeout
    /// - The health check requests timed out. Target.FailedHealthChecks
    /// - The load balancer received an error while establishing a connection to the target or
    ///   the target response was malformed. Elb.InternalError
    /// - The health checks failed due to an internal error. If the target state is unused, the
    ///   reason code can be one of the following values: Target.NotRegistered
    /// - The target is not registered with the target group. Target.NotInUse
    /// - The target group is not used by any load balancer or the target is in an Availability
    ///   Zone that is not enabled for its load balancer. Target.InvalidState
    /// - The target is in the stopped or terminated state. Target.IpUnusable
    /// - The target IP address is reserved for use by a load balancer. If the target state is
    ///   draining, the reason code can be the following value: Target.DeregistrationInProgress
    /// - The target is in the process of being deregistered and the deregistration delay period
    ///   has not expired. If the target state is unavailable, the reason code can be the
    ///   following value: Target.HealthCheckDisabled
    /// - Health checks are disabled for the target group. Elb.InternalError
    /// - Target health is unavailable due to an internal error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// A description of the target health that provides additional details. If the state is
    /// healthy, a description is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TargetHealth {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            state: Some("test-state".into()),
            reason: Some("test-reason".into()),
            description: Some("test-description".into()),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DescribeListenersInput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DescribeListenersInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeListenersRequest {
    /// The Amazon Resource Name (ARN) of the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arn: Option<String>,

    /// The Amazon Resource Names (ARN) of the listeners.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub listener_arns: Vec<String>,

    /// The marker for the next set of results. (You received this marker from a previous call.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// The maximum number of results to return with this call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeListenersRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            load_balancer_arn: Some("test-load_balancer_arn".into()),
            listener_arns: vec![],
            marker: Some("test-marker".into()),
            page_size: Some(100),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DescribeListenersOutput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DescribeListenersOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeListenersResponse {
    /// Information about the listeners.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub listeners: Vec<Listener>,

    /// If there are additional results, this is the marker for the next set of results.
    /// Otherwise, this is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

impl DescribeListenersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            listeners: vec![],
            next_marker: Some("test-next_marker".into()),
        }
    }
}

/// Information about a listener.
///
/// **AWS API**: `elbv2.v1.Listener`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//Listener>
///
/// ## Coverage
/// 5 of 9 fields included.
/// Omitted fields:
/// - `Certificates` — not selected in manifest
/// - `DefaultActions` — not selected in manifest
/// - `AlpnPolicy` — not selected in manifest
/// - `MutualAuthentication` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Listener {
    /// The Amazon Resource Name (ARN) of the listener.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_arn: Option<String>,

    /// The Amazon Resource Name (ARN) of the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arn: Option<String>,

    /// The port on which the load balancer is listening.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// The protocol for connections from clients to the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// [HTTPS or TLS listener] The security policy that defines which protocols and ciphers are
    /// supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_policy: Option<String>,
}

impl Listener {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            listener_arn: Some("test-listener_arn".into()),
            load_balancer_arn: Some("test-load_balancer_arn".into()),
            port: Some(100),
            protocol: Some("test-protocol".into()),
            ssl_policy: Some("test-ssl_policy".into()),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DescribeLoadBalancerAttributesInput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DescribeLoadBalancerAttributesInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeLoadBalancerAttributesRequest {
    /// The Amazon Resource Name (ARN) of the load balancer.
    pub load_balancer_arn: String,
}

impl DescribeLoadBalancerAttributesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            load_balancer_arn: "test-load_balancer_arn".into(),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DescribeLoadBalancerAttributesOutput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DescribeLoadBalancerAttributesOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeLoadBalancerAttributesResponse {
    /// Information about the load balancer attributes.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<LoadBalancerAttribute>,
}

impl DescribeLoadBalancerAttributesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { attributes: vec![] }
    }
}

/// Information about a load balancer attribute.
///
/// **AWS API**: `elbv2.v1.LoadBalancerAttribute`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//LoadBalancerAttribute>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadBalancerAttribute {
    /// The name of the attribute. The following attributes are supported by all load balancers:
    /// deletion_protection.enabled
    /// - Indicates whether deletion protection is enabled. The value is true or false. The
    ///   default is false. load_balancing.cross_zone.enabled
    /// - Indicates whether cross-zone load balancing is enabled. The possible values are true
    ///   and false. The default for Network Load Balancers and Gateway Load Balancers is false.
    ///   The default for Application Load Balancers is true, and can't be changed. The
    ///   following attributes are supported by both Application Load Balancers and Network Load
    ///   Balancers: access_logs.s3.enabled
    /// - Indicates whether access logs are enabled. The value is true or false. The default is
    ///   false. access_logs.s3.bucket
    /// - The name of the S3 bucket for the access logs. This attribute is required if access
    ///   logs are enabled. The bucket must exist in the same region as the load balancer and
    ///   have a bucket policy that grants Elastic Load Balancing permissions to write to the
    ///   bucket. access_logs.s3.prefix
    /// - The prefix for the location in the S3 bucket for the access logs.
    ///   ipv6.deny_all_igw_traffic
    /// - Blocks internet gateway (IGW) access to the load balancer. It is set to false for
    ///   internet-facing load balancers and true for internal load balancers, preventing
    ///   unintended access to your internal load balancer through an internet gateway.
    ///   zonal_shift.config.enabled
    /// - Indicates whether zonal shift is enabled. The possible values are true and false. The
    ///   default is false. The following attributes are supported by only Application Load
    ///   Balancers: idle_timeout.timeout_seconds
    /// - The idle timeout value, in seconds. The valid range is 1-4000 seconds. The default is
    ///   60 seconds. client_keep_alive.seconds
    /// - The client keep alive value, in seconds. The valid range is 60-604800 seconds. The
    ///   default is 3600 seconds. connection_logs.s3.enabled
    /// - Indicates whether connection logs are enabled. The value is true or false. The default
    ///   is false. connection_logs.s3.bucket
    /// - The name of the S3 bucket for the connection logs. This attribute is required if
    ///   connection logs are enabled. The bucket must exist in the same region as the load
    ///   balancer and have a bucket policy that grants Elastic Load Balancing permissions to
    ///   write to the bucket. connection_logs.s3.prefix
    /// - The prefix for the location in the S3 bucket for the connection logs.
    ///   health_check_logs.s3.enabled
    /// - Indicates whether health check logs are enabled. The value is true or false. The
    ///   default is false. health_check_logs.s3.bucket
    /// - The name of the S3 bucket for the health check logs. This attribute is required if
    ///   health check logs are enabled. The bucket must exist in the same region as the load
    ///   balancer and have a bucket policy that grants Elastic Load Balancing permissions to
    ///   write to the bucket. health_check_logs.s3.prefix
    /// - The prefix for the location in the S3 bucket for the health check logs.
    ///   routing.http.desync_mitigation_mode
    /// - Determines how the load balancer handles requests that might pose a security risk to
    ///   your application. The possible values are monitor, defensive, and strictest. The
    ///   default is defensive. routing.http.drop_invalid_header_fields.enabled
    /// - Indicates whether HTTP headers with invalid header fields are removed by the load
    ///   balancer (true) or routed to targets (false). The default is false.
    ///   routing.http.preserve_host_header.enabled
    /// - Indicates whether the Application Load Balancer should preserve the Host header in the
    ///   HTTP request and send it to the target without any change. The possible values are
    ///   true and false. The default is false.
    ///   routing.http.x_amzn_tls_version_and_cipher_suite.enabled
    /// - Indicates whether the two headers (x-amzn-tls-version and x-amzn-tls-cipher-suite),
    ///   which contain information about the negotiated TLS version and cipher suite, are added
    ///   to the client request before sending it to the target. The x-amzn-tls-version header
    ///   has information about the TLS protocol version negotiated with the client, and the
    ///   x-amzn-tls-cipher-suite header has information about the cipher suite negotiated with
    ///   the client. Both headers are in OpenSSL format. The possible values for the attribute
    ///   are true and false. The default is false. routing.http.xff_client_port.enabled
    /// - Indicates whether the X-Forwarded-For header should preserve the source port that the
    ///   client used to connect to the load balancer. The possible values are true and false.
    ///   The default is false. routing.http.xff_header_processing.mode
    /// - Enables you to modify, preserve, or remove the X-Forwarded-For header in the HTTP
    ///   request before the Application Load Balancer sends the request to the target. The
    ///   possible values are append, preserve, and remove. The default is append. If the value
    ///   is append, the Application Load Balancer adds the client IP address (of the last hop)
    ///   to the X-Forwarded-For header in the HTTP request before it sends it to targets. If
    ///   the value is preserve the Application Load Balancer preserves the X-Forwarded-For
    ///   header in the HTTP request, and sends it to targets without any change. If the value
    ///   is remove, the Application Load Balancer removes the X-Forwarded-For header in the
    ///   HTTP request before it sends it to targets. routing.http2.enabled
    /// - Indicates whether clients can connect to the load balancer using HTTP/2. If true,
    ///   clients can connect using HTTP/2 or HTTP/1.1. However, all client requests are subject
    ///   to the stricter HTTP/2 header validation rules. For example, message header names must
    ///   contain only alphanumeric characters and hyphens. If false, clients must connect using
    ///   HTTP/1.1. The default is true. waf.fail_open.enabled
    /// - Indicates whether to allow a WAF-enabled load balancer to route requests to targets if
    ///   it is unable to forward the request to Amazon Web Services WAF. The possible values
    ///   are true and false. The default is false. The following attributes are supported by
    ///   only Network Load Balancers: dns_record.client_routing_policy
    /// - Indicates how traffic is distributed among the load balancer Availability Zones. The
    ///   possible values are availability_zone_affinity with 100 percent zonal affinity,
    ///   partial_availability_zone_affinity with 85 percent zonal affinity, and
    ///   any_availability_zone with 0 percent zonal affinity.
    ///   secondary_ips.auto_assigned.per_subnet
    /// - The number of secondary IP addresses to configure for your load balancer nodes. Use to
    ///   address port allocation errors if you can't add targets. The valid range is 0 to 7.
    ///   The default is 0. After you set this value, you can't decrease it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// The value of the attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl LoadBalancerAttribute {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key: Some("test-key".into()),
            value: Some("test-value".into()),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DeleteLoadBalancerInput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DeleteLoadBalancerInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteLoadBalancerRequest {
    /// The Amazon Resource Name (ARN) of the load balancer.
    pub load_balancer_arn: String,
}

impl DeleteLoadBalancerRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            load_balancer_arn: "test-load_balancer_arn".into(),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DeleteLoadBalancerOutput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DeleteLoadBalancerOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteLoadBalancerResponse {}

impl DeleteLoadBalancerResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}

///
/// **AWS API**: `elbv2.v1.DeleteTargetGroupInput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DeleteTargetGroupInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTargetGroupRequest {
    /// The Amazon Resource Name (ARN) of the target group.
    pub target_group_arn: String,
}

impl DeleteTargetGroupRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            target_group_arn: "test-target_group_arn".into(),
        }
    }
}

///
/// **AWS API**: `elbv2.v1.DeleteTargetGroupOutput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//DeleteTargetGroupOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTargetGroupResponse {}

impl DeleteTargetGroupResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}

///
/// **AWS API**: `elbv2.v1.ModifyLoadBalancerAttributesInput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//ModifyLoadBalancerAttributesInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModifyLoadBalancerAttributesRequest {
    /// The Amazon Resource Name (ARN) of the load balancer.
    pub load_balancer_arn: String,

    /// The load balancer attributes.
    #[serde(default)]
    pub attributes: Vec<LoadBalancerAttribute>,
}

impl ModifyLoadBalancerAttributesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            load_balancer_arn: "test-load_balancer_arn".into(),
            attributes: vec![],
        }
    }
}

///
/// **AWS API**: `elbv2.v1.ModifyLoadBalancerAttributesOutput`
/// **Reference**: <https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference//ModifyLoadBalancerAttributesOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModifyLoadBalancerAttributesResponse {
    /// Information about the load balancer attributes.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<LoadBalancerAttribute>,
}

impl ModifyLoadBalancerAttributesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { attributes: vec![] }
    }
}
