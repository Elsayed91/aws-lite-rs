//! Integration tests for DynamoDB API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration dynamodb -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::dynamodb::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_REGION: &str = "eu-central-1";
const TEST_TABLE_NAME: &str = "cloud-lite-test-ralph-dynamodb";

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
        None
    }
}

fn aws_cleanup(args: &[&str]) {
    let _ = Command::new("aws").args(args).output();
}

fn aws_create_table(region: &str) {
    aws(&[
        "dynamodb",
        "create-table",
        "--table-name",
        TEST_TABLE_NAME,
        "--attribute-definitions",
        "AttributeName=pk,AttributeType=S",
        "--key-schema",
        "AttributeName=pk,KeyType=HASH",
        "--billing-mode",
        "PAY_PER_REQUEST",
        "--region",
        region,
        "--output",
        "json",
    ]);
}

fn aws_delete_table(region: &str) {
    aws_cleanup(&[
        "dynamodb",
        "delete-table",
        "--table-name",
        TEST_TABLE_NAME,
        "--region",
        region,
    ]);
}

fn aws_wait_table_active(region: &str) {
    for _ in 0..30 {
        if let Some(v) = aws(&[
            "dynamodb",
            "describe-table",
            "--table-name",
            TEST_TABLE_NAME,
            "--region",
            region,
            "--output",
            "json",
        ]) {
            if v["Table"]["TableStatus"].as_str() == Some("ACTIVE") {
                return;
            }
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    panic!("Table did not become ACTIVE within timeout");
}

fn aws_wait_table_deleted(region: &str) {
    for _ in 0..30 {
        let result = aws(&[
            "dynamodb",
            "describe-table",
            "--table-name",
            TEST_TABLE_NAME,
            "--region",
            region,
            "--output",
            "json",
        ]);
        if result.is_none() {
            return;
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    panic!("Table was not deleted within timeout");
}

fn cleanup_all(region: &str) {
    // Check if table exists first
    let exists = aws(&[
        "dynamodb",
        "describe-table",
        "--table-name",
        TEST_TABLE_NAME,
        "--region",
        region,
        "--output",
        "json",
    ]);
    if exists.is_none() {
        return; // Table doesn't exist
    }
    // Wait for table to become ACTIVE or DELETING before attempting delete
    for _ in 0..30 {
        if let Some(v) = aws(&[
            "dynamodb",
            "describe-table",
            "--table-name",
            TEST_TABLE_NAME,
            "--region",
            region,
            "--output",
            "json",
        ]) {
            match v["Table"]["TableStatus"].as_str() {
                Some("ACTIVE") => {
                    aws_delete_table(region);
                    return;
                }
                Some("DELETING") => return, // Already deleting
                _ => std::thread::sleep(Duration::from_secs(2)),
            }
        } else {
            return; // Table gone
        }
    }
    // Best effort: try delete anyway
    aws_delete_table(region);
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_dynamodb_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    // Pre-cleanup
    cleanup_all(&region);
    aws_wait_table_deleted(&region);

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_dynamodb_tests(&client, &region).await;

    // Always cleanup
    cleanup_all(&region);

    result.expect("DynamoDB operations tests failed");
    println!("\nAll DynamoDB operations tests passed!");
}

async fn run_dynamodb_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== DynamoDB Operations Test ===");
    println!("Region: {region}");

    // [0/6] Create fixtures
    println!("\n[0/6] Creating test table via CLI...");
    aws_create_table(region);
    aws_wait_table_active(region);
    println!("  Table {TEST_TABLE_NAME} is ACTIVE");

    // [1/6] ListTables
    println!("\n[1/6] ListTables...");
    let list_resp = client
        .dynamodb()
        .list_tables(&ListTablesInput::default())
        .await?;
    assert!(
        list_resp.table_names.contains(&TEST_TABLE_NAME.to_string()),
        "Expected table list to contain {TEST_TABLE_NAME}, got: {:?}",
        list_resp.table_names,
    );
    println!(
        "  Found {} table(s), includes {TEST_TABLE_NAME}",
        list_resp.table_names.len()
    );

    // [2/6] DescribeTable
    println!("\n[2/6] DescribeTable...");
    let desc_resp = client
        .dynamodb()
        .describe_table(&DescribeTableInput {
            table_name: TEST_TABLE_NAME.to_string(),
        })
        .await?;
    let table = desc_resp
        .table
        .as_ref()
        .expect("Expected Table in response");
    assert_eq!(table.table_name.as_deref(), Some(TEST_TABLE_NAME));
    assert_eq!(table.table_status.as_deref(), Some("ACTIVE"));
    assert!(table.table_arn.is_some());
    assert!(table.creation_date_time.is_some());
    assert!(!table.key_schema.is_empty(), "Expected key schema");
    assert_eq!(table.key_schema[0].attribute_name, "pk");
    assert_eq!(table.key_schema[0].key_type, "HASH");
    assert!(!table.attribute_definitions.is_empty());
    assert_eq!(table.attribute_definitions[0].attribute_name, "pk");
    assert_eq!(table.attribute_definitions[0].attribute_type, "S");
    // PAY_PER_REQUEST billing mode
    let billing = table.billing_mode_summary.as_ref();
    assert!(
        billing.is_some(),
        "Expected billing mode summary for on-demand table"
    );
    assert_eq!(
        billing.unwrap().billing_mode.as_deref(),
        Some("PAY_PER_REQUEST"),
    );
    println!(
        "  Table: {} status={} arn={} billing={}",
        table.table_name.as_deref().unwrap_or("?"),
        table.table_status.as_deref().unwrap_or("?"),
        table.table_arn.as_deref().unwrap_or("?"),
        billing
            .and_then(|b| b.billing_mode.as_deref())
            .unwrap_or("?"),
    );

    // [3/6] UpdateTable — switch from PAY_PER_REQUEST to PROVISIONED
    println!("\n[3/6] UpdateTable (switch to PROVISIONED)...");
    let update_resp = client
        .dynamodb()
        .update_table(&UpdateTableInput {
            table_name: TEST_TABLE_NAME.to_string(),
            billing_mode: Some("PROVISIONED".to_string()),
            provisioned_throughput: Some(ProvisionedThroughput {
                read_capacity_units: 5,
                write_capacity_units: 5,
            }),
            ..Default::default()
        })
        .await?;
    let updated = update_resp
        .table_description
        .as_ref()
        .expect("Expected TableDescription in UpdateTable response");
    assert_eq!(updated.table_name.as_deref(), Some(TEST_TABLE_NAME));
    // Table may be UPDATING or ACTIVE
    assert!(
        updated.table_status.as_deref() == Some("UPDATING")
            || updated.table_status.as_deref() == Some("ACTIVE"),
        "Expected UPDATING or ACTIVE, got: {:?}",
        updated.table_status,
    );
    println!(
        "  Updated table status={}, throughput: read={:?} write={:?}",
        updated.table_status.as_deref().unwrap_or("?"),
        updated
            .provisioned_throughput
            .as_ref()
            .and_then(|p| p.read_capacity_units),
        updated
            .provisioned_throughput
            .as_ref()
            .and_then(|p| p.write_capacity_units),
    );

    // [4/6] Verify update via DescribeTable
    println!("\n[4/6] DescribeTable (verify billing change)...");
    // Wait for the table to settle
    tokio::time::sleep(Duration::from_secs(5)).await;
    let desc_resp2 = client
        .dynamodb()
        .describe_table(&DescribeTableInput {
            table_name: TEST_TABLE_NAME.to_string(),
        })
        .await?;
    let table2 = desc_resp2.table.as_ref().expect("Expected Table");
    let throughput = table2.provisioned_throughput.as_ref();
    assert!(
        throughput.is_some(),
        "Expected provisioned throughput after update"
    );
    let tp = throughput.unwrap();
    assert_eq!(tp.read_capacity_units, Some(5));
    assert_eq!(tp.write_capacity_units, Some(5));
    println!(
        "  Verified: status={} RCU={:?} WCU={:?}",
        table2.table_status.as_deref().unwrap_or("?"),
        tp.read_capacity_units,
        tp.write_capacity_units,
    );

    // [5/6] DeleteTable
    println!("\n[5/6] DeleteTable...");
    // Wait for table to finish updating before deleting
    aws_wait_table_active(region);
    let delete_resp = client
        .dynamodb()
        .delete_table(&DeleteTableInput {
            table_name: TEST_TABLE_NAME.to_string(),
        })
        .await?;
    let deleted = delete_resp
        .table_description
        .as_ref()
        .expect("Expected TableDescription in DeleteTable response");
    assert_eq!(deleted.table_name.as_deref(), Some(TEST_TABLE_NAME));
    assert!(
        deleted.table_status.as_deref() == Some("DELETING")
            || deleted.table_status.as_deref() == Some("ACTIVE"),
        "Expected DELETING or ACTIVE, got: {:?}",
        deleted.table_status,
    );
    println!(
        "  Deleted table: {} status={}",
        deleted.table_name.as_deref().unwrap_or("?"),
        deleted.table_status.as_deref().unwrap_or("?"),
    );

    // [6/6] DescribeTable error case — non-existent table
    println!("\n[6/6] DescribeTable error case...");
    aws_wait_table_deleted(region);
    let err = client
        .dynamodb()
        .describe_table(&DescribeTableInput {
            table_name: "cloud-lite-test-nonexistent-table".to_string(),
        })
        .await;
    assert!(err.is_err(), "Expected error for non-existent table");
    let err_msg = format!("{}", err.unwrap_err());
    assert!(
        err_msg.contains("ResourceNotFoundException") || err_msg.contains("not found"),
        "Expected ResourceNotFoundException, got: {err_msg}",
    );
    println!("  Got expected error: {err_msg}");

    Ok(())
}
