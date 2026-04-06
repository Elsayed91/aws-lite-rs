# AWS STS Operations

## Credential Validation

### get_caller_identity

**Signature**: `pub async fn get_caller_identity(&self) -> Result<GetCallerIdentityResponse>`

Returns details about the IAM user or role whose credentials are used to call the operation. Takes no parameters — useful as a credential validation preflight check.

**Returns**: `Result<GetCallerIdentityResponse>` with fields:
- `account` — AWS account ID (numeric string)
- `arn` — Full ARN of the caller
- `user_id` — Unique identifier of the caller

---

## Cross-Account Access

### assume_role

**Signature**: `pub async fn assume_role(&self, body: &AssumeRoleRequest) -> Result<AssumeRoleResponse>`

Returns temporary security credentials for assuming an IAM role, typically for cross-account access.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.role_arn` | `String` | ARN of the role to assume (required) |
| `body.role_session_name` | `String` | Session identifier (required) |
| `body.external_id` | `Option<String>` | External ID for third-party access |

**Returns**: `Result<AssumeRoleResponse>` with:
- `credentials` — Temporary credentials (access key, secret key, session token, expiration)
- `assumed_role_user` — Identity of the assumed role (ARN and role ID)
