# SageMaker Operations

## ListNotebookInstances

Lists notebook instances in the account, with optional filters.

**Request:**
```rust
ListNotebookInstancesInput {
    status_equals: Some("InService".to_string()),  // filter by status
    name_contains: Some("my-prefix".to_string()),  // filter by name prefix
    max_results: Some(50),
    next_token: None,
    ..Default::default()
}
```

**Response fields (NotebookInstanceSummary):**
- `notebook_instance_name: String` — unique name of the notebook
- `notebook_instance_arn: String` — full ARN
- `notebook_instance_status: Option<String>` — `Pending`, `InService`, `Stopping`, `Stopped`, `Failed`, `Deleting`, `Updating`
- `instance_type: Option<String>` — e.g. `ml.t3.medium`
- `creation_time: Option<f64>` — Unix epoch float
- `last_modified_time: Option<f64>` — Unix epoch float
- `url: Option<String>` — Jupyter URL (when InService)

**Pagination:** Use `next_token` from response as `next_token` in next request.

---

## StopNotebookInstance

Stops a running notebook instance. The instance must be in `InService` status.

**Request:**
```rust
StopNotebookInstanceInput {
    notebook_instance_name: "my-notebook".to_string(),
}
```

**Response:** `Result<()>` — returns `()` on success.

**Notes:**
- Instance transitions to `Stopping` then `Stopped`
- Can be restarted via the SageMaker console or `StartNotebookInstance` API
- Returns `ValidationException` if the instance does not exist
