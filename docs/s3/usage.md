# Amazon S3 Usage Examples

## Bucket Policy

### Apply a bucket policy

```rust
use aws_lite::AwsHttpClient;

let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let s3 = client.s3();

let policy = serde_json::json!({
    "Version": "2012-10-17",
    "Statement": [{
        "Effect": "Deny",
        "Principal": "*",
        "Action": "s3:GetObject",
        "Resource": "arn:aws:s3:::my-bucket/*",
        "Condition": {
            "Bool": { "aws:SecureTransport": "false" }
        }
    }]
});

s3.put_bucket_policy("my-bucket", &policy.to_string()).await?;
```

### Remove a bucket policy

```rust
s3.delete_bucket_policy("my-bucket").await?;
```

## Public Access Block

### Block all public access

```rust
use aws_lite::types::s3::PublicAccessBlockConfiguration;

let config = PublicAccessBlockConfiguration {
    block_public_acls: Some(true),
    ignore_public_acls: Some(true),
    block_public_policy: Some(true),
    restrict_public_buckets: Some(true),
};

s3.put_public_access_block("my-bucket", &config).await?;
```

## Lifecycle Configuration

### Add lifecycle rules

```rust
use aws_lite::types::s3::*;

let lifecycle = BucketLifecycleConfiguration {
    rules: vec![LifecycleRule {
        id: Some("expire-logs".into()),
        status: "Enabled".into(),
        filter: Some(LifecycleRuleFilter {
            prefix: Some("logs/".into()),
            ..Default::default()
        }),
        expiration: Some(LifecycleExpiration {
            days: Some(90),
            ..Default::default()
        }),
        transitions: vec![Transition {
            days: Some(30),
            storage_class: Some("STANDARD_IA".into()),
            ..Default::default()
        }],
        ..Default::default()
    }],
};

s3.put_bucket_lifecycle_configuration("my-bucket", &lifecycle).await?;
```

## Testing

```rust
use aws_lite::{AwsHttpClient, MockClient};
use aws_lite::types::s3::*;

#[tokio::test]
async fn test_put_public_access_block() {
    let mut mock = MockClient::new();
    mock.expect_put("/my-bucket?publicAccessBlock")
        .returning_bytes(vec![]);

    let client = AwsHttpClient::from_mock(mock);
    let config = PublicAccessBlockConfiguration {
        block_public_acls: Some(true),
        ..Default::default()
    };

    let result = client.s3().put_public_access_block("my-bucket", &config).await;
    assert!(result.is_ok());
}
```
