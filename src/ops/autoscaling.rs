//! Operation contracts for the Amazon Auto Scaling API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/autoscaling.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::autoscaling::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon Auto Scaling API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::autoscaling::AutoscalingClient`] instead.
pub struct AutoscalingOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> AutoscalingOps<'a> {
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
        "https://autoscaling.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Describes one or more Auto Scaling groups.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeAutoScalingGroupsRequest`]
    ///
    /// # Response
    /// [`DescribeAutoScalingGroupsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_auto_scaling_groups(
        &self,
        body: &DescribeAutoScalingGroupsRequest,
    ) -> Result<DescribeAutoScalingGroupsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DescribeAutoScalingGroups", "2011-01-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "autoscaling",
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
                    message: format!("Failed to read describe_auto_scaling_groups response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_auto_scaling_groups response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeAutoScalingGroupsResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_auto_scaling_groups XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Describes one or more launch configurations.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeLaunchConfigurationsRequest`]
    ///
    /// # Response
    /// [`DescribeLaunchConfigurationsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_launch_configurations(
        &self,
        body: &DescribeLaunchConfigurationsRequest,
    ) -> Result<DescribeLaunchConfigurationsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body(
            "DescribeLaunchConfigurations",
            "2011-01-01",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "autoscaling",
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
                    message: format!("Failed to read describe_launch_configurations response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_launch_configurations response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeLaunchConfigurationsResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse describe_launch_configurations XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Updates the configuration for the specified Auto Scaling group.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`UpdateAutoScalingGroupRequest`]
    #[allow(dead_code)]
    pub(crate) async fn update_auto_scaling_group(
        &self,
        body: &UpdateAutoScalingGroupRequest,
    ) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("UpdateAutoScalingGroup", "2011-01-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "autoscaling",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Starts an instance refresh for the specified Auto Scaling group.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`StartInstanceRefreshRequest`]
    ///
    /// # Response
    /// [`StartInstanceRefreshResponse`]
    #[allow(dead_code)]
    pub(crate) async fn start_instance_refresh(
        &self,
        body: &StartInstanceRefreshRequest,
    ) -> Result<StartInstanceRefreshResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("StartInstanceRefresh", "2011-01-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "autoscaling",
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
                    message: format!("Failed to read start_instance_refresh response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in start_instance_refresh response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<StartInstanceRefreshResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse start_instance_refresh XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_auto_scaling_groups() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeAutoScalingGroupsResponse::fixture();
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
        let ops = AutoscalingOps::new(&client);

        let body = DescribeAutoScalingGroupsRequest::fixture();
        let result = ops.describe_auto_scaling_groups(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_launch_configurations() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeLaunchConfigurationsResponse::fixture();
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
        let ops = AutoscalingOps::new(&client);

        let body = DescribeLaunchConfigurationsRequest::fixture();
        let result = ops.describe_launch_configurations(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_auto_scaling_group() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = AutoscalingOps::new(&client);

        let body = UpdateAutoScalingGroupRequest::fixture();
        let result = ops.update_auto_scaling_group(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_instance_refresh() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = StartInstanceRefreshResponse::fixture();
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
        let ops = AutoscalingOps::new(&client);

        let body = StartInstanceRefreshRequest::fixture();
        let result = ops.start_instance_refresh(&body).await;
        assert!(result.is_ok());
    }
}
