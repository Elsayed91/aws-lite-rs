//! Operation contracts for the AWS Security Token Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/sts.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::sts::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the AWS Security Token Service API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::sts::StsClient`] instead.
pub struct StsOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> StsOps<'a> {
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
        "https://sts.amazonaws.com"
    }

    /// Returns details about the IAM user or role whose credentials are used to call the
    /// operation.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetCallerIdentityRequest`]
    ///
    /// # Response
    /// [`GetCallerIdentityResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_caller_identity(
        &self,
        body: &GetCallerIdentityRequest,
    ) -> Result<GetCallerIdentityResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("GetCallerIdentity", "2011-06-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "sts",
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
                    message: format!("Failed to read get_caller_identity response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_caller_identity response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetCallerIdentityResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_caller_identity XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Returns a set of temporary security credentials for cross-account access.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`AssumeRoleRequest`]
    ///
    /// # Response
    /// [`AssumeRoleResponse`]
    #[allow(dead_code)]
    pub(crate) async fn assume_role(&self, body: &AssumeRoleRequest) -> Result<AssumeRoleResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("AssumeRole", "2011-06-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "sts",
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
                    message: format!("Failed to read assume_role response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in assume_role response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<AssumeRoleResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse assume_role XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_caller_identity() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetCallerIdentityResponse::fixture();
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
        let ops = StsOps::new(&client);

        let body = GetCallerIdentityRequest::fixture();
        let result = ops.get_caller_identity(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_assume_role() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = AssumeRoleResponse::fixture();
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
        let ops = StsOps::new(&client);

        let body = AssumeRoleRequest::fixture();
        let result = ops.assume_role(&body).await;
        assert!(result.is_ok());
    }
}
