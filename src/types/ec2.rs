//! Types for the Amazon EC2 API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `ec2.v1.DescribeInstancesResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeInstancesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstancesResponse {
    /// Information about the reservations.
    #[serde(rename(serialize = "ReservationSet", deserialize = "reservationSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reservations: Vec<Reservation>,

    /// The token to include in another request to get the next page of items. This value is
    /// null when there are no more items to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeInstancesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            reservations: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a launch request for one or more instances, and includes owner, requester, and
/// security group information that applies to all instances in the launch request.
///
/// **AWS API**: `ec2.v1.Reservation`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Reservation>
///
/// ## Coverage
/// 2 of 5 fields included.
/// Omitted fields:
/// - `OwnerId` — not selected in manifest
/// - `RequesterId` — not selected in manifest
/// - `Groups` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Reservation {
    /// The ID of the reservation.
    #[serde(rename(serialize = "ReservationId", deserialize = "reservationId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,

    /// The instances.
    #[serde(rename(serialize = "InstancesSet", deserialize = "instancesSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<Instance>,
}

impl Reservation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            reservation_id: Some("test-reservation_id".into()),
            instances: vec![],
        }
    }
}

/// Describes an instance.
///
/// **AWS API**: `ec2.v1.Instance`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Instance>
///
/// ## Coverage
/// 18 of 61 fields included.
/// Omitted fields:
/// - `Architecture` — not selected in manifest
/// - `ClientToken` — not selected in manifest
/// - `EnaSupport` — not selected in manifest
/// - `Hypervisor` — not selected in manifest
/// - `InstanceLifecycle` — not selected in manifest
/// - `ElasticGpuAssociations` — not selected in manifest
/// - `ElasticInferenceAcceleratorAssociations` — not selected in manifest
/// - `OutpostArn` — not selected in manifest
/// - `RootDeviceName` — not selected in manifest
/// - `RootDeviceType` — not selected in manifest
/// - `SourceDestCheck` — not selected in manifest
/// - `SpotInstanceRequestId` — not selected in manifest
/// - `SriovNetSupport` — not selected in manifest
/// - `StateReason` — not selected in manifest
/// - `CpuOptions` — not selected in manifest
/// - `CapacityBlockId` — not selected in manifest
/// - `CapacityReservationId` — not selected in manifest
/// - `CapacityReservationSpecification` — not selected in manifest
/// - `HibernationOptions` — not selected in manifest
/// - `Licenses` — not selected in manifest
/// - `EnclaveOptions` — not selected in manifest
/// - `BootMode` — not selected in manifest
/// - `PlatformDetails` — not selected in manifest
/// - `UsageOperation` — not selected in manifest
/// - `UsageOperationUpdateTime` — not selected in manifest
/// - `PrivateDnsNameOptions` — not selected in manifest
/// - `Ipv6Address` — not selected in manifest
/// - `TpmSupport` — not selected in manifest
/// - `MaintenanceOptions` — not selected in manifest
/// - `CurrentInstanceBootMode` — not selected in manifest
/// - `NetworkPerformanceOptions` — not selected in manifest
/// - `Operator` — not selected in manifest
/// - `SecondaryInterfaces` — not selected in manifest
/// - `PrivateDnsName` — not selected in manifest
/// - `PublicDnsName` — not selected in manifest
/// - `StateTransitionReason` — not selected in manifest
/// - `KeyName` — not selected in manifest
/// - `AmiLaunchIndex` — not selected in manifest
/// - `ProductCodes` — not selected in manifest
/// - `KernelId` — not selected in manifest
/// - `RamdiskId` — not selected in manifest
/// - `Platform` — not selected in manifest
/// - `PrivateIpAddress` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Instance {
    /// The ID of the instance.
    #[serde(rename(serialize = "InstanceId", deserialize = "instanceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// The instance type.
    #[serde(rename(serialize = "InstanceType", deserialize = "instanceType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,

    /// The current state of the instance.
    #[serde(rename(serialize = "InstanceState", deserialize = "instanceState"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,

    /// The location where the instance launched, if applicable.
    #[serde(rename(serialize = "Placement", deserialize = "placement"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,

    /// The ID of the subnet in which the instance is running.
    #[serde(rename(serialize = "SubnetId", deserialize = "subnetId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,

    /// The ID of the VPC in which the instance is running.
    #[serde(rename(serialize = "VpcId", deserialize = "vpcId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// The public IPv4 address, or the Carrier IP address assigned to the instance, if
    /// applicable. A Carrier IP address only applies to an instance launched in a subnet
    /// associated with a Wavelength Zone.
    #[serde(rename(serialize = "IpAddress", deserialize = "ipAddress"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,

    /// Any block device mapping entries for the instance.
    #[serde(rename(serialize = "BlockDeviceMapping", deserialize = "blockDeviceMapping"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub block_device_mappings: Vec<InstanceBlockDeviceMapping>,

    /// The security groups for the instance.
    #[serde(rename(serialize = "GroupSet", deserialize = "groupSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_groups: Vec<GroupIdentifier>,

    /// The time that the instance was last launched. To determine the time that instance was
    /// first launched, see the attachment time for the primary network interface.
    #[serde(rename(serialize = "LaunchTime", deserialize = "launchTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<String>,

    /// The ID of the AMI used to launch the instance.
    #[serde(rename(serialize = "ImageId", deserialize = "imageId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,

    /// The IAM instance profile associated with the instance, if applicable.
    #[serde(rename(serialize = "IamInstanceProfile", deserialize = "iamInstanceProfile"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<IamInstanceProfile>,

    /// The monitoring for the instance.
    #[serde(rename(serialize = "Monitoring", deserialize = "monitoring"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<Monitoring>,

    /// The metadata options for the instance.
    #[serde(rename(serialize = "MetadataOptions", deserialize = "metadataOptions"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_options: Option<InstanceMetadataOptionsResponse>,

    /// Indicates whether the instance is optimized for Amazon EBS I/O. This optimization
    /// provides dedicated throughput to Amazon EBS and an optimized configuration stack to
    /// provide optimal I/O performance. This optimization isn't available with all instance
    /// types. Additional usage charges apply when using an EBS Optimized instance.
    #[serde(rename(serialize = "EbsOptimized", deserialize = "ebsOptimized"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,

    /// The virtualization type of the instance.
    #[serde(rename(serialize = "VirtualizationType", deserialize = "virtualizationType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtualization_type: Option<String>,

    /// The network interfaces for the instance.
    #[serde(rename(serialize = "NetworkInterfaceSet", deserialize = "networkInterfaceSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<InstanceNetworkInterface>,

    /// Any tags assigned to the instance.
    #[serde(rename(serialize = "TagSet", deserialize = "tagSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

impl Instance {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_id: Some("test-instance_id".into()),
            instance_type: Some("test-instance_type".into()),
            state: Some(InstanceState::fixture()),
            placement: Some(Placement::fixture()),
            subnet_id: Some("test-subnet_id".into()),
            vpc_id: Some("test-vpc_id".into()),
            public_ip_address: Some("test-public_ip_address".into()),
            block_device_mappings: vec![],
            security_groups: vec![],
            launch_time: Some("test-launch_time".into()),
            image_id: Some("test-image_id".into()),
            iam_instance_profile: Some(IamInstanceProfile::fixture()),
            monitoring: Some(Monitoring::fixture()),
            metadata_options: Some(InstanceMetadataOptionsResponse::fixture()),
            ebs_optimized: Some(false),
            virtualization_type: Some("test-virtualization_type".into()),
            network_interfaces: vec![],
            tags: vec![],
        }
    }
}

/// Describes the current state of an instance.
///
/// **AWS API**: `ec2.v1.InstanceState`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//InstanceState>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceState {
    /// The state of the instance as a 16-bit unsigned integer. The high byte is all of the bits
    /// between 2^8 and (2^16)-1, which equals decimal values between 256 and 65,535. These
    /// numerical values are used for internal purposes and should be ignored. The low byte is
    /// all of the bits between 2^0 and (2^8)-1, which equals decimal values between 0 and 255.
    /// The valid values for instance-state-code will all be in the range of the low byte and
    /// they are: 0 : pending 16 : running 32 : shutting-down 48 : terminated 64 : stopping 80 :
    /// stopped You can ignore the high byte value by zeroing out all of the bits above 2^8 or
    /// 256 in decimal.
    #[serde(rename(serialize = "Code", deserialize = "code"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,

    /// The current state of the instance.
    #[serde(rename(serialize = "Name", deserialize = "name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl InstanceState {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            code: Some(100),
            name: Some("test-name".into()),
        }
    }
}

/// Describes the placement of an instance.
///
/// **AWS API**: `ec2.v1.Placement`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Placement>
///
/// ## Coverage
/// 1 of 10 fields included.
/// Omitted fields:
/// - `AvailabilityZoneId` — not selected in manifest
/// - `Affinity` — not selected in manifest
/// - `GroupName` — not selected in manifest
/// - `PartitionNumber` — not selected in manifest
/// - `HostId` — not selected in manifest
/// - `Tenancy` — not selected in manifest
/// - `SpreadDomain` — not selected in manifest
/// - `HostResourceGroupArn` — not selected in manifest
/// - `GroupId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Placement {
    /// The Availability Zone of the instance. On input, you can specify AvailabilityZone or
    /// AvailabilityZoneId, but not both. If you specify neither one, Amazon EC2 automatically
    /// selects an Availability Zone for you. This parameter is not supported for CreateFleet.
    #[serde(rename(serialize = "AvailabilityZone", deserialize = "availabilityZone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
}

impl Placement {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            availability_zone: Some("test-availability_zone".into()),
        }
    }
}

/// Describes a block device mapping.
///
/// **AWS API**: `ec2.v1.InstanceBlockDeviceMapping`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//InstanceBlockDeviceMapping>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceBlockDeviceMapping {
    /// The device name.
    #[serde(rename(serialize = "DeviceName", deserialize = "deviceName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,

    /// Parameters used to automatically set up EBS volumes when the instance is launched.
    #[serde(rename(serialize = "Ebs", deserialize = "ebs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs: Option<EbsInstanceBlockDevice>,
}

impl InstanceBlockDeviceMapping {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            device_name: Some("test-device_name".into()),
            ebs: Some(EbsInstanceBlockDevice::fixture()),
        }
    }
}

/// Describes a parameter used to set up an EBS volume in a block device mapping.
///
/// **AWS API**: `ec2.v1.EbsInstanceBlockDevice`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//EbsInstanceBlockDevice>
///
/// ## Coverage
/// 2 of 8 fields included.
/// Omitted fields:
/// - `AttachTime` — not selected in manifest
/// - `DeleteOnTermination` — not selected in manifest
/// - `AssociatedResource` — not selected in manifest
/// - `VolumeOwnerId` — not selected in manifest
/// - `Operator` — not selected in manifest
/// - `EbsCardIndex` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EbsInstanceBlockDevice {
    /// The ID of the EBS volume.
    #[serde(rename(serialize = "VolumeId", deserialize = "volumeId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,

    /// The attachment state.
    #[serde(rename(serialize = "Status", deserialize = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl EbsInstanceBlockDevice {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            volume_id: Some("test-volume_id".into()),
            status: Some("test-status".into()),
        }
    }
}

/// Describes an IAM instance profile.
///
/// **AWS API**: `ec2.v1.IamInstanceProfile`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//IamInstanceProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IamInstanceProfile {
    /// The Amazon Resource Name (ARN) of the instance profile.
    #[serde(rename(serialize = "Arn", deserialize = "arn"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The ID of the instance profile.
    #[serde(rename(serialize = "Id", deserialize = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl IamInstanceProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            arn: Some("test-arn".into()),
            id: Some("test-id".into()),
        }
    }
}

/// Describes the monitoring of an instance.
///
/// **AWS API**: `ec2.v1.Monitoring`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Monitoring>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Monitoring {
    /// Indicates whether detailed monitoring is enabled. Otherwise, basic monitoring is
    /// enabled.
    #[serde(rename(serialize = "State", deserialize = "state"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl Monitoring {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            state: Some("test-state".into()),
        }
    }
}

/// The metadata options for the instance.
///
/// **AWS API**: `ec2.v1.InstanceMetadataOptionsResponse`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//InstanceMetadataOptionsResponse>
///
/// ## Coverage
/// 2 of 6 fields included.
/// Omitted fields:
/// - `State` — not selected in manifest
/// - `HttpPutResponseHopLimit` — not selected in manifest
/// - `HttpProtocolIpv6` — not selected in manifest
/// - `InstanceMetadataTags` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceMetadataOptionsResponse {
    /// Indicates whether IMDSv2 is required. optional
    /// - IMDSv2 is optional, which means that you can use either IMDSv2 or IMDSv1. required
    /// - IMDSv2 is required, which means that IMDSv1 is disabled, and you must use IMDSv2.
    #[serde(rename(serialize = "HttpTokens", deserialize = "httpTokens"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_tokens: Option<String>,

    /// Indicates whether the HTTP metadata endpoint on your instances is enabled or disabled.
    /// If the value is disabled, you cannot access your instance metadata.
    #[serde(rename(serialize = "HttpEndpoint", deserialize = "httpEndpoint"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<String>,
}

impl InstanceMetadataOptionsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            http_tokens: Some("test-http_tokens".into()),
            http_endpoint: Some("test-http_endpoint".into()),
        }
    }
}

/// Describes a network interface.
///
/// **AWS API**: `ec2.v1.InstanceNetworkInterface`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//InstanceNetworkInterface>
///
/// ## Coverage
/// 1 of 20 fields included.
/// Omitted fields:
/// - `Association` — not selected in manifest
/// - `Attachment` — not selected in manifest
/// - `Description` — not selected in manifest
/// - `Groups` — not selected in manifest
/// - `Ipv6Addresses` — not selected in manifest
/// - `MacAddress` — not selected in manifest
/// - `OwnerId` — not selected in manifest
/// - `PrivateDnsName` — not selected in manifest
/// - `PrivateIpAddress` — not selected in manifest
/// - `PrivateIpAddresses` — not selected in manifest
/// - `SourceDestCheck` — not selected in manifest
/// - `Status` — not selected in manifest
/// - `SubnetId` — not selected in manifest
/// - `VpcId` — not selected in manifest
/// - `InterfaceType` — not selected in manifest
/// - `Ipv4Prefixes` — not selected in manifest
/// - `Ipv6Prefixes` — not selected in manifest
/// - `ConnectionTrackingConfiguration` — not selected in manifest
/// - `Operator` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceNetworkInterface {
    /// The ID of the network interface.
    #[serde(rename(serialize = "NetworkInterfaceId", deserialize = "networkInterfaceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
}

impl InstanceNetworkInterface {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            network_interface_id: Some("test-network_interface_id".into()),
        }
    }
}

/// Describes a security group.
///
/// **AWS API**: `ec2.v1.GroupIdentifier`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//GroupIdentifier>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupIdentifier {
    /// The ID of the security group.
    #[serde(rename(serialize = "GroupId", deserialize = "groupId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    /// The name of the security group.
    #[serde(rename(serialize = "GroupName", deserialize = "groupName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl GroupIdentifier {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_id: Some("test-group_id".into()),
            group_name: Some("test-group_name".into()),
        }
    }
}

/// Describes a tag.
///
/// **AWS API**: `ec2.v1.Tag`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Tag>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tag {
    /// The key of the tag. Constraints: Tag keys are case-sensitive and accept a maximum of 127
    /// Unicode characters. May not begin with aws:.
    #[serde(rename(serialize = "Key", deserialize = "key"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// The value of the tag. Constraints: Tag values are case-sensitive and accept a maximum of
    /// 256 Unicode characters.
    #[serde(rename(serialize = "Value", deserialize = "value"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Tag {
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
/// **AWS API**: `ec2.v1.DescribeVolumesResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeVolumesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVolumesResponse {
    /// Information about the volumes.
    #[serde(rename(serialize = "VolumeSet", deserialize = "volumeSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<Volume>,

    /// The token to include in another request to get the next page of items. This value is
    /// null when there are no more items to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeVolumesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            volumes: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a volume.
///
/// **AWS API**: `ec2.v1.Volume`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Volume>
///
/// ## Coverage
/// 12 of 21 fields included.
/// Omitted fields:
/// - `AvailabilityZoneId` — not selected in manifest
/// - `OutpostArn` — not selected in manifest
/// - `SourceVolumeId` — not selected in manifest
/// - `FastRestored` — not selected in manifest
/// - `MultiAttachEnabled` — not selected in manifest
/// - `SseType` — not selected in manifest
/// - `Operator` — not selected in manifest
/// - `VolumeInitializationRate` — not selected in manifest
/// - `SnapshotId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Volume {
    /// The ID of the volume.
    #[serde(rename(serialize = "VolumeId", deserialize = "volumeId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,

    /// The Availability Zone for the volume.
    #[serde(rename(serialize = "AvailabilityZone", deserialize = "availabilityZone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,

    /// The size of the volume, in GiBs.
    #[serde(rename(serialize = "Size", deserialize = "size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,

    /// The volume state.
    #[serde(rename(serialize = "Status", deserialize = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The volume type.
    #[serde(rename(serialize = "VolumeType", deserialize = "volumeType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,

    /// Indicates whether the volume is encrypted.
    #[serde(rename(serialize = "Encrypted", deserialize = "encrypted"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,

    /// The Amazon Resource Name (ARN) of the KMS key that was used to protect the volume
    /// encryption key for the volume.
    #[serde(rename(serialize = "KmsKeyId", deserialize = "kmsKeyId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    /// The time stamp when volume creation was initiated.
    #[serde(rename(serialize = "CreateTime", deserialize = "createTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// The number of I/O operations per second (IOPS). For gp3, io1, and io2 volumes, this
    /// represents the number of IOPS that are provisioned for the volume. For gp2 volumes, this
    /// represents the baseline performance of the volume and the rate at which the volume
    /// accumulates I/O credits for bursting.
    #[serde(rename(serialize = "Iops", deserialize = "iops"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,

    /// The throughput that the volume supports, in MiB/s.
    #[serde(rename(serialize = "Throughput", deserialize = "throughput"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i32>,

    /// This parameter is not returned by CreateVolume. Information about the volume
    /// attachments.
    #[serde(rename(serialize = "AttachmentSet", deserialize = "attachmentSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<VolumeAttachment>,

    /// Any tags assigned to the volume.
    #[serde(rename(serialize = "TagSet", deserialize = "tagSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

impl Volume {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            volume_id: Some("test-volume_id".into()),
            availability_zone: Some("test-availability_zone".into()),
            size: Some(100),
            state: Some("test-state".into()),
            volume_type: Some("test-volume_type".into()),
            encrypted: Some(false),
            kms_key_id: Some("test-kms_key_id".into()),
            create_time: Some("test-create_time".into()),
            iops: Some(100),
            throughput: Some(100),
            attachments: vec![],
            tags: vec![],
        }
    }
}

/// Describes volume attachment details.
///
/// **AWS API**: `ec2.v1.VolumeAttachment`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//VolumeAttachment>
///
/// ## Coverage
/// 4 of 9 fields included.
/// Omitted fields:
/// - `DeleteOnTermination` — not selected in manifest
/// - `AssociatedResource` — not selected in manifest
/// - `InstanceOwningService` — not selected in manifest
/// - `EbsCardIndex` — not selected in manifest
/// - `AttachTime` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VolumeAttachment {
    /// The ID of the instance. If the volume is attached to an Amazon Web Services-managed
    /// resource, this parameter returns null.
    #[serde(rename(serialize = "InstanceId", deserialize = "instanceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// The attachment state of the volume.
    #[serde(rename(serialize = "Status", deserialize = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The device name. If the volume is attached to an Amazon Web Services-managed resource,
    /// this parameter returns null.
    #[serde(rename(serialize = "Device", deserialize = "device"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,

    /// The ID of the volume.
    #[serde(rename(serialize = "VolumeId", deserialize = "volumeId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

impl VolumeAttachment {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_id: Some("test-instance_id".into()),
            state: Some("test-state".into()),
            device: Some("test-device".into()),
            volume_id: Some("test-volume_id".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeSnapshotsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeSnapshotsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSnapshotsResponse {
    /// Information about the snapshots.
    #[serde(rename(serialize = "SnapshotSet", deserialize = "snapshotSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub snapshots: Vec<Snapshot>,

    /// The token to include in another request to get the next page of items. This value is
    /// null when there are no more items to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeSnapshotsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshots: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a snapshot.
///
/// **AWS API**: `ec2.v1.Snapshot`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Snapshot>
///
/// ## Coverage
/// 8 of 23 fields included.
/// Omitted fields:
/// - `OwnerAlias` — not selected in manifest
/// - `OutpostArn` — not selected in manifest
/// - `StorageTier` — not selected in manifest
/// - `RestoreExpiryTime` — not selected in manifest
/// - `SseType` — not selected in manifest
/// - `AvailabilityZone` — not selected in manifest
/// - `TransferType` — not selected in manifest
/// - `CompletionDurationMinutes` — not selected in manifest
/// - `CompletionTime` — not selected in manifest
/// - `FullSnapshotSizeInBytes` — not selected in manifest
/// - `StateMessage` — not selected in manifest
/// - `Progress` — not selected in manifest
/// - `OwnerId` — not selected in manifest
/// - `KmsKeyId` — not selected in manifest
/// - `DataEncryptionKeyId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Snapshot {
    /// The ID of the snapshot. Each snapshot receives a unique identifier when it is created.
    #[serde(rename(serialize = "SnapshotId", deserialize = "snapshotId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,

    /// The ID of the volume that was used to create the snapshot. Snapshots created by a copy
    /// snapshot operation have an arbitrary volume ID that you should not use for any purpose.
    #[serde(rename(serialize = "VolumeId", deserialize = "volumeId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,

    /// The size of the volume, in GiB.
    #[serde(rename(serialize = "VolumeSize", deserialize = "volumeSize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i32>,

    /// The snapshot state.
    #[serde(rename(serialize = "Status", deserialize = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The time stamp when the snapshot was initiated.
    #[serde(rename(serialize = "StartTime", deserialize = "startTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// The description for the snapshot.
    #[serde(rename(serialize = "Description", deserialize = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Indicates whether the snapshot is encrypted.
    #[serde(rename(serialize = "Encrypted", deserialize = "encrypted"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,

    /// Any tags assigned to the snapshot.
    #[serde(rename(serialize = "TagSet", deserialize = "tagSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

impl Snapshot {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshot_id: Some("test-snapshot_id".into()),
            volume_id: Some("test-volume_id".into()),
            volume_size: Some(100),
            state: Some("test-state".into()),
            start_time: Some("test-start_time".into()),
            description: Some("test-description".into()),
            encrypted: Some(false),
            tags: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeSnapshotAttributeResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeSnapshotAttributeResult>
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `ProductCodes` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSnapshotAttributeResponse {
    /// The ID of the EBS snapshot.
    #[serde(rename(serialize = "SnapshotId", deserialize = "snapshotId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,

    /// The users and groups that have the permissions for creating volumes from the snapshot.
    #[serde(rename(
        serialize = "CreateVolumePermission",
        deserialize = "createVolumePermission"
    ))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub create_volume_permissions: Vec<CreateVolumePermission>,
}

impl DescribeSnapshotAttributeResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshot_id: Some("test-snapshot_id".into()),
            create_volume_permissions: vec![],
        }
    }
}

/// Describes the user or group to be added or removed from the list of create volume
/// permissions for a volume.
///
/// **AWS API**: `ec2.v1.CreateVolumePermission`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//CreateVolumePermission>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVolumePermission {
    /// The group to be added or removed. The possible value is all.
    #[serde(rename(serialize = "Group", deserialize = "group"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// The ID of the Amazon Web Services account to be added or removed.
    #[serde(rename(serialize = "UserId", deserialize = "userId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreateVolumePermission {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group: Some("test-group".into()),
            user_id: Some("test-user_id".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeImagesResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeImagesResult>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `NextToken` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeImagesResponse {
    /// Information about the images.
    #[serde(rename(serialize = "ImagesSet", deserialize = "imagesSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Image>,
}

impl DescribeImagesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { images: vec![] }
    }
}

/// Describes an image.
///
/// **AWS API**: `ec2.v1.Image`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Image>
///
/// ## Coverage
/// 10 of 37 fields included.
/// Omitted fields:
/// - `UsageOperation` — not selected in manifest
/// - `EnaSupport` — not selected in manifest
/// - `Hypervisor` — not selected in manifest
/// - `ImageOwnerAlias` — not selected in manifest
/// - `RootDeviceName` — not selected in manifest
/// - `RootDeviceType` — not selected in manifest
/// - `SriovNetSupport` — not selected in manifest
/// - `StateReason` — not selected in manifest
/// - `VirtualizationType` — not selected in manifest
/// - `BootMode` — not selected in manifest
/// - `TpmSupport` — not selected in manifest
/// - `DeprecationTime` — not selected in manifest
/// - `ImdsSupport` — not selected in manifest
/// - `SourceInstanceId` — not selected in manifest
/// - `DeregistrationProtection` — not selected in manifest
/// - `LastLaunchedTime` — not selected in manifest
/// - `ImageAllowed` — not selected in manifest
/// - `SourceImageId` — not selected in manifest
/// - `SourceImageRegion` — not selected in manifest
/// - `FreeTierEligible` — not selected in manifest
/// - `ImageLocation` — not selected in manifest
/// - `OwnerId` — not selected in manifest
/// - `ProductCodes` — not selected in manifest
/// - `Architecture` — not selected in manifest
/// - `KernelId` — not selected in manifest
/// - `RamdiskId` — not selected in manifest
/// - `Platform` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Image {
    /// The ID of the AMI.
    #[serde(rename(serialize = "ImageId", deserialize = "imageId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,

    /// The name of the AMI that was provided during image creation.
    #[serde(rename(serialize = "Name", deserialize = "name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The current state of the AMI. If the state is available, the image is successfully
    /// registered and can be used to launch an instance.
    #[serde(rename(serialize = "ImageState", deserialize = "imageState"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Indicates whether the image has public launch permissions. The value is true if this
    /// image has public launch permissions or false if it has only implicit and explicit launch
    /// permissions.
    #[serde(rename(serialize = "IsPublic", deserialize = "isPublic"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,

    /// The type of image.
    #[serde(rename(serialize = "ImageType", deserialize = "imageType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,

    /// The platform details associated with the billing code of the AMI. For more information,
    /// see Understand AMI billing information in the Amazon EC2 User Guide.
    #[serde(rename(serialize = "PlatformDetails", deserialize = "platformDetails"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_details: Option<String>,

    /// The date and time the image was created.
    #[serde(rename(serialize = "CreationDate", deserialize = "creationDate"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,

    /// The description of the AMI that was provided during image creation.
    #[serde(rename(serialize = "Description", deserialize = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Any block device mapping entries.
    #[serde(rename(serialize = "BlockDeviceMapping", deserialize = "blockDeviceMapping"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub block_device_mappings: Vec<BlockDeviceMapping>,

    /// Any tags assigned to the image.
    #[serde(rename(serialize = "TagSet", deserialize = "tagSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

impl Image {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_id: Some("test-image_id".into()),
            name: Some("test-name".into()),
            state: Some("test-state".into()),
            public: Some(false),
            image_type: Some("test-image_type".into()),
            platform_details: Some("test-platform_details".into()),
            creation_date: Some("test-creation_date".into()),
            description: Some("test-description".into()),
            block_device_mappings: vec![],
            tags: vec![],
        }
    }
}

/// Describes a block device mapping, which defines the EBS volumes and instance store volumes
/// to attach to an instance at launch.
///
/// **AWS API**: `ec2.v1.BlockDeviceMapping`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//BlockDeviceMapping>
///
/// ## Coverage
/// 2 of 4 fields included.
/// Omitted fields:
/// - `NoDevice` — not selected in manifest
/// - `VirtualName` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockDeviceMapping {
    /// The device name. For available device names, see Device names for volumes.
    #[serde(rename(serialize = "DeviceName", deserialize = "deviceName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,

    /// Parameters used to automatically set up EBS volumes when the instance is launched.
    #[serde(rename(serialize = "Ebs", deserialize = "ebs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs: Option<EbsBlockDevice>,
}

impl BlockDeviceMapping {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            device_name: Some("test-device_name".into()),
            ebs: Some(EbsBlockDevice::fixture()),
        }
    }
}

/// Describes a block device for an EBS volume.
///
/// **AWS API**: `ec2.v1.EbsBlockDevice`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//EbsBlockDevice>
///
/// ## Coverage
/// 4 of 13 fields included.
/// Omitted fields:
/// - `DeleteOnTermination` — not selected in manifest
/// - `Iops` — not selected in manifest
/// - `KmsKeyId` — not selected in manifest
/// - `Throughput` — not selected in manifest
/// - `OutpostArn` — not selected in manifest
/// - `AvailabilityZone` — not selected in manifest
/// - `VolumeInitializationRate` — not selected in manifest
/// - `AvailabilityZoneId` — not selected in manifest
/// - `EbsCardIndex` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EbsBlockDevice {
    /// The ID of the snapshot.
    #[serde(rename(serialize = "SnapshotId", deserialize = "snapshotId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,

    /// The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size.
    /// If you specify a snapshot, the default is the snapshot size. You can specify a volume
    /// size that is equal to or larger than the snapshot size. The following are the supported
    /// sizes for each volume type: gp2: 1 - 16,384 GiB gp3: 1 - 65,536 GiB io1: 4 - 16,384 GiB
    /// io2: 4 - 65,536 GiB st1 and sc1: 125 - 16,384 GiB standard: 1 - 1024 GiB
    #[serde(rename(serialize = "VolumeSize", deserialize = "volumeSize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i32>,

    /// The volume type. For more information, see Amazon EBS volume types in the Amazon EBS
    /// User Guide.
    #[serde(rename(serialize = "VolumeType", deserialize = "volumeType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,

    /// Indicates whether the encryption state of an EBS volume is changed while being restored
    /// from a backing snapshot. The effect of setting the encryption state to true depends on
    /// the volume origin (new or from a snapshot), starting encryption state, ownership, and
    /// whether encryption by default is enabled. For more information, see Amazon EBS
    /// encryption in the Amazon EBS User Guide. In no case can you remove encryption from an
    /// encrypted volume. Encrypted volumes can only be attached to instances that support
    /// Amazon EBS encryption. For more information, see Supported instance types. This
    /// parameter is not returned by DescribeImageAttribute. For CreateImage and RegisterImage,
    /// whether you can include this parameter, and the allowed values differ depending on the
    /// type of block device mapping you are creating. If you are creating a block device
    /// mapping for a new (empty) volume, you can include this parameter, and specify either
    /// true for an encrypted volume, or false for an unencrypted volume. If you omit this
    /// parameter, it defaults to false (unencrypted). If you are creating a block device
    /// mapping from an existing encrypted or unencrypted snapshot, you must omit this
    /// parameter. If you include this parameter, the request will fail, regardless of the value
    /// that you specify. If you are creating a block device mapping from an existing
    /// unencrypted volume, you can include this parameter, but you must specify false. If you
    /// specify true, the request will fail. In this case, we recommend that you omit the
    /// parameter. If you are creating a block device mapping from an existing encrypted volume,
    /// you can include this parameter, and specify either true or false. However, if you
    /// specify false, the parameter is ignored and the block device mapping is always
    /// encrypted. In this case, we recommend that you omit the parameter.
    #[serde(rename(serialize = "Encrypted", deserialize = "encrypted"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
}

impl EbsBlockDevice {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshot_id: Some("test-snapshot_id".into()),
            volume_size: Some(100),
            volume_type: Some("test-volume_type".into()),
            encrypted: Some(false),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeSecurityGroupsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeSecurityGroupsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecurityGroupsResponse {
    /// Information about the security groups.
    #[serde(rename(serialize = "SecurityGroupInfo", deserialize = "securityGroupInfo"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_groups: Vec<SecurityGroup>,

    /// The token to include in another request to get the next page of items. This value is
    /// null when there are no more items to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeSecurityGroupsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            security_groups: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a security group.
///
/// **AWS API**: `ec2.v1.SecurityGroup`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//SecurityGroup>
///
/// ## Coverage
/// 7 of 9 fields included.
/// Omitted fields:
/// - `SecurityGroupArn` — not selected in manifest
/// - `OwnerId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityGroup {
    /// The ID of the security group.
    #[serde(rename(serialize = "GroupId", deserialize = "groupId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    /// The name of the security group.
    #[serde(rename(serialize = "GroupName", deserialize = "groupName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// A description of the security group.
    #[serde(rename(serialize = "GroupDescription", deserialize = "groupDescription"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The ID of the VPC for the security group.
    #[serde(rename(serialize = "VpcId", deserialize = "vpcId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// The inbound rules associated with the security group.
    #[serde(rename(serialize = "IpPermissions", deserialize = "ipPermissions"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ip_permissions: Vec<IpPermission>,

    /// The outbound rules associated with the security group.
    #[serde(rename(serialize = "IpPermissionsEgress", deserialize = "ipPermissionsEgress"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ip_permissions_egress: Vec<IpPermission>,

    /// Any tags assigned to the security group.
    #[serde(rename(serialize = "TagSet", deserialize = "tagSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

impl SecurityGroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_id: Some("test-group_id".into()),
            group_name: Some("test-group_name".into()),
            description: Some("test-description".into()),
            vpc_id: Some("test-vpc_id".into()),
            ip_permissions: vec![],
            ip_permissions_egress: vec![],
            tags: vec![],
        }
    }
}

/// Describes the permissions for a security group rule.
///
/// **AWS API**: `ec2.v1.IpPermission`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//IpPermission>
///
/// ## Coverage
/// 5 of 7 fields included.
/// Omitted fields:
/// - `UserIdGroupPairs` — not selected in manifest
/// - `PrefixListIds` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IpPermission {
    /// The IP protocol name (tcp, udp, icmp, icmpv6) or number (see Protocol Numbers). Use -1
    /// to specify all protocols. When authorizing security group rules, specifying -1 or a
    /// protocol number other than tcp, udp, icmp, or icmpv6 allows traffic on all ports,
    /// regardless of any port range you specify. For tcp, udp, and icmp, you must specify a
    /// port range. For icmpv6, the port range is optional; if you omit the port range, traffic
    /// for all types and codes is allowed.
    #[serde(rename(serialize = "IpProtocol", deserialize = "ipProtocol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<String>,

    /// If the protocol is TCP or UDP, this is the start of the port range. If the protocol is
    /// ICMP or ICMPv6, this is the ICMP type or -1 (all ICMP types).
    #[serde(rename(serialize = "FromPort", deserialize = "fromPort"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i32>,

    /// If the protocol is TCP or UDP, this is the end of the port range. If the protocol is
    /// ICMP or ICMPv6, this is the ICMP code or -1 (all ICMP codes). If the start port is -1
    /// (all ICMP types), then the end port must be -1 (all ICMP codes).
    #[serde(rename(serialize = "ToPort", deserialize = "toPort"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i32>,

    /// The IPv4 address ranges.
    #[serde(rename(serialize = "IpRanges", deserialize = "ipRanges"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ip_ranges: Vec<IpRange>,

    /// The IPv6 address ranges.
    #[serde(rename(serialize = "Ipv6Ranges", deserialize = "ipv6Ranges"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ipv6_ranges: Vec<Ipv6Range>,
}

impl IpPermission {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ip_protocol: Some("test-ip_protocol".into()),
            from_port: Some(100),
            to_port: Some(100),
            ip_ranges: vec![],
            ipv6_ranges: vec![],
        }
    }
}

/// Describes an IPv4 address range.
///
/// **AWS API**: `ec2.v1.IpRange`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//IpRange>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IpRange {
    /// The IPv4 address range. You can either specify a CIDR block or a source security group,
    /// not both. To specify a single IPv4 address, use the /32 prefix length. Amazon Web
    /// Services canonicalizes IPv4 and IPv6 CIDRs. For example, if you specify 100.68.0.18/18
    /// for the CIDR block, Amazon Web Services canonicalizes the CIDR block to 100.68.0.0/18.
    /// Any subsequent DescribeSecurityGroups and DescribeSecurityGroupRules calls will return
    /// the canonicalized form of the CIDR block. Additionally, if you attempt to add another
    /// rule with the non-canonical form of the CIDR (such as 100.68.0.18/18) and there is
    /// already a rule for the canonicalized form of the CIDR block (such as 100.68.0.0/18), the
    /// API throws an duplicate rule error.
    #[serde(rename(serialize = "CidrIp", deserialize = "cidrIp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,

    /// A description for the security group rule that references this IPv4 address range.
    /// Constraints: Up to 255 characters in length. Allowed characters are a-z, A-Z, 0-9,
    /// spaces, and ._-:/()#,@[]+=&amp;;{}!$*
    #[serde(rename(serialize = "Description", deserialize = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl IpRange {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cidr_ip: Some("test-cidr_ip".into()),
            description: Some("test-description".into()),
        }
    }
}

/// Describes an IPv6 address range.
///
/// **AWS API**: `ec2.v1.Ipv6Range`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Ipv6Range>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ipv6Range {
    /// The IPv6 address range. You can either specify a CIDR block or a source security group,
    /// not both. To specify a single IPv6 address, use the /128 prefix length. Amazon Web
    /// Services canonicalizes IPv4 and IPv6 CIDRs. For example, if you specify 100.68.0.18/18
    /// for the CIDR block, Amazon Web Services canonicalizes the CIDR block to 100.68.0.0/18.
    /// Any subsequent DescribeSecurityGroups and DescribeSecurityGroupRules calls will return
    /// the canonicalized form of the CIDR block. Additionally, if you attempt to add another
    /// rule with the non-canonical form of the CIDR (such as 100.68.0.18/18) and there is
    /// already a rule for the canonicalized form of the CIDR block (such as 100.68.0.0/18), the
    /// API throws an duplicate rule error.
    #[serde(rename(serialize = "CidrIpv6", deserialize = "cidrIpv6"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<String>,

    /// A description for the security group rule that references this IPv6 address range.
    /// Constraints: Up to 255 characters in length. Allowed characters are a-z, A-Z, 0-9,
    /// spaces, and ._-:/()#,@[]+=&amp;;{}!$*
    #[serde(rename(serialize = "Description", deserialize = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Ipv6Range {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cidr_ipv6: Some("test-cidr_ipv6".into()),
            description: Some("test-description".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeAddressesResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeAddressesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAddressesResponse {
    /// Information about the Elastic IP addresses.
    #[serde(rename(serialize = "AddressesSet", deserialize = "addressesSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<Address>,
}

impl DescribeAddressesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { addresses: vec![] }
    }
}

/// Describes an Elastic IP address, or a carrier IP address.
///
/// **AWS API**: `ec2.v1.Address`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Address>
///
/// ## Coverage
/// 6 of 16 fields included.
/// Omitted fields:
/// - `NetworkInterfaceOwnerId` — not selected in manifest
/// - `PrivateIpAddress` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `PublicIpv4Pool` — not selected in manifest
/// - `NetworkBorderGroup` — not selected in manifest
/// - `CustomerOwnedIp` — not selected in manifest
/// - `CustomerOwnedIpv4Pool` — not selected in manifest
/// - `CarrierIp` — not selected in manifest
/// - `SubnetId` — not selected in manifest
/// - `ServiceManaged` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Address {
    /// The ID representing the allocation of the address.
    #[serde(rename(serialize = "AllocationId", deserialize = "allocationId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<String>,

    /// The Elastic IP address.
    #[serde(rename(serialize = "PublicIp", deserialize = "publicIp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,

    /// The ID of the instance that the address is associated with (if any).
    #[serde(rename(serialize = "InstanceId", deserialize = "instanceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// The ID representing the association of the address with an instance.
    #[serde(rename(serialize = "AssociationId", deserialize = "associationId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,

    /// The ID of the network interface.
    #[serde(rename(serialize = "NetworkInterfaceId", deserialize = "networkInterfaceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,

    /// The network (vpc).
    #[serde(rename(serialize = "Domain", deserialize = "domain"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl Address {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allocation_id: Some("test-allocation_id".into()),
            public_ip: Some("test-public_ip".into()),
            instance_id: Some("test-instance_id".into()),
            association_id: Some("test-association_id".into()),
            network_interface_id: Some("test-network_interface_id".into()),
            domain: Some("test-domain".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeNatGatewaysResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeNatGatewaysResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNatGatewaysResponse {
    /// Information about the NAT gateways.
    #[serde(rename(serialize = "NatGatewaySet", deserialize = "natGatewaySet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nat_gateways: Vec<NatGateway>,

    /// The token to include in another request to get the next page of items. This value is
    /// null when there are no more items to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeNatGatewaysResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            nat_gateways: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a NAT gateway.
///
/// **AWS API**: `ec2.v1.NatGateway`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//NatGateway>
///
/// ## Coverage
/// 6 of 17 fields included.
/// Omitted fields:
/// - `DeleteTime` — not selected in manifest
/// - `FailureCode` — not selected in manifest
/// - `FailureMessage` — not selected in manifest
/// - `ProvisionedBandwidth` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `ConnectivityType` — not selected in manifest
/// - `AvailabilityMode` — not selected in manifest
/// - `AutoScalingIps` — not selected in manifest
/// - `AutoProvisionZones` — not selected in manifest
/// - `AttachedAppliances` — not selected in manifest
/// - `RouteTableId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NatGateway {
    /// The ID of the NAT gateway.
    #[serde(rename(serialize = "NatGatewayId", deserialize = "natGatewayId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_gateway_id: Option<String>,

    /// The state of the NAT gateway. pending: The NAT gateway is being created and is not ready
    /// to process traffic. failed: The NAT gateway could not be created. Check the failureCode
    /// and failureMessage fields for the reason. available: The NAT gateway is able to process
    /// traffic. This status remains until you delete the NAT gateway, and does not indicate the
    /// health of the NAT gateway. deleting: The NAT gateway is in the process of being
    /// terminated and may still be processing traffic. deleted: The NAT gateway has been
    /// terminated and is no longer processing traffic.
    #[serde(rename(serialize = "State", deserialize = "state"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The ID of the subnet in which the NAT gateway is located.
    #[serde(rename(serialize = "SubnetId", deserialize = "subnetId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,

    /// The ID of the VPC in which the NAT gateway is located.
    #[serde(rename(serialize = "VpcId", deserialize = "vpcId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// Information about the IP addresses and network interface associated with the NAT
    /// gateway.
    #[serde(rename(
        serialize = "NatGatewayAddressSet",
        deserialize = "natGatewayAddressSet"
    ))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nat_gateway_addresses: Vec<NatGatewayAddress>,

    /// The date and time the NAT gateway was created.
    #[serde(rename(serialize = "CreateTime", deserialize = "createTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

impl NatGateway {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            nat_gateway_id: Some("test-nat_gateway_id".into()),
            state: Some("test-state".into()),
            subnet_id: Some("test-subnet_id".into()),
            vpc_id: Some("test-vpc_id".into()),
            nat_gateway_addresses: vec![],
            create_time: Some("test-create_time".into()),
        }
    }
}

/// Describes the IP addresses and network interface associated with a NAT gateway.
///
/// **AWS API**: `ec2.v1.NatGatewayAddress`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//NatGatewayAddress>
///
/// ## Coverage
/// 2 of 10 fields included.
/// Omitted fields:
/// - `NetworkInterfaceId` — not selected in manifest
/// - `PrivateIp` — not selected in manifest
/// - `AssociationId` — not selected in manifest
/// - `IsPrimary` — not selected in manifest
/// - `FailureMessage` — not selected in manifest
/// - `Status` — not selected in manifest
/// - `AvailabilityZone` — not selected in manifest
/// - `AvailabilityZoneId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NatGatewayAddress {
    /// [Public NAT gateway only] The allocation ID of the Elastic IP address that's associated
    /// with the NAT gateway.
    #[serde(rename(serialize = "AllocationId", deserialize = "allocationId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<String>,

    /// [Public NAT gateway only] The Elastic IP address associated with the NAT gateway.
    #[serde(rename(serialize = "PublicIp", deserialize = "publicIp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
}

impl NatGatewayAddress {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allocation_id: Some("test-allocation_id".into()),
            public_ip: Some("test-public_ip".into()),
        }
    }
}

/// Contains the output of DescribeRouteTables.
///
/// **AWS API**: `ec2.v1.DescribeRouteTablesResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeRouteTablesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRouteTablesResponse {
    /// Information about the route tables.
    #[serde(rename(serialize = "RouteTableSet", deserialize = "routeTableSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub route_tables: Vec<RouteTable>,

    /// The token to include in another request to get the next page of items. This value is
    /// null when there are no more items to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeRouteTablesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            route_tables: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a route table.
///
/// **AWS API**: `ec2.v1.RouteTable`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//RouteTable>
///
/// ## Coverage
/// 4 of 7 fields included.
/// Omitted fields:
/// - `PropagatingVgws` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `OwnerId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RouteTable {
    /// The ID of the route table.
    #[serde(rename(serialize = "RouteTableId", deserialize = "routeTableId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_id: Option<String>,

    /// The ID of the VPC.
    #[serde(rename(serialize = "VpcId", deserialize = "vpcId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// The routes in the route table.
    #[serde(rename(serialize = "RouteSet", deserialize = "routeSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<Route>,

    /// The associations between the route table and your subnets or gateways.
    #[serde(rename(serialize = "AssociationSet", deserialize = "associationSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associations: Vec<RouteTableAssociation>,
}

impl RouteTable {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            route_table_id: Some("test-route_table_id".into()),
            vpc_id: Some("test-vpc_id".into()),
            routes: vec![],
            associations: vec![],
        }
    }
}

/// Describes a route in a route table.
///
/// **AWS API**: `ec2.v1.Route`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Route>
///
/// ## Coverage
/// 4 of 18 fields included.
/// Omitted fields:
/// - `DestinationIpv6CidrBlock` — not selected in manifest
/// - `DestinationPrefixListId` — not selected in manifest
/// - `EgressOnlyInternetGatewayId` — not selected in manifest
/// - `InstanceId` — not selected in manifest
/// - `InstanceOwnerId` — not selected in manifest
/// - `TransitGatewayId` — not selected in manifest
/// - `LocalGatewayId` — not selected in manifest
/// - `CarrierGatewayId` — not selected in manifest
/// - `NetworkInterfaceId` — not selected in manifest
/// - `Origin` — not selected in manifest
/// - `VpcPeeringConnectionId` — not selected in manifest
/// - `CoreNetworkArn` — not selected in manifest
/// - `OdbNetworkArn` — not selected in manifest
/// - `IpAddress` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Route {
    /// The IPv4 CIDR block used for the destination match.
    #[serde(rename(
        serialize = "DestinationCidrBlock",
        deserialize = "destinationCidrBlock"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_cidr_block: Option<String>,

    /// The ID of a NAT gateway.
    #[serde(rename(serialize = "NatGatewayId", deserialize = "natGatewayId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_gateway_id: Option<String>,

    /// The ID of a gateway attached to your VPC.
    #[serde(rename(serialize = "GatewayId", deserialize = "gatewayId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,

    /// The state of the route. The blackhole state indicates that the route's target isn't
    /// available (for example, the specified gateway isn't attached to the VPC, or the
    /// specified NAT instance has been terminated).
    #[serde(rename(serialize = "State", deserialize = "state"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl Route {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            destination_cidr_block: Some("test-destination_cidr_block".into()),
            nat_gateway_id: Some("test-nat_gateway_id".into()),
            gateway_id: Some("test-gateway_id".into()),
            state: Some("test-state".into()),
        }
    }
}

/// Describes an association between a route table and a subnet or gateway.
///
/// **AWS API**: `ec2.v1.RouteTableAssociation`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//RouteTableAssociation>
///
/// ## Coverage
/// 2 of 7 fields included.
/// Omitted fields:
/// - `RouteTableAssociationId` — not selected in manifest
/// - `RouteTableId` — not selected in manifest
/// - `GatewayId` — not selected in manifest
/// - `PublicIpv4Pool` — not selected in manifest
/// - `AssociationState` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RouteTableAssociation {
    /// The ID of the subnet. A subnet ID is not returned for an implicit association.
    #[serde(rename(serialize = "SubnetId", deserialize = "subnetId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,

    /// Indicates whether this is the main route table.
    #[serde(rename(serialize = "Main", deserialize = "main"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<bool>,
}

impl RouteTableAssociation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            subnet_id: Some("test-subnet_id".into()),
            main: Some(false),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeNetworkAclsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeNetworkAclsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkAclsResponse {
    /// Information about the network ACLs.
    #[serde(rename(serialize = "NetworkAclSet", deserialize = "networkAclSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub network_acls: Vec<NetworkAcl>,

    /// The token to include in another request to get the next page of items. This value is
    /// null when there are no more items to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeNetworkAclsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            network_acls: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a network ACL.
///
/// **AWS API**: `ec2.v1.NetworkAcl`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//NetworkAcl>
///
/// ## Coverage
/// 4 of 7 fields included.
/// Omitted fields:
/// - `Associations` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `OwnerId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkAcl {
    /// The ID of the network ACL.
    #[serde(rename(serialize = "NetworkAclId", deserialize = "networkAclId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_acl_id: Option<String>,

    /// The ID of the VPC for the network ACL.
    #[serde(rename(serialize = "VpcId", deserialize = "vpcId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// Indicates whether this is the default network ACL for the VPC.
    #[serde(rename(serialize = "Default", deserialize = "default"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,

    /// The entries (rules) in the network ACL.
    #[serde(rename(serialize = "EntrySet", deserialize = "entrySet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<NetworkAclEntry>,
}

impl NetworkAcl {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            network_acl_id: Some("test-network_acl_id".into()),
            vpc_id: Some("test-vpc_id".into()),
            is_default: Some(false),
            entries: vec![],
        }
    }
}

/// Describes an entry in a network ACL.
///
/// **AWS API**: `ec2.v1.NetworkAclEntry`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//NetworkAclEntry>
///
/// ## Coverage
/// 6 of 8 fields included.
/// Omitted fields:
/// - `IcmpTypeCode` — not selected in manifest
/// - `Ipv6CidrBlock` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkAclEntry {
    /// The rule number for the entry. ACL entries are processed in ascending order by rule
    /// number.
    #[serde(rename(serialize = "RuleNumber", deserialize = "ruleNumber"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_number: Option<i32>,

    /// The protocol number. A value of "-1" means all protocols.
    #[serde(rename(serialize = "Protocol", deserialize = "protocol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Indicates whether to allow or deny the traffic that matches the rule.
    #[serde(rename(serialize = "RuleAction", deserialize = "ruleAction"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_action: Option<String>,

    /// Indicates whether the rule is an egress rule (applied to traffic leaving the subnet).
    #[serde(rename(serialize = "Egress", deserialize = "egress"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress: Option<bool>,

    /// The IPv4 network range to allow or deny, in CIDR notation.
    #[serde(rename(serialize = "CidrBlock", deserialize = "cidrBlock"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,

    /// TCP or UDP protocols: The range of ports the rule applies to.
    #[serde(rename(serialize = "PortRange", deserialize = "portRange"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<PortRange>,
}

impl NetworkAclEntry {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            rule_number: Some(100),
            protocol: Some("test-protocol".into()),
            rule_action: Some("test-rule_action".into()),
            egress: Some(false),
            cidr_block: Some("test-cidr_block".into()),
            port_range: Some(PortRange::fixture()),
        }
    }
}

/// Describes a range of ports.
///
/// **AWS API**: `ec2.v1.PortRange`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//PortRange>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PortRange {
    /// The first port in the range.
    #[serde(rename(serialize = "From", deserialize = "from"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,

    /// The last port in the range.
    #[serde(rename(serialize = "To", deserialize = "to"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i32>,
}

impl PortRange {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            from: Some(100),
            to: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeFlowLogsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeFlowLogsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFlowLogsResponse {
    /// Information about the flow logs.
    #[serde(rename(serialize = "FlowLogSet", deserialize = "flowLogSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub flow_logs: Vec<FlowLog>,

    /// The token to request the next page of items. This value is null when there are no more
    /// items to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeFlowLogsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            flow_logs: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a flow log.
///
/// **AWS API**: `ec2.v1.FlowLog`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//FlowLog>
///
/// ## Coverage
/// 5 of 16 fields included.
/// Omitted fields:
/// - `CreationTime` — not selected in manifest
/// - `DeliverLogsErrorMessage` — not selected in manifest
/// - `DeliverLogsPermissionArn` — not selected in manifest
/// - `DeliverCrossAccountRole` — not selected in manifest
/// - `DeliverLogsStatus` — not selected in manifest
/// - `LogDestinationType` — not selected in manifest
/// - `LogDestination` — not selected in manifest
/// - `LogFormat` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `MaxAggregationInterval` — not selected in manifest
/// - `DestinationOptions` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FlowLog {
    /// The ID of the flow log.
    #[serde(rename(serialize = "FlowLogId", deserialize = "flowLogId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_log_id: Option<String>,

    /// The ID of the resource being monitored.
    #[serde(rename(serialize = "ResourceId", deserialize = "resourceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,

    /// The type of traffic captured for the flow log.
    #[serde(rename(serialize = "TrafficType", deserialize = "trafficType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_type: Option<String>,

    /// The name of the flow log group.
    #[serde(rename(serialize = "LogGroupName", deserialize = "logGroupName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,

    /// The status of the flow log (ACTIVE).
    #[serde(rename(serialize = "FlowLogStatus", deserialize = "flowLogStatus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_log_status: Option<String>,
}

impl FlowLog {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            flow_log_id: Some("test-flow_log_id".into()),
            resource_id: Some("test-resource_id".into()),
            traffic_type: Some("test-traffic_type".into()),
            log_group_name: Some("test-log_group_name".into()),
            flow_log_status: Some("test-flow_log_status".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeVpcsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeVpcsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVpcsResponse {
    /// Information about the VPCs.
    #[serde(rename(serialize = "VpcSet", deserialize = "vpcSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vpcs: Vec<Vpc>,

    /// The token to include in another request to get the next page of items. This value is
    /// null when there are no more items to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeVpcsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpcs: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a VPC.
///
/// **AWS API**: `ec2.v1.Vpc`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//Vpc>
///
/// ## Coverage
/// 4 of 12 fields included.
/// Omitted fields:
/// - `OwnerId` — not selected in manifest
/// - `InstanceTenancy` — not selected in manifest
/// - `Ipv6CidrBlockAssociationSet` — not selected in manifest
/// - `CidrBlockAssociationSet` — not selected in manifest
/// - `EncryptionControl` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `BlockPublicAccessStates` — not selected in manifest
/// - `DhcpOptionsId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vpc {
    /// The ID of the VPC.
    #[serde(rename(serialize = "VpcId", deserialize = "vpcId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// The primary IPv4 CIDR block for the VPC.
    #[serde(rename(serialize = "CidrBlock", deserialize = "cidrBlock"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,

    /// The current state of the VPC.
    #[serde(rename(serialize = "State", deserialize = "state"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Indicates whether the VPC is the default VPC.
    #[serde(rename(serialize = "IsDefault", deserialize = "isDefault"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl Vpc {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpc_id: Some("test-vpc_id".into()),
            cidr_block: Some("test-cidr_block".into()),
            state: Some("test-state".into()),
            is_default: Some(false),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeVpcEndpointsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeVpcEndpointsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVpcEndpointsResponse {
    /// Information about the VPC endpoints.
    #[serde(rename(serialize = "VpcEndpointSet", deserialize = "vpcEndpointSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vpc_endpoints: Vec<VpcEndpoint>,

    /// The token to use when requesting the next set of items. If there are no additional items
    /// to return, the string is empty.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeVpcEndpointsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpc_endpoints: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a VPC endpoint.
///
/// **AWS API**: `ec2.v1.VpcEndpoint`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//VpcEndpoint>
///
/// ## Coverage
/// 6 of 25 fields included.
/// Omitted fields:
/// - `VpcEndpointType` — not selected in manifest
/// - `PolicyDocument` — not selected in manifest
/// - `Groups` — not selected in manifest
/// - `IpAddressType` — not selected in manifest
/// - `DnsOptions` — not selected in manifest
/// - `PrivateDnsEnabled` — not selected in manifest
/// - `RequesterManaged` — not selected in manifest
/// - `NetworkInterfaceIds` — not selected in manifest
/// - `DnsEntries` — not selected in manifest
/// - `CreationTimestamp` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `OwnerId` — not selected in manifest
/// - `LastError` — not selected in manifest
/// - `Ipv4Prefixes` — not selected in manifest
/// - `Ipv6Prefixes` — not selected in manifest
/// - `FailureReason` — not selected in manifest
/// - `ServiceNetworkArn` — not selected in manifest
/// - `ResourceConfigurationArn` — not selected in manifest
/// - `ServiceRegion` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VpcEndpoint {
    /// The ID of the endpoint.
    #[serde(rename(serialize = "VpcEndpointId", deserialize = "vpcEndpointId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,

    /// The ID of the VPC to which the endpoint is associated.
    #[serde(rename(serialize = "VpcId", deserialize = "vpcId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// The name of the service to which the endpoint is associated.
    #[serde(rename(serialize = "ServiceName", deserialize = "serviceName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,

    /// The state of the endpoint.
    #[serde(rename(serialize = "State", deserialize = "state"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// (Gateway endpoint) The IDs of the route tables associated with the endpoint.
    #[serde(rename(serialize = "RouteTableIdSet", deserialize = "routeTableIdSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub route_table_ids: Vec<String>,

    /// (Interface endpoint) The subnets for the endpoint.
    #[serde(rename(serialize = "SubnetIdSet", deserialize = "subnetIdSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subnet_ids: Vec<String>,
}

impl VpcEndpoint {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpc_endpoint_id: Some("test-vpc_endpoint_id".into()),
            vpc_id: Some("test-vpc_id".into()),
            service_name: Some("test-service_name".into()),
            state: Some("test-state".into()),
            route_table_ids: vec![],
            subnet_ids: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeVpcPeeringConnectionsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeVpcPeeringConnectionsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVpcPeeringConnectionsResponse {
    /// Information about the VPC peering connections.
    #[serde(rename(
        serialize = "VpcPeeringConnectionSet",
        deserialize = "vpcPeeringConnectionSet"
    ))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vpc_peering_connections: Vec<VpcPeeringConnection>,

    /// The token to include in another request to get the next page of items. This value is
    /// null when there are no more items to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeVpcPeeringConnectionsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpc_peering_connections: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a VPC peering connection.
///
/// **AWS API**: `ec2.v1.VpcPeeringConnection`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//VpcPeeringConnection>
///
/// ## Coverage
/// 4 of 6 fields included.
/// Omitted fields:
/// - `ExpirationTime` — not selected in manifest
/// - `Tags` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VpcPeeringConnection {
    /// The ID of the VPC peering connection.
    #[serde(rename(
        serialize = "VpcPeeringConnectionId",
        deserialize = "vpcPeeringConnectionId"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connection_id: Option<String>,

    /// The status of the VPC peering connection.
    #[serde(rename(serialize = "Status", deserialize = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VpcPeeringConnectionStateReason>,

    /// Information about the accepter VPC. CIDR block information is only returned when
    /// describing an active VPC peering connection.
    #[serde(rename(serialize = "AccepterVpcInfo", deserialize = "accepterVpcInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepter_vpc_info: Option<VpcPeeringConnectionVpcInfo>,

    /// Information about the requester VPC. CIDR block information is only returned when
    /// describing an active VPC peering connection.
    #[serde(rename(serialize = "RequesterVpcInfo", deserialize = "requesterVpcInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_vpc_info: Option<VpcPeeringConnectionVpcInfo>,
}

impl VpcPeeringConnection {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpc_peering_connection_id: Some("test-vpc_peering_connection_id".into()),
            status: Some(VpcPeeringConnectionStateReason::fixture()),
            accepter_vpc_info: Some(VpcPeeringConnectionVpcInfo::fixture()),
            requester_vpc_info: Some(VpcPeeringConnectionVpcInfo::fixture()),
        }
    }
}

/// Describes the status of a VPC peering connection.
///
/// **AWS API**: `ec2.v1.VpcPeeringConnectionStateReason`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//VpcPeeringConnectionStateReason>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VpcPeeringConnectionStateReason {
    /// The status of the VPC peering connection.
    #[serde(rename(serialize = "Code", deserialize = "code"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// A message that provides more information about the status, if applicable.
    #[serde(rename(serialize = "Message", deserialize = "message"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl VpcPeeringConnectionStateReason {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            code: Some("test-code".into()),
            message: Some("test-message".into()),
        }
    }
}

/// Describes a VPC in a VPC peering connection.
///
/// **AWS API**: `ec2.v1.VpcPeeringConnectionVpcInfo`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//VpcPeeringConnectionVpcInfo>
///
/// ## Coverage
/// 4 of 7 fields included.
/// Omitted fields:
/// - `Ipv6CidrBlockSet` — not selected in manifest
/// - `CidrBlockSet` — not selected in manifest
/// - `PeeringOptions` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VpcPeeringConnectionVpcInfo {
    /// The ID of the VPC.
    #[serde(rename(serialize = "VpcId", deserialize = "vpcId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// The ID of the Amazon Web Services account that owns the VPC.
    #[serde(rename(serialize = "OwnerId", deserialize = "ownerId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,

    /// The IPv4 CIDR block for the VPC.
    #[serde(rename(serialize = "CidrBlock", deserialize = "cidrBlock"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,

    /// The Region in which the VPC is located.
    #[serde(rename(serialize = "Region", deserialize = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl VpcPeeringConnectionVpcInfo {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpc_id: Some("test-vpc_id".into()),
            owner_id: Some("test-owner_id".into()),
            cidr_block: Some("test-cidr_block".into()),
            region: Some("test-region".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeLaunchTemplatesResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeLaunchTemplatesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLaunchTemplatesResponse {
    /// Information about the launch templates.
    #[serde(rename(serialize = "LaunchTemplates", deserialize = "launchTemplates"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub launch_templates: Vec<LaunchTemplate>,

    /// The token to use to retrieve the next page of results. This value is null when there are
    /// no more results to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeLaunchTemplatesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_templates: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a launch template.
///
/// **AWS API**: `ec2.v1.LaunchTemplate`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//LaunchTemplate>
///
/// ## Coverage
/// 2 of 8 fields included.
/// Omitted fields:
/// - `CreateTime` — not selected in manifest
/// - `CreatedBy` — not selected in manifest
/// - `DefaultVersionNumber` — not selected in manifest
/// - `LatestVersionNumber` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `Operator` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaunchTemplate {
    /// The ID of the launch template.
    #[serde(rename(serialize = "LaunchTemplateId", deserialize = "launchTemplateId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,

    /// The name of the launch template.
    #[serde(rename(serialize = "LaunchTemplateName", deserialize = "launchTemplateName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,
}

impl LaunchTemplate {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_template_id: Some("test-launch_template_id".into()),
            launch_template_name: Some("test-launch_template_name".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeLaunchTemplateVersionsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeLaunchTemplateVersionsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLaunchTemplateVersionsResponse {
    /// Information about the launch template versions.
    #[serde(rename(
        serialize = "LaunchTemplateVersionSet",
        deserialize = "launchTemplateVersionSet"
    ))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub launch_template_versions: Vec<LaunchTemplateVersion>,

    /// The token to use to retrieve the next page of results. This value is null when there are
    /// no more results to return.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeLaunchTemplateVersionsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_template_versions: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Describes a launch template version.
///
/// **AWS API**: `ec2.v1.LaunchTemplateVersion`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//LaunchTemplateVersion>
///
/// ## Coverage
/// 4 of 9 fields included.
/// Omitted fields:
/// - `VersionDescription` — not selected in manifest
/// - `CreateTime` — not selected in manifest
/// - `CreatedBy` — not selected in manifest
/// - `DefaultVersion` — not selected in manifest
/// - `Operator` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaunchTemplateVersion {
    /// The ID of the launch template.
    #[serde(rename(serialize = "LaunchTemplateId", deserialize = "launchTemplateId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,

    /// The name of the launch template.
    #[serde(rename(serialize = "LaunchTemplateName", deserialize = "launchTemplateName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,

    /// The version number.
    #[serde(rename(serialize = "VersionNumber", deserialize = "versionNumber"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,

    /// Information about the launch template.
    #[serde(rename(serialize = "LaunchTemplateData", deserialize = "launchTemplateData"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_data: Option<ResponseLaunchTemplateData>,
}

impl LaunchTemplateVersion {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_template_id: Some("test-launch_template_id".into()),
            launch_template_name: Some("test-launch_template_name".into()),
            version_number: Some(100),
            launch_template_data: Some(ResponseLaunchTemplateData::fixture()),
        }
    }
}

/// The information for a launch template.
///
/// **AWS API**: `ec2.v1.ResponseLaunchTemplateData`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//ResponseLaunchTemplateData>
///
/// ## Coverage
/// 4 of 34 fields included.
/// Omitted fields:
/// - `KernelId` — not selected in manifest
/// - `EbsOptimized` — not selected in manifest
/// - `IamInstanceProfile` — not selected in manifest
/// - `BlockDeviceMappings` — not selected in manifest
/// - `KeyName` — not selected in manifest
/// - `Monitoring` — not selected in manifest
/// - `Placement` — not selected in manifest
/// - `RamDiskId` — not selected in manifest
/// - `DisableApiTermination` — not selected in manifest
/// - `InstanceInitiatedShutdownBehavior` — not selected in manifest
/// - `UserData` — not selected in manifest
/// - `TagSpecifications` — not selected in manifest
/// - `ElasticGpuSpecifications` — not selected in manifest
/// - `ElasticInferenceAccelerators` — not selected in manifest
/// - `SecurityGroupIds` — not selected in manifest
/// - `SecurityGroups` — not selected in manifest
/// - `InstanceMarketOptions` — not selected in manifest
/// - `CreditSpecification` — not selected in manifest
/// - `CpuOptions` — not selected in manifest
/// - `CapacityReservationSpecification` — not selected in manifest
/// - `LicenseSpecifications` — not selected in manifest
/// - `HibernationOptions` — not selected in manifest
/// - `EnclaveOptions` — not selected in manifest
/// - `InstanceRequirements` — not selected in manifest
/// - `PrivateDnsNameOptions` — not selected in manifest
/// - `MaintenanceOptions` — not selected in manifest
/// - `DisableApiStop` — not selected in manifest
/// - `Operator` — not selected in manifest
/// - `NetworkPerformanceOptions` — not selected in manifest
/// - `SecondaryInterfaces` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseLaunchTemplateData {
    /// The ID of the AMI or a Systems Manager parameter. The Systems Manager parameter will
    /// resolve to the ID of the AMI at instance launch. The value depends on what you specified
    /// in the request. The possible values are: If an AMI ID was specified in the request, then
    /// this is the AMI ID. If a Systems Manager parameter was specified in the request, and
    /// ResolveAlias was configured as true, then this is the AMI ID that the parameter is
    /// mapped to in the Parameter Store. If a Systems Manager parameter was specified in the
    /// request, and ResolveAlias was configured as false, then this is the parameter value. For
    /// more information, see Use a Systems Manager parameter instead of an AMI ID in the Amazon
    /// EC2 User Guide.
    #[serde(rename(serialize = "ImageId", deserialize = "imageId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,

    /// The instance type.
    #[serde(rename(serialize = "InstanceType", deserialize = "instanceType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,

    /// The metadata options for the instance. For more information, see Configure the Instance
    /// Metadata Service options in the Amazon EC2 User Guide.
    #[serde(rename(serialize = "MetadataOptions", deserialize = "metadataOptions"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_options: Option<LaunchTemplateInstanceMetadataOptions>,

    /// The network interfaces.
    #[serde(rename(serialize = "NetworkInterfaceSet", deserialize = "networkInterfaceSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<LaunchTemplateInstanceNetworkInterfaceSpecification>,
}

impl ResponseLaunchTemplateData {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_id: Some("test-image_id".into()),
            instance_type: Some("test-instance_type".into()),
            metadata_options: Some(LaunchTemplateInstanceMetadataOptions::fixture()),
            network_interfaces: vec![],
        }
    }
}

/// The metadata options for the instance. For more information, see Use instance metadata to
/// manage your EC2 instance in the Amazon EC2 User Guide.
///
/// **AWS API**: `ec2.v1.LaunchTemplateInstanceMetadataOptions`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//LaunchTemplateInstanceMetadataOptions>
///
/// ## Coverage
/// 1 of 6 fields included.
/// Omitted fields:
/// - `State` — not selected in manifest
/// - `HttpPutResponseHopLimit` — not selected in manifest
/// - `HttpEndpoint` — not selected in manifest
/// - `HttpProtocolIpv6` — not selected in manifest
/// - `InstanceMetadataTags` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaunchTemplateInstanceMetadataOptions {
    /// Indicates whether IMDSv2 is required. optional
    /// - IMDSv2 is optional. You can choose whether to send a session token in your instance
    ///   metadata retrieval requests. If you retrieve IAM role credentials without a session
    ///   token, you receive the IMDSv1 role credentials. If you retrieve IAM role credentials
    ///   using a valid session token, you receive the IMDSv2 role credentials. required
    /// - IMDSv2 is required. You must send a session token in your instance metadata retrieval
    ///   requests. With this option, retrieving the IAM role credentials always returns IMDSv2
    ///   credentials; IMDSv1 credentials are not available.
    #[serde(rename(serialize = "HttpTokens", deserialize = "httpTokens"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_tokens: Option<String>,
}

impl LaunchTemplateInstanceMetadataOptions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            http_tokens: Some("test-http_tokens".into()),
        }
    }
}

/// Describes a network interface.
///
/// **AWS API**: `ec2.v1.LaunchTemplateInstanceNetworkInterfaceSpecification`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//LaunchTemplateInstanceNetworkInterfaceSpecification>
///
/// ## Coverage
/// 1 of 23 fields included.
/// Omitted fields:
/// - `AssociateCarrierIpAddress` — not selected in manifest
/// - `DeleteOnTermination` — not selected in manifest
/// - `Description` — not selected in manifest
/// - `DeviceIndex` — not selected in manifest
/// - `Groups` — not selected in manifest
/// - `InterfaceType` — not selected in manifest
/// - `Ipv6AddressCount` — not selected in manifest
/// - `Ipv6Addresses` — not selected in manifest
/// - `NetworkInterfaceId` — not selected in manifest
/// - `PrivateIpAddress` — not selected in manifest
/// - `PrivateIpAddresses` — not selected in manifest
/// - `SecondaryPrivateIpAddressCount` — not selected in manifest
/// - `SubnetId` — not selected in manifest
/// - `NetworkCardIndex` — not selected in manifest
/// - `Ipv4Prefixes` — not selected in manifest
/// - `Ipv4PrefixCount` — not selected in manifest
/// - `Ipv6Prefixes` — not selected in manifest
/// - `Ipv6PrefixCount` — not selected in manifest
/// - `PrimaryIpv6` — not selected in manifest
/// - `EnaSrdSpecification` — not selected in manifest
/// - `ConnectionTrackingSpecification` — not selected in manifest
/// - `EnaQueueCount` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaunchTemplateInstanceNetworkInterfaceSpecification {
    /// Indicates whether to associate a public IPv4 address with eth0 for a new network
    /// interface. Amazon Web Services charges for all public IPv4 addresses, including public
    /// IPv4 addresses associated with running instances and Elastic IP addresses. For more
    /// information, see the Public IPv4 Address tab on the Amazon VPC pricing page.
    #[serde(rename(
        serialize = "AssociatePublicIpAddress",
        deserialize = "associatePublicIpAddress"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
}

impl LaunchTemplateInstanceNetworkInterfaceSpecification {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            associate_public_ip_address: Some(false),
        }
    }
}

///
/// **AWS API**: `ec2.v1.GetEbsEncryptionByDefaultResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//GetEbsEncryptionByDefaultResult>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `SseType` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetEbsEncryptionByDefaultResponse {
    /// Indicates whether encryption by default is enabled.
    #[serde(rename(
        serialize = "EbsEncryptionByDefault",
        deserialize = "ebsEncryptionByDefault"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_encryption_by_default: Option<bool>,
}

impl GetEbsEncryptionByDefaultResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ebs_encryption_by_default: Some(false),
        }
    }
}

///
/// **AWS API**: `ec2.v1.EnableEbsEncryptionByDefaultResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//EnableEbsEncryptionByDefaultResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnableEbsEncryptionByDefaultResponse {
    /// The updated status of encryption by default.
    #[serde(rename(
        serialize = "EbsEncryptionByDefault",
        deserialize = "ebsEncryptionByDefault"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_encryption_by_default: Option<bool>,
}

impl EnableEbsEncryptionByDefaultResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ebs_encryption_by_default: Some(false),
        }
    }
}

///
/// **AWS API**: `ec2.v1.EnableSnapshotBlockPublicAccessResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//EnableSnapshotBlockPublicAccessResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnableSnapshotBlockPublicAccessResponse {
    /// The state of block public access for snapshots for the account and Region. Returns
    /// either block-all-sharing or block-new-sharing if the request succeeds.
    #[serde(rename(serialize = "State", deserialize = "state"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl EnableSnapshotBlockPublicAccessResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            state: Some("test-state".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.EnableImageBlockPublicAccessResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//EnableImageBlockPublicAccessResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnableImageBlockPublicAccessResponse {
    /// Returns block-new-sharing if the request succeeds; otherwise, it returns an error.
    #[serde(rename(
        serialize = "ImageBlockPublicAccessState",
        deserialize = "imageBlockPublicAccessState"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_block_public_access_state: Option<String>,
}

impl EnableImageBlockPublicAccessResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_block_public_access_state: Some("test-image_block_public_access_state".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.TerminateInstancesResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//TerminateInstancesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminateInstancesResponse {
    /// Information about the terminated instances.
    #[serde(rename(serialize = "InstancesSet", deserialize = "instancesSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub terminating_instances: Vec<InstanceStateChange>,
}

impl TerminateInstancesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            terminating_instances: vec![],
        }
    }
}

/// Describes an instance state change.
///
/// **AWS API**: `ec2.v1.InstanceStateChange`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//InstanceStateChange>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceStateChange {
    /// The ID of the instance.
    #[serde(rename(serialize = "InstanceId", deserialize = "instanceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// The current state of the instance.
    #[serde(rename(serialize = "CurrentState", deserialize = "currentState"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<InstanceState>,

    /// The previous state of the instance.
    #[serde(rename(serialize = "PreviousState", deserialize = "previousState"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<InstanceState>,
}

impl InstanceStateChange {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_id: Some("test-instance_id".into()),
            current_state: Some(InstanceState::fixture()),
            previous_state: Some(InstanceState::fixture()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.StopInstancesResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//StopInstancesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StopInstancesResponse {
    /// Information about the stopped instances.
    #[serde(rename(serialize = "InstancesSet", deserialize = "instancesSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stopping_instances: Vec<InstanceStateChange>,
}

impl StopInstancesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stopping_instances: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.StartInstancesResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//StartInstancesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartInstancesResponse {
    /// Information about the started instances.
    #[serde(rename(serialize = "InstancesSet", deserialize = "instancesSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub starting_instances: Vec<InstanceStateChange>,
}

impl StartInstancesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            starting_instances: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.ModifyInstanceMetadataOptionsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//ModifyInstanceMetadataOptionsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyInstanceMetadataOptionsResponse {
    /// The ID of the instance.
    #[serde(rename(serialize = "InstanceId", deserialize = "instanceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// The metadata options for the instance.
    #[serde(rename(
        serialize = "InstanceMetadataOptions",
        deserialize = "instanceMetadataOptions"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_options: Option<InstanceMetadataOptionsResponse>,
}

impl ModifyInstanceMetadataOptionsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_id: Some("test-instance_id".into()),
            instance_metadata_options: Some(InstanceMetadataOptionsResponse::fixture()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.MonitorInstancesResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//MonitorInstancesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MonitorInstancesResponse {
    /// The monitoring information.
    #[serde(rename(serialize = "InstancesSet", deserialize = "instancesSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instance_monitorings: Vec<InstanceMonitoring>,
}

impl MonitorInstancesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_monitorings: vec![],
        }
    }
}

/// Describes the monitoring of an instance.
///
/// **AWS API**: `ec2.v1.InstanceMonitoring`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//InstanceMonitoring>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceMonitoring {
    /// The ID of the instance.
    #[serde(rename(serialize = "InstanceId", deserialize = "instanceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// The monitoring for the instance.
    #[serde(rename(serialize = "Monitoring", deserialize = "monitoring"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<Monitoring>,
}

impl InstanceMonitoring {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_id: Some("test-instance_id".into()),
            monitoring: Some(Monitoring::fixture()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.AssociateIamInstanceProfileResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//AssociateIamInstanceProfileResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssociateIamInstanceProfileResponse {
    /// Information about the IAM instance profile association.
    #[serde(rename(
        serialize = "IamInstanceProfileAssociation",
        deserialize = "iamInstanceProfileAssociation"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile_association: Option<IamInstanceProfileAssociation>,
}

impl AssociateIamInstanceProfileResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            iam_instance_profile_association: Some(IamInstanceProfileAssociation::fixture()),
        }
    }
}

/// Describes an association between an IAM instance profile and an instance.
///
/// **AWS API**: `ec2.v1.IamInstanceProfileAssociation`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//IamInstanceProfileAssociation>
///
/// ## Coverage
/// 4 of 5 fields included.
/// Omitted fields:
/// - `Timestamp` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IamInstanceProfileAssociation {
    /// The ID of the association.
    #[serde(rename(serialize = "AssociationId", deserialize = "associationId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,

    /// The ID of the instance.
    #[serde(rename(serialize = "InstanceId", deserialize = "instanceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// The IAM instance profile.
    #[serde(rename(serialize = "IamInstanceProfile", deserialize = "iamInstanceProfile"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<IamInstanceProfile>,

    /// The state of the association.
    #[serde(rename(serialize = "State", deserialize = "state"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl IamInstanceProfileAssociation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            association_id: Some("test-association_id".into()),
            instance_id: Some("test-instance_id".into()),
            iam_instance_profile: Some(IamInstanceProfile::fixture()),
            state: Some("test-state".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.ModifyVolumeResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//ModifyVolumeResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyVolumeResponse {
    /// Information about the volume modification.
    #[serde(rename(serialize = "VolumeModification", deserialize = "volumeModification"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_modification: Option<VolumeModification>,
}

impl ModifyVolumeResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            volume_modification: Some(VolumeModification::fixture()),
        }
    }
}

/// Describes the modification status of an EBS volume.
///
/// **AWS API**: `ec2.v1.VolumeModification`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//VolumeModification>
///
/// ## Coverage
/// 5 of 16 fields included.
/// Omitted fields:
/// - `StatusMessage` — not selected in manifest
/// - `TargetSize` — not selected in manifest
/// - `TargetMultiAttachEnabled` — not selected in manifest
/// - `OriginalSize` — not selected in manifest
/// - `OriginalIops` — not selected in manifest
/// - `OriginalVolumeType` — not selected in manifest
/// - `OriginalThroughput` — not selected in manifest
/// - `OriginalMultiAttachEnabled` — not selected in manifest
/// - `Progress` — not selected in manifest
/// - `StartTime` — not selected in manifest
/// - `EndTime` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VolumeModification {
    /// The ID of the volume.
    #[serde(rename(serialize = "VolumeId", deserialize = "volumeId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,

    /// The current modification state.
    #[serde(rename(serialize = "ModificationState", deserialize = "modificationState"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_state: Option<String>,

    /// The target EBS volume type of the volume.
    #[serde(rename(serialize = "TargetVolumeType", deserialize = "targetVolumeType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_volume_type: Option<String>,

    /// The target IOPS rate of the volume.
    #[serde(rename(serialize = "TargetIops", deserialize = "targetIops"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_iops: Option<i32>,

    /// The target throughput of the volume, in MiB/s.
    #[serde(rename(serialize = "TargetThroughput", deserialize = "targetThroughput"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_throughput: Option<i32>,
}

impl VolumeModification {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            volume_id: Some("test-volume_id".into()),
            modification_state: Some("test-modification_state".into()),
            target_volume_type: Some("test-target_volume_type".into()),
            target_iops: Some(100),
            target_throughput: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DeleteNatGatewayResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DeleteNatGatewayResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteNatGatewayResponse {
    /// The ID of the NAT gateway.
    #[serde(rename(serialize = "NatGatewayId", deserialize = "natGatewayId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_gateway_id: Option<String>,
}

impl DeleteNatGatewayResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            nat_gateway_id: Some("test-nat_gateway_id".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DeleteVpcEndpointsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DeleteVpcEndpointsResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteVpcEndpointsResponse {
    /// Information about the VPC endpoints that were not successfully deleted.
    #[serde(rename(serialize = "Unsuccessful", deserialize = "unsuccessful"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unsuccessful: Vec<UnsuccessfulItem>,
}

impl DeleteVpcEndpointsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            unsuccessful: vec![],
        }
    }
}

/// Information about items that were not successfully processed in a batch call.
///
/// **AWS API**: `ec2.v1.UnsuccessfulItem`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//UnsuccessfulItem>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnsuccessfulItem {
    /// The ID of the resource.
    #[serde(rename(serialize = "ResourceId", deserialize = "resourceId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,

    /// Information about the error.
    #[serde(rename(serialize = "Error", deserialize = "error"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<UnsuccessfulItemError>,
}

impl UnsuccessfulItem {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource_id: Some("test-resource_id".into()),
            error: Some(UnsuccessfulItemError::fixture()),
        }
    }
}

/// Information about the error that occurred. For more information about errors, see Error
/// codes.
///
/// **AWS API**: `ec2.v1.UnsuccessfulItemError`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//UnsuccessfulItemError>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnsuccessfulItemError {
    /// The error code.
    #[serde(rename(serialize = "Code", deserialize = "code"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// The error message accompanying the error code.
    #[serde(rename(serialize = "Message", deserialize = "message"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl UnsuccessfulItemError {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            code: Some("test-code".into()),
            message: Some("test-message".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.CreateFlowLogsResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//CreateFlowLogsResult>
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `ClientToken` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateFlowLogsResponse {
    /// The IDs of the flow logs.
    #[serde(rename(serialize = "FlowLogIdSet", deserialize = "flowLogIdSet"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub flow_log_ids: Vec<String>,

    /// Information about the flow logs that could not be created successfully.
    #[serde(rename(serialize = "Unsuccessful", deserialize = "unsuccessful"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unsuccessful: Vec<UnsuccessfulItem>,
}

impl CreateFlowLogsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            flow_log_ids: vec![],
            unsuccessful: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.RevokeSecurityGroupIngressResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//RevokeSecurityGroupIngressResult>
///
/// ## Coverage
/// 1 of 3 fields included.
/// Omitted fields:
/// - `Return` — not selected in manifest
/// - `RevokedSecurityGroupRules` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevokeSecurityGroupIngressResponse {
    /// The inbound rules that were unknown to the service. In some cases,
    /// unknownIpPermissionSet might be in a different format from the request parameter.
    #[serde(rename(
        serialize = "UnknownIpPermissionSet",
        deserialize = "unknownIpPermissionSet"
    ))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unknown_ip_permissions: Vec<IpPermission>,
}

impl RevokeSecurityGroupIngressResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            unknown_ip_permissions: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.RevokeSecurityGroupEgressResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//RevokeSecurityGroupEgressResult>
///
/// ## Coverage
/// 1 of 3 fields included.
/// Omitted fields:
/// - `Return` — not selected in manifest
/// - `RevokedSecurityGroupRules` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevokeSecurityGroupEgressResponse {
    /// The outbound rules that were unknown to the service. In some cases,
    /// unknownIpPermissionSet might be in a different format from the request parameter.
    #[serde(rename(
        serialize = "UnknownIpPermissionSet",
        deserialize = "unknownIpPermissionSet"
    ))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unknown_ip_permissions: Vec<IpPermission>,
}

impl RevokeSecurityGroupEgressResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            unknown_ip_permissions: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.AuthorizeSecurityGroupIngressResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//AuthorizeSecurityGroupIngressResult>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `Return` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizeSecurityGroupIngressResponse {
    /// Information about the inbound (ingress) security group rules that were added.
    #[serde(rename(
        serialize = "SecurityGroupRuleSet",
        deserialize = "securityGroupRuleSet"
    ))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_group_rules: Vec<SecurityGroupRule>,
}

impl AuthorizeSecurityGroupIngressResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            security_group_rules: vec![],
        }
    }
}

/// Describes a security group rule.
///
/// **AWS API**: `ec2.v1.SecurityGroupRule`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//SecurityGroupRule>
///
/// ## Coverage
/// 6 of 14 fields included.
/// Omitted fields:
/// - `GroupOwnerId` — not selected in manifest
/// - `IsEgress` — not selected in manifest
/// - `CidrIpv6` — not selected in manifest
/// - `PrefixListId` — not selected in manifest
/// - `ReferencedGroupInfo` — not selected in manifest
/// - `Description` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `SecurityGroupRuleArn` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityGroupRule {
    /// The ID of the security group rule.
    #[serde(rename(serialize = "SecurityGroupRuleId", deserialize = "securityGroupRuleId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_rule_id: Option<String>,

    /// The ID of the security group.
    #[serde(rename(serialize = "GroupId", deserialize = "groupId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    /// The IP protocol name (tcp, udp, icmp, icmpv6) or number (see Protocol Numbers). Use -1
    /// to specify all protocols.
    #[serde(rename(serialize = "IpProtocol", deserialize = "ipProtocol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<String>,

    /// If the protocol is TCP or UDP, this is the start of the port range. If the protocol is
    /// ICMP or ICMPv6, this is the ICMP type or -1 (all ICMP types).
    #[serde(rename(serialize = "FromPort", deserialize = "fromPort"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i32>,

    /// If the protocol is TCP or UDP, this is the end of the port range. If the protocol is
    /// ICMP or ICMPv6, this is the ICMP code or -1 (all ICMP codes). If the start port is -1
    /// (all ICMP types), then the end port must be -1 (all ICMP codes).
    #[serde(rename(serialize = "ToPort", deserialize = "toPort"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i32>,

    /// The IPv4 CIDR range.
    #[serde(rename(serialize = "CidrIpv4", deserialize = "cidrIpv4"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv4: Option<String>,
}

impl SecurityGroupRule {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            security_group_rule_id: Some("test-security_group_rule_id".into()),
            group_id: Some("test-group_id".into()),
            ip_protocol: Some("test-ip_protocol".into()),
            from_port: Some(100),
            to_port: Some(100),
            cidr_ipv4: Some("test-cidr_ipv4".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DeleteSecurityGroupResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DeleteSecurityGroupResult>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `Return` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteSecurityGroupResponse {
    /// The ID of the deleted security group.
    #[serde(rename(serialize = "GroupId", deserialize = "groupId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl DeleteSecurityGroupResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_id: Some("test-group_id".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DeregisterImageResult`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DeregisterImageResult>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `Return` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeregisterImageResponse {
    /// The deletion result for each snapshot associated with the AMI, including the snapshot ID
    /// and its success or error code.
    #[serde(rename(
        serialize = "DeleteSnapshotResultSet",
        deserialize = "deleteSnapshotResultSet"
    ))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub delete_snapshot_results: Vec<DeleteSnapshotReturnCode>,
}

impl DeregisterImageResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            delete_snapshot_results: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeInstancesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeInstancesRequest>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstancesRequest {
    /// The instance IDs. Default: Describes all your instances.
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instance_ids: Vec<String>,

    /// The token returned from a previous paginated request. Pagination continues from the end
    /// of the items returned by the previous request.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. To get the next page of items,
    /// make another request with the token returned in the output. For more information, see
    /// Pagination. You cannot specify this parameter and the instance IDs parameter in the same
    /// request.
    #[serde(rename(serialize = "MaxResults", deserialize = "maxResults"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeInstancesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeVolumesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeVolumesRequest>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVolumesRequest {
    /// The volume IDs. If not specified, then all volumes are included in the response.
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub volume_ids: Vec<String>,

    /// The token returned from a previous paginated request. Pagination continues from the end
    /// of the items returned by the previous request.
    #[serde(rename(serialize = "NextToken", deserialize = "nextToken"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. To get the next page of items,
    /// make another request with the token returned in the output. For more information, see
    /// Pagination.
    #[serde(rename(serialize = "MaxResults", deserialize = "maxResults"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeVolumesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            volume_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeSnapshotsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeSnapshotsRequest>
///
/// ## Coverage
/// 4 of 7 fields included.
/// Omitted fields:
/// - `RestorableByUserIds` — not selected in manifest
/// - `DryRun` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSnapshotsRequest {
    /// Scopes the results to snapshots with the specified owners. You can specify a combination
    /// of Amazon Web Services account IDs, self, and amazon.
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub owner_ids: Vec<String>,

    /// The snapshot IDs. Default: Describes the snapshots for which you have create volume
    /// permissions.
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub snapshot_ids: Vec<String>,

    /// The token returned from a previous paginated request. Pagination continues from the end
    /// of the items returned by the previous request.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. To get the next page of items,
    /// make another request with the token returned in the output. For more information, see
    /// Pagination.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeSnapshotsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            owner_ids: vec![],
            snapshot_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeImagesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeImagesRequest>
///
/// ## Coverage
/// 2 of 9 fields included.
/// Omitted fields:
/// - `ExecutableUsers` — not selected in manifest
/// - `IncludeDeprecated` — not selected in manifest
/// - `IncludeDisabled` — not selected in manifest
/// - `MaxResults` — not selected in manifest
/// - `NextToken` — not selected in manifest
/// - `DryRun` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeImagesRequest {
    /// Scopes the results to images with the specified owners. You can specify a combination of
    /// Amazon Web Services account IDs, self, amazon, aws-backup-vault, and aws-marketplace. If
    /// you omit this parameter, the results include all images for which you have launch
    /// permissions, regardless of ownership.
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<String>,

    /// The image IDs. Default: Describes all images available to you.
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image_ids: Vec<String>,
}

impl DescribeImagesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            owners: vec![],
            image_ids: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeSecurityGroupsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeSecurityGroupsRequest>
///
/// ## Coverage
/// 3 of 6 fields included.
/// Omitted fields:
/// - `GroupNames` — not selected in manifest
/// - `DryRun` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecurityGroupsRequest {
    /// The IDs of the security groups. Required for security groups in a nondefault VPC.
    /// Default: Describes all of your security groups.
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,

    /// The token returned from a previous paginated request. Pagination continues from the end
    /// of the items returned by the previous request.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. To get the next page of items,
    /// make another request with the token returned in the output. This value can be between 5
    /// and 1000. If this parameter is not specified, then all items are returned. For more
    /// information, see Pagination.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeSecurityGroupsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeAddressesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeAddressesRequest>
///
/// ## Coverage
/// 2 of 4 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAddressesRequest {
    /// Information about the allocation IDs.
    #[serde(rename = "AllocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allocation_ids: Vec<String>,

    /// One or more Elastic IP addresses. Default: Describes all your Elastic IP addresses.
    #[serde(rename = "PublicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub public_ips: Vec<String>,
}

impl DescribeAddressesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allocation_ids: vec![],
            public_ips: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeNatGatewaysRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeNatGatewaysRequest>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `Filter` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNatGatewaysRequest {
    /// The IDs of the NAT gateways.
    #[serde(rename = "NatGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nat_gateway_ids: Vec<String>,

    /// The token returned from a previous paginated request. Pagination continues from the end
    /// of the items returned by the previous request.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. To get the next page of items,
    /// make another request with the token returned in the output. For more information, see
    /// Pagination.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeNatGatewaysRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            nat_gateway_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeRouteTablesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeRouteTablesRequest>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRouteTablesRequest {
    /// The IDs of the route tables.
    #[serde(rename = "RouteTableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub route_table_ids: Vec<String>,

    /// The token returned from a previous paginated request. Pagination continues from the end
    /// of the items returned by the previous request.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. To get the next page of items,
    /// make another request with the token returned in the output. For more information, see
    /// Pagination.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeRouteTablesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            route_table_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeNetworkAclsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeNetworkAclsRequest>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkAclsRequest {
    /// The IDs of the network ACLs.
    #[serde(rename = "NetworkAclId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub network_acl_ids: Vec<String>,

    /// The token returned from a previous paginated request. Pagination continues from the end
    /// of the items returned by the previous request.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. To get the next page of items,
    /// make another request with the token returned in the output. For more information, see
    /// Pagination.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeNetworkAclsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            network_acl_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeFlowLogsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeFlowLogsRequest>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `Filter` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFlowLogsRequest {
    /// One or more flow log IDs. Constraint: Maximum of 1000 flow log IDs.
    #[serde(rename = "FlowLogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub flow_log_ids: Vec<String>,

    /// The token to request the next page of items. Pagination continues from the end of the
    /// items returned by the previous request.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. To get the next page of items,
    /// make another request with the token returned in the output. For more information, see
    /// Pagination.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeFlowLogsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            flow_log_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeVpcsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeVpcsRequest>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `Filters` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVpcsRequest {
    /// The IDs of the VPCs.
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vpc_ids: Vec<String>,

    /// The token returned from a previous paginated request. Pagination continues from the end
    /// of the items returned by the previous request.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. To get the next page of items,
    /// make another request with the token returned in the output. For more information, see
    /// Pagination.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeVpcsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpc_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeVpcEndpointsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeVpcEndpointsRequest>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVpcEndpointsRequest {
    /// The IDs of the VPC endpoints.
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vpc_endpoint_ids: Vec<String>,

    /// The token for the next set of items to return. (You received this token from a prior
    /// call.)
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. The request returns a token that
    /// you can specify in a subsequent call to get the next set of results. Constraint: If the
    /// value is greater than 1,000, we return only 1,000 items.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeVpcEndpointsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpc_endpoint_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeVpcPeeringConnectionsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeVpcPeeringConnectionsRequest>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVpcPeeringConnectionsRequest {
    /// The IDs of the VPC peering connections. Default: Describes all your VPC peering
    /// connections.
    #[serde(rename = "VpcPeeringConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vpc_peering_connection_ids: Vec<String>,

    /// The token returned from a previous paginated request. Pagination continues from the end
    /// of the items returned by the previous request.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return for this request. To get the next page of items,
    /// make another request with the token returned in the output. For more information, see
    /// Pagination.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeVpcPeeringConnectionsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpc_peering_connection_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeLaunchTemplatesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeLaunchTemplatesRequest>
///
/// ## Coverage
/// 3 of 6 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `LaunchTemplateNames` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLaunchTemplatesRequest {
    /// One or more launch template IDs.
    #[serde(rename = "LaunchTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub launch_template_ids: Vec<String>,

    /// The token to request the next page of results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of results to return in a single call. To retrieve the remaining
    /// results, make another call with the returned NextToken value. This value can be between
    /// 1 and 200.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeLaunchTemplatesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_template_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeLaunchTemplateVersionsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeLaunchTemplateVersionsRequest>
///
/// ## Coverage
/// 4 of 10 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `LaunchTemplateName` — not selected in manifest
/// - `MinVersion` — not selected in manifest
/// - `MaxVersion` — not selected in manifest
/// - `Filters` — not selected in manifest
/// - `ResolveAlias` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLaunchTemplateVersionsRequest {
    /// The ID of the launch template. To describe one or more versions of a specified launch
    /// template, you must specify either the launch template ID or the launch template name,
    /// but not both. To describe all the latest or default launch template versions in your
    /// account, you must omit this parameter.
    #[serde(rename = "LaunchTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,

    /// One or more versions of the launch template. Valid values depend on whether you are
    /// describing a specified launch template (by ID or name) or all launch templates in your
    /// account. To describe one or more versions of a specified launch template, valid values
    /// are $Latest, $Default, and numbers. To describe all launch templates in your account
    /// that are defined as the latest version, the valid value is $Latest. To describe all
    /// launch templates in your account that are defined as the default version, the valid
    /// value is $Default. You can specify $Latest and $Default in the same request. You cannot
    /// specify numbers.
    #[serde(rename = "LaunchTemplateVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<String>,

    /// The token to request the next page of results.
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of results to return in a single call. To retrieve the remaining
    /// results, make another call with the returned NextToken value. This value can be between
    /// 1 and 200.
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeLaunchTemplateVersionsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_template_id: Some("test-launch_template_id".into()),
            versions: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DescribeSnapshotAttributeRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DescribeSnapshotAttributeRequest>
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSnapshotAttributeRequest {
    /// The ID of the EBS snapshot.
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,

    /// The snapshot attribute you would like to view.
    #[serde(rename = "Attribute")]
    pub attribute: String,
}

impl DescribeSnapshotAttributeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshot_id: "test-snapshot_id".into(),
            attribute: "test-attribute".into(),
        }
    }
}

///
/// **AWS API**: `ec2.v1.GetEbsEncryptionByDefaultRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//GetEbsEncryptionByDefaultRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetEbsEncryptionByDefaultRequest {
    /// Checks whether you have the required permissions for the action, without actually making
    /// the request, and provides an error response. If you have the required permissions, the
    /// error response is DryRunOperation. Otherwise, it is UnauthorizedOperation.
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl GetEbsEncryptionByDefaultRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            dry_run: Some(false),
        }
    }
}

///
/// **AWS API**: `ec2.v1.TerminateInstancesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//TerminateInstancesRequest>
///
/// ## Coverage
/// 1 of 4 fields included.
/// Omitted fields:
/// - `Force` — not selected in manifest
/// - `SkipOsShutdown` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminateInstancesRequest {
    /// The IDs of the instances. Constraints: Up to 1000 instance IDs. We recommend breaking up
    /// this request into smaller batches.
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_ids: Vec<String>,
}

impl TerminateInstancesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_ids: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.StopInstancesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//StopInstancesRequest>
///
/// ## Coverage
/// 1 of 5 fields included.
/// Omitted fields:
/// - `Hibernate` — not selected in manifest
/// - `SkipOsShutdown` — not selected in manifest
/// - `DryRun` — not selected in manifest
/// - `Force` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StopInstancesRequest {
    /// The IDs of the instances.
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_ids: Vec<String>,
}

impl StopInstancesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_ids: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.StartInstancesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//StartInstancesRequest>
///
/// ## Coverage
/// 1 of 3 fields included.
/// Omitted fields:
/// - `AdditionalInfo` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartInstancesRequest {
    /// The IDs of the instances.
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_ids: Vec<String>,
}

impl StartInstancesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_ids: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.ModifyInstanceAttributeRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//ModifyInstanceAttributeRequest>
///
/// ## Coverage
/// 1 of 17 fields included.
/// Omitted fields:
/// - `SourceDestCheck` — not selected in manifest
/// - `DisableApiStop` — not selected in manifest
/// - `DryRun` — not selected in manifest
/// - `Attribute` — not selected in manifest
/// - `Value` — not selected in manifest
/// - `BlockDeviceMappings` — not selected in manifest
/// - `DisableApiTermination` — not selected in manifest
/// - `InstanceType` — not selected in manifest
/// - `Kernel` — not selected in manifest
/// - `Ramdisk` — not selected in manifest
/// - `UserData` — not selected in manifest
/// - `InstanceInitiatedShutdownBehavior` — not selected in manifest
/// - `Groups` — not selected in manifest
/// - `EbsOptimized` — not selected in manifest
/// - `SriovNetSupport` — not selected in manifest
/// - `EnaSupport` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyInstanceAttributeRequest {
    /// The ID of the instance.
    #[serde(rename(serialize = "InstanceId", deserialize = "instanceId"))]
    pub instance_id: String,
}

impl ModifyInstanceAttributeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_id: "test-instance_id".into(),
        }
    }
}

///
/// **AWS API**: `ec2.v1.ModifyInstanceMetadataOptionsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//ModifyInstanceMetadataOptionsRequest>
///
/// ## Coverage
/// 3 of 7 fields included.
/// Omitted fields:
/// - `HttpPutResponseHopLimit` — not selected in manifest
/// - `DryRun` — not selected in manifest
/// - `HttpProtocolIpv6` — not selected in manifest
/// - `InstanceMetadataTags` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyInstanceMetadataOptionsRequest {
    /// The ID of the instance.
    #[serde(rename = "InstanceId")]
    pub instance_id: String,

    /// Indicates whether IMDSv2 is required. optional
    /// - IMDSv2 is optional. You can choose whether to send a session token in your instance
    ///   metadata retrieval requests. If you retrieve IAM role credentials without a session
    ///   token, you receive the IMDSv1 role credentials. If you retrieve IAM role credentials
    ///   using a valid session token, you receive the IMDSv2 role credentials. required
    /// - IMDSv2 is required. You must send a session token in your instance metadata retrieval
    ///   requests. With this option, retrieving the IAM role credentials always returns IMDSv2
    ///   credentials; IMDSv1 credentials are not available. Default: If the value of
    ///   ImdsSupport for the Amazon Machine Image (AMI) for your instance is v2.0 and the
    ///   account level default is set to no-preference, the default is required. If the value
    ///   of ImdsSupport for the Amazon Machine Image (AMI) for your instance is v2.0, but the
    ///   account level default is set to V1 or V2, the default is optional. The default value
    ///   can also be affected by other combinations of parameters. For more information, see
    ///   Order of precedence for instance metadata options in the Amazon EC2 User Guide.
    #[serde(rename = "HttpTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_tokens: Option<String>,

    /// Enables or disables the HTTP metadata endpoint on your instances. If this parameter is
    /// not specified, the existing state is maintained. If you specify a value of disabled, you
    /// cannot access your instance metadata.
    #[serde(rename = "HttpEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<String>,
}

impl ModifyInstanceMetadataOptionsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_id: "test-instance_id".into(),
            http_tokens: Some("test-http_tokens".into()),
            http_endpoint: Some("test-http_endpoint".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.MonitorInstancesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//MonitorInstancesRequest>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MonitorInstancesRequest {
    /// The IDs of the instances.
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_ids: Vec<String>,
}

impl MonitorInstancesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_ids: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.AssociateIamInstanceProfileRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//AssociateIamInstanceProfileRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssociateIamInstanceProfileRequest {
    /// The IAM instance profile.
    #[serde(rename = "IamInstanceProfile")]
    pub iam_instance_profile: IamInstanceProfileSpecification,

    /// The ID of the instance.
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl AssociateIamInstanceProfileRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            iam_instance_profile: IamInstanceProfileSpecification::fixture(),
            instance_id: "test-instance_id".into(),
        }
    }
}

/// Describes an IAM instance profile.
///
/// **AWS API**: `ec2.v1.IamInstanceProfileSpecification`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//IamInstanceProfileSpecification>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IamInstanceProfileSpecification {
    /// The Amazon Resource Name (ARN) of the instance profile.
    #[serde(rename(serialize = "Arn", deserialize = "arn"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The name of the instance profile.
    #[serde(rename(serialize = "Name", deserialize = "name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl IamInstanceProfileSpecification {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            arn: Some("test-arn".into()),
            name: Some("test-name".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DetachVolumeRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DetachVolumeRequest>
///
/// ## Coverage
/// 2 of 5 fields included.
/// Omitted fields:
/// - `Device` — not selected in manifest
/// - `Force` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DetachVolumeRequest {
    /// The ID of the volume.
    #[serde(rename = "VolumeId")]
    pub volume_id: String,

    /// The ID of the instance. If you are detaching a Multi-Attach enabled volume, you must
    /// specify an instance ID.
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl DetachVolumeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            volume_id: "test-volume_id".into(),
            instance_id: Some("test-instance_id".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DeleteVolumeRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DeleteVolumeRequest>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteVolumeRequest {
    /// The ID of the volume.
    #[serde(rename = "VolumeId")]
    pub volume_id: String,
}

impl DeleteVolumeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            volume_id: "test-volume_id".into(),
        }
    }
}

///
/// **AWS API**: `ec2.v1.ModifyVolumeRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//ModifyVolumeRequest>
///
/// ## Coverage
/// 4 of 7 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `Size` — not selected in manifest
/// - `MultiAttachEnabled` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyVolumeRequest {
    /// The ID of the volume.
    #[serde(rename = "VolumeId")]
    pub volume_id: String,

    /// The target EBS volume type of the volume. For more information, see Amazon EBS volume
    /// types in the Amazon EBS User Guide. Default: The existing type is retained.
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,

    /// The target IOPS rate of the volume. This parameter is valid only for gp3, io1, and io2
    /// volumes. The following are the supported values for each volume type: gp3: 3,000 -
    /// 80,000 IOPS io1: 100 - 64,000 IOPS io2: 100 - 256,000 IOPS Instances built on the Nitro
    /// System can support up to 256,000 IOPS. Other instances can support up to 32,000 IOPS.
    /// Default: The existing value is retained if you keep the same volume type. If you change
    /// the volume type to io1, io2, or gp3, the default is 3,000.
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,

    /// The target throughput of the volume, in MiB/s. This parameter is valid only for gp3
    /// volumes. The maximum value is 2,000. Default: The existing value is retained if the
    /// source and target volume type is gp3. Otherwise, the default value is 125. Valid Range:
    /// Minimum value of 125. Maximum value of 2,000.
    #[serde(rename = "Throughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i32>,
}

impl ModifyVolumeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            volume_id: "test-volume_id".into(),
            volume_type: Some("test-volume_type".into()),
            iops: Some(100),
            throughput: Some(100),
        }
    }
}

///
/// **AWS API**: `ec2.v1.CreateSnapshotRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//CreateSnapshotRequest>
///
/// ## Coverage
/// 2 of 6 fields included.
/// Omitted fields:
/// - `OutpostArn` — not selected in manifest
/// - `TagSpecifications` — not selected in manifest
/// - `Location` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSnapshotRequest {
    /// The ID of the Amazon EBS volume.
    #[serde(rename = "VolumeId")]
    pub volume_id: String,

    /// A description for the snapshot.
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateSnapshotRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            volume_id: "test-volume_id".into(),
            description: Some("test-description".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DeleteSnapshotRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DeleteSnapshotRequest>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteSnapshotRequest {
    /// The ID of the EBS snapshot.
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

impl DeleteSnapshotRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshot_id: "test-snapshot_id".into(),
        }
    }
}

///
/// **AWS API**: `ec2.v1.ModifySnapshotAttributeRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//ModifySnapshotAttributeRequest>
///
/// ## Coverage
/// 3 of 7 fields included.
/// Omitted fields:
/// - `GroupNames` — not selected in manifest
/// - `OperationType` — not selected in manifest
/// - `UserIds` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifySnapshotAttributeRequest {
    /// The ID of the snapshot.
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,

    /// The snapshot attribute to modify. Only volume creation permissions can be modified.
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,

    /// A JSON representation of the snapshot attribute modification.
    #[serde(rename = "CreateVolumePermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_volume_permission: Option<CreateVolumePermissionModifications>,
}

impl ModifySnapshotAttributeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshot_id: "test-snapshot_id".into(),
            attribute: Some("test-attribute".into()),
            create_volume_permission: Some(CreateVolumePermissionModifications::fixture()),
        }
    }
}

/// Describes modifications to the list of create volume permissions for a volume.
///
/// **AWS API**: `ec2.v1.CreateVolumePermissionModifications`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//CreateVolumePermissionModifications>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVolumePermissionModifications {
    /// Adds the specified Amazon Web Services account ID or group to the list.
    #[serde(rename = "Add")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<CreateVolumePermission>,

    /// Removes the specified Amazon Web Services account ID or group from the list.
    #[serde(rename = "Remove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub remove: Vec<CreateVolumePermission>,
}

impl CreateVolumePermissionModifications {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            add: vec![],
            remove: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.EnableSnapshotBlockPublicAccessRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//EnableSnapshotBlockPublicAccessRequest>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnableSnapshotBlockPublicAccessRequest {
    /// The mode in which to enable block public access for snapshots for the Region. Specify
    /// one of the following values: block-all-sharing
    /// - Prevents all public sharing of snapshots in the Region. Users in the account will no
    ///   longer be able to request new public sharing. Additionally, snapshots that are already
    ///   publicly shared are treated as private and they are no longer publicly available.
    ///   block-new-sharing
    /// - Prevents only new public sharing of snapshots in the Region. Users in the account will
    ///   no longer be able to request new public sharing. However, snapshots that are already
    ///   publicly shared, remain publicly available. unblocked is not a valid value for
    ///   EnableSnapshotBlockPublicAccess.
    #[serde(rename = "State")]
    pub state: String,
}

impl EnableSnapshotBlockPublicAccessRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            state: "test-state".into(),
        }
    }
}

///
/// **AWS API**: `ec2.v1.ReleaseAddressRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//ReleaseAddressRequest>
///
/// ## Coverage
/// 1 of 4 fields included.
/// Omitted fields:
/// - `PublicIp` — not selected in manifest
/// - `NetworkBorderGroup` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReleaseAddressRequest {
    /// The allocation ID. This parameter is required.
    #[serde(rename = "AllocationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<String>,
}

impl ReleaseAddressRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allocation_id: Some("test-allocation_id".into()),
        }
    }
}

/// Contains the parameters for DeregisterImage.
///
/// **AWS API**: `ec2.v1.DeregisterImageRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DeregisterImageRequest>
///
/// ## Coverage
/// 1 of 3 fields included.
/// Omitted fields:
/// - `DeleteAssociatedSnapshots` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeregisterImageRequest {
    /// The ID of the AMI.
    #[serde(rename = "ImageId")]
    pub image_id: String,
}

impl DeregisterImageRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_id: "test-image_id".into(),
        }
    }
}

/// Contains the parameters for ModifyImageAttribute.
///
/// **AWS API**: `ec2.v1.ModifyImageAttributeRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//ModifyImageAttributeRequest>
///
/// ## Coverage
/// 2 of 13 fields included.
/// Omitted fields:
/// - `Attribute` — not selected in manifest
/// - `Description` — not selected in manifest
/// - `OperationType` — not selected in manifest
/// - `ProductCodes` — not selected in manifest
/// - `UserGroups` — not selected in manifest
/// - `UserIds` — not selected in manifest
/// - `Value` — not selected in manifest
/// - `OrganizationArns` — not selected in manifest
/// - `OrganizationalUnitArns` — not selected in manifest
/// - `ImdsSupport` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyImageAttributeRequest {
    /// The ID of the AMI.
    #[serde(rename = "ImageId")]
    pub image_id: String,

    /// A new launch permission for the AMI.
    #[serde(rename = "LaunchPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_permission: Option<LaunchPermissionModifications>,
}

impl ModifyImageAttributeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_id: "test-image_id".into(),
            launch_permission: Some(LaunchPermissionModifications::fixture()),
        }
    }
}

/// Describes a launch permission modification.
///
/// **AWS API**: `ec2.v1.LaunchPermissionModifications`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//LaunchPermissionModifications>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaunchPermissionModifications {
    /// The Amazon Web Services account ID, organization ARN, or OU ARN to add to the list of
    /// launch permissions for the AMI.
    #[serde(rename = "Add")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<LaunchPermission>,

    /// The Amazon Web Services account ID, organization ARN, or OU ARN to remove from the list
    /// of launch permissions for the AMI.
    #[serde(rename = "Remove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub remove: Vec<LaunchPermission>,
}

impl LaunchPermissionModifications {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            add: vec![],
            remove: vec![],
        }
    }
}

/// Describes a launch permission.
///
/// **AWS API**: `ec2.v1.LaunchPermission`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//LaunchPermission>
///
/// ## Coverage
/// 2 of 4 fields included.
/// Omitted fields:
/// - `OrganizationArn` — not selected in manifest
/// - `OrganizationalUnitArn` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaunchPermission {
    /// The name of the group.
    #[serde(rename(serialize = "Group", deserialize = "group"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// The Amazon Web Services account ID. Constraints: Up to 10 000 account IDs can be
    /// specified in a single request.
    #[serde(rename(serialize = "UserId", deserialize = "userId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl LaunchPermission {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group: Some("test-group".into()),
            user_id: Some("test-user_id".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.EnableImageBlockPublicAccessRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//EnableImageBlockPublicAccessRequest>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnableImageBlockPublicAccessRequest {
    /// Specify block-new-sharing to enable block public access for AMIs at the account level in
    /// the specified Region. This will block any attempt to publicly share your AMIs in the
    /// specified Region.
    #[serde(rename = "ImageBlockPublicAccessState")]
    pub image_block_public_access_state: String,
}

impl EnableImageBlockPublicAccessRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_block_public_access_state: "test-image_block_public_access_state".into(),
        }
    }
}

///
/// **AWS API**: `ec2.v1.RevokeSecurityGroupIngressRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//RevokeSecurityGroupIngressRequest>
///
/// ## Coverage
/// 2 of 11 fields included.
/// Omitted fields:
/// - `CidrIp` — not selected in manifest
/// - `FromPort` — not selected in manifest
/// - `GroupName` — not selected in manifest
/// - `IpProtocol` — not selected in manifest
/// - `SourceSecurityGroupName` — not selected in manifest
/// - `SourceSecurityGroupOwnerId` — not selected in manifest
/// - `ToPort` — not selected in manifest
/// - `SecurityGroupRuleIds` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevokeSecurityGroupIngressRequest {
    /// The ID of the security group.
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    /// The sets of IP permissions. You can't specify a source security group and a CIDR IP
    /// address range in the same set of permissions.
    #[serde(rename = "IpPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ip_permissions: Vec<IpPermission>,
}

impl RevokeSecurityGroupIngressRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_id: Some("test-group_id".into()),
            ip_permissions: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.RevokeSecurityGroupEgressRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//RevokeSecurityGroupEgressRequest>
///
/// ## Coverage
/// 2 of 10 fields included.
/// Omitted fields:
/// - `SecurityGroupRuleIds` — not selected in manifest
/// - `DryRun` — not selected in manifest
/// - `SourceSecurityGroupName` — not selected in manifest
/// - `SourceSecurityGroupOwnerId` — not selected in manifest
/// - `IpProtocol` — not selected in manifest
/// - `FromPort` — not selected in manifest
/// - `ToPort` — not selected in manifest
/// - `CidrIp` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevokeSecurityGroupEgressRequest {
    /// The ID of the security group.
    #[serde(rename(serialize = "GroupId", deserialize = "groupId"))]
    pub group_id: String,

    /// The sets of IP permissions. You can't specify a destination security group and a CIDR IP
    /// address range in the same set of permissions.
    #[serde(rename(serialize = "IpPermissions", deserialize = "ipPermissions"))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ip_permissions: Vec<IpPermission>,
}

impl RevokeSecurityGroupEgressRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_id: "test-group_id".into(),
            ip_permissions: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.AuthorizeSecurityGroupIngressRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//AuthorizeSecurityGroupIngressRequest>
///
/// ## Coverage
/// 2 of 11 fields included.
/// Omitted fields:
/// - `CidrIp` — not selected in manifest
/// - `FromPort` — not selected in manifest
/// - `GroupName` — not selected in manifest
/// - `IpProtocol` — not selected in manifest
/// - `SourceSecurityGroupName` — not selected in manifest
/// - `SourceSecurityGroupOwnerId` — not selected in manifest
/// - `ToPort` — not selected in manifest
/// - `TagSpecifications` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizeSecurityGroupIngressRequest {
    /// The ID of the security group.
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    /// The permissions for the security group rules.
    #[serde(rename = "IpPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ip_permissions: Vec<IpPermission>,
}

impl AuthorizeSecurityGroupIngressRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_id: Some("test-group_id".into()),
            ip_permissions: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.DeleteSecurityGroupRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DeleteSecurityGroupRequest>
///
/// ## Coverage
/// 1 of 3 fields included.
/// Omitted fields:
/// - `GroupName` — not selected in manifest
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteSecurityGroupRequest {
    /// The ID of the security group.
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl DeleteSecurityGroupRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_id: Some("test-group_id".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DeleteNatGatewayRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DeleteNatGatewayRequest>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteNatGatewayRequest {
    /// The ID of the NAT gateway.
    #[serde(rename = "NatGatewayId")]
    pub nat_gateway_id: String,
}

impl DeleteNatGatewayRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            nat_gateway_id: "test-nat_gateway_id".into(),
        }
    }
}

///
/// **AWS API**: `ec2.v1.DeleteVpcEndpointsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//DeleteVpcEndpointsRequest>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteVpcEndpointsRequest {
    /// The IDs of the VPC endpoints.
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    pub vpc_endpoint_ids: Vec<String>,
}

impl DeleteVpcEndpointsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vpc_endpoint_ids: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.CreateFlowLogsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//CreateFlowLogsRequest>
///
/// ## Coverage
/// 5 of 14 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
/// - `ClientToken` — not selected in manifest
/// - `DeliverCrossAccountRole` — not selected in manifest
/// - `LogDestinationType` — not selected in manifest
/// - `LogDestination` — not selected in manifest
/// - `LogFormat` — not selected in manifest
/// - `TagSpecifications` — not selected in manifest
/// - `MaxAggregationInterval` — not selected in manifest
/// - `DestinationOptions` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateFlowLogsRequest {
    /// The IDs of the resources to monitor. For example, if the resource type is VPC, specify
    /// the IDs of the VPCs. Constraints: Maximum of 25 for transit gateway resource types.
    /// Maximum of 1000 for the other resource types.
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_ids: Vec<String>,

    /// The type of resource to monitor.
    #[serde(rename = "ResourceType")]
    pub resource_type: String,

    /// The type of traffic to monitor (accepted traffic, rejected traffic, or all traffic).
    /// This parameter is not supported for transit gateway resource types. It is required for
    /// the other resource types.
    #[serde(rename = "TrafficType")]
    pub traffic_type: String,

    /// The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your
    /// flow logs. This parameter is valid only if the destination type is cloud-watch-logs.
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,

    /// The ARN of the IAM role that allows Amazon EC2 to publish flow logs to the log
    /// destination. This parameter is required if the destination type is cloud-watch-logs, or
    /// if the destination type is kinesis-data-firehose and the delivery stream and the
    /// resources to monitor are in different accounts.
    #[serde(rename = "DeliverLogsPermissionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_logs_permission_arn: Option<String>,
}

impl CreateFlowLogsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource_ids: vec![],
            resource_type: "test-resource_type".into(),
            traffic_type: "test-traffic_type".into(),
            log_group_name: Some("test-log_group_name".into()),
            deliver_logs_permission_arn: Some("test-deliver_logs_permission_arn".into()),
        }
    }
}

///
/// **AWS API**: `ec2.v1.CreateTagsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//CreateTagsRequest>
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `DryRun` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTagsRequest {
    /// The IDs of the resources, separated by spaces. Constraints: Up to 1000 resource IDs. We
    /// recommend breaking up this request into smaller batches.
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resources: Vec<String>,

    /// The tags. The value parameter is required, but if you don't want the tag to have a
    /// value, specify the parameter with no value, and we set the value to an empty string.
    #[serde(rename = "Tag")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

impl CreateTagsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resources: vec![],
            tags: vec![],
        }
    }
}

///
/// **AWS API**: `ec2.v1.EnableEbsEncryptionByDefaultRequest`
/// **Reference**: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference//EnableEbsEncryptionByDefaultRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnableEbsEncryptionByDefaultRequest {
    /// Checks whether you have the required permissions for the action, without actually making
    /// the request, and provides an error response. If you have the required permissions, the
    /// error response is DryRunOperation. Otherwise, it is UnauthorizedOperation.
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl EnableEbsEncryptionByDefaultRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            dry_run: Some(false),
        }
    }
}

// ======================================================================
// Auto-generated dependency types (referenced via $ref)
// ======================================================================

/// The snapshot ID and its deletion result code.
///
/// **AWS API**: `ec2.v1.DeleteSnapshotReturnCode`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteSnapshotReturnCode {
    /// The ID of the snapshot.
    #[serde(rename(serialize = "SnapshotId", deserialize = "snapshotId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,

    /// The result code from the snapshot deletion attempt. Possible values: success
    /// - The snapshot was successfully deleted. skipped
    /// - The snapshot was not deleted because it's associated with other AMIs. missing-
    ///   permissions
    /// - The snapshot was not deleted because the role lacks DeleteSnapshot permissions. For
    ///   more information, see How Amazon EBS works with IAM. internal-error
    /// - The snapshot was not deleted due to a server error. client-error
    /// - The snapshot was not deleted due to a client configuration error. For details about an
    ///   error, check the DeleteSnapshot event in the CloudTrail event history. For more
    ///   information, see View event history in the Amazon Web Services CloudTrail User Guide.
    #[serde(rename(serialize = "ReturnCode", deserialize = "returnCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_code: Option<String>,
}

impl DeleteSnapshotReturnCode {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            snapshot_id: Some("test-snapshot_id".into()),
            return_code: Some("test-return_code".into()),
        }
    }
}
