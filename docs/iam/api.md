# AWS IAM API

## Overview

AWS Identity and Access Management (IAM) enables management of users, policies, and access keys. This client provides read operations for listing users and their attached policies, plus write operations for detaching policies and deleting access keys.

IAM is a global service — all requests go to `iam.amazonaws.com` regardless of region.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("us-east-1")?;
let iam = client.iam();
```

## Features

- List IAM users in the account
- List managed policies attached to a user
- Detach managed policies from users
- Delete access key pairs
- Query/XML wire format (form-encoded requests, XML responses)

## Types

| Type | Description |
|------|-------------|
| `ListUsersResponse` | Response wrapper with users list and pagination |
| `User` | IAM user with arn, user_name, create_date |
| `ListAttachedUserPoliciesResponse` | Response wrapper with attached policies and pagination |
| `AttachedPolicy` | Attached policy with policy_arn and policy_name |
| `DetachUserPolicyRequest` | Request to detach a policy (user_name + policy_arn) |
| `DeleteAccessKeyRequest` | Request to delete an access key (user_name + access_key_id) |

## Error Handling

Common errors for this API:
- `AwsError::NotFound` — user or policy doesn't exist
- `AwsError::AccessDenied` — insufficient IAM permissions
- `AwsError::ServiceError` — general AWS error with code and message
