# AWS Lambda API

## Overview

AWS Lambda lets you run code without provisioning or managing servers. This client provides access to Lambda function management operations: listing functions, retrieving function configuration, and updating function settings (timeout, memory, runtime, architecture).

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let lambda = client.lambda();
```

## Features

- List all Lambda functions in the account/region
- Get detailed configuration for a specific function
- Update function configuration (timeout, memory size, runtime, architectures)
- REST-JSON wire format: path-based routing with JSON request/response bodies
- SigV4-signed requests with automatic retry and rate limiting

## Types

| Type | Description |
|------|-------------|
| `FunctionConfiguration` | Function configuration details (name, ARN, runtime, timeout, memory, etc.) |
| `ListFunctionsResponse` | Response from ListFunctions (list of FunctionConfiguration + pagination token) |
| `GetFunctionConfigurationRequest` | Request to get a specific function's config |
| `UpdateFunctionConfigurationRequest` | Request to update a function's configuration |
| `EnvironmentResponse` | Function environment variables |
| `VpcConfigResponse` | VPC networking configuration for a function |

## Error Handling

Common errors for this API:
- `AwsError::NotFound` — function doesn't exist (404, `ResourceNotFoundException`)
- `AwsError::AccessDenied` — insufficient IAM permissions
- `AwsError::ServiceError` — Lambda-specific errors (e.g., `InvalidParameterValueException`)
