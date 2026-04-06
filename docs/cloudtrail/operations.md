# AWS CloudTrail Operations

## Trail Discovery

### describe_trails

**Signature**: `pub async fn describe_trails(body: &DescribeTrailsRequest) -> Result<DescribeTrailsResponse>`

Returns trail configurations for the current region. Pass an empty request to list all trails, or filter by name/ARN.

| Field | Type | Description |
|-------|------|-------------|
| `trail_name_list` | `Vec<String>` | Names or ARNs to filter by (empty = all trails) |
| `include_shadow_trails` | `Option<bool>` | Include shadow trails from other regions (default: true) |

**Returns**: `DescribeTrailsResponse` with `trail_list: Vec<Trail>`

---

### get_trail_status

**Signature**: `pub async fn get_trail_status(body: &GetTrailStatusRequest) -> Result<GetTrailStatusResponse>`

Returns logging state and delivery metadata for a trail.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Trail name or ARN (required) |

**Returns**: `GetTrailStatusResponse` with `is_logging`, delivery timestamps, and error details.

**Errors**: `TrailNotFoundException` if the trail doesn't exist.

---

### get_event_selectors

**Signature**: `pub async fn get_event_selectors(body: &GetEventSelectorsRequest) -> Result<GetEventSelectorsResponse>`

Returns which event categories (management events, data events) a trail captures.

| Field | Type | Description |
|-------|------|-------------|
| `trail_name` | `String` | Trail name or ARN (required) |

**Returns**: `GetEventSelectorsResponse` with `trail_arn`, `event_selectors`, and `advanced_event_selectors`.

---

## Trail Management

### update_trail

**Signature**: `pub async fn update_trail(body: &UpdateTrailRequest) -> Result<UpdateTrailResponse>`

Modifies trail configuration. All fields except `name` are optional — only provided fields are changed.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Trail name or ARN (required) |
| `s3_bucket_name` | `Option<String>` | New S3 bucket for log delivery |
| `s3_key_prefix` | `Option<String>` | S3 key prefix for log files |
| `sns_topic_name` | `Option<String>` | SNS topic for notifications |
| `include_global_service_events` | `Option<bool>` | Capture IAM/STS/etc. events |
| `is_multi_region_trail` | `Option<bool>` | Replicate events to all regions |
| `enable_log_file_validation` | `Option<bool>` | Enable digest file creation |
| `cloud_watch_logs_log_group_arn` | `Option<String>` | CloudWatch Logs target ARN |
| `cloud_watch_logs_role_arn` | `Option<String>` | IAM role for CloudWatch delivery |
| `kms_key_id` | `Option<String>` | KMS key for log encryption |
| `is_organization_trail` | `Option<bool>` | Enable for AWS Organizations |

**Returns**: `UpdateTrailResponse` with the full updated trail configuration.

---

### delete_trail

**Signature**: `pub async fn delete_trail(body: &DeleteTrailRequest) -> Result<DeleteTrailResponse>`

Permanently deletes a trail. Does not delete the S3 bucket or existing log files.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Trail name or ARN (required) |

**Returns**: `DeleteTrailResponse` (empty body on success).

**Errors**: `TrailNotFoundException` if the trail doesn't exist.
