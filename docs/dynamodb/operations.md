# Amazon DynamoDB Operations

## Read Operations

### list_tables

**Signature**: `pub async fn list_tables(&self, body: &ListTablesInput) -> Result<ListTablesOutput>`

Returns an array of table names associated with the current account and endpoint.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.exclusive_start_table_name` | `Option<String>` | Pagination start key |
| `body.limit` | `Option<i32>` | Max results (default 100) |

**Returns**: `Result<ListTablesOutput>` with `table_names` Vec and optional `last_evaluated_table_name`.

---

### describe_table

**Signature**: `pub async fn describe_table(&self, body: &DescribeTableInput) -> Result<DescribeTableOutput>`

Returns information about the table, including status, key schema, billing mode, and throughput.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.table_name` | `String` | Table name or ARN (required) |

**Returns**: `Result<DescribeTableOutput>` with `table` containing a `TableDescription`.

---

## Write Operations

### update_table

**Signature**: `pub async fn update_table(&self, body: &UpdateTableInput) -> Result<UpdateTableOutput>`

Modifies table settings including billing mode, throughput, streams, and deletion protection.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.table_name` | `String` | Table name or ARN (required) |
| `body.billing_mode` | `Option<String>` | PAY_PER_REQUEST or PROVISIONED |
| `body.provisioned_throughput` | `Option<ProvisionedThroughput>` | RCU/WCU (required when switching to PROVISIONED) |
| `body.deletion_protection_enabled` | `Option<bool>` | Enable/disable deletion protection |
| `body.table_class` | `Option<String>` | STANDARD or STANDARD_INFREQUENT_ACCESS |
| `body.stream_specification` | `Option<StreamSpecification>` | DynamoDB Streams config |

**Returns**: `Result<UpdateTableOutput>` with `table_description`.

---

### delete_table

**Signature**: `pub async fn delete_table(&self, body: &DeleteTableInput) -> Result<DeleteTableOutput>`

Deletes a table and all of its items.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.table_name` | `String` | Table name or ARN (required) |

**Returns**: `Result<DeleteTableOutput>` with `table_description` showing DELETING status.
