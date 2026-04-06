# Amazon S3 API

## Overview

Amazon Simple Storage Service (S3) remediation operations. This client provides write-only operations for S3 bucket configuration used in cloud remediation workflows. Read operations are handled through AWS Config's `SelectResourceConfig`.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let s3 = client.s3();
```

## Features

- Bucket policy management (put/delete)
- Public access block configuration
- Lifecycle configuration management
- All operations use regional S3 endpoints (`s3.{region}.amazonaws.com`)
- Content-MD5 computed automatically for PUT operations

## Wire Format

**Protocol**: REST-XML

S3 uses path-style URLs with XML request/response bodies. Each operation targets `/{Bucket}?{sub-resource}`.

## Types

| Type | Description |
|------|-------------|
| `PublicAccessBlockConfiguration` | 4 boolean fields controlling public access |
| `BucketLifecycleConfiguration` | Container for lifecycle rules |
| `LifecycleRule` | Individual lifecycle rule with filter, expiration, transitions |
| `LifecycleRuleFilter` | Filter criteria (prefix, size bounds) |
| `LifecycleExpiration` | Expiration settings (date, days, delete markers) |
| `Transition` | Storage class transition settings |
| `NoncurrentVersionTransition` | Noncurrent version transition settings |
| `NoncurrentVersionExpiration` | Noncurrent version expiration settings |
| `AbortIncompleteMultipartUpload` | Abort incomplete upload settings |

## Error Handling

Common errors for this API:
- `NoSuchBucket` — bucket does not exist
- `MalformedPolicy` — invalid policy JSON
- `MalformedXML` — invalid XML request body
- `AccessDenied` — insufficient S3 permissions
