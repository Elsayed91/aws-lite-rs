//! Integration tests for Amazon Redshift API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration redshift -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::redshift::*;
use std::env;
use std::process::Command;

const TEST_CLUSTER_ID: &str = "cloud-lite-test-ralph-rs";

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "eu-central-1".into())
}

// --- CLI helpers ---

fn aws_create_redshift_cluster(cluster_id: &str, region: &str) -> bool {
    let output = Command::new("aws")
        .args([
            "redshift",
            "create-cluster",
            "--cluster-identifier",
            cluster_id,
            "--node-type",
            "dc2.large",
            "--cluster-type",
            "single-node",
            "--master-username",
            "admin",
            "--master-user-password",
            "CloudLiteTest1234!",
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
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("ClusterAlreadyExists") {
        return true;
    }
    eprintln!("  Failed to create cluster: {stderr}");
    false
}

fn aws_wait_cluster_available(cluster_id: &str, region: &str) {
    println!(
        "  Waiting for Redshift cluster to become available (this may take several minutes)..."
    );
    let _ = Command::new("aws")
        .args([
            "redshift",
            "wait",
            "cluster-available",
            "--cluster-identifier",
            cluster_id,
            "--region",
            region,
        ])
        .output();
}

fn aws_delete_redshift_cluster(cluster_id: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "redshift",
            "delete-cluster",
            "--cluster-identifier",
            cluster_id,
            "--skip-final-cluster-snapshot",
            "--region",
            region,
        ])
        .output();
}

fn aws_wait_cluster_deleted(cluster_id: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "redshift",
            "wait",
            "cluster-deleted",
            "--cluster-identifier",
            cluster_id,
            "--region",
            region,
        ])
        .output();
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials — creates real Redshift resources"]
async fn test_redshift_describe_clusters() {
    let region = region();

    // Pre-cleanup
    aws_delete_redshift_cluster(TEST_CLUSTER_ID, &region);
    aws_wait_cluster_deleted(TEST_CLUSTER_ID, &region);

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_redshift_read_tests(&client, &region).await;

    // Always cleanup
    aws_delete_redshift_cluster(TEST_CLUSTER_ID, &region);

    result.expect("Redshift read tests failed");
    println!("\nAll Redshift read tests passed!");
}

async fn run_redshift_read_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Redshift Read Operations Test ===");
    println!("Region: {region}");

    // [0/3] Create test Redshift cluster
    println!(
        "\n[0/3] Creating test Redshift cluster: {}...",
        TEST_CLUSTER_ID
    );
    let created = aws_create_redshift_cluster(TEST_CLUSTER_ID, region);
    assert!(created, "Failed to create test Redshift cluster");
    println!("  Cluster creation initiated");

    aws_wait_cluster_available(TEST_CLUSTER_ID, region);
    println!("  Cluster is available");

    // [1/3] DescribeClusters — specific cluster
    println!("\n[1/3] DescribeClusters (specific)...");
    let response = client
        .redshift()
        .describe_clusters(&DescribeClustersRequest {
            cluster_identifier: Some(TEST_CLUSTER_ID.into()),
            ..Default::default()
        })
        .await?;
    assert_eq!(
        response.clusters.len(),
        1,
        "should find exactly one cluster"
    );
    let cluster = &response.clusters[0];
    assert_eq!(cluster.cluster_identifier.as_deref(), Some(TEST_CLUSTER_ID));
    assert_eq!(cluster.node_type.as_deref(), Some("dc2.large"));
    assert_eq!(cluster.cluster_status.as_deref(), Some("available"));
    assert_eq!(cluster.number_of_nodes, Some(1));
    assert!(cluster.encrypted.is_some(), "should have encrypted field");
    assert!(
        cluster.publicly_accessible.is_some(),
        "should have publicly_accessible field"
    );
    assert!(
        cluster.cluster_namespace_arn.is_some(),
        "should have cluster_namespace_arn"
    );
    assert!(
        cluster.cluster_create_time.is_some(),
        "should have create time"
    );
    // Check endpoint
    if let Some(ep) = &cluster.endpoint {
        println!("  Endpoint: address={:?} port={:?}", ep.address, ep.port);
        assert!(ep.address.is_some(), "endpoint should have address");
        assert_eq!(ep.port, Some(5439), "default Redshift port is 5439");
    }
    println!(
        "  Cluster: id={:?} status={:?} type={:?} nodes={:?} encrypted={:?} public={:?}",
        cluster.cluster_identifier,
        cluster.cluster_status,
        cluster.node_type,
        cluster.number_of_nodes,
        cluster.encrypted,
        cluster.publicly_accessible,
    );

    // [2/3] DescribeClusters — list all
    println!("\n[2/3] DescribeClusters (list all)...");
    let response = client
        .redshift()
        .describe_clusters(&DescribeClustersRequest::default())
        .await?;
    assert!(
        !response.clusters.is_empty(),
        "should find at least the test cluster"
    );
    println!("  Total clusters: {}", response.clusters.len());
    let found = response
        .clusters
        .iter()
        .any(|c| c.cluster_identifier.as_deref() == Some(TEST_CLUSTER_ID));
    assert!(found, "test cluster should appear in list-all response");

    // [3/3] DescribeClusters — non-existent (expect error)
    println!("\n[3/3] DescribeClusters (non-existent, expect error)...");
    let result = client
        .redshift()
        .describe_clusters(&DescribeClustersRequest {
            cluster_identifier: Some("cloud-lite-test-nonexistent-12345".into()),
            ..Default::default()
        })
        .await;
    assert!(result.is_err(), "non-existent cluster should fail");
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("not found") || err.contains("ClusterNotFound"),
        "expected not-found error, got: {err}"
    );
    println!("  Got expected error: {err}");

    Ok(())
}
