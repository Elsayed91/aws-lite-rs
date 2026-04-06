# Amazon ECR Usage Examples

## List All Repositories

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::ecr::DescribeRepositoriesRequest;

let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let response = client.ecr().describe_repositories(&DescribeRepositoriesRequest::default()).await?;

for repo in &response.repositories {
    println!("{} (uri={})",
        repo.repository_name.as_deref().unwrap_or("?"),
        repo.repository_uri.as_deref().unwrap_or("?"));
}
```

## Describe a Specific Repository

```rust
use aws_lite::types::ecr::DescribeRepositoriesRequest;

let response = client.ecr().describe_repositories(&DescribeRepositoriesRequest {
    repository_names: vec!["my-app".to_string()],
    ..Default::default()
}).await?;
let repo = &response.repositories[0];
println!("ARN: {:?}", repo.repository_arn);
println!("URI: {:?}", repo.repository_uri);
```

## List Images in a Repository

```rust
use aws_lite::types::ecr::DescribeImagesRequest;

let response = client.ecr().describe_images(&DescribeImagesRequest {
    repository_name: "my-app".to_string(),
    ..Default::default()
}).await?;

for img in &response.image_details {
    println!("{} tags={:?} size={}",
        img.image_digest.as_deref().unwrap_or("?"),
        img.image_tags,
        img.image_size_in_bytes.unwrap_or(0));
}
```

## Set a Lifecycle Policy

```rust
use aws_lite::types::ecr::PutLifecyclePolicyRequest;

let policy = serde_json::json!({
    "rules": [{
        "rulePriority": 1,
        "description": "Expire untagged images older than 1 day",
        "selection": {
            "tagStatus": "untagged",
            "countType": "sinceImagePushed",
            "countUnit": "days",
            "countNumber": 1
        },
        "action": { "type": "expire" }
    }]
});

let response = client.ecr().put_lifecycle_policy(&PutLifecyclePolicyRequest {
    repository_name: "my-app".to_string(),
    lifecycle_policy_text: serde_json::to_string(&policy)?,
    ..Default::default()
}).await?;
println!("Policy set on {}", response.repository_name.as_deref().unwrap_or("?"));
```

## Delete Images by Digest

```rust
use aws_lite::types::ecr::{BatchDeleteImageRequest, ImageIdentifier};

let response = client.ecr().batch_delete_image(&BatchDeleteImageRequest {
    repository_name: "my-app".to_string(),
    image_ids: vec![
        ImageIdentifier {
            image_digest: Some("sha256:abc123...".to_string()),
            ..Default::default()
        },
    ],
    ..Default::default()
}).await?;

println!("Deleted {} image(s), {} failure(s)",
    response.image_ids.len(),
    response.failures.len());
```
