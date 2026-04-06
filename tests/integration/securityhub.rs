//! Integration tests for AWS Security Hub API.
//!
//! Needed for AWS CIS benchmark checks:
//!   - CIS 5.16 (aws_security_hub_enabled): verify Security Hub is enabled
//!     per account/region. DescribeHub succeeds → enabled;
//!     ResourceNotFoundException → not enabled.
//!
//! No resources created — read-only.
//!
//! Run with:
//! ```sh
//! AWS_REGION=us-east-1 \
//!   cargo test -p aws-lite --test integration securityhub -- --ignored --nocapture
//! ```

use aws_lite::AwsHttpClient;
use std::env;

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into())
}

#[tokio::test]
#[ignore = "requires AWS credentials (AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY or default profile)"]
async fn test_security_hub_describe() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let sh = client.securityhub();

    println!("\n=== AWS Security Hub Integration Test ===");
    println!("Region: {}", region);

    // [1/1] Check if Security Hub is enabled
    println!("\n[1/1] Checking Security Hub status...");
    let enabled = sh.is_enabled().await?;

    if enabled {
        let hub = sh.describe_hub().await?;
        println!("  Security Hub is ENABLED");
        if let Some(arn) = &hub.hub_arn {
            println!("  Hub ARN: {}", arn);
        }
        if let Some(subscribed_at) = &hub.subscribed_at {
            println!("  Subscribed at: {}", subscribed_at);
        }
        if let Some(auto_enable) = hub.auto_enable_controls {
            println!("  AutoEnableControls: {}", auto_enable);
        }
        if let Some(generator) = &hub.control_finding_generator {
            println!("  ControlFindingGenerator: {}", generator);
        }
        println!(
            "\n  CIS 5.16: Security Hub is enabled in region {}. Compliant.",
            region
        );
    } else {
        println!("  Security Hub is NOT ENABLED");
        println!(
            "\n  WARNING (CIS 5.16): Security Hub is not enabled in region {}.",
            region
        );
        println!("  To remediate: enable Security Hub via the AWS console or CLI.");
    }

    println!("\nAll Security Hub tests passed!");
    Ok(())
}
