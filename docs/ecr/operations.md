# Amazon ECR Operations

## Read Operations

### describe_repositories

**Signature**: `pub async fn describe_repositories(&self, body: &DescribeRepositoriesRequest) -> Result<DescribeRepositoriesResponse>`

Describes image repositories in a registry.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.repository_names` | `Vec<String>` | Filter by repository names |
| `body.registry_id` | `Option<String>` | Registry account ID (defaults to caller) |
| `body.next_token` | `Option<String>` | Pagination token |
| `body.max_results` | `Option<i32>` | Max results per page |

**Returns**: `Result<DescribeRepositoriesResponse>` with `repositories` Vec and optional `next_token`.

---

### describe_images

**Signature**: `pub async fn describe_images(&self, body: &DescribeImagesRequest) -> Result<DescribeImagesResponse>`

Returns metadata about the images in a repository.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.repository_name` | `String` | Repository name (required) |
| `body.registry_id` | `Option<String>` | Registry account ID |
| `body.image_ids` | `Vec<ImageIdentifier>` | Filter by image digest/tag |
| `body.filter` | `Option<DescribeImagesFilter>` | Filter by tag status |
| `body.next_token` | `Option<String>` | Pagination token |
| `body.max_results` | `Option<i32>` | Max results per page |

**Returns**: `Result<DescribeImagesResponse>` with `image_details` Vec and optional `next_token`.

---

## Write Operations

### put_lifecycle_policy

**Signature**: `pub async fn put_lifecycle_policy(&self, body: &PutLifecyclePolicyRequest) -> Result<PutLifecyclePolicyResponse>`

Creates or updates the lifecycle policy for the specified repository.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.repository_name` | `String` | Repository name (required) |
| `body.lifecycle_policy_text` | `String` | JSON lifecycle policy (required) |
| `body.registry_id` | `Option<String>` | Registry account ID |

**Returns**: `Result<PutLifecyclePolicyResponse>` with `registry_id`, `repository_name`, and `lifecycle_policy_text`.

---

### batch_delete_image

**Signature**: `pub async fn batch_delete_image(&self, body: &BatchDeleteImageRequest) -> Result<BatchDeleteImageResponse>`

Deletes a list of specified images within a repository.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.repository_name` | `String` | Repository name (required) |
| `body.image_ids` | `Vec<ImageIdentifier>` | Images to delete (by digest/tag) |
| `body.registry_id` | `Option<String>` | Registry account ID |

**Returns**: `Result<BatchDeleteImageResponse>` with `image_ids` (deleted) and `failures` Vec.
