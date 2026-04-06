# Amazon CloudFront Operations

## Read Operations

### list_distributions

**Signature**: `pub async fn list_distributions(&self) -> Result<DistributionList>`

Lists all CloudFront distributions in the account.

**Returns**: `Result<DistributionList>` with fields:
- `quantity` — Number of distributions
- `is_truncated` — Whether more results are available
- `marker` / `next_marker` — Pagination markers
- `items` — Vec of `DistributionSummary`

---

### get_distribution_config

**Signature**: `pub async fn get_distribution_config(&self, id: &str) -> Result<DistributionConfig>`

Gets the configuration for a specific distribution.

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | `&str` | Distribution ID (e.g., `E1ABC2DEF3GHIJ`) |

**Returns**: `Result<DistributionConfig>` with fields:
- `caller_reference` — Unique reference for the distribution
- `origins` — Origin configurations
- `default_cache_behavior` — Default cache behavior settings
- `comment` — Distribution description
- `enabled` — Whether the distribution is active

---

## Write Operations

### create_origin_access_control

**Signature**: `pub async fn create_origin_access_control(&self, body: &OriginAccessControlConfig) -> Result<OriginAccessControl>`

Creates a new origin access control for restricting S3 origin access.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.name` | `String` | Unique name for the OAC (required) |
| `body.description` | `Option<String>` | Description |
| `body.signing_protocol` | `String` | Signing protocol, typically `"sigv4"` |
| `body.signing_behavior` | `String` | When to sign: `"always"`, `"never"`, or `"no-override"` |
| `body.origin_access_control_origin_type` | `String` | Origin type: `"s3"`, `"mediastore"`, etc. |

**Returns**: `Result<OriginAccessControl>` with the created OAC including its assigned ID.

---

### create_distribution

**Signature**: `pub async fn create_distribution(&self, body: &DistributionConfig) -> Result<Distribution>`

Creates a new CloudFront distribution.

**Status**: Blocked — REST-XML Vec serialization issue prevents correct request body generation for types with list fields (Origins).

---

### update_distribution

**Signature**: `pub async fn update_distribution(&self, id: &str, body: &DistributionConfig) -> Result<Distribution>`

Updates an existing CloudFront distribution configuration.

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | `&str` | Distribution ID |
| `body` | `&DistributionConfig` | Updated configuration |

**Status**: Blocked — requires both REST-XML Vec serialization fix and ETag/If-Match header support.
