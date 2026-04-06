# AWS Codegen Reference

## Tools

### fetch — Download Botocore Models

```bash
cd codegen && uv run python -m codegen.cli fetch {service_name}
```

- Downloads the botocore `service-2.json` model for the specified AWS service
- Caches to `codegen/model_cache/{service}.service-2.json`
- Required before bootstrapping a new API

### bootstrap — Draft a New Manifest

```bash
cd codegen && uv run python -m codegen.cli bootstrap {service_name}
```

- Reads the cached botocore model
- Generates a draft manifest at `codegen/manifests/{service_name}.toml`
- Includes ALL shapes and operations from the model
- Developer then curates (selects which types/operations to include)
- Auto-detects wire format from botocore metadata (`protocol` field)

### extend — Browse, Add, and Diff Manifests

```bash
# Show what types/operations are available but not in the manifest
cd codegen && uv run python -m codegen.cli extend {api} --available-types
cd codegen && uv run python -m codegen.cli extend {api} --available-ops

# Generate ready-to-paste manifest entry
cd codegen && uv run python -m codegen.cli extend {api} --add-type ShapeName
cd codegen && uv run python -m codegen.cli extend {api} --add-op OperationName

# Diff manifest against botocore model
cd codegen && uv run python -m codegen.cli extend {api} --diff
```

### reference — Full Reference Manifests

```bash
cd codegen && uv run python -m codegen.cli reference               # Generate all
cd codegen && uv run python -m codegen.cli reference {api}          # Generate one API
```

Generates `codegen/reference/{api}.full.toml` with every shape and operation from the botocore model, annotated with coverage tags.

### verify — Validate Manifests

```bash
cd codegen && uv run python -m codegen.cli verify
```

- Validates all manifests against their botocore models
- Checks shape existence, field references
- Reports coverage statistics
- Runs `cargo check` and `cargo test`

### codegen CLI — Generate and Apply Rust Code

```bash
# Generate + apply (primary workflow):
cd codegen && uv run python -m codegen.cli apply

# Single API:
cd codegen && uv run python -m codegen.cli apply --api {service_name}

# Dry-run:
cd codegen && uv run python -m codegen.cli apply --dry-run
```

The pipeline:
1. **AWS plugin** reads manifest + botocore model, produces IR
2. **Shared emitter** converts IR to Rust source
3. **Apply** merges generated code into `src/`

## TOML Manifest Format

### [api] Section (Required)

```toml
[api]
name = "cloudwatch"                              # Module name (snake_case)
display_name = "Amazon CloudWatch"               # Human-readable name
version = "v1"                                   # Version label
api_version = "2010-08-01"                       # AWS API version string
service_name = "monitoring"                      # AWS service name (for SigV4)
wire_format = "query_xml"                        # Protocol: query_xml | json_target | json
endpoint_prefix = "monitoring"                   # URL prefix: {prefix}.{region}.amazonaws.com
doc_url = "https://docs.aws.amazon.com/..."      # Reference docs

# Required for json_target wire format:
target_prefix = "Logs_20140328"                  # X-Amz-Target prefix
json_version = "1.1"                             # JSON protocol version (1.0 or 1.1)
```

#### Wire Format Selection

| Botocore `protocol` | `wire_format` | Serde Naming | Request Format |
|---------------------|---------------|--------------|----------------|
| `query` | `query_xml` | PascalCase | Form-encoded body, XML response |
| `json` | `json_target` | camelCase | JSON body, X-Amz-Target header |
| `rest-json` | `json` | camelCase | JSON body |

### [api.client] Section (Required)

```toml
[api.client]
accessor_name = "cloudwatch"           # Method name on AwsHttpClient
client_struct = "CloudWatchClient"     # Rust struct name
```

### [[types]] Array

Each `[[types]]` entry defines a type to generate from a botocore shape.

```toml
[[types]]
shape = "GetMetricStatisticsOutput"    # Botocore shape name (exact match)
rust_name = "GetMetricStatisticsResponse"  # Optional: rename the Rust struct
include_fields = ["Label", "Datapoints"]   # Optional: whitelist fields

[types.field_overrides]
Unit = { enum_type = "StandardUnit" }  # Generate typed enum
Name = { required = true }             # Make field non-optional
```

**Field names use botocore casing** (PascalCase for query_xml, camelCase for json). The codegen handles snake_case conversion for Rust field names automatically.

#### Field Override Options

| Option | Type | Description |
|--------|------|-------------|
| `required` | `bool` | Make field non-optional |
| `enum_type` | `string` | Generate a typed enum for this field |
| `rust_type` | `string` | Force a specific Rust type |

#### Auto-Generated Types

Types referenced by operations (input shapes) are auto-generated even if not in the manifest. Structure dependencies (fields that reference other structures) are auto-discovered and generated transitively.

### [[operations]] Array

```toml
[[operations]]
name = "GetMetricStatistics"           # Botocore operation name (exact match)
rust_name = "get_metric_statistics"    # Rust function name
description = "Gets statistics for the specified metric."
```

Operations are simpler than GCP because:
- All AWS operations POST to `/` (no URL path construction)
- Action is determined by wire format (form body `Action=` or `X-Amz-Target` header)
- No LRO support needed
- No query params in URL (everything goes in the body)

## Wire Format Details

### query_xml Protocol

Request body is form-encoded:
```
Action=GetMetricStatistics&Version=2010-08-01&Namespace=AWS/EC2&MetricName=CPUUtilization&...
```

The `query.rs` module handles serialization:
- `build_query_body(action, version, body)` converts a struct to form params
- Nested structs use dotted notation: `Dimensions.member.1.Name=InstanceId`
- Lists use `.member.N` indexing

Response is XML wrapped in `<{Action}Response><{Action}Result>`:
```xml
<GetMetricStatisticsResponse>
  <GetMetricStatisticsResult>
    <Label>CPUUtilization</Label>
    <Datapoints>...</Datapoints>
  </GetMetricStatisticsResult>
</GetMetricStatisticsResponse>
```

The `xml.rs` module handles deserialization:
- `parse_xml_response::<T>(body)` strips wrapper elements and deserializes
- Uses `quick-xml` with serde

### json_target Protocol

Request:
```
POST / HTTP/1.1
Content-Type: application/x-amz-json-1.1
X-Amz-Target: Logs_20140328.DescribeLogGroups

{"logGroupNamePrefix": "test"}
```

Response: plain JSON body.

### json Protocol

Same as `json_target` but without `X-Amz-Target` header. Action is in the URL path for REST-style APIs.

## Examples

### CloudWatch (query_xml)

```toml
[api]
name = "cloudwatch"
wire_format = "query_xml"
endpoint_prefix = "monitoring"
service_name = "monitoring"
api_version = "2010-08-01"

[[types]]
shape = "Datapoint"
include_fields = ["Timestamp", "Average", "Sum", "Minimum", "Maximum", "SampleCount", "Unit"]
[types.field_overrides]
Unit = { enum_type = "StandardUnit" }

[[operations]]
name = "GetMetricStatistics"
rust_name = "get_metric_statistics"
```

### CloudWatch Logs (json_target)

```toml
[api]
name = "logs"
wire_format = "json_target"
target_prefix = "Logs_20140328"
json_version = "1.1"
endpoint_prefix = "logs"
service_name = "logs"

[[types]]
shape = "LogGroup"
include_fields = ["logGroupName", "arn", "creationTime", "storedBytes"]
[types.field_overrides]
logGroupClass = { enum_type = "LogGroupClass" }

[[operations]]
name = "DescribeLogGroups"
rust_name = "describe_log_groups"
```

## Naming Conventions

| Thing | Convention | Example |
|-------|-----------|---------|
| Module name | snake_case | `cloudwatch`, `logs` |
| Client struct | PascalCase + "Client" | `CloudWatchClient`, `LogsClient` |
| Accessor method | snake_case (short) | `client.cloudwatch()`, `client.logs()` |
| Operation function | snake_case | `get_metric_statistics`, `describe_log_groups` |
| Enum type | PascalCase | `StandardUnit`, `LogGroupClass` |
