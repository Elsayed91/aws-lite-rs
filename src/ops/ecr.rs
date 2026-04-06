//! Operation contracts for the Amazon Elastic Container Registry API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/ecr.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::ecr::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon Elastic Container Registry API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::ecr::EcrClient`] instead.
pub struct EcrOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> EcrOps<'a> {
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
        "https://api.ecr.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Describes image repositories in a registry.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeRepositoriesRequest`]
    ///
    /// # Response
    /// [`DescribeRepositoriesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_repositories(
        &self,
        body: &DescribeRepositoriesRequest,
    ) -> Result<DescribeRepositoriesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_repositories request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecr",
                "AmazonEC2ContainerRegistry_V20150921.DescribeRepositories",
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
                    message: format!("Failed to read describe_repositories response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_repositories response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Returns metadata about the images in a repository.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeImagesRequest`]
    ///
    /// # Response
    /// [`DescribeImagesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_images(
        &self,
        body: &DescribeImagesRequest,
    ) -> Result<DescribeImagesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_images request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecr",
                "AmazonEC2ContainerRegistry_V20150921.DescribeImages",
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
                    message: format!("Failed to read describe_images response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_images response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates the lifecycle policy for the specified repository.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`PutLifecyclePolicyRequest`]
    ///
    /// # Response
    /// [`PutLifecyclePolicyResponse`]
    #[allow(dead_code)]
    pub(crate) async fn put_lifecycle_policy(
        &self,
        body: &PutLifecyclePolicyRequest,
    ) -> Result<PutLifecyclePolicyResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize put_lifecycle_policy request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecr",
                "AmazonEC2ContainerRegistry_V20150921.PutLifecyclePolicy",
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
                    message: format!("Failed to read put_lifecycle_policy response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse put_lifecycle_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a list of specified images within a repository.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`BatchDeleteImageRequest`]
    ///
    /// # Response
    /// [`BatchDeleteImageResponse`]
    #[allow(dead_code)]
    pub(crate) async fn batch_delete_image(
        &self,
        body: &BatchDeleteImageRequest,
    ) -> Result<BatchDeleteImageResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize batch_delete_image request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecr",
                "AmazonEC2ContainerRegistry_V20150921.BatchDeleteImage",
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
                    message: format!("Failed to read batch_delete_image response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse batch_delete_image response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_repositories() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeRepositoriesResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcrOps::new(&client);

        let body = DescribeRepositoriesRequest::fixture();
        let result = ops.describe_repositories(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_images() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeImagesResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcrOps::new(&client);

        let body = DescribeImagesRequest::fixture();
        let result = ops.describe_images(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_lifecycle_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(PutLifecyclePolicyResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcrOps::new(&client);

        let body = PutLifecyclePolicyRequest::fixture();
        let result = ops.put_lifecycle_policy(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_batch_delete_image() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(BatchDeleteImageResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcrOps::new(&client);

        let body = BatchDeleteImageRequest::fixture();
        let result = ops.batch_delete_image(&body).await;
        assert!(result.is_ok());
    }
}
