//! Amazon CloudWatch API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::cloudwatch::CloudwatchOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::cloudwatch::CloudwatchOps,
    types::cloudwatch::{
        DeleteAlarmsInput, DescribeAlarmsInput, DescribeAlarmsResponse, GetMetricDataInput,
        GetMetricDataResponse, GetMetricStatisticsInput, GetMetricStatisticsResponse,
        ListMetricsInput, ListMetricsResponse, PutMetricAlarmInput,
    },
};

/// Client for the Amazon CloudWatch API
pub struct CloudWatchClient<'a> {
    ops: CloudwatchOps<'a>,
}

impl<'a> CloudWatchClient<'a> {
    /// Create a new Amazon CloudWatch API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: CloudwatchOps::new(client),
        }
    }

    /// Gets statistics for the specified metric.
    pub async fn get_metric_statistics(
        &self,
        body: &GetMetricStatisticsInput,
    ) -> Result<GetMetricStatisticsResponse> {
        self.ops.get_metric_statistics(body).await
    }

    /// List the specified metrics.
    pub async fn list_metrics(&self, body: &ListMetricsInput) -> Result<ListMetricsResponse> {
        self.ops.list_metrics(body).await
    }

    /// Retrieves the specified alarms.
    pub async fn describe_alarms(
        &self,
        body: &DescribeAlarmsInput,
    ) -> Result<DescribeAlarmsResponse> {
        self.ops.describe_alarms(body).await
    }

    /// Creates or updates an alarm and associates it with the specified metric.
    pub async fn put_metric_alarm(&self, body: &PutMetricAlarmInput) -> Result<()> {
        self.ops.put_metric_alarm(body).await
    }

    /// Deletes the specified alarms.
    pub async fn delete_alarms(&self, alarm_names: Vec<String>) -> Result<()> {
        let body = DeleteAlarmsInput { alarm_names };
        self.ops.delete_alarms(&body).await
    }

    /// Retrieves CloudWatch metric values for one or more metrics in a single batch request.
    ///
    /// A single call can include up to 500 `MetricDataQuery` structures. Supports math
    /// expressions across metrics. Use `next_token` in the returned response to page through
    /// results when the requested time range yields more than `max_datapoints` data points.
    pub async fn get_metric_data(
        &self,
        body: &GetMetricDataInput,
    ) -> Result<GetMetricDataResponse> {
        self.ops.get_metric_data(body).await
    }
}

#[cfg(test)]
mod tests {
    use crate::types::cloudwatch::{
        GetMetricDataInput, Metric, MetricDataQuery, MetricDataResult, MetricStat, ScanBy,
        StatusCode, *,
    };

    #[tokio::test]
    async fn list_metrics_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<ListMetricsResponse><ListMetricsResult>
                <Metrics>
                    <member><Namespace>AWS/EC2</Namespace><MetricName>CPUUtilization</MetricName></member>
                </Metrics>
            </ListMetricsResult></ListMetricsResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .cloudwatch()
            .list_metrics(&ListMetricsInput {
                namespace: Some("AWS/EC2".into()),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.metrics.len(), 1);
        assert_eq!(result.metrics[0].namespace.as_deref(), Some("AWS/EC2"));
        assert_eq!(
            result.metrics[0].metric_name.as_deref(),
            Some("CPUUtilization")
        );
    }

    #[tokio::test]
    async fn describe_alarms_returns_metric_alarms() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeAlarmsResponse><DescribeAlarmsResult>
                <MetricAlarms>
                    <member>
                        <AlarmName>test-alarm</AlarmName>
                        <MetricName>CPUUtilization</MetricName>
                        <Namespace>AWS/EC2</Namespace>
                        <StateValue>OK</StateValue>
                        <Threshold>90.0</Threshold>
                        <ComparisonOperator>GreaterThanThreshold</ComparisonOperator>
                        <EvaluationPeriods>1</EvaluationPeriods>
                        <Period>300</Period>
                        <Statistic>Average</Statistic>
                    </member>
                </MetricAlarms>
            </DescribeAlarmsResult></DescribeAlarmsResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .cloudwatch()
            .describe_alarms(&DescribeAlarmsInput {
                alarm_names: vec!["test-alarm".into()],
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.metric_alarms.len(), 1);
        let alarm = &result.metric_alarms[0];
        assert_eq!(alarm.alarm_name.as_deref(), Some("test-alarm"));
        assert_eq!(alarm.metric_name.as_deref(), Some("CPUUtilization"));
        assert_eq!(alarm.state_value.as_deref(), Some("OK"));
        assert!((alarm.threshold.unwrap() - 90.0).abs() < f64::EPSILON);
        assert_eq!(
            alarm.comparison_operator,
            Some(ComparisonOperator::GreaterThanThreshold)
        );
        assert_eq!(alarm.statistic, Some(Statistic::Average));
    }

    #[tokio::test]
    async fn put_metric_alarm_succeeds() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(vec![]);
        let client = crate::AwsHttpClient::from_mock(mock);
        let input = PutMetricAlarmInput {
            alarm_name: "test-alarm".into(),
            metric_name: Some("CPUUtilization".into()),
            namespace: Some("AWS/EC2".into()),
            statistic: Some(Statistic::Average),
            period: Some(300),
            evaluation_periods: 1,
            threshold: Some(90.0),
            comparison_operator: Some(ComparisonOperator::GreaterThanThreshold),
            ..Default::default()
        };
        client.cloudwatch().put_metric_alarm(&input).await.unwrap();
    }

    #[tokio::test]
    async fn delete_alarms_succeeds() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(vec![]);
        let client = crate::AwsHttpClient::from_mock(mock);
        client
            .cloudwatch()
            .delete_alarms(vec!["test-alarm".into()])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_metric_data_returns_results_for_all_query_ids() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<GetMetricDataResponse><GetMetricDataResult>
                <MetricDataResults>
                    <member>
                        <Id>cpu</Id>
                        <Label>CPUUtilization</Label>
                        <Timestamps><member>2024-01-01T00:00:00Z</member></Timestamps>
                        <Values><member>42.5</member></Values>
                        <StatusCode>Complete</StatusCode>
                    </member>
                    <member>
                        <Id>networkin</Id>
                        <Label>NetworkIn</Label>
                        <StatusCode>Complete</StatusCode>
                    </member>
                </MetricDataResults>
            </GetMetricDataResult></GetMetricDataResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let input = GetMetricDataInput {
            metric_data_queries: vec![
                MetricDataQuery {
                    id: "cpu".into(),
                    metric_stat: Some(MetricStat {
                        metric: Metric {
                            namespace: Some("AWS/EC2".into()),
                            metric_name: Some("CPUUtilization".into()),
                            dimensions: vec![],
                        },
                        period: 300,
                        stat: "Average".into(),
                        ..Default::default()
                    }),
                    return_data: Some(true),
                    ..Default::default()
                },
                MetricDataQuery {
                    id: "networkin".into(),
                    metric_stat: Some(MetricStat {
                        metric: Metric {
                            namespace: Some("AWS/EC2".into()),
                            metric_name: Some("NetworkIn".into()),
                            dimensions: vec![],
                        },
                        period: 300,
                        stat: "Sum".into(),
                        ..Default::default()
                    }),
                    return_data: Some(true),
                    ..Default::default()
                },
            ],
            start_time: "2024-01-01T00:00:00Z".into(),
            end_time: "2024-01-01T01:00:00Z".into(),
            ..Default::default()
        };
        let response = client.cloudwatch().get_metric_data(&input).await.unwrap();
        assert_eq!(response.metric_data_results.len(), 2);
        let cpu = &response.metric_data_results[0];
        assert_eq!(cpu.id.as_deref(), Some("cpu"));
        assert_eq!(cpu.label.as_deref(), Some("CPUUtilization"));
        assert_eq!(cpu.values.len(), 1);
        assert!((cpu.values[0] - 42.5).abs() < f64::EPSILON);
        assert_eq!(cpu.status_code, Some(StatusCode::Complete));
        let net = &response.metric_data_results[1];
        assert_eq!(net.id.as_deref(), Some("networkin"));
        assert_eq!(net.status_code, Some(StatusCode::Complete));
    }

    #[tokio::test]
    async fn get_metric_data_returns_next_token_for_pagination() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<GetMetricDataResponse><GetMetricDataResult>
                <MetricDataResults>
                    <member>
                        <Id>cpu</Id>
                        <StatusCode>Complete</StatusCode>
                    </member>
                </MetricDataResults>
                <NextToken>token-page2</NextToken>
            </GetMetricDataResult></GetMetricDataResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let input = GetMetricDataInput {
            metric_data_queries: vec![MetricDataQuery {
                id: "cpu".into(),
                metric_stat: Some(MetricStat {
                    metric: Metric {
                        namespace: Some("AWS/EC2".into()),
                        metric_name: Some("CPUUtilization".into()),
                        dimensions: vec![],
                    },
                    period: 300,
                    stat: "Average".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            start_time: "2024-01-01T00:00:00Z".into(),
            end_time: "2024-01-01T01:00:00Z".into(),
            max_datapoints: Some(1),
            ..Default::default()
        };
        let response = client.cloudwatch().get_metric_data(&input).await.unwrap();
        assert_eq!(response.next_token.as_deref(), Some("token-page2"));
        assert_eq!(response.metric_data_results.len(), 1);
    }

    #[tokio::test]
    async fn get_metric_data_with_math_expression_returns_only_return_data_results() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<GetMetricDataResponse><GetMetricDataResult>
                <MetricDataResults>
                    <member>
                        <Id>error_rate</Id>
                        <Label>error_rate</Label>
                        <StatusCode>Complete</StatusCode>
                    </member>
                </MetricDataResults>
            </GetMetricDataResult></GetMetricDataResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let input = GetMetricDataInput {
            metric_data_queries: vec![
                MetricDataQuery {
                    id: "errors".into(),
                    metric_stat: Some(MetricStat {
                        metric: Metric {
                            namespace: Some("AWS/Lambda".into()),
                            metric_name: Some("Errors".into()),
                            dimensions: vec![],
                        },
                        period: 300,
                        stat: "Sum".into(),
                        ..Default::default()
                    }),
                    return_data: Some(false),
                    ..Default::default()
                },
                MetricDataQuery {
                    id: "error_rate".into(),
                    expression: Some("errors / 100".into()),
                    return_data: Some(true),
                    ..Default::default()
                },
            ],
            start_time: "2024-01-01T00:00:00Z".into(),
            end_time: "2024-01-02T00:00:00Z".into(),
            scan_by: Some(ScanBy::TimestampDescending),
            ..Default::default()
        };
        let response = client.cloudwatch().get_metric_data(&input).await.unwrap();
        assert_eq!(response.metric_data_results.len(), 1);
        let result = &response.metric_data_results[0];
        assert_eq!(result.id.as_deref(), Some("error_rate"));
        assert_eq!(result.status_code, Some(StatusCode::Complete));
    }
}
