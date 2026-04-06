//! Operation contracts for the Amazon CloudWatch Logs API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/logs.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::logs::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon CloudWatch Logs API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::logs::LogsClient`] instead.
pub struct LogsOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> LogsOps<'a> {
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
        "https://logs.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Lists the specified log groups.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeLogGroupsRequest`]
    ///
    /// # Response
    /// [`DescribeLogGroupsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_log_groups(
        &self,
        body: &DescribeLogGroupsRequest,
    ) -> Result<DescribeLogGroupsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_log_groups request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "logs",
                "Logs_20140328.DescribeLogGroups",
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
                    message: format!("Failed to read describe_log_groups response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_log_groups response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Displays the tags associated with a CloudWatch Logs resource.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListTagsForResourceRequest`]
    ///
    /// # Response
    /// [`ListTagsForResourceResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_tags_for_resource(
        &self,
        body: &ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize list_tags_for_resource request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "logs",
                "Logs_20140328.ListTagsForResource",
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
                    message: format!("Failed to read list_tags_for_resource response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_tags_for_resource response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists the log streams for the specified log group.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeLogStreamsRequest`]
    ///
    /// # Response
    /// [`DescribeLogStreamsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_log_streams(
        &self,
        body: &DescribeLogStreamsRequest,
    ) -> Result<DescribeLogStreamsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_log_streams request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "logs",
                "Logs_20140328.DescribeLogStreams",
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
                    message: format!("Failed to read describe_log_streams response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_log_streams response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Sets the retention of the specified log group.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`PutRetentionPolicyRequest`]
    #[allow(dead_code)]
    pub(crate) async fn put_retention_policy(
        &self,
        body: &PutRetentionPolicyRequest,
    ) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize put_retention_policy request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "logs",
                "Logs_20140328.PutRetentionPolicy",
                "1.1",
                &body_bytes,
            )
            .await?;
        response.error_for_status("json").await?;
        Ok(())
    }

    /// Deletes the specified log stream.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteLogStreamRequest`]
    #[allow(dead_code)]
    pub(crate) async fn delete_log_stream(&self, body: &DeleteLogStreamRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize delete_log_stream request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "logs",
                "Logs_20140328.DeleteLogStream",
                "1.1",
                &body_bytes,
            )
            .await?;
        response.error_for_status("json").await?;
        Ok(())
    }

    /// Lists the specified metric filters. You can list all of the metric filters or filter the
    /// results by log name, prefix, metric name, namespace, and dimensions.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeMetricFiltersRequest`]
    ///
    /// # Response
    /// [`DescribeMetricFiltersResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_metric_filters(
        &self,
        body: &DescribeMetricFiltersRequest,
    ) -> Result<DescribeMetricFiltersResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_metric_filters request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "logs",
                "Logs_20140328.DescribeMetricFilters",
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
                    message: format!("Failed to read describe_metric_filters response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_metric_filters response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_log_groups() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeLogGroupsResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = LogsOps::new(&client);

        let body = DescribeLogGroupsRequest::fixture();
        let result = ops.describe_log_groups(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_tags_for_resource() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(ListTagsForResourceResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = LogsOps::new(&client);

        let body = ListTagsForResourceRequest::fixture();
        let result = ops.list_tags_for_resource(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_log_streams() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeLogStreamsResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = LogsOps::new(&client);

        let body = DescribeLogStreamsRequest::fixture();
        let result = ops.describe_log_streams(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_retention_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = LogsOps::new(&client);

        let body = PutRetentionPolicyRequest::fixture();
        let result = ops.put_retention_policy(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_log_stream() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = LogsOps::new(&client);

        let body = DeleteLogStreamRequest::fixture();
        let result = ops.delete_log_stream(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_metric_filters() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(
            serde_json::to_value(DescribeMetricFiltersResponse::fixture()).unwrap(),
        );

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = LogsOps::new(&client);

        let body = DescribeMetricFiltersRequest::fixture();
        let result = ops.describe_metric_filters(&body).await;
        assert!(result.is_ok());
    }
}
