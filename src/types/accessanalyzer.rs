//! Types for the AWS IAM Access Analyzer API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Retrieves a list of analyzers.
///
/// **AWS API**: `accessanalyzer.v1.ListAnalyzersRequest`
/// **Reference**: <https://docs.aws.amazon.com/access-analyzer/latest/APIReference//ListAnalyzersRequest>
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `type` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAnalyzersRequest {
    /// A token used for pagination of results returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of results to return in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl ListAnalyzersRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

/// The response to the request.
///
/// **AWS API**: `accessanalyzer.v1.ListAnalyzersResponse`
/// **Reference**: <https://docs.aws.amazon.com/access-analyzer/latest/APIReference//ListAnalyzersResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAnalyzersResponse {
    /// The analyzers retrieved.
    #[serde(default)]
    pub analyzers: Vec<AnalyzerSummary>,

    /// A token used for pagination of results returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListAnalyzersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            analyzers: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Contains information about the analyzer.
///
/// **AWS API**: `accessanalyzer.v1.AnalyzerSummary`
/// **Reference**: <https://docs.aws.amazon.com/access-analyzer/latest/APIReference//AnalyzerSummary>
///
/// ## Coverage
/// 7 of 10 fields included.
/// Omitted fields:
/// - `tags` — not selected in manifest
/// - `statusReason` — not selected in manifest
/// - `configuration` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzerSummary {
    /// The ARN of the analyzer.
    pub arn: String,

    /// The name of the analyzer.
    pub name: String,

    /// The type represents the zone of trust or scope for the analyzer.
    #[serde(rename = "type")]
    pub r#type: String,

    /// The status of the analyzer. An Active analyzer successfully monitors supported resources
    /// and generates new findings. The analyzer is Disabled when a user action, such as
    /// removing trusted access for Identity and Access Management Access Analyzer from
    /// Organizations, causes the analyzer to stop generating new findings. The status is
    /// Creating when the analyzer creation is in progress and Failed when the analyzer creation
    /// has failed.
    pub status: String,

    /// A timestamp for the time at which the analyzer was created.
    pub created_at: String,

    /// The time at which the most recently analyzed resource was analyzed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resource_analyzed_at: Option<String>,

    /// The resource that was most recently analyzed by the analyzer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resource_analyzed: Option<String>,
}

impl AnalyzerSummary {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            arn: "test-arn".into(),
            name: "test-analyzer_summary".into(),
            r#type: "test-type".into(),
            status: "test-status".into(),
            created_at: "test-created_at".into(),
            last_resource_analyzed_at: Some("test-last_resource_analyzed_at".into()),
            last_resource_analyzed: Some("test-last_resource_analyzed".into()),
        }
    }
}
