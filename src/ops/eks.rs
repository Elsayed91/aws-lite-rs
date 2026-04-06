//! Operation contracts for the Amazon Elastic Kubernetes Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/eks.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::eks::*;
use crate::{AwsHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Amazon Elastic Kubernetes Service API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::eks::EksClient`] instead.
pub struct EksOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> EksOps<'a> {
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
        "https://eks.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Returns descriptive information about an Amazon EKS cluster.
    ///
    /// **AWS API**: `GET /clusters/{name}`
    ///
    /// # Path Parameters
    /// - `name` —  *(required)*
    ///
    /// # Response
    /// [`DescribeClusterResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_cluster(&self, name: &str) -> Result<DescribeClusterResponse> {
        let url = format!("{}/clusters/{}", self.base_url(), encode(name),);
        let response = self.client.get_json(&url, "eks").await?;
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

    /// Lists the managed node groups associated with the specified cluster.
    ///
    /// **AWS API**: `GET /clusters/{name}/node-groups`
    ///
    /// # Path Parameters
    /// - `name` —  *(required)*
    ///
    /// # Query Parameters
    /// - `maxResults` —
    /// - `nextToken` —
    ///
    /// # Response
    /// [`ListNodegroupsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_nodegroups(
        &self,
        name: &str,
        max_results: &str,
        next_token: &str,
    ) -> Result<ListNodegroupsResponse> {
        let url = format!("{}/clusters/{}/node-groups", self.base_url(), encode(name),);
        let url = crate::append_query_params(
            url,
            &[("maxResults", max_results), ("nextToken", next_token)],
        );
        let response = self.client.get_json(&url, "eks").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_nodegroups response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_nodegroups response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Returns descriptive information about an Amazon EKS node group.
    ///
    /// **AWS API**: `GET /clusters/{name}/node-groups/{nodegroupName}`
    ///
    /// # Path Parameters
    /// - `name` —  *(required)*
    /// - `nodegroupName` —  *(required)*
    ///
    /// # Response
    /// [`DescribeNodegroupResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_nodegroup(
        &self,
        name: &str,
        nodegroup_name: &str,
    ) -> Result<DescribeNodegroupResponse> {
        let url = format!(
            "{}/clusters/{}/node-groups/{}",
            self.base_url(),
            encode(name),
            encode(nodegroup_name),
        );
        let response = self.client.get_json(&url, "eks").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_nodegroup response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_nodegroup response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Updates an Amazon EKS managed node group configuration.
    ///
    /// **AWS API**: `POST /clusters/{name}/node-groups/{nodegroupName}/update-config`
    ///
    /// # Path Parameters
    /// - `name` —  *(required)*
    /// - `nodegroupName` —  *(required)*
    ///
    /// # Request Body
    /// [`UpdateNodegroupConfigRequest`]
    ///
    /// # Response
    /// [`UpdateNodegroupConfigResponse`]
    #[allow(dead_code)]
    pub(crate) async fn update_nodegroup_config(
        &self,
        name: &str,
        nodegroup_name: &str,
        body: &UpdateNodegroupConfigRequest,
    ) -> Result<UpdateNodegroupConfigResponse> {
        let url = format!(
            "{}/clusters/{}/node-groups/{}/update-config",
            self.base_url(),
            encode(name),
            encode(nodegroup_name),
        );
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize update_nodegroup_config request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post(&url, "eks", &body_bytes, "application/json")
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read update_nodegroup_config response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse update_nodegroup_config response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/clusters/test-name")
            .returning_json(serde_json::to_value(DescribeClusterResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EksOps::new(&client);

        let result = ops.describe_cluster("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_nodegroups() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/clusters/test-name/node-groups?maxResults=test-maxResults&nextToken=test-nextToken",
        )
        .returning_json(serde_json::to_value(ListNodegroupsResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EksOps::new(&client);

        let result = ops
            .list_nodegroups("test-name", "test-maxResults", "test-nextToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_nodegroup() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/clusters/test-name/node-groups/test-nodegroupName")
            .returning_json(serde_json::to_value(DescribeNodegroupResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EksOps::new(&client);

        let result = ops
            .describe_nodegroup("test-name", "test-nodegroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_nodegroup_config() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/clusters/test-name/node-groups/test-nodegroupName/update-config")
            .returning_json(
                serde_json::to_value(UpdateNodegroupConfigResponse::fixture()).unwrap(),
            );

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EksOps::new(&client);

        let body = UpdateNodegroupConfigRequest::fixture();
        let result = ops
            .update_nodegroup_config("test-name", "test-nodegroupName", &body)
            .await;
        assert!(result.is_ok());
    }
}
