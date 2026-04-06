# Amazon DynamoDB API

## Overview

Amazon DynamoDB is a fully managed NoSQL database service. This client provides table management operations including listing, describing, updating, and deleting tables.

Note: This client covers table-level management operations. Item-level operations (PutItem, GetItem, Query, Scan) are not yet implemented.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let dynamodb = client.dynamodb();
```

## Features

- List tables in the current account
- Describe table properties (status, billing, key schema, throughput)
- Update table settings (billing mode, provisioned throughput, stream config)
- Delete tables

## Wire Format

DynamoDB uses the JSON Target protocol with JSON version 1.0 and target prefix `DynamoDB_20120810`. Field names use **PascalCase** (unlike ECS/ECR which use camelCase).

## Types

| Type | Description |
|------|-------------|
| `ListTablesInput` / `ListTablesOutput` | List table names with pagination |
| `DescribeTableInput` / `DescribeTableOutput` | Full table description |
| `UpdateTableInput` / `UpdateTableOutput` | Modify table settings |
| `DeleteTableInput` / `DeleteTableOutput` | Delete a table |
| `TableDescription` | Detailed table properties |
| `BillingModeSummary` | Billing mode (PAY_PER_REQUEST or PROVISIONED) |
| `ProvisionedThroughput` | Read/write capacity units (input) |
| `ProvisionedThroughputDescription` | Read/write capacity with metadata |
| `KeySchemaElement` | Primary key definition (HASH/RANGE) |
| `AttributeDefinition` | Attribute name and type |
| `StreamSpecification` | DynamoDB Streams configuration |
| `SSEDescription` | Server-side encryption status |

## Error Handling

Common errors:
- `ResourceNotFoundException` — table does not exist
- `ResourceInUseException` — table is being created/updated/deleted
- `LimitExceededException` — too many tables or throughput limits
