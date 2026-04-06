//! Types for the Amazon OpenSearch Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Possible values for `opensearch.ListDomainNamesRequest.EngineType`.
///
/// **AWS API**: `opensearch.ListDomainNamesRequest.EngineType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineType {
    #[serde(rename = "OpenSearch")]
    OpenSearch,

    #[serde(rename = "Elasticsearch")]
    Elasticsearch,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Container for the parameters to the ListDomainNames operation.
///
/// **AWS API**: `opensearch.v1.ListDomainNamesRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDomainNamesRequest {
    /// Filters the output by domain engine type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<EngineType>,
}

impl ListDomainNamesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ..Default::default()
        }
    }
}

/// The results of a ListDomainNames operation. Contains the names of all domains owned by this
/// account and their respective engine types.
///
/// **AWS API**: `opensearch.v1.ListDomainNamesResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDomainNamesResponse {
    /// The names of all OpenSearch Service domains owned by the current user and their
    /// respective engine types.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub domain_names: Vec<DomainInfo>,
}

impl ListDomainNamesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            domain_names: vec![],
        }
    }
}

/// Information about an OpenSearch Service domain.
///
/// **AWS API**: `opensearch.v1.DomainInfo`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainInfo {
    /// Name of the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,

    /// The type of search engine that the domain is running.OpenSearch for an OpenSearch
    /// engine, or Elasticsearch for a legacy Elasticsearch OSS engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<EngineType>,
}

impl DomainInfo {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            domain_name: Some("test-domain_name".into()),
            ..Default::default()
        }
    }
}

/// Container for the parameters to the DescribeDomain operation.
///
/// **AWS API**: `opensearch.v1.DescribeDomainRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDomainRequest {
    /// The name of the domain that you want information about.
    pub domain_name: String,
}

impl DescribeDomainRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            domain_name: "test-domain_name".into(),
        }
    }
}

/// Contains the status of the domain specified in the request.
///
/// **AWS API**: `opensearch.v1.DescribeDomainResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDomainResponse {
    /// List that contains the status of each specified OpenSearch Service domain.
    pub domain_status: DomainStatus,
}

impl DescribeDomainResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            domain_status: DomainStatus::fixture(),
        }
    }
}

/// The current status of an OpenSearch Service domain.
///
/// **AWS API**: `opensearch.v1.DomainStatus`
///
/// ## Coverage
/// 9 of 34 fields included.
/// Omitted fields:
/// - `Endpoints` — not selected in manifest
/// - `DomainEndpointV2HostedZoneId` — not selected in manifest
/// - `UpgradeProcessing` — not selected in manifest
/// - `ClusterConfig` — not selected in manifest
/// - `EBSOptions` — not selected in manifest
/// - `AccessPolicies` — not selected in manifest
/// - `IPAddressType` — not selected in manifest
/// - `SnapshotOptions` — not selected in manifest
/// - `VPCOptions` — not selected in manifest
/// - `CognitoOptions` — not selected in manifest
/// - `EncryptionAtRestOptions` — not selected in manifest
/// - `NodeToNodeEncryptionOptions` — not selected in manifest
/// - `AdvancedOptions` — not selected in manifest
/// - `LogPublishingOptions` — not selected in manifest
/// - `ServiceSoftwareOptions` — not selected in manifest
/// - `DomainEndpointOptions` — not selected in manifest
/// - `AdvancedSecurityOptions` — not selected in manifest
/// - `IdentityCenterOptions` — not selected in manifest
/// - `AutoTuneOptions` — not selected in manifest
/// - `ChangeProgressDetails` — not selected in manifest
/// - `OffPeakWindowOptions` — not selected in manifest
/// - `SoftwareUpdateOptions` — not selected in manifest
/// - `DomainProcessingStatus` — not selected in manifest
/// - `ModifyingProperties` — not selected in manifest
/// - `AIMLOptions` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainStatus {
    /// Unique identifier for the domain.
    pub domain_id: String,

    /// Name of the domain. Domain names are unique across all domains owned by the same account
    /// within an Amazon Web Services Region.
    pub domain_name: String,

    /// The Amazon Resource Name (ARN) of the domain. For more information, see IAM identifiers
    /// in the Amazon Web Services Identity and Access Management User Guide.
    #[serde(rename = "ARN")]
    pub arn: String,

    /// Creation status of an OpenSearch Service domain. True if domain creation is complete.
    /// False if domain creation is still in progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,

    /// Deletion status of an OpenSearch Service domain. True if domain deletion is complete.
    /// False if domain deletion is still in progress. Once deletion is complete, the status of
    /// the domain is no longer returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// Domain-specific endpoint used to submit index, search, and data upload requests to the
    /// domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    /// If IPAddressType to set to dualstack, a version 2 domain endpoint is provisioned. This
    /// endpoint functions like a normal endpoint, except that it works with both IPv4 and IPv6
    /// IP addresses. Normal endpoints work only with IPv4 IP addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_v2: Option<String>,

    /// The status of the domain configuration. True if OpenSearch Service is processing
    /// configuration changes. False if the configuration is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<bool>,

    /// Version of OpenSearch or Elasticsearch that the domain is running, in the format
    /// Elasticsearch_X.Y or OpenSearch_X.Y.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
}

impl DomainStatus {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            domain_id: "test-domain_id".into(),
            domain_name: "test-domain_name".into(),
            arn: "test-arn".into(),
            created: Some(false),
            deleted: Some(false),
            endpoint: Some("test-endpoint".into()),
            endpoint_v2: Some("test-endpoint_v2".into()),
            processing: Some(false),
            engine_version: Some("test-engine_version".into()),
        }
    }
}

/// Container for the parameters to the DeleteDomain operation.
///
/// **AWS API**: `opensearch.v1.DeleteDomainRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDomainRequest {
    /// The name of the domain you want to permanently delete.
    pub domain_name: String,
}

impl DeleteDomainRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            domain_name: "test-domain_name".into(),
        }
    }
}

/// The results of a DeleteDomain request. Contains the status of the pending deletion, or a
/// "domain not found" error if the domain and all of its resources have been deleted.
///
/// **AWS API**: `opensearch.v1.DeleteDomainResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDomainResponse {
    /// The status of the domain being deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<DomainStatus>,
}

impl DeleteDomainResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            domain_status: Some(DomainStatus::fixture()),
        }
    }
}
