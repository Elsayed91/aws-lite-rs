//! Operation contracts for the Amazon ElastiCache API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/elasticache.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::elasticache::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon ElastiCache API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::elasticache::ElasticacheClient`] instead.
pub struct ElasticacheOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> ElasticacheOps<'a> {
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
        "https://elasticache.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Returns information about all provisioned clusters if no cluster identifier is
    /// specified, or about a specific cache cluster if a cluster identifier is supplied. By
    /// default, abbreviated information about the clusters is returned. You can use the
    /// optional ShowCacheNodeInfo flag to retrieve detailed information about the cache nodes
    /// associated with the clusters. These details include the DNS address and port for the
    /// cache node endpoint. If the cluster is in the creating state, only cluster-level
    /// information is displayed until all of the nodes are successfully provisioned. If the
    /// cluster is in the deleting state, only cluster-level information is displayed. If cache
    /// nodes are currently being added to the cluster, node endpoint information and creation
    /// time for the additional nodes are not displayed until they are completely provisioned.
    /// When the cluster state is available, the cluster is ready for use. If cache nodes are
    /// currently being removed from the cluster, no endpoint information for the removed nodes
    /// is displayed.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeCacheClustersRequest`]
    ///
    /// # Response
    /// [`DescribeCacheClustersResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_cache_clusters(
        &self,
        body: &DescribeCacheClustersRequest,
    ) -> Result<DescribeCacheClustersResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DescribeCacheClusters", "2015-02-02", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticache",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_cache_clusters response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_cache_clusters response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeCacheClustersResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_cache_clusters XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Returns information about a particular replication group. If no identifier is specified,
    /// DescribeReplicationGroups returns information about all replication groups. This
    /// operation is valid for Valkey or Redis OSS only.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeReplicationGroupsRequest`]
    ///
    /// # Response
    /// [`DescribeReplicationGroupsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_replication_groups(
        &self,
        body: &DescribeReplicationGroupsRequest,
    ) -> Result<DescribeReplicationGroupsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DescribeReplicationGroups", "2015-02-02", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticache",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_replication_groups response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_replication_groups response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeReplicationGroupsResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_replication_groups XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Deletes a previously provisioned cluster. DeleteCacheCluster deletes all associated
    /// cache nodes, node endpoints and the cluster itself. When you receive a successful
    /// response from this operation, Amazon ElastiCache immediately begins deleting the
    /// cluster; you cannot cancel or revert this operation. This operation is not valid for:
    /// Valkey or Redis OSS (cluster mode enabled) clusters Valkey or Redis OSS (cluster mode
    /// disabled) clusters A cluster that is the last read replica of a replication group A
    /// cluster that is the primary node of a replication group A node group (shard) that has
    /// Multi-AZ mode enabled A cluster from a Valkey or Redis OSS (cluster mode enabled)
    /// replication group A cluster that is not in the available state
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteCacheClusterRequest`]
    ///
    /// # Response
    /// [`DeleteCacheClusterResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_cache_cluster(
        &self,
        body: &DeleteCacheClusterRequest,
    ) -> Result<DeleteCacheClusterResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DeleteCacheCluster", "2015-02-02", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticache",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read delete_cache_cluster response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in delete_cache_cluster response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeleteCacheClusterResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse delete_cache_cluster XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Deletes an existing replication group. By default, this operation deletes the entire
    /// replication group, including the primary/primaries and all of the read replicas. If the
    /// replication group has only one primary, you can optionally delete only the read
    /// replicas, while retaining the primary by setting RetainPrimaryCluster=true. When you
    /// receive a successful response from this operation, Amazon ElastiCache immediately begins
    /// deleting the selected resources; you cannot cancel or revert this operation.
    /// CreateSnapshot permission is required to create a final snapshot. Without this
    /// permission, the API call will fail with an Access Denied exception. This operation is
    /// valid for Redis OSS only.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteReplicationGroupRequest`]
    ///
    /// # Response
    /// [`DeleteReplicationGroupResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_replication_group(
        &self,
        body: &DeleteReplicationGroupRequest,
    ) -> Result<DeleteReplicationGroupResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DeleteReplicationGroup", "2015-02-02", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticache",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read delete_replication_group response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in delete_replication_group response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeleteReplicationGroupResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse delete_replication_group XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Creates a copy of an entire cluster or replication group at a specific moment in time.
    /// This operation is valid for Valkey or Redis OSS only.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`CreateSnapshotRequest`]
    ///
    /// # Response
    /// [`CreateSnapshotResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_snapshot(
        &self,
        body: &CreateSnapshotRequest,
    ) -> Result<CreateSnapshotResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("CreateSnapshot", "2015-02-02", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticache",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read create_snapshot response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in create_snapshot response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<CreateSnapshotResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse create_snapshot XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_cache_clusters() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeCacheClustersResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ElasticacheOps::new(&client);

        let body = DescribeCacheClustersRequest::fixture();
        let result = ops.describe_cache_clusters(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_replication_groups() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeReplicationGroupsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ElasticacheOps::new(&client);

        let body = DescribeReplicationGroupsRequest::fixture();
        let result = ops.describe_replication_groups(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_cache_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeleteCacheClusterResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ElasticacheOps::new(&client);

        let body = DeleteCacheClusterRequest::fixture();
        let result = ops.delete_cache_cluster(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_replication_group() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeleteReplicationGroupResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ElasticacheOps::new(&client);

        let body = DeleteReplicationGroupRequest::fixture();
        let result = ops.delete_replication_group(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_snapshot() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = CreateSnapshotResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ElasticacheOps::new(&client);

        let body = CreateSnapshotRequest::fixture();
        let result = ops.create_snapshot(&body).await;
        assert!(result.is_ok());
    }
}
