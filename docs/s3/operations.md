# Amazon S3 Operations

## Bucket Policy

### put_bucket_policy

**Signature**: `pub async fn put_bucket_policy(bucket: &str, body: &str) -> Result<()>`

Applies a JSON bucket policy to an S3 bucket.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bucket` | `&str` | Bucket name |
| `body` | `&str` | JSON policy document |

**Permission**: `s3:PutBucketPolicy`

---

### delete_bucket_policy

**Signature**: `pub async fn delete_bucket_policy(bucket: &str) -> Result<()>`

Removes the policy from a bucket.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bucket` | `&str` | Bucket name |

**Permission**: `s3:DeleteBucketPolicy`

## Public Access Block

### put_public_access_block

**Signature**: `pub async fn put_public_access_block(bucket: &str, body: &PublicAccessBlockConfiguration) -> Result<()>`

Configures the public access block settings for a bucket.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bucket` | `&str` | Bucket name |
| `body` | `&PublicAccessBlockConfiguration` | Configuration with 4 boolean fields |

**Permission**: `s3:PutBucketPublicAccessBlock`

## Lifecycle Configuration

### put_bucket_lifecycle_configuration

**Signature**: `pub async fn put_bucket_lifecycle_configuration(bucket: &str, body: &BucketLifecycleConfiguration) -> Result<()>`

Creates or replaces the lifecycle configuration for a bucket. Uses hand-built XML serialization because S3 expects element names that differ from the serde-derived names.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bucket` | `&str` | Bucket name |
| `body` | `&BucketLifecycleConfiguration` | Configuration with lifecycle rules |

**Permission**: `s3:PutLifecycleConfiguration`
