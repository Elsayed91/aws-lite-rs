# AWS CloudTrail API

## Overview

CloudTrail records API calls and events across your AWS account for auditing and compliance. This client provides read and write access to trail configuration: describing trails, checking logging status, reading event selectors, and managing trail lifecycle (update, delete).

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("us-east-1")?;
let cloudtrail = client.cloudtrail();
```

## Features

- **Describe trails** — list trail configurations in the current region (optionally filtered by name)
- **Trail status** — check logging state and last delivery timestamps
- **Event selectors** — read which API categories are captured by a trail
- **Update trail** — change trail configuration (S3 bucket, multi-region, global events, etc.)
- **Delete trail** — remove a trail permanently

## Wire Format

`json_target` — AWS CloudTrail JSON 1.1 protocol. Requests use `X-Amz-Target` header with `com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.{Operation}`.

## Types

| Type | Description |
|------|-------------|
| `Trail` | Trail configuration including name, ARN, S3 bucket, home region |
| `GetTrailStatusResponse` | Logging state, timestamps for last delivery/notification |
| `EventSelector` | ReadWriteType, management events, data resource filters |
| `AdvancedEventSelector` | Advanced selector with field-level filtering |
| `DataResource` | S3/Lambda/DynamoDB resource filter for data events |
| `UpdateTrailResponse` | Updated trail configuration |

## Error Handling

Common errors:
- `TrailNotFoundException` — trail name or ARN does not exist in the region
- `InvalidTrailNameException` — trail name format is invalid
- `S3BucketDoesNotExistException` — specified bucket doesn't exist when creating/updating
- `InsufficientS3BucketPolicyException` — bucket policy doesn't grant CloudTrail write access
- `TrailAlreadyExistsException` — trail with that name already exists
