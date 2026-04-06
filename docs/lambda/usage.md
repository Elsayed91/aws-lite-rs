# AWS Lambda Usage Examples

## List All Functions

```rust
use aws_lite::AwsHttpClient;

let client = AwsHttpClient::from_default_chain("eu-central-1")?;

let resp = client.lambda().list_functions("", "", "", "").await?;
for func in &resp.functions {
    println!(
        "Function: {} (runtime={:?}, timeout={:?}s, memory={:?}MB, arch={:?})",
        func.function_name,
        func.runtime,
        func.timeout,
        func.memory_size,
        func.architectures,
    );
}
if let Some(marker) = &resp.next_marker {
    println!("More functions available, use marker: {marker}");
}
```

## Get Function Configuration

```rust
let config = client
    .lambda()
    .get_function_configuration("my-function", "")
    .await?;

println!("Function ARN: {}", config.function_arn);
println!("Runtime: {:?}", config.runtime);
println!("Timeout: {:?}s", config.timeout);
println!("Memory: {:?}MB", config.memory_size);

if let Some(Some(env)) = &config.environment {
    for (key, value) in &env.variables {
        println!("  Env: {}={}", key, value);
    }
}
```

## Update Function Configuration

```rust
use aws_lite::types::lambda::UpdateFunctionConfigurationRequest;

// Increase timeout and memory
let updated = client
    .lambda()
    .update_function_configuration(
        "my-function",
        &UpdateFunctionConfigurationRequest {
            function_name: "my-function".to_string(),
            timeout: Some(60),          // 60 seconds
            memory_size: Some(512),     // 512 MB
            runtime: Some("python3.12".to_string()),
            ..Default::default()
        },
    )
    .await?;

println!("Updated function: {}", updated.function_name);
println!("New timeout: {:?}s", updated.timeout);
println!("New memory: {:?}MB", updated.memory_size);
```

## Paginate Through All Functions

```rust
let mut marker = String::new();
let mut all_functions = Vec::new();

loop {
    let resp = client
        .lambda()
        .list_functions("", "", &marker, "50")
        .await?;

    all_functions.extend(resp.functions);

    match resp.next_marker {
        Some(next) => marker = next,
        None => break,
    }
}

println!("Total functions: {}", all_functions.len());
```

## Testing

```rust
use aws_lite::{AwsHttpClient, MockClient};
use aws_lite::types::lambda::*;

#[tokio::test]
async fn test_list_functions() {
    let mut mock = MockClient::new();
    mock.expect_get("/2015-03-31/functions")
        .returning_json(serde_json::json!({
            "Functions": [{
                "FunctionName": "my-function",
                "FunctionArn": "arn:aws:lambda:eu-central-1:123456789012:function:my-function",
                "Runtime": "python3.12",
                "Timeout": 30,
                "MemorySize": 256,
                "Architectures": ["x86_64"]
            }]
        }));

    let client = AwsHttpClient::from_mock(mock);
    let result = client.lambda().list_functions("", "", "", "").await.unwrap();

    assert_eq!(result.functions.len(), 1);
    assert_eq!(result.functions[0].function_name, "my-function");
    assert_eq!(result.functions[0].timeout, Some(30));
}
```
