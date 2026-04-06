# AWS Lambda Operations

## Functions

### list_functions

**Signature**: `pub async fn list_functions(master_region: &str, function_version: &str, marker: &str, max_items: &str) -> Result<ListFunctionsResponse>`

Returns a list of Lambda functions in the current region.

| Parameter | Type | Description |
|-----------|------|-------------|
| `master_region` | `&str` | For Lambda@Edge functions, the Region of the master function (empty for standard) |
| `function_version` | `&str` | Set to `"ALL"` to include all published versions, or empty for latest |
| `marker` | `&str` | Pagination token from a previous request (empty to start from beginning) |
| `max_items` | `&str` | Maximum number of functions to return (empty for default, up to 50) |

**Returns**: `Result<ListFunctionsResponse>` — list of `FunctionConfiguration` objects and optional `NextMarker` pagination token.

---

### get_function_configuration

**Signature**: `pub async fn get_function_configuration(function_name: &str, qualifier: &str) -> Result<FunctionConfiguration>`

Returns the version-specific settings of a Lambda function or version.

| Parameter | Type | Description |
|-----------|------|-------------|
| `function_name` | `&str` | Function name, ARN, or partial ARN (required) |
| `qualifier` | `&str` | Version number or alias name (empty for `$LATEST`) |

**Returns**: `Result<FunctionConfiguration>` — detailed function configuration.

---

### update_function_configuration

**Signature**: `pub async fn update_function_configuration(function_name: &str, body: &UpdateFunctionConfigurationRequest) -> Result<FunctionConfiguration>`

Modifies the version-specific settings of a Lambda function. Changes apply asynchronously.

| Parameter | Type | Description |
|-----------|------|-------------|
| `function_name` | `&str` | Function name or ARN (required, path parameter) |
| `body` | `&UpdateFunctionConfigurationRequest` | Fields to update (all optional except `function_name`) |

**Body fields** (`UpdateFunctionConfigurationRequest`):

| Field | Type | Description |
|-------|------|-------------|
| `function_name` | `String` | Function name (required, also sent in URL path) |
| `timeout` | `Option<i32>` | Max execution time in seconds (1–900) |
| `memory_size` | `Option<i32>` | Memory in MB (128–10240, multiples of 1 MB) |
| `runtime` | `Option<String>` | Runtime identifier (e.g., `"python3.12"`, `"nodejs20.x"`) |

**Returns**: `Result<FunctionConfiguration>` — updated function configuration.
