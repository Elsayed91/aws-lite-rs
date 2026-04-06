# Amazon Elastic Container Service API

## Overview

Amazon Elastic Container Service (ECS) is a container orchestration service for running Docker containers on AWS. This client provides operations for managing ECS clusters, services, and task definitions using the Fargate launch type.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let ecs = client.ecs();
```

## Features

- List and describe ECS clusters
- List and describe services within a cluster
- Describe task definitions (family, revision, containers)
- Update services (force new deployment, change desired count, task definition)
- Deregister task definitions (marks as INACTIVE)
- JSON target wire format (JSON body, `X-Amz-Target` header)

## Types

| Type | Description |
|------|-------------|
| `ListClustersRequest` | Pagination parameters for listing clusters |
| `ListClustersResponse` | Response with cluster ARN list |
| `DescribeClustersRequest` | Cluster names/ARNs to describe |
| `DescribeClustersResponse` | Response with cluster details and failures |
| `Cluster` | Cluster with name, status, task/service counts |
| `ListServicesRequest` | Filter by cluster, with pagination |
| `ListServicesResponse` | Response with service ARN list |
| `DescribeServicesRequest` | Cluster + service names/ARNs to describe |
| `DescribeServicesResponse` | Response with service details and failures |
| `Service` | Service with name, status, counts, deployments, task definition |
| `Deployment` | Deployment with status, rollout state, desired/running/pending counts |
| `ServiceEvent` | Service event with message and timestamp |
| `NetworkConfiguration` | VPC networking config (subnets, security groups, public IP) |
| `AwsVpcConfiguration` | Subnets, security groups, assign public IP setting |
| `LoadBalancer` | Load balancer config (target group, container, port) |
| `DescribeTaskDefinitionRequest` | Task definition family:revision or ARN |
| `DescribeTaskDefinitionResponse` | Response with task definition details |
| `TaskDefinition` | Task def with family, revision, CPU, memory, containers, status |
| `ContainerDefinition` | Container with name, image, CPU, memory, ports, environment |
| `PortMapping` | Container port, host port, protocol |
| `KeyValuePair` | Name-value pair for environment variables |
| `UpdateServiceRequest` | Cluster, service, desired count, task def, force new deployment |
| `UpdateServiceResponse` | Response with updated service details |
| `DeregisterTaskDefinitionRequest` | Task definition family:revision or ARN |
| `DeregisterTaskDefinitionResponse` | Response with deregistered task definition |
| `Failure` | Failure with ARN, reason, detail |
| `CapacityProviderStrategyItem` | Capacity provider, weight, base |
| `DeploymentConfiguration` | Max percent, min healthy percent |
| `Tag` | Key-value tag pair |

## Error Handling

Common errors for this API:
- `AwsError::ServiceError` with `ClusterNotFoundException` -- cluster not found
- `AwsError::ServiceError` with `ClientException` -- invalid task definition or general client error
- `AwsError::ServiceError` with `ServiceNotFoundException` -- service not found
- `AwsError::ServiceError` with `InvalidParameterException` -- invalid parameters
- `AwsError::AccessDenied` -- insufficient IAM permissions

## Notes

- ECS timestamps are epoch floats (e.g., `1700000000.0`), mapped to `f64` in the types
- Non-existent clusters return `failures` in the response rather than an error
- Deregistered task definitions remain queryable with `status: "INACTIVE"`
