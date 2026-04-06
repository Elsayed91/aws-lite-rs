# API Documentation Template

When adding a new API, create `docs/{api_name}/` with three files.

## docs/{api_name}/api.md

```markdown
# {Display Name} API

## Overview

Brief description of what this API does and when you'd use it.

## Client Access

```rust
let client = AwsHttpClient::new("eu-central-1").await?;
let {accessor} = client.{accessor_name}();
```

## Features

- List the key features/capabilities this client provides
- Note any convenience methods
- Note pagination stream support

## Types

| Type | Description |
|------|-------------|
| `{Type}` | Main resource type |
| `{ListType}` | List response wrapper |

## Error Handling

Common errors for this API:
- `AwsError::NotFound` — resource doesn't exist
- `AwsError::AccessDenied` — insufficient IAM permissions
- `AwsError::ServiceError` — general AWS error with code
```

## docs/{api_name}/operations.md

```markdown
# {Display Name} Operations

## {Resource Group 1}

### {method_name}

**Signature**: `pub async fn {method}({params}) -> Result<{ReturnType}>`

{Brief description}

| Parameter | Type | Description |
|-----------|------|-------------|
| `body` | `&{InputType}` | Request parameters |

**Returns**: `Result<{Type}>`

---
```

## docs/{api_name}/usage.md

```markdown
# {Display Name} Usage Examples

## Basic CRUD

### Create a resource

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::{api}::{Type};

let client = AwsHttpClient::new("eu-central-1").await?;

let input = {InputType} {
    name: Some("my-resource".to_string()),
    ..Default::default()
};

client.{api}().create_{resource}(&input).await?;
```

### List resources

```rust
let response = client.{api}().list_{resources}(&input).await?;
for item in &response.items {
    println!("{}", item.name.as_deref().unwrap_or("unnamed"));
}
```

## Testing

```rust
use aws_lite::{AwsHttpClient, MockClient};
use aws_lite::test_support::{ApiMockHelpers};
use aws_lite::types::{api}::*;

#[tokio::test]
async fn test_example() {
    let mut mock = MockClient::new();
    mock.expect_post("/")
        .returning_json(serde_json::to_value({Type}::fixture()).unwrap());

    let client = AwsHttpClient::from_mock(mock);
    let result = client.{api}().get_{resource}(&input).await;
    assert!(result.is_ok());
}
```
```
