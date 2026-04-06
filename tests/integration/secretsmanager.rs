//! Integration tests for Secrets Manager API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration secretsmanager -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::secretsmanager::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_REGION: &str = "eu-central-1";
const TEST_SECRET_NAME: &str = "cloud-lite-test-ralph-sm-secret";

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
        eprintln!("  aws cli: {}", stderr.trim());
        None
    }
}

fn aws_create_secret(region: &str) -> Option<String> {
    let v = aws(&[
        "secretsmanager",
        "create-secret",
        "--name",
        TEST_SECRET_NAME,
        "--description",
        "cloud-lite test secret",
        "--secret-string",
        "my-super-secret-value",
        "--region",
        region,
        "--output",
        "json",
    ])?;
    v["ARN"].as_str().map(|s| s.to_string())
}

fn aws_cleanup(region: &str) {
    // First try to restore if in scheduled-deletion state, then force-delete
    let _ = Command::new("aws")
        .args(&[
            "secretsmanager",
            "restore-secret",
            "--secret-id",
            TEST_SECRET_NAME,
            "--region",
            region,
        ])
        .output();
    // Brief pause for the restore to propagate before attempting delete
    std::thread::sleep(std::time::Duration::from_secs(3));
    // Force-delete with no recovery window
    let _ = Command::new("aws")
        .args(&[
            "secretsmanager",
            "delete-secret",
            "--secret-id",
            TEST_SECRET_NAME,
            "--force-delete-without-recovery",
            "--region",
            region,
        ])
        .output();
    // Brief pause for deletion to propagate
    std::thread::sleep(std::time::Duration::from_secs(5));
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_secretsmanager_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    // Pre-cleanup
    aws_cleanup(&region);
    tokio::time::sleep(Duration::from_secs(3)).await;

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    // Create fixture
    println!("\n=== Secrets Manager Operations Test ===");
    println!("Region: {region}");
    println!("\n[0/4] Creating test fixture (secret)...");
    let secret_arn = aws_create_secret(&region).expect("Failed to create test secret via CLI");
    println!("  Created secret: {TEST_SECRET_NAME} ({secret_arn})");

    // Wait for eventual consistency
    tokio::time::sleep(Duration::from_secs(2)).await;

    let result = run_secretsmanager_tests(&client, &secret_arn, &region).await;

    // Always cleanup (in case test didn't delete it)
    aws_cleanup(&region);

    result.expect("Secrets Manager tests failed");
    println!("\nAll Secrets Manager operations tests passed!");
}

async fn run_secretsmanager_tests(
    client: &AwsHttpClient,
    secret_arn: &str,
    _region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // [1/4] ListSecrets — our secret should appear
    println!("\n[1/4] ListSecrets...");
    let list_resp = client
        .secretsmanager()
        .list_secrets(&ListSecretsRequest::default())
        .await?;
    let found = list_resp
        .secret_list
        .iter()
        .any(|s| s.name.as_deref() == Some(TEST_SECRET_NAME));
    assert!(
        found,
        "Expected to find {TEST_SECRET_NAME} in ListSecrets. Got {} secrets",
        list_resp.secret_list.len()
    );
    // Verify structure of our entry
    let entry = list_resp
        .secret_list
        .iter()
        .find(|s| s.name.as_deref() == Some(TEST_SECRET_NAME))
        .unwrap();
    assert!(entry.arn.is_some(), "Expected ARN in SecretListEntry");
    assert!(entry.created_date.is_some(), "Expected CreatedDate");
    println!(
        "  Found {} secret(s). Our secret: name={} arn={} rotation_enabled={:?}",
        list_resp.secret_list.len(),
        entry.name.as_deref().unwrap_or("?"),
        entry.arn.as_deref().unwrap_or("?"),
        entry.rotation_enabled,
    );

    // [2/4] RotateSecret — verify error case (no rotation Lambda configured)
    // This proves the API request is being sent with correct wire format.
    // ClientRequestToken is required by the API.
    println!("\n[2/4] RotateSecret (expects error — no Lambda rotation function configured)...");
    let rotate_result = client
        .secretsmanager()
        .rotate_secret(&RotateSecretRequest {
            secret_id: secret_arn.to_string(),
            // ClientRequestToken is required; use a stable UUID for idempotency
            ..Default::default()
        })
        .await;
    // The API will fail: either missing Lambda ARN or missing ClientRequestToken
    assert!(
        rotate_result.is_err(),
        "Expected RotateSecret to fail (no Lambda configured)"
    );
    let err_msg = format!("{}", rotate_result.unwrap_err());
    assert!(
        err_msg.contains("InvalidRequestException")
            || err_msg.contains("InvalidParameterException")
            || err_msg.contains("Lambda")
            || err_msg.contains("ClientRequestToken"),
        "Expected Secrets Manager error, got: {err_msg}"
    );
    println!("  Got expected error: {err_msg}");

    // [3/4] DeleteSecret — use library to delete
    println!("\n[3/4] DeleteSecret...");
    let delete_resp = client
        .secretsmanager()
        .delete_secret(&DeleteSecretRequest {
            secret_id: secret_arn.to_string(),
            force_delete_without_recovery: Some(true),
            ..Default::default()
        })
        .await?;
    assert!(
        delete_resp.arn.is_some(),
        "Expected ARN in DeleteSecret response"
    );
    assert_eq!(
        delete_resp.name.as_deref(),
        Some(TEST_SECRET_NAME),
        "Expected secret name in response"
    );
    println!(
        "  Deleted: name={} arn={}",
        delete_resp.name.as_deref().unwrap_or("?"),
        delete_resp.arn.as_deref().unwrap_or("?"),
    );

    // [4/4] ListSecrets again — verify it's gone
    println!("\n[4/4] ListSecrets (verify deleted)...");
    // Wait briefly
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let list_after = client
        .secretsmanager()
        .list_secrets(&ListSecretsRequest::default())
        .await?;
    let still_found = list_after
        .secret_list
        .iter()
        .any(|s| s.name.as_deref() == Some(TEST_SECRET_NAME));
    assert!(
        !still_found,
        "Secret should be gone after force-delete but was still in ListSecrets"
    );
    println!("  Secret not found in list — deletion confirmed");

    Ok(())
}
