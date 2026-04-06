# AWS Secrets Manager Usage Examples

## Listing Secrets (Compliance Audit)

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::secretsmanager::ListSecretsRequest;

let client = AwsHttpClient::from_default_chain("us-east-1")?;

// List all secrets
let resp = client.secretsmanager().list_secrets(&ListSecretsRequest::default()).await?;

for secret in &resp.secret_list {
    println!("Secret: {}", secret.name.as_deref().unwrap_or("?"));
    println!("  ARN: {}", secret.arn.as_deref().unwrap_or("?"));
    println!("  Rotation: {:?}", secret.rotation_enabled);
    if let Some(rules) = &secret.rotation_rules {
        println!("  Rotation period: {:?} days", rules.automatically_after_days);
    }
}

// Find unrotated secrets (CIS compliance)
let unrotated: Vec<_> = resp.secret_list.iter()
    .filter(|s| s.rotation_enabled != Some(true))
    .collect();
println!("Secrets without rotation: {}", unrotated.len());
```

## Deleting a Secret

```rust
use aws_lite::types::secretsmanager::DeleteSecretRequest;

// Force-delete (no recovery window)
let resp = client.secretsmanager().delete_secret(&DeleteSecretRequest {
    secret_id: "my-old-secret".to_string(),
    force_delete_without_recovery: Some(true),
    ..Default::default()
}).await?;
println!("Deleted: {} (ARN: {})", resp.name.unwrap_or_default(), resp.arn.unwrap_or_default());

// Scheduled deletion with 7-day recovery window
client.secretsmanager().delete_secret(&DeleteSecretRequest {
    secret_id: "my-secret".to_string(),
    recovery_window_in_days: Some(7),
    ..Default::default()
}).await?;
```

## Triggering Secret Rotation

```rust
use aws_lite::types::secretsmanager::{RotateSecretRequest, RotationRulesType};

// Rotate immediately using the pre-configured Lambda
let resp = client.secretsmanager().rotate_secret(&RotateSecretRequest {
    secret_id: "my-db-password".to_string(),
    rotate_immediately: Some(true),
    ..Default::default()
}).await?;
println!("Rotated to version: {:?}", resp.version_id);

// Rotate with custom period
client.secretsmanager().rotate_secret(&RotateSecretRequest {
    secret_id: "my-api-key".to_string(),
    rotation_rules: Some(RotationRulesType {
        automatically_after_days: Some(30),
        ..Default::default()
    }),
    ..Default::default()
}).await?;
```

## Testing with MockClient

```rust
use aws_lite::{AwsHttpClient, MockClient};
use aws_lite::types::secretsmanager::*;

#[tokio::test]
async fn test_list_secrets() {
    let mut mock = MockClient::new();
    mock.expect_post("/").returning_json(serde_json::json!({
        "SecretList": [{
            "ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret-AbCd",
            "Name": "my-secret",
            "RotationEnabled": false,
            "CreatedDate": 1700000000.0
        }]
    }));
    let client = AwsHttpClient::from_mock(mock);
    let result = client.secretsmanager()
        .list_secrets(&ListSecretsRequest::default())
        .await
        .unwrap();
    assert_eq!(result.secret_list[0].name.as_deref(), Some("my-secret"));
    // Note: ARN field uses "ARN" (all-caps) in JSON — handled by serde_rename in generated types
}
```

## Important Notes

- **ARN format**: Secrets have a random suffix added to the name (e.g., `my-secret-AbCd`). Use the full ARN for deletion to avoid name collisions
- **Force-delete propagation**: After force-deletion, the name is unavailable for ~5 seconds — add a sleep before recreating with the same name
- **Rotation requires Lambda**: `RotateSecret` only works if a rotation Lambda is configured on the secret
