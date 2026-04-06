//! AWS Query protocol support.
//!
//! Builds form-urlencoded request bodies for AWS Query/XML protocol services
//! like CloudWatch.

use serde::Serialize;

/// Build a form-urlencoded query body for an AWS Query protocol request.
///
/// Produces a body like: `Action=GetMetricStatistics&Version=2010-08-01&Param1=value1`
///
/// Lists use `Name.member.N` notation (standard Query protocol: CloudWatch, IAM, etc.)
pub fn build_query_body<T: Serialize>(action: &str, version: &str, params: Option<&T>) -> String {
    build_query_body_inner(action, version, params, true)
}

/// Build a form-urlencoded query body for an EC2 API request.
///
/// EC2 uses `Name.N` list notation (no `.member.` infix), unlike standard Query protocol.
pub fn build_ec2_query_body<T: Serialize>(
    action: &str,
    version: &str,
    params: Option<&T>,
) -> String {
    build_query_body_inner(action, version, params, false)
}

fn build_query_body_inner<T: Serialize>(
    action: &str,
    version: &str,
    params: Option<&T>,
    use_member: bool,
) -> String {
    let mut parts = vec![
        format!("Action={}", urlencoding::encode(action)),
        format!("Version={}", urlencoding::encode(version)),
    ];

    if let Some(body) = params {
        let json = serde_json::to_value(body).unwrap_or_default();
        if let serde_json::Value::Object(map) = json {
            flatten_to_query_params(&mut parts, "", &serde_json::Value::Object(map), use_member);
        }
    }

    parts.join("&")
}

/// Recursively flatten a JSON value into AWS Query-style parameters.
///
/// Handles nested structures (Dot notation) and lists.
/// When `use_member` is true, lists use `Name.member.N` (standard Query).
/// When false, lists use `Name.N` (EC2 protocol).
fn flatten_to_query_params(
    parts: &mut Vec<String>,
    prefix: &str,
    value: &serde_json::Value,
    use_member: bool,
) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                flatten_to_query_params(parts, &new_prefix, val, use_member);
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let new_prefix = if use_member {
                    format!("{prefix}.member.{}", i + 1)
                } else {
                    format!("{prefix}.{}", i + 1)
                };
                flatten_to_query_params(parts, &new_prefix, val, use_member);
            }
        }
        serde_json::Value::String(s) => {
            parts.push(format!("{}={}", prefix, urlencoding::encode(s)));
        }
        serde_json::Value::Number(n) => {
            parts.push(format!("{prefix}={n}"));
        }
        serde_json::Value::Bool(b) => {
            parts.push(format!("{prefix}={b}"));
        }
        serde_json::Value::Null => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_basic_query() {
        let body = build_query_body::<()>("GetMetricStatistics", "2010-08-01", None);
        assert!(body.contains("Action=GetMetricStatistics"));
        assert!(body.contains("Version=2010-08-01"));
    }

    #[test]
    fn build_query_with_params() {
        #[derive(Serialize)]
        struct Params {
            #[serde(rename = "Namespace")]
            namespace: String,
            #[serde(rename = "MetricName")]
            metric_name: String,
        }

        let params = Params {
            namespace: "AWS/EC2".into(),
            metric_name: "CPUUtilization".into(),
        };
        let body = build_query_body("GetMetricStatistics", "2010-08-01", Some(&params));
        assert!(body.contains("Action=GetMetricStatistics"));
        assert!(body.contains("Namespace=AWS%2FEC2"));
        assert!(body.contains("MetricName=CPUUtilization"));
    }

    #[test]
    fn build_query_with_list() {
        #[derive(Serialize)]
        struct Params {
            #[serde(rename = "Statistics")]
            statistics: Vec<String>,
        }

        let params = Params {
            statistics: vec!["Average".into(), "Maximum".into()],
        };
        let body = build_query_body("GetMetricStatistics", "2010-08-01", Some(&params));
        assert!(body.contains("Statistics.member.1=Average"));
        assert!(body.contains("Statistics.member.2=Maximum"));
    }

    #[test]
    fn build_ec2_query_with_list() {
        #[derive(Serialize)]
        struct Params {
            #[serde(rename = "Owner")]
            owner_ids: Vec<String>,
        }

        let params = Params {
            owner_ids: vec!["self".into(), "amazon".into()],
        };
        let body = build_ec2_query_body("DescribeSnapshots", "2016-11-15", Some(&params));
        assert!(body.contains("Owner.1=self"));
        assert!(body.contains("Owner.2=amazon"));
        assert!(!body.contains("member"));
    }
}
