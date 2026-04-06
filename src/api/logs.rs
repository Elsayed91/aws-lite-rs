//! Amazon CloudWatch Logs API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::logs::LogsOps`. This layer adds:
//! - Ergonomic method signatures
//! - Pagination stream helpers

use crate::{
    AwsHttpClient, Result,
    ops::logs::LogsOps,
    types::logs::{
        DeleteLogStreamRequest, DescribeLogGroupsRequest, DescribeLogGroupsResponse,
        DescribeLogStreamsRequest, DescribeLogStreamsResponse, DescribeMetricFiltersRequest,
        DescribeMetricFiltersResponse, ListTagsForResourceRequest, ListTagsForResourceResponse,
        LogGroup, MetricFilter, PutRetentionPolicyRequest,
    },
};

/// Client for the Amazon CloudWatch Logs API
pub struct LogsClient<'a> {
    ops: LogsOps<'a>,
}

impl<'a> LogsClient<'a> {
    /// Create a new Amazon CloudWatch Logs API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: LogsOps::new(client),
        }
    }

    /// Lists the specified log groups.
    pub async fn describe_log_groups(
        &self,
        body: &DescribeLogGroupsRequest,
    ) -> Result<DescribeLogGroupsResponse> {
        self.ops.describe_log_groups(body).await
    }

    /// Stream all log groups, automatically handling pagination.
    pub fn describe_log_groups_stream(
        &self,
    ) -> impl futures_core::Stream<Item = Result<LogGroup>> + '_ {
        async_stream::try_stream! {
            let mut next_token: Option<String> = None;
            loop {
                let request = DescribeLogGroupsRequest {
                    next_token: next_token.clone(),
                    ..Default::default()
                };
                let response = self.ops.describe_log_groups(&request).await?;
                for group in response.log_groups {
                    yield group;
                }
                match response.next_token {
                    Some(token) if !token.is_empty() => next_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Displays the tags associated with a CloudWatch Logs resource.
    pub async fn list_tags_for_resource(
        &self,
        body: &ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse> {
        self.ops.list_tags_for_resource(body).await
    }

    /// Lists the log streams for the specified log group.
    pub async fn describe_log_streams(
        &self,
        body: &DescribeLogStreamsRequest,
    ) -> Result<DescribeLogStreamsResponse> {
        self.ops.describe_log_streams(body).await
    }

    /// Sets the retention of the specified log group.
    pub async fn put_retention_policy(
        &self,
        log_group_name: &str,
        retention_in_days: i32,
    ) -> Result<()> {
        let body = PutRetentionPolicyRequest {
            log_group_name: log_group_name.into(),
            retention_in_days,
        };
        self.ops.put_retention_policy(&body).await
    }

    /// Deletes the specified log stream.
    pub async fn delete_log_stream(
        &self,
        log_group_name: &str,
        log_stream_name: &str,
    ) -> Result<()> {
        let body = DeleteLogStreamRequest {
            log_group_name: log_group_name.into(),
            log_stream_name: log_stream_name.into(),
        };
        self.ops.delete_log_stream(&body).await
    }

    // ── Metric Filters ─────────────────────────────────────────────────

    /// Return the first page of metric filters.
    ///
    /// Optionally filter by `log_group_name` or `filter_name_prefix`.
    /// Pass `next_token` from a previous response to paginate.
    ///
    /// CIS 5.1–5.15: check whether a metric filter with the expected pattern
    /// exists before checking the corresponding CloudWatch alarm.
    pub async fn describe_metric_filters(
        &self,
        log_group_name: Option<&str>,
        filter_name_prefix: Option<&str>,
        next_token: Option<&str>,
    ) -> Result<DescribeMetricFiltersResponse> {
        let body = DescribeMetricFiltersRequest {
            log_group_name: log_group_name.map(str::to_string),
            filter_name_prefix: filter_name_prefix.map(str::to_string),
            next_token: next_token.map(str::to_string),
            ..Default::default()
        };
        self.ops.describe_metric_filters(&body).await
    }

    /// Return all metric filters, following pagination.
    ///
    /// Optionally scope to a specific `log_group_name`.
    ///
    /// CIS 5.1–5.15: collect all metric filters, then search for the
    /// expected `filterPattern` values for each alarm check.
    pub async fn list_all_metric_filters(
        &self,
        log_group_name: Option<&str>,
    ) -> Result<Vec<MetricFilter>> {
        let mut all = Vec::new();
        let mut next_token: Option<String> = None;
        loop {
            let body = DescribeMetricFiltersRequest {
                log_group_name: log_group_name.map(str::to_string),
                next_token: next_token.clone(),
                ..Default::default()
            };
            let resp = self.ops.describe_metric_filters(&body).await?;
            all.extend(resp.metric_filters);
            match resp.next_token {
                Some(t) if !t.is_empty() => next_token = Some(t),
                _ => break,
            }
        }
        Ok(all)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_client::MockClient;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn describe_log_groups_stream_paginates() {
        let mut mock = MockClient::new();

        // Single expectation with two sequential responses
        mock.expect_post("/")
            .returning_json_sequence(vec![
                serde_json::json!({
                    "logGroups": [{"logGroupName": "/aws/lambda/page1"}],
                    "nextToken": "token-abc"
                }),
                serde_json::json!({
                    "logGroups": [{"logGroupName": "/aws/lambda/page2"}]
                }),
            ])
            .times(2);

        let client = AwsHttpClient::from_mock(mock);
        let logs = client.logs();

        let groups: Vec<LogGroup> = logs
            .describe_log_groups_stream()
            .map(|r| r.unwrap())
            .collect()
            .await;

        assert_eq!(groups.len(), 2);
        assert_eq!(
            groups[0].log_group_name,
            Some("/aws/lambda/page1".to_string())
        );
        assert_eq!(
            groups[1].log_group_name,
            Some("/aws/lambda/page2".to_string())
        );
    }

    #[tokio::test]
    async fn describe_log_groups_stream_single_page() {
        let mut mock = MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({
            "logGroups": [
                {"logGroupName": "/aws/lambda/one"},
                {"logGroupName": "/aws/lambda/two"}
            ]
        }));

        let client = AwsHttpClient::from_mock(mock);
        let logs = client.logs();

        let groups: Vec<LogGroup> = logs
            .describe_log_groups_stream()
            .map(|r| r.unwrap())
            .collect()
            .await;

        assert_eq!(groups.len(), 2);
    }

    #[tokio::test]
    async fn describe_log_groups_stream_empty() {
        let mut mock = MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({
            "logGroups": []
        }));

        let client = AwsHttpClient::from_mock(mock);
        let logs = client.logs();

        let groups: Vec<LogGroup> = logs
            .describe_log_groups_stream()
            .map(|r| r.unwrap())
            .collect()
            .await;

        assert_eq!(groups.len(), 0);
    }

    #[tokio::test]
    async fn describe_log_streams_returns_parsed_response() {
        let mut mock = MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "logStreams": [
                {
                    "logStreamName": "stream-1",
                    "creationTime": 1700000000000_i64,
                    "arn": "arn:aws:logs:us-east-1:123:log-group:/test:log-stream:stream-1"
                }
            ]
        }));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .logs()
            .describe_log_streams(&DescribeLogStreamsRequest {
                log_group_name: Some("/test".into()),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.log_streams.len(), 1);
        assert_eq!(
            result.log_streams[0].log_stream_name.as_deref(),
            Some("stream-1")
        );
        assert!(result.log_streams[0].creation_time.is_some());
    }

    #[tokio::test]
    async fn put_retention_policy_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_post("/")
            .returning_json(serde_json::json!(null));

        let client = AwsHttpClient::from_mock(mock);
        client
            .logs()
            .put_retention_policy("/test-group", 7)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn delete_log_stream_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_post("/")
            .returning_json(serde_json::json!(null));

        let client = AwsHttpClient::from_mock(mock);
        client
            .logs()
            .delete_log_stream("/test-group", "test-stream")
            .await
            .unwrap();
    }

    // ── Metric Filters ─────────────────────────────────────────────────

    #[tokio::test]
    async fn describe_metric_filters_returns_filters() {
        let mut mock = MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "metricFilters": [
                {
                    "filterName": "UnauthorizedApiCalls",
                    "filterPattern": "{ ($.errorCode = \"*UnauthorizedOperation\") || ($.errorCode = \"AccessDenied*\") }",
                    "logGroupName": "CloudTrail/DefaultLogGroup",
                    "creationTime": 1700000000000_i64,
                    "metricTransformations": [
                        {
                            "metricName": "UnauthorizedApiCallCount",
                            "metricNamespace": "CISBenchmark",
                            "metricValue": "1"
                        }
                    ]
                }
            ]
        }));

        let client = AwsHttpClient::from_mock(mock);
        let resp = client
            .logs()
            .describe_metric_filters(Some("CloudTrail/DefaultLogGroup"), None, None)
            .await
            .unwrap();

        assert_eq!(resp.metric_filters.len(), 1);
        let f = &resp.metric_filters[0];
        assert_eq!(f.filter_name.as_deref(), Some("UnauthorizedApiCalls"));
        assert!(
            f.filter_pattern
                .as_deref()
                .unwrap_or("")
                .contains("UnauthorizedOperation")
        );
        assert_eq!(
            f.log_group_name.as_deref(),
            Some("CloudTrail/DefaultLogGroup")
        );
        assert_eq!(f.metric_transformations.len(), 1);
        assert_eq!(
            f.metric_transformations[0].metric_name,
            "UnauthorizedApiCallCount"
        );
        assert_eq!(f.metric_transformations[0].metric_namespace, "CISBenchmark");
    }

    #[tokio::test]
    async fn describe_metric_filters_handles_empty() {
        let mut mock = MockClient::new();
        mock.expect_post("/")
            .returning_json(serde_json::json!({"metricFilters": []}));

        let client = AwsHttpClient::from_mock(mock);
        let resp = client
            .logs()
            .describe_metric_filters(None, None, None)
            .await
            .unwrap();
        assert!(resp.metric_filters.is_empty());
    }

    #[tokio::test]
    async fn list_all_metric_filters_paginates() {
        let mut mock = MockClient::new();
        mock.expect_post("/")
            .returning_json_sequence(vec![
                serde_json::json!({
                    "metricFilters": [{
                        "filterName": "Filter1",
                        "filterPattern": "{ $.errorCode = \"*\" }",
                        "logGroupName": "/aws/cloudtrail",
                        "metricTransformations": [{"metricName": "M1", "metricNamespace": "NS", "metricValue": "1"}]
                    }],
                    "nextToken": "page2"
                }),
                serde_json::json!({
                    "metricFilters": [{
                        "filterName": "Filter2",
                        "filterPattern": "{ $.eventName = \"ConsoleLogin\" }",
                        "logGroupName": "/aws/cloudtrail",
                        "metricTransformations": [{"metricName": "M2", "metricNamespace": "NS", "metricValue": "1"}]
                    }]
                }),
            ])
            .times(2);

        let client = AwsHttpClient::from_mock(mock);
        let filters = client
            .logs()
            .list_all_metric_filters(Some("/aws/cloudtrail"))
            .await
            .unwrap();

        assert_eq!(filters.len(), 2);
        assert_eq!(filters[0].filter_name.as_deref(), Some("Filter1"));
        assert_eq!(filters[1].filter_name.as_deref(), Some("Filter2"));
    }
}
