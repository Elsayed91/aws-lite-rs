# AWS KMS Usage Examples

## Listing Keys

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::kms::ListKeysRequest;

let client = AwsHttpClient::from_default_chain("us-east-1")?;

// List all keys (first page)
let resp = client.kms().list_keys(&ListKeysRequest::default()).await?;
for key in &resp.keys {
    println!("{}: {}", key.key_id.as_deref().unwrap_or("?"), key.key_arn.as_deref().unwrap_or("?"));
}

// Paginate if truncated
if resp.truncated == Some(true) {
    let next = client.kms().list_keys(&ListKeysRequest {
        marker: resp.next_marker.clone(),
        ..Default::default()
    }).await?;
    // process next page...
}
```

## Describing a Key

```rust
use aws_lite::types::kms::DescribeKeyRequest;

let resp = client.kms().describe_key(&DescribeKeyRequest {
    key_id: "arn:aws:kms:us-east-1:123456789012:key/abc-123".to_string(),
}).await?;

if let Some(meta) = resp.key_metadata {
    println!("State: {:?}", meta.key_state);
    println!("Manager: {:?}", meta.key_manager);
    println!("Enabled: {:?}", meta.enabled);
}
```

## Checking Rotation Status (CIS Compliance)

```rust
use aws_lite::types::kms::GetKeyRotationStatusRequest;

let status = client.kms().get_key_rotation_status(&GetKeyRotationStatusRequest {
    key_id: "abc-123".to_string(),
}).await?;

if status.key_rotation_enabled == Some(false) {
    println!("WARNING: Key rotation is disabled — CIS violation");
}
```

## Enabling Key Rotation

```rust
use aws_lite::types::kms::EnableKeyRotationRequest;

// Enable with default 365-day period
client.kms().enable_key_rotation(&EnableKeyRotationRequest {
    key_id: "abc-123".to_string(),
    ..Default::default()
}).await?;

// Enable with custom 180-day rotation
client.kms().enable_key_rotation(&EnableKeyRotationRequest {
    key_id: "abc-123".to_string(),
    rotation_period_in_days: Some(180),
}).await?;
```

## Testing with MockClient

```rust
use aws_lite::{AwsHttpClient, MockClient};
use aws_lite::types::kms::*;

#[tokio::test]
async fn test_list_keys() {
    let mut mock = MockClient::new();
    mock.expect_post("/").returning_json(serde_json::json!({
        "Keys": [
            {
                "KeyId": "abc-123",
                "KeyArn": "arn:aws:kms:us-east-1:123456789012:key/abc-123"
            }
        ],
        "Truncated": false
    }));
    let client = AwsHttpClient::from_mock(mock);
    let result = client.kms().list_keys(&ListKeysRequest::default()).await.unwrap();
    assert_eq!(result.keys.len(), 1);
    assert_eq!(result.keys[0].key_id.as_deref(), Some("abc-123"));
}
```

## Important Notes

- **AWS-managed keys** (KeyManager = "AWS") do NOT support `EnableKeyRotation` or `GetKeyRotationStatus` via this API
- **Key deletion** is deferred — use the AWS CLI `schedule-key-deletion` with a 7-30 day waiting period; there is no immediate delete
- **Key IDs** can be provided as key ID, full ARN, alias ARN (`arn:aws:kms:...:alias/...`), or alias name (`alias/my-key`)
