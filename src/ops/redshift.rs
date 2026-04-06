//! Operation contracts for the Amazon Redshift API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/redshift.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::redshift::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon Redshift API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::redshift::RedshiftClient`] instead.
pub struct RedshiftOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> RedshiftOps<'a> {
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
        "https://redshift.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Returns properties of provisioned clusters including general cluster properties, cluster
    /// database properties, maintenance and backup properties, and security and access
    /// properties. This operation supports pagination. For more information about managing
    /// clusters, go to Amazon Redshift Clusters in the Amazon Redshift Cluster Management
    /// Guide. If you specify both tag keys and tag values in the same request, Amazon Redshift
    /// returns all clusters that match any combination of the specified keys and values. For
    /// example, if you have owner and environment for tag keys, and admin and test for tag
    /// values, all clusters that have any combination of those values are returned. If both tag
    /// keys and values are omitted from the request, clusters are returned regardless of
    /// whether they have tag keys or values associated with them.
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
        let body_str = crate::query::build_query_body("DescribeClusters", "2012-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "redshift",
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
                    message: format!("Failed to read describe_clusters response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_clusters response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeClustersResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_clusters XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Pauses a cluster.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`PauseClusterRequest`]
    ///
    /// # Response
    /// [`PauseClusterResponse`]
    #[allow(dead_code)]
    pub(crate) async fn pause_cluster(
        &self,
        body: &PauseClusterRequest,
    ) -> Result<PauseClusterResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("PauseCluster", "2012-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "redshift",
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
                    message: format!("Failed to read pause_cluster response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in pause_cluster response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<PauseClusterResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse pause_cluster XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Resumes a paused cluster.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ResumeClusterRequest`]
    ///
    /// # Response
    /// [`ResumeClusterResponse`]
    #[allow(dead_code)]
    pub(crate) async fn resume_cluster(
        &self,
        body: &ResumeClusterRequest,
    ) -> Result<ResumeClusterResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("ResumeCluster", "2012-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "redshift",
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
                    message: format!("Failed to read resume_cluster response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in resume_cluster response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ResumeClusterResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse resume_cluster XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Changes the size of the cluster. You can change the cluster's type, or change the number
    /// or type of nodes. The default behavior is to use the elastic resize method. With an
    /// elastic resize, your cluster is available for read and write operations more quickly
    /// than with the classic resize method. Elastic resize operations have the following
    /// restrictions: You can only resize clusters of the following types: dc2.large dc2.8xlarge
    /// ra3.large ra3.xlplus ra3.4xlarge ra3.16xlarge The type of nodes that you add must match
    /// the node type for the cluster.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ResizeClusterRequest`]
    ///
    /// # Response
    /// [`ResizeClusterResponse`]
    #[allow(dead_code)]
    pub(crate) async fn resize_cluster(
        &self,
        body: &ResizeClusterRequest,
    ) -> Result<ResizeClusterResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("ResizeCluster", "2012-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "redshift",
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
                    message: format!("Failed to read resize_cluster response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in resize_cluster response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ResizeClusterResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse resize_cluster XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Deletes a previously provisioned cluster without its final snapshot being created. A
    /// successful response from the web service indicates that the request was received
    /// correctly. Use DescribeClusters to monitor the status of the deletion. The delete
    /// operation cannot be canceled or reverted once submitted. For more information about
    /// managing clusters, go to Amazon Redshift Clusters in the Amazon Redshift Cluster
    /// Management Guide. If you want to shut down the cluster and retain it for future use, set
    /// SkipFinalClusterSnapshot to false and specify a name for FinalClusterSnapshotIdentifier.
    /// You can later restore this snapshot to resume using the cluster. If a final cluster
    /// snapshot is requested, the status of the cluster will be "final-snapshot" while the
    /// snapshot is being taken, then it's "deleting" once Amazon Redshift begins deleting the
    /// cluster. For more information about managing clusters, go to Amazon Redshift Clusters in
    /// the Amazon Redshift Cluster Management Guide.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteClusterRequest`]
    ///
    /// # Response
    /// [`DeleteClusterResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_cluster(
        &self,
        body: &DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("DeleteCluster", "2012-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "redshift",
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
                    message: format!("Failed to read delete_cluster response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in delete_cluster response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeleteClusterResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse delete_cluster XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_clusters() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeClustersResponse::fixture();
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
        let ops = RedshiftOps::new(&client);

        let body = DescribeClustersRequest::fixture();
        let result = ops.describe_clusters(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pause_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = PauseClusterResponse::fixture();
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
        let ops = RedshiftOps::new(&client);

        let body = PauseClusterRequest::fixture();
        let result = ops.pause_cluster(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_resume_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ResumeClusterResponse::fixture();
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
        let ops = RedshiftOps::new(&client);

        let body = ResumeClusterRequest::fixture();
        let result = ops.resume_cluster(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_resize_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ResizeClusterResponse::fixture();
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
        let ops = RedshiftOps::new(&client);

        let body = ResizeClusterRequest::fixture();
        let result = ops.resize_cluster(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeleteClusterResponse::fixture();
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
        let ops = RedshiftOps::new(&client);

        let body = DeleteClusterRequest::fixture();
        let result = ops.delete_cluster(&body).await;
        assert!(result.is_ok());
    }
}
