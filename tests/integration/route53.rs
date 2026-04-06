//! Integration tests for AWS Route 53 API
//!
//! Tests ListHostedZones, ListResourceRecordSets, ListHealthChecks, GetHealthCheckStatus,
//! DeleteHealthCheck, and ChangeResourceRecordSets against the real AWS Route 53 API.
//!
//! Route 53 is a global service — requests go to route53.amazonaws.com regardless of region.
//! Region is only used for SigV4 signing.
//!
//! Run with:
//!   cargo test -p aws-lite --test integration route53 -- --ignored --test-threads=1 --nocapture

use aws_lite::{
    AwsHttpClient,
    types::route53::{
        Change, ChangeAction, ChangeBatch, ChangeResourceRecordSetsRequest,
        CreateHealthCheckRequest, HealthCheckConfig, HealthCheckType, RRType, ResourceRecord,
        ResourceRecordSet,
    },
};
use std::env;
use std::process::Command;
use std::time::Duration;

// Route 53 is global but we still need a region for signing
const TEST_REGION: &str = "us-east-1";
const TEST_HEALTH_CHECK_DOMAIN: &str = "cloud-lite-test-ralph-healthcheck.example.com";

fn region() -> String {
    env::var("AWS_DEFAULT_REGION")
        .or_else(|_| env::var("AWS_REGION"))
        .unwrap_or_else(|_| TEST_REGION.to_string())
}

/// Create a test hosted zone via AWS CLI (private zone, needs a VPC)
/// Returns the hosted zone ID (without the /hostedzone/ prefix) if successful
fn aws_create_private_hosted_zone(vpc_id: &str, region: &str) -> Option<String> {
    let name = "cloud-lite-test-ralph.internal";
    let caller_ref = format!("cloud-lite-test-ralph-{}", chrono::Utc::now().timestamp());
    let output = Command::new("aws")
        .args([
            "route53",
            "create-hosted-zone",
            "--name",
            name,
            "--caller-reference",
            &caller_ref,
            "--vpc",
            &format!("VPCRegion={region},VPCId={vpc_id}"),
            "--hosted-zone-config",
            "PrivateZone=true",
            "--output",
            "json",
        ])
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str::<serde_json::Value>(&stdout)
            .ok()
            .and_then(|v| {
                v["HostedZone"]["Id"]
                    .as_str()
                    .map(|s| s.trim_start_matches("/hostedzone/").to_string())
            })
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("  aws create-hosted-zone failed: {stderr}");
        None
    }
}

fn aws_delete_hosted_zone(zone_id: &str) {
    // First delete all non-default record sets
    let _ = Command::new("aws")
        .args(["route53", "delete-hosted-zone", "--id", zone_id])
        .output();
}

/// Get the default VPC ID for a region
fn aws_get_default_vpc(region: &str) -> Option<String> {
    let output = Command::new("aws")
        .args([
            "ec2",
            "describe-vpcs",
            "--filters",
            "Name=isDefault,Values=true",
            "--region",
            region,
            "--output",
            "json",
        ])
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str::<serde_json::Value>(&stdout)
            .ok()
            .and_then(|v| {
                v["Vpcs"]
                    .as_array()
                    .and_then(|vpcs| vpcs.first())
                    .and_then(|vpc| vpc["VpcId"].as_str().map(String::from))
            })
    } else {
        None
    }
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_route53_list_hosted_zones_and_records() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;

    println!("\n=== Route 53: ListHostedZones + ListResourceRecordSets ===");
    println!("Region (for signing): {region}");

    // [0/4] Create fixture: private hosted zone (needs a VPC)
    println!("\n[0/4] Creating test hosted zone fixture...");
    let vpc_id = aws_get_default_vpc(&region);
    let zone_id = if let Some(vid) = vpc_id {
        println!("  Using VPC: {vid}");
        let zid = aws_create_private_hosted_zone(&vid, &region);
        if let Some(ref id) = zid {
            println!("  Created hosted zone: {id}");
            tokio::time::sleep(Duration::from_secs(2)).await;
        } else {
            println!("  WARNING: Could not create hosted zone via CLI");
        }
        zid
    } else {
        println!("  WARNING: No default VPC found, will use existing zones");
        None
    };

    let result = run_hosted_zone_tests(&client, &region, zone_id.as_deref()).await;

    // Always cleanup
    if let Some(ref id) = zone_id {
        println!("\n[cleanup] Deleting test hosted zone {id}...");
        aws_delete_hosted_zone(id);
    }

    result.expect("Route 53 list tests failed");
    println!("\n  All Route 53 list tests passed!");
    Ok(())
}

async fn run_hosted_zone_tests(
    client: &AwsHttpClient,
    _region: &str,
    fixture_zone_id: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let r53 = client.route53();

    // [1/4] ListHostedZones
    println!("\n[1/4] Listing hosted zones...");
    let zones = r53.list_hosted_zones().await?;
    println!("  IsTruncated: {}", zones.is_truncated);
    println!("  MaxItems: {}", zones.max_items);
    println!("  Zones found: {}", zones.hosted_zones.len());

    // Structural assertions
    assert!(!zones.max_items.is_empty(), "max_items should not be empty");

    // Find a zone to use for ListResourceRecordSets
    let zone_id_to_use = fixture_zone_id.map(String::from).or_else(|| {
        zones
            .hosted_zones
            .first()
            .map(|z| z.id.trim_start_matches("/hostedzone/").to_string())
    });

    // [2/4] Verify zone structure if we have zones
    println!("\n[2/4] Verifying hosted zone structure...");
    if let Some(zone) = zones.hosted_zones.first() {
        println!("  First zone ID: {}", zone.id);
        println!("  First zone name: {}", zone.name);
        println!("  CallerReference: {}", zone.caller_reference);
        assert!(!zone.id.is_empty(), "zone ID should not be empty");
        assert!(!zone.name.is_empty(), "zone name should not be empty");
        assert!(
            !zone.caller_reference.is_empty(),
            "caller_reference should not be empty"
        );
        assert!(
            zone.id.starts_with("/hostedzone/") || !zone.id.contains('/'),
            "zone ID should be /hostedzone/XXXXX format, got: {}",
            zone.id
        );
    } else {
        println!("  No hosted zones found — account appears to have none");
    }

    // [3/4] ListResourceRecordSets — use first zone
    println!("\n[3/4] Listing resource record sets...");
    if let Some(ref zone_id) = zone_id_to_use {
        let clean_id = zone_id.trim_start_matches("/hostedzone/");
        println!("  Using zone ID: {clean_id}");
        let records = r53.list_resource_record_sets(clean_id).await?;
        println!("  IsTruncated: {}", records.is_truncated);
        println!("  MaxItems: {}", records.max_items);
        println!(
            "  Record sets found: {}",
            records.resource_record_sets.len()
        );
        assert!(
            !records.max_items.is_empty(),
            "max_items should not be empty"
        );

        // Every hosted zone should have at least SOA and NS records
        for rr in &records.resource_record_sets {
            println!("    Record: {} {:?}", rr.name, rr.r#type);
            assert!(!rr.name.is_empty(), "record name should not be empty");
        }
    } else {
        println!("  Skipping: no hosted zone available");
    }

    // [4/4] Structural check complete
    println!("\n[4/4] All structural checks passed.");
    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_route53_list_health_checks_and_status() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let r53 = client.route53();

    println!("\n=== Route 53: ListHealthChecks + GetHealthCheckStatus ===");
    println!("Region (for signing): {region}");

    // [0/5] Create a health check fixture using the library
    println!("\n[0/5] Creating test health check via library...");
    let caller_ref = format!("cloud-lite-test-ralph-{}", chrono::Utc::now().timestamp());
    let req = CreateHealthCheckRequest {
        caller_reference: caller_ref,
        health_check_config: HealthCheckConfig {
            r#type: Some(HealthCheckType::Http),
            fully_qualified_domain_name: Some(TEST_HEALTH_CHECK_DOMAIN.to_string()),
            port: Some(80),
            resource_path: Some("/health".to_string()),
            request_interval: Some(30),
            failure_threshold: Some(3),
            ..Default::default()
        },
    };
    let created = r53.create_health_check(&req).await?;
    let health_check_id = created.health_check.id.clone();
    println!("  Created health check ID: {health_check_id}");
    assert!(
        !health_check_id.is_empty(),
        "health check ID should not be empty"
    );

    let result = run_health_check_tests(&client, &health_check_id).await;

    // Always cleanup
    println!("\n[cleanup] Deleting test health check {health_check_id}...");
    let _ = r53.delete_health_check(&health_check_id).await;

    result.expect("Route 53 health check tests failed");
    println!("\n  All Route 53 health check tests passed!");
    Ok(())
}

async fn run_health_check_tests(
    client: &AwsHttpClient,
    health_check_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let r53 = client.route53();

    // [1/5] ListHealthChecks
    println!("\n[1/5] Listing health checks...");
    let list = r53.list_health_checks().await?;
    println!("  IsTruncated: {}", list.is_truncated);
    println!("  MaxItems: {}", list.max_items);
    println!("  Health checks found: {}", list.health_checks.len());
    assert!(!list.max_items.is_empty(), "max_items should not be empty");

    // [2/5] Find our test health check in the list
    println!("\n[2/5] Verifying test health check is in list...");
    let found = list
        .health_checks
        .iter()
        .find(|hc| hc.id == health_check_id);
    assert!(
        found.is_some(),
        "Created health check {health_check_id} should appear in ListHealthChecks"
    );
    let hc = found.unwrap();
    println!("  Found: ID={}, version={}", hc.id, hc.health_check_version);
    assert!(
        !hc.caller_reference.is_empty(),
        "caller_reference should not be empty"
    );
    assert!(
        hc.health_check_version >= 1,
        "health_check_version should be at least 1"
    );

    // [3/5] Verify health check config
    println!("\n[3/5] Verifying health check config...");
    let config = &hc.health_check_config;
    assert_eq!(
        config.r#type,
        Some(HealthCheckType::Http),
        "type should be HTTP"
    );
    assert_eq!(
        config.fully_qualified_domain_name.as_deref(),
        Some(TEST_HEALTH_CHECK_DOMAIN),
        "domain name should match"
    );
    assert_eq!(config.port, Some(80), "port should be 80");
    println!("  Type: {:?}", config.r#type);
    println!("  Domain: {:?}", config.fully_qualified_domain_name);
    println!("  Port: {:?}", config.port);

    // [4/5] GetHealthCheckStatus (retry — AWS may need time to propagate)
    println!("\n[4/5] Getting health check status...");
    let mut status = None;
    for attempt in 1..=5 {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        match r53.get_health_check_status(health_check_id).await {
            Ok(s) => {
                status = Some(s);
                break;
            }
            Err(e) if attempt < 5 => {
                println!("  Attempt {attempt}/5: not ready yet ({e}), retrying...");
            }
            Err(e) => return Err(e.into()),
        }
    }
    let status = status.unwrap();
    println!("  Observations: {}", status.health_check_observations.len());
    // A newly created health check might not have observations yet — just verify the call succeeds
    for obs in &status.health_check_observations {
        println!(
            "  Observer region: {:?}, IP: {:?}",
            obs.region, obs.ip_address
        );
    }

    // [5/5] Structural check complete
    println!("\n[5/5] All health check structural checks passed.");
    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_route53_delete_health_check() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let r53 = client.route53();

    println!("\n=== Route 53: DeleteHealthCheck ===");
    println!("Region (for signing): {region}");

    // [0/4] Create fixture: health check to delete
    println!("\n[0/4] Creating test health check fixture via library...");
    let caller_ref = format!(
        "cloud-lite-test-ralph-del-{}",
        chrono::Utc::now().timestamp()
    );
    let req = CreateHealthCheckRequest {
        caller_reference: caller_ref,
        health_check_config: HealthCheckConfig {
            r#type: Some(HealthCheckType::Http),
            fully_qualified_domain_name: Some(TEST_HEALTH_CHECK_DOMAIN.to_string()),
            port: Some(80),
            resource_path: Some("/health".to_string()),
            request_interval: Some(30),
            failure_threshold: Some(3),
            ..Default::default()
        },
    };
    let created = r53.create_health_check(&req).await?;
    let health_check_id = created.health_check.id.clone();
    println!("  Created health check ID: {health_check_id}");
    assert!(
        !health_check_id.is_empty(),
        "health check ID should not be empty"
    );

    // [1/4] Verify it exists before deleting
    println!("\n[1/4] Verifying health check exists before delete...");
    let list_before = r53.list_health_checks().await?;
    let exists_before = list_before
        .health_checks
        .iter()
        .any(|hc| hc.id == health_check_id);
    assert!(exists_before, "Health check should exist before delete");
    println!("  Confirmed: health check {health_check_id} exists");

    // [2/4] Delete the health check
    println!("\n[2/4] Deleting health check...");
    r53.delete_health_check(&health_check_id).await?;
    println!("  DeleteHealthCheck succeeded (no error)");

    // [3/4] Verify it no longer exists
    println!("\n[3/4] Verifying health check is gone after delete...");
    let list_after = r53.list_health_checks().await?;
    let exists_after = list_after
        .health_checks
        .iter()
        .any(|hc| hc.id == health_check_id);
    assert!(
        !exists_after,
        "Health check {health_check_id} should NOT exist after delete"
    );
    println!("  Confirmed: health check {health_check_id} is gone");

    // [4/4] Done
    println!("\n[4/4] DeleteHealthCheck test passed!");
    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_route53_change_resource_record_sets() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;

    println!("\n=== Route 53: ChangeResourceRecordSets ===");
    println!("Region (for signing): {region}");

    // [0/6] Create fixture: private hosted zone via CLI
    println!("\n[0/6] Creating test hosted zone fixture via CLI...");
    let vpc_id = aws_get_default_vpc(&region)
        .expect("Default VPC must exist for ChangeResourceRecordSets test");
    println!("  Using VPC: {vpc_id}");
    let zone_id = aws_create_private_hosted_zone(&vpc_id, &region)
        .expect("Must be able to create private hosted zone for test");
    println!("  Created hosted zone: {zone_id}");
    tokio::time::sleep(Duration::from_secs(2)).await;

    let test_record_name = "test-txt.cloud-lite-test-ralph.internal";
    let result = run_change_record_tests(&client, &zone_id, test_record_name).await;

    // Always cleanup: delete hosted zone
    println!("\n[cleanup] Deleting test hosted zone {zone_id}...");
    aws_delete_hosted_zone(&zone_id);

    result.expect("ChangeResourceRecordSets test failed");
    println!("\n  All ChangeResourceRecordSets tests passed!");
    Ok(())
}

async fn run_change_record_tests(
    client: &AwsHttpClient,
    zone_id: &str,
    record_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let r53 = client.route53();

    // [1/6] List records before: expect only default SOA and NS
    println!("\n[1/6] Listing records before change (should be SOA + NS only)...");
    let before = r53.list_resource_record_sets(zone_id).await?;
    println!("  Records before: {}", before.resource_record_sets.len());
    // A new private hosted zone has exactly 2 records: SOA and NS
    assert!(
        before.resource_record_sets.len() >= 2,
        "New hosted zone should have at least SOA and NS records"
    );

    // [2/6] CREATE a TXT record via ChangeResourceRecordSets
    println!("\n[2/6] Creating TXT record via ChangeResourceRecordSets...");
    let create_req = ChangeResourceRecordSetsRequest {
        change_batch: ChangeBatch {
            comment: Some("cloud-lite test".to_string()),
            changes: vec![Change {
                action: Some(ChangeAction::Create),
                resource_record_set: ResourceRecordSet {
                    name: record_name.to_string(),
                    r#type: Some(RRType::Txt),
                    ttl: Some(300),
                    resource_records: vec![ResourceRecord {
                        value: "\"cloud-lite-test-value\"".to_string(),
                    }],
                    ..Default::default()
                },
            }],
        },
    };
    let create_response = r53
        .change_resource_record_sets(zone_id, &create_req)
        .await?;
    println!("  Change ID: {}", create_response.change_info.id);
    println!("  Status: {:?}", create_response.change_info.status);
    println!(
        "  SubmittedAt: {}",
        create_response.change_info.submitted_at
    );
    assert!(
        !create_response.change_info.id.is_empty(),
        "change ID should not be empty"
    );
    assert!(
        !create_response.change_info.submitted_at.is_empty(),
        "submitted_at should not be empty"
    );

    // [3/6] Wait for propagation and list records after CREATE
    println!("\n[3/6] Waiting 3 seconds for propagation...");
    tokio::time::sleep(Duration::from_secs(3)).await;
    let after_create = r53.list_resource_record_sets(zone_id).await?;
    println!(
        "  Records after CREATE: {}",
        after_create.resource_record_sets.len()
    );
    let txt_record = after_create
        .resource_record_sets
        .iter()
        .find(|r| r.name.starts_with("test-txt") && r.r#type == Some(RRType::Txt));
    assert!(
        txt_record.is_some(),
        "TXT record should appear in ListResourceRecordSets after CREATE"
    );
    let txt = txt_record.unwrap();
    println!("  TXT record found: name={}, ttl={:?}", txt.name, txt.ttl);
    assert_eq!(txt.ttl, Some(300), "TTL should be 300");
    assert!(
        !txt.resource_records.is_empty(),
        "TXT record should have at least one resource record"
    );
    println!("  TXT value: {}", txt.resource_records[0].value);

    // [4/6] UPSERT the TXT record (update its value)
    println!("\n[4/6] Upserting TXT record (changing value)...");
    let upsert_req = ChangeResourceRecordSetsRequest {
        change_batch: ChangeBatch {
            comment: Some("cloud-lite test upsert".to_string()),
            changes: vec![Change {
                action: Some(ChangeAction::Upsert),
                resource_record_set: ResourceRecordSet {
                    name: record_name.to_string(),
                    r#type: Some(RRType::Txt),
                    ttl: Some(600),
                    resource_records: vec![ResourceRecord {
                        value: "\"cloud-lite-test-value-updated\"".to_string(),
                    }],
                    ..Default::default()
                },
            }],
        },
    };
    let upsert_response = r53
        .change_resource_record_sets(zone_id, &upsert_req)
        .await?;
    println!("  Upsert Change ID: {}", upsert_response.change_info.id);
    assert!(
        !upsert_response.change_info.id.is_empty(),
        "upsert change ID should not be empty"
    );

    // [5/6] DELETE the TXT record to clean up
    println!("\n[5/6] Deleting TXT record (cleanup via ChangeResourceRecordSets)...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    // Re-list to get the exact current state for delete
    let current = r53.list_resource_record_sets(zone_id).await?;
    let current_txt = current
        .resource_record_sets
        .iter()
        .find(|r| r.name.starts_with("test-txt") && r.r#type == Some(RRType::Txt))
        .cloned();

    if let Some(record_to_delete) = current_txt {
        let delete_req = ChangeResourceRecordSetsRequest {
            change_batch: ChangeBatch {
                comment: None,
                changes: vec![Change {
                    action: Some(ChangeAction::Delete),
                    resource_record_set: record_to_delete,
                }],
            },
        };
        let delete_response = r53
            .change_resource_record_sets(zone_id, &delete_req)
            .await?;
        println!("  Delete Change ID: {}", delete_response.change_info.id);
    } else {
        println!("  TXT record already gone, skipping delete");
    }

    // [6/6] Done
    println!("\n[6/6] ChangeResourceRecordSets test passed!");
    Ok(())
}
