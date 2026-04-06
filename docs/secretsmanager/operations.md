# AWS Secrets Manager Operations

## Secret Discovery

### list_secrets

**Signature**: `pub async fn list_secrets(body: &ListSecretsRequest) -> Result<ListSecretsResponse>`

Lists all secrets in the current account and region. Supports sorting and pagination.

| Field | Type | Description |
|-------|------|-------------|
| `max_results` | `Option<i32>` | Max secrets per page |
| `next_token` | `Option<String>` | Pagination token |
| `sort_order` | `Option<String>` | `"asc"` or `"desc"` |
| `sort_by` | `Option<String>` | Field to sort by |

**Returns**: `ListSecretsResponse` with `secret_list: Vec<SecretListEntry>` and `next_token`.

Each `SecretListEntry` includes:
- `arn` — full ARN (e.g., `arn:aws:secretsmanager:...:secret:name-AbCd`)
- `name` — secret name
- `rotation_enabled` — whether auto-rotation is configured
- `rotation_rules` — rotation period (`automatically_after_days`) and schedule
- `last_rotated_date`, `last_changed_date`, `last_accessed_date`, `created_date` — Unix timestamps (f64)

---

## Secret Lifecycle

### delete_secret

**Signature**: `pub async fn delete_secret(body: &DeleteSecretRequest) -> Result<DeleteSecretResponse>`

Deletes a secret. By default, schedules deletion with a 30-day recovery window.

| Field | Type | Description |
|-------|------|-------------|
| `secret_id` | `String` | Secret name or ARN (required) |
| `recovery_window_in_days` | `Option<i64>` | Days before deletion (7-30, default 30) |
| `force_delete_without_recovery` | `Option<bool>` | Skip recovery window (immediate) |

**Returns**: `DeleteSecretResponse` with `arn`, `name`, `deletion_date`.

**Errors**: `ResourceNotFoundException` if secret doesn't exist.

---

## Secret Rotation

### rotate_secret

**Signature**: `pub async fn rotate_secret(body: &RotateSecretRequest) -> Result<RotateSecretResponse>`

Triggers rotation of a secret using the configured Lambda rotation function.

| Field | Type | Description |
|-------|------|-------------|
| `secret_id` | `String` | Secret name or ARN (required) |
| `rotation_lambda_arn` | `Option<String>` | Lambda ARN (required if not pre-configured) |
| `rotation_rules` | `Option<RotationRulesType>` | Override rotation schedule |
| `rotate_immediately` | `Option<bool>` | Rotate immediately vs next scheduled time |

**Returns**: `RotateSecretResponse` with `arn`, `name`, `version_id`.

**Errors**:
- `InvalidRequestException` — no rotation Lambda configured
- `InvalidParameterException` — missing `ClientRequestToken`
