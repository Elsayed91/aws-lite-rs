//! Operation contracts for the Amazon SageMaker API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/sagemaker.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::sagemaker::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon SageMaker API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::sagemaker::SagemakerClient`] instead.
pub struct SagemakerOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> SagemakerOps<'a> {
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
        "https://api.sagemaker.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Returns a list of the Amazon SageMaker notebook instances in the requester's account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListNotebookInstancesInput`]
    ///
    /// # Response
    /// [`ListNotebookInstancesOutput`]
    #[allow(dead_code)]
    pub(crate) async fn list_notebook_instances(
        &self,
        body: &ListNotebookInstancesInput,
    ) -> Result<ListNotebookInstancesOutput> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize list_notebook_instances request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "sagemaker",
                "SageMaker.ListNotebookInstances",
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
                    message: format!("Failed to read list_notebook_instances response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_notebook_instances response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Terminates the ML compute instance. Before terminating the instance, SageMaker
    /// disconnects the ML storage volume from it.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`StopNotebookInstanceInput`]
    #[allow(dead_code)]
    pub(crate) async fn stop_notebook_instance(
        &self,
        body: &StopNotebookInstanceInput,
    ) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize stop_notebook_instance request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "sagemaker",
                "SageMaker.StopNotebookInstance",
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
    async fn test_list_notebook_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(ListNotebookInstancesOutput::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = SagemakerOps::new(&client);

        let body = ListNotebookInstancesInput::fixture();
        let result = ops.list_notebook_instances(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stop_notebook_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = SagemakerOps::new(&client);

        let body = StopNotebookInstanceInput::fixture();
        let result = ops.stop_notebook_instance(&body).await;
        assert!(result.is_ok());
    }
}
