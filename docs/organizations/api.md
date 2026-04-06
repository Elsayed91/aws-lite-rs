# AWS Organizations API

## Overview

AWS Organizations provides account management and organizational structure queries. Currently supports listing organizational units (OUs) and accounts within a parent OU or root.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("us-east-1")?;
let orgs = client.organizations();
```

## Features

- List organizational units in a parent OU or root
- List accounts in a parent OU or root
- Automatic pagination with stream helpers
- Typed enums for account status and join method

## Types

| Type | Description |
|------|-------------|
| `OrganizationalUnit` | OU with id, arn, and name |
| `Account` | AWS account with id, arn, email, name, status, join method, and timestamp |
| `AccountStatus` | Enum: `Active`, `Suspended`, `PendingClosure` |
| `AccountJoinedMethod` | Enum: `Invited`, `Created` |
| `ListOrganizationalUnitsForParentRequest` | Request with parent_id and pagination |
| `ListOrganizationalUnitsForParentResponse` | Response with OUs and next token |
| `ListAccountsForParentRequest` | Request with parent_id and pagination |
| `ListAccountsForParentResponse` | Response with accounts and next token |

## Error Handling

Common errors for this API:
- `AwsError::AccessDenied` — insufficient IAM permissions
- `AwsError::ServiceError` with code `ParentNotFoundException` — invalid parent ID
- `AwsError::ServiceError` with code `AWSOrganizationsNotInUseException` — Organizations not enabled
