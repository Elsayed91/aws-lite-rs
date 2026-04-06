//! Types for the Amazon Auto Scaling API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `autoscaling.v1.AutoScalingGroupsType`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//AutoScalingGroupsType>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeAutoScalingGroupsResponse {
    /// The groups.
    #[serde(default)]
    pub auto_scaling_groups: Vec<AutoScalingGroup>,

    /// A string that indicates that the response contains more items than can be returned in a
    /// single response. To receive additional items, specify this string for the NextToken
    /// value when requesting the next set of items. This value is null when there are no more
    /// items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeAutoScalingGroupsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            auto_scaling_groups: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes an Auto Scaling group.
///
/// **AWS API**: `autoscaling.v1.AutoScalingGroup`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//AutoScalingGroup>
///
/// ## Coverage
/// 9 of 40 fields included.
/// Omitted fields:
/// - `PredictedCapacity` — not selected in manifest
/// - `DefaultCooldown` — not selected in manifest
/// - `LoadBalancerNames` — not selected in manifest
/// - `TargetGroupARNs` — not selected in manifest
/// - `HealthCheckType` — not selected in manifest
/// - `HealthCheckGracePeriod` — not selected in manifest
/// - `Instances` — not selected in manifest
/// - `CreatedTime` — not selected in manifest
/// - `SuspendedProcesses` — not selected in manifest
/// - `PlacementGroup` — not selected in manifest
/// - `VPCZoneIdentifier` — not selected in manifest
/// - `EnabledMetrics` — not selected in manifest
/// - `Status` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `TerminationPolicies` — not selected in manifest
/// - `NewInstancesProtectedFromScaleIn` — not selected in manifest
/// - `ServiceLinkedRoleARN` — not selected in manifest
/// - `MaxInstanceLifetime` — not selected in manifest
/// - `CapacityRebalance` — not selected in manifest
/// - `WarmPoolConfiguration` — not selected in manifest
/// - `WarmPoolSize` — not selected in manifest
/// - `Context` — not selected in manifest
/// - `DesiredCapacityType` — not selected in manifest
/// - `DefaultInstanceWarmup` — not selected in manifest
/// - `TrafficSources` — not selected in manifest
/// - `InstanceMaintenancePolicy` — not selected in manifest
/// - `DeletionProtection` — not selected in manifest
/// - `AvailabilityZoneDistribution` — not selected in manifest
/// - `AvailabilityZoneImpairmentPolicy` — not selected in manifest
/// - `CapacityReservationSpecification` — not selected in manifest
/// - `InstanceLifecyclePolicy` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AutoScalingGroup {
    /// The name of the Auto Scaling group.
    pub auto_scaling_group_name: String,

    /// The Amazon Resource Name (ARN) of the Auto Scaling group.
    #[serde(rename = "AutoScalingGroupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_arn: Option<String>,

    /// One or more Availability Zones for the Auto Scaling group.
    #[serde(default)]
    pub availability_zones: Vec<String>,

    /// The desired size of the Auto Scaling group.
    pub desired_capacity: i32,

    /// The minimum size of the Auto Scaling group.
    pub min_size: i32,

    /// The maximum size of the Auto Scaling group.
    pub max_size: i32,

    /// The launch template for the Auto Scaling group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,

    /// The name of the associated launch configuration for the Auto Scaling group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,

    /// The mixed instances policy for the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
}

impl AutoScalingGroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            auto_scaling_group_name: "test-auto_scaling_group_name".into(),
            auto_scaling_group_arn: Some("test-auto_scaling_group_arn".into()),
            availability_zones: vec![],
            desired_capacity: 100,
            min_size: 100,
            max_size: 100,
            launch_template: Some(LaunchTemplateSpecification::fixture()),
            launch_configuration_name: Some("test-launch_configuration_name".into()),
            mixed_instances_policy: Some(MixedInstancesPolicy::fixture()),
        }
    }
}

/// Describes the launch template and the version of the launch template that Amazon EC2 Auto
/// Scaling uses to launch Amazon EC2 instances. For more information about launch templates,
/// see Launch templates in the Amazon EC2 Auto Scaling User Guide.
///
/// **AWS API**: `autoscaling.v1.LaunchTemplateSpecification`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//LaunchTemplateSpecification>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchTemplateSpecification {
    /// The ID of the launch template. To get the template ID, use the Amazon EC2
    /// DescribeLaunchTemplates API operation. New launch templates can be created using the
    /// Amazon EC2 CreateLaunchTemplate API. Conditional: You must specify either a
    /// LaunchTemplateId or a LaunchTemplateName.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,

    /// The name of the launch template. To get the template name, use the Amazon EC2
    /// DescribeLaunchTemplates API operation. New launch templates can be created using the
    /// Amazon EC2 CreateLaunchTemplate API. Conditional: You must specify either a
    /// LaunchTemplateId or a LaunchTemplateName.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,

    /// The version number, $Latest, or $Default. To get the version number, use the Amazon EC2
    /// DescribeLaunchTemplateVersions API operation. New launch template versions can be
    /// created using the Amazon EC2 CreateLaunchTemplateVersion API. If the value is $Latest,
    /// Amazon EC2 Auto Scaling selects the latest version of the launch template when launching
    /// instances. If the value is $Default, Amazon EC2 Auto Scaling selects the default version
    /// of the launch template when launching instances. The default value is $Default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl LaunchTemplateSpecification {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_template_id: Some("test-launch_template_id".into()),
            launch_template_name: Some("test-launch_template_name".into()),
            version: Some("test-version".into()),
        }
    }
}

/// Use this structure to launch multiple instance types and On-Demand Instances and Spot
/// Instances within a single Auto Scaling group. A mixed instances policy contains information
/// that Amazon EC2 Auto Scaling can use to launch instances and help optimize your costs. For
/// more information, see Auto Scaling groups with multiple instance types and purchase options
/// in the Amazon EC2 Auto Scaling User Guide.
///
/// **AWS API**: `autoscaling.v1.MixedInstancesPolicy`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//MixedInstancesPolicy>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `InstancesDistribution` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MixedInstancesPolicy {
    /// One or more launch templates and the instance types (overrides) that are used to launch
    /// EC2 instances to fulfill On-Demand and Spot capacities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplate>,
}

impl MixedInstancesPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_template: Some(LaunchTemplate::fixture()),
        }
    }
}

/// Use this structure to specify the launch templates and instance types (overrides) for a
/// mixed instances policy.
///
/// **AWS API**: `autoscaling.v1.LaunchTemplate`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//LaunchTemplate>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `Overrides` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchTemplate {
    /// The launch template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_specification: Option<LaunchTemplateSpecification>,
}

impl LaunchTemplate {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_template_specification: Some(LaunchTemplateSpecification::fixture()),
        }
    }
}

///
/// **AWS API**: `autoscaling.v1.LaunchConfigurationsType`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//LaunchConfigurationsType>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeLaunchConfigurationsResponse {
    /// The launch configurations.
    #[serde(default)]
    pub launch_configurations: Vec<LaunchConfiguration>,

    /// A string that indicates that the response contains more items than can be returned in a
    /// single response. To receive additional items, specify this string for the NextToken
    /// value when requesting the next set of items. This value is null when there are no more
    /// items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeLaunchConfigurationsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_configurations: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a launch configuration.
///
/// **AWS API**: `autoscaling.v1.LaunchConfiguration`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//LaunchConfiguration>
///
/// ## Coverage
/// 5 of 20 fields included.
/// Omitted fields:
/// - `LaunchConfigurationARN` — not selected in manifest
/// - `KeyName` — not selected in manifest
/// - `ClassicLinkVPCId` — not selected in manifest
/// - `ClassicLinkVPCSecurityGroups` — not selected in manifest
/// - `UserData` — not selected in manifest
/// - `KernelId` — not selected in manifest
/// - `RamdiskId` — not selected in manifest
/// - `BlockDeviceMappings` — not selected in manifest
/// - `InstanceMonitoring` — not selected in manifest
/// - `SpotPrice` — not selected in manifest
/// - `IamInstanceProfile` — not selected in manifest
/// - `CreatedTime` — not selected in manifest
/// - `EbsOptimized` — not selected in manifest
/// - `PlacementTenancy` — not selected in manifest
/// - `MetadataOptions` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchConfiguration {
    /// The name of the launch configuration.
    pub launch_configuration_name: String,

    /// The ID of the Amazon Machine Image (AMI) to use to launch your EC2 instances. For more
    /// information, see Find a Linux AMI in the Amazon EC2 User Guide.
    pub image_id: String,

    /// The instance type for the instances. For information about available instance types, see
    /// Available instance types in the Amazon EC2 User Guide.
    pub instance_type: String,

    /// A list that contains the security groups to assign to the instances in the Auto Scaling
    /// group. For more information, see Control traffic to your Amazon Web Services resources
    /// using security groups in the Amazon Virtual Private Cloud User Guide.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_groups: Vec<String>,

    /// Specifies whether to assign a public IPv4 address to the group's instances. If the
    /// instance is launched into a default subnet, the default is to assign a public IPv4
    /// address, unless you disabled the option to assign a public IPv4 address on the subnet.
    /// If the instance is launched into a nondefault subnet, the default is not to assign a
    /// public IPv4 address, unless you enabled the option to assign a public IPv4 address on
    /// the subnet. For more information, see Provide network connectivity for your Auto Scaling
    /// instances using Amazon VPC in the Amazon EC2 Auto Scaling User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
}

impl LaunchConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_configuration_name: "test-launch_configuration_name".into(),
            image_id: "test-image_id".into(),
            instance_type: "test-instance_type".into(),
            security_groups: vec![],
            associate_public_ip_address: Some(false),
        }
    }
}

///
/// **AWS API**: `autoscaling.v1.AutoScalingGroupNamesType`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//AutoScalingGroupNamesType>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `IncludeInstances` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeAutoScalingGroupsRequest {
    /// The names of the Auto Scaling groups. By default, you can only specify up to 50 names.
    /// You can optionally increase this limit using the MaxRecords property. If you omit this
    /// property, all Auto Scaling groups are described.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auto_scaling_group_names: Vec<String>,

    /// The token for the next set of items to return. (You received this token from a previous
    /// call.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return with this call. The default value is 50 and the
    /// maximum value is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

impl DescribeAutoScalingGroupsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            auto_scaling_group_names: vec![],
            next_token: Some("test-next_token".into()),
            max_records: Some(100),
        }
    }
}

///
/// **AWS API**: `autoscaling.v1.LaunchConfigurationNamesType`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//LaunchConfigurationNamesType>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeLaunchConfigurationsRequest {
    /// The launch configuration names. If you omit this property, all launch configurations are
    /// described. Array Members: Maximum number of 50 items.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub launch_configuration_names: Vec<String>,

    /// The token for the next set of items to return. (You received this token from a previous
    /// call.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return with this call. The default value is 50 and the
    /// maximum value is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

impl DescribeLaunchConfigurationsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_configuration_names: vec![],
            next_token: Some("test-next_token".into()),
            max_records: Some(100),
        }
    }
}

///
/// **AWS API**: `autoscaling.v1.UpdateAutoScalingGroupType`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//UpdateAutoScalingGroupType>
///
/// ## Coverage
/// 5 of 28 fields included.
/// Omitted fields:
/// - `LaunchConfigurationName` — not selected in manifest
/// - `MixedInstancesPolicy` — not selected in manifest
/// - `DefaultCooldown` — not selected in manifest
/// - `AvailabilityZones` — not selected in manifest
/// - `HealthCheckType` — not selected in manifest
/// - `HealthCheckGracePeriod` — not selected in manifest
/// - `PlacementGroup` — not selected in manifest
/// - `VPCZoneIdentifier` — not selected in manifest
/// - `TerminationPolicies` — not selected in manifest
/// - `NewInstancesProtectedFromScaleIn` — not selected in manifest
/// - `ServiceLinkedRoleARN` — not selected in manifest
/// - `MaxInstanceLifetime` — not selected in manifest
/// - `CapacityRebalance` — not selected in manifest
/// - `Context` — not selected in manifest
/// - `DesiredCapacityType` — not selected in manifest
/// - `DefaultInstanceWarmup` — not selected in manifest
/// - `InstanceMaintenancePolicy` — not selected in manifest
/// - `AvailabilityZoneDistribution` — not selected in manifest
/// - `AvailabilityZoneImpairmentPolicy` — not selected in manifest
/// - `SkipZonalShiftValidation` — not selected in manifest
/// - `CapacityReservationSpecification` — not selected in manifest
/// - `InstanceLifecyclePolicy` — not selected in manifest
/// - `DeletionProtection` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateAutoScalingGroupRequest {
    /// The name of the Auto Scaling group.
    pub auto_scaling_group_name: String,

    /// The launch template and version to use to specify the updates. If you specify
    /// LaunchTemplate in your update request, you can't specify LaunchConfigurationName or
    /// MixedInstancesPolicy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,

    /// The desired capacity is the initial capacity of the Auto Scaling group after this
    /// operation completes and the capacity it attempts to maintain. This number must be
    /// greater than or equal to the minimum size of the group and less than or equal to the
    /// maximum size of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<i32>,

    /// The minimum size of the Auto Scaling group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,

    /// The maximum size of the Auto Scaling group. With a mixed instances policy that uses
    /// instance weighting, Amazon EC2 Auto Scaling may need to go above MaxSize to meet your
    /// capacity requirements. In this event, Amazon EC2 Auto Scaling will never go above
    /// MaxSize by more than your largest instance weight (weights that define how many units
    /// each instance contributes to the desired capacity of the group).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
}

impl UpdateAutoScalingGroupRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            auto_scaling_group_name: "test-auto_scaling_group_name".into(),
            launch_template: Some(LaunchTemplateSpecification::fixture()),
            desired_capacity: Some(100),
            min_size: Some(100),
            max_size: Some(100),
        }
    }
}

///
/// **AWS API**: `autoscaling.v1.StartInstanceRefreshType`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//StartInstanceRefreshType>
///
/// ## Coverage
/// 3 of 4 fields included.
/// Omitted fields:
/// - `DesiredConfiguration` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartInstanceRefreshRequest {
    /// The name of the Auto Scaling group.
    pub auto_scaling_group_name: String,

    /// The strategy to use for the instance refresh. The default value is Rolling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,

    /// Sets your preferences for the instance refresh so that it performs as expected when you
    /// start it. Includes the instance warmup time, the minimum and maximum healthy
    /// percentages, and the behaviors that you want Amazon EC2 Auto Scaling to use if instances
    /// that are in Standby state or protected from scale in are found. You can also choose to
    /// enable additional features, such as the following: Auto rollback Checkpoints CloudWatch
    /// alarms Skip matching Bake time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences: Option<RefreshPreferences>,
}

impl StartInstanceRefreshRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            auto_scaling_group_name: "test-auto_scaling_group_name".into(),
            strategy: Some("test-strategy".into()),
            preferences: Some(RefreshPreferences::fixture()),
        }
    }
}

/// Describes the preferences for an instance refresh.
///
/// **AWS API**: `autoscaling.v1.RefreshPreferences`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//RefreshPreferences>
///
/// ## Coverage
/// 2 of 11 fields included.
/// Omitted fields:
/// - `CheckpointPercentages` — not selected in manifest
/// - `CheckpointDelay` — not selected in manifest
/// - `SkipMatching` — not selected in manifest
/// - `AutoRollback` — not selected in manifest
/// - `ScaleInProtectedInstances` — not selected in manifest
/// - `StandbyInstances` — not selected in manifest
/// - `AlarmSpecification` — not selected in manifest
/// - `MaxHealthyPercentage` — not selected in manifest
/// - `BakeTime` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RefreshPreferences {
    /// Specifies the minimum percentage of the group to keep in service, healthy, and ready to
    /// use to support your workload to allow the operation to continue. The value is expressed
    /// as a percentage of the desired capacity of the Auto Scaling group. Value range is 0 to
    /// 100. If you do not specify this property, the default is 90 percent, or the percentage
    /// set in the instance maintenance policy for the Auto Scaling group, if defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_healthy_percentage: Option<i32>,

    /// A time period, in seconds, during which an instance refresh waits before moving on to
    /// replacing the next instance after a new instance enters the InService state. This
    /// property is not required for normal usage. Instead, use the DefaultInstanceWarmup
    /// property of the Auto Scaling group. The InstanceWarmup and DefaultInstanceWarmup
    /// properties work the same way. Only specify this property if you must override the
    /// DefaultInstanceWarmup property. If you do not specify this property, the instance warmup
    /// by default is the value of the DefaultInstanceWarmup property, if defined (which is
    /// recommended in all cases), or the HealthCheckGracePeriod property otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_warmup: Option<i32>,
}

impl RefreshPreferences {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            min_healthy_percentage: Some(100),
            instance_warmup: Some(100),
        }
    }
}

///
/// **AWS API**: `autoscaling.v1.StartInstanceRefreshAnswer`
/// **Reference**: <https://docs.aws.amazon.com/autoscaling/ec2/APIReference//StartInstanceRefreshAnswer>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartInstanceRefreshResponse {
    /// A unique ID for tracking the progress of the instance refresh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_refresh_id: Option<String>,
}

impl StartInstanceRefreshResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_refresh_id: Some("test-instance_refresh_id".into()),
        }
    }
}
