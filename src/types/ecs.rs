//! Types for the Amazon Elastic Container Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///
/// **AWS API**: `ecs.v1.ListClustersRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//ListClustersRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListClustersRequest {
    /// The nextToken value returned from a ListClusters request indicating that more results
    /// are available to fulfill the request and further calls are needed. If maxResults was
    /// provided, it's possible the number of results to be fewer than maxResults. This token
    /// should be treated as an opaque identifier that is only used to retrieve the next items
    /// in a list and not for other programmatic purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of cluster results that ListClusters returned in paginated output.
    /// When this parameter is used, ListClusters only returns maxResults results in a single
    /// page along with a nextToken response element. The remaining results of the initial
    /// request can be seen by sending another ListClusters request with the returned nextToken
    /// value. This value can be between 1 and 100. If this parameter isn't used, then
    /// ListClusters returns up to 100 results and a nextToken value if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl ListClustersRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `ecs.v1.ListClustersResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//ListClustersResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListClustersResponse {
    /// The list of full Amazon Resource Name (ARN) entries for each cluster that's associated
    /// with your account.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cluster_arns: Vec<String>,

    /// The nextToken value to include in a future ListClusters request. When the results of a
    /// ListClusters request exceed maxResults, this value can be used to retrieve the next page
    /// of results. This value is null when there are no more results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListClustersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_arns: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

///
/// **AWS API**: `ecs.v1.DescribeClustersRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//DescribeClustersRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeClustersRequest {
    /// A list of up to 100 cluster names or full cluster Amazon Resource Name (ARN) entries. If
    /// you do not specify a cluster, the default cluster is assumed.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clusters: Vec<String>,

    /// Determines whether to include additional information about the clusters in the response.
    /// If this field is omitted, this information isn't included. If ATTACHMENTS is specified,
    /// the attachments for the container instances or tasks within the cluster are included,
    /// for example the capacity providers. If SETTINGS is specified, the settings for the
    /// cluster are included. If CONFIGURATIONS is specified, the configuration for the cluster
    /// is included. If STATISTICS is specified, the task and service count is included,
    /// separated by launch type. If TAGS is specified, the metadata tags associated with the
    /// cluster are included.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
}

impl DescribeClustersRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            clusters: vec![],
            include: vec![],
        }
    }
}

///
/// **AWS API**: `ecs.v1.DescribeClustersResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//DescribeClustersResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeClustersResponse {
    /// The list of clusters.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clusters: Vec<Cluster>,

    /// Any failures associated with the call.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<Failure>,
}

impl DescribeClustersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            clusters: vec![],
            failures: vec![],
        }
    }
}

/// A regional grouping of one or more container instances where you can run task requests. Each
/// account receives a default cluster the first time you use the Amazon ECS service, but you
/// may also create other clusters. Clusters may contain more than one instance type
/// simultaneously.
///
/// **AWS API**: `ecs.v1.Cluster`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//Cluster>
///
/// ## Coverage
/// 10 of 16 fields included.
/// Omitted fields:
/// - `configuration` — not selected in manifest
/// - `statistics` — not selected in manifest
/// - `defaultCapacityProviderStrategy` — not selected in manifest
/// - `attachments` — not selected in manifest
/// - `attachmentsStatus` — not selected in manifest
/// - `serviceConnectDefaults` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    /// The Amazon Resource Name (ARN) that identifies the cluster. For more information about
    /// the ARN format, see Amazon Resource Name (ARN) in the Amazon ECS Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,

    /// A user-generated string that you use to identify your cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,

    /// The status of the cluster. The following are the possible states that are returned.
    /// ACTIVE The cluster is ready to accept tasks and if applicable you can register container
    /// instances with the cluster. PROVISIONING The cluster has capacity providers that are
    /// associated with it and the resources needed for the capacity provider are being created.
    /// DEPROVISIONING The cluster has capacity providers that are associated with it and the
    /// resources needed for the capacity provider are being deleted. FAILED The cluster has
    /// capacity providers that are associated with it and the resources needed for the capacity
    /// provider have failed to create. INACTIVE The cluster has been deleted. Clusters with an
    /// INACTIVE status may remain discoverable in your account for a period of time. However,
    /// this behavior is subject to change in the future. We don't recommend that you rely on
    /// INACTIVE clusters persisting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The number of container instances registered into the cluster. This includes container
    /// instances in both ACTIVE and DRAINING status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_container_instances_count: Option<i32>,

    /// The number of tasks in the cluster that are in the RUNNING state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_tasks_count: Option<i32>,

    /// The number of tasks in the cluster that are in the PENDING state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_tasks_count: Option<i32>,

    /// The number of services that are running on the cluster in an ACTIVE state. You can view
    /// these services with ListServices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_services_count: Option<i32>,

    /// The metadata that you apply to the cluster to help you categorize and organize them.
    /// Each tag consists of a key and an optional value. You define both. The following basic
    /// restrictions apply to tags: Maximum number of tags per resource - 50 For each resource,
    /// each tag key must be unique, and each tag key can have only one value. Maximum key
    /// length - 128 Unicode characters in UTF-8 Maximum value length - 256 Unicode characters
    /// in UTF-8 If your tagging schema is used across multiple services and resources, remember
    /// that other services may have restrictions on allowed characters. Generally allowed
    /// characters are: letters, numbers, and spaces representable in UTF-8, and the following
    /// characters: + - = . _ : / @. Tag keys and values are case-sensitive. Do not use aws:,
    /// AWS:, or any upper or lowercase combination of such as a prefix for either keys or
    /// values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys
    /// or values with this prefix. Tags with this prefix do not count against your tags per
    /// resource limit.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,

    /// The settings for the cluster. This parameter indicates whether CloudWatch Container
    /// Insights is on or off for a cluster.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub settings: Vec<ClusterSetting>,

    /// The capacity providers associated with the cluster.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub capacity_providers: Vec<String>,
}

impl Cluster {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster_arn: Some("test-cluster_arn".into()),
            cluster_name: Some("test-cluster_name".into()),
            status: Some("test-status".into()),
            registered_container_instances_count: Some(100),
            running_tasks_count: Some(100),
            pending_tasks_count: Some(100),
            active_services_count: Some(100),
            tags: vec![],
            settings: vec![],
            capacity_providers: vec![],
        }
    }
}

/// A key-value pair object.
///
/// **AWS API**: `ecs.v1.KeyValuePair`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//KeyValuePair>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePair {
    /// The name of the key-value pair. For environment variables, this is the name of the
    /// environment variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The value of the key-value pair. For environment variables, this is the value of the
    /// environment variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl KeyValuePair {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-key_value_pair".into()),
            value: Some("test-value".into()),
        }
    }
}

/// The settings to use when creating a cluster. This parameter is used to turn on CloudWatch
/// Container Insights with enhanced observability or CloudWatch Container Insights for a
/// cluster. Container Insights with enhanced observability provides all the Container Insights
/// metrics, plus additional task and container metrics. This version supports enhanced
/// observability for Amazon ECS clusters using the Amazon EC2 and Fargate launch types. After
/// you configure Container Insights with enhanced observability on Amazon ECS, Container
/// Insights auto-collects detailed infrastructure telemetry from the cluster level down to the
/// container level in your environment and displays these critical performance data in curated
/// dashboards removing the heavy lifting in observability set-up. For more information, see
/// Monitor Amazon ECS containers using Container Insights with enhanced observability in the
/// Amazon Elastic Container Service Developer Guide.
///
/// **AWS API**: `ecs.v1.ClusterSetting`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//ClusterSetting>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSetting {
    /// The name of the cluster setting. The value is containerInsights.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The value to set for the cluster setting. The supported values are enhanced, enabled,
    /// and disabled. To use Container Insights with enhanced observability, set the
    /// containerInsights account setting to enhanced. To use Container Insights, set the
    /// containerInsights account setting to enabled. If a cluster value is specified, it will
    /// override the containerInsights value set with PutAccountSetting or
    /// PutAccountSettingDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ClusterSetting {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-cluster_setting".into()),
            value: Some("test-value".into()),
        }
    }
}

/// The metadata that you apply to a resource to help you categorize and organize them. Each tag
/// consists of a key and an optional value. You define them. The following basic restrictions
/// apply to tags: Maximum number of tags per resource - 50 For each resource, each tag key must
/// be unique, and each tag key can have only one value. Maximum key length - 128 Unicode
/// characters in UTF-8 Maximum value length - 256 Unicode characters in UTF-8 If your tagging
/// schema is used across multiple services and resources, remember that other services may have
/// restrictions on allowed characters. Generally allowed characters are: letters, numbers, and
/// spaces representable in UTF-8, and the following characters: + - = . _ : / @. Tag keys and
/// values are case-sensitive. Do not use aws:, AWS:, or any upper or lowercase combination of
/// such as a prefix for either keys or values as it is reserved for Amazon Web Services use.
/// You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not
/// count against your tags per resource limit.
///
/// **AWS API**: `ecs.v1.Tag`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//Tag>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// One part of a key-value pair that make up a tag. A key is a general label that acts like
    /// a category for more specific tag values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// The optional part of a key-value pair that make up a tag. A value acts as a descriptor
    /// within a tag category (key).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Tag {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key: Some("test-key".into()),
            value: Some("test-value".into()),
        }
    }
}

/// A failed resource. For a list of common causes, see API failure reasons in the Amazon
/// Elastic Container Service Developer Guide.
///
/// **AWS API**: `ecs.v1.Failure`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//Failure>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Failure {
    /// The Amazon Resource Name (ARN) of the failed resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The reason for the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// The details of the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl Failure {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            arn: Some("test-arn".into()),
            reason: Some("test-reason".into()),
            detail: Some("test-detail".into()),
        }
    }
}

///
/// **AWS API**: `ecs.v1.ListServicesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//ListServicesRequest>
///
/// ## Coverage
/// 5 of 6 fields included.
/// Omitted fields:
/// - `resourceManagementType` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListServicesRequest {
    /// The short name or full Amazon Resource Name (ARN) of the cluster to use when filtering
    /// the ListServices results. If you do not specify a cluster, the default cluster is
    /// assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,

    /// The nextToken value returned from a ListServices request indicating that more results
    /// are available to fulfill the request and further calls will be needed. If maxResults was
    /// provided, it is possible the number of results to be fewer than maxResults. This token
    /// should be treated as an opaque identifier that is only used to retrieve the next items
    /// in a list and not for other programmatic purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of service results that ListServices returned in paginated output.
    /// When this parameter is used, ListServices only returns maxResults results in a single
    /// page along with a nextToken response element. The remaining results of the initial
    /// request can be seen by sending another ListServices request with the returned nextToken
    /// value. This value can be between 1 and 100. If this parameter isn't used, then
    /// ListServices returns up to 10 results and a nextToken value if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,

    /// The launch type to use when filtering the ListServices results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,

    /// The scheduling strategy to use when filtering the ListServices results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
}

impl ListServicesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster: Some("test-cluster".into()),
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
            launch_type: Some("test-launch_type".into()),
            scheduling_strategy: Some("test-scheduling_strategy".into()),
        }
    }
}

///
/// **AWS API**: `ecs.v1.ListServicesResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//ListServicesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListServicesResponse {
    /// The list of full ARN entries for each service that's associated with the specified
    /// cluster.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service_arns: Vec<String>,

    /// The nextToken value to include in a future ListServices request. When the results of a
    /// ListServices request exceed maxResults, this value can be used to retrieve the next page
    /// of results. This value is null when there are no more results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListServicesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            service_arns: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

///
/// **AWS API**: `ecs.v1.DescribeServicesRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//DescribeServicesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeServicesRequest {
    /// The short name or full Amazon Resource Name (ARN)the cluster that hosts the service to
    /// describe. If you do not specify a cluster, the default cluster is assumed. This
    /// parameter is required if the service or services you are describing were launched in any
    /// cluster other than the default cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,

    /// A list of services to describe. You may specify up to 10 services to describe in a
    /// single operation.
    #[serde(default)]
    pub services: Vec<String>,

    /// Determines whether you want to see the resource tags for the service. If TAGS is
    /// specified, the tags are included in the response. If this field is omitted, tags aren't
    /// included in the response.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
}

impl DescribeServicesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster: Some("test-cluster".into()),
            services: vec![],
            include: vec![],
        }
    }
}

///
/// **AWS API**: `ecs.v1.DescribeServicesResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//DescribeServicesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeServicesResponse {
    /// The list of services described.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<Service>,

    /// Any failures associated with the call.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<Failure>,
}

impl DescribeServicesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            services: vec![],
            failures: vec![],
        }
    }
}

/// Details on a service within a cluster.
///
/// **AWS API**: `ecs.v1.Service`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//Service>
///
/// ## Coverage
/// 17 of 35 fields included.
/// Omitted fields:
/// - `serviceRegistries` — not selected in manifest
/// - `capacityProviderStrategy` — not selected in manifest
/// - `platformFamily` — not selected in manifest
/// - `deploymentConfiguration` — not selected in manifest
/// - `taskSets` — not selected in manifest
/// - `roleArn` — not selected in manifest
/// - `currentServiceDeployment` — not selected in manifest
/// - `currentServiceRevisions` — not selected in manifest
/// - `placementConstraints` — not selected in manifest
/// - `placementStrategy` — not selected in manifest
/// - `healthCheckGracePeriodSeconds` — not selected in manifest
/// - `deploymentController` — not selected in manifest
/// - `tags` — not selected in manifest
/// - `createdBy` — not selected in manifest
/// - `enableECSManagedTags` — not selected in manifest
/// - `propagateTags` — not selected in manifest
/// - `availabilityZoneRebalancing` — not selected in manifest
/// - `resourceManagementType` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    /// The ARN that identifies the service. For more information about the ARN format, see
    /// Amazon Resource Name (ARN) in the Amazon ECS Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,

    /// The name of your service. Up to 255 letters (uppercase and lowercase), numbers,
    /// underscores, and hyphens are allowed. Service names must be unique within a cluster.
    /// However, you can have similarly named services in multiple clusters within a Region or
    /// across multiple Regions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,

    /// The Amazon Resource Name (ARN) of the cluster that hosts the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,

    /// A list of Elastic Load Balancing load balancer objects. It contains the load balancer
    /// name, the container name, and the container port to access from the load balancer. The
    /// container name is as it appears in a container definition.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub load_balancers: Vec<LoadBalancer>,

    /// The status of the service. The valid values are ACTIVE, DRAINING, or INACTIVE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The desired number of instantiations of the task definition to keep running on the
    /// service. This value is specified when the service is created with CreateService , and it
    /// can be modified with UpdateService.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i32>,

    /// The number of tasks in the cluster that are in the RUNNING state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,

    /// The number of tasks in the cluster that are in the PENDING state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,

    /// The launch type the service is using. When using the DescribeServices API, this field is
    /// omitted if the service was created using a capacity provider strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,

    /// The task definition to use for tasks in the service. This value is specified when the
    /// service is created with CreateService, and it can be modified with UpdateService.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,

    /// The current state of deployments for the service.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deployments: Vec<Deployment>,

    /// The event stream for your service. A maximum of 100 of the latest events are displayed.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<ServiceEvent>,

    /// The Unix timestamp for the time when the service was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,

    /// The platform version to run your service on. A platform version is only specified for
    /// tasks that are hosted on Fargate. If one isn't specified, the LATEST platform version is
    /// used. For more information, see Fargate Platform Versions in the Amazon Elastic
    /// Container Service Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,

    /// The VPC subnet and security group configuration for tasks that receive their own elastic
    /// network interface by using the awsvpc networking mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,

    /// The scheduling strategy to use for the service. For more information, see Services.
    /// There are two service scheduler strategies available. REPLICA-The replica scheduling
    /// strategy places and maintains the desired number of tasks across your cluster. By
    /// default, the service scheduler spreads tasks across Availability Zones. You can use task
    /// placement strategies and constraints to customize task placement decisions. DAEMON-The
    /// daemon scheduling strategy deploys exactly one task on each active container instance.
    /// This task meets all of the task placement constraints that you specify in your cluster.
    /// The service scheduler also evaluates the task placement constraints for running tasks.
    /// It stop tasks that don't meet the placement constraints. Fargate tasks don't support the
    /// DAEMON scheduling strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,

    /// Determines whether the execute command functionality is turned on for the service. If
    /// true, the execute command functionality is turned on for all containers in tasks as part
    /// of the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
}

impl Service {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            service_arn: Some("test-service_arn".into()),
            service_name: Some("test-service_name".into()),
            cluster_arn: Some("test-cluster_arn".into()),
            load_balancers: vec![],
            status: Some("test-status".into()),
            desired_count: Some(100),
            running_count: Some(100),
            pending_count: Some(100),
            launch_type: Some("test-launch_type".into()),
            task_definition: Some("test-task_definition".into()),
            deployments: vec![],
            events: vec![],
            platform_version: Some("test-platform_version".into()),
            network_configuration: Some(NetworkConfiguration::fixture()),
            scheduling_strategy: Some("test-scheduling_strategy".into()),
            enable_execute_command: Some(false),
            ..Default::default()
        }
    }
}

/// The details of an Amazon ECS service deployment. This is used only when a service uses the
/// ECS deployment controller type.
///
/// **AWS API**: `ecs.v1.Deployment`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//Deployment>
///
/// ## Coverage
/// 13 of 21 fields included.
/// Omitted fields:
/// - `capacityProviderStrategy` — not selected in manifest
/// - `platformFamily` — not selected in manifest
/// - `networkConfiguration` — not selected in manifest
/// - `serviceConnectConfiguration` — not selected in manifest
/// - `serviceConnectResources` — not selected in manifest
/// - `volumeConfigurations` — not selected in manifest
/// - `fargateEphemeralStorage` — not selected in manifest
/// - `vpcLatticeConfigurations` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    /// The ID of the deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of the deployment. The following describes each state. PRIMARY The most
    /// recent deployment of a service. ACTIVE A service deployment that still has running
    /// tasks, but are in the process of being replaced with a new PRIMARY deployment. INACTIVE
    /// A deployment that has been completely replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The most recent task definition that was specified for the tasks in the service to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,

    /// The most recent desired count of tasks that was specified for the service to deploy or
    /// maintain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i32>,

    /// The number of tasks in the deployment that are in the PENDING status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,

    /// The number of tasks in the deployment that are in the RUNNING status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,

    /// The number of consecutively failed tasks in the deployment. A task is considered a
    /// failure if the service scheduler can't launch the task, the task doesn't transition to a
    /// RUNNING state, or if it fails any of its defined health checks and is stopped. Once a
    /// service deployment has one or more successfully running tasks, the failed task count
    /// resets to zero and stops being evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_tasks: Option<i32>,

    /// The Unix timestamp for the time when the service deployment was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,

    /// The Unix timestamp for the time when the service deployment was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,

    /// The launch type the tasks in the service are using. For more information, see Amazon ECS
    /// Launch Types in the Amazon Elastic Container Service Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,

    /// The platform version that your tasks in the service run on. A platform version is only
    /// specified for tasks using the Fargate launch type. If one isn't specified, the LATEST
    /// platform version is used. For more information, see Fargate Platform Versions in the
    /// Amazon Elastic Container Service Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,

    /// The rolloutState of a service is only returned for services that use the rolling update
    /// (ECS) deployment type that aren't behind a Classic Load Balancer. The rollout state of
    /// the deployment. When a service deployment is started, it begins in an IN_PROGRESS state.
    /// When the service reaches a steady state, the deployment transitions to a COMPLETED
    /// state. If the service fails to reach a steady state and circuit breaker is turned on,
    /// the deployment transitions to a FAILED state. A deployment in FAILED state doesn't
    /// launch any new tasks. For more information, see DeploymentCircuitBreaker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_state: Option<String>,

    /// A description of the rollout state of a deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_state_reason: Option<String>,
}

impl Deployment {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            status: Some("test-status".into()),
            task_definition: Some("test-task_definition".into()),
            desired_count: Some(100),
            pending_count: Some(100),
            running_count: Some(100),
            failed_tasks: Some(100),
            launch_type: Some("test-launch_type".into()),
            platform_version: Some("test-platform_version".into()),
            rollout_state: Some("test-rollout_state".into()),
            rollout_state_reason: Some("test-rollout_state_reason".into()),
            ..Default::default()
        }
    }
}

/// The details for an event that's associated with a service.
///
/// **AWS API**: `ecs.v1.ServiceEvent`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//ServiceEvent>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceEvent {
    /// The ID string for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The Unix timestamp for the time when the event was triggered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,

    /// The event message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ServiceEvent {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            message: Some("test-message".into()),
            ..Default::default()
        }
    }
}

/// The load balancer configuration to use with a service or task set. When you add, update, or
/// remove a load balancer configuration, Amazon ECS starts a new deployment with the updated
/// Elastic Load Balancing configuration. This causes tasks to register to and deregister from
/// load balancers. We recommend that you verify this on a test environment before you update
/// the Elastic Load Balancing configuration. A service-linked role is required for services
/// that use multiple target groups. For more information, see Using service-linked roles in the
/// Amazon Elastic Container Service Developer Guide.
///
/// **AWS API**: `ecs.v1.LoadBalancer`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//LoadBalancer>
///
/// ## Coverage
/// 4 of 5 fields included.
/// Omitted fields:
/// - `advancedConfiguration` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancer {
    /// The full Amazon Resource Name (ARN) of the Elastic Load Balancing target group or groups
    /// associated with a service or task set. A target group ARN is only specified when using
    /// an Application Load Balancer or Network Load Balancer. For services using the ECS
    /// deployment controller, you can specify one or multiple target groups. For more
    /// information, see Registering multiple target groups with a service in the Amazon Elastic
    /// Container Service Developer Guide. For services using the CODE_DEPLOY deployment
    /// controller, you're required to define two target groups for the load balancer. For more
    /// information, see Blue/green deployment with CodeDeploy in the Amazon Elastic Container
    /// Service Developer Guide. If your service's task definition uses the awsvpc network mode,
    /// you must choose ip as the target type, not instance. Do this when creating your target
    /// groups because tasks that use the awsvpc network mode are associated with an elastic
    /// network interface, not an Amazon EC2 instance. This network mode is required for the
    /// Fargate launch type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,

    /// The name of the load balancer to associate with the Amazon ECS service or task set. If
    /// you are using an Application Load Balancer or a Network Load Balancer the load balancer
    /// name parameter should be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,

    /// The name of the container (as it appears in a container definition) to associate with
    /// the load balancer. You need to specify the container name when configuring the target
    /// group for an Amazon ECS load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,

    /// The port on the container to associate with the load balancer. This port must correspond
    /// to a containerPort in the task definition the tasks in the service are using. For tasks
    /// that use the EC2 launch type, the container instance they're launched on must allow
    /// ingress traffic on the hostPort of the port mapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,
}

impl LoadBalancer {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            target_group_arn: Some("test-target_group_arn".into()),
            load_balancer_name: Some("test-load_balancer_name".into()),
            container_name: Some("test-container_name".into()),
            container_port: Some(100),
        }
    }
}

/// The network configuration for a task or service.
///
/// **AWS API**: `ecs.v1.NetworkConfiguration`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//NetworkConfiguration>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConfiguration {
    /// The VPC subnets and security groups that are associated with a task. All specified
    /// subnets and security groups must be from the same VPC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

impl NetworkConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            awsvpc_configuration: Some(AwsVpcConfiguration::fixture()),
        }
    }
}

/// An object representing the networking details for a task or service. For example
/// awsVpcConfiguration={subnets=["subnet-12344321"],securityGroups=["sg-12344321"]}.
///
/// **AWS API**: `ecs.v1.AwsVpcConfiguration`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//AwsVpcConfiguration>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwsVpcConfiguration {
    /// The IDs of the subnets associated with the task or service. There's a limit of 16
    /// subnets that can be specified. All specified subnets must be from the same VPC.
    #[serde(default)]
    pub subnets: Vec<String>,

    /// The IDs of the security groups associated with the task or service. If you don't specify
    /// a security group, the default security group for the VPC is used. There's a limit of 5
    /// security groups that can be specified. All specified security groups must be from the
    /// same VPC.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_groups: Vec<String>,

    /// Whether the task's elastic network interface receives a public IP address. Consider the
    /// following when you set this value: When you use create-service or update-service, the
    /// default is DISABLED. When the service deploymentController is ECS, the value must be
    /// DISABLED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
}

impl AwsVpcConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            subnets: vec![],
            security_groups: vec![],
            assign_public_ip: Some("test-assign_public_ip".into()),
        }
    }
}

///
/// **AWS API**: `ecs.v1.DescribeTaskDefinitionRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//DescribeTaskDefinitionRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeTaskDefinitionRequest {
    /// The family for the latest ACTIVE revision, family and revision (family:revision) for a
    /// specific revision in the family, or full Amazon Resource Name (ARN) of the task
    /// definition to describe.
    pub task_definition: String,

    /// Determines whether to see the resource tags for the task definition. If TAGS is
    /// specified, the tags are included in the response. If this field is omitted, tags aren't
    /// included in the response.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
}

impl DescribeTaskDefinitionRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            task_definition: "test-task_definition".into(),
            include: vec![],
        }
    }
}

///
/// **AWS API**: `ecs.v1.DescribeTaskDefinitionResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//DescribeTaskDefinitionResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeTaskDefinitionResponse {
    /// The full task definition description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,

    /// The metadata that's applied to the task definition to help you categorize and organize
    /// them. Each tag consists of a key and an optional value. You define both. The following
    /// basic restrictions apply to tags: Maximum number of tags per resource - 50 For each
    /// resource, each tag key must be unique, and each tag key can have only one value. Maximum
    /// key length - 128 Unicode characters in UTF-8 Maximum value length - 256 Unicode
    /// characters in UTF-8 If your tagging schema is used across multiple services and
    /// resources, remember that other services may have restrictions on allowed characters.
    /// Generally allowed characters are: letters, numbers, and spaces representable in UTF-8,
    /// and the following characters: + - = . _ : / @. Tag keys and values are case-sensitive.
    /// Do not use aws:, AWS:, or any upper or lowercase combination of such as a prefix for
    /// either keys or values as it is reserved for Amazon Web Services use. You cannot edit or
    /// delete tag keys or values with this prefix. Tags with this prefix do not count against
    /// your tags per resource limit.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

impl DescribeTaskDefinitionResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            task_definition: Some(TaskDefinition::fixture()),
            tags: vec![],
        }
    }
}

/// The details of a task definition which describes the container and volume definitions of an
/// Amazon Elastic Container Service task. You can specify which Docker images to use, the
/// required resources, and other configurations related to launching the task definition
/// through an Amazon ECS service or task.
///
/// **AWS API**: `ecs.v1.TaskDefinition`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//TaskDefinition>
///
/// ## Coverage
/// 14 of 25 fields included.
/// Omitted fields:
/// - `volumes` — not selected in manifest
/// - `requiresAttributes` — not selected in manifest
/// - `placementConstraints` — not selected in manifest
/// - `runtimePlatform` — not selected in manifest
/// - `inferenceAccelerators` — not selected in manifest
/// - `pidMode` — not selected in manifest
/// - `ipcMode` — not selected in manifest
/// - `proxyConfiguration` — not selected in manifest
/// - `registeredBy` — not selected in manifest
/// - `ephemeralStorage` — not selected in manifest
/// - `enableFaultInjection` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDefinition {
    /// The full Amazon Resource Name (ARN) of the task definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arn: Option<String>,

    /// A list of container definitions in JSON format that describe the different containers
    /// that make up your task. For more information about container definition parameters and
    /// defaults, see Amazon ECS Task Definitions in the Amazon Elastic Container Service
    /// Developer Guide.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub container_definitions: Vec<ContainerDefinition>,

    /// The name of a family that this task definition is registered to. Up to 255 characters
    /// are allowed. Letters (both uppercase and lowercase letters), numbers, hyphens (-), and
    /// underscores (_) are allowed. A family groups multiple versions of a task definition.
    /// Amazon ECS gives the first task definition that you registered to a family a revision
    /// number of 1. Amazon ECS gives sequential revision numbers to each task definition that
    /// you add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,

    /// The short name or full Amazon Resource Name (ARN) of the Identity and Access Management
    /// role that grants containers in the task permission to call Amazon Web Services APIs on
    /// your behalf. For informationabout the required IAM roles for Amazon ECS, see IAM roles
    /// for Amazon ECS in the Amazon Elastic Container Service Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,

    /// The Amazon Resource Name (ARN) of the task execution role that grants the Amazon ECS
    /// container agent permission to make Amazon Web Services API calls on your behalf. For
    /// informationabout the required IAM roles for Amazon ECS, see IAM roles for Amazon ECS in
    /// the Amazon Elastic Container Service Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,

    /// The Docker networking mode to use for the containers in the task. The valid values are
    /// none, bridge, awsvpc, and host. If no network mode is specified, the default is bridge.
    /// For Amazon ECS tasks on Fargate, the awsvpc network mode is required. For Amazon ECS
    /// tasks on Amazon EC2 Linux instances, any network mode can be used. For Amazon ECS tasks
    /// on Amazon EC2 Windows instances, &lt;default&gt; or awsvpc can be used. If the network
    /// mode is set to none, you cannot specify port mappings in your container definitions, and
    /// the tasks containers do not have external connectivity. The host and awsvpc network
    /// modes offer the highest networking performance for containers because they use the EC2
    /// network stack instead of the virtualized network stack provided by the bridge mode. With
    /// the host and awsvpc network modes, exposed container ports are mapped directly to the
    /// corresponding host port (for the host network mode) or the attached elastic network
    /// interface port (for the awsvpc network mode), so you cannot take advantage of dynamic
    /// host port mappings. When using the host network mode, you should not run containers
    /// using the root user (UID 0). It is considered best practice to use a non-root user. If
    /// the network mode is awsvpc, the task is allocated an elastic network interface, and you
    /// must specify a NetworkConfiguration value when you create a service or run a task with
    /// the task definition. For more information, see Task Networking in the Amazon Elastic
    /// Container Service Developer Guide. If the network mode is host, you cannot run multiple
    /// instantiations of the same task on a single container instance when port mappings are
    /// used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,

    /// The revision of the task in a particular family. The revision is a version number of a
    /// task definition in a family. When you register a task definition for the first time, the
    /// revision is 1. Each time that you register a new revision of a task definition in the
    /// same family, the revision value always increases by one. This is even if you
    /// deregistered previous revisions in this family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,

    /// The status of the task definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Amazon ECS validates the task definition parameters with those supported by the launch
    /// type. For more information, see Amazon ECS launch types in the Amazon Elastic Container
    /// Service Developer Guide.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub compatibilities: Vec<String>,

    /// The task launch types the task definition was validated against. The valid values are
    /// MANAGED_INSTANCES, EC2, FARGATE, and EXTERNAL. For more information, see Amazon ECS
    /// launch types in the Amazon Elastic Container Service Developer Guide.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub requires_compatibilities: Vec<String>,

    /// The number of cpu units used by the task. If you use the EC2 launch type, this field is
    /// optional. Any value can be used. If you use the Fargate launch type, this field is
    /// required. You must use one of the following values. The value that you choose determines
    /// your range of valid values for the memory parameter. If you're using the EC2 launch type
    /// or the external launch type, this field is optional. Supported values are between 128
    /// CPU units (0.125 vCPUs) and 196608 CPU units (192 vCPUs). This field is required for
    /// Fargate. For information about the valid values, see Task size in the Amazon Elastic
    /// Container Service Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,

    /// The amount (in MiB) of memory used by the task. If your tasks runs on Amazon EC2
    /// instances, you must specify either a task-level memory value or a container-level memory
    /// value. This field is optional and any value can be used. If a task-level memory value is
    /// specified, the container-level memory value is optional. For more information regarding
    /// container-level memory and memory reservation, see ContainerDefinition. If your tasks
    /// runs on Fargate, this field is required. You must use one of the following values. The
    /// value you choose determines your range of valid values for the cpu parameter. 512 (0.5
    /// GB), 1024 (1 GB), 2048 (2 GB)
    /// - Available cpu values: 256 (.25 vCPU) 1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4
    ///   GB)
    /// - Available cpu values: 512 (.5 vCPU) 2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5
    ///   GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB)
    /// - Available cpu values: 1024 (1 vCPU) Between 4096 (4 GB) and 16384 (16 GB) in
    ///   increments of 1024 (1 GB)
    /// - Available cpu values: 2048 (2 vCPU) Between 8192 (8 GB) and 30720 (30 GB) in
    ///   increments of 1024 (1 GB)
    /// - Available cpu values: 4096 (4 vCPU) Between 16 GB and 60 GB in 4 GB increments
    /// - Available cpu values: 8192 (8 vCPU) This option requires Linux platform 1.4.0 or
    ///   later. Between 32GB and 120 GB in 8 GB increments
    /// - Available cpu values: 16384 (16 vCPU) This option requires Linux platform 1.4.0 or
    ///   later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,

    /// The Unix timestamp for the time when the task definition was registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<f64>,

    /// The Unix timestamp for the time when the task definition was deregistered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregistered_at: Option<f64>,
}

impl TaskDefinition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            task_definition_arn: Some("test-task_definition_arn".into()),
            container_definitions: vec![],
            family: Some("test-family".into()),
            task_role_arn: Some("test-task_role_arn".into()),
            execution_role_arn: Some("test-execution_role_arn".into()),
            network_mode: Some("test-network_mode".into()),
            revision: Some(100),
            status: Some("test-status".into()),
            compatibilities: vec![],
            requires_compatibilities: vec![],
            cpu: Some("test-cpu".into()),
            memory: Some("test-memory".into()),
            ..Default::default()
        }
    }
}

/// Container definitions are used in task definitions to describe the different containers that
/// are launched as part of a task.
///
/// **AWS API**: `ecs.v1.ContainerDefinition`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//ContainerDefinition>
///
/// ## Coverage
/// 11 of 42 fields included.
/// Omitted fields:
/// - `repositoryCredentials` — not selected in manifest
/// - `links` — not selected in manifest
/// - `restartPolicy` — not selected in manifest
/// - `environmentFiles` — not selected in manifest
/// - `mountPoints` — not selected in manifest
/// - `volumesFrom` — not selected in manifest
/// - `linuxParameters` — not selected in manifest
/// - `secrets` — not selected in manifest
/// - `dependsOn` — not selected in manifest
/// - `startTimeout` — not selected in manifest
/// - `stopTimeout` — not selected in manifest
/// - `versionConsistency` — not selected in manifest
/// - `hostname` — not selected in manifest
/// - `user` — not selected in manifest
/// - `workingDirectory` — not selected in manifest
/// - `disableNetworking` — not selected in manifest
/// - `privileged` — not selected in manifest
/// - `readonlyRootFilesystem` — not selected in manifest
/// - `dnsServers` — not selected in manifest
/// - `dnsSearchDomains` — not selected in manifest
/// - `extraHosts` — not selected in manifest
/// - `dockerSecurityOptions` — not selected in manifest
/// - `interactive` — not selected in manifest
/// - `pseudoTerminal` — not selected in manifest
/// - `dockerLabels` — not selected in manifest
/// - `ulimits` — not selected in manifest
/// - `healthCheck` — not selected in manifest
/// - `systemControls` — not selected in manifest
/// - `resourceRequirements` — not selected in manifest
/// - `firelensConfiguration` — not selected in manifest
/// - `credentialSpecs` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerDefinition {
    /// The name of a container. If you're linking multiple containers together in a task
    /// definition, the name of one container can be entered in the links of another container
    /// to connect the containers. Up to 255 letters (uppercase and lowercase), numbers,
    /// underscores, and hyphens are allowed. This parameter maps to name in the docker
    /// container create command and the --name option to docker run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The image used to start a container. This string is passed directly to the Docker
    /// daemon. By default, images in the Docker Hub registry are available. Other repositories
    /// are specified with either repository-url/image:tag or repository-url/image@digest . For
    /// images using tags (repository-url/image:tag), up to 255 characters total are allowed,
    /// including letters (uppercase and lowercase), numbers, hyphens, underscores, colons,
    /// periods, forward slashes, and number signs (#). For images using digests (repository-
    /// url/image@digest), the 255 character limit applies only to the repository URL and image
    /// name (everything before the @ sign). The only supported hash function is sha256, and the
    /// hash value after sha256: must be exactly 64 characters (only letters A-F, a-f, and
    /// numbers 0-9 are allowed). This parameter maps to Image in the docker container create
    /// command and the IMAGE parameter of docker run. When a new task starts, the Amazon ECS
    /// container agent pulls the latest version of the specified image and tag for the
    /// container to use. However, subsequent updates to a repository image aren't propagated to
    /// already running tasks. Images in Amazon ECR repositories can be specified by either
    /// using the full registry/repository:tag or registry/repository@digest. For example,
    /// 012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-name&gt;:latest or
    /// 012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-
    /// name&gt;@sha256:94afd1f2e64d908bc90dbca0035a5b567EXAMPLE. Images in official
    /// repositories on Docker Hub use a single name (for example, ubuntu or mongo). Images in
    /// other repositories on Docker Hub are qualified with an organization name (for example,
    /// amazon/amazon-ecs-agent). Images in other online repositories are qualified further by a
    /// domain name (for example, quay.io/assemblyline/ubuntu).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// The number of cpu units reserved for the container. This parameter maps to CpuShares in
    /// the docker container create command and the --cpu-shares option to docker run. This
    /// field is optional for tasks using the Fargate launch type, and the only requirement is
    /// that the total amount of CPU reserved for all containers within a task be lower than the
    /// task-level cpu value. You can determine the number of CPU units that are available per
    /// EC2 instance type by multiplying the vCPUs listed for that instance type on the Amazon
    /// EC2 Instances detail page by 1,024. Linux containers share unallocated CPU units with
    /// other containers on the container instance with the same ratio as their allocated
    /// amount. For example, if you run a single-container task on a single-core instance type
    /// with 512 CPU units specified for that container, and that's the only task running on the
    /// container instance, that container could use the full 1,024 CPU unit share at any given
    /// time. However, if you launched another copy of the same task on that container instance,
    /// each task is guaranteed a minimum of 512 CPU units when needed. Moreover, each container
    /// could float to higher CPU usage if the other container was not using it. If both tasks
    /// were 100% active all of the time, they would be limited to 512 CPU units. On Linux
    /// container instances, the Docker daemon on the container instance uses the CPU value to
    /// calculate the relative CPU share ratios for running containers. The minimum valid CPU
    /// share value that the Linux kernel allows is 2, and the maximum valid CPU share value
    /// that the Linux kernel allows is 262144. However, the CPU parameter isn't required, and
    /// you can use CPU values below 2 or above 262144 in your container definitions. For CPU
    /// values below 2 (including null) or above 262144, the behavior varies based on your
    /// Amazon ECS container agent version: Agent versions less than or equal to 1.1.0: Null and
    /// zero CPU values are passed to Docker as 0, which Docker then converts to 1,024 CPU
    /// shares. CPU values of 1 are passed to Docker as 1, which the Linux kernel converts to
    /// two CPU shares. Agent versions greater than or equal to 1.2.0: Null, zero, and CPU
    /// values of 1 are passed to Docker as 2. Agent versions greater than or equal to 1.84.0:
    /// CPU values greater than 256 vCPU are passed to Docker as 256, which is equivalent to
    /// 262144 CPU shares. On Windows container instances, the CPU limit is enforced as an
    /// absolute limit, or a quota. Windows containers only have access to the specified amount
    /// of CPU that's described in the task definition. A null or zero CPU value is passed to
    /// Docker as 0, which Windows interprets as 1% of one CPU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,

    /// The amount (in MiB) of memory to present to the container. If your container attempts to
    /// exceed the memory specified here, the container is killed. The total amount of memory
    /// reserved for all containers within a task must be lower than the task memory value, if
    /// one is specified. This parameter maps to Memory in the docker container create command
    /// and the --memory option to docker run. If using the Fargate launch type, this parameter
    /// is optional. If using the EC2 launch type, you must specify either a task-level memory
    /// value or a container-level memory value. If you specify both a container-level memory
    /// and memoryReservation value, memory must be greater than memoryReservation. If you
    /// specify memoryReservation, then that value is subtracted from the available memory
    /// resources for the container instance where the container is placed. Otherwise, the value
    /// of memory is used. The Docker 20.10.0 or later daemon reserves a minimum of 6 MiB of
    /// memory for a container. So, don't specify less than 6 MiB of memory for your containers.
    /// The Docker 19.03.13-ce or earlier daemon reserves a minimum of 4 MiB of memory for a
    /// container. So, don't specify less than 4 MiB of memory for your containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,

    /// The soft limit (in MiB) of memory to reserve for the container. When system memory is
    /// under heavy contention, Docker attempts to keep the container memory to this soft limit.
    /// However, your container can consume more memory when it needs to, up to either the hard
    /// limit specified with the memory parameter (if applicable), or all of the available
    /// memory on the container instance, whichever comes first. This parameter maps to
    /// MemoryReservation in the docker container create command and the --memory-reservation
    /// option to docker run. If a task-level memory value is not specified, you must specify a
    /// non-zero integer for one or both of memory or memoryReservation in a container
    /// definition. If you specify both, memory must be greater than memoryReservation. If you
    /// specify memoryReservation, then that value is subtracted from the available memory
    /// resources for the container instance where the container is placed. Otherwise, the value
    /// of memory is used. For example, if your container normally uses 128 MiB of memory, but
    /// occasionally bursts to 256 MiB of memory for short periods of time, you can set a
    /// memoryReservation of 128 MiB, and a memory hard limit of 300 MiB. This configuration
    /// would allow the container to only reserve 128 MiB of memory from the remaining resources
    /// on the container instance, but also allow the container to consume more memory resources
    /// when needed. The Docker 20.10.0 or later daemon reserves a minimum of 6 MiB of memory
    /// for a container. So, don't specify less than 6 MiB of memory for your containers. The
    /// Docker 19.03.13-ce or earlier daemon reserves a minimum of 4 MiB of memory for a
    /// container. So, don't specify less than 4 MiB of memory for your containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i32>,

    /// The list of port mappings for the container. Port mappings allow containers to access
    /// ports on the host container instance to send or receive traffic. For task definitions
    /// that use the awsvpc network mode, only specify the containerPort. The hostPort can be
    /// left blank or it must be the same value as the containerPort. Port mappings on Windows
    /// use the NetNAT gateway address rather than localhost. There's no loopback for port
    /// mappings on Windows, so you can't access a container's mapped port from the host itself.
    /// This parameter maps to PortBindings in the docker container create command and the
    /// --publish option to docker run. If the network mode of a task definition is set to none,
    /// then you can't specify port mappings. If the network mode of a task definition is set to
    /// host, then host ports must either be undefined or they must match the container port in
    /// the port mapping. After a task reaches the RUNNING status, manual and automatic host and
    /// container port assignments are visible in the Network Bindings section of a container
    /// description for a selected task in the Amazon ECS console. The assignments are also
    /// visible in the networkBindings section DescribeTasks responses.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub port_mappings: Vec<PortMapping>,

    /// If the essential parameter of a container is marked as true, and that container fails or
    /// stops for any reason, all other containers that are part of the task are stopped. If the
    /// essential parameter of a container is marked as false, its failure doesn't affect the
    /// rest of the containers in a task. If this parameter is omitted, a container is assumed
    /// to be essential. All tasks must have at least one essential container. If you have an
    /// application that's composed of multiple containers, group containers that are used for a
    /// common purpose into components, and separate the different components into multiple task
    /// definitions. For more information, see Application Architecture in the Amazon Elastic
    /// Container Service Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,

    /// The environment variables to pass to a container. This parameter maps to Env in the
    /// docker container create command and the --env option to docker run. We don't recommend
    /// that you use plaintext environment variables for sensitive information, such as
    /// credential data.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub environment: Vec<KeyValuePair>,

    /// The command that's passed to the container. This parameter maps to Cmd in the docker
    /// container create command and the COMMAND parameter to docker run. If there are multiple
    /// arguments, each argument is a separated string in the array.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,

    /// Early versions of the Amazon ECS container agent don't properly handle entryPoint
    /// parameters. If you have problems using entryPoint, update your container agent or enter
    /// your commands and arguments as command array items instead. The entry point that's
    /// passed to the container. This parameter maps to Entrypoint in the docker container
    /// create command and the --entrypoint option to docker run.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entry_point: Vec<String>,

    /// The log configuration specification for the container. This parameter maps to LogConfig
    /// in the docker container create command and the --log-driver option to docker run. By
    /// default, containers use the same logging driver that the Docker daemon uses. However the
    /// container can use a different logging driver than the Docker daemon by specifying a log
    /// driver with this parameter in the container definition. To use a different logging
    /// driver for a container, the log system must be configured properly on the container
    /// instance (or on a different log server for remote logging options). Amazon ECS currently
    /// supports a subset of the logging drivers available to the Docker daemon (shown in the
    /// LogConfiguration data type). Additional log drivers may be available in future releases
    /// of the Amazon ECS container agent. This parameter requires version 1.18 of the Docker
    /// Remote API or greater on your container instance. To check the Docker Remote API version
    /// on your container instance, log in to your container instance and run the following
    /// command: sudo docker version --format '{{.Server.APIVersion}}' The Amazon ECS container
    /// agent running on a container instance must register the logging drivers available on
    /// that instance with the ECS_AVAILABLE_LOGGING_DRIVERS environment variable before
    /// containers placed on that instance can use these log configuration options. For more
    /// information, see Amazon ECS Container Agent Configuration in the Amazon Elastic
    /// Container Service Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
}

impl ContainerDefinition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-container_definition".into()),
            image: Some("test-image".into()),
            cpu: Some(100),
            memory: Some(100),
            memory_reservation: Some(100),
            port_mappings: vec![],
            essential: Some(false),
            environment: vec![],
            command: vec![],
            entry_point: vec![],
            log_configuration: Some(LogConfiguration::fixture()),
        }
    }
}

/// Port mappings allow containers to access ports on the host container instance to send or
/// receive traffic. Port mappings are specified as part of the container definition. If you use
/// containers in a task with the awsvpc or host network mode, specify the exposed ports using
/// containerPort. The hostPort can be left blank or it must be the same value as the
/// containerPort. Most fields of this parameter (containerPort, hostPort, protocol) maps to
/// PortBindings in the docker container create command and the --publish option to docker run.
/// If the network mode of a task definition is set to host, host ports must either be undefined
/// or match the container port in the port mapping. You can't expose the same container port
/// for multiple protocols. If you attempt this, an error is returned. After a task reaches the
/// RUNNING status, manual and automatic host and container port assignments are visible in the
/// networkBindings section of DescribeTasks API responses.
///
/// **AWS API**: `ecs.v1.PortMapping`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//PortMapping>
///
/// ## Coverage
/// 3 of 6 fields included.
/// Omitted fields:
/// - `name` — not selected in manifest
/// - `appProtocol` — not selected in manifest
/// - `containerPortRange` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortMapping {
    /// The port number on the container that's bound to the user-specified or automatically
    /// assigned host port. If you use containers in a task with the awsvpc or host network
    /// mode, specify the exposed ports using containerPort. If you use containers in a task
    /// with the bridge network mode and you specify a container port and not a host port, your
    /// container automatically receives a host port in the ephemeral port range. For more
    /// information, see hostPort. Port mappings that are automatically assigned in this way do
    /// not count toward the 100 reserved ports limit of a container instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,

    /// The port number on the container instance to reserve for your container. If you specify
    /// a containerPortRange, leave this field empty and the value of the hostPort is set as
    /// follows: For containers in a task with the awsvpc network mode, the hostPort is set to
    /// the same value as the containerPort. This is a static mapping strategy. For containers
    /// in a task with the bridge network mode, the Amazon ECS agent finds open ports on the
    /// host and automatically binds them to the container ports. This is a dynamic mapping
    /// strategy. If you use containers in a task with the awsvpc or host network mode, the
    /// hostPort can either be left blank or set to the same value as the containerPort. If you
    /// use containers in a task with the bridge network mode, you can specify a non-reserved
    /// host port for your container port mapping, or you can omit the hostPort (or set it to 0)
    /// while specifying a containerPort and your container automatically receives a port in the
    /// ephemeral port range for your container instance operating system and Docker version.
    /// The default ephemeral port range for Docker version 1.6.0 and later is listed on the
    /// instance under /proc/sys/net/ipv4/ip_local_port_range. If this kernel parameter is
    /// unavailable, the default ephemeral port range from 49153 through 65535 (Linux) or 49152
    /// through 65535 (Windows) is used. Do not attempt to specify a host port in the ephemeral
    /// port range as these are reserved for automatic assignment. In general, ports below 32768
    /// are outside of the ephemeral port range. The default reserved ports are 22 for SSH, the
    /// Docker ports 2375 and 2376, and the Amazon ECS container agent ports 51678-51680. Any
    /// host port that was previously specified in a running task is also reserved while the
    /// task is running. That is, after a task stops, the host port is released. The current
    /// reserved ports are displayed in the remainingResources of DescribeContainerInstances
    /// output. A container instance can have up to 100 reserved ports at a time. This number
    /// includes the default reserved ports. Automatically assigned ports aren't included in the
    /// 100 reserved ports quota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,

    /// The protocol used for the port mapping. Valid values are tcp and udp. The default is
    /// tcp. protocol is immutable in a Service Connect service. Updating this field requires a
    /// service deletion and redeployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

impl PortMapping {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            container_port: Some(100),
            host_port: Some(100),
            protocol: Some("test-protocol".into()),
        }
    }
}

/// The log configuration for the container. This parameter maps to LogConfig in the docker
/// container create command and the --log-driver option to docker run. By default, containers
/// use the same logging driver that the Docker daemon uses. However, the container might use a
/// different logging driver than the Docker daemon by specifying a log driver configuration in
/// the container definition. Understand the following when specifying a log configuration for
/// your containers. Amazon ECS currently supports a subset of the logging drivers available to
/// the Docker daemon. Additional log drivers may be available in future releases of the Amazon
/// ECS container agent. For tasks on Fargate, the supported log drivers are awslogs, splunk,
/// and awsfirelens. For tasks hosted on Amazon EC2 instances, the supported log drivers are
/// awslogs, fluentd, gelf, json-file, journald,syslog, splunk, and awsfirelens. This parameter
/// requires version 1.18 of the Docker Remote API or greater on your container instance. For
/// tasks that are hosted on Amazon EC2 instances, the Amazon ECS container agent must register
/// the available logging drivers with the ECS_AVAILABLE_LOGGING_DRIVERS environment variable
/// before containers placed on that instance can use these log configuration options. For more
/// information, see Amazon ECS container agent configuration in the Amazon Elastic Container
/// Service Developer Guide. For tasks that are on Fargate, because you don't have access to the
/// underlying infrastructure your tasks are hosted on, any additional software needed must be
/// installed outside of the task. For example, the Fluentd output aggregators or a remote host
/// running Logstash to send Gelf logs to.
///
/// **AWS API**: `ecs.v1.LogConfiguration`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//LogConfiguration>
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `secretOptions` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogConfiguration {
    /// The log driver to use for the container. For tasks on Fargate, the supported log drivers
    /// are awslogs, splunk, and awsfirelens. For tasks hosted on Amazon EC2 instances, the
    /// supported log drivers are awslogs, fluentd, gelf, json-file, journald, syslog, splunk,
    /// and awsfirelens. For more information about using the awslogs log driver, see Send
    /// Amazon ECS logs to CloudWatch in the Amazon Elastic Container Service Developer Guide.
    /// For more information about using the awsfirelens log driver, see Send Amazon ECS logs to
    /// an Amazon Web Services service or Amazon Web Services Partner. If you have a custom
    /// driver that isn't listed, you can fork the Amazon ECS container agent project that's
    /// available on GitHub and customize it to work with that driver. We encourage you to
    /// submit pull requests for changes that you would like to have included. However, we don't
    /// currently provide support for running modified copies of this software.
    pub log_driver: String,

    /// The configuration options to send to the log driver. The options you can specify depend
    /// on the log driver. Some of the options you can specify when you use the awslogs log
    /// driver to route logs to Amazon CloudWatch include the following: awslogs-create-group
    /// Required: No Specify whether you want the log group to be created automatically. If this
    /// option isn't specified, it defaults to false. Your IAM policy must include the
    /// logs:CreateLogGroup permission before you attempt to use awslogs-create-group. awslogs-
    /// region Required: Yes Specify the Amazon Web Services Region that the awslogs log driver
    /// is to send your Docker logs to. You can choose to send all of your logs from clusters in
    /// different Regions to a single region in CloudWatch Logs. This is so that they're all
    /// visible in one location. Otherwise, you can separate them by Region for more
    /// granularity. Make sure that the specified log group exists in the Region that you
    /// specify with this option. awslogs-group Required: Yes Make sure to specify a log group
    /// that the awslogs log driver sends its log streams to. awslogs-stream-prefix Required:
    /// Yes, when using Fargate.Optional when using EC2. Use the awslogs-stream-prefix option to
    /// associate a log stream with the specified prefix, the container name, and the ID of the
    /// Amazon ECS task that the container belongs to. If you specify a prefix with this option,
    /// then the log stream takes the format prefix-name/container-name/ecs-task-id. If you
    /// don't specify a prefix with this option, then the log stream is named after the
    /// container ID that's assigned by the Docker daemon on the container instance. Because
    /// it's difficult to trace logs back to the container that sent them with just the Docker
    /// container ID (which is only available on the container instance), we recommend that you
    /// specify a prefix with this option. For Amazon ECS services, you can use the service name
    /// as the prefix. Doing so, you can trace log streams to the service that the container
    /// belongs to, the name of the container that sent them, and the ID of the task that the
    /// container belongs to. You must specify a stream-prefix for your logs to have your logs
    /// appear in the Log pane when using the Amazon ECS console. awslogs-datetime-format
    /// Required: No This option defines a multiline start pattern in Python strftime format. A
    /// log message consists of a line that matches the pattern and any following lines that
    /// don’t match the pattern. The matched line is the delimiter between log messages. One
    /// example of a use case for using this format is for parsing output such as a stack dump,
    /// which might otherwise be logged in multiple entries. The correct pattern allows it to be
    /// captured in a single entry. For more information, see awslogs-datetime-format. You
    /// cannot configure both the awslogs-datetime-format and awslogs-multiline-pattern options.
    /// Multiline logging performs regular expression parsing and matching of all log messages.
    /// This might have a negative impact on logging performance. awslogs-multiline-pattern
    /// Required: No This option defines a multiline start pattern that uses a regular
    /// expression. A log message consists of a line that matches the pattern and any following
    /// lines that don’t match the pattern. The matched line is the delimiter between log
    /// messages. For more information, see awslogs-multiline-pattern. This option is ignored if
    /// awslogs-datetime-format is also configured. You cannot configure both the awslogs-
    /// datetime-format and awslogs-multiline-pattern options. Multiline logging performs
    /// regular expression parsing and matching of all log messages. This might have a negative
    /// impact on logging performance. The following options apply to all supported log drivers.
    /// mode Required: No Valid values: non-blocking | blocking This option defines the delivery
    /// mode of log messages from the container to the log driver specified using logDriver. The
    /// delivery mode you choose affects application availability when the flow of logs from
    /// container is interrupted. If you use the blocking mode and the flow of logs is
    /// interrupted, calls from container code to write to the stdout and stderr streams will
    /// block. The logging thread of the application will block as a result. This may cause the
    /// application to become unresponsive and lead to container healthcheck failure. If you use
    /// the non-blocking mode, the container's logs are instead stored in an in-memory
    /// intermediate buffer configured with the max-buffer-size option. This prevents the
    /// application from becoming unresponsive when logs cannot be sent. We recommend using this
    /// mode if you want to ensure service availability and are okay with some log loss. For
    /// more information, see Preventing log loss with non-blocking mode in the awslogs
    /// container log driver. You can set a default mode for all containers in a specific Amazon
    /// Web Services Region by using the defaultLogDriverMode account setting. If you don't
    /// specify the mode option or configure the account setting, Amazon ECS will default to the
    /// non-blocking mode. For more information about the account setting, see Default log
    /// driver mode in the Amazon Elastic Container Service Developer Guide. On June 25, 2025,
    /// Amazon ECS changed the default log driver mode from blocking to non-blocking to
    /// prioritize task availability over logging. To continue using the blocking mode after
    /// this change, do one of the following: Set the mode option in your container definition's
    /// logConfiguration as blocking. Set the defaultLogDriverMode account setting to blocking.
    /// max-buffer-size Required: No Default value: 10m When non-blocking mode is used, the max-
    /// buffer-size log option controls the size of the buffer that's used for intermediate
    /// message storage. Make sure to specify an adequate buffer size based on your application.
    /// When the buffer fills up, further logs cannot be stored. Logs that cannot be stored are
    /// lost. To route logs using the splunk log router, you need to specify a splunk-token and
    /// a splunk-url. When you use the awsfirelens log router to route logs to an Amazon Web
    /// Services Service or Amazon Web Services Partner Network destination for log storage and
    /// analytics, you can set the log-driver-buffer-limit option to limit the number of events
    /// that are buffered in memory, before being sent to the log router container. It can help
    /// to resolve potential log loss issue because high throughput might result in memory
    /// running out for the buffer inside of Docker. Other options you can specify when using
    /// awsfirelens to route logs depend on the destination. When you export logs to Amazon Data
    /// Firehose, you can specify the Amazon Web Services Region with region and a name for the
    /// log stream with delivery_stream. When you export logs to Amazon Kinesis Data Streams,
    /// you can specify an Amazon Web Services Region with region and a data stream name with
    /// stream. When you export logs to Amazon OpenSearch Service, you can specify options like
    /// Name, Host (OpenSearch Service endpoint without protocol), Port, Index, Type, Aws_auth,
    /// Aws_region, Suppress_Type_Name, and tls. For more information, see Under the hood:
    /// FireLens for Amazon ECS Tasks. When you export logs to Amazon S3, you can specify the
    /// bucket using the bucket option. You can also specify region, total_file_size,
    /// upload_timeout, and use_put_object as options. This parameter requires version 1.19 of
    /// the Docker Remote API or greater on your container instance. To check the Docker Remote
    /// API version on your container instance, log in to your container instance and run the
    /// following command: sudo docker version --format '{{.Server.APIVersion}}'
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub options: HashMap<String, String>,
}

impl LogConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            log_driver: "test-log_driver".into(),
            options: Default::default(),
        }
    }
}

///
/// **AWS API**: `ecs.v1.UpdateServiceRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//UpdateServiceRequest>
///
/// ## Coverage
/// 7 of 22 fields included.
/// Omitted fields:
/// - `capacityProviderStrategy` — not selected in manifest
/// - `deploymentConfiguration` — not selected in manifest
/// - `availabilityZoneRebalancing` — not selected in manifest
/// - `placementConstraints` — not selected in manifest
/// - `placementStrategy` — not selected in manifest
/// - `platformVersion` — not selected in manifest
/// - `deploymentController` — not selected in manifest
/// - `enableExecuteCommand` — not selected in manifest
/// - `enableECSManagedTags` — not selected in manifest
/// - `loadBalancers` — not selected in manifest
/// - `propagateTags` — not selected in manifest
/// - `serviceRegistries` — not selected in manifest
/// - `serviceConnectConfiguration` — not selected in manifest
/// - `volumeConfigurations` — not selected in manifest
/// - `vpcLatticeConfigurations` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceRequest {
    /// The short name or full Amazon Resource Name (ARN) of the cluster that your service runs
    /// on. If you do not specify a cluster, the default cluster is assumed. You can't change
    /// the cluster name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,

    /// The name of the service to update.
    pub service: String,

    /// The number of instantiations of the task to place and keep running in your service. This
    /// parameter doesn't trigger a new service deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i32>,

    /// The family and revision (family:revision) or full ARN of the task definition to run in
    /// your service. If a revision is not specified, the latest ACTIVE revision is used. If you
    /// modify the task definition with UpdateService, Amazon ECS spawns a task with the new
    /// version of the task definition and then stops an old task after the new version is
    /// running. This parameter triggers a new service deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,

    /// An object representing the network configuration for the service. This parameter
    /// triggers a new service deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,

    /// Determines whether to force a new deployment of the service. By default, deployments
    /// aren't forced. You can use this option to start a new deployment with no service
    /// definition changes. For example, you can update a service's tasks to use a newer Docker
    /// image with the same image/tag combination (my_image:latest) or to roll Fargate tasks
    /// onto a newer platform version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_new_deployment: Option<bool>,

    /// The period of time, in seconds, that the Amazon ECS service scheduler ignores unhealthy
    /// Elastic Load Balancing, VPC Lattice, and container health checks after a task has first
    /// started. If you don't specify a health check grace period value, the default value of 0
    /// is used. If you don't use any of the health checks, then healthCheckGracePeriodSeconds
    /// is unused. If your service's tasks take a while to start and respond to health checks,
    /// you can specify a health check grace period of up to 2,147,483,647 seconds (about 69
    /// years). During that time, the Amazon ECS service scheduler ignores health check status.
    /// This grace period can prevent the service scheduler from marking tasks as unhealthy and
    /// stopping them before they have time to come up. If your service has more running tasks
    /// than desired, unhealthy tasks in the grace period might be stopped to reach the desired
    /// count. This parameter doesn't trigger a new service deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i32>,
}

impl UpdateServiceRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cluster: Some("test-cluster".into()),
            service: "test-service".into(),
            desired_count: Some(100),
            task_definition: Some("test-task_definition".into()),
            network_configuration: Some(NetworkConfiguration::fixture()),
            force_new_deployment: Some(false),
            health_check_grace_period_seconds: Some(100),
        }
    }
}

///
/// **AWS API**: `ecs.v1.UpdateServiceResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//UpdateServiceResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceResponse {
    /// The full description of your service following the update call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

impl UpdateServiceResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            service: Some(Service::fixture()),
        }
    }
}

///
/// **AWS API**: `ecs.v1.DeregisterTaskDefinitionRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//DeregisterTaskDefinitionRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeregisterTaskDefinitionRequest {
    /// The family and revision (family:revision) or full Amazon Resource Name (ARN) of the task
    /// definition to deregister. You must specify a revision.
    pub task_definition: String,
}

impl DeregisterTaskDefinitionRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            task_definition: "test-task_definition".into(),
        }
    }
}

///
/// **AWS API**: `ecs.v1.DeregisterTaskDefinitionResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonECS/latest/APIReference//DeregisterTaskDefinitionResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeregisterTaskDefinitionResponse {
    /// The full description of the deregistered task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

impl DeregisterTaskDefinitionResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            task_definition: Some(TaskDefinition::fixture()),
        }
    }
}
