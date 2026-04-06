//! Operation contracts for the Elastic Load Balancing v2 API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/elbv2.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::elbv2::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Elastic Load Balancing v2 API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::elbv2::Elbv2Client`] instead.
pub struct Elbv2Ops<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> Elbv2Ops<'a> {
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
        "https://elasticloadbalancing.{region}.amazonaws.com"
            .replace("{region}", self.client.region())
    }

    /// Describes the specified load balancers or all of your load balancers.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeLoadBalancersRequest`]
    ///
    /// # Response
    /// [`DescribeLoadBalancersResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_load_balancers(
        &self,
        body: &DescribeLoadBalancersRequest,
    ) -> Result<DescribeLoadBalancersResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DescribeLoadBalancers", "2015-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticloadbalancing",
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
                    message: format!("Failed to read describe_load_balancers response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_load_balancers response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeLoadBalancersResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_load_balancers XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified target groups or all of your target groups. By default, all
    /// target groups are described. Alternatively, you can specify one of the following to
    /// filter the results: the ARN of the load balancer, the names of one or more target
    /// groups, or the ARNs of one or more target groups.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeTargetGroupsRequest`]
    ///
    /// # Response
    /// [`DescribeTargetGroupsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_target_groups(
        &self,
        body: &DescribeTargetGroupsRequest,
    ) -> Result<DescribeTargetGroupsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DescribeTargetGroups", "2015-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticloadbalancing",
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
                    message: format!("Failed to read describe_target_groups response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_target_groups response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeTargetGroupsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_target_groups XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the health of the specified targets or all of your targets.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeTargetHealthRequest`]
    ///
    /// # Response
    /// [`DescribeTargetHealthResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_target_health(
        &self,
        body: &DescribeTargetHealthRequest,
    ) -> Result<DescribeTargetHealthResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DescribeTargetHealth", "2015-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticloadbalancing",
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
                    message: format!("Failed to read describe_target_health response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_target_health response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeTargetHealthResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_target_health XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified listeners or the listeners for the specified Application Load
    /// Balancer, Network Load Balancer, or Gateway Load Balancer. You must specify either a
    /// load balancer or one or more listeners.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeListenersRequest`]
    ///
    /// # Response
    /// [`DescribeListenersResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_listeners(
        &self,
        body: &DescribeListenersRequest,
    ) -> Result<DescribeListenersResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DescribeListeners", "2015-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticloadbalancing",
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
                    message: format!("Failed to read describe_listeners response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_listeners response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeListenersResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_listeners XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the attributes for the specified Application Load Balancer, Network Load
    /// Balancer, or Gateway Load Balancer. For more information, see the following: Load
    /// balancer attributes in the Application Load Balancers Guide Load balancer attributes in
    /// the Network Load Balancers Guide Load balancer attributes in the Gateway Load Balancers
    /// Guide
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeLoadBalancerAttributesRequest`]
    ///
    /// # Response
    /// [`DescribeLoadBalancerAttributesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_load_balancer_attributes(
        &self,
        body: &DescribeLoadBalancerAttributesRequest,
    ) -> Result<DescribeLoadBalancerAttributesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body(
            "DescribeLoadBalancerAttributes",
            "2015-12-01",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "elasticloadbalancing",
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
                        "Failed to read describe_load_balancer_attributes response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Invalid UTF-8 in describe_load_balancer_attributes response: {e}"
                ),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeLoadBalancerAttributesResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse describe_load_balancer_attributes XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Deletes the specified Application Load Balancer, Network Load Balancer, or Gateway Load
    /// Balancer. Deleting a load balancer also deletes its listeners. You can't delete a load
    /// balancer if deletion protection is enabled. If the load balancer does not exist or has
    /// already been deleted, the call succeeds. Deleting a load balancer does not affect its
    /// registered targets. For example, your EC2 instances continue to run and are still
    /// registered to their target groups. If you no longer need these EC2 instances, you can
    /// stop or terminate them.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteLoadBalancerRequest`]
    ///
    /// # Response
    /// [`DeleteLoadBalancerResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_load_balancer(
        &self,
        body: &DeleteLoadBalancerRequest,
    ) -> Result<DeleteLoadBalancerResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DeleteLoadBalancer", "2015-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticloadbalancing",
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
                    message: format!("Failed to read delete_load_balancer response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in delete_load_balancer response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeleteLoadBalancerResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse delete_load_balancer XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Deletes the specified target group. You can delete a target group if it is not
    /// referenced by any actions. Deleting a target group also deletes any associated health
    /// checks. Deleting a target group does not affect its registered targets. For example, any
    /// EC2 instances continue to run until you stop or terminate them.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteTargetGroupRequest`]
    ///
    /// # Response
    /// [`DeleteTargetGroupResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_target_group(
        &self,
        body: &DeleteTargetGroupRequest,
    ) -> Result<DeleteTargetGroupResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("DeleteTargetGroup", "2015-12-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "elasticloadbalancing",
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
                    message: format!("Failed to read delete_target_group response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in delete_target_group response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeleteTargetGroupResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse delete_target_group XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Modifies the specified attributes of the specified Application Load Balancer, Network
    /// Load Balancer, or Gateway Load Balancer. If any of the specified attributes can't be
    /// modified as requested, the call fails. Any existing attributes that you do not modify
    /// retain their current values.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ModifyLoadBalancerAttributesRequest`]
    ///
    /// # Response
    /// [`ModifyLoadBalancerAttributesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn modify_load_balancer_attributes(
        &self,
        body: &ModifyLoadBalancerAttributesRequest,
    ) -> Result<ModifyLoadBalancerAttributesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body(
            "ModifyLoadBalancerAttributes",
            "2015-12-01",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "elasticloadbalancing",
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
                        "Failed to read modify_load_balancer_attributes response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in modify_load_balancer_attributes response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ModifyLoadBalancerAttributesResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse modify_load_balancer_attributes XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_load_balancers() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeLoadBalancersResponse::fixture();
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
        let ops = Elbv2Ops::new(&client);

        let body = DescribeLoadBalancersRequest::fixture();
        let result = ops.describe_load_balancers(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_target_groups() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeTargetGroupsResponse::fixture();
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
        let ops = Elbv2Ops::new(&client);

        let body = DescribeTargetGroupsRequest::fixture();
        let result = ops.describe_target_groups(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_target_health() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeTargetHealthResponse::fixture();
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
        let ops = Elbv2Ops::new(&client);

        let body = DescribeTargetHealthRequest::fixture();
        let result = ops.describe_target_health(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_listeners() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeListenersResponse::fixture();
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
        let ops = Elbv2Ops::new(&client);

        let body = DescribeListenersRequest::fixture();
        let result = ops.describe_listeners(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_load_balancer_attributes() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeLoadBalancerAttributesResponse::fixture();
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
        let ops = Elbv2Ops::new(&client);

        let body = DescribeLoadBalancerAttributesRequest::fixture();
        let result = ops.describe_load_balancer_attributes(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_load_balancer() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeleteLoadBalancerResponse::fixture();
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
        let ops = Elbv2Ops::new(&client);

        let body = DeleteLoadBalancerRequest::fixture();
        let result = ops.delete_load_balancer(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_target_group() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeleteTargetGroupResponse::fixture();
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
        let ops = Elbv2Ops::new(&client);

        let body = DeleteTargetGroupRequest::fixture();
        let result = ops.delete_target_group(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modify_load_balancer_attributes() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ModifyLoadBalancerAttributesResponse::fixture();
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
        let ops = Elbv2Ops::new(&client);

        let body = ModifyLoadBalancerAttributesRequest::fixture();
        let result = ops.modify_load_balancer_attributes(&body).await;
        assert!(result.is_ok());
    }
}
