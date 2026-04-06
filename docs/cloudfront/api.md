# Amazon CloudFront API

## Overview

Amazon CloudFront is a content delivery network (CDN) that delivers data, videos, applications, and APIs globally with low latency. This client provides distribution management, configuration retrieval, and origin access control creation.

CloudFront is a global service — all requests go to `cloudfront.amazonaws.com` regardless of region.

Wire format: REST-XML (XML request/response bodies with RESTful URL routing).

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("us-east-1")?;
let cf = client.cloudfront();
```

## Features

- List CloudFront distributions with pagination metadata
- Get distribution configuration details (origins, cache behaviors, etc.)
- Create and update distributions
- Create origin access controls (OAC) for S3 origin authentication

## Known Limitations

- **CreateDistribution / UpdateDistribution**: Blocked by REST-XML Vec serialization issue — `quick_xml` doesn't wrap list items in their type name element. See `.claude/backlog/issue-tracker/cloudfront-rest-xml-vec-serialization.md` for details.
- **UpdateDistribution / DeleteDistribution / DeleteOriginAccessControl**: Blocked by missing ETag/If-Match header support in `AwsHttpClient`.

## Types

| Type | Description |
|------|-------------|
| `DistributionList` | List response with distribution summaries and pagination |
| `DistributionSummary` | Summary of a distribution (ID, ARN, status, domain, etc.) |
| `Distribution` | Full distribution resource with embedded config |
| `DistributionConfig` | Distribution configuration (origins, cache behavior, etc.) |
| `Origins` | Container for origin definitions |
| `Origin` | An origin (S3 bucket, custom endpoint, etc.) |
| `S3OriginConfig` | S3-specific origin configuration |
| `DefaultCacheBehavior` | Default cache behavior (viewer protocol policy, target origin) |
| `OriginAccessControl` | Origin access control resource with ID and config |
| `OriginAccessControlConfig` | OAC settings (signing protocol, behavior, origin type) |

## Error Handling

Common errors for this API:
- `AwsError::ServiceError` with code `NoSuchDistribution` — distribution ID doesn't exist
- `AwsError::ServiceError` with code `OriginAccessControlAlreadyExists` — OAC name conflict
- `AwsError::ServiceError` with code `MalformedInput` — invalid XML request body
- `AwsError::AccessDenied` — insufficient CloudFront permissions
