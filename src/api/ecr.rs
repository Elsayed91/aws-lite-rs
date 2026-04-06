//! Amazon Elastic Container Registry API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::ecr::EcrOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::ecr::EcrOps,
    types::ecr::{
        BatchDeleteImageRequest, BatchDeleteImageResponse, DescribeImagesRequest,
        DescribeImagesResponse, DescribeRepositoriesRequest, DescribeRepositoriesResponse,
        PutLifecyclePolicyRequest, PutLifecyclePolicyResponse,
    },
};

/// Client for the Amazon Elastic Container Registry API
pub struct EcrClient<'a> {
    ops: EcrOps<'a>,
}

impl<'a> EcrClient<'a> {
    /// Create a new Amazon Elastic Container Registry API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: EcrOps::new(client),
        }
    }

    /// Describes image repositories in a registry.
    pub async fn describe_repositories(
        &self,
        body: &DescribeRepositoriesRequest,
    ) -> Result<DescribeRepositoriesResponse> {
        self.ops.describe_repositories(body).await
    }

    /// Returns metadata about the images in a repository.
    pub async fn describe_images(
        &self,
        body: &DescribeImagesRequest,
    ) -> Result<DescribeImagesResponse> {
        self.ops.describe_images(body).await
    }

    /// Creates or updates the lifecycle policy for the specified repository.
    pub async fn put_lifecycle_policy(
        &self,
        body: &PutLifecyclePolicyRequest,
    ) -> Result<PutLifecyclePolicyResponse> {
        self.ops.put_lifecycle_policy(body).await
    }

    /// Deletes a list of specified images within a repository.
    pub async fn batch_delete_image(
        &self,
        body: &BatchDeleteImageRequest,
    ) -> Result<BatchDeleteImageResponse> {
        self.ops.batch_delete_image(body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ecr::ImageIdentifier;

    #[tokio::test]
    async fn describe_repositories_returns_repos() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "repositories": [{
                "repositoryArn": "arn:aws:ecr:us-east-1:123456789012:repository/my-repo",
                "registryId": "123456789012",
                "repositoryName": "my-repo",
                "repositoryUri": "123456789012.dkr.ecr.us-east-1.amazonaws.com/my-repo",
                "createdAt": 1700000000.0,
                "imageTagMutability": "MUTABLE"
            }]
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecr()
            .describe_repositories(&DescribeRepositoriesRequest {
                repository_names: vec!["my-repo".into()],
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.repositories.len(), 1);
        let repo = &result.repositories[0];
        assert_eq!(repo.repository_name.as_deref(), Some("my-repo"));
        assert_eq!(repo.image_tag_mutability.as_deref(), Some("MUTABLE"));
        assert!(repo.repository_uri.as_deref().unwrap().contains("my-repo"));
        assert!(repo.created_at.is_some());
    }

    #[tokio::test]
    async fn describe_images_returns_details() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "imageDetails": [{
                "registryId": "123456789012",
                "repositoryName": "my-repo",
                "imageDigest": "sha256:abc123",
                "imageTags": ["latest", "v1.0"],
                "imageSizeInBytes": 12345678,
                "imagePushedAt": 1700000000.0,
                "imageManifestMediaType": "application/vnd.docker.distribution.manifest.v2+json"
            }]
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecr()
            .describe_images(&DescribeImagesRequest {
                repository_name: "my-repo".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.image_details.len(), 1);
        let img = &result.image_details[0];
        assert_eq!(img.image_digest.as_deref(), Some("sha256:abc123"));
        assert_eq!(img.image_tags, vec!["latest", "v1.0"]);
        assert_eq!(img.image_size_in_bytes, Some(12345678));
        assert!(img.image_pushed_at.is_some());
        assert_eq!(
            img.image_manifest_media_type.as_deref(),
            Some("application/vnd.docker.distribution.manifest.v2+json")
        );
    }

    #[tokio::test]
    async fn put_lifecycle_policy_returns_policy() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "registryId": "123456789012",
            "repositoryName": "my-repo",
            "lifecyclePolicyText": "{\"rules\":[{\"rulePriority\":1}]}"
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecr()
            .put_lifecycle_policy(&PutLifecyclePolicyRequest {
                repository_name: "my-repo".into(),
                lifecycle_policy_text: "{\"rules\":[{\"rulePriority\":1}]}".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.repository_name.as_deref(), Some("my-repo"));
        assert_eq!(result.registry_id.as_deref(), Some("123456789012"));
        assert!(result.lifecycle_policy_text.is_some());
    }

    #[tokio::test]
    async fn batch_delete_image_returns_deleted_ids() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "imageIds": [{
                "imageDigest": "sha256:abc123",
                "imageTag": "latest"
            }],
            "failures": []
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecr()
            .batch_delete_image(&BatchDeleteImageRequest {
                repository_name: "my-repo".into(),
                image_ids: vec![ImageIdentifier {
                    image_digest: Some("sha256:abc123".into()),
                    image_tag: Some("latest".into()),
                }],
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.image_ids.len(), 1);
        assert_eq!(
            result.image_ids[0].image_digest.as_deref(),
            Some("sha256:abc123")
        );
        assert!(result.failures.is_empty());
    }
}
