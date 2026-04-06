//! Amazon Elastic Kubernetes Service API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::eks::EksOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::eks::EksOps,
    types::eks::{
        DescribeClusterResponse, DescribeNodegroupResponse, ListNodegroupsResponse,
        UpdateNodegroupConfigRequest, UpdateNodegroupConfigResponse,
    },
};

/// Client for the Amazon Elastic Kubernetes Service API
pub struct EksClient<'a> {
    ops: EksOps<'a>,
}

impl<'a> EksClient<'a> {
    /// Create a new Amazon Elastic Kubernetes Service API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: EksOps::new(client),
        }
    }

    /// Returns descriptive information about an Amazon EKS cluster.
    pub async fn describe_cluster(&self, name: &str) -> Result<DescribeClusterResponse> {
        self.ops.describe_cluster(name).await
    }

    /// Lists the managed node groups associated with the specified cluster.
    pub async fn list_nodegroups(
        &self,
        name: &str,
        max_results: &str,
        next_token: &str,
    ) -> Result<ListNodegroupsResponse> {
        self.ops
            .list_nodegroups(name, max_results, next_token)
            .await
    }

    /// Returns descriptive information about an Amazon EKS node group.
    pub async fn describe_nodegroup(
        &self,
        name: &str,
        nodegroup_name: &str,
    ) -> Result<DescribeNodegroupResponse> {
        self.ops.describe_nodegroup(name, nodegroup_name).await
    }

    /// Updates an Amazon EKS managed node group configuration.
    pub async fn update_nodegroup_config(
        &self,
        name: &str,
        nodegroup_name: &str,
        body: &UpdateNodegroupConfigRequest,
    ) -> Result<UpdateNodegroupConfigResponse> {
        self.ops
            .update_nodegroup_config(name, nodegroup_name, body)
            .await
    }
}
