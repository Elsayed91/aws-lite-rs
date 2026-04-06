//! Amazon EMR API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::emr::EmrOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::emr::EmrOps,
    types::emr::{
        DescribeClusterInput, DescribeClusterOutput, ListClustersInput, ListClustersOutput,
        TerminateJobFlowsInput,
    },
};

/// Client for the Amazon EMR API
pub struct EmrClient<'a> {
    ops: EmrOps<'a>,
}

impl<'a> EmrClient<'a> {
    /// Create a new Amazon EMR API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: EmrOps::new(client),
        }
    }

    /// Provides the status of all clusters visible to this Amazon Web Services account.
    pub async fn list_clusters(&self, body: &ListClustersInput) -> Result<ListClustersOutput> {
        self.ops.list_clusters(body).await
    }

    /// Provides cluster-level details including status, hardware and software configuration, VPC settings, and so on.
    pub async fn describe_cluster(
        &self,
        body: &DescribeClusterInput,
    ) -> Result<DescribeClusterOutput> {
        self.ops.describe_cluster(body).await
    }

    /// TerminateJobFlows shuts a list of clusters (job flows) down. When a job flow is shut down, any step not yet completed is
    pub async fn terminate_job_flows(&self, body: &TerminateJobFlowsInput) -> Result<()> {
        self.ops.terminate_job_flows(body).await
    }
}
