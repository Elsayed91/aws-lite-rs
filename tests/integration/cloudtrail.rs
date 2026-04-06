//! Integration tests for CloudTrail API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration cloudtrail -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::cloudtrail::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_REGION: &str = "eu-central-1";
const TEST_TRAIL_NAME: &str = "cloud-lite-test-ralph-trail";
const TEST_BUCKET_NAME: &str = "cloud-lite-test-ralph-trail-logs";

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
        None
    }
}

fn aws_cleanup(args: &[&str]) {
    let _ = Command::new("aws").args(args).output();
}

fn get_account_id() -> String {
    let v =
        aws(&["sts", "get-caller-identity", "--output", "json"]).expect("Failed to get account ID");
    v["Account"].as_str().unwrap_or("").to_string()
}

fn aws_create_s3_bucket(region: &str) {
    if region == "us-east-1" {
        aws(&[
            "s3api",
            "create-bucket",
            "--bucket",
            TEST_BUCKET_NAME,
            "--region",
            region,
            "--output",
            "json",
        ]);
    } else {
        aws(&[
            "s3api",
            "create-bucket",
            "--bucket",
            TEST_BUCKET_NAME,
            "--region",
            region,
            "--create-bucket-configuration",
            &format!("LocationConstraint={region}"),
            "--output",
            "json",
        ]);
    }
}

fn aws_put_bucket_policy(account_id: &str, region: &str) {
    let policy = serde_json::json!({
        "Version": "2012-10-17",
        "Statement": [
            {
                "Sid": "AWSCloudTrailAclCheck",
                "Effect": "Allow",
                "Principal": {
                    "Service": "cloudtrail.amazonaws.com"
                },
                "Action": "s3:GetBucketAcl",
                "Resource": format!("arn:aws:s3:::{TEST_BUCKET_NAME}")
            },
            {
                "Sid": "AWSCloudTrailWrite",
                "Effect": "Allow",
                "Principal": {
                    "Service": "cloudtrail.amazonaws.com"
                },
                "Action": "s3:PutObject",
                "Resource": format!("arn:aws:s3:::{TEST_BUCKET_NAME}/AWSLogs/{account_id}/*"),
                "Condition": {
                    "StringEquals": {
                        "s3:x-amz-acl": "bucket-owner-full-control"
                    }
                }
            }
        ]
    });
    let policy_str = serde_json::to_string(&policy).unwrap();
    aws(&[
        "s3api",
        "put-bucket-policy",
        "--bucket",
        TEST_BUCKET_NAME,
        "--policy",
        &policy_str,
        "--region",
        region,
    ]);
}

fn aws_delete_s3_bucket(region: &str) {
    // Empty the bucket first
    aws_cleanup(&[
        "s3",
        "rm",
        &format!("s3://{TEST_BUCKET_NAME}"),
        "--recursive",
        "--region",
        region,
    ]);
    aws_cleanup(&[
        "s3api",
        "delete-bucket",
        "--bucket",
        TEST_BUCKET_NAME,
        "--region",
        region,
    ]);
}

fn aws_create_trail(region: &str) {
    aws(&[
        "cloudtrail",
        "create-trail",
        "--name",
        TEST_TRAIL_NAME,
        "--s3-bucket-name",
        TEST_BUCKET_NAME,
        "--region",
        region,
        "--output",
        "json",
    ]);
    // Start logging on the trail
    aws(&[
        "cloudtrail",
        "start-logging",
        "--name",
        TEST_TRAIL_NAME,
        "--region",
        region,
    ]);
}

fn aws_delete_trail(region: &str) {
    aws_cleanup(&[
        "cloudtrail",
        "delete-trail",
        "--name",
        TEST_TRAIL_NAME,
        "--region",
        region,
    ]);
}

fn cleanup_all(region: &str) {
    aws_delete_trail(region);
    std::thread::sleep(Duration::from_secs(2));
    aws_delete_s3_bucket(region);
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_cloudtrail_write_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    // Pre-cleanup
    cleanup_all(&region);
    tokio::time::sleep(Duration::from_secs(3)).await;

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_cloudtrail_write_tests(&client, &region).await;

    // Always cleanup
    cleanup_all(&region);

    result.expect("CloudTrail write operations tests failed");
    println!("\nAll CloudTrail write operations tests passed!");
}

async fn run_cloudtrail_write_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== CloudTrail Write Operations Test ===");
    println!("Region: {region}");

    // [0/3] Create fixtures: S3 bucket + CloudTrail trail
    println!("\n[0/3] Creating test fixtures...");
    let account_id = get_account_id();
    aws_create_s3_bucket(region);
    tokio::time::sleep(Duration::from_secs(2)).await;
    aws_put_bucket_policy(&account_id, region);
    aws_create_trail(region);
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("  Created S3 bucket: {TEST_BUCKET_NAME}");
    println!("  Created trail: {TEST_TRAIL_NAME}");

    // [1/3] UpdateTrail — toggle include_global_service_events
    println!("\n[1/3] UpdateTrail...");
    let update_resp = client
        .cloudtrail()
        .update_trail(&UpdateTrailRequest {
            name: TEST_TRAIL_NAME.to_string(),
            include_global_service_events: Some(false),
            ..Default::default()
        })
        .await?;
    assert!(
        update_resp.trail_arn.is_some(),
        "Expected TrailARN in UpdateTrail response"
    );
    assert_eq!(
        update_resp.name.as_deref(),
        Some(TEST_TRAIL_NAME),
        "Expected trail name in response"
    );
    assert_eq!(
        update_resp.include_global_service_events,
        Some(false),
        "Expected include_global_service_events to be false after update"
    );
    println!(
        "  Updated: name={} arn={} include_global_service_events={:?}",
        update_resp.name.as_deref().unwrap_or("?"),
        update_resp.trail_arn.as_deref().unwrap_or("?"),
        update_resp.include_global_service_events,
    );

    // [2/3] UpdateTrail — restore include_global_service_events
    println!("\n[2/3] UpdateTrail (restore)...");
    let restore_resp = client
        .cloudtrail()
        .update_trail(&UpdateTrailRequest {
            name: TEST_TRAIL_NAME.to_string(),
            include_global_service_events: Some(true),
            ..Default::default()
        })
        .await?;
    assert_eq!(
        restore_resp.include_global_service_events,
        Some(true),
        "Expected include_global_service_events restored to true"
    );
    println!(
        "  Restored: include_global_service_events={:?}",
        restore_resp.include_global_service_events,
    );

    // [3/3] DeleteTrail
    println!("\n[3/3] DeleteTrail...");
    let _delete_resp = client
        .cloudtrail()
        .delete_trail(&DeleteTrailRequest {
            name: TEST_TRAIL_NAME.to_string(),
        })
        .await?;
    // Verify the trail is gone by calling DescribeTrails
    tokio::time::sleep(Duration::from_secs(2)).await;
    let verify_resp = client
        .cloudtrail()
        .describe_trails(&DescribeTrailsRequest {
            trail_name_list: vec![TEST_TRAIL_NAME.to_string()],
            ..Default::default()
        })
        .await?;
    let still_exists = verify_resp
        .trail_list
        .iter()
        .any(|t| t.name.as_deref() == Some(TEST_TRAIL_NAME));
    assert!(
        !still_exists,
        "Trail should be deleted but was found in DescribeTrails"
    );
    println!("  Trail deleted and verified gone");

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_cloudtrail_read_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    // Pre-cleanup
    cleanup_all(&region);
    tokio::time::sleep(Duration::from_secs(3)).await;

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_cloudtrail_read_tests(&client, &region).await;

    // Always cleanup
    cleanup_all(&region);

    result.expect("CloudTrail read operations tests failed");
    println!("\nAll CloudTrail read operations tests passed!");
}

async fn run_cloudtrail_read_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== CloudTrail Read Operations Test ===");
    println!("Region: {region}");

    // [0/5] Create fixtures: S3 bucket + CloudTrail trail
    println!("\n[0/5] Creating test fixtures...");
    let account_id = get_account_id();
    aws_create_s3_bucket(region);
    tokio::time::sleep(Duration::from_secs(2)).await;
    aws_put_bucket_policy(&account_id, region);
    aws_create_trail(region);
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("  Created S3 bucket: {TEST_BUCKET_NAME}");
    println!("  Created trail: {TEST_TRAIL_NAME}");

    // [1/5] DescribeTrails
    println!("\n[1/5] DescribeTrails...");
    let desc_resp = client
        .cloudtrail()
        .describe_trails(&DescribeTrailsRequest {
            trail_name_list: vec![TEST_TRAIL_NAME.to_string()],
            ..Default::default()
        })
        .await?;
    assert!(
        !desc_resp.trail_list.is_empty(),
        "Expected trail list to contain our trail"
    );
    let trail = &desc_resp.trail_list[0];
    assert_eq!(trail.name.as_deref(), Some(TEST_TRAIL_NAME));
    assert!(trail.trail_arn.is_some(), "Expected TrailARN");
    assert!(trail.home_region.is_some(), "Expected HomeRegion");
    assert_eq!(trail.s3_bucket_name.as_deref(), Some(TEST_BUCKET_NAME));
    assert_eq!(
        trail.is_multi_region_trail,
        Some(false),
        "Should not be multi-region"
    );
    assert_eq!(
        trail.is_organization_trail,
        Some(false),
        "Should not be org trail"
    );
    println!(
        "  Trail: name={} arn={} home_region={} s3_bucket={} multi_region={:?}",
        trail.name.as_deref().unwrap_or("?"),
        trail.trail_arn.as_deref().unwrap_or("?"),
        trail.home_region.as_deref().unwrap_or("?"),
        trail.s3_bucket_name.as_deref().unwrap_or("?"),
        trail.is_multi_region_trail,
    );

    // [2/5] GetTrailStatus
    println!("\n[2/5] GetTrailStatus...");
    let status_resp = client
        .cloudtrail()
        .get_trail_status(&GetTrailStatusRequest {
            name: TEST_TRAIL_NAME.to_string(),
        })
        .await?;
    assert_eq!(
        status_resp.is_logging,
        Some(true),
        "Expected trail to be logging (we called start-logging)"
    );
    assert!(
        status_resp.start_logging_time.is_some(),
        "Expected StartLoggingTime"
    );
    println!(
        "  Status: is_logging={:?} start_logging_time={:?}",
        status_resp.is_logging, status_resp.start_logging_time,
    );

    // [3/5] GetEventSelectors
    println!("\n[3/5] GetEventSelectors...");
    let selectors_resp = client
        .cloudtrail()
        .get_event_selectors(&GetEventSelectorsRequest {
            trail_name: TEST_TRAIL_NAME.to_string(),
        })
        .await?;
    assert!(
        selectors_resp.trail_arn.is_some(),
        "Expected TrailARN in response"
    );
    // Default trail should have management event selectors
    println!(
        "  TrailARN={} event_selectors={} advanced_selectors={}",
        selectors_resp.trail_arn.as_deref().unwrap_or("?"),
        selectors_resp.event_selectors.len(),
        selectors_resp.advanced_event_selectors.len(),
    );

    // [4/5] DescribeTrails with all trails (no filter)
    println!("\n[4/5] DescribeTrails (list all)...");
    let all_trails_resp = client
        .cloudtrail()
        .describe_trails(&DescribeTrailsRequest::default())
        .await?;
    let found = all_trails_resp
        .trail_list
        .iter()
        .any(|t| t.name.as_deref() == Some(TEST_TRAIL_NAME));
    assert!(
        found,
        "Expected to find our trail in full list, got: {:?}",
        all_trails_resp
            .trail_list
            .iter()
            .map(|t| t.name.as_deref())
            .collect::<Vec<_>>()
    );
    println!(
        "  Found {} trail(s) total, includes {}",
        all_trails_resp.trail_list.len(),
        TEST_TRAIL_NAME
    );

    // [5/5] GetTrailStatus error case
    println!("\n[5/5] GetTrailStatus error case...");
    let err = client
        .cloudtrail()
        .get_trail_status(&GetTrailStatusRequest {
            name: "cloud-lite-test-nonexistent-trail".to_string(),
        })
        .await;
    assert!(err.is_err(), "Expected error for non-existent trail");
    let err_msg = format!("{}", err.unwrap_err());
    assert!(
        err_msg.contains("TrailNotFoundException")
            || err_msg.contains("does not exist")
            || err_msg.contains("not found"),
        "Expected TrailNotFoundException, got: {err_msg}",
    );
    println!("  Got expected error: {err_msg}");

    Ok(())
}
