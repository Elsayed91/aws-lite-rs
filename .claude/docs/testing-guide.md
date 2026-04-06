# AWS Testing Guide

## Integration-First Development

**Core principle**: Integration tests drive API development. Unit tests encode proven behavior.

For the full methodology (structure, fixture management, CLI helpers, checklist), see **[`integration-testing-methodology.md`](integration-testing-methodology.md)**.

AWS APIs have their own quirks that only integration tests catch:
- Wire format differences affect serialization (PascalCase vs camelCase, XML vs JSON)
- SigV4 signing must include the correct service name and region
- Error responses differ by wire format (XML `<ErrorResponse>` vs JSON `{"__type": "..."}`)
- Some operations require specific header combinations (`X-Amz-Target`, `Content-Type`)
- Some APIs use unexpected conventions (Config uses PascalCase JSON despite `Aws_json1.1`)

### Proven by Integration Testing

| API | Finding | Would unit tests have caught it? |
|-----|---------|----------------------------------|
| Config | SQL `LIMIT` rejected — must use request body `limit` | No |
| Config | `SELECT *` returns only metadata fields (no `configuration`, `tags`) | No |
| Config | Invalid column names silently succeed | No |

## Authentication Setup

AWS uses the default credential chain:

```bash
# Option 1: AWS CLI credentials
aws configure

# Option 2: Environment variables
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...
export AWS_DEFAULT_REGION=eu-central-1

# Option 3: SSO login
aws sso login --profile your-profile
```

The `AwsHttpClient` loads credentials via `AwsCredentials::from_default_chain(region)`.

## Running AWS Tests

```bash
# All unit tests + generated tests
cargo nextest run --lib

# Single API integration tests
cargo nextest run --test integration config -- --ignored --test-threads=1 --nocapture

# All integration tests
cargo nextest run --test integration -- --ignored --test-threads=1 --nocapture
```

## Test Categories

### 1. Generated Operation Tests (in `src/ops/{api}.rs`)

Auto-generated tests validating URL, HTTP method, and serialization for each operation. Created by codegen — never edit manually.

For `query_xml` APIs, tests verify form-encoded body construction and XML response parsing.
For `json_target` APIs, tests verify JSON body serialization and response parsing.

### 2. Unit Tests (in `src/api/{api}.rs`)

Hand-written tests in `#[cfg(test)] mod tests` blocks. **Only written AFTER integration tests pass.**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_client::MockClient;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn select_resource_config_stream_paginates() {
        let mut mock = MockClient::new();
        mock.expect_post("/")
            .returning_json_sequence(vec![
                serde_json::json!({
                    "Results": ["{\"resourceId\":\"vol-1\"}"],
                    "NextToken": "token-page2"
                }),
                serde_json::json!({
                    "Results": ["{\"resourceId\":\"vol-2\"}"]
                }),
            ])
            .times(2);

        let client = AwsHttpClient::from_mock(mock);
        let results: Vec<String> = client.config()
            .select_resource_config_stream("SELECT resourceId")
            .map(|r| r.unwrap())
            .collect()
            .await;

        assert_eq!(results.len(), 2);
        assert!(results[0].contains("vol-1"));
    }
}
```

### 3. Integration Tests (in `tests/integration/`)

Real AWS API calls. Follow the **[shared methodology](integration-testing-methodology.md)** exactly.

**CRITICAL — Read the Fixture Rule in the shared methodology.** Every prerequisite resource MUST be created by the test. Use library clients when the library supports it, fall back to AWS CLI when it doesn't. Never query pre-existing account resources and silently pass when none exist.

#### Fixture Creation

See [shared methodology](integration-testing-methodology.md#fixture-management-library-first-cli-as-fallback) for the Fixture Rule, CLI helper patterns, and rules.

#### AWS CLI Helper Template

```rust
fn aws(args: &[&str]) -> Option<serde_json::Value> {
    let output = Command::new("aws")
        .args(args)
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(&stdout).ok()
    } else {
        None
    }
}

fn aws_cleanup(args: &[&str]) {
    let _ = Command::new("aws").args(args).output();
}
```

#### Pinning Async Streams in Tests

`async_stream` types are not `Unpin`. Use `std::pin::pin!` for `while let` iteration:

```rust
use std::pin::pin;
use tokio_stream::StreamExt;

let mut stream = pin!(client.config().select_resource_config_stream("SELECT ..."));
while let Some(result) = stream.next().await {
    // ...
}
```

#### Full Integration Test Template (AWS)

```rust
//! Integration tests for {API_NAME}
//!
//! Run with:
//!   cargo test --test integration {api} -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_REGION: &str = "eu-central-1";
const TEST_RESOURCE_NAME: &str = "cloud-lite-test-{api}-resource";

fn aws_create_resource(region: &str) -> Option<String> {
    let output = Command::new("aws")
        .args([
            "{service}", "create-{resource}",
            "--name", TEST_RESOURCE_NAME,
            "--region", region,
            "--output", "json",
        ])
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str::<serde_json::Value>(&stdout)
            .ok()
            .and_then(|v| v["{ResourceArn}"].as_str().map(String::from))
    } else {
        None
    }
}

fn aws_delete_resource(arn: &str, region: &str) {
    let _ = Command::new("aws")
        .args(["{service}", "delete-{resource}", "--{resource}-arn", arn, "--region", region])
        .output();
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_{api}_lifecycle() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    // Pre-cleanup
    aws_delete_resource(TEST_RESOURCE_NAME, &region);
    tokio::time::sleep(Duration::from_secs(2)).await;

    let client = AwsHttpClient::new(&region).await.expect("AWS credentials required");

    // Run tests, always cleanup
    let result = run_lifecycle_tests(&client, &region).await;

    // Always cleanup
    aws_delete_resource(TEST_RESOURCE_NAME, &region);

    // Propagate after cleanup
    result.expect("{api} lifecycle tests failed");
    println!("\nAll {api} tests passed!");
}

async fn run_lifecycle_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== {API_NAME} Lifecycle Test ===");
    println!("Region: {region}");

    // [0/N] Create fixture (via library if supported, CLI if not)
    println!("\n[0/6] Creating test resource...");
    // Use library OR CLI — but ALWAYS create the resource
    let arn = aws_create_resource(region).expect("fixture creation failed");
    println!("  Created: {arn}");
    tokio::time::sleep(Duration::from_secs(3)).await;

    // [1/N] Get
    println!("\n[1/6] Getting resource...");
    // ...

    // [2/N] List
    println!("\n[2/6] Listing resources...");
    // ...

    // [3/N] Pagination (force with limit=1)
    println!("\n[3/6] Testing pagination (limit=1)...");
    // ...

    // [4/N] Delete
    println!("\n[4/6] Deleting resource...");
    // ...

    // [5/N] Verify deleted
    println!("\n[5/6] Verifying deletion...");
    // ...

    // [6/N] Error case — non-existent resource
    println!("\n[6/6] Getting non-existent resource (expect error)...");
    // let err = result.unwrap_err();
    // assert!(format!("{err}").contains("SpecificErrorCode"),
    //     "Expected SpecificErrorCode, got: {err}");

    Ok(())
}
```

#### AWS Integration Test Checklist

See [shared methodology](integration-testing-methodology.md#what-to-test-comprehensive-checklist) for the full checklist. AWS-specific additions:
- Tests pagination with limit=1 (verify page 2 differs)
- Tests error cases with **specific error code assertions** (not just `is_err()`)
- Verifies wire-format-specific request construction
- Tests both XML and JSON error response parsing where applicable

## Wire Format Testing Considerations

### query_xml APIs (CloudWatch, IAM)
- Form-encoded body must use correct PascalCase parameter names
- XML response parsing must handle wrapper elements (`<ActionResponse><ActionResult>`)
- Mock tests return XML wrapped in the expected envelope

### json_target APIs (CloudWatch Logs, Config)
- JSON body field casing depends on the service:
  - Logs: camelCase (`logGroupName`, `nextToken`)
  - Config: PascalCase (`Expression`, `NextToken`, `Results`)
- Mock tests return plain JSON with the correct casing
- `X-Amz-Target` header must match `{target_prefix}.{OperationName}`

### Error Response Testing
- XML APIs: `<ErrorResponse><Error><Code>...</Code><Message>...</Message></Error></ErrorResponse>`
- JSON APIs: `{"__type": "ExceptionType", "message": "..."}`
- Both formats are tested in `src/error.rs`

## Current Test Coverage

| Category | Status |
|----------|--------|
| Generated op tests | Yes (cloudwatch, config, logs) |
| Unit tests (hand-written) | Config (pagination stream), Logs (pagination stream) |
| Integration tests | CloudWatch (metric stats), Config (comprehensive), Logs (placeholder) |
| Mock integration tests | Not yet |
| Property tests | Not yet |
| Snapshot tests | Not yet |
