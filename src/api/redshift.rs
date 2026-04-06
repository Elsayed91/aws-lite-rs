//! Amazon Redshift API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::redshift::RedshiftOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::redshift::RedshiftOps,
    types::redshift::{
        DeleteClusterRequest, DeleteClusterResponse, DescribeClustersRequest,
        DescribeClustersResponse, PauseClusterRequest, PauseClusterResponse, ResizeClusterRequest,
        ResizeClusterResponse, ResumeClusterRequest, ResumeClusterResponse,
    },
};

/// Client for the Amazon Redshift API
pub struct RedshiftClient<'a> {
    ops: RedshiftOps<'a>,
}

impl<'a> RedshiftClient<'a> {
    /// Create a new Amazon Redshift API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: RedshiftOps::new(client),
        }
    }

    /// Returns properties of provisioned clusters including general cluster properties, cluster database properties, maintenanc
    pub async fn describe_clusters(
        &self,
        body: &DescribeClustersRequest,
    ) -> Result<DescribeClustersResponse> {
        self.ops.describe_clusters(body).await
    }

    /// Pauses a cluster.
    pub async fn pause_cluster(&self, body: &PauseClusterRequest) -> Result<PauseClusterResponse> {
        self.ops.pause_cluster(body).await
    }

    /// Resumes a paused cluster.
    pub async fn resume_cluster(
        &self,
        body: &ResumeClusterRequest,
    ) -> Result<ResumeClusterResponse> {
        self.ops.resume_cluster(body).await
    }

    /// Changes the size of the cluster. You can change the cluster's type, or change the number or type of nodes. The default b
    pub async fn resize_cluster(
        &self,
        body: &ResizeClusterRequest,
    ) -> Result<ResizeClusterResponse> {
        self.ops.resize_cluster(body).await
    }

    /// Deletes a previously provisioned cluster without its final snapshot being created. A successful response from the web se
    pub async fn delete_cluster(
        &self,
        body: &DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse> {
        self.ops.delete_cluster(body).await
    }
}
