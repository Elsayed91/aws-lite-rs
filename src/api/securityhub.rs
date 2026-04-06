//! AWS Security Hub API client.
//!
//! Thin wrapper over generated ops.
//!
//! Needed by AWS CIS benchmark checks:
//!   - CIS 5.16 (aws_security_hub_enabled): verify Security Hub is enabled
//!     per account/region. `describe_hub()` succeeds → enabled;
//!     `AwsError::NotFound` → not enabled.

use crate::{
    AwsError, AwsHttpClient, Result, ops::securityhub::SecurityhubOps,
    types::securityhub::DescribeHubResponse,
};

/// Client for the AWS Security Hub API.
pub struct SecurityHubClient<'a> {
    ops: SecurityhubOps<'a>,
}

impl<'a> SecurityHubClient<'a> {
    /// Create a new Security Hub client.
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: SecurityhubOps::new(client),
        }
    }

    // ── Hub ───────────────────────────────────────────────────────────

    /// Return details about the Security Hub resource in the current account/region.
    ///
    /// Returns `Err(AwsError::NotFound { .. })` if Security Hub is not enabled.
    ///
    /// CIS 5.16: Security Hub must be enabled in every active region.
    pub async fn describe_hub(&self) -> Result<DescribeHubResponse> {
        self.ops.describe_hub("").await
    }

    /// Return `true` if Security Hub is enabled in the current account/region.
    ///
    /// Treats both 404 (`ResourceNotFoundException`) and 403
    /// (`InvalidAccessException` — "not subscribed") as "not enabled".
    ///
    /// CIS 5.16: all active regions should have Security Hub enabled.
    pub async fn is_enabled(&self) -> Result<bool> {
        match self.ops.describe_hub("").await {
            Ok(_) => Ok(true),
            Err(AwsError::NotFound { .. }) => Ok(false),
            // InvalidAccessException (403): "Account is not subscribed to Security Hub"
            Err(AwsError::ServiceError { status: 403, .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::AwsError;
    use serde_json::json;

    #[tokio::test]
    async fn test_describe_hub_returns_hub_details() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/accounts")
            .returning_json(json!({
                "HubArn": "arn:aws:securityhub:us-east-1:123456789012:hub/default",
                "SubscribedAt": "2024-01-15T10:00:00.000Z",
                "AutoEnableControls": true,
                "ControlFindingGenerator": "SECURITY_CONTROL"
            }))
            .times(1);

        let client = crate::AwsHttpClient::from_mock(mock);
        let sh = client.securityhub();
        let hub = sh.describe_hub().await.unwrap();
        assert_eq!(
            hub.hub_arn.as_deref(),
            Some("arn:aws:securityhub:us-east-1:123456789012:hub/default")
        );
        assert_eq!(hub.auto_enable_controls, Some(true));
        assert_eq!(
            hub.control_finding_generator.as_deref(),
            Some("SECURITY_CONTROL")
        );
    }

    #[tokio::test]
    async fn test_is_enabled_true() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/accounts")
            .returning_json(json!({
                "HubArn": "arn:aws:securityhub:us-east-1:123456789012:hub/default",
                "SubscribedAt": "2024-01-15T10:00:00.000Z"
            }))
            .times(1);

        let client = crate::AwsHttpClient::from_mock(mock);
        assert!(client.securityhub().is_enabled().await.unwrap());
    }

    #[tokio::test]
    async fn test_is_enabled_false_when_not_found() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/accounts")
            .returning_error(AwsError::NotFound {
                resource: "Account is not subscribed to AWS Security Hub".into(),
            })
            .times(1);

        let client = crate::AwsHttpClient::from_mock(mock);
        assert!(!client.securityhub().is_enabled().await.unwrap());
    }

    #[tokio::test]
    async fn test_is_enabled_false_when_invalid_access() {
        // InvalidAccessException (403) = "Account is not subscribed" — real-world case
        let mut mock = crate::MockClient::new();
        mock.expect_get("/accounts")
            .returning_error(AwsError::ServiceError {
                code: "HttpError403".into(),
                message: "The AWS Access Key Id needs a subscription for the service".into(),
                status: 403,
            })
            .times(1);

        let client = crate::AwsHttpClient::from_mock(mock);
        assert!(!client.securityhub().is_enabled().await.unwrap());
    }
}
