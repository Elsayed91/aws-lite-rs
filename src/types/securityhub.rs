//! Types for the AWS Security Hub API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `securityhub.v1.DescribeHubResponse`
/// **Reference**: <https://docs.aws.amazon.com/securityhub/1.0/APIReference//DescribeHubResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeHubResponse {
    /// The ARN of the Hub resource that was retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_arn: Option<String>,

    /// The date and time when Security Hub CSPM was enabled in the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_at: Option<String>,

    /// Whether to automatically enable new controls when they are added to standards that are
    /// enabled. If set to true, then new controls for enabled standards are enabled
    /// automatically. If set to false, then new controls are not enabled. When you
    /// automatically enable new controls, you can interact with the controls in the console and
    /// programmatically immediately after release. However, automatically enabled controls have
    /// a temporary default status of DISABLED. It can take up to several days for Security Hub
    /// CSPM to process the control release and designate the control as ENABLED in your
    /// account. During the processing period, you can manually enable or disable a control, and
    /// Security Hub CSPM will maintain that designation regardless of whether you have
    /// AutoEnableControls set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_controls: Option<bool>,

    /// Specifies whether the calling account has consolidated control findings turned on. If
    /// the value for this field is set to SECURITY_CONTROL, Security Hub CSPM generates a
    /// single finding for a control check even when the check applies to multiple enabled
    /// standards. If the value for this field is set to STANDARD_CONTROL, Security Hub CSPM
    /// generates separate findings for a control check when the check applies to multiple
    /// enabled standards. The value for this field in a member account matches the value in the
    /// administrator account. For accounts that aren't part of an organization, the default
    /// value of this field is SECURITY_CONTROL if you enabled Security Hub CSPM on or after
    /// February 23, 2023.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_finding_generator: Option<String>,
}

impl DescribeHubResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            hub_arn: Some("test-hub_arn".into()),
            subscribed_at: Some("test-subscribed_at".into()),
            auto_enable_controls: Some(false),
            control_finding_generator: Some("test-control_finding_generator".into()),
        }
    }
}

///
/// **AWS API**: `securityhub.v1.DescribeHubRequest`
/// **Reference**: <https://docs.aws.amazon.com/securityhub/1.0/APIReference//DescribeHubRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeHubRequest {
    /// The ARN of the Hub resource to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_arn: Option<String>,
}

impl DescribeHubRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            hub_arn: Some("test-hub_arn".into()),
        }
    }
}
