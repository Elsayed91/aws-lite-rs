//! Types for the AWS Lambda API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///
/// **AWS API**: `lambda.v1.ListFunctionsRequest`
///
/// ## Coverage
/// 2 of 4 fields included.
/// Omitted fields:
/// - `MasterRegion` — not selected in manifest
/// - `FunctionVersion` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListFunctionsRequest {
    /// Specify the pagination token that's returned by a previous request to retrieve the next
    /// page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// The maximum number of functions to return in the response. Note that ListFunctions
    /// returns a maximum of 50 items in each response, even if you set the number higher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListFunctionsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

/// A list of Lambda functions.
///
/// **AWS API**: `lambda.v1.ListFunctionsResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListFunctionsResponse {
    /// A list of Lambda functions.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<FunctionConfiguration>,

    /// The pagination token that's included if more results are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

impl ListFunctionsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            functions: vec![],
            next_marker: Some("test-next_marker".into()),
        }
    }
}

/// Details about a function's configuration.
///
/// **AWS API**: `lambda.v1.FunctionConfiguration`
///
/// ## Coverage
/// 12 of 40 fields included.
/// Omitted fields:
/// - `Role` — not selected in manifest
/// - `CodeSha256` — not selected in manifest
/// - `Version` — not selected in manifest
/// - `DeadLetterConfig` — not selected in manifest
/// - `KMSKeyArn` — not selected in manifest
/// - `TracingConfig` — not selected in manifest
/// - `MasterArn` — not selected in manifest
/// - `RevisionId` — not selected in manifest
/// - `Layers` — not selected in manifest
/// - `State` — not selected in manifest
/// - `StateReason` — not selected in manifest
/// - `StateReasonCode` — not selected in manifest
/// - `LastUpdateStatus` — not selected in manifest
/// - `LastUpdateStatusReason` — not selected in manifest
/// - `LastUpdateStatusReasonCode` — not selected in manifest
/// - `FileSystemConfigs` — not selected in manifest
/// - `PackageType` — not selected in manifest
/// - `ImageConfigResponse` — not selected in manifest
/// - `SigningProfileVersionArn` — not selected in manifest
/// - `SigningJobArn` — not selected in manifest
/// - `EphemeralStorage` — not selected in manifest
/// - `SnapStart` — not selected in manifest
/// - `RuntimeVersionConfig` — not selected in manifest
/// - `LoggingConfig` — not selected in manifest
/// - `CapacityProviderConfig` — not selected in manifest
/// - `ConfigSha256` — not selected in manifest
/// - `DurableConfig` — not selected in manifest
/// - `TenancyConfig` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionConfiguration {
    /// The name of the function.
    pub function_name: String,

    /// The function's Amazon Resource Name (ARN).
    pub function_arn: String,

    /// The identifier of the function's runtime. Runtime is required if the deployment package
    /// is a .zip file archive. Specifying a runtime results in an error if you're deploying a
    /// function using a container image. The following list includes deprecated runtimes.
    /// Lambda blocks creating new functions and updating existing functions shortly after each
    /// runtime is deprecated. For more information, see Runtime use after deprecation. For a
    /// list of all currently supported runtimes, see Supported runtimes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,

    /// The function that Lambda calls to begin running your function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,

    /// The size of the function's deployment package, in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,

    /// The function's description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The amount of time in seconds that Lambda allows a function to run before stopping it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,

    /// The amount of memory available to the function at runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i32>,

    /// The date and time that the function was last updated, in ISO-8601 format (YYYY-MM-
    /// DDThh:mm:ss.sTZD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,

    /// The function's environment variables. Omitted from CloudTrail logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Option<EnvironmentResponse>>,

    /// The function's networking configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<Option<VpcConfigResponse>>,

    /// The instruction set architecture that the function supports. Architecture is a string
    /// array with one of the valid values. The default architecture value is x86_64.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub architectures: Vec<String>,
}

impl FunctionConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            function_name: "test-function_name".into(),
            function_arn: "test-function_arn".into(),
            runtime: Some("test-runtime".into()),
            handler: Some("test-handler".into()),
            code_size: Some(100),
            description: Some("test-description".into()),
            timeout: Some(100),
            memory_size: Some(100),
            last_modified: Some("test-last_modified".into()),
            architectures: vec![],
            ..Default::default()
        }
    }
}

/// The results of an operation to update or read environment variables. If the operation
/// succeeds, the response contains the environment variables. If it fails, the response
/// contains details about the error.
///
/// **AWS API**: `lambda.v1.EnvironmentResponse`
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `Error` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnvironmentResponse {
    /// Environment variable key-value pairs. Omitted from CloudTrail logs.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub variables: std::collections::HashMap<String, String>,
}

impl EnvironmentResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ..Default::default()
        }
    }
}

/// The VPC security groups and subnets that are attached to a Lambda function.
///
/// **AWS API**: `lambda.v1.VpcConfigResponse`
///
/// ## Coverage
/// 3 of 4 fields included.
/// Omitted fields:
/// - `Ipv6AllowedForDualStack` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VpcConfigResponse {
    /// A list of VPC subnet IDs.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subnet_ids: Vec<String>,

    /// A list of VPC security group IDs.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_group_ids: Vec<String>,

    /// The ID of the VPC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl VpcConfigResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            subnet_ids: vec![],
            security_group_ids: vec![],
            vpc_id: Some("test-vpc_id".into()),
        }
    }
}

///
/// **AWS API**: `lambda.v1.GetFunctionConfigurationRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetFunctionConfigurationRequest {
    /// The name or ARN of the Lambda function, version, or alias. Name formats Function name –
    /// my-function (name-only), my-function:v1 (with alias). Function ARN – arn:aws:lambda:us-
    /// west-2:123456789012:function:my-function. Partial ARN – 123456789012:function:my-
    /// function. You can append a version number or alias to any of the formats. The length
    /// constraint applies only to the full ARN. If you specify only the function name, it is
    /// limited to 64 characters in length.
    pub function_name: String,

    /// Specify a version or alias to get details about a published version of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

impl GetFunctionConfigurationRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            function_name: "test-function_name".into(),
            qualifier: Some("test-qualifier".into()),
        }
    }
}

///
/// **AWS API**: `lambda.v1.UpdateFunctionConfigurationRequest`
///
/// ## Coverage
/// 4 of 21 fields included.
/// Omitted fields:
/// - `Role` — not selected in manifest
/// - `Handler` — not selected in manifest
/// - `Description` — not selected in manifest
/// - `VpcConfig` — not selected in manifest
/// - `Environment` — not selected in manifest
/// - `DeadLetterConfig` — not selected in manifest
/// - `KMSKeyArn` — not selected in manifest
/// - `TracingConfig` — not selected in manifest
/// - `RevisionId` — not selected in manifest
/// - `Layers` — not selected in manifest
/// - `FileSystemConfigs` — not selected in manifest
/// - `ImageConfig` — not selected in manifest
/// - `EphemeralStorage` — not selected in manifest
/// - `SnapStart` — not selected in manifest
/// - `LoggingConfig` — not selected in manifest
/// - `CapacityProviderConfig` — not selected in manifest
/// - `DurableConfig` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateFunctionConfigurationRequest {
    /// The name or ARN of the Lambda function. Name formats Function name – my-function.
    /// Function ARN – arn:aws:lambda:us-west-2:123456789012:function:my-function. Partial ARN –
    /// 123456789012:function:my-function. The length constraint applies only to the full ARN.
    /// If you specify only the function name, it is limited to 64 characters in length.
    pub function_name: String,

    /// The amount of time (in seconds) that Lambda allows a function to run before stopping it.
    /// The default is 3 seconds. The maximum allowed value is 900 seconds. For more
    /// information, see Lambda execution environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,

    /// The amount of memory available to the function at runtime. Increasing the function
    /// memory also increases its CPU allocation. The default value is 128 MB. The value can be
    /// any multiple of 1 MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i32>,

    /// The identifier of the function's runtime. Runtime is required if the deployment package
    /// is a .zip file archive. Specifying a runtime results in an error if you're deploying a
    /// function using a container image. The following list includes deprecated runtimes.
    /// Lambda blocks creating new functions and updating existing functions shortly after each
    /// runtime is deprecated. For more information, see Runtime use after deprecation. For a
    /// list of all currently supported runtimes, see Supported runtimes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
}

impl UpdateFunctionConfigurationRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            function_name: "test-function_name".into(),
            timeout: Some(100),
            memory_size: Some(100),
            runtime: Some("test-runtime".into()),
        }
    }
}
