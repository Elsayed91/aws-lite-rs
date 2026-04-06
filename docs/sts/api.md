# AWS STS API

## Overview

AWS Security Token Service (STS) enables requesting temporary, limited-privilege credentials for IAM users or federated users. This client provides credential validation via GetCallerIdentity and cross-account access via AssumeRole.

STS supports both global (`sts.amazonaws.com`) and regional endpoints.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("us-east-1")?;
let sts = client.sts();
```

## Features

- Validate credentials by retrieving caller identity (account, ARN, user ID)
- Assume IAM roles for cross-account access
- Query/XML wire format (form-encoded requests, XML responses)

## Types

| Type | Description |
|------|-------------|
| `GetCallerIdentityResponse` | Response with account, ARN, and user ID |
| `AssumeRoleRequest` | Request with role ARN, session name, optional external ID |
| `AssumeRoleResponse` | Response with temporary credentials and assumed role user |
| `Credentials` | Temporary credentials (access key, secret key, session token, expiration) |
| `AssumedRoleUser` | Assumed role identity (ARN and role ID) |

## Error Handling

Common errors for this API:
- `AwsError::AccessDenied` — insufficient permissions to assume role
- `AwsError::ServiceError` — general AWS error (e.g., malformed ARN, expired token)
