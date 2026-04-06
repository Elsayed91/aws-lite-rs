//! Integration tests for Elastic Load Balancing v2 (ALB/NLB) API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration elbv2 -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::elbv2::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_LB_NAME: &str = "cloud-lite-test-ralph-lb";
const TEST_TG_NAME: &str = "cloud-lite-test-ralph-tg";

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "eu-central-1".into())
}

// --- CLI helpers ---

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

fn get_default_vpc(region: &str) -> Option<String> {
    let val = aws(&[
        "ec2",
        "describe-vpcs",
        "--filters",
        "Name=isDefault,Values=true",
        "--region",
        region,
        "--output",
        "json",
    ])?;
    val["Vpcs"][0]["VpcId"].as_str().map(String::from)
}

fn get_subnets_for_vpc(vpc_id: &str, region: &str) -> Vec<String> {
    let val = aws(&[
        "ec2",
        "describe-subnets",
        "--filters",
        &format!("Name=vpc-id,Values={vpc_id}"),
        "--region",
        region,
        "--output",
        "json",
    ]);
    match val {
        Some(v) => v["Subnets"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|s| s["SubnetId"].as_str().map(String::from))
            .take(2) // ALB needs at least 2 subnets in different AZs
            .collect(),
        None => vec![],
    }
}

fn aws_create_lb(name: &str, subnets: &[String], region: &str) -> Option<String> {
    let subnet_args: Vec<&str> = subnets.iter().map(|s| s.as_str()).collect();
    let mut args = vec![
        "elbv2",
        "create-load-balancer",
        "--name",
        name,
        "--type",
        "application",
        "--scheme",
        "internal",
        "--subnets",
    ];
    args.extend_from_slice(&subnet_args);
    args.extend_from_slice(&["--region", region, "--output", "json"]);
    let val = aws(&args)?;
    val["LoadBalancers"][0]["LoadBalancerArn"]
        .as_str()
        .map(String::from)
}

fn aws_create_tg(name: &str, vpc_id: &str, region: &str) -> Option<String> {
    let val = aws(&[
        "elbv2",
        "create-target-group",
        "--name",
        name,
        "--protocol",
        "HTTP",
        "--port",
        "80",
        "--target-type",
        "ip",
        "--vpc-id",
        vpc_id,
        "--region",
        region,
        "--output",
        "json",
    ])?;
    val["TargetGroups"][0]["TargetGroupArn"]
        .as_str()
        .map(String::from)
}

fn aws_create_listener(lb_arn: &str, tg_arn: &str, region: &str) -> Option<String> {
    let default_actions = format!(r#"[{{"Type":"forward","TargetGroupArn":"{tg_arn}"}}]"#);
    let val = aws(&[
        "elbv2",
        "create-listener",
        "--load-balancer-arn",
        lb_arn,
        "--protocol",
        "HTTP",
        "--port",
        "80",
        "--default-actions",
        &default_actions,
        "--region",
        region,
        "--output",
        "json",
    ])?;
    val["Listeners"][0]["ListenerArn"]
        .as_str()
        .map(String::from)
}

fn aws_delete_listener(arn: &str, region: &str) {
    aws_cleanup(&[
        "elbv2",
        "delete-listener",
        "--listener-arn",
        arn,
        "--region",
        region,
    ]);
}

fn aws_delete_lb(arn: &str, region: &str) {
    aws_cleanup(&[
        "elbv2",
        "delete-load-balancer",
        "--load-balancer-arn",
        arn,
        "--region",
        region,
    ]);
}

fn aws_delete_tg(arn: &str, region: &str) {
    aws_cleanup(&[
        "elbv2",
        "delete-target-group",
        "--target-group-arn",
        arn,
        "--region",
        region,
    ]);
}

fn aws_wait_lb_available(arn: &str, region: &str) {
    println!("  Waiting for load balancer to become active...");
    let _ = Command::new("aws")
        .args([
            "elbv2",
            "wait",
            "load-balancer-available",
            "--load-balancer-arns",
            arn,
            "--region",
            region,
        ])
        .output();
}

fn aws_wait_lb_deleted(arn: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "elbv2",
            "wait",
            "load-balancers-deleted",
            "--load-balancer-arns",
            arn,
            "--region",
            region,
        ])
        .output();
}

/// Find and delete any existing test resources by name
fn cleanup_by_name(region: &str) {
    // Find and delete LB by name
    if let Some(val) = aws(&[
        "elbv2",
        "describe-load-balancers",
        "--names",
        TEST_LB_NAME,
        "--region",
        region,
        "--output",
        "json",
    ]) {
        if let Some(arn) = val["LoadBalancers"][0]["LoadBalancerArn"].as_str() {
            // Delete listeners first
            if let Some(listeners) = aws(&[
                "elbv2",
                "describe-listeners",
                "--load-balancer-arn",
                arn,
                "--region",
                region,
                "--output",
                "json",
            ]) {
                if let Some(arr) = listeners["Listeners"].as_array() {
                    for l in arr {
                        if let Some(listener_arn) = l["ListenerArn"].as_str() {
                            aws_delete_listener(listener_arn, region);
                        }
                    }
                }
            }
            aws_delete_lb(arn, region);
            aws_wait_lb_deleted(arn, region);
        }
    }

    // Find and delete TG by name
    if let Some(val) = aws(&[
        "elbv2",
        "describe-target-groups",
        "--names",
        TEST_TG_NAME,
        "--region",
        region,
        "--output",
        "json",
    ]) {
        if let Some(arn) = val["TargetGroups"][0]["TargetGroupArn"].as_str() {
            aws_delete_tg(arn, region);
        }
    }
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials — creates real ELBv2 resources"]
async fn test_elbv2_read_operations() {
    let region = region();

    // Pre-cleanup
    cleanup_by_name(&region);
    tokio::time::sleep(Duration::from_secs(2)).await;

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    // Track ARNs for cleanup
    let mut lb_arn: Option<String> = None;
    let mut tg_arn: Option<String> = None;
    let mut listener_arn: Option<String> = None;

    let result = run_elbv2_read_tests(
        &client,
        &region,
        &mut lb_arn,
        &mut tg_arn,
        &mut listener_arn,
    )
    .await;

    // Always cleanup
    if let Some(ref arn) = listener_arn {
        aws_delete_listener(arn, &region);
    }
    if let Some(ref arn) = lb_arn {
        aws_delete_lb(arn, &region);
        aws_wait_lb_deleted(arn, &region);
    }
    if let Some(ref arn) = tg_arn {
        aws_delete_tg(arn, &region);
    }

    result.expect("ELBv2 read tests failed");
    println!("\nAll ELBv2 read tests passed!");
}

async fn run_elbv2_read_tests(
    client: &AwsHttpClient,
    region: &str,
    lb_arn_out: &mut Option<String>,
    tg_arn_out: &mut Option<String>,
    listener_arn_out: &mut Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== ELBv2 Read Operations Test ===");
    println!("Region: {region}");

    // [0/7] Create fixtures: VPC, subnets, ALB, target group, listener
    println!("\n[0/7] Creating test fixtures...");
    let vpc_id = get_default_vpc(region).expect("default VPC required for ELBv2 tests");
    println!("  VPC: {vpc_id}");
    let subnets = get_subnets_for_vpc(&vpc_id, region);
    assert!(
        subnets.len() >= 2,
        "need at least 2 subnets in default VPC, found {}",
        subnets.len()
    );
    println!("  Subnets: {:?}", subnets);

    let lb_arn =
        aws_create_lb(TEST_LB_NAME, &subnets, region).expect("failed to create test load balancer");
    *lb_arn_out = Some(lb_arn.clone());
    println!("  Load Balancer ARN: {lb_arn}");

    let tg_arn =
        aws_create_tg(TEST_TG_NAME, &vpc_id, region).expect("failed to create test target group");
    *tg_arn_out = Some(tg_arn.clone());
    println!("  Target Group ARN: {tg_arn}");

    aws_wait_lb_available(&lb_arn, region);
    println!("  Load Balancer is active");

    let l_arn =
        aws_create_listener(&lb_arn, &tg_arn, region).expect("failed to create test listener");
    *listener_arn_out = Some(l_arn.clone());
    println!("  Listener ARN: {l_arn}");

    // [1/7] DescribeLoadBalancers — specific LB by name
    println!("\n[1/7] DescribeLoadBalancers (by name)...");
    let response = client
        .elbv2()
        .describe_load_balancers(&DescribeLoadBalancersRequest {
            names: vec![TEST_LB_NAME.into()],
            ..Default::default()
        })
        .await?;
    assert_eq!(
        response.load_balancers.len(),
        1,
        "should find exactly one load balancer"
    );
    let lb = &response.load_balancers[0];
    assert_eq!(lb.load_balancer_name.as_deref(), Some(TEST_LB_NAME));
    assert_eq!(lb.load_balancer_arn.as_deref(), Some(lb_arn.as_str()));
    assert_eq!(lb.r#type.as_deref(), Some("application"));
    assert_eq!(lb.scheme.as_deref(), Some("internal"));
    assert!(lb.dns_name.is_some(), "should have DNS name");
    assert!(lb.vpc_id.is_some(), "should have VPC ID");
    assert!(lb.state.is_some(), "should have state");
    let state = lb.state.as_ref().unwrap();
    assert_eq!(state.code.as_deref(), Some("active"));
    assert!(!lb.availability_zones.is_empty(), "should have AZs");
    println!(
        "  LB: name={:?} type={:?} scheme={:?} state={:?} dns={:?} azs={}",
        lb.load_balancer_name,
        lb.r#type,
        lb.scheme,
        state.code,
        lb.dns_name,
        lb.availability_zones.len(),
    );

    // [2/7] DescribeLoadBalancers — list all
    println!("\n[2/7] DescribeLoadBalancers (list all)...");
    let response = client
        .elbv2()
        .describe_load_balancers(&DescribeLoadBalancersRequest::default())
        .await?;
    assert!(
        !response.load_balancers.is_empty(),
        "should find at least one LB"
    );
    let found = response
        .load_balancers
        .iter()
        .any(|lb| lb.load_balancer_name.as_deref() == Some(TEST_LB_NAME));
    assert!(found, "test LB should appear in list-all response");
    println!("  Total load balancers: {}", response.load_balancers.len());

    // [3/7] DescribeTargetGroups — specific TG by ARN
    println!("\n[3/7] DescribeTargetGroups (by ARN)...");
    let response = client
        .elbv2()
        .describe_target_groups(&DescribeTargetGroupsRequest {
            target_group_arns: vec![tg_arn.clone()],
            ..Default::default()
        })
        .await?;
    assert_eq!(
        response.target_groups.len(),
        1,
        "should find exactly one target group"
    );
    let tg = &response.target_groups[0];
    assert_eq!(tg.target_group_name.as_deref(), Some(TEST_TG_NAME));
    assert_eq!(tg.target_group_arn.as_deref(), Some(tg_arn.as_str()));
    assert_eq!(tg.protocol.as_deref(), Some("HTTP"));
    assert_eq!(tg.port, Some(80));
    assert_eq!(tg.target_type.as_deref(), Some("ip"));
    assert!(
        tg.health_check_enabled.is_some(),
        "should have health check enabled"
    );
    assert!(
        !tg.load_balancer_arns.is_empty(),
        "should be associated with LB"
    );
    println!(
        "  TG: name={:?} protocol={:?} port={:?} type={:?} health={:?}",
        tg.target_group_name, tg.protocol, tg.port, tg.target_type, tg.health_check_enabled,
    );

    // [4/7] DescribeTargetHealth — no targets registered, should return empty
    println!("\n[4/7] DescribeTargetHealth (no targets registered)...");
    let response = client
        .elbv2()
        .describe_target_health(&DescribeTargetHealthRequest {
            target_group_arn: tg_arn.clone(),
        })
        .await?;
    // No targets registered, so empty is valid
    println!(
        "  Target health descriptions: {}",
        response.target_health_descriptions.len()
    );

    // [5/7] DescribeListeners — by LB ARN
    println!("\n[5/7] DescribeListeners (by LB ARN)...");
    let response = client
        .elbv2()
        .describe_listeners(&DescribeListenersRequest {
            load_balancer_arn: Some(lb_arn.clone()),
            ..Default::default()
        })
        .await?;
    assert_eq!(
        response.listeners.len(),
        1,
        "should find exactly one listener"
    );
    let listener = &response.listeners[0];
    assert_eq!(listener.listener_arn.as_deref(), Some(l_arn.as_str()));
    assert_eq!(listener.load_balancer_arn.as_deref(), Some(lb_arn.as_str()));
    assert_eq!(listener.protocol.as_deref(), Some("HTTP"));
    assert_eq!(listener.port, Some(80));
    println!(
        "  Listener: protocol={:?} port={:?} ssl_policy={:?}",
        listener.protocol, listener.port, listener.ssl_policy,
    );

    // [6/7] DescribeLoadBalancerAttributes — check attributes
    println!("\n[6/7] DescribeLoadBalancerAttributes...");
    let response = client
        .elbv2()
        .describe_load_balancer_attributes(&DescribeLoadBalancerAttributesRequest {
            load_balancer_arn: lb_arn.clone(),
        })
        .await?;
    assert!(
        !response.attributes.is_empty(),
        "should have load balancer attributes"
    );
    // Check for well-known attributes
    let has_deletion_protection = response
        .attributes
        .iter()
        .any(|a| a.key.as_deref() == Some("deletion_protection.enabled"));
    assert!(
        has_deletion_protection,
        "should have deletion_protection.enabled attribute"
    );
    let has_access_logs = response
        .attributes
        .iter()
        .any(|a| a.key.as_deref() == Some("access_logs.s3.enabled"));
    assert!(
        has_access_logs,
        "should have access_logs.s3.enabled attribute"
    );
    println!("  Attributes ({}):", response.attributes.len());
    for attr in &response.attributes {
        println!("    {:?} = {:?}", attr.key, attr.value);
    }

    // [7/7] DescribeLoadBalancers error — non-existent LB
    println!("\n[7/7] DescribeLoadBalancers (non-existent, expect error)...");
    let result = client
        .elbv2()
        .describe_load_balancers(&DescribeLoadBalancersRequest {
            names: vec!["cl-test-nonexistent-lb".into()],
            ..Default::default()
        })
        .await;
    assert!(result.is_err(), "non-existent LB should fail");
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("LoadBalancerNotFound") || err.contains("not found"),
        "expected LoadBalancerNotFound error, got: {err}"
    );
    println!("  Got expected error: {err}");

    Ok(())
}

// --- Write Operations Tests ---

const TEST_WRITE_LB_NAME: &str = "cl-test-ralph-lb-w";
const TEST_WRITE_TG_NAME: &str = "cl-test-ralph-tg-w";

/// Find and delete write test resources by name
fn cleanup_write_by_name(region: &str) {
    // Find and delete LB by name
    if let Some(val) = aws(&[
        "elbv2",
        "describe-load-balancers",
        "--names",
        TEST_WRITE_LB_NAME,
        "--region",
        region,
        "--output",
        "json",
    ]) {
        if let Some(arn) = val["LoadBalancers"][0]["LoadBalancerArn"].as_str() {
            // Delete listeners first
            if let Some(listeners) = aws(&[
                "elbv2",
                "describe-listeners",
                "--load-balancer-arn",
                arn,
                "--region",
                region,
                "--output",
                "json",
            ]) {
                if let Some(arr) = listeners["Listeners"].as_array() {
                    for l in arr {
                        if let Some(listener_arn) = l["ListenerArn"].as_str() {
                            aws_delete_listener(listener_arn, region);
                        }
                    }
                }
            }
            aws_delete_lb(arn, region);
            aws_wait_lb_deleted(arn, region);
        }
    }

    // Find and delete TG by name
    if let Some(val) = aws(&[
        "elbv2",
        "describe-target-groups",
        "--names",
        TEST_WRITE_TG_NAME,
        "--region",
        region,
        "--output",
        "json",
    ]) {
        if let Some(arn) = val["TargetGroups"][0]["TargetGroupArn"].as_str() {
            aws_delete_tg(arn, region);
        }
    }
}

#[tokio::test]
#[ignore = "requires AWS credentials — creates real ELBv2 resources"]
async fn test_elbv2_write_operations() {
    let region = region();

    // Pre-cleanup
    cleanup_write_by_name(&region);
    tokio::time::sleep(Duration::from_secs(2)).await;

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    // Track ARNs for cleanup
    let mut lb_arn: Option<String> = None;
    let mut tg_arn: Option<String> = None;

    let result = run_elbv2_write_tests(&client, &region, &mut lb_arn, &mut tg_arn).await;

    // Always cleanup (best-effort)
    if let Some(ref arn) = lb_arn {
        aws_delete_lb(arn, &region);
        aws_wait_lb_deleted(arn, &region);
    }
    if let Some(ref arn) = tg_arn {
        aws_delete_tg(arn, &region);
    }

    result.expect("ELBv2 write tests failed");
    println!("\nAll ELBv2 write tests passed!");
}

async fn run_elbv2_write_tests(
    client: &AwsHttpClient,
    region: &str,
    lb_arn_out: &mut Option<String>,
    tg_arn_out: &mut Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== ELBv2 Write Operations Test ===");
    println!("Region: {region}");

    // [0/5] Create fixtures: ALB + target group
    println!("\n[0/5] Creating test fixtures...");
    let vpc_id = get_default_vpc(region).expect("default VPC required for ELBv2 tests");
    println!("  VPC: {vpc_id}");
    let subnets = get_subnets_for_vpc(&vpc_id, region);
    assert!(
        subnets.len() >= 2,
        "need at least 2 subnets in default VPC, found {}",
        subnets.len()
    );

    let lb_arn = aws_create_lb(TEST_WRITE_LB_NAME, &subnets, region)
        .expect("failed to create test load balancer");
    *lb_arn_out = Some(lb_arn.clone());
    println!("  Load Balancer ARN: {lb_arn}");

    let tg_arn = aws_create_tg(TEST_WRITE_TG_NAME, &vpc_id, region)
        .expect("failed to create test target group");
    *tg_arn_out = Some(tg_arn.clone());
    println!("  Target Group ARN: {tg_arn}");

    aws_wait_lb_available(&lb_arn, region);
    println!("  Load Balancer is active");

    // [1/5] ModifyLoadBalancerAttributes — change idle timeout from 60 to 120
    println!("\n[1/5] ModifyLoadBalancerAttributes (change idle_timeout to 120)...");
    let response = client
        .elbv2()
        .modify_load_balancer_attributes(&ModifyLoadBalancerAttributesRequest {
            load_balancer_arn: lb_arn.clone(),
            attributes: vec![LoadBalancerAttribute {
                key: Some("idle_timeout.timeout_seconds".into()),
                value: Some("120".into()),
            }],
        })
        .await?;
    assert!(
        !response.attributes.is_empty(),
        "should return modified attributes"
    );
    let idle_timeout = response
        .attributes
        .iter()
        .find(|a| a.key.as_deref() == Some("idle_timeout.timeout_seconds"));
    assert!(
        idle_timeout.is_some(),
        "should return idle_timeout attribute"
    );
    assert_eq!(
        idle_timeout.unwrap().value.as_deref(),
        Some("120"),
        "idle_timeout should be updated to 120"
    );
    println!("  Modified attributes ({}):", response.attributes.len());
    for attr in &response.attributes {
        println!("    {:?} = {:?}", attr.key, attr.value);
    }

    // [2/5] Verify the modification with DescribeLoadBalancerAttributes
    println!("\n[2/5] Verifying modification with DescribeLoadBalancerAttributes...");
    let response = client
        .elbv2()
        .describe_load_balancer_attributes(&DescribeLoadBalancerAttributesRequest {
            load_balancer_arn: lb_arn.clone(),
        })
        .await?;
    let idle_timeout = response
        .attributes
        .iter()
        .find(|a| a.key.as_deref() == Some("idle_timeout.timeout_seconds"));
    assert_eq!(
        idle_timeout.unwrap().value.as_deref(),
        Some("120"),
        "idle_timeout should still be 120 after describe"
    );
    println!("  Verified: idle_timeout.timeout_seconds = 120");

    // [3/5] DeleteLoadBalancer — delete the LB via library
    println!("\n[3/5] DeleteLoadBalancer...");
    let _response = client
        .elbv2()
        .delete_load_balancer(&DeleteLoadBalancerRequest {
            load_balancer_arn: lb_arn.clone(),
        })
        .await?;
    println!("  Delete request succeeded");

    // Wait for LB to finish deleting
    println!("  Waiting for load balancer deletion...");
    aws_wait_lb_deleted(&lb_arn, region);
    println!("  Load balancer deleted");
    // Clear the ARN so cleanup doesn't try to delete again
    *lb_arn_out = None;

    // [4/5] Verify LB is deleted — describe should fail
    println!("\n[4/5] Verifying LB deletion...");
    let result = client
        .elbv2()
        .describe_load_balancers(&DescribeLoadBalancersRequest {
            load_balancer_arns: vec![lb_arn.clone()],
            ..Default::default()
        })
        .await;
    assert!(result.is_err(), "deleted LB should not be found");
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("LoadBalancerNotFound") || err.contains("not found"),
        "expected LoadBalancerNotFound, got: {err}"
    );
    println!("  Confirmed: LB not found after deletion");

    // [5/5] DeleteTargetGroup — delete the TG via library (no longer associated with LB)
    println!("\n[5/5] DeleteTargetGroup...");
    let _response = client
        .elbv2()
        .delete_target_group(&DeleteTargetGroupRequest {
            target_group_arn: tg_arn.clone(),
        })
        .await?;
    println!("  Delete request succeeded");
    // Clear the ARN so cleanup doesn't try again
    *tg_arn_out = None;

    // Verify TG is deleted — describe should fail
    tokio::time::sleep(Duration::from_secs(2)).await;
    let result = client
        .elbv2()
        .describe_target_groups(&DescribeTargetGroupsRequest {
            target_group_arns: vec![tg_arn.clone()],
            ..Default::default()
        })
        .await;
    assert!(result.is_err(), "deleted TG should not be found");
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("TargetGroupNotFound") || err.contains("not found"),
        "expected TargetGroupNotFound, got: {err}"
    );
    println!("  Confirmed: TG not found after deletion");

    Ok(())
}
