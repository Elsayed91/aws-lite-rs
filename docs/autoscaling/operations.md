# AWS Auto Scaling Operations

## Read Operations

### describe_auto_scaling_groups

**Signature**: `pub async fn describe_auto_scaling_groups(&self) -> Result<DescribeAutoScalingGroupsResponse>`

Lists all Auto Scaling groups in the region.

**Returns**: `Result<DescribeAutoScalingGroupsResponse>` with `auto_scaling_groups` Vec.

---

### describe_launch_configurations

**Signature**: `pub async fn describe_launch_configurations(&self) -> Result<DescribeLaunchConfigurationsResponse>`

Lists all launch configurations in the region.

**Returns**: `Result<DescribeLaunchConfigurationsResponse>` with `launch_configurations` Vec.

---

## Write Operations

### update_auto_scaling_group

**Signature**: `pub async fn update_auto_scaling_group(&self, body: &UpdateAutoScalingGroupRequest) -> Result<()>`

Updates the configuration for the specified Auto Scaling group.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.auto_scaling_group_name` | `String` | ASG name (required) |
| `body.desired_capacity` | `Option<i32>` | Target capacity |
| `body.min_size` | `Option<i32>` | Minimum size |
| `body.max_size` | `Option<i32>` | Maximum size |
| `body.launch_template` | `Option<LaunchTemplateSpecification>` | New launch template |

---

### start_instance_refresh

**Signature**: `pub async fn start_instance_refresh(&self, body: &StartInstanceRefreshRequest) -> Result<StartInstanceRefreshResponse>`

Starts a rolling instance refresh for the specified Auto Scaling group.

| Parameter | Type | Description |
|-----------|------|-------------|
| `body.auto_scaling_group_name` | `String` | ASG name (required) |
| `body.strategy` | `Option<String>` | Refresh strategy |
| `body.preferences` | `Option<RefreshPreferences>` | Rolling update preferences |

**Returns**: `Result<StartInstanceRefreshResponse>` with `instance_refresh_id`.
