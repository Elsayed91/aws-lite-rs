//! Operation contracts for the Amazon Route 53 API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/route53.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::route53::*;
use crate::{AwsHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Amazon Route 53 API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::route53::Route53Client`] instead.
pub struct Route53Ops<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> Route53Ops<'a> {
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://route53.amazonaws.com"
    }

    /// Retrieves a list of the public and private hosted zones associated with the current AWS
    /// account.
    ///
    /// **AWS API**: `GET /2013-04-01/hostedzone`
    ///
    /// # Response
    /// [`ListHostedZonesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_hosted_zones(&self) -> Result<ListHostedZonesResponse> {
        let url = format!("{}/2013-04-01/hostedzone", self.base_url(),);
        let response = self.client.get(&url, "route53").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_hosted_zones response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_hosted_zones response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<ListHostedZonesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_hosted_zones XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Lists the resource record sets in a specified hosted zone.
    ///
    /// **AWS API**: `GET /2013-04-01/hostedzone/{Id}/rrset`
    ///
    /// # Path Parameters
    /// - `Id` —  *(required)*
    ///
    /// # Response
    /// [`ListResourceRecordSetsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_resource_record_sets(
        &self,
        id: &str,
    ) -> Result<ListResourceRecordSetsResponse> {
        let url = format!(
            "{}/2013-04-01/hostedzone/{}/rrset",
            self.base_url(),
            encode(id),
        );
        let response = self.client.get(&url, "route53").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_resource_record_sets response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_resource_record_sets response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<ListResourceRecordSetsResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_resource_record_sets XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Retrieve a list of the health checks associated with the current AWS account.
    ///
    /// **AWS API**: `GET /2013-04-01/healthcheck`
    ///
    /// # Response
    /// [`ListHealthChecksResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_health_checks(&self) -> Result<ListHealthChecksResponse> {
        let url = format!("{}/2013-04-01/healthcheck", self.base_url(),);
        let response = self.client.get(&url, "route53").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_health_checks response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_health_checks response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<ListHealthChecksResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_health_checks XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Gets status of a health check based on the most recent checker observations.
    ///
    /// **AWS API**: `GET /2013-04-01/healthcheck/{HealthCheckId}/status`
    ///
    /// # Path Parameters
    /// - `HealthCheckId` —  *(required)*
    ///
    /// # Response
    /// [`GetHealthCheckStatusResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_health_check_status(
        &self,
        health_check_id: &str,
    ) -> Result<GetHealthCheckStatusResponse> {
        let url = format!(
            "{}/2013-04-01/healthcheck/{}/status",
            self.base_url(),
            encode(health_check_id),
        );
        let response = self.client.get(&url, "route53").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_health_check_status response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_health_check_status response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<GetHealthCheckStatusResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_health_check_status XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Creates a new health check.
    ///
    /// **AWS API**: `POST /2013-04-01/healthcheck`
    ///
    /// # Request Body
    /// [`CreateHealthCheckRequest`]
    ///
    /// # Response
    /// [`CreateHealthCheckResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_health_check(
        &self,
        body: &CreateHealthCheckRequest,
    ) -> Result<CreateHealthCheckResponse> {
        let url = format!("{}/2013-04-01/healthcheck", self.base_url(),);
        let body_xml =
            quick_xml::se::to_string(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize create_health_check request to XML: {e}"),
                body: None,
            })?;
        // Inject XML namespace required by route53.
        let body_xml = crate::xml::inject_xml_namespace(
            &body_xml,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        let response = self
            .client
            .post(&url, "route53", body_xml.as_bytes(), "application/xml")
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read create_health_check response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in create_health_check response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<CreateHealthCheckResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse create_health_check XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Deletes a health check.
    ///
    /// **AWS API**: `DELETE /2013-04-01/healthcheck/{HealthCheckId}`
    ///
    /// # Path Parameters
    /// - `HealthCheckId` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_health_check(&self, health_check_id: &str) -> Result<()> {
        let url = format!(
            "{}/2013-04-01/healthcheck/{}",
            self.base_url(),
            encode(health_check_id),
        );
        let response = self.client.delete(&url, "route53").await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Creates, changes, or deletes a resource record set.
    ///
    /// **AWS API**: `POST /2013-04-01/hostedzone/{Id}/rrset/`
    ///
    /// # Path Parameters
    /// - `Id` —  *(required)*
    ///
    /// # Request Body
    /// [`ChangeResourceRecordSetsRequest`]
    ///
    /// # Response
    /// [`ChangeResourceRecordSetsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn change_resource_record_sets(
        &self,
        id: &str,
        body: &ChangeResourceRecordSetsRequest,
    ) -> Result<ChangeResourceRecordSetsResponse> {
        let url = format!(
            "{}/2013-04-01/hostedzone/{}/rrset/",
            self.base_url(),
            encode(id),
        );
        let body_xml =
            quick_xml::se::to_string(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to serialize change_resource_record_sets request to XML: {e}"
                ),
                body: None,
            })?;
        // Inject XML namespace required by route53.
        let body_xml = crate::xml::inject_xml_namespace(
            &body_xml,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        let response = self
            .client
            .post(&url, "route53", body_xml.as_bytes(), "application/xml")
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read change_resource_record_sets response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in change_resource_record_sets response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<ChangeResourceRecordSetsResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse change_resource_record_sets XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_hosted_zones() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/2013-04-01/hostedzone").returning_bytes({
            let fixture = ListHostedZonesResponse::fixture();
            quick_xml::se::to_string(&fixture).unwrap().into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Route53Ops::new(&client);

        let result = ops.list_hosted_zones().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_resource_record_sets() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/2013-04-01/hostedzone/test-Id/rrset")
            .returning_bytes({
                let fixture = ListResourceRecordSetsResponse::fixture();
                quick_xml::se::to_string(&fixture).unwrap().into_bytes()
            });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Route53Ops::new(&client);

        let result = ops.list_resource_record_sets("test-Id").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_health_checks() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/2013-04-01/healthcheck").returning_bytes({
            let fixture = ListHealthChecksResponse::fixture();
            quick_xml::se::to_string(&fixture).unwrap().into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Route53Ops::new(&client);

        let result = ops.list_health_checks().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_health_check_status() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/2013-04-01/healthcheck/test-HealthCheckId/status")
            .returning_bytes({
                let fixture = GetHealthCheckStatusResponse::fixture();
                quick_xml::se::to_string(&fixture).unwrap().into_bytes()
            });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Route53Ops::new(&client);

        let result = ops.get_health_check_status("test-HealthCheckId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_health_check() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/2013-04-01/healthcheck")
            .returning_bytes({
                let fixture = CreateHealthCheckResponse::fixture();
                quick_xml::se::to_string(&fixture).unwrap().into_bytes()
            });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Route53Ops::new(&client);

        let body = CreateHealthCheckRequest::fixture();
        let result = ops.create_health_check(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_health_check() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/2013-04-01/healthcheck/test-HealthCheckId")
            .returning_bytes(vec![]);

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Route53Ops::new(&client);

        let result = ops.delete_health_check("test-HealthCheckId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_change_resource_record_sets() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/2013-04-01/hostedzone/test-Id/rrset/")
            .returning_bytes({
                let fixture = ChangeResourceRecordSetsResponse::fixture();
                quick_xml::se::to_string(&fixture).unwrap().into_bytes()
            });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Route53Ops::new(&client);

        let body = ChangeResourceRecordSetsRequest::fixture();
        let result = ops.change_resource_record_sets("test-Id", &body).await;
        assert!(result.is_ok());
    }
}
