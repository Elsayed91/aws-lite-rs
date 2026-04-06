//! Operation contracts for the Amazon EMR API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/emr.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::emr::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon EMR API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::emr::EmrClient`] instead.
pub struct EmrOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> EmrOps<'a> {
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
        "https://elasticmapreduce.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Provides the status of all clusters visible to this Amazon Web Services account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListClustersInput`]
    ///
    /// # Response
    /// [`ListClustersOutput`]
    #[allow(dead_code)]
    pub(crate) async fn list_clusters(
        &self,
        body: &ListClustersInput,
    ) -> Result<ListClustersOutput> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize list_clusters request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "elasticmapreduce",
                "ElasticMapReduce.ListClusters",
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
                    message: format!("Failed to read list_clusters response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_clusters response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Provides cluster-level details including status, hardware and software configuration,
    /// VPC settings, and so on.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeClusterInput`]
    ///
    /// # Response
    /// [`DescribeClusterOutput`]
    #[allow(dead_code)]
    pub(crate) async fn describe_cluster(
        &self,
        body: &DescribeClusterInput,
    ) -> Result<DescribeClusterOutput> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_cluster request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "elasticmapreduce",
                "ElasticMapReduce.DescribeCluster",
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
                    message: format!("Failed to read describe_cluster response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_cluster response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// TerminateJobFlows shuts a list of clusters (job flows) down. When a job flow is shut
    /// down, any step not yet completed is canceled and the EC2 instances on which the cluster
    /// is running are stopped.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`TerminateJobFlowsInput`]
    #[allow(dead_code)]
    pub(crate) async fn terminate_job_flows(&self, body: &TerminateJobFlowsInput) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize terminate_job_flows request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "elasticmapreduce",
                "ElasticMapReduce.TerminateJobFlows",
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
    async fn test_list_clusters() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(ListClustersOutput::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EmrOps::new(&client);

        let body = ListClustersInput::fixture();
        let result = ops.list_clusters(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeClusterOutput::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EmrOps::new(&client);

        let body = DescribeClusterInput::fixture();
        let result = ops.describe_cluster(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_terminate_job_flows() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EmrOps::new(&client);

        let body = TerminateJobFlowsInput::fixture();
        let result = ops.terminate_job_flows(&body).await;
        assert!(result.is_ok());
    }
}
