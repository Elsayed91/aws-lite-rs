# AWS Config API

## Overview

AWS Config provides a SQL-like query interface (`SelectResourceConfig`) for querying resources discovered by AWS Config. Returns raw JSON strings representing resource configurations.

## Client Access

```rust
let client = AwsHttpClient::builder()
    .credentials(AwsCredentials::from_default_chain("us-east-1")?)
    .region("us-east-1")
    .build()?;
let config = client.config();
```

## Features

- SQL-like resource queries via `SelectResourceConfig`
- Automatic pagination with `select_resource_config_stream()`
- Returns raw JSON strings (not typed objects) for maximum flexibility

## Types

| Type | Description |
|------|-------------|
| `SelectResourceConfigRequest` | Query request with SQL expression, limit, and pagination token |
| `SelectResourceConfigResponse` | Response with results (JSON strings), query info, and next token |
| `QueryInfo` | Metadata about the query (selected fields) |
| `FieldInfo` | Individual field metadata (name) |

## Error Handling

Common errors for this API:
- `AwsError::ServiceError` with code `InvalidExpressionException` — malformed SQL expression
- `AwsError::ServiceError` with code `InvalidLimitException` — invalid limit value
- `AwsError::ServiceError` with code `InvalidNextTokenException` — invalid pagination token
- `AwsError::AccessDenied` — insufficient IAM permissions
