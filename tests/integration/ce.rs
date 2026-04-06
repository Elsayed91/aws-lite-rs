//! Integration tests for Cost Explorer API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration ce -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::ce::*;
use std::env;

// Cost Explorer is a global service but must use us-east-1
const TEST_REGION: &str = "us-east-1";

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_ce_operations() {
    // CE is global but only works from us-east-1
    let region = env::var("AWS_CE_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    println!("\n=== Cost Explorer Operations Test ===");
    println!("Region: {region}");

    run_ce_tests(&client)
        .await
        .expect("Cost Explorer tests failed");
    println!("\nAll Cost Explorer operations tests passed!");
}

async fn run_ce_tests(client: &AwsHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    // [1/2] GetCostAndUsage — monthly granularity, last 2 months
    // Use a fixed past time period so results are deterministic
    println!("\n[1/2] GetCostAndUsage (monthly, last 2 months)...");
    let req = GetCostAndUsageRequest {
        time_period: DateInterval {
            start: "2024-11-01".to_string(),
            end: "2025-01-01".to_string(),
        },
        granularity: "MONTHLY".to_string(),
        metrics: vec!["BlendedCost".to_string(), "UnblendedCost".to_string()],
        ..Default::default()
    };
    let resp = client.ce().get_cost_and_usage(&req).await?;

    // Should return 2 monthly time periods
    assert!(
        !resp.results_by_time.is_empty(),
        "Expected at least one result period; got empty ResultsByTime"
    );
    println!("  Received {} time period(s)", resp.results_by_time.len());

    // Verify structure of first result
    let first = &resp.results_by_time[0];
    assert!(
        first.time_period.is_some(),
        "Expected TimePeriod in first result"
    );
    let period = first.time_period.as_ref().unwrap();
    assert!(
        !period.start.is_empty(),
        "Expected non-empty Start in TimePeriod"
    );
    assert!(
        !period.end.is_empty(),
        "Expected non-empty End in TimePeriod"
    );
    println!("  First period: {} to {}", period.start, period.end);

    // Verify Total map has BlendedCost metric
    assert!(
        first.total.contains_key("BlendedCost"),
        "Expected BlendedCost in Total metrics map"
    );
    let blended = &first.total["BlendedCost"];
    assert!(
        blended.amount.is_some(),
        "Expected Amount in BlendedCost MetricValue"
    );
    assert!(
        blended.unit.is_some(),
        "Expected Unit in BlendedCost MetricValue"
    );
    println!(
        "  BlendedCost: {} {}",
        blended.amount.as_deref().unwrap_or("?"),
        blended.unit.as_deref().unwrap_or("?"),
    );

    // [2/2] GetCostAndUsage — with GroupBy SERVICE
    println!("\n[2/2] GetCostAndUsage (monthly, grouped by SERVICE)...");
    let req_grouped = GetCostAndUsageRequest {
        time_period: DateInterval {
            start: "2024-12-01".to_string(),
            end: "2025-01-01".to_string(),
        },
        granularity: "MONTHLY".to_string(),
        metrics: vec!["UnblendedCost".to_string()],
        group_by: vec![GroupDefinition {
            r#type: Some("DIMENSION".to_string()),
            key: Some("SERVICE".to_string()),
        }],
        ..Default::default()
    };
    let resp_grouped = client.ce().get_cost_and_usage(&req_grouped).await?;

    assert!(
        !resp_grouped.results_by_time.is_empty(),
        "Expected at least one result period in grouped query"
    );
    let first_grouped = &resp_grouped.results_by_time[0];
    // When groups are requested, results come in groups[] not total
    println!(
        "  Grouped result: {} group(s) in period",
        first_grouped.groups.len()
    );
    if !first_grouped.groups.is_empty() {
        let g = &first_grouped.groups[0];
        assert!(!g.keys.is_empty(), "Expected at least one Key in group");
        assert!(
            g.metrics.contains_key("UnblendedCost"),
            "Expected UnblendedCost in group Metrics"
        );
        println!(
            "  First group: keys={:?} UnblendedCost={}",
            g.keys,
            g.metrics["UnblendedCost"].amount.as_deref().unwrap_or("?"),
        );
    }

    Ok(())
}
