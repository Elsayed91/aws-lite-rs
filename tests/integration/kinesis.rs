//! Integration tests for Kinesis API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration kinesis -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::kinesis::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_REGION: &str = "eu-central-1";
const TEST_STREAM_NAME: &str = "cloud-lite-test-ralph-kinesis-stream";

// --- CLI Helpers ---

fn aws(args: &[&str]) -> Option<serde_json::Value> {
    let output = Command::new("aws")
        .args(args)
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(&stdout).ok()
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("  aws cli: {stderr}");
        None
    }
}

/// Create a Kinesis PROVISIONED stream (1 shard) and return when confirmed created.
fn aws_create_stream(name: &str, region: &str) {
    let _ = Command::new("aws")
        .args(&[
            "kinesis",
            "create-stream",
            "--stream-name",
            name,
            "--shard-count",
            "1",
            "--region",
            region,
        ])
        .output();
}

/// Delete a Kinesis stream; ignore errors (e.g. already gone).
fn aws_delete_stream(name: &str, region: &str) {
    let _ = Command::new("aws")
        .args(&[
            "kinesis",
            "delete-stream",
            "--stream-name",
            name,
            "--region",
            region,
        ])
        .output();
}

/// Wait until the stream reaches ACTIVE status (up to ~60s).
async fn wait_for_active(name: &str, region: &str) -> bool {
    for _ in 0..30 {
        let v = aws(&[
            "kinesis",
            "describe-stream-summary",
            "--stream-name",
            name,
            "--region",
            region,
            "--output",
            "json",
        ]);
        if let Some(v) = v {
            let status = v["StreamDescriptionSummary"]["StreamStatus"]
                .as_str()
                .unwrap_or("");
            if status == "ACTIVE" {
                return true;
            }
            eprintln!("  stream status: {status}, waiting...");
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    false
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_kinesis_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    // Pre-cleanup: delete any leftover stream from a previous test run
    println!("\n=== Kinesis Operations Test ===");
    println!("Region: {region}");
    println!("\n[pre-cleanup] Deleting any leftover test stream...");
    aws_delete_stream(TEST_STREAM_NAME, &region);
    // Brief wait for deletion to register
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Create fixture stream via CLI
    println!("\n[fixture] Creating PROVISIONED Kinesis stream (1 shard)...");
    aws_create_stream(TEST_STREAM_NAME, &region);

    // Wait for ACTIVE status
    let active = wait_for_active(TEST_STREAM_NAME, &region).await;
    assert!(active, "Test stream did not become ACTIVE within timeout");
    println!("  Stream is ACTIVE: {TEST_STREAM_NAME}");

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_kinesis_tests(&client, &region).await;

    // Always cleanup
    println!("\n[cleanup] Deleting test stream...");
    aws_delete_stream(TEST_STREAM_NAME, &region);

    result.expect("Kinesis operations tests failed");
    println!("\nAll Kinesis operations tests passed!");
}

async fn run_kinesis_tests(
    client: &AwsHttpClient,
    _region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // [1/5] ListStreams — verify our stream appears
    println!("\n[1/5] ListStreams...");
    let list_resp = client
        .kinesis()
        .list_streams(&ListStreamsInput::default())
        .await?;

    // The response may use StreamNames or StreamSummaries depending on account/SDK version
    let found_in_names = list_resp.stream_names.iter().any(|n| n == TEST_STREAM_NAME);
    let found_in_summaries = list_resp
        .stream_summaries
        .iter()
        .any(|s| s.stream_name == TEST_STREAM_NAME);

    assert!(
        found_in_names || found_in_summaries,
        "Expected to find {} in ListStreams, got names={:?} summaries_count={}",
        TEST_STREAM_NAME,
        list_resp.stream_names,
        list_resp.stream_summaries.len(),
    );
    println!(
        "  Found {} stream name(s), {} stream summary(ies)",
        list_resp.stream_names.len(),
        list_resp.stream_summaries.len(),
    );
    if let Some(summary) = list_resp
        .stream_summaries
        .iter()
        .find(|s| s.stream_name == TEST_STREAM_NAME)
    {
        assert!(
            !summary.stream_arn.is_empty(),
            "Expected non-empty StreamARN"
        );
        assert!(
            !summary.stream_status.is_empty(),
            "Expected non-empty StreamStatus"
        );
        println!(
            "  Stream: name={} arn={} status={}",
            summary.stream_name, summary.stream_arn, summary.stream_status
        );
    }

    // [2/5] DescribeStreamSummary — verify full metadata
    println!("\n[2/5] DescribeStreamSummary...");
    let desc_resp = client
        .kinesis()
        .describe_stream_summary(&DescribeStreamSummaryInput {
            stream_name: Some(TEST_STREAM_NAME.to_string()),
            ..Default::default()
        })
        .await?;

    let summary = &desc_resp.stream_description_summary;
    assert_eq!(summary.stream_name, TEST_STREAM_NAME, "StreamName mismatch");
    assert!(
        !summary.stream_arn.is_empty(),
        "Expected non-empty StreamARN"
    );
    assert_eq!(
        summary.stream_status.as_str(),
        "ACTIVE",
        "Expected ACTIVE stream status"
    );
    assert!(
        summary.open_shard_count >= 1,
        "Expected at least 1 open shard, got {}",
        summary.open_shard_count
    );
    assert!(
        summary.stream_creation_timestamp > 0.0,
        "Expected positive StreamCreationTimestamp"
    );
    println!(
        "  Stream: name={} arn={} status={} shards={} created={}",
        summary.stream_name,
        summary.stream_arn,
        summary.stream_status,
        summary.open_shard_count,
        summary.stream_creation_timestamp,
    );
    if let Some(mode) = &summary.stream_mode_details {
        println!("  Mode: {}", mode.stream_mode);
    }

    // Capture ARN for UpdateStreamMode
    let stream_arn = summary.stream_arn.clone();

    // [3/5] UpdateStreamMode — switch to ON_DEMAND
    println!("\n[3/5] UpdateStreamMode (PROVISIONED -> ON_DEMAND)...");
    client
        .kinesis()
        .update_stream_mode(&UpdateStreamModeInput {
            stream_arn: stream_arn.clone(),
            stream_mode_details: StreamModeDetails {
                stream_mode: "ON_DEMAND".to_string(),
            },
        })
        .await?;
    println!("  UpdateStreamMode succeeded");

    // Brief wait for mode change to propagate
    tokio::time::sleep(Duration::from_secs(3)).await;

    // [4/5] DescribeStreamSummary — verify mode changed to ON_DEMAND
    println!("\n[4/5] DescribeStreamSummary (verify ON_DEMAND mode)...");
    let desc_after = client
        .kinesis()
        .describe_stream_summary(&DescribeStreamSummaryInput {
            stream_name: Some(TEST_STREAM_NAME.to_string()),
            ..Default::default()
        })
        .await?;
    let mode_after = desc_after
        .stream_description_summary
        .stream_mode_details
        .as_ref()
        .map(|m| m.stream_mode.as_str())
        .unwrap_or("PROVISIONED");
    assert_eq!(
        mode_after, "ON_DEMAND",
        "Expected ON_DEMAND stream mode after UpdateStreamMode"
    );
    println!("  Stream mode is now: {mode_after}");

    // [5/5] DeleteStream — via library
    println!("\n[5/5] DeleteStream...");
    client
        .kinesis()
        .delete_stream(&DeleteStreamInput {
            stream_name: Some(TEST_STREAM_NAME.to_string()),
            ..Default::default()
        })
        .await?;
    println!("  DeleteStream succeeded");

    // Brief pause then verify gone via ListStreams
    tokio::time::sleep(Duration::from_secs(3)).await;
    let list_after = client
        .kinesis()
        .list_streams(&ListStreamsInput::default())
        .await?;
    let still_present = list_after
        .stream_names
        .iter()
        .any(|n| n == TEST_STREAM_NAME)
        || list_after
            .stream_summaries
            .iter()
            .any(|s| s.stream_name == TEST_STREAM_NAME && s.stream_status != "DELETING");
    assert!(
        !still_present,
        "Stream {} should be gone or DELETING after DeleteStream",
        TEST_STREAM_NAME
    );
    println!("  Stream is no longer ACTIVE in list (deleted or DELETING)");

    Ok(())
}
