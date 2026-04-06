//! Integration tests for Amazon RDS API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration rds -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::rds::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_DB_INSTANCE_ID: &str = "cloud-lite-test-ralph-rds";
const TEST_SNAPSHOT_ID: &str = "cloud-lite-test-ralph-rds-snap";

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "eu-central-1".into())
}

// --- CLI helpers ---

fn aws_create_db_instance(instance_id: &str, region: &str) -> bool {
    let output = Command::new("aws")
        .args([
            "rds",
            "create-db-instance",
            "--db-instance-identifier",
            instance_id,
            "--db-instance-class",
            "db.t3.micro",
            "--engine",
            "mysql",
            "--master-username",
            "admin",
            "--master-user-password",
            "CloudLiteTest1234!",
            "--allocated-storage",
            "20",
            "--backup-retention-period",
            "0",
            "--no-multi-az",
            "--no-auto-minor-version-upgrade",
            "--region",
            region,
            "--output",
            "json",
        ])
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        return true;
    }
    // Treat "already exists" as success — instance from a prior run
    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.contains("DBInstanceAlreadyExists")
}

fn aws_wait_db_available(instance_id: &str, region: &str) {
    println!("  Waiting for DB instance to become available (this may take several minutes)...");
    let _ = Command::new("aws")
        .args([
            "rds",
            "wait",
            "db-instance-available",
            "--db-instance-identifier",
            instance_id,
            "--region",
            region,
        ])
        .output();
}

fn aws_delete_db_instance(instance_id: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "rds",
            "delete-db-instance",
            "--db-instance-identifier",
            instance_id,
            "--skip-final-snapshot",
            "--delete-automated-backups",
            "--region",
            region,
        ])
        .output();
}

fn aws_delete_db_snapshot(snapshot_id: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "rds",
            "delete-db-snapshot",
            "--db-snapshot-identifier",
            snapshot_id,
            "--region",
            region,
        ])
        .output();
}

fn aws_wait_db_deleted(instance_id: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "rds",
            "wait",
            "db-instance-deleted",
            "--db-instance-identifier",
            instance_id,
            "--region",
            region,
        ])
        .output();
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials — creates real RDS resources (slow)"]
async fn test_rds_describe_instances_and_snapshots() {
    let region = region();

    // Pre-cleanup
    aws_delete_db_snapshot(TEST_SNAPSHOT_ID, &region);
    aws_delete_db_instance(TEST_DB_INSTANCE_ID, &region);
    aws_wait_db_deleted(TEST_DB_INSTANCE_ID, &region);

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_rds_read_tests(&client, &region).await;

    // Always cleanup
    aws_delete_db_snapshot(TEST_SNAPSHOT_ID, &region);
    aws_delete_db_instance(TEST_DB_INSTANCE_ID, &region);

    result.expect("RDS read tests failed");
    println!("\nAll RDS read tests passed!");
}

async fn run_rds_read_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== RDS Read Operations Test ===");
    println!("Region: {region}");

    // [0/6] Create test DB instance
    println!(
        "\n[0/6] Creating test DB instance: {}...",
        TEST_DB_INSTANCE_ID
    );
    let created = aws_create_db_instance(TEST_DB_INSTANCE_ID, region);
    assert!(created, "Failed to create test DB instance");
    println!("  DB instance creation initiated");

    aws_wait_db_available(TEST_DB_INSTANCE_ID, region);
    println!("  DB instance is available");

    // [1/6] DescribeDBInstances — specific instance
    println!("\n[1/6] DescribeDBInstances (specific)...");
    let response = client
        .rds()
        .describe_db_instances(&DescribeDBInstancesRequest {
            db_instance_identifier: Some(TEST_DB_INSTANCE_ID.into()),
            ..Default::default()
        })
        .await?;
    assert_eq!(
        response.db_instances.len(),
        1,
        "should find exactly one instance"
    );
    let instance = &response.db_instances[0];
    assert_eq!(
        instance.db_instance_identifier.as_deref(),
        Some(TEST_DB_INSTANCE_ID)
    );
    assert_eq!(instance.engine.as_deref(), Some("mysql"));
    assert_eq!(instance.db_instance_class.as_deref(), Some("db.t3.micro"));
    assert_eq!(instance.db_instance_status.as_deref(), Some("available"));
    assert_eq!(instance.allocated_storage, Some(20));
    assert!(
        instance.endpoint.is_some(),
        "should have endpoint when available"
    );
    let endpoint = instance.endpoint.as_ref().unwrap();
    assert!(endpoint.address.is_some(), "endpoint should have address");
    assert!(endpoint.port.is_some(), "endpoint should have port");
    println!(
        "  Instance: id={:?} status={:?} engine={:?} class={:?} endpoint={:?}:{}",
        instance.db_instance_identifier,
        instance.db_instance_status,
        instance.engine,
        instance.db_instance_class,
        endpoint.address,
        endpoint.port.unwrap_or(0)
    );

    // [2/6] DescribeDBInstances — list all (verify pagination structure)
    println!("\n[2/6] DescribeDBInstances (list all)...");
    let response = client
        .rds()
        .describe_db_instances(&DescribeDBInstancesRequest::default())
        .await?;
    assert!(
        !response.db_instances.is_empty(),
        "should find at least the test instance"
    );
    println!("  Total DB instances: {}", response.db_instances.len());
    let found = response
        .db_instances
        .iter()
        .any(|i| i.db_instance_identifier.as_deref() == Some(TEST_DB_INSTANCE_ID));
    assert!(found, "test instance should appear in list-all response");

    // [3/6] CreateDBSnapshot (via library) then DescribeDBSnapshots
    println!("\n[3/6] Creating DB snapshot: {}...", TEST_SNAPSHOT_ID);
    let snap_response = client
        .rds()
        .create_db_snapshot(&CreateDBSnapshotRequest {
            db_snapshot_identifier: TEST_SNAPSHOT_ID.into(),
            db_instance_identifier: TEST_DB_INSTANCE_ID.into(),
            ..Default::default()
        })
        .await?;
    let snap = snap_response
        .db_snapshot
        .as_ref()
        .expect("should return snapshot");
    assert_eq!(
        snap.db_snapshot_identifier.as_deref(),
        Some(TEST_SNAPSHOT_ID)
    );
    println!("  Snapshot creation initiated: status={:?}", snap.status);

    // Wait for snapshot to become available
    println!("  Waiting for snapshot to become available...");
    loop {
        tokio::time::sleep(Duration::from_secs(15)).await;
        let check = client
            .rds()
            .describe_db_snapshots(&DescribeDBSnapshotsRequest {
                db_snapshot_identifier: Some(TEST_SNAPSHOT_ID.into()),
                ..Default::default()
            })
            .await?;
        if let Some(s) = check.db_snapshots.first() {
            let status = s.status.as_deref().unwrap_or("unknown");
            println!("    status={status}");
            if status == "available" {
                break;
            }
        }
    }

    // [4/6] DescribeDBSnapshots — specific snapshot
    println!("\n[4/6] DescribeDBSnapshots (specific)...");
    let response = client
        .rds()
        .describe_db_snapshots(&DescribeDBSnapshotsRequest {
            db_snapshot_identifier: Some(TEST_SNAPSHOT_ID.into()),
            ..Default::default()
        })
        .await?;
    assert_eq!(
        response.db_snapshots.len(),
        1,
        "should find exactly one snapshot"
    );
    let snapshot = &response.db_snapshots[0];
    assert_eq!(
        snapshot.db_snapshot_identifier.as_deref(),
        Some(TEST_SNAPSHOT_ID)
    );
    assert_eq!(
        snapshot.db_instance_identifier.as_deref(),
        Some(TEST_DB_INSTANCE_ID)
    );
    assert_eq!(snapshot.engine.as_deref(), Some("mysql"));
    assert_eq!(snapshot.status.as_deref(), Some("available"));
    assert_eq!(snapshot.allocated_storage, Some(20));
    println!(
        "  Snapshot: id={:?} status={:?} engine={:?} storage={:?}",
        snapshot.db_snapshot_identifier,
        snapshot.status,
        snapshot.engine,
        snapshot.allocated_storage
    );

    // [5/6] DescribeDBSnapshotAttributes
    println!("\n[5/6] DescribeDBSnapshotAttributes...");
    let response = client
        .rds()
        .describe_db_snapshot_attributes(&DescribeDBSnapshotAttributesRequest {
            db_snapshot_identifier: TEST_SNAPSHOT_ID.into(),
        })
        .await?;
    let attrs_result = response
        .db_snapshot_attributes_result
        .as_ref()
        .expect("should have attributes result");
    assert_eq!(
        attrs_result.db_snapshot_identifier.as_deref(),
        Some(TEST_SNAPSHOT_ID)
    );
    println!(
        "  Attributes: snapshot={:?} count={}",
        attrs_result.db_snapshot_identifier,
        attrs_result.db_snapshot_attributes.len()
    );
    // The default snapshot should have a 'restore' attribute with no values
    let restore_attr = attrs_result
        .db_snapshot_attributes
        .iter()
        .find(|a| a.attribute_name.as_deref() == Some("restore"));
    assert!(restore_attr.is_some(), "should have 'restore' attribute");
    println!(
        "  Restore attribute values: {:?}",
        restore_attr.unwrap().attribute_values
    );

    // [6/6] DescribeDBInstances error — non-existent instance
    println!("\n[6/6] DescribeDBInstances (non-existent, expect error)...");
    let result = client
        .rds()
        .describe_db_instances(&DescribeDBInstancesRequest {
            db_instance_identifier: Some("cloud-lite-test-nonexistent-12345".into()),
            ..Default::default()
        })
        .await;
    assert!(result.is_err(), "non-existent instance should fail");
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("not found") || err.contains("DBInstanceNotFound"),
        "expected not-found error, got: {err}"
    );
    println!("  Got expected error: {err}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials — creates real RDS resources (very slow: stop/start cycle)"]
async fn test_rds_write_operations() {
    let region = region();

    // Pre-cleanup
    aws_delete_db_snapshot(TEST_SNAPSHOT_ID, &region);
    aws_delete_db_instance(TEST_DB_INSTANCE_ID, &region);
    aws_wait_db_deleted(TEST_DB_INSTANCE_ID, &region);

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_rds_write_tests(&client, &region).await;

    // Always cleanup
    aws_delete_db_snapshot(TEST_SNAPSHOT_ID, &region);
    aws_delete_db_instance(TEST_DB_INSTANCE_ID, &region);

    result.expect("RDS write tests failed");
    println!("\nAll RDS write tests passed!");
}

async fn run_rds_write_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== RDS Write Operations Test ===");
    println!("Region: {region}");

    // [0/7] Create test DB instance
    println!(
        "\n[0/7] Creating test DB instance: {}...",
        TEST_DB_INSTANCE_ID
    );
    let created = aws_create_db_instance(TEST_DB_INSTANCE_ID, region);
    assert!(created, "Failed to create test DB instance");
    aws_wait_db_available(TEST_DB_INSTANCE_ID, region);
    println!("  DB instance is available");

    // [1/7] ModifyDBInstance — change backup retention period
    println!("\n[1/7] ModifyDBInstance (change backup retention)...");
    let response = client
        .rds()
        .modify_db_instance(&ModifyDBInstanceRequest {
            db_instance_identifier: TEST_DB_INSTANCE_ID.into(),
            backup_retention_period: Some(1),
            apply_immediately: Some(true),
            ..Default::default()
        })
        .await?;
    let modified = response
        .db_instance
        .as_ref()
        .expect("should return instance");
    assert_eq!(
        modified.db_instance_identifier.as_deref(),
        Some(TEST_DB_INSTANCE_ID)
    );
    println!(
        "  Modified: id={:?} status={:?}",
        modified.db_instance_identifier, modified.db_instance_status
    );

    // Wait for modification to complete
    aws_wait_db_available(TEST_DB_INSTANCE_ID, region);
    println!("  Modification complete, instance available");

    // [2/7] CreateDBSnapshot + wait
    println!("\n[2/7] CreateDBSnapshot...");
    let snap_resp = client
        .rds()
        .create_db_snapshot(&CreateDBSnapshotRequest {
            db_snapshot_identifier: TEST_SNAPSHOT_ID.into(),
            db_instance_identifier: TEST_DB_INSTANCE_ID.into(),
            ..Default::default()
        })
        .await?;
    let snap = snap_resp
        .db_snapshot
        .as_ref()
        .expect("should return snapshot");
    assert_eq!(
        snap.db_snapshot_identifier.as_deref(),
        Some(TEST_SNAPSHOT_ID)
    );
    println!("  Snapshot creation initiated: status={:?}", snap.status);

    // Wait for snapshot
    println!("  Waiting for snapshot to become available...");
    loop {
        tokio::time::sleep(Duration::from_secs(15)).await;
        let check = client
            .rds()
            .describe_db_snapshots(&DescribeDBSnapshotsRequest {
                db_snapshot_identifier: Some(TEST_SNAPSHOT_ID.into()),
                ..Default::default()
            })
            .await?;
        if let Some(s) = check.db_snapshots.first() {
            let status = s.status.as_deref().unwrap_or("unknown");
            println!("    status={status}");
            if status == "available" {
                break;
            }
        }
    }

    // [3/7] ModifyDBSnapshotAttribute — share snapshot with a dummy account
    println!("\n[3/7] ModifyDBSnapshotAttribute (add restore account)...");
    let response = client
        .rds()
        .modify_db_snapshot_attribute(&ModifyDBSnapshotAttributeRequest {
            db_snapshot_identifier: TEST_SNAPSHOT_ID.into(),
            attribute_name: "restore".into(),
            values_to_add: vec!["123456789012".into()],
            ..Default::default()
        })
        .await?;
    let attrs_result = response
        .db_snapshot_attributes_result
        .as_ref()
        .expect("should have attributes result");
    assert_eq!(
        attrs_result.db_snapshot_identifier.as_deref(),
        Some(TEST_SNAPSHOT_ID)
    );
    let restore_attr = attrs_result
        .db_snapshot_attributes
        .iter()
        .find(|a| a.attribute_name.as_deref() == Some("restore"));
    assert!(restore_attr.is_some(), "should have restore attribute");
    let values = &restore_attr.unwrap().attribute_values;
    assert!(
        values.iter().any(|v| v == "123456789012"),
        "should include shared account, got: {values:?}"
    );
    println!("  Shared snapshot with account 123456789012: values={values:?}");

    // [4/7] DeleteDBSnapshot
    println!("\n[4/7] DeleteDBSnapshot...");
    let response = client
        .rds()
        .delete_db_snapshot(&DeleteDBSnapshotRequest {
            db_snapshot_identifier: TEST_SNAPSHOT_ID.into(),
        })
        .await?;
    let deleted_snap = response
        .db_snapshot
        .as_ref()
        .expect("should return snapshot");
    assert_eq!(
        deleted_snap.db_snapshot_identifier.as_deref(),
        Some(TEST_SNAPSHOT_ID)
    );
    println!(
        "  Deleted snapshot: id={:?} status={:?}",
        deleted_snap.db_snapshot_identifier, deleted_snap.status
    );

    // [5/7] StopDBInstance — ensure instance is available first
    println!("\n[5/7] StopDBInstance...");
    aws_wait_db_available(TEST_DB_INSTANCE_ID, region);
    let response = client
        .rds()
        .stop_db_instance(&StopDBInstanceRequest {
            db_instance_identifier: TEST_DB_INSTANCE_ID.into(),
            ..Default::default()
        })
        .await?;
    let stopped = response
        .db_instance
        .as_ref()
        .expect("should return instance");
    assert_eq!(
        stopped.db_instance_identifier.as_deref(),
        Some(TEST_DB_INSTANCE_ID)
    );
    println!("  Stop initiated: status={:?}", stopped.db_instance_status);

    // Wait for stop — poll via library to confirm "stopped" state
    println!("  Waiting for instance to stop...");
    loop {
        tokio::time::sleep(Duration::from_secs(15)).await;
        let check = client
            .rds()
            .describe_db_instances(&DescribeDBInstancesRequest {
                db_instance_identifier: Some(TEST_DB_INSTANCE_ID.into()),
                ..Default::default()
            })
            .await?;
        if let Some(inst) = check.db_instances.first() {
            let status = inst.db_instance_status.as_deref().unwrap_or("unknown");
            println!("    status={status}");
            if status == "stopped" {
                break;
            }
        }
    }
    println!("  Instance stopped");

    // [6/7] StartDBInstance
    println!("\n[6/7] StartDBInstance...");
    let response = client
        .rds()
        .start_db_instance(&StartDBInstanceRequest {
            db_instance_identifier: TEST_DB_INSTANCE_ID.into(),
        })
        .await?;
    let started = response
        .db_instance
        .as_ref()
        .expect("should return instance");
    assert_eq!(
        started.db_instance_identifier.as_deref(),
        Some(TEST_DB_INSTANCE_ID)
    );
    println!("  Start initiated: status={:?}", started.db_instance_status);

    aws_wait_db_available(TEST_DB_INSTANCE_ID, region);
    println!("  Instance available again");

    // [7/7] DeleteDBInstance
    println!("\n[7/7] DeleteDBInstance...");
    let response = client
        .rds()
        .delete_db_instance(&DeleteDBInstanceRequest {
            db_instance_identifier: TEST_DB_INSTANCE_ID.into(),
            skip_final_snapshot: Some(true),
            delete_automated_backups: Some(true),
            ..Default::default()
        })
        .await?;
    let deleted = response
        .db_instance
        .as_ref()
        .expect("should return instance");
    assert_eq!(
        deleted.db_instance_identifier.as_deref(),
        Some(TEST_DB_INSTANCE_ID)
    );
    println!(
        "  Delete initiated: status={:?}",
        deleted.db_instance_status
    );

    Ok(())
}
