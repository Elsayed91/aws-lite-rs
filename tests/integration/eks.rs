//! Integration tests for Amazon EKS API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration eks -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::eks::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_REGION: &str = "eu-central-1";
const TEST_CLUSTER_NAME: &str = "cloud-lite-test-eks-cluster";
const TEST_NODEGROUP_NAME: &str = "cloud-lite-test-eks-ng";
const EKS_ROLE_NAME: &str = "cloud-lite-test-eks-role";
const NODEGROUP_ROLE_NAME: &str = "cloud-lite-test-eks-ng-role";

// --- AWS CLI helpers ---

fn aws(region: &str, service: &str, args: &[&str]) -> Option<serde_json::Value> {
    let mut cmd_args: Vec<&str> = vec![service];
    cmd_args.extend_from_slice(args);
    cmd_args.extend_from_slice(&["--region", region, "--output", "json"]);

    let output = Command::new("aws")
        .args(&cmd_args)
        .output()
        .expect("aws cli must be installed");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(&stdout).ok()
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("  aws {} {:?} failed: {}", service, args, stderr.trim());
        None
    }
}

fn aws_cleanup(service: &str, args: &[&str]) {
    let _ = Command::new("aws")
        .args(std::iter::once(service).chain(args.iter().copied()))
        .output();
}

/// Ensure the EKS cluster IAM role exists, creating it if needed.
/// Returns the role ARN.
fn ensure_eks_role(region: &str) -> Option<String> {
    // Check if role already exists
    if let Some(val) = aws(region, "iam", &["get-role", "--role-name", EKS_ROLE_NAME]) {
        return val["Role"]["Arn"].as_str().map(String::from);
    }

    // Create trust policy document
    let trust_policy = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"Service":"eks.amazonaws.com"},"Action":"sts:AssumeRole"}]}"#;

    let result = aws(
        region,
        "iam",
        &[
            "create-role",
            "--role-name",
            EKS_ROLE_NAME,
            "--assume-role-policy-document",
            trust_policy,
        ],
    )?;

    let arn = result["Role"]["Arn"].as_str().map(String::from)?;

    // Attach required policies
    aws_cleanup(
        "iam",
        &[
            "attach-role-policy",
            "--role-name",
            EKS_ROLE_NAME,
            "--policy-arn",
            "arn:aws:iam::aws:policy/AmazonEKSClusterPolicy",
        ],
    );

    Some(arn)
}

/// Ensure the EKS nodegroup IAM role exists, creating it if needed.
/// Returns the role ARN.
fn ensure_nodegroup_role(region: &str) -> Option<String> {
    // Check if role already exists
    if let Some(val) = aws(
        region,
        "iam",
        &["get-role", "--role-name", NODEGROUP_ROLE_NAME],
    ) {
        return val["Role"]["Arn"].as_str().map(String::from);
    }

    let trust_policy = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"Service":"ec2.amazonaws.com"},"Action":"sts:AssumeRole"}]}"#;

    let result = aws(
        region,
        "iam",
        &[
            "create-role",
            "--role-name",
            NODEGROUP_ROLE_NAME,
            "--assume-role-policy-document",
            trust_policy,
        ],
    )?;

    let arn = result["Role"]["Arn"].as_str().map(String::from)?;

    // Attach required policies for node groups
    for policy in &[
        "arn:aws:iam::aws:policy/AmazonEKSWorkerNodePolicy",
        "arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy",
        "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly",
    ] {
        aws_cleanup(
            "iam",
            &[
                "attach-role-policy",
                "--role-name",
                NODEGROUP_ROLE_NAME,
                "--policy-arn",
                policy,
            ],
        );
    }

    Some(arn)
}

/// Get subnet IDs for the default VPC in the region.
fn get_default_vpc_subnets(region: &str) -> Option<Vec<String>> {
    let val = aws(
        region,
        "ec2",
        &[
            "describe-subnets",
            "--filters",
            "Name=defaultForAz,Values=true",
        ],
    )?;

    let subnets = val["Subnets"].as_array()?;
    let ids: Vec<String> = subnets
        .iter()
        .filter_map(|s| s["SubnetId"].as_str().map(String::from))
        .collect();

    if ids.is_empty() { None } else { Some(ids) }
}

/// Create the EKS cluster via AWS CLI. Returns the cluster name on success.
fn aws_create_cluster(region: &str, role_arn: &str, subnet_ids: &[String]) -> Option<String> {
    let resources_vpc_config = format!(
        "subnetIds={},endpointPublicAccess=true,endpointPrivateAccess=false",
        subnet_ids.join(",")
    );

    let result = aws(
        region,
        "eks",
        &[
            "create-cluster",
            "--name",
            TEST_CLUSTER_NAME,
            "--kubernetes-version",
            "1.31",
            "--role-arn",
            role_arn,
            "--resources-vpc-config",
            &resources_vpc_config,
        ],
    )?;

    result["cluster"]["name"].as_str().map(String::from)
}

/// Wait for the EKS cluster to become ACTIVE (polls up to 25 minutes).
async fn wait_for_cluster_active(region: &str) -> bool {
    for attempt in 0..30 {
        tokio::time::sleep(Duration::from_secs(50)).await;
        if let Some(val) = aws(
            region,
            "eks",
            &["describe-cluster", "--name", TEST_CLUSTER_NAME],
        ) {
            let status = val["cluster"]["status"].as_str().unwrap_or("");
            println!("  Cluster status (attempt {}/30): {}", attempt + 1, status);
            if status == "ACTIVE" {
                return true;
            }
            if status == "FAILED" {
                return false;
            }
        }
    }
    false
}

/// Create a nodegroup for the cluster. Returns nodegroup name on success.
fn aws_create_nodegroup(
    region: &str,
    node_role_arn: &str,
    subnet_ids: &[String],
) -> Option<String> {
    // Build args with each subnet as a separate argument (AWS CLI requires this)
    let mut cmd_args: Vec<String> = vec![
        "eks".to_string(),
        "create-nodegroup".to_string(),
        "--cluster-name".to_string(),
        TEST_CLUSTER_NAME.to_string(),
        "--nodegroup-name".to_string(),
        TEST_NODEGROUP_NAME.to_string(),
        "--node-role".to_string(),
        node_role_arn.to_string(),
        "--subnets".to_string(),
    ];
    for s in subnet_ids {
        cmd_args.push(s.clone());
    }
    cmd_args.extend_from_slice(&[
        "--instance-types".to_string(),
        "t3.medium".to_string(),
        "--scaling-config".to_string(),
        "minSize=1,maxSize=2,desiredSize=1".to_string(),
        "--ami-type".to_string(),
        "AL2_x86_64".to_string(),
        "--region".to_string(),
        region.to_string(),
        "--output".to_string(),
        "json".to_string(),
    ]);

    let output = Command::new("aws")
        .args(&cmd_args)
        .output()
        .expect("aws cli must be installed");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str::<serde_json::Value>(&stdout)
            .ok()
            .and_then(|v| v["nodegroup"]["nodegroupName"].as_str().map(String::from))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("  aws eks create-nodegroup failed: {}", stderr.trim());
        None
    }
}

/// Wait for nodegroup to reach a stable state (polls up to 30 minutes).
async fn wait_for_nodegroup_active(region: &str) -> bool {
    for attempt in 0..36 {
        tokio::time::sleep(Duration::from_secs(50)).await;
        if let Some(val) = aws(
            region,
            "eks",
            &[
                "describe-nodegroup",
                "--cluster-name",
                TEST_CLUSTER_NAME,
                "--nodegroup-name",
                TEST_NODEGROUP_NAME,
            ],
        ) {
            let status = val["nodegroup"]["status"].as_str().unwrap_or("");
            println!(
                "  Nodegroup status (attempt {}/24): {}",
                attempt + 1,
                status
            );
            if status == "ACTIVE" {
                return true;
            }
            if status == "CREATE_FAILED" || status == "DEGRADED" {
                return false;
            }
        }
    }
    false
}

/// Delete nodegroup and cluster, waiting for deletion to complete.
fn aws_cleanup_cluster(region: &str) {
    // Delete nodegroup first
    aws_cleanup(
        "eks",
        &[
            "delete-nodegroup",
            "--cluster-name",
            TEST_CLUSTER_NAME,
            "--nodegroup-name",
            TEST_NODEGROUP_NAME,
            "--region",
            region,
        ],
    );

    // Wait for nodegroup deletion (up to 25 minutes)
    for _ in 0..30 {
        std::thread::sleep(Duration::from_secs(50));
        let result = Command::new("aws")
            .args([
                "eks",
                "describe-nodegroup",
                "--cluster-name",
                TEST_CLUSTER_NAME,
                "--nodegroup-name",
                TEST_NODEGROUP_NAME,
                "--region",
                region,
                "--output",
                "json",
            ])
            .output();

        if let Ok(out) = result {
            if !out.status.success() {
                break; // Nodegroup deleted (describe returned error)
            }
            let stdout = String::from_utf8_lossy(&out.stdout);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&stdout) {
                let status = val["nodegroup"]["status"].as_str().unwrap_or("");
                println!("  Nodegroup delete status: {}", status);
                if status == "DELETED" {
                    break;
                }
            }
        } else {
            break;
        }
    }

    // Delete the cluster
    aws_cleanup(
        "eks",
        &[
            "delete-cluster",
            "--name",
            TEST_CLUSTER_NAME,
            "--region",
            region,
        ],
    );
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_eks_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    println!("\n=== AWS EKS Operations Test ===");
    println!("Region: {region}");

    // Pre-cleanup any leftover test resources
    aws_cleanup_cluster(&region);

    let result = run_eks_tests(&client, &region).await;

    // Always cleanup
    aws_cleanup_cluster(&region);

    result.expect("EKS operations tests failed");
    println!("\nAll EKS operations tests passed!");
}

async fn run_eks_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // [0/6] Create EKS cluster via CLI (library does not support CreateCluster)
    println!("\n[0/6] Setting up test infrastructure...");

    let eks_role_arn = ensure_eks_role(region).ok_or("Failed to create/find EKS IAM role")?;
    println!("  EKS role ARN: {eks_role_arn}");

    let ng_role_arn =
        ensure_nodegroup_role(region).ok_or("Failed to create/find nodegroup IAM role")?;
    println!("  Nodegroup role ARN: {ng_role_arn}");

    let subnet_ids = get_default_vpc_subnets(region).ok_or("Failed to find default VPC subnets")?;
    println!(
        "  Using {} subnet(s): {:?}",
        subnet_ids.len(),
        &subnet_ids[..subnet_ids.len().min(3)]
    );

    println!("  Creating EKS cluster '{TEST_CLUSTER_NAME}' (takes ~15 minutes)...");
    aws_create_cluster(region, &eks_role_arn, &subnet_ids).ok_or("Failed to create EKS cluster")?;

    println!("  Waiting for cluster to become ACTIVE...");
    if !wait_for_cluster_active(region).await {
        return Err("EKS cluster did not become ACTIVE within timeout".into());
    }
    println!("  Cluster is ACTIVE.");

    // [1/6] DescribeCluster — verify cluster details
    println!("\n[1/6] DescribeCluster (cluster={TEST_CLUSTER_NAME})...");
    let desc = client.eks().describe_cluster(TEST_CLUSTER_NAME).await?;

    let cluster = desc.cluster.expect("Expected cluster in response");
    assert_eq!(
        cluster.name.as_deref(),
        Some(TEST_CLUSTER_NAME),
        "Expected cluster name to match"
    );
    assert!(
        cluster.arn.as_deref().is_some_and(|s| !s.is_empty()),
        "Expected non-empty cluster ARN"
    );
    assert!(
        cluster.version.is_some(),
        "Expected cluster version to be set"
    );
    println!(
        "  Cluster: name={:?} arn={:?} version={:?} status={:?}",
        cluster.name, cluster.arn, cluster.version, cluster.status,
    );

    // [2/6] ListNodegroups — should be empty initially
    println!("\n[2/6] ListNodegroups (cluster={TEST_CLUSTER_NAME})...");
    let list_resp = client
        .eks()
        .list_nodegroups(TEST_CLUSTER_NAME, "", "")
        .await?;

    println!("  Found {} nodegroup(s)", list_resp.nodegroups.len());
    assert!(
        list_resp.nodegroups.is_empty(),
        "Expected empty nodegroup list before creation"
    );

    // [3/6] Create nodegroup, wait for ACTIVE
    println!("\n[3/6] Creating nodegroup '{TEST_NODEGROUP_NAME}'...");
    aws_create_nodegroup(region, &ng_role_arn, &subnet_ids).ok_or("Failed to create nodegroup")?;

    println!("  Waiting for nodegroup to become ACTIVE...");
    if !wait_for_nodegroup_active(region).await {
        return Err("EKS nodegroup did not become ACTIVE within timeout".into());
    }
    println!("  Nodegroup is ACTIVE.");

    // [4/6] DescribeNodegroup — verify nodegroup details
    println!(
        "\n[4/6] DescribeNodegroup (cluster={TEST_CLUSTER_NAME}, nodegroup={TEST_NODEGROUP_NAME})..."
    );
    let ng_desc = client
        .eks()
        .describe_nodegroup(TEST_CLUSTER_NAME, TEST_NODEGROUP_NAME)
        .await?;

    let nodegroup = ng_desc.nodegroup.expect("Expected nodegroup in response");
    assert_eq!(
        nodegroup.nodegroup_name.as_deref(),
        Some(TEST_NODEGROUP_NAME),
        "Expected nodegroup name to match"
    );
    assert_eq!(
        nodegroup.cluster_name.as_deref(),
        Some(TEST_CLUSTER_NAME),
        "Expected cluster name to match"
    );
    assert!(
        nodegroup
            .nodegroup_arn
            .as_deref()
            .is_some_and(|s| !s.is_empty()),
        "Expected non-empty nodegroup ARN"
    );
    println!(
        "  Nodegroup: name={:?} cluster={:?} arn={:?} status={:?} scaling={:?}",
        nodegroup.nodegroup_name,
        nodegroup.cluster_name,
        nodegroup.nodegroup_arn,
        nodegroup.status,
        nodegroup.scaling_config,
    );

    // [5/6] UpdateNodegroupConfig — update scaling config (keep same values to be non-destructive)
    let current_scaling = nodegroup.scaling_config.as_ref();
    let current_min = current_scaling.and_then(|s| s.min_size).unwrap_or(1);
    let current_max = current_scaling.and_then(|s| s.max_size).unwrap_or(2);
    let current_desired = current_scaling.and_then(|s| s.desired_size).unwrap_or(1);

    println!(
        "\n[5/6] UpdateNodegroupConfig (restoring same scaling: min={current_min} max={current_max} desired={current_desired})..."
    );
    let update_resp = client
        .eks()
        .update_nodegroup_config(
            TEST_CLUSTER_NAME,
            TEST_NODEGROUP_NAME,
            &UpdateNodegroupConfigRequest {
                cluster_name: TEST_CLUSTER_NAME.to_string(),
                nodegroup_name: TEST_NODEGROUP_NAME.to_string(),
                scaling_config: Some(NodegroupScalingConfig {
                    min_size: Some(current_min),
                    max_size: Some(current_max),
                    desired_size: Some(current_desired),
                }),
                ..Default::default()
            },
        )
        .await?;

    let update = update_resp.update.expect("Expected update in response");
    assert!(
        update.id.as_deref().is_some_and(|s| !s.is_empty()),
        "Expected non-empty update ID"
    );
    println!(
        "  Update: id={:?} status={:?} type={:?}",
        update.id, update.status, update.r#type,
    );

    // [6/6] DescribeCluster error case — non-existent cluster
    println!("\n[6/6] DescribeCluster error case (non-existent cluster)...");
    let err = client
        .eks()
        .describe_cluster("cloud-lite-test-eks-nonexistent-xyz")
        .await;

    assert!(err.is_err(), "Expected error for non-existent cluster");
    let err_msg = format!("{:?}", err.unwrap_err());
    assert!(
        err_msg.contains("ResourceNotFoundException")
            || err_msg.contains("404")
            || err_msg.contains("No cluster found"),
        "Expected ResourceNotFoundException, got: {err_msg}"
    );
    println!(
        "  Got expected error: {}",
        &err_msg[..err_msg.len().min(120)]
    );

    Ok(())
}
