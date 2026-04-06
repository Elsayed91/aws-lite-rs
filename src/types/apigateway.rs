//! Types for the Amazon API Gateway API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The GET request to list existing RestApis defined for your collection.
///
/// **AWS API**: `apigateway.v1.GetRestApisRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRestApisRequest {
    /// The current pagination position in the paged result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,

    /// The maximum number of returned results per page. The default value is 25 and the maximum
    /// value is 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl GetRestApisRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            position: Some("test-position".into()),
            limit: Some(100),
        }
    }
}

/// Contains references to your APIs and links that guide you in how to interact with your
/// collection. A collection offers a paginated view of your APIs.
///
/// **AWS API**: `apigateway.v1.RestApis`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestApis {
    /// The `position` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,

    /// The current page of elements from this collection.
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RestApi>,
}

impl RestApis {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            position: Some("test-position".into()),
            items: vec![],
        }
    }
}

/// Represents a REST API.
///
/// **AWS API**: `apigateway.v1.RestApi`
///
/// ## Coverage
/// 3 of 18 fields included.
/// Omitted fields:
/// - `createdDate` — not selected in manifest
/// - `version` — not selected in manifest
/// - `warnings` — not selected in manifest
/// - `binaryMediaTypes` — not selected in manifest
/// - `minimumCompressionSize` — not selected in manifest
/// - `apiKeySource` — not selected in manifest
/// - `endpointConfiguration` — not selected in manifest
/// - `policy` — not selected in manifest
/// - `tags` — not selected in manifest
/// - `disableExecuteApiEndpoint` — not selected in manifest
/// - `rootResourceId` — not selected in manifest
/// - `securityPolicy` — not selected in manifest
/// - `endpointAccessMode` — not selected in manifest
/// - `apiStatus` — not selected in manifest
/// - `apiStatusMessage` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestApi {
    /// The API's identifier. This identifier is unique across all of your APIs in API Gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The API's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The API's description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl RestApi {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-rest_api".into()),
            description: Some("test-description".into()),
        }
    }
}

/// Requests API Gateway to get information about one or more Stage resources.
///
/// **AWS API**: `apigateway.v1.GetStagesRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStagesRequest {
    /// The string identifier of the associated RestApi.
    pub rest_api_id: String,

    /// The stages' deployment identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

impl GetStagesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            rest_api_id: "test-rest_api_id".into(),
            deployment_id: Some("test-deployment_id".into()),
        }
    }
}

/// A list of Stage resources that are associated with the ApiKey resource.
///
/// **AWS API**: `apigateway.v1.Stages`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stages {
    /// The current page of elements from this collection.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<Stage>,
}

impl Stages {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { item: vec![] }
    }
}

/// Represents a unique identifier for a version of a deployed RestApi that is callable by
/// users.
///
/// **AWS API**: `apigateway.v1.Stage`
///
/// ## Coverage
/// 8 of 17 fields included.
/// Omitted fields:
/// - `clientCertificateId` — not selected in manifest
/// - `variables` — not selected in manifest
/// - `documentationVersion` — not selected in manifest
/// - `accessLogSettings` — not selected in manifest
/// - `canarySettings` — not selected in manifest
/// - `webAclArn` — not selected in manifest
/// - `tags` — not selected in manifest
/// - `createdDate` — not selected in manifest
/// - `lastUpdatedDate` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    /// The name of the stage is the first path segment in the Uniform Resource Identifier (URI)
    /// of a call to API Gateway. Stage names can only contain alphanumeric characters, hyphens,
    /// and underscores. Maximum length is 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,

    /// The identifier of the Deployment that the stage points to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,

    /// The stage's description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Specifies whether a cache cluster is enabled for the stage. To activate a method-level
    /// cache, set CachingEnabled to true for a method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,

    /// The stage's cache capacity in GB. For more information about choosing a cache size, see
    /// Enabling API caching to enhance responsiveness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,

    /// The status of the cache cluster for the stage, if enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_status: Option<String>,

    /// A map that defines the method settings for a Stage resource. Keys (designated as
    /// /{method_setting_key below) are method paths defined as {resource_path}/{http_method}
    /// for an individual method override, or /\*/\* for overriding all methods in the stage.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub method_settings: std::collections::HashMap<String, MethodSetting>,

    /// Specifies whether active tracing with X-ray is enabled for the Stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_enabled: Option<bool>,
}

impl Stage {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stage_name: Some("test-stage_name".into()),
            deployment_id: Some("test-deployment_id".into()),
            description: Some("test-description".into()),
            cache_cluster_enabled: Some(false),
            cache_cluster_size: Some("test-cache_cluster_size".into()),
            cache_cluster_status: Some("test-cache_cluster_status".into()),
            tracing_enabled: Some(false),
            ..Default::default()
        }
    }
}

/// Specifies the method setting properties.
///
/// **AWS API**: `apigateway.v1.MethodSetting`
///
/// ## Coverage
/// 8 of 10 fields included.
/// Omitted fields:
/// - `requireAuthorizationForCacheControl` — not selected in manifest
/// - `unauthorizedCacheControlHeaderStrategy` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MethodSetting {
    /// Specifies whether Amazon CloudWatch metrics are enabled for this method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_enabled: Option<bool>,

    /// Specifies the logging level for this method, which affects the log entries pushed to
    /// Amazon CloudWatch Logs. Valid values are OFF, ERROR, and INFO. Choose ERROR to write
    /// only error-level entries to CloudWatch Logs, or choose INFO to include all ERROR events
    /// as well as extra informational events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,

    /// Specifies whether data trace logging is enabled for this method, which affects the log
    /// entries pushed to Amazon CloudWatch Logs. This can be useful to troubleshoot APIs, but
    /// can result in logging sensitive data. We recommend that you don't enable this option for
    /// production APIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,

    /// Specifies the throttling burst limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i32>,

    /// Specifies the throttling rate limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,

    /// Specifies whether responses should be cached and returned for requests. A cache cluster
    /// must be enabled on the stage for responses to be cached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_enabled: Option<bool>,

    /// Specifies the time to live (TTL), in seconds, for cached responses. The higher the TTL,
    /// the longer the response will be cached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl_in_seconds: Option<i32>,

    /// Specifies whether the cached responses are encrypted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_data_encrypted: Option<bool>,
}

impl MethodSetting {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metrics_enabled: Some(false),
            logging_level: Some("test-logging_level".into()),
            data_trace_enabled: Some(false),
            throttling_burst_limit: Some(100),
            caching_enabled: Some(false),
            cache_ttl_in_seconds: Some(100),
            cache_data_encrypted: Some(false),
            ..Default::default()
        }
    }
}

/// Requests API Gateway to change information about a Stage resource.
///
/// **AWS API**: `apigateway.v1.UpdateStageRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStageRequest {
    /// The string identifier of the associated RestApi.
    pub rest_api_id: String,

    /// The name of the Stage resource to change information about.
    pub stage_name: String,

    /// For more information about supported patch operations, see Patch Operations.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub patch_operations: Vec<PatchOperation>,
}

impl UpdateStageRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            rest_api_id: "test-rest_api_id".into(),
            stage_name: "test-stage_name".into(),
            patch_operations: vec![],
        }
    }
}

/// For more information about supported patch operations, see Patch Operations.
///
/// **AWS API**: `apigateway.v1.PatchOperation`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchOperation {
    /// An update operation to be performed with this PATCH request. The valid value can be add,
    /// remove, replace or copy. Not all valid operations are supported for a given resource.
    /// Support of the operations depends on specific operational contexts. Attempts to apply an
    /// unsupported operation on a resource will return an error message..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,

    /// The op operation's target, as identified by a JSON Pointer value that references a
    /// location within the targeted resource. For example, if the target resource has an
    /// updateable property of {"name":"value"}, the path for this property is /name. If the
    /// name property value is a JSON object (e.g., {"name": {"child/name": "child-value"}}),
    /// the path for the child/name property will be /name/child~1name. Any slash ("/")
    /// character appearing in path names must be escaped with "~1", as shown in the example
    /// above. Each op operation can have only one path associated with it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// The new target value of the update operation. It is applicable for the add or replace
    /// operation. When using AWS CLI to update a property of a JSON value, enclose the JSON
    /// object with a pair of single quotes in a Linux shell, e.g., '{"a": ...}'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The copy update operation's source as identified by a JSON-Pointer value referencing the
    /// location within the targeted resource to copy the value from. For example, to promote a
    /// canary deployment, you copy the canary deployment ID to the affiliated deployment ID by
    /// calling a PATCH request on a Stage resource with "op":"copy",
    /// "from":"/canarySettings/deploymentId" and "path":"/deploymentId".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

impl PatchOperation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            op: Some("test-op".into()),
            path: Some("test-path".into()),
            value: Some("test-value".into()),
            from: Some("test-from".into()),
        }
    }
}
