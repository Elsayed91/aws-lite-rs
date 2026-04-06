# Elastic Load Balancing v2 Operations

## Read Operations

### describe_load_balancers

**Signature**: `pub async fn describe_load_balancers(&self, body: &DescribeLoadBalancersRequest) -> Result<DescribeLoadBalancersResponse>`

Describes the specified load balancers or all load balancers in the region.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.names` | `Vec<String>` | Filter by load balancer names |
| `body.load_balancer_arns` | `Vec<String>` | Filter by load balancer ARNs |
| `body.marker` | `Option<String>` | Pagination token |
| `body.page_size` | `Option<i32>` | Max results per page |

**Returns**: `Result<DescribeLoadBalancersResponse>` with `load_balancers` Vec and optional `next_marker`.

---

### describe_target_groups

**Signature**: `pub async fn describe_target_groups(&self, body: &DescribeTargetGroupsRequest) -> Result<DescribeTargetGroupsResponse>`

Describes the specified target groups or all target groups.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.load_balancer_arn` | `Option<String>` | Filter by associated LB ARN |
| `body.target_group_arns` | `Vec<String>` | Filter by target group ARNs |
| `body.names` | `Vec<String>` | Filter by target group names |
| `body.marker` | `Option<String>` | Pagination token |
| `body.page_size` | `Option<i32>` | Max results per page |

**Returns**: `Result<DescribeTargetGroupsResponse>` with `target_groups` Vec.

---

### describe_target_health

**Signature**: `pub async fn describe_target_health(&self, body: &DescribeTargetHealthRequest) -> Result<DescribeTargetHealthResponse>`

Describes the health of the targets registered with the specified target group.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.target_group_arn` | `String` | Target group ARN (required) |

**Returns**: `Result<DescribeTargetHealthResponse>` with `target_health_descriptions` Vec.

---

### describe_listeners

**Signature**: `pub async fn describe_listeners(&self, body: &DescribeListenersRequest) -> Result<DescribeListenersResponse>`

Describes the listeners for the specified load balancer.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.load_balancer_arn` | `Option<String>` | Load balancer ARN |
| `body.listener_arns` | `Vec<String>` | Filter by listener ARNs |
| `body.marker` | `Option<String>` | Pagination token |
| `body.page_size` | `Option<i32>` | Max results per page |

**Returns**: `Result<DescribeListenersResponse>` with `listeners` Vec.

---

### describe_load_balancer_attributes

**Signature**: `pub async fn describe_load_balancer_attributes(&self, body: &DescribeLoadBalancerAttributesRequest) -> Result<DescribeLoadBalancerAttributesResponse>`

Describes the attributes for the specified load balancer.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.load_balancer_arn` | `String` | Load balancer ARN (required) |

**Returns**: `Result<DescribeLoadBalancerAttributesResponse>` with `attributes` Vec of key-value pairs.

---

## Write Operations

### delete_load_balancer

**Signature**: `pub async fn delete_load_balancer(&self, body: &DeleteLoadBalancerRequest) -> Result<DeleteLoadBalancerResponse>`

Deletes the specified load balancer and its attached listeners.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.load_balancer_arn` | `String` | Load balancer ARN (required) |

**Returns**: `Result<DeleteLoadBalancerResponse>` (empty on success).

---

### delete_target_group

**Signature**: `pub async fn delete_target_group(&self, body: &DeleteTargetGroupRequest) -> Result<DeleteTargetGroupResponse>`

Deletes the specified target group. The target group must not be referenced by any listener actions.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.target_group_arn` | `String` | Target group ARN (required) |

**Returns**: `Result<DeleteTargetGroupResponse>` (empty on success).

---

### modify_load_balancer_attributes

**Signature**: `pub async fn modify_load_balancer_attributes(&self, body: &ModifyLoadBalancerAttributesRequest) -> Result<ModifyLoadBalancerAttributesResponse>`

Modifies the attributes of the specified load balancer.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.load_balancer_arn` | `String` | Load balancer ARN (required) |
| `body.attributes` | `Vec<LoadBalancerAttribute>` | Key-value attributes to modify |

**Returns**: `Result<ModifyLoadBalancerAttributesResponse>` with updated `attributes` Vec.
