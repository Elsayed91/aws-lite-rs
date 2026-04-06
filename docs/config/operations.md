# AWS Config Operations

## SelectResourceConfig

### select_resource_config

**Signature**: `pub async fn select_resource_config(&self, body: &SelectResourceConfigRequest) -> Result<SelectResourceConfigResponse>`

Accepts a SQL SELECT command and returns matching resource configurations.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.expression` | `String` | SQL query SELECT command (required). Note: `LIMIT` is not valid in the SQL expression — use the `limit` field instead. |
| `body.limit` | `Option<i32>` | Maximum results per page (0-100). This is the only way to limit results — SQL `LIMIT` is not supported. |
| `body.next_token` | `Option<String>` | Pagination token from previous response |

**Returns**: `Result<SelectResourceConfigResponse>` with `results` (Vec of JSON strings), `query_info`, and `next_token`.

---

### select_resource_config_stream

**Signature**: `pub fn select_resource_config_stream(&self, expression: &str) -> impl Stream<Item = Result<String>>`

Convenience streaming wrapper that automatically handles pagination. Each yielded item is a raw JSON string representing a single resource configuration.

| Parameter | Type | Description |
|-----------|------|-------------|
| `expression` | `&str` | SQL query SELECT command |

**Returns**: `impl Stream<Item = Result<String>>` — each item is a JSON-encoded resource.
