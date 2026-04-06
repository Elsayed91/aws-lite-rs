//! Integration tests for AWS STS API
//!
//! Tests GetCallerIdentity and AssumeRole against the real AWS STS API.
//!
//! STS is a global service — the region is used for SigV4 signing
//! but requests go to sts.amazonaws.com (or regional endpoints).
//!
//! Run with:
//!   AWS_PROFILE=<profile> AWS_REGION=us-east-1 \
//!     cargo test -p aws-lite --test integration sts -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use std::env;

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_get_caller_identity() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let sts = client.sts();

    println!("\n=== GetCallerIdentity ===");

    // 1. Call GetCallerIdentity
    println!("\n[1/2] Getting caller identity...");
    let response = sts.get_caller_identity().await?;
    println!("  Account: {:?}", response.account);
    println!("  Arn:     {:?}", response.arn);
    println!("  UserId:  {:?}", response.user_id);

    // 2. Verify response structure
    println!("\n[2/2] Verifying response fields...");
    let account = response.account.as_deref().expect("Account should be set");
    assert!(!account.is_empty(), "Account should not be empty");
    assert!(
        account.chars().all(|c| c.is_ascii_digit()),
        "Account should be numeric, got: {account}"
    );

    let arn = response.arn.as_deref().expect("Arn should be set");
    assert!(
        arn.starts_with("arn:aws:"),
        "Arn should start with arn:aws:, got: {arn}"
    );

    let user_id = response.user_id.as_deref().expect("UserId should be set");
    assert!(!user_id.is_empty(), "UserId should not be empty");
    println!("  All fields present and valid");

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_assume_role_error_case() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let sts = client.sts();

    println!("\n=== AssumeRole Error Cases ===");

    // 1. Try to assume a non-existent role (should fail)
    println!("\n[1/1] Assuming non-existent role...");
    let body = aws_lite::types::sts::AssumeRoleRequest {
        role_arn: "arn:aws:iam::000000000000:role/cloud-lite-test-ralph-nonexistent".to_string(),
        role_session_name: "cloud-lite-test-ralph-session".to_string(),
        ..Default::default()
    };
    let result = sts.assume_role(&body).await;
    assert!(result.is_err(), "non-existent role should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}
