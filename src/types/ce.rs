//! Types for the AWS Cost Explorer API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The time period of the request.
///
/// **AWS API**: `ce.v1.DateInterval`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DateInterval {
    /// The beginning of the time period. The start date is inclusive. For example, if start is
    /// 2017-01-01, Amazon Web Services retrieves cost and usage data starting at 2017-01-01 up
    /// to the end date. The start date must be equal to or no later than the current date to
    /// avoid a validation error.
    pub start: String,

    /// The end of the time period. The end date is exclusive. For example, if end is
    /// 2017-05-01, Amazon Web Services retrieves cost and usage data from the start date up to,
    /// but not including, 2017-05-01.
    pub end: String,
}

impl DateInterval {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            start: "test-start".into(),
            end: "test-end".into(),
        }
    }
}

/// Represents a group when you specify a group by criteria or in the response to a query with a
/// specific grouping.
///
/// **AWS API**: `ce.v1.GroupDefinition`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GroupDefinition {
    /// The string that represents the type of group.
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The string that represents a key for a specified group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl GroupDefinition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            r#type: Some("test-type".into()),
            key: Some("test-key".into()),
        }
    }
}

/// The aggregated value for a metric.
///
/// **AWS API**: `ce.v1.MetricValue`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetricValue {
    /// The actual number that represents the metric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// The unit that the metric is given in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl MetricValue {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            amount: Some("test-amount".into()),
            unit: Some("test-unit".into()),
        }
    }
}

/// One level of grouped data in the results.
///
/// **AWS API**: `ce.v1.Group`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Group {
    /// The keys that are included in this group.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,

    /// The metrics that are included in this group.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metrics: std::collections::HashMap<String, MetricValue>,
}

impl Group {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            keys: vec![],
            ..Default::default()
        }
    }
}

/// The result that's associated with a time period.
///
/// **AWS API**: `ce.v1.ResultByTime`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResultByTime {
    /// The time period that the result covers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,

    /// The total amount of cost or usage accrued during the time period.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub total: std::collections::HashMap<String, MetricValue>,

    /// The groups that this time period includes.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Group>,

    /// Determines whether the result is estimated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated: Option<bool>,
}

impl ResultByTime {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            time_period: Some(DateInterval::fixture()),
            groups: vec![],
            estimated: Some(false),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `ce.v1.GetCostAndUsageRequest`
///
/// ## Coverage
/// 5 of 7 fields included.
/// Omitted fields:
/// - `Filter` — not selected in manifest
/// - `BillingViewArn` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCostAndUsageRequest {
    /// Sets the start date and end date for retrieving Amazon Web Services costs. The start
    /// date is inclusive, but the end date is exclusive. For example, if start is 2017-01-01
    /// and end is 2017-05-01, then the cost and usage data is retrieved from 2017-01-01 up to
    /// and including 2017-04-30 but not including 2017-05-01.
    pub time_period: DateInterval,

    /// Sets the Amazon Web Services cost granularity to MONTHLY or DAILY, or HOURLY. If
    /// Granularity isn't set, the response object doesn't include the Granularity, either
    /// MONTHLY or DAILY, or HOURLY.
    pub granularity: String,

    /// Which metrics are returned in the query. For more information about blended and
    /// unblended rates, see Why does the "blended" annotation appear on some line items in my
    /// bill?. Valid values are AmortizedCost, BlendedCost, NetAmortizedCost, NetUnblendedCost,
    /// NormalizedUsageAmount, UnblendedCost, and UsageQuantity. If you return the UsageQuantity
    /// metric, the service aggregates all usage numbers without taking into account the units.
    /// For example, if you aggregate usageQuantity across all of Amazon EC2, the results aren't
    /// meaningful because Amazon EC2 compute hours and data transfer are measured in different
    /// units (for example, hours and GB). To get more meaningful UsageQuantity metrics, filter
    /// by UsageType or UsageTypeGroups. Metrics is required for GetCostAndUsage requests.
    #[serde(default)]
    pub metrics: Vec<String>,

    /// You can group Amazon Web Services costs using up to two different groups, either
    /// dimensions, tag keys, cost categories, or any two group by types. Valid values for the
    /// DIMENSION type are AZ, INSTANCE_TYPE, LEGAL_ENTITY_NAME, INVOICING_ENTITY,
    /// LINKED_ACCOUNT, OPERATION, PLATFORM, PURCHASE_TYPE, SERVICE, TENANCY, RECORD_TYPE, and
    /// USAGE_TYPE. When you group by the TAG type and include a valid tag key, you get all tag
    /// values, including empty strings.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub group_by: Vec<GroupDefinition>,

    /// The token to retrieve the next set of results. Amazon Web Services provides the token
    /// when the response from a previous call has more results than the maximum page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl GetCostAndUsageRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            time_period: DateInterval::fixture(),
            granularity: "test-granularity".into(),
            metrics: vec![],
            group_by: vec![],
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

///
/// **AWS API**: `ce.v1.GetCostAndUsageResponse`
///
/// ## Coverage
/// 3 of 4 fields included.
/// Omitted fields:
/// - `DimensionValueAttributes` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCostAndUsageResponse {
    /// The token for the next set of retrievable results. Amazon Web Services provides the
    /// token when the response from a previous call has more results than the maximum page
    /// size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,

    /// The groups that are specified by the Filter or GroupBy parameters in the request.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub group_definitions: Vec<GroupDefinition>,

    /// The time period that's covered by the results in the response.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub results_by_time: Vec<ResultByTime>,
}

impl GetCostAndUsageResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            next_page_token: Some("test-next_page_token".into()),
            group_definitions: vec![],
            results_by_time: vec![],
        }
    }
}
