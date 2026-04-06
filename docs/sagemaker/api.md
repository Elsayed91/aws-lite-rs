# Amazon SageMaker API

## Overview

The SageMaker API client provides access to Amazon SageMaker ML compute resource management.
It is primarily used for detecting and stopping idle notebook instances to reduce costs.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let sagemaker = client.sagemaker();
```

## Features

- `list_notebook_instances` — enumerate notebook instances with optional status/name filters
- `stop_notebook_instance` — stop a running notebook instance by name

## Wire Format

SageMaker uses `json_target` (JSON with `X-Amz-Target` header), `SageMaker` target prefix,
endpoint `api.sagemaker.{region}.amazonaws.com`.

## Types

| Type | Description |
|------|-------------|
| `ListNotebookInstancesInput` | Request for listing notebook instances (all fields optional) |
| `ListNotebookInstancesOutput` | Response containing `notebook_instances` and optional `next_token` |
| `NotebookInstanceSummary` | Per-instance summary with name, ARN, status, instance type, timestamps |
| `StopNotebookInstanceInput` | Request to stop a notebook (requires `notebook_instance_name`) |

## Error Handling

Common errors:
- `ServiceError(ValidationException)` — notebook instance name not found
- `ServiceError(AccessDeniedException)` — insufficient IAM permissions for SageMaker
