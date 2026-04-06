//! Integration tests for KMS API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration kms -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::kms::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_REGION: &str = "eu-central-1";
const TEST_KEY_DESCRIPTION: &str = "cloud-lite-test-ralph-kms-key";

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
        eprintln!("  aws cli error: {stderr}");
        None
    }
}

fn aws_create_key(region: &str) -> Option<String> {
    let v = aws(&[
        "kms",
        "create-key",
        "--description",
        TEST_KEY_DESCRIPTION,
        "--key-usage",
        "ENCRYPT_DECRYPT",
        "--key-spec",
        "SYMMETRIC_DEFAULT",
        "--region",
        region,
        "--output",
        "json",
    ])?;
    v["KeyMetadata"]["KeyId"].as_str().map(|s| s.to_string())
}

fn aws_schedule_key_deletion(key_id: &str, region: &str) {
    // Minimum waiting period is 7 days
    let _ = Command::new("aws")
        .args(&[
            "kms",
            "schedule-key-deletion",
            "--key-id",
            key_id,
            "--pending-window-in-days",
            "7",
            "--region",
            region,
        ])
        .output();
}

fn aws_cleanup_keys(region: &str) {
    // List all keys and schedule deletion for our test keys
    let v = aws(&["kms", "list-keys", "--region", region, "--output", "json"]);
    if let Some(v) = v {
        if let Some(keys) = v["Keys"].as_array() {
            for key in keys {
                if let Some(key_id) = key["KeyId"].as_str() {
                    // Describe to check if it's our test key
                    let desc = aws(&[
                        "kms",
                        "describe-key",
                        "--key-id",
                        key_id,
                        "--region",
                        region,
                        "--output",
                        "json",
                    ]);
                    if let Some(desc) = desc {
                        let description = desc["KeyMetadata"]["Description"].as_str().unwrap_or("");
                        let key_state = desc["KeyMetadata"]["KeyState"].as_str().unwrap_or("");
                        if description == TEST_KEY_DESCRIPTION && key_state != "PendingDeletion" {
                            eprintln!("  Scheduling deletion of test key: {key_id}");
                            aws_schedule_key_deletion(key_id, region);
                        }
                    }
                }
            }
        }
    }
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_kms_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    // Pre-cleanup: schedule deletion of any leftover test keys
    aws_cleanup_keys(&region);
    tokio::time::sleep(Duration::from_secs(2)).await;

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    // Create a CMK fixture
    println!("\n=== KMS Operations Test ===");
    println!("Region: {region}");
    println!("\n[0/5] Creating test fixture (customer-managed KMS key)...");
    let key_id = aws_create_key(&region).expect("Failed to create test KMS key via CLI");
    println!("  Created CMK: {key_id}");

    // Wait briefly for eventual consistency
    tokio::time::sleep(Duration::from_secs(2)).await;

    let result = run_kms_tests(&client, &key_id, &region).await;

    // Always cleanup — schedule key for deletion
    println!("\n[cleanup] Scheduling CMK deletion...");
    aws_schedule_key_deletion(&key_id, &region);

    result.expect("KMS operations tests failed");
    println!("\nAll KMS operations tests passed!");
}

async fn run_kms_tests(
    client: &AwsHttpClient,
    test_key_id: &str,
    _region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // [1/5] ListKeys — our new key should appear
    println!("\n[1/5] ListKeys...");
    let list_resp = client.kms().list_keys(&ListKeysRequest::default()).await?;
    let found_key = list_resp
        .keys
        .iter()
        .any(|k| k.key_id.as_deref() == Some(test_key_id));
    assert!(
        found_key,
        "Expected to find our test key in ListKeys, key_id={}. Got {} keys",
        test_key_id,
        list_resp.keys.len()
    );
    // Verify structure of a key entry
    let first = &list_resp.keys[0];
    assert!(first.key_id.is_some(), "Expected key_id in KeyListEntry");
    assert!(first.key_arn.is_some(), "Expected key_arn in KeyListEntry");
    println!(
        "  Found {} key(s), includes our test key. Sample: id={} arn={}",
        list_resp.keys.len(),
        first.key_id.as_deref().unwrap_or("?"),
        first.key_arn.as_deref().unwrap_or("?"),
    );

    // [2/5] DescribeKey — verify metadata of our CMK
    println!("\n[2/5] DescribeKey...");
    let desc_resp = client
        .kms()
        .describe_key(&DescribeKeyRequest {
            key_id: test_key_id.to_string(),
        })
        .await?;
    let meta = desc_resp
        .key_metadata
        .as_ref()
        .expect("Expected KeyMetadata");
    assert_eq!(meta.key_id.as_str(), test_key_id, "KeyId mismatch");
    assert!(meta.arn.is_some(), "Expected Arn in KeyMetadata");
    assert_eq!(
        meta.enabled,
        Some(true),
        "Expected newly created key to be enabled"
    );
    assert_eq!(
        meta.key_manager.as_deref(),
        Some("CUSTOMER"),
        "Expected CUSTOMER key manager"
    );
    assert_eq!(
        meta.key_state.as_deref(),
        Some("Enabled"),
        "Expected key state to be Enabled"
    );
    assert!(meta.creation_date.is_some(), "Expected CreationDate");
    println!(
        "  Key: id={} arn={} state={} manager={} enabled={:?}",
        meta.key_id.as_str(),
        meta.arn.as_deref().unwrap_or("?"),
        meta.key_state.as_deref().unwrap_or("?"),
        meta.key_manager.as_deref().unwrap_or("?"),
        meta.enabled,
    );

    // [3/5] GetKeyRotationStatus — check initial rotation state
    println!("\n[3/5] GetKeyRotationStatus...");
    let rotation_resp = client
        .kms()
        .get_key_rotation_status(&GetKeyRotationStatusRequest {
            key_id: test_key_id.to_string(),
        })
        .await?;
    // A newly created key defaults to rotation disabled
    println!(
        "  Rotation enabled: {:?}, rotation_period_in_days: {:?}",
        rotation_resp.key_rotation_enabled, rotation_resp.rotation_period_in_days,
    );
    // Just verify we can read the field (could be true or false depending on account defaults)
    assert!(
        rotation_resp.key_rotation_enabled.is_some(),
        "Expected KeyRotationEnabled in response"
    );

    // [4/5] EnableKeyRotation — enable automatic rotation
    println!("\n[4/5] EnableKeyRotation...");
    client
        .kms()
        .enable_key_rotation(&EnableKeyRotationRequest {
            key_id: test_key_id.to_string(),
            ..Default::default()
        })
        .await?;
    println!("  EnableKeyRotation succeeded (no error)");

    // [5/5] GetKeyRotationStatus again — verify rotation is now enabled
    println!("\n[5/5] GetKeyRotationStatus (verify rotation enabled)...");
    let rotation_after = client
        .kms()
        .get_key_rotation_status(&GetKeyRotationStatusRequest {
            key_id: test_key_id.to_string(),
        })
        .await?;
    assert_eq!(
        rotation_after.key_rotation_enabled,
        Some(true),
        "Expected key rotation to be enabled after EnableKeyRotation"
    );
    println!(
        "  Rotation enabled: {:?}, period_in_days: {:?}, next_rotation: {:?}",
        rotation_after.key_rotation_enabled,
        rotation_after.rotation_period_in_days,
        rotation_after.next_rotation_date,
    );

    Ok(())
}
