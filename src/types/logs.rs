//! Types for the Amazon CloudWatch Logs API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Possible values for `logs.LogGroup.logGroupClass`.
///
/// **AWS API**: `logs.LogGroup.logGroupClass`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogGroupClass {
    Standard,

    InfrequentAccess,

    Delivery,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Represents a log group.
///
/// **AWS API**: `logs.v1.LogGroup`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//LogGroup>
///
/// ## Coverage
/// 8 of 12 fields included.
/// Omitted fields:
/// - `metricFilterCount` — not selected in manifest
/// - `dataProtectionStatus` — not selected in manifest
/// - `inheritedProperties` — not selected in manifest
/// - `deletionProtectionEnabled` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogGroup {
    /// The name of the log group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,

    /// The Amazon Resource Name (ARN) of the log group. This version of the ARN includes a
    /// trailing :* after the log group name. Use this version to refer to the ARN in IAM
    /// policies when specifying permissions for most API actions. The exception is when
    /// specifying permissions for TagResource, UntagResource, and ListTagsForResource. The
    /// permissions for those three actions require the ARN version that doesn't include a
    /// trailing :*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The creation time of the log group, expressed as the number of milliseconds after Jan 1,
    /// 1970 00:00:00 UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,

    /// The number of bytes stored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_bytes: Option<i64>,

    /// The `retentionInDays` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,

    /// The Amazon Resource Name (ARN) of the KMS key to use when encrypting log data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    /// This specifies the log group class for this log group. There are three classes: The
    /// Standard log class supports all CloudWatch Logs features. The Infrequent Access log
    /// class supports a subset of CloudWatch Logs features and incurs lower costs. Use the
    /// Delivery log class only for delivering Lambda logs to store in Amazon S3 or Amazon Data
    /// Firehose. Log events in log groups in the Delivery class are kept in CloudWatch Logs for
    /// only one day. This log class doesn't offer rich CloudWatch Logs capabilities such as
    /// CloudWatch Logs Insights queries. For details about the features supported by the
    /// Standard and Infrequent Access classes, see Log classes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_class: Option<LogGroupClass>,

    /// The Amazon Resource Name (ARN) of the log group. This version of the ARN doesn't include
    /// a trailing :* after the log group name. Use this version to refer to the ARN in the
    /// following situations: In the logGroupIdentifier input field in many CloudWatch Logs
    /// APIs. In the resourceArn field in tagging APIs In IAM policies, when specifying
    /// permissions for TagResource, UntagResource, and ListTagsForResource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

impl LogGroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            log_group_name: Some("test-log_group_name".into()),
            arn: Some("test-arn".into()),
            creation_time: Some(100),
            stored_bytes: Some(100),
            retention_in_days: Some(100),
            kms_key_id: Some("test-kms_key_id".into()),
            log_group_arn: Some("test-log_group_arn".into()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `logs.v1.DescribeLogGroupsResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//DescribeLogGroupsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeLogGroupsResponse {
    /// An array of structures, where each structure contains the information about one log
    /// group.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub log_groups: Vec<LogGroup>,

    /// The `nextToken` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeLogGroupsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            log_groups: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

///
/// **AWS API**: `logs.v1.ListTagsForResourceResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//ListTagsForResourceResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTagsForResourceResponse {
    /// The list of tags associated with the requested resource.&gt;
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

impl ListTagsForResourceResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tags: Default::default(),
        }
    }
}

///
/// **AWS API**: `logs.v1.DescribeLogStreamsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//DescribeLogStreamsRequest>
///
/// ## Coverage
/// 6 of 7 fields included.
/// Omitted fields:
/// - `logGroupIdentifier` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeLogStreamsRequest {
    /// The name of the log group. You must include either logGroupIdentifier or logGroupName,
    /// but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,

    /// The prefix to match. If orderBy is LastEventTime, you cannot specify this parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,

    /// If the value is LogStreamName, the results are ordered by log stream name. If the value
    /// is LastEventTime, the results are ordered by the event time. The default value is
    /// LogStreamName. If you order the results by event time, you cannot specify the
    /// logStreamNamePrefix parameter. lastEventTimestamp represents the time of the most recent
    /// log event in the log stream in CloudWatch Logs. This number is expressed as the number
    /// of milliseconds after Jan 1, 1970 00:00:00 UTC. lastEventTimestamp updates on an
    /// eventual consistency basis. It typically updates in less than an hour from ingestion,
    /// but in rare situations might take longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,

    /// If the value is true, results are returned in descending order. If the value is to
    /// false, results are returned in ascending order. The default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descending: Option<bool>,

    /// The token for the next set of items to return. (You received this token from a previous
    /// call.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items returned. If you don't specify a value, the default is up to
    /// 50 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl DescribeLogStreamsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            log_group_name: Some("test-log_group_name".into()),
            log_stream_name_prefix: Some("test-log_stream_name_prefix".into()),
            order_by: Some("test-order_by".into()),
            descending: Some(false),
            next_token: Some("test-next_token".into()),
            limit: Some(100),
        }
    }
}

///
/// **AWS API**: `logs.v1.DescribeLogStreamsResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//DescribeLogStreamsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeLogStreamsResponse {
    /// The log streams.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub log_streams: Vec<LogStream>,

    /// The `nextToken` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeLogStreamsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            log_streams: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Represents a log stream, which is a sequence of log events from a single emitter of logs.
///
/// **AWS API**: `logs.v1.LogStream`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//LogStream>
///
/// ## Coverage
/// 6 of 8 fields included.
/// Omitted fields:
/// - `uploadSequenceToken` — not selected in manifest
/// - `storedBytes` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogStream {
    /// The name of the log stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,

    /// The creation time of the stream, expressed as the number of milliseconds after Jan 1,
    /// 1970 00:00:00 UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,

    /// The time of the first event, expressed as the number of milliseconds after Jan 1, 1970
    /// 00:00:00 UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_event_timestamp: Option<i64>,

    /// The time of the most recent log event in the log stream in CloudWatch Logs. This number
    /// is expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. The
    /// lastEventTime value updates on an eventual consistency basis. It typically updates in
    /// less than an hour from ingestion, but in rare situations might take longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_timestamp: Option<i64>,

    /// The ingestion time, expressed as the number of milliseconds after Jan 1, 1970 00:00:00
    /// UTC The lastIngestionTime value updates on an eventual consistency basis. It typically
    /// updates in less than an hour after ingestion, but in rare situations might take longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ingestion_time: Option<i64>,

    /// The Amazon Resource Name (ARN) of the log stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

impl LogStream {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            log_stream_name: Some("test-log_stream_name".into()),
            creation_time: Some(100),
            first_event_timestamp: Some(100),
            last_event_timestamp: Some(100),
            last_ingestion_time: Some(100),
            arn: Some("test-arn".into()),
        }
    }
}

///
/// **AWS API**: `logs.v1.PutRetentionPolicyRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//PutRetentionPolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutRetentionPolicyRequest {
    /// The name of the log group.
    pub log_group_name: String,

    /// The `retentionInDays` field.
    pub retention_in_days: i32,
}

impl PutRetentionPolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            log_group_name: "test-log_group_name".into(),
            retention_in_days: 100,
        }
    }
}

///
/// **AWS API**: `logs.v1.DeleteLogStreamRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//DeleteLogStreamRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteLogStreamRequest {
    /// The name of the log group.
    pub log_group_name: String,

    /// The name of the log stream.
    pub log_stream_name: String,
}

impl DeleteLogStreamRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            log_group_name: "test-log_group_name".into(),
            log_stream_name: "test-log_stream_name".into(),
        }
    }
}

///
/// **AWS API**: `logs.v1.DescribeMetricFiltersResponse`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//DescribeMetricFiltersResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeMetricFiltersResponse {
    /// The metric filters.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metric_filters: Vec<MetricFilter>,

    /// The `nextToken` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeMetricFiltersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metric_filters: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Metric filters express how CloudWatch Logs would extract metric observations from ingested
/// log events and transform them into metric data in a CloudWatch metric.
///
/// **AWS API**: `logs.v1.MetricFilter`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//MetricFilter>
///
/// ## Coverage
/// 5 of 8 fields included.
/// Omitted fields:
/// - `applyOnTransformedLogs` — not selected in manifest
/// - `fieldSelectionCriteria` — not selected in manifest
/// - `emitSystemFieldDimensions` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricFilter {
    /// The name of the metric filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<String>,

    /// The `filterPattern` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,

    /// The metric transformations.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metric_transformations: Vec<MetricTransformation>,

    /// The name of the log group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,

    /// The creation time of the metric filter, expressed as the number of milliseconds after
    /// Jan 1, 1970 00:00:00 UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
}

impl MetricFilter {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            filter_name: Some("test-filter_name".into()),
            filter_pattern: Some("test-filter_pattern".into()),
            metric_transformations: vec![],
            log_group_name: Some("test-log_group_name".into()),
            creation_time: Some(100),
        }
    }
}

/// Indicates how to transform ingested log events to metric data in a CloudWatch metric.
///
/// **AWS API**: `logs.v1.MetricTransformation`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//MetricTransformation>
///
/// ## Coverage
/// 3 of 6 fields included.
/// Omitted fields:
/// - `defaultValue` — not selected in manifest
/// - `dimensions` — not selected in manifest
/// - `unit` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricTransformation {
    /// The name of the CloudWatch metric.
    pub metric_name: String,

    /// A custom namespace to contain your metric in CloudWatch. Use namespaces to group
    /// together metrics that are similar. For more information, see Namespaces.
    pub metric_namespace: String,

    /// The value to publish to the CloudWatch metric when a filter pattern matches a log event.
    pub metric_value: String,
}

impl MetricTransformation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metric_name: "test-metric_name".into(),
            metric_namespace: "test-metric_namespace".into(),
            metric_value: "test-metric_value".into(),
        }
    }
}

///
/// **AWS API**: `logs.v1.DescribeLogGroupsRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//DescribeLogGroupsRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeLogGroupsRequest {
    /// When includeLinkedAccounts is set to true, use this parameter to specify the list of
    /// accounts to search. You can specify as many as 20 account IDs in the array.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub account_identifiers: Vec<String>,

    /// The prefix to match. logGroupNamePrefix and logGroupNamePattern are mutually exclusive.
    /// Only one of these parameters can be passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name_prefix: Option<String>,

    /// If you specify a string for this parameter, the operation returns only log groups that
    /// have names that match the string based on a case-sensitive substring search. For
    /// example, if you specify DataLogs, log groups named DataLogs, aws/DataLogs, and
    /// GroupDataLogs would match, but datalogs, Data/log/s and Groupdata would not match. If
    /// you specify logGroupNamePattern in your request, then only arn, creationTime, and
    /// logGroupName are included in the response. logGroupNamePattern and logGroupNamePrefix
    /// are mutually exclusive. Only one of these parameters can be passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name_pattern: Option<String>,

    /// The token for the next set of items to return. (You received this token from a previous
    /// call.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items returned. If you don't specify a value, the default is up to
    /// 50 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// If you are using a monitoring account, set this to true to have the operation return log
    /// groups in the accounts listed in accountIdentifiers. If this parameter is set to true
    /// and accountIdentifiers contains a null value, the operation returns all log groups in
    /// the monitoring account and all log groups in all source accounts that are linked to the
    /// monitoring account. The default for this parameter is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_linked_accounts: Option<bool>,

    /// Use this parameter to limit the results to only those log groups in the specified log
    /// group class. If you omit this parameter, log groups of all classes can be returned.
    /// Specifies the log group class for this log group. There are three classes: The Standard
    /// log class supports all CloudWatch Logs features. The Infrequent Access log class
    /// supports a subset of CloudWatch Logs features and incurs lower costs. Use the Delivery
    /// log class only for delivering Lambda logs to store in Amazon S3 or Amazon Data Firehose.
    /// Log events in log groups in the Delivery class are kept in CloudWatch Logs for only one
    /// day. This log class doesn't offer rich CloudWatch Logs capabilities such as CloudWatch
    /// Logs Insights queries. For details about the features supported by each class, see Log
    /// classes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_class: Option<String>,

    /// Use this array to filter the list of log groups returned. If you specify this parameter,
    /// the only other filter that you can choose to specify is includeLinkedAccounts. If you
    /// are using this operation in a monitoring account, you can specify the ARNs of log groups
    /// in source accounts and in the monitoring account itself. If you are using this operation
    /// in an account that is not a cross-account monitoring account, you can specify only log
    /// group names in the same account as the operation.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub log_group_identifiers: Vec<String>,
}

impl DescribeLogGroupsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            account_identifiers: vec![],
            log_group_name_prefix: Some("test-log_group_name_prefix".into()),
            log_group_name_pattern: Some("test-log_group_name_pattern".into()),
            next_token: Some("test-next_token".into()),
            limit: Some(100),
            include_linked_accounts: Some(false),
            log_group_class: Some("test-log_group_class".into()),
            log_group_identifiers: vec![],
        }
    }
}

///
/// **AWS API**: `logs.v1.ListTagsForResourceRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//ListTagsForResourceRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTagsForResourceRequest {
    /// The ARN of the resource that you want to view tags for. The ARN format of a log group is
    /// arn:aws:logs:Region:account-id:log-group:log-group-name The ARN format of a destination
    /// is arn:aws:logs:Region:account-id:destination:destination-name For more information
    /// about ARN format, see CloudWatch Logs resources and operations.
    pub resource_arn: String,
}

impl ListTagsForResourceRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource_arn: "test-resource_arn".into(),
        }
    }
}

///
/// **AWS API**: `logs.v1.DescribeMetricFiltersRequest`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference//DescribeMetricFiltersRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeMetricFiltersRequest {
    /// The name of the log group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,

    /// The prefix to match. CloudWatch Logs uses the value that you set here only if you also
    /// include the logGroupName parameter in your request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name_prefix: Option<String>,

    /// The token for the next set of items to return. (You received this token from a previous
    /// call.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items returned. If you don't specify a value, the default is up to
    /// 50 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Filters results to include only those with the specified metric name. If you include
    /// this parameter in your request, you must also include the metricNamespace parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,

    /// Filters results to include only those in the specified namespace. If you include this
    /// parameter in your request, you must also include the metricName parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
}

impl DescribeMetricFiltersRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            log_group_name: Some("test-log_group_name".into()),
            filter_name_prefix: Some("test-filter_name_prefix".into()),
            next_token: Some("test-next_token".into()),
            limit: Some(100),
            metric_name: Some("test-metric_name".into()),
            metric_namespace: Some("test-metric_namespace".into()),
        }
    }
}
