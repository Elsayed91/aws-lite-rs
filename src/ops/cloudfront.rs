//! Operation contracts for the Amazon CloudFront API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/cloudfront.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::cloudfront::*;
use crate::{AwsHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Amazon CloudFront API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::cloudfront::CloudfrontClient`] instead.
pub struct CloudfrontOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> CloudfrontOps<'a> {
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
        "https://cloudfront.amazonaws.com"
    }

    /// List CloudFront distributions.
    ///
    /// **AWS API**: `GET /2020-05-31/distribution`
    ///
    /// # Response
    /// [`DistributionList`]
    #[allow(dead_code)]
    pub(crate) async fn list_distributions(&self) -> Result<DistributionList> {
        let url = format!("{}/2020-05-31/distribution", self.base_url(),);
        let response = self.client.get(&url, "cloudfront").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_distributions response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_distributions response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<DistributionList>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_distributions XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Get the configuration for a CloudFront distribution.
    ///
    /// **AWS API**: `GET /2020-05-31/distribution/{Id}/config`
    ///
    /// # Path Parameters
    /// - `Id` —  *(required)*
    ///
    /// # Response
    /// [`DistributionConfig`]
    #[allow(dead_code)]
    pub(crate) async fn get_distribution_config(&self, id: &str) -> Result<DistributionConfig> {
        let url = format!(
            "{}/2020-05-31/distribution/{}/config",
            self.base_url(),
            encode(id),
        );
        let response = self.client.get(&url, "cloudfront").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_distribution_config response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_distribution_config response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<DistributionConfig>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_distribution_config XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Update a CloudFront distribution configuration.
    ///
    /// **AWS API**: `PUT /2020-05-31/distribution/{Id}/config`
    ///
    /// # Path Parameters
    /// - `Id` —  *(required)*
    ///
    /// # Request Body
    /// [`DistributionConfig`]
    ///
    /// # Response
    /// [`Distribution`]
    #[allow(dead_code)]
    pub(crate) async fn update_distribution(
        &self,
        id: &str,
        body: &DistributionConfig,
    ) -> Result<Distribution> {
        let url = format!(
            "{}/2020-05-31/distribution/{}/config",
            self.base_url(),
            encode(id),
        );
        let body_xml =
            quick_xml::se::to_string(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize update_distribution request to XML: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .put(&url, "cloudfront", body_xml.as_bytes(), "application/xml")
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read update_distribution response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in update_distribution response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<Distribution>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse update_distribution XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Creates a new origin access control in CloudFront.
    ///
    /// **AWS API**: `POST /2020-05-31/origin-access-control`
    ///
    /// # Request Body
    /// [`OriginAccessControlConfig`]
    ///
    /// # Response
    /// [`OriginAccessControl`]
    #[allow(dead_code)]
    pub(crate) async fn create_origin_access_control(
        &self,
        body: &OriginAccessControlConfig,
    ) -> Result<OriginAccessControl> {
        let url = format!("{}/2020-05-31/origin-access-control", self.base_url(),);
        let body_xml =
            quick_xml::se::to_string(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to serialize create_origin_access_control request to XML: {e}"
                ),
                body: None,
            })?;
        let response = self
            .client
            .post(&url, "cloudfront", body_xml.as_bytes(), "application/xml")
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read create_origin_access_control response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in create_origin_access_control response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<OriginAccessControl>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse create_origin_access_control XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Creates a new CloudFront distribution.
    ///
    /// **AWS API**: `POST /2020-05-31/distribution`
    ///
    /// # Request Body
    /// [`DistributionConfig`]
    ///
    /// # Response
    /// [`Distribution`]
    #[allow(dead_code)]
    pub(crate) async fn create_distribution(
        &self,
        body: &DistributionConfig,
    ) -> Result<Distribution> {
        let url = format!("{}/2020-05-31/distribution", self.base_url(),);
        let body_xml =
            quick_xml::se::to_string(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize create_distribution request to XML: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post(&url, "cloudfront", body_xml.as_bytes(), "application/xml")
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read create_distribution response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in create_distribution response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<Distribution>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse create_distribution XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_distributions() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/2020-05-31/distribution")
            .returning_bytes({
                let fixture = DistributionList::fixture();
                quick_xml::se::to_string(&fixture).unwrap().into_bytes()
            });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudfrontOps::new(&client);

        let result = ops.list_distributions().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_distribution_config() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/2020-05-31/distribution/test-Id/config")
            .returning_bytes({
                let fixture = DistributionConfig::fixture();
                quick_xml::se::to_string(&fixture).unwrap().into_bytes()
            });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudfrontOps::new(&client);

        let result = ops.get_distribution_config("test-Id").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_distribution() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/2020-05-31/distribution/test-Id/config")
            .returning_bytes({
                let fixture = Distribution::fixture();
                quick_xml::se::to_string(&fixture).unwrap().into_bytes()
            });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudfrontOps::new(&client);

        let body = DistributionConfig::fixture();
        let result = ops.update_distribution("test-Id", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_origin_access_control() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/2020-05-31/origin-access-control")
            .returning_bytes({
                let fixture = OriginAccessControl::fixture();
                quick_xml::se::to_string(&fixture).unwrap().into_bytes()
            });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudfrontOps::new(&client);

        let body = OriginAccessControlConfig::fixture();
        let result = ops.create_origin_access_control(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_distribution() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/2020-05-31/distribution")
            .returning_bytes({
                let fixture = Distribution::fixture();
                quick_xml::se::to_string(&fixture).unwrap().into_bytes()
            });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudfrontOps::new(&client);

        let body = DistributionConfig::fixture();
        let result = ops.create_distribution(&body).await;
        assert!(result.is_ok());
    }
}
