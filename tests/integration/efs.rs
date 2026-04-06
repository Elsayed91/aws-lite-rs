//! Integration tests for AWS Elastic File System (EFS) API.
//!
//! Needed for AWS CIS benchmark checks:
//!   - CIS 3.3.1 (aws_efs_encryption): verify all EFS file systems have
//!     encryption at rest enabled (`Encrypted: true`).
//!
//! No resources created — read-only listing of existing file systems.
//!
//! Run with:
//! ```sh
//! AWS_REGION=us-east-1 \
//!   cargo test -p aws-lite --test integration efs -- --ignored --nocapture
//! ```

use aws_lite::AwsHttpClient;
use std::env;

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into())
}

#[tokio::test]
#[ignore = "requires AWS credentials (AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY or default profile)"]
async fn test_efs_list_file_systems() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let efs = client.efs();

    println!("\n=== AWS EFS Integration Test ===");
    println!("Region: {}", region);

    // [1/2] List all file systems via paginating helper
    println!("\n[1/2] Listing all EFS file systems...");
    let file_systems = efs.list_all_file_systems().await?;
    println!("  Found {} file system(s)", file_systems.len());

    let mut unencrypted = Vec::new();
    for fs in &file_systems {
        let encrypted = fs.encrypted.unwrap_or(false);
        println!(
            "  - {} (state: {}, encrypted: {}, performance: {})",
            fs.file_system_id, fs.life_cycle_state, encrypted, fs.performance_mode,
        );
        if let Some(arn) = &fs.file_system_arn {
            println!("    ARN: {}", arn);
        }
        if let Some(kms) = &fs.kms_key_id {
            println!("    KMS key: {}", kms);
        }
        assert!(
            !fs.file_system_id.is_empty(),
            "file_system_id should not be empty"
        );
        assert!(
            !fs.life_cycle_state.is_empty(),
            "life_cycle_state should not be empty"
        );
        assert!(
            !fs.performance_mode.is_empty(),
            "performance_mode should not be empty"
        );
        if !encrypted {
            unencrypted.push(fs.file_system_id.clone());
        }
    }

    // [2/2] CIS 3.3.1 compliance summary
    println!("\n[2/2] CIS 3.3.1 encryption compliance summary...");
    if file_systems.is_empty() {
        println!("  No EFS file systems found in region {}.", region);
    } else if unencrypted.is_empty() {
        println!(
            "  CIS 3.3.1: All {} file system(s) are encrypted at rest. Compliant.",
            file_systems.len()
        );
    } else {
        println!(
            "  WARNING (CIS 3.3.1): {} file system(s) are NOT encrypted at rest: {:?}",
            unencrypted.len(),
            unencrypted
        );
        println!("  To remediate: EFS encryption can only be enabled at creation time.");
    }

    println!("\nAll EFS tests passed!");
    Ok(())
}
