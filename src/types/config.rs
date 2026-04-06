//! Types for the AWS Config API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Possible values for `config.ConfigurationRecorderStatus.lastStatus`.
///
/// **AWS API**: `config.ConfigurationRecorderStatus.lastStatus`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecorderStatus {
    #[serde(rename = "Pending")]
    Pending,

    #[serde(rename = "Success")]
    Success,

    #[serde(rename = "Failure")]
    Failure,

    #[serde(rename = "NotApplicable")]
    NotApplicable,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

///
/// **AWS API**: `config.v1.SelectResourceConfigResponse`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//SelectResourceConfigResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SelectResourceConfigResponse {
    /// Returns the results for the SQL query.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<String>,

    /// Returns the QueryInfo object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_info: Option<QueryInfo>,

    /// The nextToken string returned in a previous request that you use to request the next
    /// page of results in a paginated response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl SelectResourceConfigResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            results: vec![],
            query_info: Some(QueryInfo::fixture()),
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Details about the query.
///
/// **AWS API**: `config.v1.QueryInfo`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//QueryInfo>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryInfo {
    /// Returns a FieldInfo object.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub select_fields: Vec<FieldInfo>,
}

impl QueryInfo {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            select_fields: vec![],
        }
    }
}

/// Details about the fields such as name of the field.
///
/// **AWS API**: `config.v1.FieldInfo`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//FieldInfo>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FieldInfo {
    /// Name of the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl FieldInfo {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-name".into()),
        }
    }
}

/// The output for the DescribeConfigurationRecorders action.
///
/// **AWS API**: `config.v1.DescribeConfigurationRecordersResponse`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//DescribeConfigurationRecordersResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeConfigurationRecordersResponse {
    /// A list that contains the descriptions of the specified configuration recorders.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub configuration_recorders: Vec<ConfigurationRecorder>,
}

impl DescribeConfigurationRecordersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            configuration_recorders: vec![],
        }
    }
}

/// Records configuration changes to the resource types in scope. For more information about the
/// configuration recorder, see Working with the Configuration Recorder in the Config Developer
/// Guide.
///
/// **AWS API**: `config.v1.ConfigurationRecorder`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//ConfigurationRecorder>
///
/// ## Coverage
/// 4 of 7 fields included.
/// Omitted fields:
/// - `recordingMode` — not selected in manifest
/// - `recordingScope` — not selected in manifest
/// - `servicePrincipal` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigurationRecorder {
    /// The Amazon Resource Name (ARN) of the specified configuration recorder.
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The name of the configuration recorder. For customer managed configuration recorders,
    /// Config automatically assigns the name of "default" when creating a configuration
    /// recorder if you do not specify a name at creation time. For service-linked configuration
    /// recorders, Config automatically assigns a name that has the prefix
    /// "AWSConfigurationRecorderFor" to a new service-linked configuration recorder. Changing
    /// the name of a configuration recorder To change the name of the customer managed
    /// configuration recorder, you must delete it and create a new customer managed
    /// configuration recorder with a new name. You cannot change the name of a service-linked
    /// configuration recorder.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The Amazon Resource Name (ARN) of the IAM role assumed by Config and used by the
    /// specified configuration recorder. The server will reject a request without a defined
    /// roleARN for the configuration recorder While the API model does not require this field,
    /// the server will reject a request without a defined roleARN for the configuration
    /// recorder. Policies and compliance results IAM policies and other policies managed in
    /// Organizations can impact whether Config has permissions to record configuration changes
    /// for your resources. Additionally, rules directly evaluate the configuration of a
    /// resource and rules don't take into account these policies when running evaluations. Make
    /// sure that the policies in effect align with how you intend to use Config. Keep Minimum
    /// Permisions When Reusing an IAM role If you use an Amazon Web Services service that uses
    /// Config, such as Security Hub CSPM or Control Tower, and an IAM role has already been
    /// created, make sure that the IAM role that you use when setting up Config keeps the same
    /// minimum permissions as the pre-existing IAM role. You must do this to ensure that the
    /// other Amazon Web Services service continues to run as expected. For example, if Control
    /// Tower has an IAM role that allows Config to read S3 objects, make sure that the same
    /// permissions are granted to the IAM role you use when setting up Config. Otherwise, it
    /// may interfere with how Control Tower operates. The service-linked IAM role for Config
    /// must be used for service-linked configuration recorders For service-linked configuration
    /// recorders, you must use the service-linked IAM role for Config: AWSServiceRoleForConfig.
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,

    /// Specifies which resource types are in scope for the configuration recorder to record.
    /// High Number of Config Evaluations You might notice increased activity in your account
    /// during your initial month recording with Config when compared to subsequent months.
    /// During the initial bootstrapping process, Config runs evaluations on all the resources
    /// in your account that you have selected for Config to record. If you are running
    /// ephemeral workloads, you may see increased activity from Config as it records
    /// configuration changes associated with creating and deleting these temporary resources.
    /// An ephemeral workload is a temporary use of computing resources that are loaded and run
    /// when needed. Examples include Amazon Elastic Compute Cloud (Amazon EC2) Spot Instances,
    /// Amazon EMR jobs, and Auto Scaling. If you want to avoid the increased activity from
    /// running ephemeral workloads, you can set up the configuration recorder to exclude these
    /// resource types from being recorded, or run these types of workloads in a separate
    /// account with Config turned off to avoid increased configuration recording and rule
    /// evaluations.
    #[serde(rename = "recordingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_group: Option<RecordingGroup>,
}

impl ConfigurationRecorder {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            arn: Some("test-arn".into()),
            name: Some("test-configuration_recorder".into()),
            role_arn: Some("test-role_arn".into()),
            recording_group: Some(RecordingGroup::fixture()),
        }
    }
}

/// Specifies which resource types Config records for configuration changes. By default, Config
/// records configuration changes for all current and future supported resource types in the
/// Amazon Web Services Region where you have enabled Config, excluding the global IAM resource
/// types: IAM users, groups, roles, and customer managed policies. In the recording group, you
/// specify whether you want to record all supported current and future supported resource types
/// or to include or exclude specific resources types. For a list of supported resource types,
/// see Supported Resource Types in the Config developer guide. If you don't want Config to
/// record all current and future supported resource types (excluding the global IAM resource
/// types), use one of the following recording strategies: Record all current and future
/// resource types with exclusions (EXCLUSION_BY_RESOURCE_TYPES), or Record specific resource
/// types (INCLUSION_BY_RESOURCE_TYPES). If you use the recording strategy to Record all current
/// and future resource types (ALL_SUPPORTED_RESOURCE_TYPES), you can use the flag
/// includeGlobalResourceTypes to include the global IAM resource types in your recording.
/// Aurora global clusters are recorded in all enabled Regions The AWS::RDS::GlobalCluster
/// resource type will be recorded in all supported Config Regions where the configuration
/// recorder is enabled. If you do not want to record AWS::RDS::GlobalCluster in all enabled
/// Regions, use the EXCLUSION_BY_RESOURCE_TYPES or INCLUSION_BY_RESOURCE_TYPES recording
/// strategy.
///
/// **AWS API**: `config.v1.RecordingGroup`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//RecordingGroup>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `resourceTypes` — not selected in manifest
/// - `exclusionByResourceTypes` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecordingGroup {
    /// Specifies whether Config records configuration changes for all supported resource types,
    /// excluding the global IAM resource types. If you set this field to true, when Config adds
    /// support for a new resource type, Config starts recording resources of that type
    /// automatically. If you set this field to true, you cannot enumerate specific resource
    /// types to record in the resourceTypes field of RecordingGroup, or to exclude in the
    /// resourceTypes field of ExclusionByResourceTypes. Region availability Check Resource
    /// Coverage by Region Availability to see if a resource type is supported in the Amazon Web
    /// Services Region where you set up Config.
    #[serde(rename = "allSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_supported: Option<bool>,

    /// This option is a bundle which only applies to the global IAM resource types: IAM users,
    /// groups, roles, and customer managed policies. These global IAM resource types can only
    /// be recorded by Config in Regions where Config was available before February 2022. You
    /// cannot be record the global IAM resouce types in Regions supported by Config after
    /// February 2022. For a list of those Regions, see Recording Amazon Web Services Resources
    /// | Global Resources. Aurora global clusters are recorded in all enabled Regions The
    /// AWS::RDS::GlobalCluster resource type will be recorded in all supported Config Regions
    /// where the configuration recorder is enabled, even if includeGlobalResourceTypes is
    /// setfalse. The includeGlobalResourceTypes option is a bundle which only applies to IAM
    /// users, groups, roles, and customer managed policies. If you do not want to record
    /// AWS::RDS::GlobalCluster in all enabled Regions, use one of the following recording
    /// strategies: Record all current and future resource types with exclusions
    /// (EXCLUSION_BY_RESOURCE_TYPES), or Record specific resource types
    /// (INCLUSION_BY_RESOURCE_TYPES). For more information, see Selecting Which Resources are
    /// Recorded in the Config developer guide. includeGlobalResourceTypes and the exclusion
    /// recording strategy The includeGlobalResourceTypes field has no impact on the
    /// EXCLUSION_BY_RESOURCE_TYPES recording strategy. This means that the global IAM resource
    /// types (IAM users, groups, roles, and customer managed policies) will not be
    /// automatically added as exclusions for exclusionByResourceTypes when
    /// includeGlobalResourceTypes is set to false. The includeGlobalResourceTypes field should
    /// only be used to modify the AllSupported field, as the default for the AllSupported field
    /// is to record configuration changes for all supported resource types excluding the global
    /// IAM resource types. To include the global IAM resource types when AllSupported is set to
    /// true, make sure to set includeGlobalResourceTypes to true. To exclude the global IAM
    /// resource types for the EXCLUSION_BY_RESOURCE_TYPES recording strategy, you need to
    /// manually add them to the resourceTypes field of exclusionByResourceTypes. Required and
    /// optional fields Before you set this field to true, set the allSupported field of
    /// RecordingGroup to true. Optionally, you can set the useOnly field of RecordingStrategy
    /// to ALL_SUPPORTED_RESOURCE_TYPES. Overriding fields If you set this field to false but
    /// list global IAM resource types in the resourceTypes field of RecordingGroup, Config will
    /// still record configuration changes for those specified resource types regardless of if
    /// you set the includeGlobalResourceTypes field to false. If you do not want to record
    /// configuration changes to the global IAM resource types (IAM users, groups, roles, and
    /// customer managed policies), make sure to not list them in the resourceTypes field in
    /// addition to setting the includeGlobalResourceTypes field to false.
    #[serde(rename = "includeGlobalResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_resource_types: Option<bool>,

    /// An object that specifies the recording strategy for the configuration recorder. If you
    /// set the useOnly field of RecordingStrategy to ALL_SUPPORTED_RESOURCE_TYPES, Config
    /// records configuration changes for all supported resource types, excluding the global IAM
    /// resource types. You also must set the allSupported field of RecordingGroup to true. When
    /// Config adds support for a new resource type, Config automatically starts recording
    /// resources of that type. If you set the useOnly field of RecordingStrategy to
    /// INCLUSION_BY_RESOURCE_TYPES, Config records configuration changes for only the resource
    /// types you specify in the resourceTypes field of RecordingGroup. If you set the useOnly
    /// field of RecordingStrategy to EXCLUSION_BY_RESOURCE_TYPES, Config records configuration
    /// changes for all supported resource types except the resource types that you specify to
    /// exclude from being recorded in the resourceTypes field of ExclusionByResourceTypes.
    /// Required and optional fields The recordingStrategy field is optional when you set the
    /// allSupported field of RecordingGroup to true. The recordingStrategy field is optional
    /// when you list resource types in the resourceTypes field of RecordingGroup. The
    /// recordingStrategy field is required if you list resource types to exclude from recording
    /// in the resourceTypes field of ExclusionByResourceTypes. Overriding fields If you choose
    /// EXCLUSION_BY_RESOURCE_TYPES for the recording strategy, the exclusionByResourceTypes
    /// field will override other properties in the request. For example, even if you set
    /// includeGlobalResourceTypes to false, global IAM resource types will still be
    /// automatically recorded in this option unless those resource types are specifically
    /// listed as exclusions in the resourceTypes field of exclusionByResourceTypes. Global
    /// resources types and the resource exclusion recording strategy By default, if you choose
    /// the EXCLUSION_BY_RESOURCE_TYPES recording strategy, when Config adds support for a new
    /// resource type in the Region where you set up the configuration recorder, including
    /// global resource types, Config starts recording resources of that type automatically.
    /// Unless specifically listed as exclusions, AWS::RDS::GlobalCluster will be recorded
    /// automatically in all supported Config Regions were the configuration recorder is
    /// enabled. IAM users, groups, roles, and customer managed policies will be recorded in the
    /// Region where you set up the configuration recorder if that is a Region where Config was
    /// available before February 2022. You cannot be record the global IAM resouce types in
    /// Regions supported by Config after February 2022. For a list of those Regions, see
    /// Recording Amazon Web Services Resources | Global Resources.
    #[serde(rename = "recordingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_strategy: Option<RecordingStrategy>,
}

impl RecordingGroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            all_supported: Some(false),
            include_global_resource_types: Some(false),
            recording_strategy: Some(RecordingStrategy::fixture()),
        }
    }
}

/// Specifies the recording strategy of the configuration recorder.
///
/// **AWS API**: `config.v1.RecordingStrategy`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//RecordingStrategy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecordingStrategy {
    /// The recording strategy for the configuration recorder. If you set this option to
    /// ALL_SUPPORTED_RESOURCE_TYPES, Config records configuration changes for all supported
    /// resource types, excluding the global IAM resource types. You also must set the
    /// allSupported field of RecordingGroup to true. When Config adds support for a new
    /// resource type, Config automatically starts recording resources of that type. For a list
    /// of supported resource types, see Supported Resource Types in the Config developer guide.
    /// If you set this option to INCLUSION_BY_RESOURCE_TYPES, Config records configuration
    /// changes for only the resource types that you specify in the resourceTypes field of
    /// RecordingGroup. If you set this option to EXCLUSION_BY_RESOURCE_TYPES, Config records
    /// configuration changes for all supported resource types, except the resource types that
    /// you specify to exclude from being recorded in the resourceTypes field of
    /// ExclusionByResourceTypes. Required and optional fields The recordingStrategy field is
    /// optional when you set the allSupported field of RecordingGroup to true. The
    /// recordingStrategy field is optional when you list resource types in the resourceTypes
    /// field of RecordingGroup. The recordingStrategy field is required if you list resource
    /// types to exclude from recording in the resourceTypes field of ExclusionByResourceTypes.
    /// Overriding fields If you choose EXCLUSION_BY_RESOURCE_TYPES for the recording strategy,
    /// the exclusionByResourceTypes field will override other properties in the request. For
    /// example, even if you set includeGlobalResourceTypes to false, global IAM resource types
    /// will still be automatically recorded in this option unless those resource types are
    /// specifically listed as exclusions in the resourceTypes field of
    /// exclusionByResourceTypes. Global resource types and the exclusion recording strategy By
    /// default, if you choose the EXCLUSION_BY_RESOURCE_TYPES recording strategy, when Config
    /// adds support for a new resource type in the Region where you set up the configuration
    /// recorder, including global resource types, Config starts recording resources of that
    /// type automatically. Unless specifically listed as exclusions, AWS::RDS::GlobalCluster
    /// will be recorded automatically in all supported Config Regions were the configuration
    /// recorder is enabled. IAM users, groups, roles, and customer managed policies will be
    /// recorded in the Region where you set up the configuration recorder if that is a Region
    /// where Config was available before February 2022. You cannot be record the global IAM
    /// resouce types in Regions supported by Config after February 2022. This list where you
    /// cannot record the global IAM resource types includes the following Regions: Asia Pacific
    /// (Hyderabad) Asia Pacific (Melbourne) Canada West (Calgary) Europe (Spain) Europe
    /// (Zurich) Israel (Tel Aviv) Middle East (UAE)
    #[serde(rename = "useOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_only: Option<String>,
}

impl RecordingStrategy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            use_only: Some("test-use_only".into()),
        }
    }
}

/// The output for the DescribeConfigurationRecorderStatus action, in JSON format.
///
/// **AWS API**: `config.v1.DescribeConfigurationRecorderStatusResponse`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//DescribeConfigurationRecorderStatusResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeConfigurationRecorderStatusResponse {
    /// A list that contains status of the specified recorders.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub configuration_recorders_status: Vec<ConfigurationRecorderStatus>,
}

impl DescribeConfigurationRecorderStatusResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            configuration_recorders_status: vec![],
        }
    }
}

/// The current status of the configuration recorder. For a detailed status of recording events
/// over time, add your Config events to CloudWatch metrics and use CloudWatch metrics.
///
/// **AWS API**: `config.v1.ConfigurationRecorderStatus`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//ConfigurationRecorderStatus>
///
/// ## Coverage
/// 6 of 10 fields included.
/// Omitted fields:
/// - `lastStartTime` — not selected in manifest
/// - `lastStopTime` — not selected in manifest
/// - `lastStatusChangeTime` — not selected in manifest
/// - `servicePrincipal` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigurationRecorderStatus {
    /// The Amazon Resource Name (ARN) of the configuration recorder.
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The name of the configuration recorder.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Specifies whether or not the recorder is currently recording.
    #[serde(rename = "recording")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording: Option<bool>,

    /// The status of the latest recording event processed by the recorder.
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<RecorderStatus>,

    /// The latest error code from when the recorder last failed.
    #[serde(rename = "lastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,

    /// The latest error message from when the recorder last failed.
    #[serde(rename = "lastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
}

impl ConfigurationRecorderStatus {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            arn: Some("test-arn".into()),
            name: Some("test-configuration_recorder_status".into()),
            recording: Some(false),
            last_error_code: Some("test-last_error_code".into()),
            last_error_message: Some("test-last_error_message".into()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `config.v1.SelectResourceConfigRequest`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//SelectResourceConfigRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SelectResourceConfigRequest {
    /// The SQL query SELECT command.
    pub expression: String,

    /// The maximum number of query results returned on each page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// The nextToken string returned in a previous request that you use to request the next
    /// page of results in a paginated response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl SelectResourceConfigRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            expression: "test-expression".into(),
            limit: Some(100),
            next_token: Some("test-next_token".into()),
        }
    }
}

/// The input for the DescribeConfigurationRecorders action.
///
/// **AWS API**: `config.v1.DescribeConfigurationRecordersRequest`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//DescribeConfigurationRecordersRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeConfigurationRecordersRequest {
    /// A list of names of the configuration recorders that you want to specify. When making a
    /// request to this operation, you can only specify one configuration recorder.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub configuration_recorder_names: Vec<String>,

    /// For service-linked configuration recorders, you can use the service principal of the
    /// linked Amazon Web Services service to specify the configuration recorder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,

    /// The Amazon Resource Name (ARN) of the configuration recorder that you want to specify.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

impl DescribeConfigurationRecordersRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            configuration_recorder_names: vec![],
            service_principal: Some("test-service_principal".into()),
            arn: Some("test-arn".into()),
        }
    }
}

/// The input for the DescribeConfigurationRecorderStatus action.
///
/// **AWS API**: `config.v1.DescribeConfigurationRecorderStatusRequest`
/// **Reference**: <https://docs.aws.amazon.com/config/latest/APIReference//DescribeConfigurationRecorderStatusRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeConfigurationRecorderStatusRequest {
    /// The name of the configuration recorder. If the name is not specified, the operation
    /// returns the status for the customer managed configuration recorder configured for the
    /// account, if applicable. When making a request to this operation, you can only specify
    /// one configuration recorder.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub configuration_recorder_names: Vec<String>,

    /// For service-linked configuration recorders, you can use the service principal of the
    /// linked Amazon Web Services service to specify the configuration recorder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,

    /// The Amazon Resource Name (ARN) of the configuration recorder that you want to specify.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

impl DescribeConfigurationRecorderStatusRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            configuration_recorder_names: vec![],
            service_principal: Some("test-service_principal".into()),
            arn: Some("test-arn".into()),
        }
    }
}
