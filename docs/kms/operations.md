# AWS KMS Operations

## Key Discovery

### list_keys

**Signature**: `pub async fn list_keys(body: &ListKeysRequest) -> Result<ListKeysResponse>`

Lists all KMS key IDs and ARNs in the current account and region. Supports pagination.

| Field | Type | Description |
|-------|------|-------------|
| `limit` | `Option<i32>` | Max results per page (1-1000) |
| `marker` | `Option<String>` | Pagination token from a previous truncated response |

**Returns**: `ListKeysResponse` with `keys: Vec<KeyListEntry>`, `next_marker`, `truncated`.

---

### describe_key

**Signature**: `pub async fn describe_key(body: &DescribeKeyRequest) -> Result<DescribeKeyResponse>`

Returns full metadata for a KMS key: state, manager, spec, creation date, and more.

| Field | Type | Description |
|-------|------|-------------|
| `key_id` | `String` | Key ID, ARN, alias ARN, or alias name (required) |

**Returns**: `DescribeKeyResponse` with `key_metadata: Option<KeyMetadata>`.

**Errors**: `NotFoundException` if key doesn't exist.

---

## Key Rotation

### get_key_rotation_status

**Signature**: `pub async fn get_key_rotation_status(body: &GetKeyRotationStatusRequest) -> Result<GetKeyRotationStatusResponse>`

Returns whether automatic rotation is enabled for a symmetric customer-managed key.

| Field | Type | Description |
|-------|------|-------------|
| `key_id` | `String` | Key ID, ARN, alias ARN, or alias name (required) |

**Returns**: `GetKeyRotationStatusResponse` with:
- `key_rotation_enabled: Option<bool>` — true if automatic rotation is on
- `rotation_period_in_days: Option<i32>` — rotation period (default 365 when enabled)
- `next_rotation_date: Option<f64>` — Unix timestamp of next scheduled rotation

**Errors**: `UnsupportedOperationException` for AWS-managed or asymmetric keys.

---

### enable_key_rotation

**Signature**: `pub async fn enable_key_rotation(body: &EnableKeyRotationRequest) -> Result<()>`

Enables automatic annual rotation of key material for a symmetric customer-managed key.

| Field | Type | Description |
|-------|------|-------------|
| `key_id` | `String` | Key ID, ARN, alias ARN, or alias name (required) |
| `rotation_period_in_days` | `Option<i32>` | Custom rotation period (90-2560 days, default 365) |

**Returns**: `()` on success (empty response body).

**Errors**:
- `DisabledException` — key must be enabled to rotate
- `KMSInvalidStateException` — key in wrong state
- `UnsupportedOperationException` — only works on symmetric CMKs
