//! Operation contracts for the Amazon Kinesis API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/kinesis.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::kinesis::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon Kinesis API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::kinesis::KinesisClient`] instead.
pub struct KinesisOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> KinesisOps<'a> {
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
        "https://kinesis.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Lists the Kinesis data streams associated with the AWS account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListStreamsInput`]
    ///
    /// # Response
    /// [`ListStreamsOutput`]
    #[allow(dead_code)]
    pub(crate) async fn list_streams(&self, body: &ListStreamsInput) -> Result<ListStreamsOutput> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize list_streams request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "kinesis",
                "Kinesis_20131202.ListStreams",
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
                    message: format!("Failed to read list_streams response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_streams response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Provides a summarized description of the specified Kinesis data stream.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeStreamSummaryInput`]
    ///
    /// # Response
    /// [`DescribeStreamSummaryOutput`]
    #[allow(dead_code)]
    pub(crate) async fn describe_stream_summary(
        &self,
        body: &DescribeStreamSummaryInput,
    ) -> Result<DescribeStreamSummaryOutput> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_stream_summary request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "kinesis",
                "Kinesis_20131202.DescribeStreamSummary",
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
                    message: format!("Failed to read describe_stream_summary response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_stream_summary response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a Kinesis data stream and all its shards and data.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteStreamInput`]
    #[allow(dead_code)]
    pub(crate) async fn delete_stream(&self, body: &DeleteStreamInput) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize delete_stream request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "kinesis",
                "Kinesis_20131202.DeleteStream",
                "1.1",
                &body_bytes,
            )
            .await?;
        response.error_for_status("json").await?;
        Ok(())
    }

    /// Updates the capacity mode of a Kinesis data stream (PROVISIONED or ON_DEMAND).
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`UpdateStreamModeInput`]
    #[allow(dead_code)]
    pub(crate) async fn update_stream_mode(&self, body: &UpdateStreamModeInput) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize update_stream_mode request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "kinesis",
                "Kinesis_20131202.UpdateStreamMode",
                "1.1",
                &body_bytes,
            )
            .await?;
        response.error_for_status("json").await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_streams() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(ListStreamsOutput::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = KinesisOps::new(&client);

        let body = ListStreamsInput::fixture();
        let result = ops.list_streams(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_stream_summary() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeStreamSummaryOutput::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = KinesisOps::new(&client);

        let body = DescribeStreamSummaryInput::fixture();
        let result = ops.describe_stream_summary(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_stream() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = KinesisOps::new(&client);

        let body = DeleteStreamInput::fixture();
        let result = ops.delete_stream(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_stream_mode() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = KinesisOps::new(&client);

        let body = UpdateStreamModeInput::fixture();
        let result = ops.update_stream_mode(&body).await;
        assert!(result.is_ok());
    }
}
