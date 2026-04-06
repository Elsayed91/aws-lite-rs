# AWS Auto Scaling Usage Examples

## List Auto Scaling Groups

```rust
use aws_lite::AwsHttpClient;

let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let response = client.autoscaling().describe_auto_scaling_groups().await?;

for group in &response.auto_scaling_groups {
    println!("{} (desired={}, min={}, max={})",
        group.auto_scaling_group_name,
        group.desired_capacity, group.min_size, group.max_size);
}
```

## Update ASG Capacity

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::autoscaling::UpdateAutoScalingGroupRequest;

let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let body = UpdateAutoScalingGroupRequest {
    auto_scaling_group_name: "my-asg".to_string(),
    desired_capacity: Some(5),
    ..Default::default()
};
client.autoscaling().update_auto_scaling_group(&body).await?;
```

## Start Instance Refresh

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::autoscaling::StartInstanceRefreshRequest;

let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let body = StartInstanceRefreshRequest {
    auto_scaling_group_name: "my-asg".to_string(),
    ..Default::default()
};
let response = client.autoscaling().start_instance_refresh(&body).await?;
println!("Refresh ID: {:?}", response.instance_refresh_id);
```
