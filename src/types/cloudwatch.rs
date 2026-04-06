//! Types for the Amazon CloudWatch API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Possible values for `cloudwatch.Datapoint.Unit`.
///
/// **AWS API**: `cloudwatch.Datapoint.Unit`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardUnit {
    #[serde(rename = "Seconds")]
    Seconds,

    #[serde(rename = "Microseconds")]
    Microseconds,

    #[serde(rename = "Milliseconds")]
    Milliseconds,

    #[serde(rename = "Bytes")]
    Bytes,

    #[serde(rename = "Kilobytes")]
    Kilobytes,

    #[serde(rename = "Megabytes")]
    Megabytes,

    #[serde(rename = "Gigabytes")]
    Gigabytes,

    #[serde(rename = "Terabytes")]
    Terabytes,

    #[serde(rename = "Bits")]
    Bits,

    #[serde(rename = "Kilobits")]
    Kilobits,

    #[serde(rename = "Megabits")]
    Megabits,

    #[serde(rename = "Gigabits")]
    Gigabits,

    #[serde(rename = "Terabits")]
    Terabits,

    #[serde(rename = "Percent")]
    Percent,

    #[serde(rename = "Count")]
    Count,

    #[serde(rename = "Bytes/Second")]
    BytesPerSecond,

    #[serde(rename = "Kilobytes/Second")]
    KilobytesPerSecond,

    #[serde(rename = "Megabytes/Second")]
    MegabytesPerSecond,

    #[serde(rename = "Gigabytes/Second")]
    GigabytesPerSecond,

    #[serde(rename = "Terabytes/Second")]
    TerabytesPerSecond,

    #[serde(rename = "Bits/Second")]
    BitsPerSecond,

    #[serde(rename = "Kilobits/Second")]
    KilobitsPerSecond,

    #[serde(rename = "Megabits/Second")]
    MegabitsPerSecond,

    #[serde(rename = "Gigabits/Second")]
    GigabitsPerSecond,

    #[serde(rename = "Terabits/Second")]
    TerabitsPerSecond,

    #[serde(rename = "Count/Second")]
    CountPerSecond,

    #[serde(rename = "None")]
    None,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `cloudwatch.GetMetricStatisticsInput.Statistics`.
///
/// **AWS API**: `cloudwatch.GetMetricStatisticsInput.Statistics`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Statistic {
    #[serde(rename = "SampleCount")]
    SampleCount,

    #[serde(rename = "Average")]
    Average,

    #[serde(rename = "Sum")]
    Sum,

    #[serde(rename = "Minimum")]
    Minimum,

    #[serde(rename = "Maximum")]
    Maximum,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `cloudwatch.MetricAlarm.ComparisonOperator`.
///
/// **AWS API**: `cloudwatch.MetricAlarm.ComparisonOperator`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    #[serde(rename = "GreaterThanOrEqualToThreshold")]
    GreaterThanOrEqualToThreshold,

    #[serde(rename = "GreaterThanThreshold")]
    GreaterThanThreshold,

    #[serde(rename = "LessThanThreshold")]
    LessThanThreshold,

    #[serde(rename = "LessThanOrEqualToThreshold")]
    LessThanOrEqualToThreshold,

    #[serde(rename = "LessThanLowerOrGreaterThanUpperThreshold")]
    LessThanLowerOrGreaterThanUpperThreshold,

    #[serde(rename = "LessThanLowerThreshold")]
    LessThanLowerThreshold,

    #[serde(rename = "GreaterThanUpperThreshold")]
    GreaterThanUpperThreshold,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `cloudwatch.GetMetricDataInput.ScanBy`.
///
/// **AWS API**: `cloudwatch.GetMetricDataInput.ScanBy`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanBy {
    #[serde(rename = "TimestampDescending")]
    TimestampDescending,

    #[serde(rename = "TimestampAscending")]
    TimestampAscending,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `cloudwatch.MetricDataResult.StatusCode`.
///
/// **AWS API**: `cloudwatch.MetricDataResult.StatusCode`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusCode {
    #[serde(rename = "Complete")]
    Complete,

    #[serde(rename = "InternalError")]
    InternalError,

    #[serde(rename = "PartialData")]
    PartialData,

    #[serde(rename = "Forbidden")]
    Forbidden,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

///
/// **AWS API**: `cloudwatch.v1.GetMetricStatisticsOutput`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//GetMetricStatisticsOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetMetricStatisticsResponse {
    /// A label for the specified metric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The data points for the specified metric.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub datapoints: Vec<Datapoint>,
}

impl GetMetricStatisticsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            label: Some("test-label".into()),
            datapoints: vec![],
        }
    }
}

/// Encapsulates the statistical data that CloudWatch computes from metric data.
///
/// **AWS API**: `cloudwatch.v1.Datapoint`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//Datapoint>
///
/// ## Coverage
/// 7 of 8 fields included.
/// Omitted fields:
/// - `ExtendedStatistics` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Datapoint {
    /// The time stamp used for the data point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    /// The average of the metric values that correspond to the data point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,

    /// The sum of the metric values for the data point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,

    /// The minimum metric value for the data point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    /// The maximum metric value for the data point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

    /// The number of metric values that contributed to the aggregate value of this data point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<f64>,

    /// The standard unit for the data point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<StandardUnit>,
}

impl Datapoint {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            timestamp: Some("test-timestamp".into()),
            ..Default::default()
        }
    }
}

/// A dimension is a name/value pair that is part of the identity of a metric. Because
/// dimensions are part of the unique identifier for a metric, whenever you add a unique
/// name/value pair to one of your metrics, you are creating a new variation of that metric. For
/// example, many Amazon EC2 metrics publish InstanceId as a dimension name, and the actual
/// instance ID as the value for that dimension. You can assign up to 30 dimensions to a metric.
///
/// **AWS API**: `cloudwatch.v1.Dimension`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//Dimension>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dimension {
    /// The name of the dimension. Dimension names must contain only ASCII characters, must
    /// include at least one non-whitespace character, and cannot start with a colon (:). ASCII
    /// control characters are not supported as part of dimension names.
    pub name: String,

    /// The value of the dimension. Dimension values must contain only ASCII characters and must
    /// include at least one non-whitespace character. ASCII control characters are not
    /// supported as part of dimension values.
    pub value: String,
}

impl Dimension {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-name".into(),
            value: "test-value".into(),
        }
    }
}

///
/// **AWS API**: `cloudwatch.v1.GetMetricStatisticsInput`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//GetMetricStatisticsInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetMetricStatisticsInput {
    /// The namespace of the metric, with or without spaces.
    pub namespace: String,

    /// The name of the metric, with or without spaces.
    pub metric_name: String,

    /// The dimensions. If the metric contains multiple dimensions, you must include a value for
    /// each dimension. CloudWatch treats each unique combination of dimensions as a separate
    /// metric. If a specific combination of dimensions was not published, you can't retrieve
    /// statistics for it. You must specify the same dimensions that were used when the metrics
    /// were created. For an example, see Dimension Combinations in the Amazon CloudWatch User
    /// Guide. For more information about specifying dimensions, see Publishing Metrics in the
    /// Amazon CloudWatch User Guide.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,

    /// The time stamp that determines the first data point to return. Start times are evaluated
    /// relative to the time that CloudWatch receives the request. The value specified is
    /// inclusive; results include data points with the specified time stamp. In a raw HTTP
    /// query, the time stamp must be in ISO 8601 UTC format (for example,
    /// 2016-10-03T23:00:00Z). CloudWatch rounds the specified time stamp as follows: Start time
    /// less than 15 days ago
    /// - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to
    ///   12:32:00. Start time between 15 and 63 days ago
    /// - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded
    ///   down to 12:30:00. Start time greater than 63 days ago
    /// - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down
    ///   to 12:00:00. If you set Period to 5, 10, 20, or 30, the start time of your request is
    ///   rounded down to the nearest time that corresponds to even 5-, 10-, 20-, or 30-second
    ///   divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the
    ///   previous 10-second period, the start time of your request is rounded down and you
    ///   receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the
    ///   previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped
    ///   between 15:02:15 and 15:07:15.
    pub start_time: String,

    /// The time stamp that determines the last data point to return. The value specified is
    /// exclusive; results include data points up to the specified time stamp. In a raw HTTP
    /// query, the time stamp must be in ISO 8601 UTC format (for example,
    /// 2016-10-10T23:00:00Z).
    pub end_time: String,

    /// The granularity, in seconds, of the returned data points. For metrics with regular
    /// resolution, a period can be as short as one minute (60 seconds) and must be a multiple
    /// of 60. For high-resolution metrics that are collected at intervals of less than one
    /// minute, the period can be 1, 5, 10, 20, 30, 60, or any multiple of 60. High-resolution
    /// metrics are those metrics stored by a PutMetricData call that includes a
    /// StorageResolution of 1 second. If the StartTime parameter specifies a time stamp that is
    /// greater than 3 hours ago, you must specify the period as follows or no data points in
    /// that time range is returned: Start time between 3 hours and 15 days ago
    /// - Use a multiple of 60 seconds (1 minute). Start time between 15 and 63 days ago
    /// - Use a multiple of 300 seconds (5 minutes). Start time greater than 63 days ago
    /// - Use a multiple of 3600 seconds (1 hour).
    pub period: i32,

    /// The metric statistics, other than percentile. For percentile statistics, use
    /// ExtendedStatistics. When calling GetMetricStatistics, you must specify either Statistics
    /// or ExtendedStatistics, but not both.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub statistics: Vec<Statistic>,

    /// The percentile statistics. Specify values between p0.0 and p100. When calling
    /// GetMetricStatistics, you must specify either Statistics or ExtendedStatistics, but not
    /// both. Percentile statistics are not available for metrics when any of the metric values
    /// are negative numbers.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extended_statistics: Vec<String>,

    /// The unit for a given metric. If you omit Unit, all data that was collected with any unit
    /// is returned, along with the corresponding units that were specified when the data was
    /// reported to CloudWatch. If you specify a unit, the operation returns only data that was
    /// collected with that unit specified. If you specify a unit that does not match the data
    /// collected, the results of the operation are null. CloudWatch does not perform unit
    /// conversions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<StandardUnit>,
}

impl GetMetricStatisticsInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            namespace: "test-namespace".into(),
            metric_name: "test-metric_name".into(),
            dimensions: vec![],
            start_time: "test-start_time".into(),
            end_time: "test-end_time".into(),
            period: 100,
            statistics: vec![],
            extended_statistics: vec![],
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `cloudwatch.v1.ListMetricsInput`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//ListMetricsInput>
///
/// ## Coverage
/// 4 of 7 fields included.
/// Omitted fields:
/// - `RecentlyActive` — not selected in manifest
/// - `IncludeLinkedAccounts` — not selected in manifest
/// - `OwningAccount` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListMetricsInput {
    /// The metric namespace to filter against. Only the namespace that matches exactly will be
    /// returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The name of the metric to filter against. Only the metrics with names that match exactly
    /// will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,

    /// The dimensions to filter against. Only the dimension with names that match exactly will
    /// be returned. If you specify one dimension name and a metric has that dimension and also
    /// other dimensions, it will be returned.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<DimensionFilter>,

    /// The token returned by a previous call to indicate that there is more data available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListMetricsInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            namespace: Some("test-namespace".into()),
            metric_name: Some("test-metric_name".into()),
            dimensions: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

///
/// **AWS API**: `cloudwatch.v1.ListMetricsOutput`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//ListMetricsOutput>
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `OwningAccounts` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListMetricsResponse {
    /// The metrics that match your request.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<Metric>,

    /// The token that marks the start of the next batch of returned results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListMetricsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metrics: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Represents a specific metric.
///
/// **AWS API**: `cloudwatch.v1.Metric`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//Metric>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metric {
    /// The namespace of the metric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The name of the metric. This is a required field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,

    /// The dimensions for the metric.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
}

impl Metric {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            namespace: Some("test-namespace".into()),
            metric_name: Some("test-metric_name".into()),
            dimensions: vec![],
        }
    }
}

/// Represents filters for a dimension.
///
/// **AWS API**: `cloudwatch.v1.DimensionFilter`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//DimensionFilter>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DimensionFilter {
    /// The dimension name to be matched.
    pub name: String,

    /// The value of the dimension to be matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DimensionFilter {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-name".into(),
            value: Some("test-value".into()),
        }
    }
}

///
/// **AWS API**: `cloudwatch.v1.DescribeAlarmsInput`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//DescribeAlarmsInput>
///
/// ## Coverage
/// 5 of 9 fields included.
/// Omitted fields:
/// - `AlarmTypes` — not selected in manifest
/// - `ChildrenOfAlarmName` — not selected in manifest
/// - `ParentsOfAlarmName` — not selected in manifest
/// - `ActionPrefix` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeAlarmsInput {
    /// The names of the alarms to retrieve information about.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alarm_names: Vec<String>,

    /// An alarm name prefix. If you specify this parameter, you receive information about all
    /// alarms that have names that start with this prefix. If this parameter is specified, you
    /// cannot specify AlarmNames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name_prefix: Option<String>,

    /// Specify this parameter to receive information only about alarms that are currently in
    /// the state that you specify.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_value: Option<String>,

    /// The maximum number of alarm descriptions to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,

    /// The token returned by a previous call to indicate that there is more data available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeAlarmsInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            alarm_names: vec![],
            alarm_name_prefix: Some("test-alarm_name_prefix".into()),
            state_value: Some("test-state_value".into()),
            max_records: Some(100),
            next_token: Some("test-next_token".into()),
        }
    }
}

///
/// **AWS API**: `cloudwatch.v1.DescribeAlarmsOutput`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//DescribeAlarmsOutput>
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `CompositeAlarms` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeAlarmsResponse {
    /// The information about any metric alarms returned by the operation.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metric_alarms: Vec<MetricAlarm>,

    /// The token that marks the start of the next batch of returned results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeAlarmsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metric_alarms: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// The details about a metric alarm.
///
/// **AWS API**: `cloudwatch.v1.MetricAlarm`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//MetricAlarm>
///
/// ## Coverage
/// 19 of 29 fields included.
/// Omitted fields:
/// - `AlarmConfigurationUpdatedTimestamp` — not selected in manifest
/// - `StateReasonData` — not selected in manifest
/// - `StateUpdatedTimestamp` — not selected in manifest
/// - `ExtendedStatistic` — not selected in manifest
/// - `Unit` — not selected in manifest
/// - `EvaluateLowSampleCountPercentile` — not selected in manifest
/// - `Metrics` — not selected in manifest
/// - `ThresholdMetricId` — not selected in manifest
/// - `EvaluationState` — not selected in manifest
/// - `StateTransitionedTimestamp` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetricAlarm {
    /// The name of the alarm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<String>,

    /// The Amazon Resource Name (ARN) of the alarm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_arn: Option<String>,

    /// The description of the alarm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_description: Option<String>,

    /// Indicates whether actions should be executed during any changes to the alarm state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_enabled: Option<bool>,

    /// The actions to execute when this alarm transitions to the OK state from any other state.
    /// Each action is specified as an Amazon Resource Name (ARN).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ok_actions: Vec<String>,

    /// The actions to execute when this alarm transitions to the ALARM state from any other
    /// state. Each action is specified as an Amazon Resource Name (ARN).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alarm_actions: Vec<String>,

    /// The actions to execute when this alarm transitions to the INSUFFICIENT_DATA state from
    /// any other state. Each action is specified as an Amazon Resource Name (ARN).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub insufficient_data_actions: Vec<String>,

    /// The state value for the alarm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_value: Option<String>,

    /// An explanation for the alarm state, in text format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,

    /// The name of the metric associated with the alarm, if this is an alarm based on a single
    /// metric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,

    /// The namespace of the metric associated with the alarm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The statistic for the metric associated with the alarm, other than percentile. For
    /// percentile statistics, use ExtendedStatistic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<Statistic>,

    /// The dimensions for the metric associated with the alarm.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,

    /// The period, in seconds, over which the statistic is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,

    /// The number of periods over which data is compared to the specified threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i32>,

    /// The number of data points that must be breaching to trigger the alarm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,

    /// The value to compare with the specified statistic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,

    /// The arithmetic operation to use when comparing the specified statistic and threshold.
    /// The specified statistic value is used as the first operand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<ComparisonOperator>,

    /// Sets how this alarm is to handle missing data points. The valid values are breaching,
    /// notBreaching, ignore, and missing. For more information, see Configuring how CloudWatch
    /// alarms treat missing data. If this parameter is omitted, the default behavior of missing
    /// is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
}

impl MetricAlarm {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            alarm_name: Some("test-alarm_name".into()),
            alarm_arn: Some("test-alarm_arn".into()),
            alarm_description: Some("test-alarm_description".into()),
            actions_enabled: Some(false),
            ok_actions: vec![],
            alarm_actions: vec![],
            insufficient_data_actions: vec![],
            state_value: Some("test-state_value".into()),
            state_reason: Some("test-state_reason".into()),
            metric_name: Some("test-metric_name".into()),
            namespace: Some("test-namespace".into()),
            dimensions: vec![],
            period: Some(100),
            evaluation_periods: Some(100),
            datapoints_to_alarm: Some(100),
            treat_missing_data: Some("test-treat_missing_data".into()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `cloudwatch.v1.PutMetricAlarmInput`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//PutMetricAlarmInput>
///
/// ## Coverage
/// 17 of 22 fields included.
/// Omitted fields:
/// - `ExtendedStatistic` — not selected in manifest
/// - `EvaluateLowSampleCountPercentile` — not selected in manifest
/// - `Metrics` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `ThresholdMetricId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PutMetricAlarmInput {
    /// The name for the alarm. This name must be unique within the Region. The name must
    /// contain only UTF-8 characters, and can't contain ASCII control characters
    pub alarm_name: String,

    /// The description for the alarm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_description: Option<String>,

    /// Indicates whether actions should be executed during any changes to the alarm state. The
    /// default is TRUE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_enabled: Option<bool>,

    /// The actions to execute when this alarm transitions to an OK state from any other state.
    /// Each action is specified as an Amazon Resource Name (ARN). Valid values: EC2 actions:
    /// arn:aws:automate:region:ec2:stop arn:aws:automate:region:ec2:terminate
    /// arn:aws:automate:region:ec2:reboot arn:aws:automate:region:ec2:recover
    /// arn:aws:swf:region:account-id:action/actions/AWS_EC2.InstanceId.Stop/1.0
    /// arn:aws:swf:region:account-id:action/actions/AWS_EC2.InstanceId.Terminate/1.0
    /// arn:aws:swf:region:account-id:action/actions/AWS_EC2.InstanceId.Reboot/1.0
    /// arn:aws:swf:region:account-id:action/actions/AWS_EC2.InstanceId.Recover/1.0 Autoscaling
    /// action: arn:aws:autoscaling:region:account-id:scalingPolicy:policy-
    /// id:autoScalingGroupName/group-friendly-name:policyName/policy-friendly-name Lambda
    /// actions: Invoke the latest version of a Lambda function: arn:aws:lambda:region:account-
    /// id:function:function-name Invoke a specific version of a Lambda function:
    /// arn:aws:lambda:region:account-id:function:function-name:version-number Invoke a function
    /// by using an alias Lambda function: arn:aws:lambda:region:account-id:function:function-
    /// name:alias-name SNS notification action: arn:aws:sns:region:account-id:sns-topic-name
    /// SSM integration actions: arn:aws:ssm:region:account-
    /// id:opsitem:severity#CATEGORY=category-name arn:aws:ssm-incidents::account-
    /// id:responseplan/response-plan-name
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ok_actions: Vec<String>,

    /// The actions to execute when this alarm transitions to the ALARM state from any other
    /// state. Each action is specified as an Amazon Resource Name (ARN). Valid values: EC2
    /// actions: arn:aws:automate:region:ec2:stop arn:aws:automate:region:ec2:terminate
    /// arn:aws:automate:region:ec2:reboot arn:aws:automate:region:ec2:recover
    /// arn:aws:swf:region:account-id:action/actions/AWS_EC2.InstanceId.Stop/1.0
    /// arn:aws:swf:region:account-id:action/actions/AWS_EC2.InstanceId.Terminate/1.0
    /// arn:aws:swf:region:account-id:action/actions/AWS_EC2.InstanceId.Reboot/1.0
    /// arn:aws:swf:region:account-id:action/actions/AWS_EC2.InstanceId.Recover/1.0 Autoscaling
    /// action: arn:aws:autoscaling:region:account-id:scalingPolicy:policy-
    /// id:autoScalingGroupName/group-friendly-name:policyName/policy-friendly-name Lambda
    /// actions: Invoke the latest version of a Lambda function: arn:aws:lambda:region:account-
    /// id:function:function-name Invoke a specific version of a Lambda function:
    /// arn:aws:lambda:region:account-id:function:function-name:version-number Invoke a function
    /// by using an alias Lambda function: arn:aws:lambda:region:account-id:function:function-
    /// name:alias-name SNS notification action: arn:aws:sns:region:account-id:sns-topic-name
    /// SSM integration actions: arn:aws:ssm:region:account-
    /// id:opsitem:severity#CATEGORY=category-name arn:aws:ssm-incidents::account-
    /// id:responseplan/response-plan-name Start a Amazon Q Developer operational investigation
    /// arn:aws:aiops:region:account-id:investigation-group:investigation-group-id
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alarm_actions: Vec<String>,

    /// The actions to execute when this alarm transitions to the INSUFFICIENT_DATA state from
    /// any other state. Each action is specified as an Amazon Resource Name (ARN). Valid
    /// values: EC2 actions: arn:aws:automate:region:ec2:stop
    /// arn:aws:automate:region:ec2:terminate arn:aws:automate:region:ec2:reboot
    /// arn:aws:automate:region:ec2:recover arn:aws:swf:region:account-
    /// id:action/actions/AWS_EC2.InstanceId.Stop/1.0 arn:aws:swf:region:account-
    /// id:action/actions/AWS_EC2.InstanceId.Terminate/1.0 arn:aws:swf:region:account-
    /// id:action/actions/AWS_EC2.InstanceId.Reboot/1.0 arn:aws:swf:region:account-
    /// id:action/actions/AWS_EC2.InstanceId.Recover/1.0 Autoscaling action:
    /// arn:aws:autoscaling:region:account-id:scalingPolicy:policy-
    /// id:autoScalingGroupName/group-friendly-name:policyName/policy-friendly-name Lambda
    /// actions: Invoke the latest version of a Lambda function: arn:aws:lambda:region:account-
    /// id:function:function-name Invoke a specific version of a Lambda function:
    /// arn:aws:lambda:region:account-id:function:function-name:version-number Invoke a function
    /// by using an alias Lambda function: arn:aws:lambda:region:account-id:function:function-
    /// name:alias-name SNS notification action: arn:aws:sns:region:account-id:sns-topic-name
    /// SSM integration actions: arn:aws:ssm:region:account-
    /// id:opsitem:severity#CATEGORY=category-name arn:aws:ssm-incidents::account-
    /// id:responseplan/response-plan-name
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub insufficient_data_actions: Vec<String>,

    /// The name for the metric associated with the alarm. For each PutMetricAlarm operation,
    /// you must specify either MetricName or a Metrics array. If you are creating an alarm
    /// based on a math expression, you cannot specify this parameter, or any of the Namespace,
    /// Dimensions, Period, Unit, Statistic, or ExtendedStatistic parameters. Instead, you
    /// specify all this information in the Metrics array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,

    /// The namespace for the metric associated specified in MetricName.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The statistic for the metric specified in MetricName, other than percentile. For
    /// percentile statistics, use ExtendedStatistic. When you call PutMetricAlarm and specify a
    /// MetricName, you must specify either Statistic or ExtendedStatistic, but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<Statistic>,

    /// The dimensions for the metric specified in MetricName.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,

    /// The length, in seconds, used each time the metric specified in MetricName is evaluated.
    /// Valid values are 10, 20, 30, and any multiple of 60. Period is required for alarms based
    /// on static thresholds. If you are creating an alarm based on a metric math expression,
    /// you specify the period for each metric within the objects in the Metrics array. Be sure
    /// to specify 10, 20, or 30 only for metrics that are stored by a PutMetricData call with a
    /// StorageResolution of 1. If you specify a period of 10, 20, or 30 for a metric that does
    /// not have sub-minute resolution, the alarm still attempts to gather data at the period
    /// rate that you specify. In this case, it does not receive data for the attempts that do
    /// not correspond to a one-minute data resolution, and the alarm might often lapse into
    /// INSUFFICENT_DATA status. Specifying 10, 20, or 30 also sets this alarm as a high-
    /// resolution alarm, which has a higher charge than other alarms. For more information
    /// about pricing, see Amazon CloudWatch Pricing. An alarm's total current evaluation period
    /// can be no longer than seven days, so Period multiplied by EvaluationPeriods can't be
    /// more than 604,800 seconds. For alarms with a period of less than one hour (3,600
    /// seconds), the total evaluation period can't be longer than one day (86,400 seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,

    /// The unit of measure for the statistic. For example, the units for the Amazon EC2
    /// NetworkIn metric are Bytes because NetworkIn tracks the number of bytes that an instance
    /// receives on all network interfaces. You can also specify a unit when you create a custom
    /// metric. Units help provide conceptual meaning to your data. Metric data points that
    /// specify a unit of measure, such as Percent, are aggregated separately. If you are
    /// creating an alarm based on a metric math expression, you can specify the unit for each
    /// metric (if needed) within the objects in the Metrics array. If you don't specify Unit,
    /// CloudWatch retrieves all unit types that have been published for the metric and attempts
    /// to evaluate the alarm. Usually, metrics are published with only one unit, so the alarm
    /// works as intended. However, if the metric is published with multiple types of units and
    /// you don't specify a unit, the alarm's behavior is not defined and it behaves
    /// unpredictably. We recommend omitting Unit so that you don't inadvertently specify an
    /// incorrect unit that is not published for this metric. Doing so causes the alarm to be
    /// stuck in the INSUFFICIENT DATA state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<StandardUnit>,

    /// The number of periods over which data is compared to the specified threshold. If you are
    /// setting an alarm that requires that a number of consecutive data points be breaching to
    /// trigger the alarm, this value specifies that number. If you are setting an "M out of N"
    /// alarm, this value is the N.
    pub evaluation_periods: i32,

    /// The number of data points that must be breaching to trigger the alarm. This is used only
    /// if you are setting an "M out of N" alarm. In that case, this value is the M. For more
    /// information, see Evaluating an Alarm in the Amazon CloudWatch User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,

    /// The value against which the specified statistic is compared. This parameter is required
    /// for alarms based on static thresholds, but should not be used for alarms based on
    /// anomaly detection models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,

    /// The arithmetic operation to use when comparing the specified statistic and threshold.
    /// The specified statistic value is used as the first operand. The values
    /// LessThanLowerOrGreaterThanUpperThreshold, LessThanLowerThreshold, and
    /// GreaterThanUpperThreshold are used only for alarms based on anomaly detection models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<ComparisonOperator>,

    /// Sets how this alarm is to handle missing data points. If TreatMissingData is omitted,
    /// the default behavior of missing is used. For more information, see Configuring How
    /// CloudWatch Alarms Treats Missing Data. Valid Values: breaching | notBreaching | ignore |
    /// missing Alarms that evaluate metrics in the AWS/DynamoDB namespace always ignore missing
    /// data even if you choose a different option for TreatMissingData. When an AWS/DynamoDB
    /// metric has missing data, alarms that evaluate that metric remain in their current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
}

impl PutMetricAlarmInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            alarm_name: "test-alarm_name".into(),
            alarm_description: Some("test-alarm_description".into()),
            actions_enabled: Some(false),
            ok_actions: vec![],
            alarm_actions: vec![],
            insufficient_data_actions: vec![],
            metric_name: Some("test-metric_name".into()),
            namespace: Some("test-namespace".into()),
            dimensions: vec![],
            period: Some(100),
            evaluation_periods: 100,
            datapoints_to_alarm: Some(100),
            treat_missing_data: Some("test-treat_missing_data".into()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `cloudwatch.v1.DeleteAlarmsInput`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//DeleteAlarmsInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteAlarmsInput {
    /// The alarms to be deleted. Do not enclose the alarm names in quote marks.
    #[serde(default)]
    pub alarm_names: Vec<String>,
}

impl DeleteAlarmsInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            alarm_names: vec![],
        }
    }
}

///
/// **AWS API**: `cloudwatch.v1.GetMetricDataInput`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//GetMetricDataInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetMetricDataInput {
    /// The metric queries to be returned. A single GetMetricData call can include as many as
    /// 500 MetricDataQuery structures. Each of these structures can specify either a metric to
    /// retrieve, a Metrics Insights query, or a math expression to perform on retrieved data.
    #[serde(default)]
    pub metric_data_queries: Vec<MetricDataQuery>,

    /// The time stamp indicating the earliest data to be returned. The value specified is
    /// inclusive; results include data points with the specified time stamp. CloudWatch rounds
    /// the specified time stamp as follows: Start time less than 15 days ago
    /// - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to
    ///   12:32:00. Start time between 15 and 63 days ago
    /// - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded
    ///   down to 12:30:00. Start time greater than 63 days ago
    /// - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down
    ///   to 12:00:00. If you set Period to 5, 10, 20, or 30, the start time of your request is
    ///   rounded down to the nearest time that corresponds to even 5-, 10-, 20-, or 30-second
    ///   divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the
    ///   previous 10-second period, the start time of your request is rounded down and you
    ///   receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the
    ///   previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped
    ///   between 15:02:15 and 15:07:15. For better performance, specify StartTime and EndTime
    ///   values that align with the value of the metric's Period and sync up with the beginning
    ///   and end of an hour. For example, if the Period of a metric is 5 minutes, specifying
    ///   12:05 or 12:30 as StartTime can get a faster response from CloudWatch than setting
    ///   12:07 or 12:29 as the StartTime.
    pub start_time: String,

    /// The time stamp indicating the latest data to be returned. The value specified is
    /// exclusive; results include data points up to the specified time stamp. For better
    /// performance, specify StartTime and EndTime values that align with the value of the
    /// metric's Period and sync up with the beginning and end of an hour. For example, if the
    /// Period of a metric is 5 minutes, specifying 12:05 or 12:30 as EndTime can get a faster
    /// response from CloudWatch than setting 12:07 or 12:29 as the EndTime.
    pub end_time: String,

    /// Include this value, if it was returned by the previous GetMetricData operation, to get
    /// the next set of data points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The order in which data points should be returned. TimestampDescending returns the
    /// newest data first and paginates when the MaxDatapoints limit is reached.
    /// TimestampAscending returns the oldest data first and paginates when the MaxDatapoints
    /// limit is reached. If you omit this parameter, the default of TimestampDescending is
    /// used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_by: Option<ScanBy>,

    /// The maximum number of data points the request should return before paginating. If you
    /// omit this, the default of 100,800 is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_datapoints: Option<i32>,

    /// This structure includes the Timezone parameter, which you can use to specify your time
    /// zone so that the labels of returned data display the correct time for your time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_options: Option<LabelOptions>,
}

impl GetMetricDataInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metric_data_queries: vec![],
            start_time: "test-start_time".into(),
            end_time: "test-end_time".into(),
            next_token: Some("test-next_token".into()),
            max_datapoints: Some(100),
            label_options: Some(LabelOptions::fixture()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `cloudwatch.v1.GetMetricDataOutput`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//GetMetricDataOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetMetricDataResponse {
    /// The metrics that are returned, including the metric name, namespace, and dimensions.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metric_data_results: Vec<MetricDataResult>,

    /// A token that marks the next batch of returned results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// Contains a message about this GetMetricData operation, if the operation results in such
    /// a message. An example of a message that might be returned is Maximum number of allowed
    /// metrics exceeded. If there is a message, as much of the operation as possible is still
    /// executed. A message appears here only if it is related to the global GetMetricData
    /// operation. Any message about a specific metric returned by the operation appears in the
    /// MetricDataResult object returned for that metric.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<MessageData>,
}

impl GetMetricDataResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metric_data_results: vec![],
            next_token: Some("test-next_token".into()),
            messages: vec![],
        }
    }
}

/// This structure is used in both GetMetricData and PutMetricAlarm. The supported use of this
/// structure is different for those two operations. When used in GetMetricData, it indicates
/// the metric data to return, and whether this call is just retrieving a batch set of data for
/// one metric, or is performing a Metrics Insights query or a math expression. A single
/// GetMetricData call can include up to 500 MetricDataQuery structures. When used in
/// PutMetricAlarm, it enables you to create an alarm based on a metric math expression. Each
/// MetricDataQuery in the array specifies either a metric to retrieve, or a math expression to
/// be performed on retrieved metrics. A single PutMetricAlarm call can include up to 20
/// MetricDataQuery structures in the array. The 20 structures can include as many as 10
/// structures that contain a MetricStat parameter to retrieve a metric, and as many as 10
/// structures that contain the Expression parameter to perform a math expression. Of those
/// Expression structures, one must have true as the value for ReturnData. The result of this
/// expression is the value the alarm watches. Any expression used in a PutMetricAlarm operation
/// must return a single time series. For more information, see Metric Math Syntax and Functions
/// in the Amazon CloudWatch User Guide. Some of the parameters of this structure also have
/// different uses whether you are using this structure in a GetMetricData operation or a
/// PutMetricAlarm operation. These differences are explained in the following parameter list.
///
/// **AWS API**: `cloudwatch.v1.MetricDataQuery`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//MetricDataQuery>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetricDataQuery {
    /// A short name used to tie this object to the results in the response. This name must be
    /// unique within a single call to GetMetricData. If you are performing math expressions on
    /// this set of data, this name represents that data and can serve as a variable in the
    /// mathematical expression. The valid characters are letters, numbers, and underscore. The
    /// first character must be a lowercase letter.
    pub id: String,

    /// The metric to be returned, along with statistics, period, and units. Use this parameter
    /// only if this object is retrieving a metric and not performing a math expression on
    /// returned data. Within one MetricDataQuery object, you must specify either Expression or
    /// MetricStat but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_stat: Option<MetricStat>,

    /// This field can contain either a Metrics Insights query, or a metric math expression to
    /// be performed on the returned data. For more information about Metrics Insights queries,
    /// see Metrics Insights query components and syntax in the Amazon CloudWatch User Guide. A
    /// math expression can use the Id of the other metrics or queries to refer to those
    /// metrics, and can also use the Id of other expressions to use the result of those
    /// expressions. For more information about metric math expressions, see Metric Math Syntax
    /// and Functions in the Amazon CloudWatch User Guide. Within each MetricDataQuery object,
    /// you must specify either Expression or MetricStat but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// A human-readable label for this metric or expression. This is especially useful if this
    /// is an expression, so that you know what the value represents. If the metric or
    /// expression is shown in a CloudWatch dashboard widget, the label is shown. If Label is
    /// omitted, CloudWatch generates a default. You can put dynamic expressions into a label,
    /// so that it is more descriptive. For more information, see Using Dynamic Labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// When used in GetMetricData, this option indicates whether to return the timestamps and
    /// raw data values of this metric. If you are performing this call just to do math
    /// expressions and do not also need the raw data returned, you can specify false. If you
    /// omit this, the default of true is used. When used in PutMetricAlarm, specify true for
    /// the one expression result to use as the alarm. For all other metrics and expressions in
    /// the same PutMetricAlarm operation, specify ReturnData as False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<bool>,

    /// The granularity, in seconds, of the returned data points. For metrics with regular
    /// resolution, a period can be as short as one minute (60 seconds) and must be a multiple
    /// of 60. For high-resolution metrics that are collected at intervals of less than one
    /// minute, the period can be 1, 5, 10, 20, 30, 60, or any multiple of 60. High-resolution
    /// metrics are those metrics stored by a PutMetricData operation that includes a
    /// StorageResolution of 1 second.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,

    /// The ID of the account where the metrics are located. If you are performing a
    /// GetMetricData operation in a monitoring account, use this to specify which account to
    /// retrieve this metric from. If you are performing a PutMetricAlarm operation, use this to
    /// specify which account contains the metric that the alarm is watching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl MetricDataQuery {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            metric_stat: Some(MetricStat::fixture()),
            expression: Some("test-expression".into()),
            label: Some("test-label".into()),
            return_data: Some(false),
            period: Some(100),
            account_id: Some("test-account_id".into()),
        }
    }
}

/// A GetMetricData call returns an array of MetricDataResult structures. Each of these
/// structures includes the data points for that metric, along with the timestamps of those data
/// points and other identifying information.
///
/// **AWS API**: `cloudwatch.v1.MetricDataResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//MetricDataResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetricDataResult {
    /// The short name you specified to represent this metric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The human-readable label associated with the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The timestamps for the data points, formatted in Unix timestamp format. The number of
    /// timestamps always matches the number of values and the value for Timestamps[x] is
    /// Values[x].
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,

    /// The data points for the metric corresponding to Timestamps. The number of values always
    /// matches the number of timestamps and the timestamp for Values[x] is Timestamps[x].
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<f64>,

    /// The status of the returned data. Complete indicates that all data points in the
    /// requested time range were returned. PartialData means that an incomplete set of data
    /// points were returned. You can use the NextToken value that was returned and repeat your
    /// request to get more data points. NextToken is not returned if you are performing a math
    /// expression. InternalError indicates that an error occurred. Retry your request using
    /// NextToken, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<StatusCode>,

    /// A list of messages with additional information about the data returned.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<MessageData>,
}

impl MetricDataResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            label: Some("test-label".into()),
            timestamps: vec![],
            values: vec![],
            messages: vec![],
            ..Default::default()
        }
    }
}

/// This structure defines the metric to be returned, along with the statistics, period, and
/// units.
///
/// **AWS API**: `cloudwatch.v1.MetricStat`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//MetricStat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetricStat {
    /// The metric to return, including the metric name, namespace, and dimensions.
    pub metric: Metric,

    /// The granularity, in seconds, of the returned data points. For metrics with regular
    /// resolution, a period can be as short as one minute (60 seconds) and must be a multiple
    /// of 60. For high-resolution metrics that are collected at intervals of less than one
    /// minute, the period can be 1, 5, 10, 20, 30, 60, or any multiple of 60. High-resolution
    /// metrics are those metrics stored by a PutMetricData call that includes a
    /// StorageResolution of 1 second. If the StartTime parameter specifies a time stamp that is
    /// greater than 3 hours ago, you must specify the period as follows or no data points in
    /// that time range is returned: Start time between 3 hours and 15 days ago
    /// - Use a multiple of 60 seconds (1 minute). Start time between 15 and 63 days ago
    /// - Use a multiple of 300 seconds (5 minutes). Start time greater than 63 days ago
    /// - Use a multiple of 3600 seconds (1 hour).
    pub period: i32,

    /// The statistic to return. It can include any CloudWatch statistic or extended statistic.
    pub stat: String,

    /// When you are using a Put operation, this defines what unit you want to use when storing
    /// the metric. In a Get operation, if you omit Unit then all data that was collected with
    /// any unit is returned, along with the corresponding units that were specified when the
    /// data was reported to CloudWatch. If you specify a unit, the operation returns only data
    /// that was collected with that unit specified. If you specify a unit that does not match
    /// the data collected, the results of the operation are null. CloudWatch does not perform
    /// unit conversions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<StandardUnit>,
}

impl MetricStat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metric: Metric::fixture(),
            period: 100,
            stat: "test-stat".into(),
            ..Default::default()
        }
    }
}

/// This structure includes the Timezone parameter, which you can use to specify your time zone
/// so that the labels that are associated with returned metrics display the correct time for
/// your time zone. The Timezone value affects a label only if you have a time-based dynamic
/// expression in the label. For more information about dynamic expressions in labels, see Using
/// Dynamic Labels.
///
/// **AWS API**: `cloudwatch.v1.LabelOptions`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//LabelOptions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LabelOptions {
    /// The time zone to use for metric data return in this operation. The format is + or
    /// - followed by four digits. The first two digits indicate the number of hours ahead or
    ///   behind of UTC, and the final two digits are the number of minutes. For example, +0130
    ///   indicates a time zone that is 1 hour and 30 minutes ahead of UTC. The default is
    ///   +0000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl LabelOptions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            timezone: Some("test-timezone".into()),
        }
    }
}

/// A message returned by the GetMetricDataAPI, including a code and a description. If a cross-
/// Region GetMetricData operation fails with a code of Forbidden and a value of Authentication
/// too complex to retrieve cross region data, you can correct the problem by running the
/// GetMetricData operation in the same Region where the metric data is.
///
/// **AWS API**: `cloudwatch.v1.MessageData`
/// **Reference**: <https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference//MessageData>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageData {
    /// The error code or status code associated with the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// The message text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl MessageData {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            code: Some("test-code".into()),
            value: Some("test-value".into()),
        }
    }
}
