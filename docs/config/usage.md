# AWS Config Usage Examples

## Query EC2 Volumes

```rust
use aws_lite::{AwsHttpClient, auth::AwsCredentials};
use aws_lite::types::config::SelectResourceConfigRequest;

let client = AwsHttpClient::builder()
    .credentials(AwsCredentials::from_default_chain("us-east-1")?)
    .region("us-east-1")
    .build()?;

let request = SelectResourceConfigRequest {
    expression: "SELECT resourceId, resourceType, configuration, tags \
                 WHERE resourceType = 'AWS::EC2::Volume'".into(),
    ..Default::default()
};

let response = client.config().select_resource_config(&request).await?;
for result in &response.results {
    println!("{result}");
}
```

## Stream All Results with Pagination

```rust
use tokio_stream::StreamExt;

use std::pin::pin;

let mut stream = pin!(client.config().select_resource_config_stream(
    "SELECT resourceId, resourceType, accountId, awsRegion \
     WHERE resourceType = 'AWS::EC2::Volume'"
));

while let Some(result) = stream.next().await {
    let json_str = result?;
    println!("{json_str}");
}
```

## Testing with MockClient

```rust
use aws_lite::{AwsHttpClient, MockClient};

#[tokio::test]
async fn test_config_query() {
    let mut mock = MockClient::new();
    mock.expect_post("/").returning_json(serde_json::json!({
        "Results": [
            "{\"resourceId\":\"vol-123\",\"resourceType\":\"AWS::EC2::Volume\"}"
        ]
    }));

    let client = AwsHttpClient::from_mock(mock);
    let response = client.config()
        .select_resource_config(&SelectResourceConfigRequest {
            expression: "SELECT resourceId WHERE resourceType = 'AWS::EC2::Volume'".into(),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(response.results.len(), 1);
}
```
