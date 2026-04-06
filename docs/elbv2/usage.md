# Elastic Load Balancing v2 Usage Examples

## List All Load Balancers

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::elbv2::DescribeLoadBalancersRequest;

let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let response = client.elbv2().describe_load_balancers(&DescribeLoadBalancersRequest::default()).await?;

for lb in &response.load_balancers {
    println!("{} ({:?}) - {:?}",
        lb.load_balancer_name.as_deref().unwrap_or("?"),
        lb.r#type.as_deref().unwrap_or("?"),
        lb.state.as_ref().map(|s| s.code.as_deref().unwrap_or("?")));
}
```

## Describe Load Balancer by Name

```rust
use aws_lite::types::elbv2::DescribeLoadBalancersRequest;

let response = client.elbv2().describe_load_balancers(&DescribeLoadBalancersRequest {
    names: vec!["my-alb".to_string()],
    ..Default::default()
}).await?;
let lb = &response.load_balancers[0];
println!("ARN: {:?}", lb.load_balancer_arn);
println!("DNS: {:?}", lb.dns_name);
```

## Describe Target Groups for a Load Balancer

```rust
use aws_lite::types::elbv2::DescribeTargetGroupsRequest;

let response = client.elbv2().describe_target_groups(&DescribeTargetGroupsRequest {
    load_balancer_arn: Some(lb_arn.clone()),
    ..Default::default()
}).await?;

for tg in &response.target_groups {
    println!("{} - {}:{}",
        tg.target_group_name.as_deref().unwrap_or("?"),
        tg.protocol.as_deref().unwrap_or("?"),
        tg.port.unwrap_or(0));
}
```

## Check Target Health

```rust
use aws_lite::types::elbv2::DescribeTargetHealthRequest;

let response = client.elbv2().describe_target_health(&DescribeTargetHealthRequest {
    target_group_arn: tg_arn.clone(),
}).await?;

for desc in &response.target_health_descriptions {
    let target = desc.target.as_ref().unwrap();
    let health = desc.target_health.as_ref().unwrap();
    println!("{}:{} -> {:?}",
        target.id, target.port.unwrap_or(0),
        health.state.as_deref().unwrap_or("unknown"));
}
```

## Modify Load Balancer Attributes

```rust
use aws_lite::types::elbv2::{ModifyLoadBalancerAttributesRequest, LoadBalancerAttribute};

let response = client.elbv2().modify_load_balancer_attributes(&ModifyLoadBalancerAttributesRequest {
    load_balancer_arn: lb_arn.clone(),
    attributes: vec![
        LoadBalancerAttribute {
            key: Some("idle_timeout.timeout_seconds".to_string()),
            value: Some("120".to_string()),
        },
    ],
}).await?;

for attr in &response.attributes {
    println!("{}: {}",
        attr.key.as_deref().unwrap_or("?"),
        attr.value.as_deref().unwrap_or("?"));
}
```

## Delete a Load Balancer

```rust
use aws_lite::types::elbv2::DeleteLoadBalancerRequest;

client.elbv2().delete_load_balancer(&DeleteLoadBalancerRequest {
    load_balancer_arn: lb_arn.clone(),
}).await?;
println!("Load balancer deleted");
```

## Delete a Target Group

```rust
use aws_lite::types::elbv2::DeleteTargetGroupRequest;

client.elbv2().delete_target_group(&DeleteTargetGroupRequest {
    target_group_arn: tg_arn.clone(),
}).await?;
println!("Target group deleted");
```
