# Amazon ECS Usage Examples

## List All Clusters

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::ecs::ListClustersRequest;

let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let response = client.ecs().list_clusters(&ListClustersRequest::default()).await?;

for arn in &response.cluster_arns {
    println!("Cluster: {arn}");
}
```

## Describe a Cluster

```rust
use aws_lite::types::ecs::DescribeClustersRequest;

let response = client.ecs().describe_clusters(&DescribeClustersRequest {
    clusters: vec!["my-cluster".to_string()],
    ..Default::default()
}).await?;

for cluster in &response.clusters {
    println!("{} (status={}, services={}, tasks={})",
        cluster.cluster_name.as_deref().unwrap_or("?"),
        cluster.status.as_deref().unwrap_or("?"),
        cluster.active_services_count.unwrap_or(0),
        cluster.running_tasks_count.unwrap_or(0));
}
```

## List Services in a Cluster

```rust
use aws_lite::types::ecs::ListServicesRequest;

let response = client.ecs().list_services(&ListServicesRequest {
    cluster: Some("my-cluster".to_string()),
    ..Default::default()
}).await?;

for arn in &response.service_arns {
    println!("Service: {arn}");
}
```

## Describe a Service

```rust
use aws_lite::types::ecs::DescribeServicesRequest;

let response = client.ecs().describe_services(&DescribeServicesRequest {
    cluster: Some("my-cluster".to_string()),
    services: vec!["my-service".to_string()],
    ..Default::default()
}).await?;

let svc = &response.services[0];
println!("{} desired={} running={} task_def={}",
    svc.service_name.as_deref().unwrap_or("?"),
    svc.desired_count.unwrap_or(0),
    svc.running_count.unwrap_or(0),
    svc.task_definition.as_deref().unwrap_or("?"));
```

## Describe a Task Definition

```rust
use aws_lite::types::ecs::DescribeTaskDefinitionRequest;

let response = client.ecs().describe_task_definition(&DescribeTaskDefinitionRequest {
    task_definition: "my-task:5".to_string(),
    ..Default::default()
}).await?;

let td = response.task_definition.as_ref().unwrap();
println!("{} rev={} cpu={} memory={} containers={}",
    td.family.as_deref().unwrap_or("?"),
    td.revision.unwrap_or(0),
    td.cpu.as_deref().unwrap_or("?"),
    td.memory.as_deref().unwrap_or("?"),
    td.container_definitions.len());
```

## Force New Deployment

```rust
use aws_lite::types::ecs::UpdateServiceRequest;

let response = client.ecs().update_service(&UpdateServiceRequest {
    cluster: Some("my-cluster".to_string()),
    service: "my-service".to_string(),
    force_new_deployment: Some(true),
    ..Default::default()
}).await?;

let svc = response.service.as_ref().unwrap();
println!("Deployments: {}", svc.deployments.len());
```

## Deregister a Task Definition

```rust
use aws_lite::types::ecs::DeregisterTaskDefinitionRequest;

let response = client.ecs().deregister_task_definition(&DeregisterTaskDefinitionRequest {
    task_definition: "my-task:5".to_string(),
}).await?;

let td = response.task_definition.as_ref().unwrap();
println!("Status: {}", td.status.as_deref().unwrap_or("?"));
// Output: Status: INACTIVE
```
