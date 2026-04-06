//! AWS API type definitions.

pub mod accessanalyzer;
pub mod apigateway;
pub mod autoscaling;
pub mod ce;
pub mod cloudfront;
pub mod cloudtrail;
pub mod cloudwatch;
pub mod config;
pub mod dynamodb;
pub mod ec2;
pub mod ecr;
pub mod ecs;
pub mod efs;
pub mod eks;
pub mod elasticache;
pub mod elbv2;
pub mod emr;
pub mod iam;
pub mod kinesis;
pub mod kms;
pub mod lambda;
pub mod logs;
pub mod opensearch;
pub mod organizations;
pub mod rds;
pub mod redshift;
pub mod route53;
pub mod s3;
pub mod sagemaker;
pub mod secretsmanager;
pub mod securityhub;
pub mod sts;

pub use accessanalyzer::{AnalyzerSummary, ListAnalyzersRequest, ListAnalyzersResponse};
pub use apigateway::{
    GetRestApisRequest, GetStagesRequest, MethodSetting, PatchOperation, RestApi, RestApis, Stage,
    Stages, UpdateStageRequest,
};
pub use autoscaling::{
    AutoScalingGroup, DescribeAutoScalingGroupsRequest, DescribeAutoScalingGroupsResponse,
    DescribeLaunchConfigurationsRequest, DescribeLaunchConfigurationsResponse, LaunchConfiguration,
    LaunchTemplateSpecification, MixedInstancesPolicy, RefreshPreferences,
    StartInstanceRefreshRequest, StartInstanceRefreshResponse, UpdateAutoScalingGroupRequest,
};
pub use ce::{
    DateInterval, GetCostAndUsageRequest, GetCostAndUsageResponse, GroupDefinition, MetricValue,
    ResultByTime,
};
pub use cloudfront::{
    Aliases, DefaultCacheBehavior, Distribution, DistributionConfig, DistributionList,
    DistributionSummary, Origin, OriginAccessControl, OriginAccessControlConfig, Origins,
    S3OriginConfig, ViewerCertificate,
};
pub use cloudtrail::{
    AdvancedEventSelector, AdvancedFieldSelector, DataResource, DeleteTrailRequest,
    DeleteTrailResponse, DescribeTrailsRequest, DescribeTrailsResponse, EventSelector,
    GetEventSelectorsRequest, GetEventSelectorsResponse, GetTrailStatusRequest,
    GetTrailStatusResponse, Trail, UpdateTrailRequest, UpdateTrailResponse,
};
pub use cloudwatch::{
    ComparisonOperator, Datapoint, DeleteAlarmsInput, DescribeAlarmsInput, DescribeAlarmsResponse,
    Dimension, DimensionFilter, GetMetricDataInput, GetMetricDataResponse,
    GetMetricStatisticsInput, GetMetricStatisticsResponse, LabelOptions, ListMetricsInput,
    ListMetricsResponse, MessageData, Metric, MetricAlarm, MetricDataQuery, MetricDataResult,
    MetricStat, PutMetricAlarmInput, ScanBy, StandardUnit, Statistic, StatusCode,
};
pub use config::{
    ConfigurationRecorder, ConfigurationRecorderStatus, DescribeConfigurationRecorderStatusRequest,
    DescribeConfigurationRecorderStatusResponse, DescribeConfigurationRecordersRequest,
    DescribeConfigurationRecordersResponse, FieldInfo, QueryInfo, RecorderStatus, RecordingGroup,
    RecordingStrategy, SelectResourceConfigRequest, SelectResourceConfigResponse,
};
pub use dynamodb::{
    ArchivalSummary, AttributeDefinition, BillingModeSummary, DeleteTableInput, DeleteTableOutput,
    DescribeTableInput, DescribeTableOutput, KeySchemaElement, ListTablesInput, ListTablesOutput,
    ProvisionedThroughput, ProvisionedThroughputDescription, SSEDescription, StreamSpecification,
    TableClassSummary, TableDescription, UpdateTableInput, UpdateTableOutput,
};
pub use ec2::{
    Address, AssociateIamInstanceProfileRequest, AssociateIamInstanceProfileResponse,
    AuthorizeSecurityGroupIngressRequest, AuthorizeSecurityGroupIngressResponse,
    BlockDeviceMapping, CreateFlowLogsRequest, CreateFlowLogsResponse, CreateTagsRequest,
    CreateVolumePermission, CreateVolumePermissionModifications, DeleteNatGatewayRequest,
    DeleteNatGatewayResponse, DeleteSecurityGroupRequest, DeleteSecurityGroupResponse,
    DeleteSnapshotRequest, DeleteVolumeRequest, DeleteVpcEndpointsRequest,
    DeleteVpcEndpointsResponse, DeregisterImageRequest, DeregisterImageResponse,
    DescribeAddressesRequest, DescribeAddressesResponse, DescribeFlowLogsRequest,
    DescribeFlowLogsResponse, DescribeInstancesRequest, DescribeInstancesResponse,
    DescribeLaunchTemplateVersionsRequest, DescribeLaunchTemplateVersionsResponse,
    DescribeLaunchTemplatesRequest, DescribeLaunchTemplatesResponse, DescribeNatGatewaysRequest,
    DescribeNatGatewaysResponse, DescribeNetworkAclsRequest, DescribeNetworkAclsResponse,
    DescribeRouteTablesRequest, DescribeRouteTablesResponse, DescribeSecurityGroupsRequest,
    DescribeSecurityGroupsResponse, DescribeSnapshotAttributeRequest,
    DescribeSnapshotAttributeResponse, DescribeSnapshotsRequest, DescribeSnapshotsResponse,
    DescribeVolumesRequest, DescribeVolumesResponse, DescribeVpcEndpointsRequest,
    DescribeVpcEndpointsResponse, DescribeVpcPeeringConnectionsRequest,
    DescribeVpcPeeringConnectionsResponse, DescribeVpcsRequest, DescribeVpcsResponse,
    DetachVolumeRequest, EbsBlockDevice, EbsInstanceBlockDevice,
    EnableEbsEncryptionByDefaultRequest, EnableEbsEncryptionByDefaultResponse,
    EnableImageBlockPublicAccessRequest, EnableImageBlockPublicAccessResponse,
    EnableSnapshotBlockPublicAccessRequest, EnableSnapshotBlockPublicAccessResponse, FlowLog,
    GetEbsEncryptionByDefaultRequest, GetEbsEncryptionByDefaultResponse, GroupIdentifier,
    IamInstanceProfile, IamInstanceProfileAssociation, IamInstanceProfileSpecification, Image,
    Instance, InstanceBlockDeviceMapping, InstanceMetadataOptionsResponse, InstanceMonitoring,
    InstanceNetworkInterface, InstanceState, InstanceStateChange, IpPermission, IpRange, Ipv6Range,
    LaunchPermission, LaunchPermissionModifications, LaunchTemplateInstanceMetadataOptions,
    LaunchTemplateInstanceNetworkInterfaceSpecification, LaunchTemplateVersion,
    ModifyImageAttributeRequest, ModifyInstanceAttributeRequest,
    ModifyInstanceMetadataOptionsRequest, ModifyInstanceMetadataOptionsResponse,
    ModifySnapshotAttributeRequest, ModifyVolumeRequest, ModifyVolumeResponse,
    MonitorInstancesRequest, MonitorInstancesResponse, Monitoring, NatGateway, NatGatewayAddress,
    NetworkAcl, NetworkAclEntry, Placement, PortRange, ReleaseAddressRequest, Reservation,
    ResponseLaunchTemplateData, RevokeSecurityGroupEgressRequest,
    RevokeSecurityGroupEgressResponse, RevokeSecurityGroupIngressRequest,
    RevokeSecurityGroupIngressResponse, Route, RouteTable, RouteTableAssociation, SecurityGroup,
    SecurityGroupRule, StartInstancesRequest, StartInstancesResponse, StopInstancesRequest,
    StopInstancesResponse, TerminateInstancesRequest, TerminateInstancesResponse, UnsuccessfulItem,
    UnsuccessfulItemError, Volume, VolumeAttachment, VolumeModification, Vpc, VpcEndpoint,
    VpcPeeringConnection, VpcPeeringConnectionStateReason, VpcPeeringConnectionVpcInfo,
};
pub use ecr::{
    BatchDeleteImageRequest, BatchDeleteImageResponse, DescribeImagesFilter,
    DescribeRepositoriesRequest, DescribeRepositoriesResponse, ImageDetail, ImageFailure,
    ImageIdentifier, PutLifecyclePolicyRequest, PutLifecyclePolicyResponse, Repository,
};
pub use ecs::{
    AwsVpcConfiguration, ClusterSetting, ContainerDefinition, Deployment,
    DeregisterTaskDefinitionRequest, DeregisterTaskDefinitionResponse, DescribeServicesRequest,
    DescribeServicesResponse, DescribeTaskDefinitionRequest, DescribeTaskDefinitionResponse,
    Failure, KeyValuePair, ListClustersRequest, ListClustersResponse, ListServicesRequest,
    ListServicesResponse, LogConfiguration, NetworkConfiguration, PortMapping, Service,
    ServiceEvent, TaskDefinition, UpdateServiceRequest, UpdateServiceResponse,
};
pub use efs::{DescribeFileSystemsRequest, DescribeFileSystemsResponse, FileSystemDescription};
pub use eks::{
    DescribeClusterRequest, DescribeClusterResponse, DescribeNodegroupRequest,
    DescribeNodegroupResponse, ErrorDetail, Issue, ListNodegroupsRequest, ListNodegroupsResponse,
    Nodegroup, NodegroupHealth, NodegroupScalingConfig, NodegroupUpdateConfig, Update,
    UpdateNodegroupConfigRequest, UpdateNodegroupConfigResponse,
};
pub use elasticache::{
    CacheCluster, CacheNode, CreateSnapshotResponse, DeleteCacheClusterRequest,
    DeleteCacheClusterResponse, DeleteReplicationGroupRequest, DeleteReplicationGroupResponse,
    DescribeCacheClustersRequest, DescribeCacheClustersResponse, DescribeReplicationGroupsRequest,
    DescribeReplicationGroupsResponse, NodeGroup, NodeGroupMember, ReplicationGroup,
};
pub use elbv2::{
    AvailabilityZone, DeleteLoadBalancerRequest, DeleteLoadBalancerResponse,
    DeleteTargetGroupRequest, DeleteTargetGroupResponse, DescribeListenersRequest,
    DescribeListenersResponse, DescribeLoadBalancerAttributesRequest,
    DescribeLoadBalancerAttributesResponse, DescribeLoadBalancersRequest,
    DescribeLoadBalancersResponse, DescribeTargetGroupsRequest, DescribeTargetGroupsResponse,
    DescribeTargetHealthRequest, DescribeTargetHealthResponse, Listener, LoadBalancerAttribute,
    LoadBalancerState, ModifyLoadBalancerAttributesRequest, ModifyLoadBalancerAttributesResponse,
    TargetDescription, TargetGroup, TargetHealth, TargetHealthDescription,
};
pub use emr::{
    ClusterStateChangeReason, ClusterStatus, ClusterSummary, ClusterTimeline, DescribeClusterInput,
    DescribeClusterOutput, Ec2InstanceAttributes, ListClustersInput, ListClustersOutput,
    TerminateJobFlowsInput,
};
pub use iam::{
    AccessKeyLastUsed, AccessKeyMetadata, AccessKeyStatus, AttachRolePolicyRequest, AttachedPolicy,
    CreateServiceLinkedRoleRequest, CreateServiceLinkedRoleResponse, DeleteAccessKeyRequest,
    DeleteUserPolicyRequest, DetachRolePolicyRequest, DetachUserPolicyRequest,
    GenerateCredentialReportResponse, GetAccessKeyLastUsedRequest, GetAccessKeyLastUsedResponse,
    GetAccountPasswordPolicyResponse, GetAccountSummaryResponse, GetCredentialReportResponse,
    GetLoginProfileRequest, GetLoginProfileResponse, GetPolicyVersionRequest,
    GetPolicyVersionResponse, GetUserPolicyRequest, GetUserPolicyResponse, ListAccessKeysRequest,
    ListAccessKeysResponse, ListAttachedGroupPoliciesRequest, ListAttachedGroupPoliciesResponse,
    ListAttachedUserPoliciesRequest, ListAttachedUserPoliciesResponse,
    ListEntitiesForPolicyRequest, ListEntitiesForPolicyResponse, ListGroupsForUserRequest,
    ListGroupsForUserResponse, ListMFADevicesRequest, ListMFADevicesResponse, ListPoliciesRequest,
    ListPoliciesResponse, ListRolesRequest, ListRolesResponse, ListServerCertificatesRequest,
    ListServerCertificatesResponse, ListUserPoliciesRequest, ListUserPoliciesResponse,
    ListUsersRequest, ListUsersResponse, ListVirtualMFADevicesRequest,
    ListVirtualMFADevicesResponse, LoginProfile, MFADevice, PasswordPolicy, Policy, PolicyGroup,
    PolicyRole, PolicyUser, PolicyVersion, ReportFormatType, ReportStateType, Role,
    ServerCertificateMetadata, UpdateAccessKeyRequest, UpdateAccountPasswordPolicyRequest, User,
    VirtualMFADevice,
};
pub use kinesis::{
    DeleteStreamInput, DescribeStreamSummaryInput, DescribeStreamSummaryOutput, ListStreamsInput,
    ListStreamsOutput, StreamDescriptionSummary, StreamModeDetails, StreamSummary,
    UpdateStreamModeInput,
};
pub use kms::{
    DescribeKeyRequest, DescribeKeyResponse, EnableKeyRotationRequest, GetKeyRotationStatusRequest,
    GetKeyRotationStatusResponse, KeyListEntry, KeyMetadata, ListKeysRequest, ListKeysResponse,
};
pub use lambda::{
    EnvironmentResponse, FunctionConfiguration, GetFunctionConfigurationRequest,
    ListFunctionsRequest, ListFunctionsResponse, UpdateFunctionConfigurationRequest,
};
pub use logs::{
    DeleteLogStreamRequest, DescribeLogGroupsRequest, DescribeLogGroupsResponse,
    DescribeLogStreamsRequest, DescribeLogStreamsResponse, DescribeMetricFiltersRequest,
    DescribeMetricFiltersResponse, ListTagsForResourceRequest, ListTagsForResourceResponse,
    LogGroup, LogGroupClass, LogStream, MetricFilter, MetricTransformation,
    PutRetentionPolicyRequest,
};
pub use opensearch::{
    DeleteDomainRequest, DeleteDomainResponse, DescribeDomainRequest, DescribeDomainResponse,
    DomainInfo, DomainStatus, EngineType, ListDomainNamesRequest, ListDomainNamesResponse,
};
pub use organizations::{
    Account, AccountJoinedMethod, AccountStatus, ListAccountsForParentRequest,
    ListAccountsForParentResponse, ListOrganizationalUnitsForParentRequest,
    ListOrganizationalUnitsForParentResponse, OrganizationalUnit,
};
pub use rds::{
    CreateDBSnapshotRequest, CreateDBSnapshotResponse, DBInstance, DBSnapshot, DBSnapshotAttribute,
    DBSnapshotAttributesResult, DeleteDBInstanceRequest, DeleteDBInstanceResponse,
    DeleteDBSnapshotRequest, DeleteDBSnapshotResponse, DescribeDBInstancesRequest,
    DescribeDBInstancesResponse, DescribeDBSnapshotAttributesRequest,
    DescribeDBSnapshotAttributesResponse, DescribeDBSnapshotsRequest, DescribeDBSnapshotsResponse,
    Filter, ModifyDBInstanceRequest, ModifyDBInstanceResponse, ModifyDBSnapshotAttributeRequest,
    ModifyDBSnapshotAttributeResponse, StartDBInstanceRequest, StartDBInstanceResponse,
    StopDBInstanceRequest, StopDBInstanceResponse,
};
pub use redshift::{
    DeleteClusterRequest, DeleteClusterResponse, PauseClusterRequest, PauseClusterResponse,
    ResizeClusterRequest, ResizeClusterResponse, ResumeClusterRequest, ResumeClusterResponse,
};
pub use route53::{
    AliasTarget, Change, ChangeAction, ChangeBatch, ChangeInfo, ChangeResourceRecordSetsRequest,
    ChangeResourceRecordSetsResponse, ChangeStatus, CreateHealthCheckRequest,
    CreateHealthCheckResponse, GetHealthCheckStatusResponse, HealthCheck, HealthCheckConfig,
    HealthCheckObservation, HealthCheckType, HostedZone, HostedZoneConfig,
    ListHealthChecksResponse, ListHostedZonesResponse, ListResourceRecordSetsResponse, RRType,
    ResourceRecord, ResourceRecordSet, StatusReport,
};
pub use s3::{
    Bucket, BucketLifecycleConfiguration, BucketLoggingStatus, GetBucketAclResponse,
    GetBucketEncryptionResponse, GetBucketLifecycleConfigurationResponse, GetBucketLoggingResponse,
    GetBucketVersioningResponse, GetPublicAccessBlockResponse, Grant, Grantee, LifecycleRule,
    LifecycleRuleFilter, ListBucketsResponse, LoggingEnabled, Owner,
    PublicAccessBlockConfiguration, ServerSideEncryptionByDefault,
    ServerSideEncryptionConfiguration, ServerSideEncryptionRule, VersioningConfiguration,
};
pub use sagemaker::{
    ListNotebookInstancesInput, ListNotebookInstancesOutput, NotebookInstanceSummary,
    StopNotebookInstanceInput,
};
pub use secretsmanager::{
    DeleteSecretRequest, DeleteSecretResponse, ListSecretsRequest, ListSecretsResponse,
    RotateSecretRequest, RotateSecretResponse, RotationRulesType, SecretListEntry,
};
pub use securityhub::{DescribeHubRequest, DescribeHubResponse};
pub use sts::{
    AssumeRoleRequest, AssumeRoleResponse, AssumedRoleUser, Credentials, GetCallerIdentityRequest,
    GetCallerIdentityResponse,
};
