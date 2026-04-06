//! Integration tests for AWS CloudFront API
//!
//! Tests ListDistributions, GetDistributionConfig, CreateOriginAccessControl,
//! and CreateDistribution against the real AWS CloudFront API.
//!
//! CloudFront is a global service — requests go to cloudfront.amazonaws.com.
//!
//! Note: UpdateDistribution and delete operations require ETag/If-Match header
//! support which is not yet implemented. These are tracked as a known limitation.
//!
//! Run with:
//!   AWS_PROFILE=<profile> AWS_REGION=us-east-1 \
//!     cargo test -p aws-lite --test integration cloudfront -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use std::env;

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_list_distributions_and_get_config() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let cf = client.cloudfront();

    println!("\n=== CloudFront: ListDistributions + GetDistributionConfig ===");

    // 1. List distributions
    println!("\n[1/3] Listing CloudFront distributions...");
    let list = cf.list_distributions().await?;
    println!("  Quantity: {}", list.quantity);
    println!("  IsTruncated: {}", list.is_truncated);
    println!("  Distributions found: {}", list.items.len());

    // 2. Verify response structure
    println!("\n[2/3] Verifying list response structure...");
    assert!(list.quantity >= 0, "quantity should be non-negative");
    // quantity should match the items count (when not truncated)
    if !list.is_truncated {
        assert_eq!(
            list.quantity as usize,
            list.items.len(),
            "quantity should match items count when not truncated"
        );
    }

    // 3. If there are distributions, get the config for the first one
    if let Some(dist) = list.items.first() {
        println!("\n[3/3] Getting config for distribution {}...", dist.id);
        println!("  Domain: {}", dist.domain_name);
        println!("  Status: {}", dist.status);
        println!("  PriceClass: {}", dist.price_class);
        println!(
            "  ViewerProtocolPolicy: {}",
            dist.default_cache_behavior.viewer_protocol_policy
        );

        assert!(!dist.id.is_empty(), "distribution ID should not be empty");
        assert!(!dist.arn.is_empty(), "distribution ARN should not be empty");
        assert!(
            dist.arn.starts_with("arn:aws:cloudfront:"),
            "ARN should start with arn:aws:cloudfront:, got: {}",
            dist.arn
        );

        let config = cf.get_distribution_config(&dist.id).await?;
        println!("  CallerReference: {}", config.caller_reference);
        println!("  Enabled: {}", config.enabled);
        println!("  Comment: {}", config.comment);
        println!("  Origins count: {}", config.origins.quantity);

        assert!(
            !config.caller_reference.is_empty(),
            "caller_reference should not be empty"
        );
    } else {
        println!("\n[3/3] No distributions found — skipping GetDistributionConfig");
        println!("  (This is expected in fresh accounts)");
    }

    println!("\n  All CloudFront read operations passed!");
    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_create_origin_access_control() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let cf = client.cloudfront();

    println!("\n=== CloudFront: CreateOriginAccessControl ===");

    // 1. Create an Origin Access Control with a unique name
    let unique_suffix = chrono::Utc::now().timestamp();
    let oac_name = format!("cloud-lite-test-ralph-oac-{unique_suffix}");
    println!("\n[1/2] Creating Origin Access Control '{oac_name}'...");
    let oac_config = aws_lite::types::cloudfront::OriginAccessControlConfig {
        name: oac_name.clone(),
        description: Some("Test OAC created by cloud-lite integration tests".to_string()),
        signing_protocol: "sigv4".to_string(),
        signing_behavior: "always".to_string(),
        origin_access_control_origin_type: "s3".to_string(),
    };
    let oac = cf.create_origin_access_control(&oac_config).await?;

    // 2. Verify response structure
    println!("\n[2/2] Verifying Origin Access Control response...");
    assert!(!oac.id.is_empty(), "OAC ID should not be empty");
    println!("  OAC ID: {}", oac.id);

    let config = oac
        .origin_access_control_config
        .as_ref()
        .expect("OAC config should be present");
    assert_eq!(config.name, oac_name);
    assert_eq!(config.signing_protocol, "sigv4");
    assert_eq!(config.signing_behavior, "always");
    assert_eq!(config.origin_access_control_origin_type, "s3");
    println!("  Name: {}", config.name);
    println!("  SigningProtocol: {}", config.signing_protocol);
    println!("  SigningBehavior: {}", config.signing_behavior);
    println!("  OriginType: {}", config.origin_access_control_origin_type);

    // NOTE: Cleanup requires DeleteOriginAccessControl with If-Match ETag header,
    // which is not yet supported. The created OAC should be manually cleaned up:
    //   aws cloudfront delete-origin-access-control --id {oac.id} --if-match <etag>
    println!(
        "\n  WARNING: OAC {} created but cannot be auto-cleaned (If-Match header not supported)",
        oac.id
    );
    println!("  Manual cleanup required.");

    println!("\n  CreateOriginAccessControl passed!");
    Ok(())
}

// NOTE: CreateDistribution and UpdateDistribution integration tests are blocked.
//
// Known limitations:
// 1. REST-XML serialization of Vec<T> fields: quick_xml serializes Vec items
//    as repeated parent-name elements, but CloudFront expects <Items><Origin>...</Origin></Items>.
//    This requires custom XML serialization for REST-XML request bodies.
// 2. ETag/If-Match header: CloudFront requires ETag for update/delete operations,
//    but the AwsHttpClient doesn't expose response headers or support custom request headers.
//
// These limitations affect: CreateDistribution, UpdateDistribution, DeleteDistribution,
// DeleteOriginAccessControl.
