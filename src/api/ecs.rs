//! Amazon Elastic Container Service API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::ecs::EcsOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::ecs::EcsOps,
    types::ecs::{
        DeregisterTaskDefinitionRequest, DeregisterTaskDefinitionResponse, DescribeClustersRequest,
        DescribeClustersResponse, DescribeServicesRequest, DescribeServicesResponse,
        DescribeTaskDefinitionRequest, DescribeTaskDefinitionResponse, ListClustersRequest,
        ListClustersResponse, ListServicesRequest, ListServicesResponse, UpdateServiceRequest,
        UpdateServiceResponse,
    },
};

/// Client for the Amazon Elastic Container Service API
pub struct EcsClient<'a> {
    ops: EcsOps<'a>,
}

impl<'a> EcsClient<'a> {
    /// Create a new Amazon Elastic Container Service API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: EcsOps::new(client),
        }
    }

    /// Returns a list of existing clusters.
    pub async fn list_clusters(&self, body: &ListClustersRequest) -> Result<ListClustersResponse> {
        self.ops.list_clusters(body).await
    }

    /// Describes one or more of your clusters. For CLI examples, see describe-clusters.rst on GitHub.
    pub async fn describe_clusters(
        &self,
        body: &DescribeClustersRequest,
    ) -> Result<DescribeClustersResponse> {
        self.ops.describe_clusters(body).await
    }

    /// Returns a list of services. You can filter the results by cluster, launch type, and scheduling strategy.
    pub async fn list_services(&self, body: &ListServicesRequest) -> Result<ListServicesResponse> {
        self.ops.list_services(body).await
    }

    /// Describes the specified services running in your cluster.
    pub async fn describe_services(
        &self,
        body: &DescribeServicesRequest,
    ) -> Result<DescribeServicesResponse> {
        self.ops.describe_services(body).await
    }

    /// Describes a task definition. You can specify a family and revision to find information about a specific task definition,
    pub async fn describe_task_definition(
        &self,
        body: &DescribeTaskDefinitionRequest,
    ) -> Result<DescribeTaskDefinitionResponse> {
        self.ops.describe_task_definition(body).await
    }

    /// Modifies the parameters of a service. On March 21, 2024, a change was made to resolve the task definition revision befor
    pub async fn update_service(
        &self,
        body: &UpdateServiceRequest,
    ) -> Result<UpdateServiceResponse> {
        self.ops.update_service(body).await
    }

    /// Deregisters the specified task definition by family and revision. Upon deregistration, the task definition is marked as
    pub async fn deregister_task_definition(
        &self,
        body: &DeregisterTaskDefinitionRequest,
    ) -> Result<DeregisterTaskDefinitionResponse> {
        self.ops.deregister_task_definition(body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn list_clusters_returns_arns() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "clusterArns": [
                "arn:aws:ecs:us-east-1:123456789012:cluster/my-cluster",
                "arn:aws:ecs:us-east-1:123456789012:cluster/default"
            ]
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecs()
            .list_clusters(&ListClustersRequest::default())
            .await
            .unwrap();
        assert_eq!(result.cluster_arns.len(), 2);
        assert!(result.cluster_arns[0].contains("my-cluster"));
        assert!(result.cluster_arns[1].contains("default"));
    }

    #[tokio::test]
    async fn describe_clusters_returns_details() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "clusters": [{
                "clusterArn": "arn:aws:ecs:us-east-1:123456789012:cluster/my-cluster",
                "clusterName": "my-cluster",
                "status": "ACTIVE",
                "registeredContainerInstancesCount": 3,
                "runningTasksCount": 10,
                "pendingTasksCount": 0,
                "activeServicesCount": 2
            }],
            "failures": []
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecs()
            .describe_clusters(&DescribeClustersRequest {
                clusters: vec!["my-cluster".into()],
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.clusters.len(), 1);
        let c = &result.clusters[0];
        assert_eq!(c.cluster_name.as_deref(), Some("my-cluster"));
        assert_eq!(c.status.as_deref(), Some("ACTIVE"));
        assert_eq!(c.running_tasks_count, Some(10));
        assert_eq!(c.active_services_count, Some(2));
        assert_eq!(c.registered_container_instances_count, Some(3));
        assert!(result.failures.is_empty());
    }

    #[tokio::test]
    async fn describe_services_returns_service_details() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "services": [{
                "serviceArn": "arn:aws:ecs:us-east-1:123456789012:service/my-cluster/my-svc",
                "serviceName": "my-svc",
                "clusterArn": "arn:aws:ecs:us-east-1:123456789012:cluster/my-cluster",
                "status": "ACTIVE",
                "desiredCount": 3,
                "runningCount": 3,
                "pendingCount": 0,
                "launchType": "FARGATE",
                "taskDefinition": "arn:aws:ecs:us-east-1:123456789012:task-definition/my-task:5",
                "schedulingStrategy": "REPLICA",
                "deployments": [{
                    "id": "ecs-svc/123",
                    "status": "PRIMARY",
                    "taskDefinition": "arn:aws:ecs:us-east-1:123456789012:task-definition/my-task:5",
                    "desiredCount": 3,
                    "runningCount": 3,
                    "pendingCount": 0,
                    "failedTasks": 0,
                    "createdAt": 1700000000.0,
                    "updatedAt": 1700000100.0,
                    "rolloutState": "COMPLETED"
                }],
                "createdAt": 1700000000.0
            }],
            "failures": []
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecs()
            .describe_services(&DescribeServicesRequest {
                cluster: Some("my-cluster".into()),
                services: vec!["my-svc".into()],
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.services.len(), 1);
        let svc = &result.services[0];
        assert_eq!(svc.service_name.as_deref(), Some("my-svc"));
        assert_eq!(svc.status.as_deref(), Some("ACTIVE"));
        assert_eq!(svc.desired_count, Some(3));
        assert_eq!(svc.running_count, Some(3));
        assert_eq!(svc.launch_type.as_deref(), Some("FARGATE"));
        assert_eq!(svc.deployments.len(), 1);
        let dep = &svc.deployments[0];
        assert_eq!(dep.status.as_deref(), Some("PRIMARY"));
        assert_eq!(dep.rollout_state.as_deref(), Some("COMPLETED"));
    }

    #[tokio::test]
    async fn describe_task_definition_returns_details() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "taskDefinition": {
                "taskDefinitionArn": "arn:aws:ecs:us-east-1:123456789012:task-definition/my-task:5",
                "family": "my-task",
                "revision": 5,
                "cpu": "512",
                "memory": "1024",
                "networkMode": "awsvpc",
                "status": "ACTIVE",
                "requiresCompatibilities": ["FARGATE"],
                "containerDefinitions": [{
                    "name": "web",
                    "image": "nginx:latest",
                    "cpu": 256,
                    "memory": 512,
                    "essential": true,
                    "portMappings": [{
                        "containerPort": 80,
                        "protocol": "tcp"
                    }]
                }],
                "registeredAt": 1700000000.0
            }
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecs()
            .describe_task_definition(&DescribeTaskDefinitionRequest {
                task_definition: "my-task:5".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        let td = result.task_definition.as_ref().unwrap();
        assert_eq!(td.family.as_deref(), Some("my-task"));
        assert_eq!(td.revision, Some(5));
        assert_eq!(td.cpu.as_deref(), Some("512"));
        assert_eq!(td.memory.as_deref(), Some("1024"));
        assert_eq!(td.network_mode.as_deref(), Some("awsvpc"));
        assert_eq!(td.requires_compatibilities, vec!["FARGATE"]);
        assert_eq!(td.container_definitions.len(), 1);
        let c = &td.container_definitions[0];
        assert_eq!(c.name.as_deref(), Some("web"));
        assert_eq!(c.image.as_deref(), Some("nginx:latest"));
        assert_eq!(c.cpu, Some(256));
        assert_eq!(c.essential, Some(true));
        assert_eq!(c.port_mappings.len(), 1);
        assert_eq!(c.port_mappings[0].container_port, Some(80));
    }

    #[tokio::test]
    async fn list_services_returns_arns() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "serviceArns": [
                "arn:aws:ecs:us-east-1:123456789012:service/my-cluster/svc-a",
                "arn:aws:ecs:us-east-1:123456789012:service/my-cluster/svc-b"
            ]
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecs()
            .list_services(&ListServicesRequest {
                cluster: Some("my-cluster".into()),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.service_arns.len(), 2);
        assert!(result.service_arns[0].contains("svc-a"));
        assert!(result.service_arns[1].contains("svc-b"));
    }

    #[tokio::test]
    async fn update_service_forces_new_deployment() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "service": {
                "serviceArn": "arn:aws:ecs:us-east-1:123456789012:service/my-cluster/my-svc",
                "serviceName": "my-svc",
                "clusterArn": "arn:aws:ecs:us-east-1:123456789012:cluster/my-cluster",
                "status": "ACTIVE",
                "desiredCount": 3,
                "runningCount": 3,
                "pendingCount": 0,
                "launchType": "FARGATE",
                "taskDefinition": "arn:aws:ecs:us-east-1:123456789012:task-definition/my-task:5",
                "deployments": [
                    {
                        "id": "ecs-svc/new",
                        "status": "PRIMARY",
                        "taskDefinition": "arn:aws:ecs:us-east-1:123456789012:task-definition/my-task:5",
                        "desiredCount": 3,
                        "runningCount": 0,
                        "pendingCount": 3,
                        "createdAt": 1700000200.0,
                        "updatedAt": 1700000200.0,
                        "rolloutState": "IN_PROGRESS"
                    },
                    {
                        "id": "ecs-svc/old",
                        "status": "ACTIVE",
                        "taskDefinition": "arn:aws:ecs:us-east-1:123456789012:task-definition/my-task:5",
                        "desiredCount": 3,
                        "runningCount": 3,
                        "pendingCount": 0,
                        "createdAt": 1700000000.0,
                        "updatedAt": 1700000100.0,
                        "rolloutState": "COMPLETED"
                    }
                ],
                "createdAt": 1700000000.0
            }
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecs()
            .update_service(&UpdateServiceRequest {
                cluster: Some("my-cluster".into()),
                service: "my-svc".into(),
                force_new_deployment: Some(true),
                ..Default::default()
            })
            .await
            .unwrap();
        let svc = result.service.as_ref().unwrap();
        assert_eq!(svc.service_name.as_deref(), Some("my-svc"));
        assert_eq!(svc.deployments.len(), 2);
        assert_eq!(svc.deployments[0].status.as_deref(), Some("PRIMARY"));
        assert_eq!(
            svc.deployments[0].rollout_state.as_deref(),
            Some("IN_PROGRESS")
        );
    }

    #[tokio::test]
    async fn deregister_task_definition_returns_inactive() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "taskDefinition": {
                "taskDefinitionArn": "arn:aws:ecs:us-east-1:123456789012:task-definition/my-task:5",
                "family": "my-task",
                "revision": 5,
                "cpu": "512",
                "memory": "1024",
                "networkMode": "awsvpc",
                "status": "INACTIVE",
                "requiresCompatibilities": ["FARGATE"],
                "containerDefinitions": [{
                    "name": "web",
                    "image": "nginx:latest",
                    "cpu": 256,
                    "memory": 512,
                    "essential": true,
                    "portMappings": [{
                        "containerPort": 80,
                        "protocol": "tcp"
                    }]
                }],
                "registeredAt": 1700000000.0,
                "deregisteredAt": 1700001000.0
            }
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .ecs()
            .deregister_task_definition(&DeregisterTaskDefinitionRequest {
                task_definition: "my-task:5".into(),
            })
            .await
            .unwrap();
        let td = result.task_definition.as_ref().unwrap();
        assert_eq!(td.family.as_deref(), Some("my-task"));
        assert_eq!(td.revision, Some(5));
        assert_eq!(td.status.as_deref(), Some("INACTIVE"));
        assert!(td.deregistered_at.is_some());
    }
}
