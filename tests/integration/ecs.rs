//! Integration tests for ECS (Elastic Container Service) API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration ecs -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::ecs::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_REGION: &str = "eu-central-1";
const TEST_CLUSTER_NAME: &str = "cloud-lite-test-ralph-ecs";
const TEST_TASK_FAMILY: &str = "cloud-lite-test-ralph-ecs-td";
const TEST_SERVICE_NAME: &str = "cl-test-ralph-ecs-svc";

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

fn aws_create_cluster(region: &str) -> Option<String> {
    let v = aws(&[
        "ecs",
        "create-cluster",
        "--cluster-name",
        TEST_CLUSTER_NAME,
        "--region",
        region,
        "--output",
        "json",
    ])?;
    v["cluster"]["clusterArn"].as_str().map(String::from)
}

fn aws_delete_cluster(region: &str) {
    aws_cleanup(&[
        "ecs",
        "delete-cluster",
        "--cluster",
        TEST_CLUSTER_NAME,
        "--region",
        region,
    ]);
}

fn aws_register_task_definition(region: &str) -> Option<String> {
    let container_defs = serde_json::json!([{
        "name": "test-container",
        "image": "nginx:alpine",
        "cpu": 256,
        "memory": 512,
        "essential": true,
        "portMappings": [{
            "containerPort": 80,
            "protocol": "tcp"
        }]
    }]);
    let container_defs_str = serde_json::to_string(&container_defs).ok()?;
    let v = aws(&[
        "ecs",
        "register-task-definition",
        "--family",
        TEST_TASK_FAMILY,
        "--network-mode",
        "awsvpc",
        "--requires-compatibilities",
        "FARGATE",
        "--cpu",
        "256",
        "--memory",
        "512",
        "--container-definitions",
        &container_defs_str,
        "--region",
        region,
        "--output",
        "json",
    ])?;
    v["taskDefinition"]["taskDefinitionArn"]
        .as_str()
        .map(String::from)
}

fn aws_deregister_task_definition(td_arn: &str, region: &str) {
    aws_cleanup(&[
        "ecs",
        "deregister-task-definition",
        "--task-definition",
        td_arn,
        "--region",
        region,
    ]);
}

fn aws_create_service(cluster: &str, td_arn: &str, region: &str) -> Option<String> {
    // Get default VPC subnets for awsvpc networking
    let subnets = aws_get_default_subnets(region)?;
    if subnets.is_empty() {
        eprintln!("  Warning: no default subnets found");
        return None;
    }
    let subnet = &subnets[0];

    let network_config = serde_json::json!({
        "awsvpcConfiguration": {
            "subnets": [subnet],
            "assignPublicIp": "ENABLED"
        }
    });
    let network_config_str = serde_json::to_string(&network_config).ok()?;

    let v = aws(&[
        "ecs",
        "create-service",
        "--cluster",
        cluster,
        "--service-name",
        TEST_SERVICE_NAME,
        "--task-definition",
        td_arn,
        "--desired-count",
        "0",
        "--launch-type",
        "FARGATE",
        "--network-configuration",
        &network_config_str,
        "--region",
        region,
        "--output",
        "json",
    ])?;
    v["service"]["serviceArn"].as_str().map(String::from)
}

fn aws_delete_service(cluster: &str, region: &str) {
    // Force delete (sets desired count to 0 + deletes)
    aws_cleanup(&[
        "ecs",
        "delete-service",
        "--cluster",
        cluster,
        "--service",
        TEST_SERVICE_NAME,
        "--force",
        "--region",
        region,
    ]);
}

fn aws_get_default_subnets(region: &str) -> Option<Vec<String>> {
    let v = aws(&[
        "ec2",
        "describe-subnets",
        "--filters",
        "Name=default-for-az,Values=true",
        "--region",
        region,
        "--output",
        "json",
    ])?;
    let subnets = v["Subnets"].as_array()?;
    Some(
        subnets
            .iter()
            .filter_map(|s| s["SubnetId"].as_str().map(String::from))
            .collect(),
    )
}

fn cleanup_all(td_arns: &[String], region: &str) {
    aws_delete_service(TEST_CLUSTER_NAME, region);
    // Wait briefly for service deletion
    std::thread::sleep(Duration::from_secs(2));
    for arn in td_arns {
        aws_deregister_task_definition(arn, region);
    }
    // Also try to deregister by family name (catches revisions we might have missed)
    for rev in 1..=3 {
        aws_cleanup(&[
            "ecs",
            "deregister-task-definition",
            "--task-definition",
            &format!("{TEST_TASK_FAMILY}:{rev}"),
            "--region",
            region,
        ]);
    }
    aws_delete_cluster(region);
}

// --- Test ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_ecs_read_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    // Pre-cleanup
    cleanup_all(&[], &region);
    tokio::time::sleep(Duration::from_secs(3)).await;

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    // Track task definition ARNs for cleanup
    let mut td_arns: Vec<String> = Vec::new();

    let result = run_ecs_read_tests(&client, &region, &mut td_arns).await;

    // Always cleanup
    cleanup_all(&td_arns, &region);

    result.expect("ECS read operations tests failed");
    println!("\nAll ECS read operations tests passed!");
}

async fn run_ecs_read_tests(
    client: &AwsHttpClient,
    region: &str,
    td_arns_out: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== ECS Read Operations Test ===");
    println!("Region: {region}");

    // [0/7] Create fixtures
    println!("\n[0/7] Creating test fixtures...");

    let cluster_arn = aws_create_cluster(region).expect("Failed to create ECS cluster");
    println!("  Created cluster: {cluster_arn}");

    let td_arn = aws_register_task_definition(region).expect("Failed to register task definition");
    println!("  Registered task def: {td_arn}");
    td_arns_out.push(td_arn.clone());

    let svc_arn = aws_create_service(TEST_CLUSTER_NAME, &td_arn, region)
        .expect("Failed to create ECS service");
    println!("  Created service: {svc_arn}");
    tokio::time::sleep(Duration::from_secs(3)).await;

    // [1/7] ListClusters
    println!("\n[1/7] ListClusters...");
    let list_resp = client
        .ecs()
        .list_clusters(&ListClustersRequest::default())
        .await?;
    println!("  Found {} cluster(s)", list_resp.cluster_arns.len());
    assert!(
        list_resp
            .cluster_arns
            .iter()
            .any(|a| a.contains(TEST_CLUSTER_NAME)),
        "Expected to find test cluster in list, got: {:?}",
        list_resp.cluster_arns,
    );

    // [2/7] DescribeClusters
    println!("\n[2/7] DescribeClusters...");
    let desc_resp = client
        .ecs()
        .describe_clusters(&DescribeClustersRequest {
            clusters: vec![TEST_CLUSTER_NAME.to_string()],
            ..Default::default()
        })
        .await?;
    assert_eq!(desc_resp.clusters.len(), 1, "Expected 1 cluster");
    let cluster = &desc_resp.clusters[0];
    assert_eq!(cluster.cluster_name.as_deref(), Some(TEST_CLUSTER_NAME));
    assert_eq!(cluster.status.as_deref(), Some("ACTIVE"));
    println!(
        "  Cluster: {} (status={}, services={}, tasks={})",
        cluster.cluster_name.as_deref().unwrap_or("?"),
        cluster.status.as_deref().unwrap_or("?"),
        cluster.active_services_count.unwrap_or(0),
        cluster.running_tasks_count.unwrap_or(0),
    );

    // [3/7] ListServices
    println!("\n[3/7] ListServices...");
    let list_svc_resp = client
        .ecs()
        .list_services(&ListServicesRequest {
            cluster: Some(TEST_CLUSTER_NAME.to_string()),
            ..Default::default()
        })
        .await?;
    println!("  Found {} service(s)", list_svc_resp.service_arns.len());
    assert!(
        list_svc_resp
            .service_arns
            .iter()
            .any(|a| a.contains(TEST_SERVICE_NAME)),
        "Expected to find test service in list, got: {:?}",
        list_svc_resp.service_arns,
    );

    // [4/7] DescribeServices
    println!("\n[4/7] DescribeServices...");
    let desc_svc_resp = client
        .ecs()
        .describe_services(&DescribeServicesRequest {
            cluster: Some(TEST_CLUSTER_NAME.to_string()),
            services: vec![TEST_SERVICE_NAME.to_string()],
            ..Default::default()
        })
        .await?;
    assert_eq!(desc_svc_resp.services.len(), 1, "Expected 1 service");
    let svc = &desc_svc_resp.services[0];
    assert_eq!(svc.service_name.as_deref(), Some(TEST_SERVICE_NAME));
    assert_eq!(svc.desired_count, Some(0));
    assert_eq!(svc.launch_type.as_deref(), Some("FARGATE"));
    println!(
        "  Service: {} (status={}, desired={}, running={}, launch={})",
        svc.service_name.as_deref().unwrap_or("?"),
        svc.status.as_deref().unwrap_or("?"),
        svc.desired_count.unwrap_or(0),
        svc.running_count.unwrap_or(0),
        svc.launch_type.as_deref().unwrap_or("?"),
    );

    // [5/7] DescribeTaskDefinition
    println!("\n[5/7] DescribeTaskDefinition...");
    let desc_td_resp = client
        .ecs()
        .describe_task_definition(&DescribeTaskDefinitionRequest {
            task_definition: format!("{TEST_TASK_FAMILY}:1"),
            ..Default::default()
        })
        .await?;
    let td = desc_td_resp
        .task_definition
        .as_ref()
        .expect("Expected task definition");
    assert_eq!(td.family.as_deref(), Some(TEST_TASK_FAMILY));
    assert_eq!(td.cpu.as_deref(), Some("256"));
    assert_eq!(td.memory.as_deref(), Some("512"));
    assert_eq!(td.revision, Some(1));
    assert!(
        !td.container_definitions.is_empty(),
        "Expected at least 1 container def"
    );
    let container = &td.container_definitions[0];
    assert_eq!(container.name.as_deref(), Some("test-container"));
    assert_eq!(container.image.as_deref(), Some("nginx:alpine"));
    println!(
        "  Task def: {} rev={} cpu={} memory={} containers={}",
        td.family.as_deref().unwrap_or("?"),
        td.revision.unwrap_or(0),
        td.cpu.as_deref().unwrap_or("?"),
        td.memory.as_deref().unwrap_or("?"),
        td.container_definitions.len(),
    );

    // [6/7] DescribeClusters with non-existent cluster
    println!("\n[6/7] DescribeClusters with non-existent cluster...");
    let desc_missing = client
        .ecs()
        .describe_clusters(&DescribeClustersRequest {
            clusters: vec!["cloud-lite-test-nonexistent-cluster".to_string()],
            ..Default::default()
        })
        .await?;
    // ECS returns failures for non-existent clusters, not an error
    assert!(
        !desc_missing.failures.is_empty(),
        "Expected failures for non-existent cluster"
    );
    let failure = &desc_missing.failures[0];
    assert!(
        failure.reason.as_deref().unwrap_or("").contains("MISSING"),
        "Expected MISSING reason, got: {:?}",
        failure.reason,
    );
    println!(
        "  Got expected failure: reason={}, arn={}",
        failure.reason.as_deref().unwrap_or("?"),
        failure.arn.as_deref().unwrap_or("?"),
    );

    // [7/7] DescribeTaskDefinition with non-existent family
    println!("\n[7/7] DescribeTaskDefinition error case...");
    let err = client
        .ecs()
        .describe_task_definition(&DescribeTaskDefinitionRequest {
            task_definition: "cloud-lite-test-nonexistent-td:1".to_string(),
            ..Default::default()
        })
        .await;
    assert!(
        err.is_err(),
        "Expected error for non-existent task definition"
    );
    let err_msg = format!("{}", err.unwrap_err());
    assert!(
        err_msg.contains("ClientException")
            || err_msg.contains("Unable to describe task definition"),
        "Expected ClientException, got: {err_msg}",
    );
    println!("  Got expected error: {err_msg}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_ecs_write_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    // Pre-cleanup
    cleanup_all(&[], &region);
    tokio::time::sleep(Duration::from_secs(3)).await;

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    // Track task definition ARNs for cleanup
    let mut td_arns: Vec<String> = Vec::new();

    let result = run_ecs_write_tests(&client, &region, &mut td_arns).await;

    // Always cleanup
    cleanup_all(&td_arns, &region);

    result.expect("ECS write operations tests failed");
    println!("\nAll ECS write operations tests passed!");
}

async fn run_ecs_write_tests(
    client: &AwsHttpClient,
    region: &str,
    td_arns_out: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== ECS Write Operations Test ===");
    println!("Region: {region}");

    // [0/4] Create fixtures
    println!("\n[0/4] Creating test fixtures...");

    let cluster_arn = aws_create_cluster(region).expect("Failed to create ECS cluster");
    println!("  Created cluster: {cluster_arn}");

    let td_arn = aws_register_task_definition(region).expect("Failed to register task definition");
    println!("  Registered task def: {td_arn}");
    td_arns_out.push(td_arn.clone());

    let svc_arn = aws_create_service(TEST_CLUSTER_NAME, &td_arn, region)
        .expect("Failed to create ECS service");
    println!("  Created service: {svc_arn}");
    tokio::time::sleep(Duration::from_secs(3)).await;

    // [1/4] UpdateService — force new deployment
    println!("\n[1/4] UpdateService (force new deployment)...");
    let update_resp = client
        .ecs()
        .update_service(&UpdateServiceRequest {
            cluster: Some(TEST_CLUSTER_NAME.to_string()),
            service: TEST_SERVICE_NAME.to_string(),
            force_new_deployment: Some(true),
            ..Default::default()
        })
        .await?;
    let updated_svc = update_resp
        .service
        .as_ref()
        .expect("Expected service in response");
    assert_eq!(updated_svc.service_name.as_deref(), Some(TEST_SERVICE_NAME));
    // After force new deployment, there should be at least one deployment
    assert!(
        !updated_svc.deployments.is_empty(),
        "Expected at least one deployment after force new deployment"
    );
    println!(
        "  Updated service: {} (deployments={})",
        updated_svc.service_name.as_deref().unwrap_or("?"),
        updated_svc.deployments.len(),
    );

    // [2/4] Verify UpdateService via DescribeServices
    println!("\n[2/4] Verifying UpdateService via DescribeServices...");
    let desc_resp = client
        .ecs()
        .describe_services(&DescribeServicesRequest {
            cluster: Some(TEST_CLUSTER_NAME.to_string()),
            services: vec![TEST_SERVICE_NAME.to_string()],
            ..Default::default()
        })
        .await?;
    assert_eq!(desc_resp.services.len(), 1);
    let svc = &desc_resp.services[0];
    assert_eq!(svc.service_name.as_deref(), Some(TEST_SERVICE_NAME));
    // Verify task definition is still set
    assert!(
        svc.task_definition
            .as_deref()
            .unwrap_or("")
            .contains(TEST_TASK_FAMILY),
        "Expected task definition to contain family name, got: {:?}",
        svc.task_definition,
    );
    println!(
        "  Verified: {} (task_def={}, deployments={})",
        svc.service_name.as_deref().unwrap_or("?"),
        svc.task_definition.as_deref().unwrap_or("?"),
        svc.deployments.len(),
    );

    // [3/4] DeregisterTaskDefinition via library
    // First register a second revision so we can deregister it without affecting the service
    println!("\n[3/4] DeregisterTaskDefinition...");
    let td_arn2 =
        aws_register_task_definition(region).expect("Failed to register second task definition");
    println!("  Registered second task def: {td_arn2}");
    td_arns_out.push(td_arn2.clone());

    let dereg_resp = client
        .ecs()
        .deregister_task_definition(&DeregisterTaskDefinitionRequest {
            task_definition: td_arn2.clone(),
        })
        .await?;
    let dereg_td = dereg_resp
        .task_definition
        .as_ref()
        .expect("Expected task definition in response");
    assert_eq!(dereg_td.status.as_deref(), Some("INACTIVE"));
    assert_eq!(dereg_td.family.as_deref(), Some(TEST_TASK_FAMILY));
    println!(
        "  Deregistered: {} rev={} (status={})",
        dereg_td.family.as_deref().unwrap_or("?"),
        dereg_td.revision.unwrap_or(0),
        dereg_td.status.as_deref().unwrap_or("?"),
    );

    // [4/4] Verify DeregisterTaskDefinition via DescribeTaskDefinition
    println!("\n[4/4] Verifying DeregisterTaskDefinition via DescribeTaskDefinition...");
    let verify_td = client
        .ecs()
        .describe_task_definition(&DescribeTaskDefinitionRequest {
            task_definition: td_arn2.clone(),
            ..Default::default()
        })
        .await?;
    let td = verify_td
        .task_definition
        .as_ref()
        .expect("Expected task definition");
    assert_eq!(td.status.as_deref(), Some("INACTIVE"));
    println!(
        "  Verified: {} rev={} status={}",
        td.family.as_deref().unwrap_or("?"),
        td.revision.unwrap_or(0),
        td.status.as_deref().unwrap_or("?"),
    );

    Ok(())
}
