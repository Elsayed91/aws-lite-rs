//! Integration tests for AWS S3 API
//!
//! Tests S3 read/write operations against the real AWS S3 API.
//!
//! Run with:
//!   AWS_REGION=eu-central-1 \
//!     cargo test -p aws-lite --test integration s3 -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::s3::{
    BucketLifecycleConfiguration, BucketLoggingStatus, LifecycleRule, LifecycleRuleFilter,
    LoggingEnabled, PublicAccessBlockConfiguration, ServerSideEncryptionByDefault,
    ServerSideEncryptionConfiguration, ServerSideEncryptionRule, VersioningConfiguration,
};
use std::env;
use std::process::Command;

const TEST_BUCKET: &str = "cloud-lite-test-s3-integration";

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "eu-central-1".into())
}

// -- CLI Helpers --------------------------------------------------------------

fn aws_create_bucket(bucket: &str, region: &str) {
    // Pre-cleanup: delete if exists
    aws_delete_bucket(bucket, region);

    let mut args = vec![
        "s3api".to_string(),
        "create-bucket".to_string(),
        "--bucket".to_string(),
        bucket.to_string(),
        "--region".to_string(),
        region.to_string(),
    ];

    // For non-us-east-1 regions, must specify LocationConstraint
    if region != "us-east-1" {
        args.push("--create-bucket-configuration".to_string());
        args.push(format!("LocationConstraint={region}"));
    }

    let output = Command::new("aws")
        .args(&args)
        .output()
        .expect("aws cli must be installed");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.contains("BucketAlreadyOwnedByYou") {
            panic!("Failed to create bucket {bucket}: {stderr}");
        }
    }
}

fn aws_delete_bucket(bucket: &str, region: &str) {
    // Delete all objects first (bucket must be empty)
    let _ = Command::new("aws")
        .args([
            "s3",
            "rm",
            &format!("s3://{bucket}"),
            "--recursive",
            "--region",
            region,
        ])
        .output();

    let _ = Command::new("aws")
        .args([
            "s3api",
            "delete-bucket",
            "--bucket",
            bucket,
            "--region",
            region,
        ])
        .output();
}

fn aws_get_bucket_policy(bucket: &str, region: &str) -> Option<String> {
    let output = Command::new("aws")
        .args([
            "s3api",
            "get-bucket-policy",
            "--bucket",
            bucket,
            "--region",
            region,
        ])
        .output()
        .expect("aws cli must be installed");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str::<serde_json::Value>(&stdout)
            .ok()
            .and_then(|v| v["Policy"].as_str().map(String::from))
    } else {
        None
    }
}

fn aws_get_public_access_block(bucket: &str, region: &str) -> Option<serde_json::Value> {
    let output = Command::new("aws")
        .args([
            "s3api",
            "get-public-access-block",
            "--bucket",
            bucket,
            "--region",
            region,
        ])
        .output()
        .expect("aws cli must be installed");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str::<serde_json::Value>(&stdout).ok()
    } else {
        None
    }
}

fn aws_get_bucket_lifecycle(bucket: &str, region: &str) -> Option<serde_json::Value> {
    let output = Command::new("aws")
        .args([
            "s3api",
            "get-bucket-lifecycle-configuration",
            "--bucket",
            bucket,
            "--region",
            region,
        ])
        .output()
        .expect("aws cli must be installed");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str::<serde_json::Value>(&stdout).ok()
    } else {
        None
    }
}

// -- Integration Tests --------------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_s3_remediation_operations() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;

    println!("\n=== S3 Remediation Operations ===");
    println!("Region: {region}");
    println!("Bucket: {TEST_BUCKET}");

    // Pre-cleanup + create fixture bucket
    println!("\n[0/8] Creating test bucket...");
    aws_create_bucket(TEST_BUCKET, &region);
    println!("  Created bucket: {TEST_BUCKET}");

    // Run all tests, capturing result
    let result = run_remediation_tests(&client).await;

    // Always cleanup
    println!("\n[cleanup] Deleting test bucket...");
    aws_delete_bucket(TEST_BUCKET, &region);
    println!("  Deleted bucket: {TEST_BUCKET}");

    // Propagate after cleanup
    result
}

async fn run_remediation_tests(client: &AwsHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let s3 = client.s3();

    // -- PutBucketPolicy --
    println!("\n[1/8] PutBucketPolicy — setting a bucket policy...");
    let account_id = get_account_id(&region);
    let policy = serde_json::json!({
        "Version": "2012-10-17",
        "Statement": [{
            "Sid": "CloudLiteTestPolicy",
            "Effect": "Deny",
            "Principal": "*",
            "Action": "s3:GetObject",
            "Resource": format!("arn:aws:s3:::{TEST_BUCKET}/*"),
            "Condition": {
                "StringNotEquals": {
                    "aws:PrincipalAccount": account_id.as_deref().unwrap_or("000000000000")
                }
            }
        }]
    });
    let policy_str = serde_json::to_string(&policy)?;
    s3.put_bucket_policy(TEST_BUCKET, &policy_str).await?;
    println!("  Policy applied successfully");

    // 2. Verify policy was applied
    println!("\n[2/8] Verifying bucket policy...");
    let retrieved = aws_get_bucket_policy(TEST_BUCKET, &region);
    assert!(retrieved.is_some(), "bucket policy should be retrievable");
    let retrieved_policy = retrieved.unwrap();
    assert!(
        retrieved_policy.contains("CloudLiteTestPolicy"),
        "policy should contain our statement SID"
    );
    println!("  Policy verified: contains CloudLiteTestPolicy statement");

    // -- DeleteBucketPolicy --
    println!("\n[3/8] DeleteBucketPolicy — removing the policy...");
    s3.delete_bucket_policy(TEST_BUCKET).await?;
    println!("  Policy deleted successfully");

    // 4. Verify policy was deleted
    println!("\n[4/8] Verifying policy deletion...");
    let retrieved = aws_get_bucket_policy(TEST_BUCKET, &region);
    assert!(
        retrieved.is_none(),
        "bucket policy should not exist after deletion"
    );
    println!("  Policy confirmed deleted");

    // -- PutPublicAccessBlock --
    println!("\n[5/8] PutPublicAccessBlock — blocking all public access...");
    let config = PublicAccessBlockConfiguration {
        block_public_acls: Some(true),
        ignore_public_acls: Some(true),
        block_public_policy: Some(true),
        restrict_public_buckets: Some(true),
    };
    s3.put_public_access_block(TEST_BUCKET, &config).await?;
    println!("  Public access block configured successfully");

    // 6. Verify public access block
    println!("\n[6/8] Verifying public access block...");
    let retrieved = aws_get_public_access_block(TEST_BUCKET, &region);
    assert!(
        retrieved.is_some(),
        "public access block should be retrievable"
    );
    let pab = retrieved.unwrap();
    let pab_config = &pab["PublicAccessBlockConfiguration"];
    assert_eq!(pab_config["BlockPublicAcls"], true);
    assert_eq!(pab_config["IgnorePublicAcls"], true);
    assert_eq!(pab_config["BlockPublicPolicy"], true);
    assert_eq!(pab_config["RestrictPublicBuckets"], true);
    println!("  All 4 public access block settings confirmed enabled");

    // -- PutBucketLifecycleConfiguration --
    println!("\n[7/8] PutBucketLifecycleConfiguration — adding lifecycle rules...");
    let lifecycle = BucketLifecycleConfiguration {
        rules: vec![LifecycleRule {
            id: Some("cloud-lite-test-rule".into()),
            status: "Enabled".into(),
            filter: Some(LifecycleRuleFilter {
                prefix: Some("logs/".into()),
                ..Default::default()
            }),
            expiration: Some(aws_lite::types::s3::LifecycleExpiration {
                days: Some(30),
                ..Default::default()
            }),
            ..Default::default()
        }],
    };
    s3.put_bucket_lifecycle_configuration(TEST_BUCKET, &lifecycle)
        .await?;
    println!("  Lifecycle configuration applied successfully");

    // 8. Verify lifecycle configuration
    println!("\n[8/8] Verifying lifecycle configuration...");
    let retrieved = aws_get_bucket_lifecycle(TEST_BUCKET, &region);
    assert!(
        retrieved.is_some(),
        "lifecycle configuration should be retrievable"
    );
    let lc = retrieved.unwrap();
    let rules = lc["Rules"].as_array().expect("Rules should be an array");
    assert_eq!(rules.len(), 1, "should have exactly 1 rule");
    assert_eq!(rules[0]["ID"], "cloud-lite-test-rule");
    assert_eq!(rules[0]["Status"], "Enabled");
    println!("  Lifecycle rule verified: id=cloud-lite-test-rule, status=Enabled");

    Ok(())
}

fn get_account_id(region: &str) -> Option<String> {
    let output = Command::new("aws")
        .args(["sts", "get-caller-identity", "--region", region])
        .output()
        .ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str::<serde_json::Value>(&stdout)
            .ok()
            .and_then(|v| v["Account"].as_str().map(String::from))
    } else {
        None
    }
}

// -- Read Operations ----------------------------------------------------------

const TEST_READ_BUCKET: &str = "cloud-lite-test-s3-read-ops";
const TEST_LOG_BUCKET: &str = "cloud-lite-test-s3-log-target";

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_s3_list_buckets() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let s3 = client.s3();

    println!("\n=== S3 ListBuckets ===");

    // Create a fixture bucket so we know at least one exists
    println!("\n[setup] Creating test bucket...");
    aws_create_bucket(TEST_READ_BUCKET, &region);

    let result = async {
        println!("\n[1/2] ListBuckets...");
        let response = s3.list_buckets().await?;
        assert!(
            !response.buckets.is_empty(),
            "should have at least one bucket"
        );
        println!("  Found {} buckets", response.buckets.len());

        // Verify our test bucket is in the list
        let found = response
            .buckets
            .iter()
            .any(|b| b.name.as_deref() == Some(TEST_READ_BUCKET));
        assert!(found, "test bucket should be in the list");
        println!("  Verified test bucket in list");

        // Verify owner is present
        println!("\n[2/2] Checking owner...");
        assert!(response.owner.is_some(), "owner should be present");
        let owner = response.owner.as_ref().unwrap();
        assert!(owner.id.is_some(), "owner ID should be present");
        println!("  Owner ID: {}", owner.id.as_deref().unwrap_or("?"));

        Ok::<_, Box<dyn std::error::Error>>(())
    }
    .await;

    println!("\n[cleanup] Deleting test bucket...");
    aws_delete_bucket(TEST_READ_BUCKET, &region);
    result
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_s3_get_bucket_versioning() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let s3 = client.s3();

    println!("\n=== S3 GetBucketVersioning ===");
    println!("\n[setup] Creating test bucket...");
    aws_create_bucket(TEST_READ_BUCKET, &region);

    let result = async {
        // 1. Read versioning on a fresh bucket (should be empty/unset)
        println!("\n[1/3] GetBucketVersioning on fresh bucket...");
        let response = s3.get_bucket_versioning(TEST_READ_BUCKET).await?;
        println!(
            "  Versioning status: {:?}",
            response.status.as_deref().unwrap_or("(none)")
        );

        // 2. Enable versioning using our client
        println!("\n[2/3] PutBucketVersioning — enabling versioning...");
        let config = VersioningConfiguration {
            status: Some("Enabled".into()),
            ..Default::default()
        };
        s3.put_bucket_versioning(TEST_READ_BUCKET, &config).await?;
        println!("  Versioning enabled");

        // 3. Verify versioning is enabled
        println!("\n[3/3] GetBucketVersioning — verifying enabled...");
        let response = s3.get_bucket_versioning(TEST_READ_BUCKET).await?;
        assert_eq!(
            response.status.as_deref(),
            Some("Enabled"),
            "versioning should be Enabled"
        );
        println!("  Confirmed: versioning is Enabled");

        Ok::<_, Box<dyn std::error::Error>>(())
    }
    .await;

    println!("\n[cleanup] Deleting test bucket...");
    aws_delete_bucket(TEST_READ_BUCKET, &region);
    result
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_s3_get_bucket_encryption() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let s3 = client.s3();

    println!("\n=== S3 GetBucketEncryption ===");
    println!("\n[setup] Creating test bucket...");
    aws_create_bucket(TEST_READ_BUCKET, &region);

    let result = async {
        // 1. Read default encryption (S3 buckets now have SSE-S3 by default)
        println!("\n[1/3] GetBucketEncryption — reading default...");
        let response = s3.get_bucket_encryption(TEST_READ_BUCKET).await?;
        assert!(
            !response.rules.is_empty(),
            "should have at least one encryption rule"
        );
        let default_rule = &response.rules[0];
        println!(
            "  Default algorithm: {:?}",
            default_rule
                .apply_server_side_encryption_by_default
                .as_ref()
                .map(|d| d.sse_algorithm.as_str())
        );

        // 2. Set explicit encryption config
        println!("\n[2/3] PutBucketEncryption — setting SSE-S3...");
        let enc_config = ServerSideEncryptionConfiguration {
            rules: vec![ServerSideEncryptionRule {
                apply_server_side_encryption_by_default: Some(ServerSideEncryptionByDefault {
                    sse_algorithm: "AES256".into(),
                    kms_master_key_id: None,
                }),
                bucket_key_enabled: Some(false),
            }],
        };
        s3.put_bucket_encryption(TEST_READ_BUCKET, &enc_config)
            .await?;
        println!("  Encryption configured");

        // 3. Verify
        println!("\n[3/3] GetBucketEncryption — verifying...");
        let response = s3.get_bucket_encryption(TEST_READ_BUCKET).await?;
        assert!(!response.rules.is_empty());
        let rule = &response.rules[0];
        let default = rule
            .apply_server_side_encryption_by_default
            .as_ref()
            .expect("should have default encryption");
        assert_eq!(default.sse_algorithm, "AES256");
        println!("  Confirmed: SSE algorithm = AES256");

        Ok::<_, Box<dyn std::error::Error>>(())
    }
    .await;

    println!("\n[cleanup] Deleting test bucket...");
    aws_delete_bucket(TEST_READ_BUCKET, &region);
    result
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_s3_get_bucket_logging() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let s3 = client.s3();

    println!("\n=== S3 GetBucketLogging ===");
    println!("\n[setup] Creating test buckets...");
    aws_create_bucket(TEST_READ_BUCKET, &region);
    aws_create_bucket(TEST_LOG_BUCKET, &region);

    let result = async {
        // 1. Read logging on a fresh bucket (should be disabled)
        println!("\n[1/3] GetBucketLogging — fresh bucket...");
        let response = s3.get_bucket_logging(TEST_READ_BUCKET).await?;
        assert!(
            response.logging_enabled.is_none(),
            "logging should not be enabled on a fresh bucket"
        );
        println!("  Logging is disabled (as expected)");

        // 2. Enable logging
        println!("\n[2/3] PutBucketLogging — enabling logging...");
        let logging = BucketLoggingStatus {
            logging_enabled: Some(LoggingEnabled {
                target_bucket: TEST_LOG_BUCKET.into(),
                target_prefix: "logs/".into(),
            }),
        };
        s3.put_bucket_logging(TEST_READ_BUCKET, &logging).await?;
        println!("  Logging enabled");

        // 3. Verify
        println!("\n[3/3] GetBucketLogging — verifying...");
        let response = s3.get_bucket_logging(TEST_READ_BUCKET).await?;
        let enabled = response
            .logging_enabled
            .as_ref()
            .expect("logging should be enabled");
        assert_eq!(enabled.target_bucket, TEST_LOG_BUCKET);
        assert_eq!(enabled.target_prefix, "logs/");
        println!(
            "  Confirmed: target_bucket={}, target_prefix=logs/",
            TEST_LOG_BUCKET
        );

        Ok::<_, Box<dyn std::error::Error>>(())
    }
    .await;

    println!("\n[cleanup] Deleting test buckets...");
    aws_delete_bucket(TEST_READ_BUCKET, &region);
    aws_delete_bucket(TEST_LOG_BUCKET, &region);
    result
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_s3_get_bucket_acl() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let s3 = client.s3();

    println!("\n=== S3 GetBucketAcl ===");
    println!("\n[setup] Creating test bucket...");
    aws_create_bucket(TEST_READ_BUCKET, &region);

    let result = async {
        println!("\n[1/1] GetBucketAcl...");
        let response = s3.get_bucket_acl(TEST_READ_BUCKET).await?;

        // Every bucket has an owner
        assert!(response.owner.is_some(), "ACL should have an owner");
        let owner = response.owner.as_ref().unwrap();
        assert!(owner.id.is_some(), "owner should have an ID");
        println!("  Owner ID: {}", owner.id.as_deref().unwrap_or("?"));

        // Default ACL gives full control to the owner
        println!("  Grants: {} entries", response.grants.len());
        for grant in &response.grants {
            println!(
                "    Permission={:?}, Grantee ID={:?}",
                grant.permission.as_deref().unwrap_or("?"),
                grant
                    .grantee
                    .as_ref()
                    .and_then(|g| g.id.as_deref())
                    .unwrap_or("?"),
            );
        }

        Ok::<_, Box<dyn std::error::Error>>(())
    }
    .await;

    println!("\n[cleanup] Deleting test bucket...");
    aws_delete_bucket(TEST_READ_BUCKET, &region);
    result
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_s3_get_bucket_lifecycle_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let s3 = client.s3();

    println!("\n=== S3 GetBucketLifecycleConfiguration ===");
    println!("\n[setup] Creating test bucket...");
    aws_create_bucket(TEST_READ_BUCKET, &region);

    let result = async {
        // 1. Set lifecycle config first
        println!("\n[1/2] PutBucketLifecycleConfiguration...");
        let lifecycle = BucketLifecycleConfiguration {
            rules: vec![LifecycleRule {
                id: Some("cloud-lite-test-lifecycle".into()),
                status: "Enabled".into(),
                filter: Some(LifecycleRuleFilter {
                    prefix: Some("temp/".into()),
                    ..Default::default()
                }),
                expiration: Some(aws_lite::types::s3::LifecycleExpiration {
                    days: Some(7),
                    ..Default::default()
                }),
                ..Default::default()
            }],
        };
        s3.put_bucket_lifecycle_configuration(TEST_READ_BUCKET, &lifecycle)
            .await?;
        println!("  Lifecycle set");

        // 2. Read it back
        println!("\n[2/2] GetBucketLifecycleConfiguration...");
        let response = s3
            .get_bucket_lifecycle_configuration(TEST_READ_BUCKET)
            .await?;
        assert!(
            !response.rules.is_empty(),
            "should have at least one lifecycle rule"
        );
        let rule = &response.rules[0];
        assert_eq!(rule.id.as_deref(), Some("cloud-lite-test-lifecycle"));
        assert_eq!(rule.status, "Enabled");
        println!(
            "  Confirmed: rule id={:?}, status={}",
            rule.id.as_deref(),
            rule.status
        );

        Ok::<_, Box<dyn std::error::Error>>(())
    }
    .await;

    println!("\n[cleanup] Deleting test bucket...");
    aws_delete_bucket(TEST_READ_BUCKET, &region);
    result
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_s3_get_public_access_block() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let s3 = client.s3();

    println!("\n=== S3 GetPublicAccessBlock ===");
    println!("\n[setup] Creating test bucket...");
    aws_create_bucket(TEST_READ_BUCKET, &region);

    let result = async {
        // 1. Set public access block
        println!("\n[1/2] PutPublicAccessBlock...");
        let config = PublicAccessBlockConfiguration {
            block_public_acls: Some(true),
            ignore_public_acls: Some(true),
            block_public_policy: Some(true),
            restrict_public_buckets: Some(true),
        };
        s3.put_public_access_block(TEST_READ_BUCKET, &config)
            .await?;
        println!("  Public access block set");

        // 2. Read it back
        println!("\n[2/2] GetPublicAccessBlock...");
        let response = s3.get_public_access_block(TEST_READ_BUCKET).await?;
        assert_eq!(response.block_public_acls, Some(true));
        assert_eq!(response.ignore_public_acls, Some(true));
        assert_eq!(response.block_public_policy, Some(true));
        assert_eq!(response.restrict_public_buckets, Some(true));
        println!("  All 4 settings confirmed true");

        Ok::<_, Box<dyn std::error::Error>>(())
    }
    .await;

    println!("\n[cleanup] Deleting test bucket...");
    aws_delete_bucket(TEST_READ_BUCKET, &region);
    result
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_s3_get_bucket_policy() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let s3 = client.s3();

    println!("\n=== S3 GetBucketPolicy ===");
    println!("\n[setup] Creating test bucket...");
    aws_create_bucket(TEST_READ_BUCKET, &region);

    let result = async {
        // 1. Set a policy first
        println!("\n[1/3] PutBucketPolicy...");
        let account_id = get_account_id(&region);
        let policy = serde_json::json!({
            "Version": "2012-10-17",
            "Statement": [{
                "Sid": "CloudLiteReadTest",
                "Effect": "Deny",
                "Principal": "*",
                "Action": "s3:GetObject",
                "Resource": format!("arn:aws:s3:::{TEST_READ_BUCKET}/*"),
                "Condition": {
                    "StringNotEquals": {
                        "aws:PrincipalAccount": account_id.as_deref().unwrap_or("000000000000")
                    }
                }
            }]
        });
        let policy_str = serde_json::to_string(&policy)?;
        s3.put_bucket_policy(TEST_READ_BUCKET, &policy_str).await?;
        println!("  Policy set");

        // 2. Read it back
        println!("\n[2/3] GetBucketPolicy...");
        let retrieved = s3.get_bucket_policy(TEST_READ_BUCKET).await?;
        let has_sid = retrieved
            .statement
            .iter()
            .any(|s| s.sid.as_deref() == Some("CloudLiteReadTest"));
        assert!(has_sid, "policy should contain our statement SID");
        println!("  Policy contains 'CloudLiteReadTest'");

        // Verify it has statements
        assert!(
            !retrieved.statement.is_empty(),
            "parsed policy should have Statement array"
        );
        println!("  Policy has {} statement(s)", retrieved.statement.len());

        // 3. Error case: non-existent bucket
        println!("\n[3/3] GetBucketPolicy on non-existent bucket...");
        let result = s3
            .get_bucket_policy("cloud-lite-test-nonexistent-bucket-999")
            .await;
        assert!(result.is_err(), "non-existent bucket should fail");
        println!("  Got expected error");

        Ok::<_, Box<dyn std::error::Error>>(())
    }
    .await;

    println!("\n[cleanup] Deleting test bucket...");
    aws_delete_bucket(TEST_READ_BUCKET, &region);
    result
}

// -- Error Cases --------------------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_s3_error_cases() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let s3 = client.s3();

    println!("\n=== S3 Error Cases ===");

    // 1. PutBucketPolicy on non-existent bucket
    println!("\n[1/3] PutBucketPolicy on non-existent bucket...");
    let result = s3
        .put_bucket_policy(
            "cloud-lite-test-nonexistent-bucket-999",
            r#"{"Version":"2012-10-17","Statement":[]}"#,
        )
        .await;
    assert!(result.is_err(), "non-existent bucket should fail");
    let err = format!("{}", result.unwrap_err());
    println!("  Got expected error: {}", &err[..err.len().min(200)]);

    // 2. DeleteBucketPolicy on non-existent bucket
    println!("\n[2/3] DeleteBucketPolicy on non-existent bucket...");
    let result = s3
        .delete_bucket_policy("cloud-lite-test-nonexistent-bucket-999")
        .await;
    assert!(result.is_err(), "non-existent bucket should fail");
    let err = format!("{}", result.unwrap_err());
    println!("  Got expected error: {}", &err[..err.len().min(200)]);

    // 3. PutPublicAccessBlock on non-existent bucket
    println!("\n[3/3] PutPublicAccessBlock on non-existent bucket...");
    let config = PublicAccessBlockConfiguration {
        block_public_acls: Some(true),
        ..Default::default()
    };
    let result = s3
        .put_public_access_block("cloud-lite-test-nonexistent-bucket-999", &config)
        .await;
    assert!(result.is_err(), "non-existent bucket should fail");
    let err = format!("{}", result.unwrap_err());
    println!("  Got expected error: {}", &err[..err.len().min(200)]);

    Ok(())
}
