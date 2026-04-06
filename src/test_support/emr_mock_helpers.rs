//! MockClient helpers for Amazon EMR API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon EMR helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait EmrMockHelpers {
    /// Helper to expect `list_clusters`: Provides the status of all clusters visible to this Amazon
    /// Web Services account.
    fn expect_list_clusters(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_cluster`: Provides cluster-level details including status,
    /// hardware and software configuration, VPC settings, and so on.
    fn expect_describe_cluster(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `terminate_job_flows`: TerminateJobFlows shuts a list of clusters (job
    /// flows) down. When a job flow is shut down, any step not yet completed is canceled and the
    /// EC2 instances on which the cluster is running are stopped.
    fn expect_terminate_job_flows(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl EmrMockHelpers for MockClient {
    /// Helper to expect `list_clusters`: Provides the status of all clusters visible to this Amazon
    /// Web Services account.
    fn expect_list_clusters(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_cluster`: Provides cluster-level details including status,
    /// hardware and software configuration, VPC settings, and so on.
    fn expect_describe_cluster(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `terminate_job_flows`: TerminateJobFlows shuts a list of clusters (job
    /// flows) down. When a job flow is shut down, any step not yet completed is canceled and the
    /// EC2 instances on which the cluster is running are stopped.
    fn expect_terminate_job_flows(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
