//! Integration tests for Amazon EMR API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration emr -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::emr::*;
use std::env;

const TEST_REGION: &str = "eu-central-1";

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_emr_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    println!("\n=== Amazon EMR Operations Test ===");
    println!("Region: {region}");

    run_emr_tests(&client)
        .await
        .expect("EMR operations tests failed");
    println!("\nAll EMR operations tests passed!");
}

async fn run_emr_tests(client: &AwsHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    // [1/3] ListClusters — verify API is accessible and response structure is correct
    println!("\n[1/3] ListClusters (all clusters)...");
    let list_resp = client
        .emr()
        .list_clusters(&ListClustersInput::default())
        .await?;

    println!("  Found {} cluster(s)", list_resp.clusters.len());
    for cluster in &list_resp.clusters {
        assert!(!cluster.id.is_empty(), "Expected non-empty cluster Id");
        assert!(!cluster.name.is_empty(), "Expected non-empty cluster Name");
        println!(
            "  Cluster: id={} name={} arn={:?}",
            cluster.id, cluster.name, cluster.cluster_arn,
        );
    }

    // [2/3] DescribeCluster — test error path with non-existent cluster ID
    // EMR clusters are expensive and slow to provision (~15+ minutes).
    // We verify the API is reachable by testing the error response for an invalid cluster ID.
    println!("\n[2/3] DescribeCluster (error case — non-existent cluster ID)...");
    let describe_err = client
        .emr()
        .describe_cluster(&DescribeClusterInput {
            cluster_id: "j-NONEXISTENTCLUSTER".to_string(),
        })
        .await;

    assert!(
        describe_err.is_err(),
        "Expected error when describing non-existent cluster"
    );
    let err_msg = format!("{:?}", describe_err.unwrap_err());
    // EMR returns InvalidRequestException for non-existent cluster IDs
    assert!(
        err_msg.contains("InvalidRequestException")
            || err_msg.contains("ResourceNotFoundException")
            || err_msg.contains("does not exist")
            || err_msg.contains("Cluster id")
            || err_msg.contains("j-NONEXISTENTCLUSTER"),
        "Expected not-found error for non-existent cluster, got: {err_msg}"
    );
    println!(
        "  Got expected error for non-existent cluster: {}",
        &err_msg[..err_msg.len().min(120)]
    );

    // [3/3] TerminateJobFlows — test error path with non-existent cluster ID
    // TerminateJobFlows is idempotent and typically returns 200 even for non-existent clusters,
    // so we send a request with a clearly invalid ID and verify the API endpoint is reachable.
    println!(
        "\n[3/3] TerminateJobFlows (non-existent cluster — expect success or descriptive error)..."
    );
    let terminate_result = client
        .emr()
        .terminate_job_flows(&TerminateJobFlowsInput {
            job_flow_ids: vec!["j-NONEXISTENTCLUSTER".to_string()],
        })
        .await;

    // TerminateJobFlows is fire-and-forget — AWS accepts it even for non-existent clusters
    // (returns 200 with no error). If it errors, verify it's a structured AWS error.
    match &terminate_result {
        Ok(_) => println!("  TerminateJobFlows accepted (fire-and-forget semantics)"),
        Err(e) => {
            let err_msg = format!("{e:?}");
            assert!(
                err_msg.contains("InvalidRequestException")
                    || err_msg.contains("AccessDeniedException")
                    || err_msg.contains("cluster"),
                "Expected structured AWS error, got: {err_msg}"
            );
            println!(
                "  Got expected error: {}",
                &err_msg[..err_msg.len().min(120)]
            );
        }
    }

    Ok(())
}
