//! Integration tests for Amazon CloudWatch API
//!
//! Tests GetMetricStatistics against the real AWS CloudWatch API.
//!
//! Run with:
//!   AWS_PROFILE=<profile> AWS_REGION=eu-central-1 \
//!     cargo test --test integration cloudwatch -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::cloudwatch::*;
use std::env;

const TEST_ALARM_NAME: &str = "cloud-lite-test-ralph-cw-alarm";

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "eu-central-1".into())
}

// -- GetMetricStatistics -------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_get_metric_statistics() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let cw = client.cloudwatch();

    println!("\n=== GetMetricStatistics ===");

    let now = chrono::Utc::now();
    let one_hour_ago = now - chrono::Duration::hours(1);

    // 1. Basic query — CPUUtilization for the last hour
    println!("\n[1/3] Querying CPUUtilization (last 1h, 300s period)...");
    let input = GetMetricStatisticsInput {
        namespace: "AWS/EC2".into(),
        metric_name: "CPUUtilization".into(),
        start_time: one_hour_ago.to_rfc3339(),
        end_time: now.to_rfc3339(),
        period: 300,
        statistics: vec![Statistic::Average, Statistic::Maximum],
        ..Default::default()
    };

    let response = cw.get_metric_statistics(&input).await?;
    println!("  Label: {:?}", response.label);
    println!("  Datapoints: {}", response.datapoints.len());
    for dp in response.datapoints.iter().take(3) {
        println!(
            "    ts={:?} avg={:?} max={:?}",
            dp.timestamp, dp.average, dp.maximum
        );
    }

    // 2. Query with a dimension filter for non-existent instance
    println!("\n[2/3] Querying with dimension filter (non-existent instance)...");
    let input_with_dim = GetMetricStatisticsInput {
        namespace: "AWS/EC2".into(),
        metric_name: "CPUUtilization".into(),
        start_time: one_hour_ago.to_rfc3339(),
        end_time: now.to_rfc3339(),
        period: 300,
        statistics: vec![Statistic::Average],
        dimensions: vec![Dimension {
            name: "InstanceId".into(),
            value: "i-nonexistent".into(),
        }],
        ..Default::default()
    };
    let response = cw.get_metric_statistics(&input_with_dim).await?;
    println!(
        "  Datapoints for non-existent instance: {}",
        response.datapoints.len()
    );
    // Non-existent instance should return 0 datapoints (not an error)
    assert!(
        response.datapoints.is_empty(),
        "non-existent instance should return no datapoints"
    );

    // 3. Query a different namespace (Lambda)
    println!("\n[3/3] Querying Lambda Invocations (may be empty)...");
    let lambda_input = GetMetricStatisticsInput {
        namespace: "AWS/Lambda".into(),
        metric_name: "Invocations".into(),
        start_time: (now - chrono::Duration::hours(24)).to_rfc3339(),
        end_time: now.to_rfc3339(),
        period: 3600,
        statistics: vec![Statistic::Sum],
        ..Default::default()
    };
    let lambda_response = cw.get_metric_statistics(&lambda_input).await?;
    println!(
        "  Lambda invocations datapoints: {}",
        lambda_response.datapoints.len()
    );

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_get_metric_statistics_error_cases() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let cw = client.cloudwatch();

    println!("\n=== GetMetricStatistics Error Cases ===");

    let now = chrono::Utc::now();

    // 1. No statistics specified — AWS rejects this with InvalidParameterCombination
    println!("\n[1/1] Querying with no statistics...");
    let input = GetMetricStatisticsInput {
        namespace: "AWS/EC2".into(),
        metric_name: "CPUUtilization".into(),
        start_time: (now - chrono::Duration::hours(1)).to_rfc3339(),
        end_time: now.to_rfc3339(),
        period: 300,
        ..Default::default()
    };
    let result = cw.get_metric_statistics(&input).await;
    assert!(result.is_err(), "no statistics should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}

// -- GetMetricData -------------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_get_metric_data() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let cw = client.cloudwatch();

    println!("\n=== GetMetricData ===");

    let now = chrono::Utc::now();
    let one_hour_ago = now - chrono::Duration::hours(1);

    // 1. Batch query: two metrics in one request
    println!("\n[1/3] Batching CPUUtilization + NetworkIn (last 1h, 300s period)...");
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
        start_time: one_hour_ago.to_rfc3339(),
        end_time: now.to_rfc3339(),
        ..Default::default()
    };
    let response = cw.get_metric_data(&input).await?;
    println!("  Results returned: {}", response.metric_data_results.len());
    for result in &response.metric_data_results {
        println!(
            "    id={:?} label={:?} values={} status={:?}",
            result.id,
            result.label,
            result.values.len(),
            result.status_code
        );
    }
    // The response should have entries for both metric IDs (may have 0 values if no data)
    assert_eq!(
        response.metric_data_results.len(),
        2,
        "should return results for both metric queries"
    );
    let ids: Vec<_> = response
        .metric_data_results
        .iter()
        .filter_map(|r| r.id.as_deref())
        .collect();
    assert!(ids.contains(&"cpu"), "should include cpu result");
    assert!(
        ids.contains(&"networkin"),
        "should include networkin result"
    );
    println!("  NextToken: {:?}", response.next_token);

    // 2. Math expression: error rate as Errors/Invocations
    println!("\n[2/3] Lambda error rate via math expression...");
    let math_input = GetMetricDataInput {
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
                id: "invocations".into(),
                metric_stat: Some(MetricStat {
                    metric: Metric {
                        namespace: Some("AWS/Lambda".into()),
                        metric_name: Some("Invocations".into()),
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
                expression: Some("errors / invocations * 100".into()),
                return_data: Some(true),
                ..Default::default()
            },
        ],
        start_time: (now - chrono::Duration::hours(24)).to_rfc3339(),
        end_time: now.to_rfc3339(),
        ..Default::default()
    };
    let math_response = cw.get_metric_data(&math_input).await?;
    println!(
        "  Math expression results: {}",
        math_response.metric_data_results.len()
    );
    for r in &math_response.metric_data_results {
        println!(
            "    id={:?} values={} status={:?}",
            r.id,
            r.values.len(),
            r.status_code
        );
    }
    // Only error_rate has return_data=true
    assert_eq!(
        math_response.metric_data_results.len(),
        1,
        "only the math expression result should be returned"
    );
    assert_eq!(
        math_response.metric_data_results[0].id.as_deref(),
        Some("error_rate"),
        "result id should be error_rate"
    );

    // 3. Pagination: MaxDatapoints=1 to force a second page
    println!("\n[3/3] Testing pagination (MaxDatapoints=1)...");
    let page1_input = GetMetricDataInput {
        metric_data_queries: vec![MetricDataQuery {
            id: "cpu_paged".into(),
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
        }],
        start_time: (now - chrono::Duration::hours(24)).to_rfc3339(),
        end_time: now.to_rfc3339(),
        max_datapoints: Some(1),
        ..Default::default()
    };
    let page1 = cw.get_metric_data(&page1_input).await?;
    println!(
        "  Page 1: {} result(s), next_token={:?}",
        page1.metric_data_results.len(),
        page1.next_token
    );
    // If there are any datapoints at all, we expect a next_token due to max_datapoints=1
    if page1.next_token.is_some() {
        let page2_input = GetMetricDataInput {
            next_token: page1.next_token.clone(),
            ..page1_input.clone()
        };
        let page2 = cw.get_metric_data(&page2_input).await?;
        println!(
            "  Page 2: {} result(s), next_token={:?}",
            page2.metric_data_results.len(),
            page2.next_token
        );
    } else {
        println!("  No pagination needed (no EC2 data in last 24h)");
    }

    println!("\nAll GetMetricData tests passed!");
    Ok(())
}

// -- ListMetrics ---------------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_list_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let cw = client.cloudwatch();

    println!("\n=== ListMetrics ===");

    // 1. List metrics in AWS/EC2 namespace
    println!("\n[1/2] Listing EC2 metrics...");
    let input = ListMetricsInput {
        namespace: Some("AWS/EC2".into()),
        ..Default::default()
    };
    let response = cw.list_metrics(&input).await?;
    println!("  Metrics found: {}", response.metrics.len());
    for metric in response.metrics.iter().take(3) {
        println!(
            "    ns={:?} name={:?} dims={}",
            metric.namespace,
            metric.metric_name,
            metric.dimensions.len()
        );
    }
    // EC2 namespace should have at least some standard metrics
    // (may be empty if no EC2 instances exist, but the call should succeed)

    // 2. List with a specific metric name filter
    println!("\n[2/2] Listing CPUUtilization metrics...");
    let input = ListMetricsInput {
        namespace: Some("AWS/EC2".into()),
        metric_name: Some("CPUUtilization".into()),
        ..Default::default()
    };
    let response = cw.list_metrics(&input).await?;
    println!("  CPUUtilization metrics: {}", response.metrics.len());
    // All returned metrics should have the requested name
    for m in &response.metrics {
        assert_eq!(
            m.metric_name.as_deref(),
            Some("CPUUtilization"),
            "all returned metrics should be CPUUtilization"
        );
    }

    Ok(())
}

// -- PutMetricAlarm + DescribeAlarms + DeleteAlarms ----------------------

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_put_describe_delete_alarm() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let cw = client.cloudwatch();

    println!("\n=== PutMetricAlarm + DescribeAlarms + DeleteAlarms ===");

    // 1. Create a test alarm
    println!("\n[1/4] Creating test alarm: {}...", TEST_ALARM_NAME);
    let alarm_input = PutMetricAlarmInput {
        alarm_name: TEST_ALARM_NAME.into(),
        alarm_description: Some("Integration test alarm for cloud-lite".into()),
        metric_name: Some("CPUUtilization".into()),
        namespace: Some("AWS/EC2".into()),
        statistic: Some(Statistic::Average),
        period: Some(300),
        evaluation_periods: 1,
        threshold: Some(90.0),
        comparison_operator: Some(ComparisonOperator::GreaterThanThreshold),
        treat_missing_data: Some("notBreaching".into()),
        ..Default::default()
    };
    cw.put_metric_alarm(&alarm_input).await?;
    println!("  Alarm created");

    // 2. Describe by name
    println!("\n[2/4] Describing alarm by name...");
    let describe_input = DescribeAlarmsInput {
        alarm_names: vec![TEST_ALARM_NAME.into()],
        ..Default::default()
    };
    let response = cw.describe_alarms(&describe_input).await?;
    println!(
        "  MetricAlarms: {}, NextToken: {:?}",
        response.metric_alarms.len(),
        response.next_token
    );
    assert_eq!(
        response.metric_alarms.len(),
        1,
        "should find exactly one alarm"
    );
    let alarm = &response.metric_alarms[0];
    assert_eq!(alarm.alarm_name.as_deref(), Some(TEST_ALARM_NAME));
    assert_eq!(alarm.metric_name.as_deref(), Some("CPUUtilization"));
    assert_eq!(alarm.namespace.as_deref(), Some("AWS/EC2"));
    assert_eq!(alarm.threshold, Some(90.0));
    assert_eq!(
        alarm.comparison_operator,
        Some(ComparisonOperator::GreaterThanThreshold)
    );
    assert_eq!(alarm.treat_missing_data.as_deref(), Some("notBreaching"));
    println!("  Alarm verified: state={:?}", alarm.state_value);

    // 3. Update the alarm threshold
    println!("\n[3/4] Updating alarm threshold to 80.0...");
    let update_input = PutMetricAlarmInput {
        alarm_name: TEST_ALARM_NAME.into(),
        metric_name: Some("CPUUtilization".into()),
        namespace: Some("AWS/EC2".into()),
        statistic: Some(Statistic::Average),
        period: Some(300),
        evaluation_periods: 1,
        threshold: Some(80.0),
        comparison_operator: Some(ComparisonOperator::GreaterThanThreshold),
        ..Default::default()
    };
    cw.put_metric_alarm(&update_input).await?;

    // Verify the update
    let response = cw.describe_alarms(&describe_input).await?;
    assert_eq!(response.metric_alarms[0].threshold, Some(80.0));
    println!("  Threshold updated and verified");

    // 4. Delete the alarm
    println!("\n[4/4] Deleting test alarm...");
    cw.delete_alarms(vec![TEST_ALARM_NAME.into()]).await?;
    println!("  Alarm deleted");

    // Verify deletion
    let response = cw.describe_alarms(&describe_input).await?;
    assert!(response.metric_alarms.is_empty(), "alarm should be deleted");
    println!("  Deletion verified");

    Ok(())
}
