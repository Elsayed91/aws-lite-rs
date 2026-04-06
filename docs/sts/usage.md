# AWS STS Usage Examples

## Validate Credentials

```rust
use aws_lite::AwsHttpClient;

let client = AwsHttpClient::from_default_chain("us-east-1")?;
let response = client.sts().get_caller_identity().await?;

println!("Account: {}", response.account.unwrap_or_default());
println!("ARN: {}", response.arn.unwrap_or_default());
println!("UserID: {}", response.user_id.unwrap_or_default());
```

## Assume a Role

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::sts::AssumeRoleRequest;

let client = AwsHttpClient::from_default_chain("us-east-1")?;

let request = AssumeRoleRequest {
    role_arn: "arn:aws:iam::123456789012:role/CrossAccountRole".to_string(),
    role_session_name: "my-session".to_string(),
    ..Default::default()
};

let response = client.sts().assume_role(&request).await?;
let creds = response.credentials.expect("credentials present");

println!("Access Key: {}", creds.access_key_id);
println!("Expires: {}", creds.expiration);
```

## Testing

```rust
use aws_lite::{AwsHttpClient, mock_client::MockClient};
use aws_lite::test_support::sts_mock_helpers::StsMockHelpers;

#[tokio::test]
async fn test_get_caller_identity() {
    let mut mock = MockClient::new();
    mock.expect_get_caller_identity()
        .returning_bytes(
            "<GetCallerIdentityResponse><GetCallerIdentityResult>\
             <Account>123456789012</Account>\
             <Arn>arn:aws:iam::123456789012:user/test</Arn>\
             <UserId>AIDAEXAMPLE</UserId>\
             </GetCallerIdentityResult></GetCallerIdentityResponse>"
                .as_bytes().to_vec(),
        );

    let client = AwsHttpClient::from_mock(mock);
    let response = client.sts().get_caller_identity().await.unwrap();
    assert_eq!(response.account.as_deref(), Some("123456789012"));
}
```
