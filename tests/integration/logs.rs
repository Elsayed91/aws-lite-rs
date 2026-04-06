//! Integration tests for Amazon CloudWatch Logs API
//!
//! Tests DescribeLogGroups and ListTagsForResource against the real AWS API.
//!
//! Run with:
//!   AWS_PROFILE=<profile> AWS_REGION=eu-central-1 \
//!     cargo test --test integration logs -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::logs::*;
use std::env;
use std::process::Command;

const TEST_LOG_GROUP: &str = "/cloud-lite/test-ralph-logs-ext";
const TEST_LOG_STREAM: &str = "test-stream-001";

fn aws_create_log_group(name: &str, region: &str) {
    Command::new("aws")
        .args([
            "logs",
            "create-log-group",
            "--log-group-name",
            name,
            "--region",
            region,
        ])
        .output()
        .expect("failed to run aws cli");
}

fn aws_create_log_stream(group: &str, stream: &str, region: &str) {
    Command::new("aws")
        .args([
            "logs",
            "create-log-stream",
            "--log-group-name",
            group,
            "--log-stream-name",
            stream,
            "--region",
            region,
        ])
        .output()
        .expect("failed to run aws cli");
}

fn aws_delete_log_group(name: &str, region: &str) {
    Command::new("aws")
        .args([
            "logs",
            "delete-log-group",
            "--log-group-name",
            name,
            "--region",
            region,
        ])
        .output()
        .expect("failed to run aws cli");
}

fn aws_put_metric_filter(
    log_group: &str,
    filter_name: &str,
    filter_pattern: &str,
    metric_name: &str,
    metric_namespace: &str,
    region: &str,
) {
    Command::new("aws")
        .args([
            "logs",
            "put-metric-filter",
            "--log-group-name",
            log_group,
            "--filter-name",
            filter_name,
            "--filter-pattern",
            filter_pattern,
            "--metric-transformations",
            &format!(
                "metricName={},metricNamespace={},metricValue=1",
                metric_name, metric_namespace
            ),
            "--region",
            region,
        ])
        .output()
        .expect("failed to run aws cli");
}

fn aws_delete_metric_filter(log_group: &str, filter_name: &str, region: &str) {
    Command::new("aws")
        .args([
            "logs",
            "delete-metric-filter",
            "--log-group-name",
            log_group,
            "--filter-name",
            filter_name,
            "--region",
            region,
        ])
        .output()
        .expect("failed to run aws cli");
}

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "eu-central-1".into())
}

// -- DescribeLogGroups ---------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_describe_log_groups() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let logs = client.logs();

    println!("\n=== DescribeLogGroups ===");

    // 1. List all log groups (default, no filter)
    println!("\n[1/3] Listing all log groups...");
    let request = DescribeLogGroupsRequest::default();
    let response = logs.describe_log_groups(&request).await?;
    println!("  Got {} log groups", response.log_groups.len());
    println!("  Has next token: {}", response.next_token.is_some());

    for lg in response.log_groups.iter().take(5) {
        println!(
            "  - {} (arn={:?}, bytes={:?})",
            lg.log_group_name.as_deref().unwrap_or("?"),
            lg.arn.as_deref().map(|a| &a[..a.len().min(60)]),
            lg.stored_bytes,
        );
    }

    // 2. List with prefix filter
    println!("\n[2/3] Listing with prefix '/aws'...");
    let request = DescribeLogGroupsRequest {
        log_group_name_prefix: Some("/aws".into()),
        ..Default::default()
    };
    let filtered = logs.describe_log_groups(&request).await?;
    println!(
        "  Got {} log groups with /aws prefix",
        filtered.log_groups.len()
    );
    for lg in filtered.log_groups.iter().take(3) {
        let name = lg.log_group_name.as_deref().unwrap_or("?");
        assert!(
            name.starts_with("/aws"),
            "filtered log group should start with /aws, got: {}",
            name
        );
        println!("  - {}", name);
    }

    // 3. List with limit
    println!("\n[3/3] Listing with limit=2...");
    let request = DescribeLogGroupsRequest {
        limit: Some(2),
        ..Default::default()
    };
    let limited = logs.describe_log_groups(&request).await?;
    println!("  Got {} log groups (limit=2)", limited.log_groups.len());
    assert!(
        limited.log_groups.len() <= 2,
        "should respect limit, got {}",
        limited.log_groups.len()
    );

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_describe_log_groups_pagination() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let logs = client.logs();

    println!("\n=== DescribeLogGroups Pagination ===");

    // Request with limit=1 to force pagination
    println!("[1/1] Paginating with limit=1...");
    let request = DescribeLogGroupsRequest {
        limit: Some(1),
        ..Default::default()
    };
    let page1 = logs.describe_log_groups(&request).await?;
    println!("  Page 1: {} log groups", page1.log_groups.len());

    if let Some(ref token) = page1.next_token {
        println!("  Next token present, fetching page 2...");
        let page2_request = DescribeLogGroupsRequest {
            limit: Some(1),
            next_token: Some(token.clone()),
            ..Default::default()
        };
        let page2 = logs.describe_log_groups(&page2_request).await?;
        println!("  Page 2: {} log groups", page2.log_groups.len());

        // Verify no overlap
        if !page1.log_groups.is_empty() && !page2.log_groups.is_empty() {
            let name1 = page1.log_groups[0].log_group_name.as_deref().unwrap_or("");
            let name2 = page2.log_groups[0].log_group_name.as_deref().unwrap_or("");
            assert_ne!(name1, name2, "paginated results should not overlap");
            println!("  Page 1: {}", name1);
            println!("  Page 2: {}", name2);
        }
    } else {
        println!("  No next token (only one log group or none)");
    }

    Ok(())
}

// -- ListTagsForResource -------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_list_tags_for_resource() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let logs = client.logs();

    println!("\n=== ListTagsForResource ===");

    // First, get a log group ARN to query tags for
    let request = DescribeLogGroupsRequest {
        limit: Some(1),
        ..Default::default()
    };
    let groups = logs.describe_log_groups(&request).await?;

    if groups.log_groups.is_empty() {
        println!("  No log groups found, skipping tag test");
        return Ok(());
    }

    let log_group = &groups.log_groups[0];
    // Use the log_group_arn (without trailing :*) for ListTagsForResource
    let arn = log_group
        .log_group_arn
        .as_deref()
        .or(log_group.arn.as_deref())
        .expect("log group should have an ARN");

    // Strip trailing :* if present (ListTagsForResource needs the ARN without it)
    let arn = arn.trim_end_matches(":*");

    println!(
        "[1/1] Getting tags for {}...",
        log_group.log_group_name.as_deref().unwrap_or("?")
    );
    let tag_request = ListTagsForResourceRequest {
        resource_arn: arn.to_string(),
    };
    let tag_response = logs.list_tags_for_resource(&tag_request).await?;
    println!("  Tags: {:?}", tag_response.tags);

    Ok(())
}

// -- DescribeLogStreams ---------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_describe_log_streams() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let logs = client.logs();

    println!("\n=== DescribeLogStreams ===");

    // Fixture: create log group and stream via CLI
    println!("\n  Setup: creating log group and stream...");
    aws_create_log_group(TEST_LOG_GROUP, &region);
    aws_create_log_stream(TEST_LOG_GROUP, TEST_LOG_STREAM, &region);

    // 1. Describe log streams for our test group
    println!("\n[1/2] Describing log streams for {}...", TEST_LOG_GROUP);
    let request = DescribeLogStreamsRequest {
        log_group_name: Some(TEST_LOG_GROUP.into()),
        ..Default::default()
    };
    let response = logs.describe_log_streams(&request).await?;
    println!("  Streams found: {}", response.log_streams.len());
    assert!(
        !response.log_streams.is_empty(),
        "should find at least one log stream"
    );
    let stream = response
        .log_streams
        .iter()
        .find(|s| s.log_stream_name.as_deref() == Some(TEST_LOG_STREAM));
    assert!(stream.is_some(), "should find our test stream");
    let stream = stream.unwrap();
    println!(
        "  Found: name={:?}, created={:?}",
        stream.log_stream_name, stream.creation_time
    );
    assert!(stream.creation_time.is_some());

    // 2. Describe with prefix filter
    println!("\n[2/2] Describing with prefix 'test-stream'...");
    let request = DescribeLogStreamsRequest {
        log_group_name: Some(TEST_LOG_GROUP.into()),
        log_stream_name_prefix: Some("test-stream".into()),
        ..Default::default()
    };
    let response = logs.describe_log_streams(&request).await?;
    println!("  Filtered streams: {}", response.log_streams.len());
    for s in &response.log_streams {
        assert!(
            s.log_stream_name
                .as_deref()
                .unwrap_or("")
                .starts_with("test-stream"),
            "filtered stream should start with 'test-stream'"
        );
    }

    Ok(())
}

// -- PutRetentionPolicy ---------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_put_retention_policy() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let logs = client.logs();

    println!("\n=== PutRetentionPolicy ===");

    // Fixture: ensure log group exists
    aws_create_log_group(TEST_LOG_GROUP, &region);

    // 1. Set retention to 7 days
    println!("\n[1/2] Setting retention to 7 days...");
    logs.put_retention_policy(TEST_LOG_GROUP, 7).await?;
    println!("  Retention set");

    // Verify by describing the log group
    let request = DescribeLogGroupsRequest {
        log_group_name_prefix: Some(TEST_LOG_GROUP.into()),
        ..Default::default()
    };
    let response = logs.describe_log_groups(&request).await?;
    let group = response
        .log_groups
        .iter()
        .find(|g| g.log_group_name.as_deref() == Some(TEST_LOG_GROUP));
    assert!(group.is_some(), "should find our test log group");
    assert_eq!(group.unwrap().retention_in_days, Some(7));
    println!("  Verified: retention_in_days=7");

    // 2. Update retention to 14 days
    println!("\n[2/2] Updating retention to 14 days...");
    logs.put_retention_policy(TEST_LOG_GROUP, 14).await?;
    let response = logs.describe_log_groups(&request).await?;
    let group = response
        .log_groups
        .iter()
        .find(|g| g.log_group_name.as_deref() == Some(TEST_LOG_GROUP));
    assert_eq!(group.unwrap().retention_in_days, Some(14));
    println!("  Verified: retention_in_days=14");

    Ok(())
}

// -- DeleteLogStream ------------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_delete_log_stream() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let logs = client.logs();

    println!("\n=== DeleteLogStream ===");

    // Fixture: create a fresh stream to delete
    let delete_stream = "test-stream-to-delete";
    aws_create_log_group(TEST_LOG_GROUP, &region);
    aws_create_log_stream(TEST_LOG_GROUP, delete_stream, &region);

    // 1. Verify stream exists
    println!("\n[1/2] Verifying stream exists...");
    let request = DescribeLogStreamsRequest {
        log_group_name: Some(TEST_LOG_GROUP.into()),
        log_stream_name_prefix: Some(delete_stream.into()),
        ..Default::default()
    };
    let response = logs.describe_log_streams(&request).await?;
    assert!(
        response
            .log_streams
            .iter()
            .any(|s| s.log_stream_name.as_deref() == Some(delete_stream)),
        "stream should exist before deletion"
    );
    println!("  Stream found");

    // 2. Delete it
    println!("\n[2/2] Deleting log stream...");
    logs.delete_log_stream(TEST_LOG_GROUP, delete_stream)
        .await?;
    println!("  Stream deleted");

    // Verify deletion
    let response = logs.describe_log_streams(&request).await?;
    assert!(
        !response
            .log_streams
            .iter()
            .any(|s| s.log_stream_name.as_deref() == Some(delete_stream)),
        "stream should be deleted"
    );
    println!("  Deletion verified");

    // Cleanup
    aws_delete_log_group(TEST_LOG_GROUP, &region);

    Ok(())
}

// -- DescribeMetricFilters ------------------------------------------------

const METRIC_FILTER_NAME: &str = "cloud-lite-test-ralph-cis-unauthorized-api";
const METRIC_FILTER_PATTERN: &str =
    r#"{ ($.errorCode = "AccessDenied") || ($.errorCode = "UnauthorizedOperation") }"#;
const METRIC_FILTER_METRIC_NAME: &str = "cloud-lite-test-ralph-UnauthorizedApiCalls";
const METRIC_FILTER_NAMESPACE: &str = "cloud-lite-test-ralph-CISBenchmark";

/// CIS 5.1–5.15: verifies DescribeMetricFilters can list metric filters
/// and the paginated helper works correctly.
///
/// Creates a test metric filter, queries it, then cleans up.
#[tokio::test]
#[ignore = "requires AWS credentials and region"]
async fn test_describe_metric_filters() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let logs = client.logs();

    println!("\n=== DescribeMetricFilters ===");
    println!("Region: {region}");

    // Pre-cleanup from any previous run
    aws_delete_metric_filter(TEST_LOG_GROUP, METRIC_FILTER_NAME, &region);
    aws_delete_log_group(TEST_LOG_GROUP, &region);

    // Setup: create log group + metric filter
    println!("\n  Setup: creating log group and metric filter...");
    aws_create_log_group(TEST_LOG_GROUP, &region);
    aws_put_metric_filter(
        TEST_LOG_GROUP,
        METRIC_FILTER_NAME,
        METRIC_FILTER_PATTERN,
        METRIC_FILTER_METRIC_NAME,
        METRIC_FILTER_NAMESPACE,
        &region,
    );
    println!("  Created log group: {TEST_LOG_GROUP}");
    println!("  Created metric filter: {METRIC_FILTER_NAME}");

    let result = run_describe_metric_filters_tests(&logs, &region).await;

    // Always cleanup
    aws_delete_metric_filter(TEST_LOG_GROUP, METRIC_FILTER_NAME, &region);
    aws_delete_log_group(TEST_LOG_GROUP, &region);
    println!("\n  Cleanup complete.");

    result
}

async fn run_describe_metric_filters_tests(
    logs: &aws_lite::api::logs::LogsClient<'_>,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // [1/4] describe_metric_filters scoped to our test log group
    println!("\n[1/4] Describing metric filters for test log group...");
    let resp = logs
        .describe_metric_filters(Some(TEST_LOG_GROUP), None, None)
        .await?;
    println!("  Found {} filter(s)", resp.metric_filters.len());
    assert!(
        !resp.metric_filters.is_empty(),
        "should find at least one metric filter in test log group"
    );
    let f = resp
        .metric_filters
        .iter()
        .find(|f| f.filter_name.as_deref() == Some(METRIC_FILTER_NAME))
        .expect("should find our test metric filter");

    println!("  filter_name: {:?}", f.filter_name);
    println!("  filter_pattern: {:?}", f.filter_pattern);
    println!("  log_group_name: {:?}", f.log_group_name);
    println!("  creation_time: {:?}", f.creation_time);
    println!(
        "  metric_transformations: {} item(s)",
        f.metric_transformations.len()
    );

    assert_eq!(f.filter_name.as_deref(), Some(METRIC_FILTER_NAME));
    assert_eq!(
        f.filter_pattern.as_deref(),
        Some(METRIC_FILTER_PATTERN),
        "filter_pattern mismatch"
    );
    assert_eq!(f.log_group_name.as_deref(), Some(TEST_LOG_GROUP));
    assert!(f.creation_time.is_some(), "creation_time should be set");

    // [2/4] Verify metric transformation fields
    println!("\n[2/4] Verifying metric transformation fields...");
    assert_eq!(
        f.metric_transformations.len(),
        1,
        "should have exactly one metric transformation"
    );
    let t = &f.metric_transformations[0];
    println!(
        "  metricName={}, metricNamespace={}, metricValue={}",
        t.metric_name, t.metric_namespace, t.metric_value
    );
    assert_eq!(t.metric_name, METRIC_FILTER_METRIC_NAME);
    assert_eq!(t.metric_namespace, METRIC_FILTER_NAMESPACE);
    assert_eq!(t.metric_value, "1");

    // [3/4] list_all_metric_filters — paginated helper
    println!("\n[3/4] Using list_all_metric_filters paginated helper...");
    let all = logs.list_all_metric_filters(Some(TEST_LOG_GROUP)).await?;
    println!("  Total filters in test group: {}", all.len());
    assert!(
        all.iter()
            .any(|f| f.filter_name.as_deref() == Some(METRIC_FILTER_NAME)),
        "paginated helper should include our test filter"
    );
    println!("  Verified: test filter present in paginated results");

    // [4/4] list_all_metric_filters without filter (account-wide)
    // This verifies the no-scope call works and reports CIS coverage
    println!("\n[4/4] CIS 5.1–5.15 compliance check (account-wide metric filters)...");
    let all_account = logs.list_all_metric_filters(None).await?;
    println!("  Total metric filters in account: {}", all_account.len());

    // Check for common CIS alarm patterns
    let cis_patterns = [
        ("CIS 5.1 — unauthorized API", "UnauthorizedOperation"),
        (
            "CIS 5.2 — root account usage",
            "\"userIdentity.type\":\"Root\"",
        ),
        ("CIS 5.3 — IAM policy changes", "UpdateGroupPolicy"),
        ("CIS 5.4 — CloudTrail config changes", "StopLogging"),
        (
            "CIS 5.5 — S3 bucket policy changes",
            "s3:DeleteBucketPolicy",
        ),
        ("CIS 5.9 — AWS Config changes", "ConfigurationRecorder"),
        (
            "CIS 5.12 — network gateway changes",
            "CreateCustomerGateway",
        ),
        ("CIS 5.13 — route table changes", "CreateRoute"),
        ("CIS 5.14 — VPC changes", "CreateVpc"),
    ];
    for (check, keyword) in &cis_patterns {
        let found = all_account
            .iter()
            .any(|f| f.filter_pattern.as_deref().unwrap_or("").contains(keyword));
        if found {
            println!("  ✓ {check} — filter pattern found");
        } else {
            println!("  ~ {check} — no matching filter pattern (may be non-compliant)");
        }
    }
    let _ = region; // suppress unused warning
    println!("\n  Note: missing patterns indicate CIS non-compliance, not test failure.");

    Ok(())
}
