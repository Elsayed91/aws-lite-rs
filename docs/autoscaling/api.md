# AWS Auto Scaling API

## Overview

Amazon Auto Scaling manages Auto Scaling groups and launch configurations. This client provides read operations for listing groups and launch configs, plus write operations for updating group configuration and triggering instance refreshes.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let asg = client.autoscaling();
```

## Features

- List Auto Scaling groups and launch configurations
- Update ASG configuration (capacity, launch template)
- Start instance refreshes for rolling updates
- Query/XML wire format (form-encoded requests, XML responses)

## Types

| Type | Description |
|------|-------------|
| `DescribeAutoScalingGroupsResponse` | Response with ASG list and pagination |
| `AutoScalingGroup` | ASG with name, capacity, launch template |
| `DescribeLaunchConfigurationsResponse` | Response with launch config list and pagination |
| `LaunchConfiguration` | Legacy launch config with image, instance type |
| `LaunchTemplateSpecification` | Launch template reference (ID, name, version) |
| `UpdateAutoScalingGroupRequest` | Request to update ASG config |
| `StartInstanceRefreshRequest` | Request to start instance refresh |
| `StartInstanceRefreshResponse` | Response with instance refresh ID |

## Error Handling

Common errors for this API:
- `AwsError::ServiceError` with `ValidationError` — ASG not found
- `AwsError::AccessDenied` — insufficient IAM permissions
