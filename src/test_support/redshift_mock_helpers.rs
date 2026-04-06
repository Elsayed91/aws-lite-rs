//! MockClient helpers for Amazon Redshift API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon Redshift helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait RedshiftMockHelpers {
    /// Helper to expect `describe_clusters`: Returns properties of provisioned clusters including
    /// general cluster properties, cluster database properties, maintenance and backup properties,
    /// and security and access properties. This operation supports pagination. For more information
    /// about managing clusters, go to Amazon Redshift Clusters in the Amazon Redshift Cluster
    /// Management Guide. If you specify both tag keys and tag values in the same request, Amazon
    /// Redshift returns all clusters that match any combination of the specified keys and values.
    /// For example, if you have owner and environment for tag keys, and admin and test for tag
    /// values, all clusters that have any combination of those values are returned. If both tag
    /// keys and values are omitted from the request, clusters are returned regardless of whether
    /// they have tag keys or values associated with them.
    fn expect_describe_clusters(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `pause_cluster`: Pauses a cluster.
    fn expect_pause_cluster(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `resume_cluster`: Resumes a paused cluster.
    fn expect_resume_cluster(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `resize_cluster`: Changes the size of the cluster. You can change the
    /// cluster's type, or change the number or type of nodes. The default behavior is to use the
    /// elastic resize method. With an elastic resize, your cluster is available for read and write
    /// operations more quickly than with the classic resize method. Elastic resize operations have
    /// the following restrictions: You can only resize clusters of the following types: dc2.large
    /// dc2.8xlarge ra3.large ra3.xlplus ra3.4xlarge ra3.16xlarge The type of nodes that you add
    /// must match the node type for the cluster.
    fn expect_resize_cluster(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_cluster`: Deletes a previously provisioned cluster without its
    /// final snapshot being created. A successful response from the web service indicates that the
    /// request was received correctly. Use DescribeClusters to monitor the status of the deletion.
    /// The delete operation cannot be canceled or reverted once submitted. For more information
    /// about managing clusters, go to Amazon Redshift Clusters in the Amazon Redshift Cluster
    /// Management Guide. If you want to shut down the cluster and retain it for future use, set
    /// SkipFinalClusterSnapshot to false and specify a name for FinalClusterSnapshotIdentifier. You
    /// can later restore this snapshot to resume using the cluster. If a final cluster snapshot is
    /// requested, the status of the cluster will be "final-snapshot" while the snapshot is being
    /// taken, then it's "deleting" once Amazon Redshift begins deleting the cluster. For more
    /// information about managing clusters, go to Amazon Redshift Clusters in the Amazon Redshift
    /// Cluster Management Guide.
    fn expect_delete_cluster(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl RedshiftMockHelpers for MockClient {
    /// Helper to expect `describe_clusters`: Returns properties of provisioned clusters including
    /// general cluster properties, cluster database properties, maintenance and backup properties,
    /// and security and access properties. This operation supports pagination. For more information
    /// about managing clusters, go to Amazon Redshift Clusters in the Amazon Redshift Cluster
    /// Management Guide. If you specify both tag keys and tag values in the same request, Amazon
    /// Redshift returns all clusters that match any combination of the specified keys and values.
    /// For example, if you have owner and environment for tag keys, and admin and test for tag
    /// values, all clusters that have any combination of those values are returned. If both tag
    /// keys and values are omitted from the request, clusters are returned regardless of whether
    /// they have tag keys or values associated with them.
    fn expect_describe_clusters(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `pause_cluster`: Pauses a cluster.
    fn expect_pause_cluster(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `resume_cluster`: Resumes a paused cluster.
    fn expect_resume_cluster(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `resize_cluster`: Changes the size of the cluster. You can change the
    /// cluster's type, or change the number or type of nodes. The default behavior is to use the
    /// elastic resize method. With an elastic resize, your cluster is available for read and write
    /// operations more quickly than with the classic resize method. Elastic resize operations have
    /// the following restrictions: You can only resize clusters of the following types: dc2.large
    /// dc2.8xlarge ra3.large ra3.xlplus ra3.4xlarge ra3.16xlarge The type of nodes that you add
    /// must match the node type for the cluster.
    fn expect_resize_cluster(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_cluster`: Deletes a previously provisioned cluster without its
    /// final snapshot being created. A successful response from the web service indicates that the
    /// request was received correctly. Use DescribeClusters to monitor the status of the deletion.
    /// The delete operation cannot be canceled or reverted once submitted. For more information
    /// about managing clusters, go to Amazon Redshift Clusters in the Amazon Redshift Cluster
    /// Management Guide. If you want to shut down the cluster and retain it for future use, set
    /// SkipFinalClusterSnapshot to false and specify a name for FinalClusterSnapshotIdentifier. You
    /// can later restore this snapshot to resume using the cluster. If a final cluster snapshot is
    /// requested, the status of the cluster will be "final-snapshot" while the snapshot is being
    /// taken, then it's "deleting" once Amazon Redshift begins deleting the cluster. For more
    /// information about managing clusters, go to Amazon Redshift Clusters in the Amazon Redshift
    /// Cluster Management Guide.
    fn expect_delete_cluster(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
