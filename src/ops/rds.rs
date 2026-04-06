//! Operation contracts for the Amazon Relational Database Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/rds.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::rds::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon Relational Database Service API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::rds::RdsClient`] instead.
pub struct RdsOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> RdsOps<'a> {
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
        "https://rds.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Describes provisioned RDS instances.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeDBInstancesRequest`]
    ///
    /// # Response
    /// [`DescribeDBInstancesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_db_instances(
        &self,
        body: &DescribeDBInstancesRequest,
    ) -> Result<DescribeDBInstancesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DescribeDBInstances", "2014-10-31", Some(body));
        let response = self
            .client
            .post(
                &url,
                "rds",
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
                    message: format!("Failed to read describe_db_instances response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_db_instances response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeDBInstancesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_db_instances XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Returns information about DB snapshots.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeDBSnapshotsRequest`]
    ///
    /// # Response
    /// [`DescribeDBSnapshotsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_db_snapshots(
        &self,
        body: &DescribeDBSnapshotsRequest,
    ) -> Result<DescribeDBSnapshotsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DescribeDBSnapshots", "2014-10-31", Some(body));
        let response = self
            .client
            .post(
                &url,
                "rds",
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
                    message: format!("Failed to read describe_db_snapshots response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_db_snapshots response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeDBSnapshotsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_db_snapshots XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Returns a list of DB snapshot attribute names and values for a manual DB snapshot.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeDBSnapshotAttributesRequest`]
    ///
    /// # Response
    /// [`DescribeDBSnapshotAttributesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_db_snapshot_attributes(
        &self,
        body: &DescribeDBSnapshotAttributesRequest,
    ) -> Result<DescribeDBSnapshotAttributesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body(
            "DescribeDBSnapshotAttributes",
            "2014-10-31",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "rds",
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
                    message: format!(
                        "Failed to read describe_db_snapshot_attributes response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_db_snapshot_attributes response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeDBSnapshotAttributesResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse describe_db_snapshot_attributes XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Modifies settings for a DB instance.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ModifyDBInstanceRequest`]
    ///
    /// # Response
    /// [`ModifyDBInstanceResponse`]
    #[allow(dead_code)]
    pub(crate) async fn modify_db_instance(
        &self,
        body: &ModifyDBInstanceRequest,
    ) -> Result<ModifyDBInstanceResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("ModifyDBInstance", "2014-10-31", Some(body));
        let response = self
            .client
            .post(
                &url,
                "rds",
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
                    message: format!("Failed to read modify_db_instance response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in modify_db_instance response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ModifyDBInstanceResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse modify_db_instance XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Stops an Amazon RDS DB instance.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`StopDBInstanceRequest`]
    ///
    /// # Response
    /// [`StopDBInstanceResponse`]
    #[allow(dead_code)]
    pub(crate) async fn stop_db_instance(
        &self,
        body: &StopDBInstanceRequest,
    ) -> Result<StopDBInstanceResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("StopDBInstance", "2014-10-31", Some(body));
        let response = self
            .client
            .post(
                &url,
                "rds",
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
                    message: format!("Failed to read stop_db_instance response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in stop_db_instance response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<StopDBInstanceResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse stop_db_instance XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Starts an Amazon RDS DB instance that was stopped.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`StartDBInstanceRequest`]
    ///
    /// # Response
    /// [`StartDBInstanceResponse`]
    #[allow(dead_code)]
    pub(crate) async fn start_db_instance(
        &self,
        body: &StartDBInstanceRequest,
    ) -> Result<StartDBInstanceResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("StartDBInstance", "2014-10-31", Some(body));
        let response = self
            .client
            .post(
                &url,
                "rds",
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
                    message: format!("Failed to read start_db_instance response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in start_db_instance response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<StartDBInstanceResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse start_db_instance XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Deletes a previously provisioned DB instance.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteDBInstanceRequest`]
    ///
    /// # Response
    /// [`DeleteDBInstanceResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_db_instance(
        &self,
        body: &DeleteDBInstanceRequest,
    ) -> Result<DeleteDBInstanceResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("DeleteDBInstance", "2014-10-31", Some(body));
        let response = self
            .client
            .post(
                &url,
                "rds",
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
                    message: format!("Failed to read delete_db_instance response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in delete_db_instance response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeleteDBInstanceResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse delete_db_instance XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Creates a snapshot of a DB instance.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`CreateDBSnapshotRequest`]
    ///
    /// # Response
    /// [`CreateDBSnapshotResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_db_snapshot(
        &self,
        body: &CreateDBSnapshotRequest,
    ) -> Result<CreateDBSnapshotResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("CreateDBSnapshot", "2014-10-31", Some(body));
        let response = self
            .client
            .post(
                &url,
                "rds",
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
                    message: format!("Failed to read create_db_snapshot response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in create_db_snapshot response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<CreateDBSnapshotResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse create_db_snapshot XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Deletes a DB snapshot.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteDBSnapshotRequest`]
    ///
    /// # Response
    /// [`DeleteDBSnapshotResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_db_snapshot(
        &self,
        body: &DeleteDBSnapshotRequest,
    ) -> Result<DeleteDBSnapshotResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("DeleteDBSnapshot", "2014-10-31", Some(body));
        let response = self
            .client
            .post(
                &url,
                "rds",
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
                    message: format!("Failed to read delete_db_snapshot response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in delete_db_snapshot response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeleteDBSnapshotResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse delete_db_snapshot XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Adds an attribute and values to, or removes an attribute and values from, a manual DB
    /// snapshot.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ModifyDBSnapshotAttributeRequest`]
    ///
    /// # Response
    /// [`ModifyDBSnapshotAttributeResponse`]
    #[allow(dead_code)]
    pub(crate) async fn modify_db_snapshot_attribute(
        &self,
        body: &ModifyDBSnapshotAttributeRequest,
    ) -> Result<ModifyDBSnapshotAttributeResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("ModifyDBSnapshotAttribute", "2014-10-31", Some(body));
        let response = self
            .client
            .post(
                &url,
                "rds",
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
                    message: format!("Failed to read modify_db_snapshot_attribute response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in modify_db_snapshot_attribute response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ModifyDBSnapshotAttributeResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse modify_db_snapshot_attribute XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_db_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeDBInstancesResponse::fixture();
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
        let ops = RdsOps::new(&client);

        let body = DescribeDBInstancesRequest::fixture();
        let result = ops.describe_db_instances(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_db_snapshots() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeDBSnapshotsResponse::fixture();
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
        let ops = RdsOps::new(&client);

        let body = DescribeDBSnapshotsRequest::fixture();
        let result = ops.describe_db_snapshots(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_db_snapshot_attributes() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeDBSnapshotAttributesResponse::fixture();
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
        let ops = RdsOps::new(&client);

        let body = DescribeDBSnapshotAttributesRequest::fixture();
        let result = ops.describe_db_snapshot_attributes(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modify_db_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ModifyDBInstanceResponse::fixture();
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
        let ops = RdsOps::new(&client);

        let body = ModifyDBInstanceRequest::fixture();
        let result = ops.modify_db_instance(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stop_db_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = StopDBInstanceResponse::fixture();
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
        let ops = RdsOps::new(&client);

        let body = StopDBInstanceRequest::fixture();
        let result = ops.stop_db_instance(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_db_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = StartDBInstanceResponse::fixture();
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
        let ops = RdsOps::new(&client);

        let body = StartDBInstanceRequest::fixture();
        let result = ops.start_db_instance(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_db_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeleteDBInstanceResponse::fixture();
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
        let ops = RdsOps::new(&client);

        let body = DeleteDBInstanceRequest::fixture();
        let result = ops.delete_db_instance(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_db_snapshot() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = CreateDBSnapshotResponse::fixture();
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
        let ops = RdsOps::new(&client);

        let body = CreateDBSnapshotRequest::fixture();
        let result = ops.create_db_snapshot(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_db_snapshot() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeleteDBSnapshotResponse::fixture();
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
        let ops = RdsOps::new(&client);

        let body = DeleteDBSnapshotRequest::fixture();
        let result = ops.delete_db_snapshot(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modify_db_snapshot_attribute() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ModifyDBSnapshotAttributeResponse::fixture();
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
        let ops = RdsOps::new(&client);

        let body = ModifyDBSnapshotAttributeRequest::fixture();
        let result = ops.modify_db_snapshot_attribute(&body).await;
        assert!(result.is_ok());
    }
}
