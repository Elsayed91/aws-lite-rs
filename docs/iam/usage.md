# IAM Usage Examples

## List Users

```rust
use aws_lite::AwsHttpClient;

let client = AwsHttpClient::from_default_chain("us-east-1")?;
let iam = client.iam();

let response = iam.list_users().await?;
for user in &response.users {
    println!("{} ({})", user.user_name, user.arn);
}
```

## List Attached Policies

```rust
let policies = iam.list_attached_user_policies("alice").await?;
for policy in &policies.attached_policies {
    println!(
        "{}: {}",
        policy.policy_name.as_deref().unwrap_or("?"),
        policy.policy_arn.as_deref().unwrap_or("?"),
    );
}
```

## Detach a Policy

```rust
iam.detach_user_policy("alice", "arn:aws:iam::aws:policy/ReadOnlyAccess").await?;
```

## Delete an Access Key

```rust
iam.delete_access_key("alice", "AKIAIOSFODNN7EXAMPLE").await?;
```

## Testing

```rust
use aws_lite::{AwsHttpClient, MockClient};
use aws_lite::test_support::iam_mock_helpers::IamMockHelpers;

#[tokio::test]
async fn test_list_users() {
    let mut mock = MockClient::new();
    mock.expect_list_users().returning_bytes(
        "<ListUsersResponse><ListUsersResult>\
           <Users><member>\
             <Arn>arn:aws:iam::123:user/test</Arn>\
             <UserName>test</UserName>\
             <CreateDate>2024-01-01T00:00:00Z</CreateDate>\
           </member></Users>\
         </ListUsersResult></ListUsersResponse>"
            .as_bytes()
            .to_vec(),
    );

    let client = AwsHttpClient::from_mock(mock);
    let response = client.iam().list_users().await.unwrap();
    assert_eq!(response.users.len(), 1);
}
```
