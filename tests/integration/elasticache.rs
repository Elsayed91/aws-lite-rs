//! Integration tests for Amazon ElastiCache API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration elasticache -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::elasticache::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_CLUSTER_ID: &str = "cloud-lite-test-ralph-ec";
const TEST_REDIS_CLUSTER_ID: &str = "cloud-lite-test-ralph-ecr";
const TEST_REPL_GROUP_ID: &str = "cloud-lite-test-ralph-rg";
const TEST_SNAPSHOT_NAME: &str = "cloud-lite-test-ralph-ec-snap";

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "eu-central-1".into())
}

// --- CLI helpers ---

fn aws_create_cache_cluster(cluster_id: &str, region: &str) -> bool {
    let output = Command::new("aws")
        .args([
            "elasticache",
            "create-cache-cluster",
            "--cache-cluster-id",
            cluster_id,
            "--cache-node-type",
            "cache.t3.micro",
            "--engine",
            "memcached",
            "--num-cache-nodes",
            "1",
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
    stderr.contains("CacheClusterAlreadyExists")
}

fn aws_wait_cache_available(cluster_id: &str, region: &str) {
    println!("  Waiting for cache cluster to become available...");
    let _ = Command::new("aws")
        .args([
            "elasticache",
            "wait",
            "cache-cluster-available",
            "--cache-cluster-id",
            cluster_id,
            "--region",
            region,
        ])
        .output();
}

fn aws_delete_cache_cluster(cluster_id: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "elasticache",
            "delete-cache-cluster",
            "--cache-cluster-id",
            cluster_id,
            "--region",
            region,
        ])
        .output();
}

fn aws_wait_cache_deleted(cluster_id: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "elasticache",
            "wait",
            "cache-cluster-deleted",
            "--cache-cluster-id",
            cluster_id,
            "--region",
            region,
        ])
        .output();
}

// --- CLI helpers for write ops ---

fn aws_create_replication_group(rg_id: &str, region: &str) -> bool {
    let output = Command::new("aws")
        .args([
            "elasticache",
            "create-replication-group",
            "--replication-group-id",
            rg_id,
            "--replication-group-description",
            "cloud-lite integration test replication group",
            "--cache-node-type",
            "cache.t3.micro",
            "--engine",
            "redis",
            "--num-cache-clusters",
            "1",
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
    stderr.contains("ReplicationGroupAlreadyExists")
}

fn aws_wait_replication_group_available(rg_id: &str, region: &str) {
    println!("  Waiting for replication group to become available...");
    let _ = Command::new("aws")
        .args([
            "elasticache",
            "wait",
            "replication-group-available",
            "--replication-group-id",
            rg_id,
            "--region",
            region,
        ])
        .output();
}

fn aws_delete_replication_group(rg_id: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "elasticache",
            "delete-replication-group",
            "--replication-group-id",
            rg_id,
            "--no-retain-primary-cluster",
            "--region",
            region,
        ])
        .output();
}

fn aws_wait_replication_group_deleted(rg_id: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "elasticache",
            "wait",
            "replication-group-deleted",
            "--replication-group-id",
            rg_id,
            "--region",
            region,
        ])
        .output();
}

fn aws_delete_snapshot(snapshot_name: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "elasticache",
            "delete-snapshot",
            "--snapshot-name",
            snapshot_name,
            "--region",
            region,
        ])
        .output();
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials — creates real ElastiCache resources"]
async fn test_elasticache_describe_clusters_and_replication_groups() {
    let region = region();

    // Pre-cleanup
    aws_delete_cache_cluster(TEST_CLUSTER_ID, &region);
    aws_wait_cache_deleted(TEST_CLUSTER_ID, &region);

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_elasticache_read_tests(&client, &region).await;

    // Always cleanup
    aws_delete_cache_cluster(TEST_CLUSTER_ID, &region);

    result.expect("ElastiCache read tests failed");
    println!("\nAll ElastiCache read tests passed!");
}

#[tokio::test]
#[ignore = "requires AWS credentials — creates real ElastiCache resources"]
async fn test_elasticache_write_operations() {
    let region = region();

    // Pre-cleanup: delete any leftover resources from previous runs
    aws_delete_snapshot(TEST_SNAPSHOT_NAME, &region);
    aws_delete_cache_cluster(TEST_REDIS_CLUSTER_ID, &region);
    aws_delete_replication_group(TEST_REPL_GROUP_ID, &region);
    aws_wait_replication_group_deleted(TEST_REPL_GROUP_ID, &region);
    aws_wait_cache_deleted(TEST_REDIS_CLUSTER_ID, &region);

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_elasticache_write_tests(&client, &region).await;

    // Always cleanup
    aws_delete_snapshot(TEST_SNAPSHOT_NAME, &region);
    aws_delete_replication_group(TEST_REPL_GROUP_ID, &region);
    aws_delete_cache_cluster(TEST_REDIS_CLUSTER_ID, &region);

    result.expect("ElastiCache write tests failed");
    println!("\nAll ElastiCache write tests passed!");
}

async fn run_elasticache_write_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== ElastiCache Write Operations Test ===");
    println!("Region: {region}");

    // [0/5] Create a Redis replication group (needed for snapshot + delete tests)
    println!(
        "\n[0/5] Creating Redis replication group: {}...",
        TEST_REPL_GROUP_ID
    );
    let created = aws_create_replication_group(TEST_REPL_GROUP_ID, region);
    assert!(created, "Failed to create replication group");
    println!("  Replication group creation initiated");
    aws_wait_replication_group_available(TEST_REPL_GROUP_ID, region);
    println!("  Replication group is available");

    // [1/5] CreateSnapshot — snapshot the cluster member of the replication group
    // For cluster-mode disabled replication groups, must specify cache_cluster_id
    let member_cluster_id = format!("{}-001", TEST_REPL_GROUP_ID);
    println!(
        "\n[1/5] CreateSnapshot on cache cluster: {}...",
        member_cluster_id
    );
    let snap_resp = client
        .elasticache()
        .create_snapshot(&CreateSnapshotRequest {
            snapshot_name: TEST_SNAPSHOT_NAME.into(),
            cache_cluster_id: Some(member_cluster_id.clone()),
            ..Default::default()
        })
        .await?;
    let snapshot = snap_resp.snapshot.as_ref().expect("should return snapshot");
    assert_eq!(snapshot.snapshot_name.as_deref(), Some(TEST_SNAPSHOT_NAME));
    assert!(
        snapshot.snapshot_status.as_deref() == Some("creating")
            || snapshot.snapshot_status.as_deref() == Some("available"),
        "snapshot status should be creating or available, got: {:?}",
        snapshot.snapshot_status,
    );
    println!(
        "  Snapshot: name={:?} status={:?} source={:?} engine={:?}",
        snapshot.snapshot_name, snapshot.snapshot_status, snapshot.snapshot_source, snapshot.engine,
    );

    // Wait for snapshot to complete before deleting the replication group
    println!("  Waiting for snapshot to become available...");
    loop {
        tokio::time::sleep(Duration::from_secs(15)).await;
        // Check snapshot status by describing cache clusters (there's no DescribeSnapshots op)
        // Use CLI to check snapshot status
        let output = Command::new("aws")
            .args([
                "elasticache",
                "describe-snapshots",
                "--snapshot-name",
                TEST_SNAPSHOT_NAME,
                "--region",
                region,
                "--query",
                "Snapshots[0].SnapshotStatus",
                "--output",
                "text",
            ])
            .output()
            .expect("aws cli must be installed");
        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("    snapshot status={status}");
        if status == "available" {
            break;
        }
    }

    // [2/5] DeleteReplicationGroup — delete the replication group via library
    println!("\n[2/5] DeleteReplicationGroup...");
    let del_rg_resp = client
        .elasticache()
        .delete_replication_group(&DeleteReplicationGroupRequest {
            replication_group_id: TEST_REPL_GROUP_ID.into(),
            ..Default::default()
        })
        .await?;
    let rg = del_rg_resp
        .replication_group
        .as_ref()
        .expect("should return replication group");
    assert_eq!(rg.replication_group_id.as_deref(), Some(TEST_REPL_GROUP_ID));
    println!(
        "  Deleted RG: id={:?} status={:?}",
        rg.replication_group_id, rg.status
    );

    // Wait for replication group to finish deleting before creating standalone cluster
    println!("  Waiting for replication group deletion...");
    aws_wait_replication_group_deleted(TEST_REPL_GROUP_ID, region);
    println!("  Replication group deleted");

    // [3/5] Create a standalone Redis cache cluster for delete test
    println!(
        "\n[3/5] Creating standalone Redis cache cluster: {}...",
        TEST_REDIS_CLUSTER_ID
    );
    let output = Command::new("aws")
        .args([
            "elasticache",
            "create-cache-cluster",
            "--cache-cluster-id",
            TEST_REDIS_CLUSTER_ID,
            "--cache-node-type",
            "cache.t3.micro",
            "--engine",
            "redis",
            "--num-cache-nodes",
            "1",
            "--region",
            region,
            "--output",
            "json",
        ])
        .output()
        .expect("aws cli must be installed");
    assert!(
        output.status.success()
            || String::from_utf8_lossy(&output.stderr).contains("CacheClusterAlreadyExists"),
        "Failed to create Redis cache cluster"
    );
    aws_wait_cache_available(TEST_REDIS_CLUSTER_ID, region);
    println!("  Cache cluster is available");

    // [4/5] DeleteCacheCluster — delete via library
    println!("\n[4/5] DeleteCacheCluster...");
    let del_resp = client
        .elasticache()
        .delete_cache_cluster(&DeleteCacheClusterRequest {
            cache_cluster_id: TEST_REDIS_CLUSTER_ID.into(),
            ..Default::default()
        })
        .await?;
    let cluster = del_resp
        .cache_cluster
        .as_ref()
        .expect("should return cache cluster");
    assert_eq!(
        cluster.cache_cluster_id.as_deref(),
        Some(TEST_REDIS_CLUSTER_ID)
    );
    assert!(
        cluster.cache_cluster_status.as_deref() == Some("deleting"),
        "cluster status should be deleting, got: {:?}",
        cluster.cache_cluster_status,
    );
    println!(
        "  Deleted cluster: id={:?} status={:?} engine={:?}",
        cluster.cache_cluster_id, cluster.cache_cluster_status, cluster.engine,
    );

    // [5/5] Clean up snapshot
    println!("\n[5/5] Cleaning up snapshot...");
    aws_delete_snapshot(TEST_SNAPSHOT_NAME, region);
    println!("  Snapshot deletion initiated");

    Ok(())
}

async fn run_elasticache_read_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== ElastiCache Read Operations Test ===");
    println!("Region: {region}");

    // [0/4] Create test cache cluster
    println!(
        "\n[0/4] Creating test cache cluster: {}...",
        TEST_CLUSTER_ID
    );
    let created = aws_create_cache_cluster(TEST_CLUSTER_ID, region);
    assert!(created, "Failed to create test cache cluster");
    println!("  Cache cluster creation initiated");

    aws_wait_cache_available(TEST_CLUSTER_ID, region);
    println!("  Cache cluster is available");

    // [1/4] DescribeCacheClusters — specific cluster
    println!("\n[1/4] DescribeCacheClusters (specific)...");
    let response = client
        .elasticache()
        .describe_cache_clusters(&DescribeCacheClustersRequest {
            cache_cluster_id: Some(TEST_CLUSTER_ID.into()),
            show_cache_node_info: Some(true),
            ..Default::default()
        })
        .await?;
    assert_eq!(
        response.cache_clusters.len(),
        1,
        "should find exactly one cluster"
    );
    let cluster = &response.cache_clusters[0];
    assert_eq!(cluster.cache_cluster_id.as_deref(), Some(TEST_CLUSTER_ID));
    assert_eq!(cluster.engine.as_deref(), Some("memcached"));
    assert_eq!(cluster.cache_node_type.as_deref(), Some("cache.t3.micro"));
    assert_eq!(cluster.cache_cluster_status.as_deref(), Some("available"));
    assert_eq!(cluster.num_cache_nodes, Some(1));
    assert!(cluster.arn.is_some(), "should have ARN");
    println!(
        "  Cluster: id={:?} status={:?} engine={:?} type={:?} nodes={:?}",
        cluster.cache_cluster_id,
        cluster.cache_cluster_status,
        cluster.engine,
        cluster.cache_node_type,
        cluster.num_cache_nodes,
    );
    // Verify cache node info (we requested ShowCacheNodeInfo=true)
    if !cluster.cache_nodes.is_empty() {
        let node = &cluster.cache_nodes[0];
        println!(
            "  CacheNode: id={:?} status={:?}",
            node.cache_node_id, node.cache_node_status
        );
        assert!(node.cache_node_id.is_some(), "cache node should have an ID");
    }

    // [2/4] DescribeCacheClusters — list all
    println!("\n[2/4] DescribeCacheClusters (list all)...");
    let response = client
        .elasticache()
        .describe_cache_clusters(&DescribeCacheClustersRequest::default())
        .await?;
    assert!(
        !response.cache_clusters.is_empty(),
        "should find at least the test cluster"
    );
    println!("  Total cache clusters: {}", response.cache_clusters.len());
    let found = response
        .cache_clusters
        .iter()
        .any(|c| c.cache_cluster_id.as_deref() == Some(TEST_CLUSTER_ID));
    assert!(found, "test cluster should appear in list-all response");

    // [3/4] DescribeReplicationGroups — list all (may be empty)
    println!("\n[3/4] DescribeReplicationGroups (list all)...");
    let response = client
        .elasticache()
        .describe_replication_groups(&DescribeReplicationGroupsRequest::default())
        .await?;
    println!(
        "  Total replication groups: {}",
        response.replication_groups.len()
    );
    // We created a Memcached cluster, not Redis, so there may be no replication groups.
    // Just verify the API call succeeded and returned a valid response.
    for rg in &response.replication_groups {
        println!(
            "  ReplicationGroup: id={:?} status={:?} engine={:?} encrypt_transit={:?} encrypt_rest={:?}",
            rg.replication_group_id,
            rg.status,
            rg.engine,
            rg.transit_encryption_enabled,
            rg.at_rest_encryption_enabled,
        );
    }

    // [4/4] DescribeCacheClusters error — non-existent cluster
    println!("\n[4/4] DescribeCacheClusters (non-existent, expect error)...");
    let result = client
        .elasticache()
        .describe_cache_clusters(&DescribeCacheClustersRequest {
            cache_cluster_id: Some("cloud-lite-test-nonexistent-12345".into()),
            ..Default::default()
        })
        .await;
    assert!(result.is_err(), "non-existent cluster should fail");
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("not found") || err.contains("CacheClusterNotFound"),
        "expected not-found error, got: {err}"
    );
    println!("  Got expected error: {err}");

    Ok(())
}
