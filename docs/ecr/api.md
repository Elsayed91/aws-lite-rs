# Amazon Elastic Container Registry API

## Overview

Amazon Elastic Container Registry (ECR) is a managed container image registry service. This client provides operations for managing ECR repositories, describing images, managing lifecycle policies, and batch deleting images.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let ecr = client.ecr();
```

## Features

- Describe repositories (by name or list all)
- Describe images in a repository (with digest, tags, size, push time)
- Create or update lifecycle policies for image retention
- Batch delete images by digest or tag
- JSON target wire format (JSON body, `X-Amz-Target` header)

## Types

| Type | Description |
|------|-------------|
| `DescribeRepositoriesRequest` | Filter by names, with pagination |
| `DescribeRepositoriesResponse` | Response with repository list |
| `Repository` | Repository with name, ARN, URI, creation time, tag mutability |
| `DescribeImagesRequest` | Repository name, optional image IDs and filter |
| `DescribeImagesResponse` | Response with image detail list |
| `ImageDetail` | Image with digest, tags, size, push time, manifest type |
| `DescribeImagesFilter` | Filter by tag status (TAGGED/UNTAGGED) |
| `ImageIdentifier` | Image digest and/or tag |
| `PutLifecyclePolicyRequest` | Repository name + lifecycle policy JSON |
| `PutLifecyclePolicyResponse` | Confirmation with repository and policy text |
| `BatchDeleteImageRequest` | Repository name + image IDs to delete |
| `BatchDeleteImageResponse` | Deleted image IDs and failures |
| `ImageFailure` | Image ID, failure code, and reason |

## Error Handling

Common errors for this API:
- `AwsError::ServiceError` with `RepositoryNotFoundException` -- repository not found
- `AwsError::ServiceError` with `ImageNotFoundException` -- image not found
- `AwsError::ServiceError` with `InvalidParameterException` -- invalid parameters
- `AwsError::AccessDenied` -- insufficient IAM permissions

## Notes

- ECR timestamps (createdAt, imagePushedAt) are epoch floats mapped to `f64`
- The endpoint prefix is `api.ecr` (URL: `api.ecr.{region}.amazonaws.com`) but the SigV4 signing scope uses `ecr`
- BatchDeleteImage returns failures for non-existent images rather than raising an error
