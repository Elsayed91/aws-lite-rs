//! Integration tests for AWS IAM Access Analyzer API.
//!
//! Needed for AWS CIS benchmark checks:
//!   - CIS 2.19 (aws_iam_access_analyzer): confirm at least one ACTIVE analyzer
//!     exists per region (ACCOUNT or ORGANIZATION type).
//!
//! No resources created — read-only listing of existing analyzers.
//!
//! Run with:
//! ```sh
//! AWS_REGION=us-east-1 \
//!   cargo test -p aws-lite --test integration accessanalyzer -- --ignored --nocapture
//! ```

use aws_lite::AwsHttpClient;
use std::env;

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into())
}

#[tokio::test]
#[ignore = "requires AWS credentials (AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY or default profile)"]
async fn test_access_analyzer_list() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let aa = client.accessanalyzer();

    println!("\n=== AWS Access Analyzer Integration Test ===");
    println!("Region: {}", region);

    // [1/1] List all analyzers in this region
    println!("\n[1/1] Listing IAM Access Analyzers...");
    let analyzers = aa.list_analyzers().await?;
    println!("  Found {} analyzer(s)", analyzers.len());

    for a in &analyzers {
        println!(
            "  - {} (type: {}, status: {}, arn: {})",
            a.name, a.r#type, a.status, a.arn
        );
        // Verify structure
        assert!(!a.arn.is_empty(), "arn should not be empty");
        assert!(!a.name.is_empty(), "name should not be empty");
        assert!(!a.r#type.is_empty(), "type should not be empty");
        assert!(!a.status.is_empty(), "status should not be empty");
    }

    // CIS 2.19 check: warn if no ACTIVE analyzer found
    let active_count = analyzers.iter().filter(|a| a.status == "ACTIVE").count();

    if active_count == 0 {
        println!(
            "\n  WARNING (CIS 2.19): No ACTIVE analyzer found in region {}.",
            region
        );
        println!(
            "  To remediate: create an account-level analyzer (ACCOUNT or ORGANIZATION type)."
        );
    } else {
        println!(
            "\n  CIS 2.19: {} ACTIVE analyzer(s) found in region {}. Compliant.",
            active_count, region
        );
    }

    println!("\nAll Access Analyzer tests passed!");
    Ok(())
}
