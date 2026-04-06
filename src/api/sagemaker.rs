//! Amazon SageMaker API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::sagemaker::SagemakerOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::sagemaker::SagemakerOps,
    types::sagemaker::{
        ListNotebookInstancesInput, ListNotebookInstancesOutput, StopNotebookInstanceInput,
    },
};

/// Client for the Amazon SageMaker API
pub struct SagemakerClient<'a> {
    ops: SagemakerOps<'a>,
}

impl<'a> SagemakerClient<'a> {
    /// Create a new Amazon SageMaker API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: SagemakerOps::new(client),
        }
    }

    /// Returns a list of the Amazon SageMaker notebook instances in the requester's account.
    pub async fn list_notebook_instances(
        &self,
        body: &ListNotebookInstancesInput,
    ) -> Result<ListNotebookInstancesOutput> {
        self.ops.list_notebook_instances(body).await
    }

    /// Terminates the ML compute instance. Before terminating the instance, SageMaker disconnects the ML storage volume from it.
    pub async fn stop_notebook_instance(&self, body: &StopNotebookInstanceInput) -> Result<()> {
        self.ops.stop_notebook_instance(body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::sagemaker::*;

    #[tokio::test]
    async fn list_notebook_instances_returns_empty() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "NotebookInstances": []
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .sagemaker()
            .list_notebook_instances(&ListNotebookInstancesInput::default())
            .await
            .unwrap();
        assert_eq!(result.notebook_instances.len(), 0);
        assert!(result.next_token.is_none());
    }

    #[tokio::test]
    async fn list_notebook_instances_returns_instances() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "NotebookInstances": [
                {
                    "NotebookInstanceName": "my-notebook",
                    "NotebookInstanceArn": "arn:aws:sagemaker:eu-central-1:123456789012:notebook-instance/my-notebook",
                    "NotebookInstanceStatus": "InService",
                    "InstanceType": "ml.t3.medium",
                    "CreationTime": 1700000000.0,
                    "LastModifiedTime": 1700001000.0,
                    "Url": "my-notebook.notebook.eu-central-1.sagemaker.aws"
                }
            ],
            "NextToken": null
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .sagemaker()
            .list_notebook_instances(&ListNotebookInstancesInput {
                status_equals: Some("InService".to_string()),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.notebook_instances.len(), 1);
        let nb = &result.notebook_instances[0];
        assert_eq!(nb.notebook_instance_name.as_str(), "my-notebook");
        assert_eq!(
            nb.notebook_instance_arn.as_str(),
            "arn:aws:sagemaker:eu-central-1:123456789012:notebook-instance/my-notebook"
        );
        assert_eq!(nb.notebook_instance_status.as_deref(), Some("InService"));
        assert_eq!(nb.instance_type.as_deref(), Some("ml.t3.medium"));
        assert_eq!(nb.creation_time, Some(1700000000.0));
        assert_eq!(nb.last_modified_time, Some(1700001000.0));
        assert_eq!(
            nb.url.as_deref(),
            Some("my-notebook.notebook.eu-central-1.sagemaker.aws")
        );
    }

    #[tokio::test]
    async fn stop_notebook_instance_succeeds() {
        let mut mock = crate::MockClient::new();
        // StopNotebookInstance returns empty body on success
        mock.expect_post("/").returning_json(serde_json::json!({}));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .sagemaker()
            .stop_notebook_instance(&StopNotebookInstanceInput {
                notebook_instance_name: "my-notebook".to_string(),
            })
            .await;
        assert!(result.is_ok(), "Expected stop_notebook_instance to succeed");
    }
}
