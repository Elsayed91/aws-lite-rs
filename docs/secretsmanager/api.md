# AWS Secrets Manager API

## Overview

Secrets Manager stores and manages secrets (database credentials, API keys, etc.) with automatic rotation support. This client covers secret discovery, deletion, and rotation triggering.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("us-east-1")?;
let sm = client.secretsmanager();
```

## Features

- **List secrets** — enumerate all secrets with rotation metadata (for compliance auditing)
- **Delete secret** — immediate force-delete or scheduled deletion with recovery window
- **Rotate secret** — trigger secret rotation via configured Lambda function

## Wire Format

`json_target` — AWS Secrets Manager JSON 1.1 protocol with PascalCase field names. Requests use `X-Amz-Target` header with `secretsmanager.{Operation}`.

**Note**: The `ARN` field is returned in all-caps (`"ARN"`) by the API, unlike typical PascalCase conventions. The manifest handles this with an explicit `serde_rename`.

## Types

| Type | Description |
|------|-------------|
| `SecretListEntry` | Secret metadata: ARN, name, rotation status, timestamps |
| `RotationRulesType` | Rotation period and schedule expression |
| `DeleteSecretResponse` | ARN, name, and scheduled deletion date |
| `RotateSecretResponse` | ARN, name, and new version ID |

## Error Handling

Common errors:
- `ResourceNotFoundException` — secret does not exist
- `InvalidRequestException` — operation not valid (e.g., no rotation Lambda configured)
- `InvalidParameterException` — missing required parameter (e.g., `ClientRequestToken` for rotation)
- `ResourceExistsException` — secret name already in use (even if pending deletion)

## Notes

- **Deletion** has a recovery window (default 30 days) unless `force_delete_without_recovery: Some(true)` is used
- **Force-deleted secrets** take a few seconds to fully disappear from the account — subsequent `create-secret` calls with the same name may fail briefly after force-deletion
- **RotateSecret** requires a rotation Lambda ARN configured on the secret — calling without one returns `InvalidRequestException`
