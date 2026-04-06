# Elastic Load Balancing v2 API

## Overview

Amazon Elastic Load Balancing v2 manages Application Load Balancers (ALB), Network Load Balancers (NLB), and Gateway Load Balancers. This client provides read operations for describing load balancers, target groups, listeners, target health, and load balancer attributes, plus write operations for deleting load balancers/target groups and modifying load balancer attributes.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let elbv2 = client.elbv2();
```

## Features

- Describe load balancers (by name or ARN, or list all)
- Describe target groups (by name, ARN, or associated load balancer)
- Describe target health for registered targets
- Describe listeners for a load balancer
- Describe and modify load balancer attributes (idle timeout, deletion protection, etc.)
- Delete load balancers and target groups
- Query/XML wire format (form-encoded requests, XML responses)

## Types

| Type | Description |
|------|-------------|
| `DescribeLoadBalancersRequest` | Filter by names or ARNs, with pagination |
| `DescribeLoadBalancersResponse` | Response with load balancer list |
| `LoadBalancer` | ALB/NLB with ARN, DNS name, scheme, VPC, AZs, state |
| `LoadBalancerState` | State code and reason |
| `AvailabilityZone` | Zone name and subnet ID |
| `DescribeTargetGroupsRequest` | Filter by names, ARNs, or LB ARN |
| `DescribeTargetGroupsResponse` | Response with target group list |
| `TargetGroup` | Target group with health check config, protocol, port |
| `DescribeTargetHealthRequest` | Target group ARN (required) |
| `DescribeTargetHealthResponse` | Response with target health descriptions |
| `TargetHealthDescription` | Target with health check port and health status |
| `TargetDescription` | Target ID, port, and availability zone |
| `TargetHealth` | Health state, reason, and description |
| `DescribeListenersRequest` | Filter by LB ARN or listener ARNs |
| `DescribeListenersResponse` | Response with listener list |
| `Listener` | Listener with ARN, port, protocol, SSL policy |
| `DescribeLoadBalancerAttributesRequest` | LB ARN (required) |
| `DescribeLoadBalancerAttributesResponse` | Response with attribute list |
| `LoadBalancerAttribute` | Key-value attribute pair |
| `DeleteLoadBalancerRequest` | LB ARN (required) |
| `DeleteTargetGroupRequest` | Target group ARN (required) |
| `ModifyLoadBalancerAttributesRequest` | LB ARN + attributes to modify |
| `ModifyLoadBalancerAttributesResponse` | Updated attributes |

## Error Handling

Common errors for this API:
- `AwsError::ServiceError` with `LoadBalancerNotFound` — load balancer not found
- `AwsError::ServiceError` with `TargetGroupNotFound` — target group not found
- `AwsError::ServiceError` with `ValidationError` — invalid parameters
- `AwsError::AccessDenied` — insufficient IAM permissions
