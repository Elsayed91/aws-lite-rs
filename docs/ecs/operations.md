# Amazon ECS Operations

## Read Operations

### list_clusters

**Signature**: `pub async fn list_clusters(&self, body: &ListClustersRequest) -> Result<ListClustersResponse>`

Returns a list of existing cluster ARNs.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.next_token` | `Option<String>` | Pagination token |
| `body.max_results` | `Option<i32>` | Max results per page |

**Returns**: `Result<ListClustersResponse>` with `cluster_arns` Vec and optional `next_token`.

---

### describe_clusters

**Signature**: `pub async fn describe_clusters(&self, body: &DescribeClustersRequest) -> Result<DescribeClustersResponse>`

Describes one or more clusters. Non-existent clusters appear in `failures` rather than raising an error.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.clusters` | `Vec<String>` | Cluster names or ARNs to describe |
| `body.include` | `Vec<String>` | Additional info (e.g., ATTACHMENTS, SETTINGS, STATISTICS) |

**Returns**: `Result<DescribeClustersResponse>` with `clusters` Vec and `failures` Vec.

---

### list_services

**Signature**: `pub async fn list_services(&self, body: &ListServicesRequest) -> Result<ListServicesResponse>`

Returns a list of service ARNs in a cluster.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.cluster` | `Option<String>` | Cluster name or ARN (defaults to "default") |
| `body.next_token` | `Option<String>` | Pagination token |
| `body.max_results` | `Option<i32>` | Max results per page |
| `body.launch_type` | `Option<String>` | Filter by launch type (EC2 or FARGATE) |
| `body.scheduling_strategy` | `Option<String>` | Filter by scheduling strategy (REPLICA or DAEMON) |

**Returns**: `Result<ListServicesResponse>` with `service_arns` Vec and optional `next_token`.

---

### describe_services

**Signature**: `pub async fn describe_services(&self, body: &DescribeServicesRequest) -> Result<DescribeServicesResponse>`

Describes the specified services running in a cluster.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.cluster` | `Option<String>` | Cluster name or ARN |
| `body.services` | `Vec<String>` | Service names or ARNs (up to 10) |
| `body.include` | `Vec<String>` | Additional info (e.g., TAGS) |

**Returns**: `Result<DescribeServicesResponse>` with `services` Vec and `failures` Vec.

---

### describe_task_definition

**Signature**: `pub async fn describe_task_definition(&self, body: &DescribeTaskDefinitionRequest) -> Result<DescribeTaskDefinitionResponse>`

Describes a task definition by family:revision or full ARN.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.task_definition` | `String` | Task definition family:revision or ARN (required) |
| `body.include` | `Vec<String>` | Additional info (e.g., TAGS) |

**Returns**: `Result<DescribeTaskDefinitionResponse>` with optional `task_definition`.

---

## Write Operations

### update_service

**Signature**: `pub async fn update_service(&self, body: &UpdateServiceRequest) -> Result<UpdateServiceResponse>`

Modifies the parameters of a service. Can force a new deployment, change desired count, or update the task definition.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.cluster` | `Option<String>` | Cluster name or ARN |
| `body.service` | `String` | Service name (required) |
| `body.desired_count` | `Option<i32>` | New desired task count |
| `body.task_definition` | `Option<String>` | New task definition family:revision or ARN |
| `body.network_configuration` | `Option<NetworkConfiguration>` | New network config |
| `body.force_new_deployment` | `Option<bool>` | Force new deployment without definition changes |
| `body.health_check_grace_period_seconds` | `Option<i32>` | Health check grace period |

**Returns**: `Result<UpdateServiceResponse>` with updated `service` details.

---

### deregister_task_definition

**Signature**: `pub async fn deregister_task_definition(&self, body: &DeregisterTaskDefinitionRequest) -> Result<DeregisterTaskDefinitionResponse>`

Deregisters a task definition. The task definition is marked as INACTIVE but remains queryable.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.task_definition` | `String` | Task definition family:revision or ARN (required) |

**Returns**: `Result<DeregisterTaskDefinitionResponse>` with deregistered `task_definition` (status will be "INACTIVE").
