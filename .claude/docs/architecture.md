# AWS Architecture Reference

## Workspace Structure

```
src/
├── lib.rs              # Public exports
├── client.rs           # AwsHttpClient (SigV4, retry, rate limiting)
├── error.rs            # AwsError enum (XML + JSON parsing)
├── auth/
│   ├── mod.rs          # Auth module
│   ├── credentials.rs  # AwsCredentials (default chain, env vars)
│   └── sigv4.rs        # SigV4 request signing
├── query.rs            # Query/XML protocol body builder
├── xml.rs              # XML response parser
├── serde_base64.rs     # Base64 serde helpers
├── mock_client.rs      # MockClient for testing (feature-gated)
├── types/              # Generated types (NEVER edit)
│   ├── mod.rs
│   ├── cloudwatch.rs
│   └── logs.rs
├── ops/                # Generated operations (NEVER edit)
│   ├── mod.rs
│   ├── cloudwatch.rs
│   └── logs.rs
├── test_support/       # Generated mock helpers (NEVER edit)
│   ├── mod.rs
│   ├── cloudwatch_mock_helpers.rs
│   └── logs_mock_helpers.rs
└── api/                # Hand-written API wrappers (YES edit)
    ├── mod.rs
    ├── iam.rs
    ├── cloudwatch.rs
    └── logs.rs
```

## Three-Layer Design

### Layer 1: Types (`src/types/*.rs`)

**Owner**: Codegen (never edit manually)

Generated Rust structs and enums from AWS Botocore service-2.json models.

Properties:
- Derive `Default`, `Serialize`, `Deserialize`, `Debug`, `Clone`
- Serde rename depends on wire format:
  - `query_xml`: `#[serde(rename_all = "PascalCase")]` (CloudWatch, IAM, STS)
  - `json_target` / `json`: `#[serde(rename_all = "camelCase")]` (CloudWatch Logs, DynamoDB)
- `Option<T>` fields: `#[serde(skip_serializing_if = "Option::is_none")]`
- `Vec<T>` fields: `#[serde(default)]` + `#[serde(skip_serializing_if = "Vec::is_empty")]`
- Enums: typed with `#[serde(other)] Unknown` variant
- Every type gets a `fixture()` method for test data (feature-gated)

### Layer 2: Ops (`src/ops/*.rs`)

**Owner**: Codegen (never edit manually)

Raw HTTP operations encoding the correct wire format per API.

Properties:
- Visibility: `pub(crate)` (internal only)
- Each ops struct wraps a `&AwsHttpClient` reference
- Functions handle wire-format-specific request construction:
  - `query_xml`: form-encoded body (`Action=X&Version=Y&...`), XML response parsing
  - `json_target`: JSON body, `X-Amz-Target` header, JSON response
  - `json`: JSON body, JSON response
- All operations POST to `"/"` (AWS convention: action is in body/header, not URL)
- Regional base URL: `https://{endpoint_prefix}.{region}.amazonaws.com`

### Layer 3: API (`src/api/*.rs`)

**Owner**: Developer (hand-written, scaffolded once by codegen)

Thin wrappers adding ergonomic signatures.

Structure:
```rust
pub struct CloudWatchClient<'a> {
    ops: CloudwatchOps<'a>,
}

impl<'a> CloudWatchClient<'a> {
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self { ops: CloudwatchOps::new(client) }
    }

    pub async fn get_metric_statistics(
        &self, body: &GetMetricStatisticsInput,
    ) -> Result<GetMetricStatisticsResponse> {
        self.ops.get_metric_statistics(body).await
    }
}
```

### Layer 4: Test Support (`src/test_support/*.rs`)

**Owner**: Codegen (never edit manually)

MockClient extension traits for ergonomic test setup, same pattern as gcp-lite.

## Key Differences from GCP

| Aspect | GCP (gcp-lite) | AWS (aws-lite) |
|--------|----------------|----------------|
| **Auth** | OAuth2 / ADC | SigV4 request signing |
| **Base URL** | Per-API static URL | Regional: `{prefix}.{region}.amazonaws.com` |
| **Request format** | JSON body, action in URL path | POST to `/`, action in body or `X-Amz-Target` header |
| **Wire formats** | Always JSON | `query_xml`, `json_target`, `json` |
| **LROs** | Yes (selflink, name_based, etc.) | No (all operations are synchronous) |
| **Serde naming** | Always `camelCase` | `PascalCase` (query_xml) or `camelCase` (json) |
| **Error format** | JSON with `error.message` | XML `<ErrorResponse>` or JSON `{"__type": "..."}` |
| **Spec source** | GCP Discovery Documents | AWS Botocore service-2.json models |

## Wire Format Details

### `query_xml` (CloudWatch, IAM, STS, EC2)
- Request: form-encoded body (`Action=GetMetricStatistics&Version=2010-08-01&...`)
- Content-Type: `application/x-www-form-urlencoded`
- Response: XML wrapped in `<{Action}Response><{Action}Result>...</{Action}Result></...>`
- Serde: `#[serde(rename_all = "PascalCase")]`

### `json_target` (CloudWatch Logs, DynamoDB, KMS)
- Request: JSON body
- Headers: `Content-Type: application/x-amz-json-{version}`, `X-Amz-Target: {prefix}.{action}`
- Response: JSON
- Serde: `#[serde(rename_all = "camelCase")]`

### `json` (Lambda, API Gateway, newer APIs)
- Request: JSON body
- Content-Type: `application/json`
- Response: JSON
- Serde: `#[serde(rename_all = "camelCase")]`

## Module Registration

When a new API is added, register in two places:

### 1. `src/api/mod.rs`
```rust
pub mod cloudwatch;
pub mod new_api;  // add module

pub use cloudwatch::CloudWatchClient;
pub use new_api::NewApiClient;  // re-export client struct
```

### 2. `src/client.rs` (accessor method)
```rust
// Between the === Generated API Accessors === markers:
pub fn new_api(&self) -> crate::api::NewApiClient<'_> {
    crate::api::NewApiClient::new(self)
}
```

## Error Handling

`AwsError` enum (uses `thiserror`, derives `Clone`) with variants:
- `Auth` — 401/403 with expired or invalid credentials
- `AccessDenied` — 403 insufficient IAM permissions
- `NotFound` — 404 resource not found
- `Throttled` — 429 or throttling error code (with `retry_after`)
- `ServiceError` — General AWS error (code, message, status)
- `Network` — Connection/DNS failures
- `InvalidResponse` — Unparseable response
- `XmlParse` — XML parsing errors

Error parsing handles both XML (`<ErrorResponse><Error><Code>...</Code>`) and JSON (`{"__type": "...", "message": "..."}`) formats automatically based on response Content-Type.

Key methods: `is_retryable()`, `retry_after()`

## Codegen Pipeline

See `.claude/docs/codegen-reference.md` for the full pipeline, manifest format, and tools.
