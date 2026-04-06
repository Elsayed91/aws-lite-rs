//! AWS Config API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::config::ConfigOps`. This layer adds:
//! - Ergonomic method signatures
//! - Pagination stream helpers

use crate::{
    AwsHttpClient, Result,
    ops::config::ConfigOps,
    types::config::{
        ConfigurationRecorder, ConfigurationRecorderStatus,
        DescribeConfigurationRecorderStatusRequest, DescribeConfigurationRecorderStatusResponse,
        DescribeConfigurationRecordersRequest, DescribeConfigurationRecordersResponse,
        SelectResourceConfigRequest, SelectResourceConfigResponse,
    },
};

/// Client for the AWS Config API
pub struct ConfigClient<'a> {
    ops: ConfigOps<'a>,
}

impl<'a> ConfigClient<'a> {
    /// Create a new AWS Config API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: ConfigOps::new(client),
        }
    }

    /// Accepts a SQL SELECT command and returns matching resource configurations.
    pub async fn select_resource_config(
        &self,
        body: &SelectResourceConfigRequest,
    ) -> Result<SelectResourceConfigResponse> {
        self.ops.select_resource_config(body).await
    }

    // ── Configuration Recorders ────────────────────────────────────────

    /// Return all configuration recorders in the current account/region.
    ///
    /// Optionally filter by recorder name(s). Passing an empty slice returns
    /// all recorders (typically just one named `"default"`).
    ///
    /// CIS 4.3: at least one active configuration recorder must exist in each
    /// region. Call `describe_configuration_recorder_status` to confirm it is
    /// actively recording.
    pub async fn describe_configuration_recorders(
        &self,
        recorder_names: &[&str],
    ) -> Result<DescribeConfigurationRecordersResponse> {
        let body = DescribeConfigurationRecordersRequest {
            configuration_recorder_names: recorder_names.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        };
        self.ops.describe_configuration_recorders(&body).await
    }

    /// Return all configuration recorders as a flat `Vec`.
    pub async fn list_configuration_recorders(&self) -> Result<Vec<ConfigurationRecorder>> {
        let resp = self.describe_configuration_recorders(&[]).await?;
        Ok(resp.configuration_recorders)
    }

    /// Return the recording status for all (or named) configuration recorders.
    ///
    /// CIS 4.3: verify `recording == true` and `last_status` is not `Failure`
    /// to confirm Config is actively recording in the region.
    pub async fn describe_configuration_recorder_status(
        &self,
        recorder_names: &[&str],
    ) -> Result<DescribeConfigurationRecorderStatusResponse> {
        let body = DescribeConfigurationRecorderStatusRequest {
            configuration_recorder_names: recorder_names.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        };
        self.ops.describe_configuration_recorder_status(&body).await
    }

    /// Return the recording status for all recorders as a flat `Vec`.
    pub async fn list_configuration_recorder_statuses(
        &self,
    ) -> Result<Vec<ConfigurationRecorderStatus>> {
        let resp = self.describe_configuration_recorder_status(&[]).await?;
        Ok(resp.configuration_recorders_status)
    }

    /// Stream all results for a SQL SELECT query, automatically handling pagination.
    ///
    /// Each item is a raw JSON string representing a resource configuration.
    pub fn select_resource_config_stream(
        &self,
        expression: &str,
    ) -> impl futures_core::Stream<Item = Result<String>> + '_ {
        let expression = expression.to_string();
        async_stream::try_stream! {
            let mut next_token: Option<String> = None;
            loop {
                let request = SelectResourceConfigRequest {
                    expression: expression.clone(),
                    next_token: next_token.clone(),
                    ..Default::default()
                };
                let response = self.ops.select_resource_config(&request).await?;
                for result in response.results {
                    yield result;
                }
                match response.next_token {
                    Some(token) if !token.is_empty() => next_token = Some(token),
                    _ => break,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_client::MockClient;
    use crate::test_support::config_mock_helpers::ConfigMockHelpers;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn select_resource_config_stream_paginates() {
        let mut mock = MockClient::new();

        mock.expect_post("/")
            .returning_json_sequence(vec![
                serde_json::json!({
                    "Results": ["{\"resourceId\":\"vol-1\",\"resourceType\":\"AWS::EC2::Volume\"}"],
                    "NextToken": "token-page2"
                }),
                serde_json::json!({
                    "Results": ["{\"resourceId\":\"vol-2\",\"resourceType\":\"AWS::EC2::Volume\"}"]
                }),
            ])
            .times(2);

        let client = AwsHttpClient::from_mock(mock);
        let config = client.config();

        let results: Vec<String> = config
            .select_resource_config_stream(
                "SELECT resourceId WHERE resourceType = 'AWS::EC2::Volume'",
            )
            .map(|r| r.unwrap())
            .collect()
            .await;

        assert_eq!(results.len(), 2);
        assert!(results[0].contains("vol-1"));
        assert!(results[1].contains("vol-2"));
    }

    #[tokio::test]
    async fn select_resource_config_stream_single_page() {
        let mut mock = MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({
            "Results": [
                "{\"resourceId\":\"vol-1\"}",
                "{\"resourceId\":\"vol-2\"}"
            ]
        }));

        let client = AwsHttpClient::from_mock(mock);
        let config = client.config();

        let results: Vec<String> = config
            .select_resource_config_stream("SELECT resourceId")
            .map(|r| r.unwrap())
            .collect()
            .await;

        assert_eq!(results.len(), 2);
    }

    // ── Configuration Recorders ────────────────────────────────────────

    #[tokio::test]
    async fn describe_configuration_recorders_returns_recorders() {
        let mut mock = MockClient::new();
        mock.expect_describe_configuration_recorders()
            .returning_json(serde_json::json!({
                "ConfigurationRecorders": [
                    {
                        "name": "default",
                        "roleARN": "arn:aws:iam::123456789012:role/aws-service-role/config.amazonaws.com/AWSServiceRoleForConfig",
                        "recordingGroup": {
                            "allSupported": true,
                            "includeGlobalResourceTypes": true
                        }
                    }
                ]
            }));

        let client = crate::AwsHttpClient::from_mock(mock);
        let resp = client
            .config()
            .describe_configuration_recorders(&[])
            .await
            .unwrap();
        assert_eq!(resp.configuration_recorders.len(), 1);
        let r = &resp.configuration_recorders[0];
        assert_eq!(r.name.as_deref(), Some("default"));
        assert!(
            r.role_arn
                .as_deref()
                .unwrap_or("")
                .contains("AWSServiceRoleForConfig")
        );
        let group = r
            .recording_group
            .as_ref()
            .expect("recording_group should be set");
        assert_eq!(group.all_supported, Some(true));
    }

    #[tokio::test]
    async fn describe_configuration_recorders_handles_empty() {
        let mut mock = MockClient::new();
        mock.expect_describe_configuration_recorders()
            .returning_json(serde_json::json!({"ConfigurationRecorders": []}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let resp = client
            .config()
            .describe_configuration_recorders(&[])
            .await
            .unwrap();
        assert!(resp.configuration_recorders.is_empty());
    }

    #[tokio::test]
    async fn describe_configuration_recorder_status_returns_status() {
        let mut mock = MockClient::new();
        mock.expect_describe_configuration_recorder_status()
            .returning_json(serde_json::json!({
                "ConfigurationRecordersStatus": [
                    {
                        "name": "default",
                        "recording": true,
                        "lastStatus": "Success"
                    }
                ]
            }));

        let client = crate::AwsHttpClient::from_mock(mock);
        let resp = client
            .config()
            .describe_configuration_recorder_status(&[])
            .await
            .unwrap();
        assert_eq!(resp.configuration_recorders_status.len(), 1);
        let s = &resp.configuration_recorders_status[0];
        assert_eq!(s.name.as_deref(), Some("default"));
        assert_eq!(s.recording, Some(true));
        assert_eq!(
            s.last_status,
            Some(crate::types::config::RecorderStatus::Success)
        );
    }

    #[tokio::test]
    async fn describe_configuration_recorder_status_handles_empty() {
        let mut mock = MockClient::new();
        mock.expect_describe_configuration_recorder_status()
            .returning_json(serde_json::json!({"ConfigurationRecordersStatus": []}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let resp = client
            .config()
            .describe_configuration_recorder_status(&[])
            .await
            .unwrap();
        assert!(resp.configuration_recorders_status.is_empty());
    }

    #[tokio::test]
    async fn select_resource_config_stream_empty() {
        let mut mock = MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({
            "Results": []
        }));

        let client = AwsHttpClient::from_mock(mock);
        let config = client.config();

        let results: Vec<String> = config
            .select_resource_config_stream(
                "SELECT resourceId WHERE resourceType = 'AWS::Fake::Thing'",
            )
            .map(|r| r.unwrap())
            .collect()
            .await;

        assert_eq!(results.len(), 0);
    }
}
