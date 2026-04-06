//! Types for the Amazon Elastic Container Registry API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `ecr.v1.DescribeRepositoriesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//DescribeRepositoriesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeRepositoriesRequest {
    /// The Amazon Web Services account ID associated with the registry that contains the
    /// repositories to be described. If you do not specify a registry, the default registry is
    /// assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,

    /// A list of repositories to describe. If this parameter is omitted, then all repositories
    /// in a registry are described.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub repository_names: Vec<String>,

    /// The nextToken value returned from a previous paginated DescribeRepositories request
    /// where maxResults was used and the results exceeded the value of that parameter.
    /// Pagination continues from the end of the previous results that returned the nextToken
    /// value. This value is null when there are no more results to return. This option cannot
    /// be used when you specify repositories with repositoryNames. This token should be treated
    /// as an opaque identifier that is only used to retrieve the next items in a list and not
    /// for other programmatic purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of repository results returned by DescribeRepositories in paginated
    /// output. When this parameter is used, DescribeRepositories only returns maxResults
    /// results in a single page along with a nextToken response element. The remaining results
    /// of the initial request can be seen by sending another DescribeRepositories request with
    /// the returned nextToken value. This value can be between 1 and 1000. If this parameter is
    /// not used, then DescribeRepositories returns up to 100 results and a nextToken value, if
    /// applicable. This option cannot be used when you specify repositories with
    /// repositoryNames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeRepositoriesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            registry_id: Some("test-registry_id".into()),
            repository_names: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ecr.v1.DescribeRepositoriesResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//DescribeRepositoriesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeRepositoriesResponse {
    /// A list of repository objects corresponding to valid repositories.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,

    /// The nextToken value to include in a future DescribeRepositories request. When the
    /// results of a DescribeRepositories request exceed maxResults, this value can be used to
    /// retrieve the next page of results. This value is null when there are no more results to
    /// return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeRepositoriesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            repositories: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// An object representing a repository.
///
/// **AWS API**: `ecr.v1.Repository`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//Repository>
///
/// ## Coverage
/// 6 of 9 fields included.
/// Omitted fields:
/// - `imageTagMutabilityExclusionFilters` — not selected in manifest
/// - `imageScanningConfiguration` — not selected in manifest
/// - `encryptionConfiguration` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    /// The Amazon Resource Name (ARN) that identifies the repository. The ARN contains the
    /// arn:aws:ecr namespace, followed by the region of the repository, Amazon Web Services
    /// account ID of the repository owner, repository namespace, and repository name. For
    /// example, arn:aws:ecr:region:012345678910:repository-namespace/repository-name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_arn: Option<String>,

    /// The Amazon Web Services account ID associated with the registry that contains the
    /// repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,

    /// The name of the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,

    /// The URI for the repository. You can use this URI for container image push and pull
    /// operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_uri: Option<String>,

    /// The date and time, in JavaScript date format, when the repository was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,

    /// The tag mutability setting for the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
}

impl Repository {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            repository_arn: Some("test-repository_arn".into()),
            registry_id: Some("test-registry_id".into()),
            repository_name: Some("test-repository_name".into()),
            repository_uri: Some("test-repository_uri".into()),
            image_tag_mutability: Some("test-image_tag_mutability".into()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `ecr.v1.DescribeImagesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//DescribeImagesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeImagesRequest {
    /// The Amazon Web Services account ID associated with the registry that contains the
    /// repository in which to describe images. If you do not specify a registry, the default
    /// registry is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,

    /// The repository that contains the images to describe.
    pub repository_name: String,

    /// The list of image IDs for the requested repository.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image_ids: Vec<ImageIdentifier>,

    /// The nextToken value returned from a previous paginated DescribeImages request where
    /// maxResults was used and the results exceeded the value of that parameter. Pagination
    /// continues from the end of the previous results that returned the nextToken value. This
    /// value is null when there are no more results to return. This option cannot be used when
    /// you specify images with imageIds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of repository results returned by DescribeImages in paginated output.
    /// When this parameter is used, DescribeImages only returns maxResults results in a single
    /// page along with a nextToken response element. The remaining results of the initial
    /// request can be seen by sending another DescribeImages request with the returned
    /// nextToken value. This value can be between 1 and 1000. If this parameter is not used,
    /// then DescribeImages returns up to 100 results and a nextToken value, if applicable. This
    /// option cannot be used when you specify images with imageIds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,

    /// The filter key and value with which to filter your DescribeImages results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DescribeImagesFilter>,
}

impl DescribeImagesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            registry_id: Some("test-registry_id".into()),
            repository_name: "test-repository_name".into(),
            image_ids: vec![],
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
            filter: Some(DescribeImagesFilter::fixture()),
        }
    }
}

///
/// **AWS API**: `ecr.v1.DescribeImagesResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//DescribeImagesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeImagesResponse {
    /// A list of ImageDetail objects that contain data about the image.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image_details: Vec<ImageDetail>,

    /// The nextToken value to include in a future DescribeImages request. When the results of a
    /// DescribeImages request exceed maxResults, this value can be used to retrieve the next
    /// page of results. This value is null when there are no more results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeImagesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_details: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// An object that describes an image returned by a DescribeImages operation.
///
/// **AWS API**: `ecr.v1.ImageDetail`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//ImageDetail>
///
/// ## Coverage
/// 8 of 15 fields included.
/// Omitted fields:
/// - `imageScanStatus` — not selected in manifest
/// - `imageScanFindingsSummary` — not selected in manifest
/// - `artifactMediaType` — not selected in manifest
/// - `subjectManifestDigest` — not selected in manifest
/// - `imageStatus` — not selected in manifest
/// - `lastArchivedAt` — not selected in manifest
/// - `lastActivatedAt` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageDetail {
    /// The Amazon Web Services account ID associated with the registry to which this image
    /// belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,

    /// The name of the repository to which this image belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,

    /// The sha256 digest of the image manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,

    /// The list of tags associated with this image.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image_tags: Vec<String>,

    /// The size, in bytes, of the image in the repository. If the image is a manifest list,
    /// this will be the max size of all manifests in the list. Starting with Docker version
    /// 1.9, the Docker client compresses image layers before pushing them to a V2 Docker
    /// registry. The output of the docker images command shows the uncompressed image size.
    /// Therefore, Docker might return a larger image than the image shown in the Amazon Web
    /// Services Management Console.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size_in_bytes: Option<i64>,

    /// The date and time, expressed in standard JavaScript date format, at which the current
    /// image was pushed to the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pushed_at: Option<f64>,

    /// The media type of the image manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_manifest_media_type: Option<String>,

    /// The date and time, expressed in standard JavaScript date format, when Amazon ECR
    /// recorded the last image pull. Amazon ECR refreshes the last image pull timestamp at
    /// least once every 24 hours. For example, if you pull an image once a day then the
    /// lastRecordedPullTime timestamp will indicate the exact time that the image was last
    /// pulled. However, if you pull an image once an hour, because Amazon ECR refreshes the
    /// lastRecordedPullTime timestamp at least once every 24 hours, the result may not be the
    /// exact time that the image was last pulled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_recorded_pull_time: Option<f64>,
}

impl ImageDetail {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            registry_id: Some("test-registry_id".into()),
            repository_name: Some("test-repository_name".into()),
            image_digest: Some("test-image_digest".into()),
            image_tags: vec![],
            image_size_in_bytes: Some(100),
            image_manifest_media_type: Some("test-image_manifest_media_type".into()),
            ..Default::default()
        }
    }
}

/// An object representing a filter on a DescribeImages operation.
///
/// **AWS API**: `ecr.v1.DescribeImagesFilter`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//DescribeImagesFilter>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `imageStatus` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeImagesFilter {
    /// The tag status with which to filter your DescribeImages results. You can filter results
    /// based on whether they are TAGGED or UNTAGGED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_status: Option<String>,
}

impl DescribeImagesFilter {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tag_status: Some("test-tag_status".into()),
        }
    }
}

/// An object with identifying information for an image in an Amazon ECR repository.
///
/// **AWS API**: `ecr.v1.ImageIdentifier`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//ImageIdentifier>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageIdentifier {
    /// The sha256 digest of the image manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,

    /// The tag used for the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
}

impl ImageIdentifier {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_digest: Some("test-image_digest".into()),
            image_tag: Some("test-image_tag".into()),
        }
    }
}

///
/// **AWS API**: `ecr.v1.PutLifecyclePolicyRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//PutLifecyclePolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutLifecyclePolicyRequest {
    /// The Amazon Web Services account ID associated with the registry that contains the
    /// repository. If you do&#x2028; not specify a registry, the default registry is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,

    /// The name of the repository to receive the policy.
    pub repository_name: String,

    /// The JSON repository policy text to apply to the repository.
    pub lifecycle_policy_text: String,
}

impl PutLifecyclePolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            registry_id: Some("test-registry_id".into()),
            repository_name: "test-repository_name".into(),
            lifecycle_policy_text: "test-lifecycle_policy_text".into(),
        }
    }
}

///
/// **AWS API**: `ecr.v1.PutLifecyclePolicyResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//PutLifecyclePolicyResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutLifecyclePolicyResponse {
    /// The registry ID associated with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,

    /// The repository name associated with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,

    /// The JSON repository policy text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
}

impl PutLifecyclePolicyResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            registry_id: Some("test-registry_id".into()),
            repository_name: Some("test-repository_name".into()),
            lifecycle_policy_text: Some("test-lifecycle_policy_text".into()),
        }
    }
}

/// Deletes specified images within a specified repository. Images are specified with either the
/// imageTag or imageDigest.
///
/// **AWS API**: `ecr.v1.BatchDeleteImageRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//BatchDeleteImageRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteImageRequest {
    /// The Amazon Web Services account ID associated with the registry that contains the image
    /// to delete. If you do not specify a registry, the default registry is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,

    /// The repository that contains the image to delete.
    pub repository_name: String,

    /// A list of image ID references that correspond to images to delete. The format of the
    /// imageIds reference is imageTag=tag or imageDigest=digest.
    #[serde(default)]
    pub image_ids: Vec<ImageIdentifier>,
}

impl BatchDeleteImageRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            registry_id: Some("test-registry_id".into()),
            repository_name: "test-repository_name".into(),
            image_ids: vec![],
        }
    }
}

///
/// **AWS API**: `ecr.v1.BatchDeleteImageResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//BatchDeleteImageResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteImageResponse {
    /// The image IDs of the deleted images.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image_ids: Vec<ImageIdentifier>,

    /// Any failures associated with the call.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<ImageFailure>,
}

impl BatchDeleteImageResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_ids: vec![],
            failures: vec![],
        }
    }
}

/// An object representing an Amazon ECR image failure.
///
/// **AWS API**: `ecr.v1.ImageFailure`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECR/latest/APIReference//ImageFailure>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageFailure {
    /// The image ID associated with the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,

    /// The code associated with the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,

    /// The reason for the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

impl ImageFailure {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_id: Some(ImageIdentifier::fixture()),
            failure_code: Some("test-failure_code".into()),
            failure_reason: Some("test-failure_reason".into()),
        }
    }
}
