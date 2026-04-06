//! Operation contracts for the Amazon CloudWatch API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/cloudwatch.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::cloudwatch::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon CloudWatch API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::cloudwatch::CloudwatchClient`] instead.
pub struct CloudwatchOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> CloudwatchOps<'a> {
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
        "https://monitoring.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Gets statistics for the specified metric.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetMetricStatisticsInput`]
    ///
    /// # Response
    /// [`GetMetricStatisticsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_metric_statistics(
        &self,
        body: &GetMetricStatisticsInput,
    ) -> Result<GetMetricStatisticsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("GetMetricStatistics", "2010-08-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "monitoring",
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
                    message: format!("Failed to read get_metric_statistics response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_metric_statistics response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetMetricStatisticsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_metric_statistics XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// List the specified metrics.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListMetricsInput`]
    ///
    /// # Response
    /// [`ListMetricsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_metrics(
        &self,
        body: &ListMetricsInput,
    ) -> Result<ListMetricsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("ListMetrics", "2010-08-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "monitoring",
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
                    message: format!("Failed to read list_metrics response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_metrics response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListMetricsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_metrics XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Retrieves the specified alarms.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeAlarmsInput`]
    ///
    /// # Response
    /// [`DescribeAlarmsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_alarms(
        &self,
        body: &DescribeAlarmsInput,
    ) -> Result<DescribeAlarmsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("DescribeAlarms", "2010-08-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "monitoring",
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
                    message: format!("Failed to read describe_alarms response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_alarms response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeAlarmsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_alarms XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Creates or updates an alarm and associates it with the specified metric.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`PutMetricAlarmInput`]
    #[allow(dead_code)]
    pub(crate) async fn put_metric_alarm(&self, body: &PutMetricAlarmInput) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("PutMetricAlarm", "2010-08-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "monitoring",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Deletes the specified alarms.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteAlarmsInput`]
    #[allow(dead_code)]
    pub(crate) async fn delete_alarms(&self, body: &DeleteAlarmsInput) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("DeleteAlarms", "2010-08-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "monitoring",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Retrieves CloudWatch metric values for one or more metrics in a single batch request.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetMetricDataInput`]
    ///
    /// # Response
    /// [`GetMetricDataResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_metric_data(
        &self,
        body: &GetMetricDataInput,
    ) -> Result<GetMetricDataResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("GetMetricData", "2010-08-01", Some(body));
        let response = self
            .client
            .post(
                &url,
                "monitoring",
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
                    message: format!("Failed to read get_metric_data response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_metric_data response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetMetricDataResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_metric_data XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_metric_statistics() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetMetricStatisticsResponse::fixture();
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
        let ops = CloudwatchOps::new(&client);

        let body = GetMetricStatisticsInput::fixture();
        let result = ops.get_metric_statistics(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_metrics() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListMetricsResponse::fixture();
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
        let ops = CloudwatchOps::new(&client);

        let body = ListMetricsInput::fixture();
        let result = ops.list_metrics(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_alarms() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeAlarmsResponse::fixture();
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
        let ops = CloudwatchOps::new(&client);

        let body = DescribeAlarmsInput::fixture();
        let result = ops.describe_alarms(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_metric_alarm() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudwatchOps::new(&client);

        let body = PutMetricAlarmInput::fixture();
        let result = ops.put_metric_alarm(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_alarms() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudwatchOps::new(&client);

        let body = DeleteAlarmsInput::fixture();
        let result = ops.delete_alarms(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_metric_data() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetMetricDataResponse::fixture();
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
        let ops = CloudwatchOps::new(&client);

        let body = GetMetricDataInput::fixture();
        let result = ops.get_metric_data(&body).await;
        assert!(result.is_ok());
    }
}
