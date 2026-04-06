//! Operation contracts for the AWS Config API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/config.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::config::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the AWS Config API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::config::ConfigClient`] instead.
pub struct ConfigOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> ConfigOps<'a> {
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> String {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/').to_string();
            }
        }
        "https://config.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Accepts a SQL SELECT command and returns matching resource configurations.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`SelectResourceConfigRequest`]
    ///
    /// # Response
    /// [`SelectResourceConfigResponse`]
    #[allow(dead_code)]
    pub(crate) async fn select_resource_config(
        &self,
        body: &SelectResourceConfigRequest,
    ) -> Result<SelectResourceConfigResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize select_resource_config request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "config",
                "StarlingDoveService.SelectResourceConfig",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read select_resource_config response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse select_resource_config response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Returns the details for the specified configuration recorders.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeConfigurationRecordersRequest`]
    ///
    /// # Response
    /// [`DescribeConfigurationRecordersResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_configuration_recorders(
        &self,
        body: &DescribeConfigurationRecordersRequest,
    ) -> Result<DescribeConfigurationRecordersResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to serialize describe_configuration_recorders request: {e}"
                ),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "config",
                "StarlingDoveService.DescribeConfigurationRecorders",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read describe_configuration_recorders response: {e}"
                    ),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_configuration_recorders response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Returns the current status of the specified configuration recorder.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeConfigurationRecorderStatusRequest`]
    ///
    /// # Response
    /// [`DescribeConfigurationRecorderStatusResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_configuration_recorder_status(
        &self,
        body: &DescribeConfigurationRecorderStatusRequest,
    ) -> Result<DescribeConfigurationRecorderStatusResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to serialize describe_configuration_recorder_status request: {e}"
                ),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "config",
                "StarlingDoveService.DescribeConfigurationRecorderStatus",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read describe_configuration_recorder_status response: {e}"
                    ),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!(
                "Failed to parse describe_configuration_recorder_status response: {e}"
            ),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_select_resource_config() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(SelectResourceConfigResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ConfigOps::new(&client);

        let body = SelectResourceConfigRequest::fixture();
        let result = ops.select_resource_config(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_configuration_recorders() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(
            serde_json::to_value(DescribeConfigurationRecordersResponse::fixture()).unwrap(),
        );

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ConfigOps::new(&client);

        let body = DescribeConfigurationRecordersRequest::fixture();
        let result = ops.describe_configuration_recorders(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_configuration_recorder_status() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(
            serde_json::to_value(DescribeConfigurationRecorderStatusResponse::fixture()).unwrap(),
        );

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ConfigOps::new(&client);

        let body = DescribeConfigurationRecorderStatusRequest::fixture();
        let result = ops.describe_configuration_recorder_status(&body).await;
        assert!(result.is_ok());
    }
}
