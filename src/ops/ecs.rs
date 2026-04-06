//! Operation contracts for the Amazon Elastic Container Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/ecs.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::ecs::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon Elastic Container Service API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::ecs::EcsClient`] instead.
pub struct EcsOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> EcsOps<'a> {
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> String {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/').to_string();
            }
        }
        "https://ecs.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Returns a list of existing clusters.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListClustersRequest`]
    ///
    /// # Response
    /// [`ListClustersResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_clusters(
        &self,
        body: &ListClustersRequest,
    ) -> Result<ListClustersResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize list_clusters request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecs",
                "AmazonEC2ContainerServiceV20141113.ListClusters",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_clusters response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_clusters response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Describes one or more of your clusters. For CLI examples, see describe-clusters.rst on
    /// GitHub.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeClustersRequest`]
    ///
    /// # Response
    /// [`DescribeClustersResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_clusters(
        &self,
        body: &DescribeClustersRequest,
    ) -> Result<DescribeClustersResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_clusters request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecs",
                "AmazonEC2ContainerServiceV20141113.DescribeClusters",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_clusters response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_clusters response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Returns a list of services. You can filter the results by cluster, launch type, and
    /// scheduling strategy.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListServicesRequest`]
    ///
    /// # Response
    /// [`ListServicesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_services(
        &self,
        body: &ListServicesRequest,
    ) -> Result<ListServicesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize list_services request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecs",
                "AmazonEC2ContainerServiceV20141113.ListServices",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_services response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_services response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Describes the specified services running in your cluster.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeServicesRequest`]
    ///
    /// # Response
    /// [`DescribeServicesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_services(
        &self,
        body: &DescribeServicesRequest,
    ) -> Result<DescribeServicesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_services request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecs",
                "AmazonEC2ContainerServiceV20141113.DescribeServices",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_services response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_services response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Describes a task definition. You can specify a family and revision to find information
    /// about a specific task definition, or you can simply specify the family to find the
    /// latest ACTIVE revision in that family. You can only describe INACTIVE task definitions
    /// while an active task or service references them.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeTaskDefinitionRequest`]
    ///
    /// # Response
    /// [`DescribeTaskDefinitionResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_task_definition(
        &self,
        body: &DescribeTaskDefinitionRequest,
    ) -> Result<DescribeTaskDefinitionResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_task_definition request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecs",
                "AmazonEC2ContainerServiceV20141113.DescribeTaskDefinition",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_task_definition response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_task_definition response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Modifies the parameters of a service. On March 21, 2024, a change was made to resolve
    /// the task definition revision before authorization. When a task definition revision is
    /// not specified, authorization will occur using the latest revision of a task definition.
    /// For services using the rolling update (ECS) you can update the desired count, deployment
    /// configuration, network configuration, load balancers, service registries, enable ECS
    /// managed tags option, propagate tags option, task placement constraints and strategies,
    /// and task definition. When you update any of these parameters, Amazon ECS starts new
    /// tasks with the new configuration. You can attach Amazon EBS volumes to Amazon ECS tasks
    /// by configuring the volume when starting or running a task, or when creating or updating
    /// a service. For more information, see Amazon EBS volumes in the Amazon Elastic Container
    /// Service Developer Guide. You can update your volume configurations and trigger a new
    /// deployment. volumeConfigurations is only supported for REPLICA service and not DAEMON
    /// service. If you leave volumeConfigurations null, it doesn't trigger a new deployment.
    /// For more information on volumes, see Amazon EBS volumes in the Amazon Elastic Container
    /// Service Developer Guide. For services using the blue/green (CODE_DEPLOY) deployment
    /// controller, only the desired count, deployment configuration, health check grace period,
    /// task placement constraints and strategies, enable ECS managed tags option, and propagate
    /// tags can be updated using this API. If the network configuration, platform version, task
    /// definition, or load balancer need to be updated, create a new CodeDeploy deployment. For
    /// more information, see CreateDeployment in the CodeDeploy API Reference. For services
    /// using an external deployment controller, you can update only the desired count, task
    /// placement constraints and strategies, health check grace period, enable ECS managed tags
    /// option, and propagate tags option, using this API. If the launch type, load balancer,
    /// network configuration, platform version, or task definition need to be updated, create a
    /// new task set For more information, see CreateTaskSet. You can add to or subtract from
    /// the number of instantiations of a task definition in a service by specifying the cluster
    /// that the service is running in and a new desiredCount parameter. You can attach Amazon
    /// EBS volumes to Amazon ECS tasks by configuring the volume when starting or running a
    /// task, or when creating or updating a service. For more information, see Amazon EBS
    /// volumes in the Amazon Elastic Container Service Developer Guide. If you have updated the
    /// container image of your application, you can create a new task definition with that
    /// image and deploy it to your service. The service scheduler uses the minimum healthy
    /// percent and maximum percent parameters (in the service's deployment configuration) to
    /// determine the deployment strategy. If your updated Docker image uses the same tag as
    /// what is in the existing task definition for your service (for example, my_image:latest),
    /// you don't need to create a new revision of your task definition. You can update the
    /// service using the forceNewDeployment option. The new tasks launched by the deployment
    /// pull the current image/tag combination from your repository when they start. You can
    /// also update the deployment configuration of a service. When a deployment is triggered by
    /// updating the task definition of a service, the service scheduler uses the deployment
    /// configuration parameters, minimumHealthyPercent and maximumPercent, to determine the
    /// deployment strategy. If minimumHealthyPercent is below 100%, the scheduler can ignore
    /// desiredCount temporarily during a deployment. For example, if desiredCount is four
    /// tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting
    /// two new tasks. Tasks for services that don't use a load balancer are considered healthy
    /// if they're in the RUNNING state. Tasks for services that use a load balancer are
    /// considered healthy if they're in the RUNNING state and are reported as healthy by the
    /// load balancer. The maximumPercent parameter represents an upper limit on the number of
    /// running tasks during a deployment. You can use it to define the deployment batch size.
    /// For example, if desiredCount is four tasks, a maximum of 200% starts four new tasks
    /// before stopping the four older tasks (provided that the cluster resources required to do
    /// this are available). When UpdateService stops a task during a deployment, the equivalent
    /// of docker stop is issued to the containers running in the task. This results in a
    /// SIGTERM and a 30-second timeout. After this, SIGKILL is sent and the containers are
    /// forcibly stopped. If the container handles the SIGTERM gracefully and exits within 30
    /// seconds from receiving it, no SIGKILL is sent. When the service scheduler launches new
    /// tasks, it determines task placement in your cluster with the following logic. Determine
    /// which of the container instances in your cluster can support your service's task
    /// definition. For example, they have the required CPU, memory, ports, and container
    /// instance attributes. By default, the service scheduler attempts to balance tasks across
    /// Availability Zones in this manner even though you can choose a different placement
    /// strategy. Sort the valid container instances by the fewest number of running tasks for
    /// this service in the same Availability Zone as the instance. For example, if zone A has
    /// one running service task and zones B and C each have zero, valid container instances in
    /// either zone B or C are considered optimal for placement. Place the new service task on a
    /// valid container instance in an optimal Availability Zone (based on the previous steps),
    /// favoring container instances with the fewest number of running tasks for this service.
    /// When the service scheduler stops running tasks, it attempts to maintain balance across
    /// the Availability Zones in your cluster using the following logic: Sort the container
    /// instances by the largest number of running tasks for this service in the same
    /// Availability Zone as the instance. For example, if zone A has one running service task
    /// and zones B and C each have two, container instances in either zone B or C are
    /// considered optimal for termination. Stop the task on a container instance in an optimal
    /// Availability Zone (based on the previous steps), favoring container instances with the
    /// largest number of running tasks for this service.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`UpdateServiceRequest`]
    ///
    /// # Response
    /// [`UpdateServiceResponse`]
    #[allow(dead_code)]
    pub(crate) async fn update_service(
        &self,
        body: &UpdateServiceRequest,
    ) -> Result<UpdateServiceResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize update_service request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecs",
                "AmazonEC2ContainerServiceV20141113.UpdateService",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read update_service response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse update_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deregisters the specified task definition by family and revision. Upon deregistration,
    /// the task definition is marked as INACTIVE. Existing tasks and services that reference an
    /// INACTIVE task definition continue to run without disruption. Existing services that
    /// reference an INACTIVE task definition can still scale up or down by modifying the
    /// service's desired count. If you want to delete a task definition revision, you must
    /// first deregister the task definition revision. You can't use an INACTIVE task definition
    /// to run new tasks or create new services, and you can't update an existing service to
    /// reference an INACTIVE task definition. However, there may be up to a 10-minute window
    /// following deregistration where these restrictions have not yet taken effect. At this
    /// time, INACTIVE task definitions remain discoverable in your account indefinitely.
    /// However, this behavior is subject to change in the future. We don't recommend that you
    /// rely on INACTIVE task definitions persisting beyond the lifecycle of any associated
    /// tasks and services. You must deregister a task definition revision before you delete it.
    /// For more information, see DeleteTaskDefinitions.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeregisterTaskDefinitionRequest`]
    ///
    /// # Response
    /// [`DeregisterTaskDefinitionResponse`]
    #[allow(dead_code)]
    pub(crate) async fn deregister_task_definition(
        &self,
        body: &DeregisterTaskDefinitionRequest,
    ) -> Result<DeregisterTaskDefinitionResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize deregister_task_definition request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "ecs",
                "AmazonEC2ContainerServiceV20141113.DeregisterTaskDefinition",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read deregister_task_definition response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse deregister_task_definition response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_clusters() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(ListClustersResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcsOps::new(&client);

        let body = ListClustersRequest::fixture();
        let result = ops.list_clusters(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_clusters() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeClustersResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcsOps::new(&client);

        let body = DescribeClustersRequest::fixture();
        let result = ops.describe_clusters(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_services() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(ListServicesResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcsOps::new(&client);

        let body = ListServicesRequest::fixture();
        let result = ops.list_services(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_services() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeServicesResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcsOps::new(&client);

        let body = DescribeServicesRequest::fixture();
        let result = ops.describe_services(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_task_definition() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(
            serde_json::to_value(DescribeTaskDefinitionResponse::fixture()).unwrap(),
        );

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcsOps::new(&client);

        let body = DescribeTaskDefinitionRequest::fixture();
        let result = ops.describe_task_definition(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(UpdateServiceResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcsOps::new(&client);

        let body = UpdateServiceRequest::fixture();
        let result = ops.update_service(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_deregister_task_definition() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(
            serde_json::to_value(DeregisterTaskDefinitionResponse::fixture()).unwrap(),
        );

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = EcsOps::new(&client);

        let body = DeregisterTaskDefinitionRequest::fixture();
        let result = ops.deregister_task_definition(&body).await;
        assert!(result.is_ok());
    }
}
