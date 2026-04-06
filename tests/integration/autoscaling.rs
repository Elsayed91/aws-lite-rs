//! Integration tests for AWS Auto Scaling API
//!
//! Tests DescribeAutoScalingGroups, DescribeLaunchConfigurations,
//! UpdateAutoScalingGroup, and StartInstanceRefresh against the real AWS API.
//!
//! Run with:
//!   AWS_PROFILE=<profile> AWS_REGION=eu-central-1 \
//!     cargo test -p aws-lite --test integration autoscaling -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use std::env;

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "eu-central-1".into())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_describe_auto_scaling_groups() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let asg = client.autoscaling();

    println!("\n=== DescribeAutoScalingGroups ===");

    // 1. List all ASGs
    println!("\n[1/2] Listing Auto Scaling groups...");
    let response = asg.describe_auto_scaling_groups().await?;
    println!("  Found {} groups", response.auto_scaling_groups.len());
    for group in response.auto_scaling_groups.iter().take(5) {
        println!(
            "    {} (min={:?}, max={:?}, desired={:?})",
            group.auto_scaling_group_name, group.min_size, group.max_size, group.desired_capacity,
        );
    }

    // 2. Verify structure (even if empty)
    println!("\n[2/2] Verifying response structure...");
    if let Some(group) = response.auto_scaling_groups.first() {
        assert!(
            !group.auto_scaling_group_name.is_empty(),
            "ASG name should not be empty"
        );
        println!("  Group fields present and valid");
    } else {
        println!("  No ASGs found (account may have none)");
    }

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_describe_launch_configurations() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let asg = client.autoscaling();

    println!("\n=== DescribeLaunchConfigurations ===");

    // 1. List all launch configurations
    println!("\n[1/2] Listing launch configurations...");
    let response = asg.describe_launch_configurations().await?;
    println!(
        "  Found {} launch configurations",
        response.launch_configurations.len()
    );
    for lc in response.launch_configurations.iter().take(5) {
        println!(
            "    {} (type={}, image={})",
            lc.launch_configuration_name, lc.instance_type, lc.image_id,
        );
    }

    // 2. Verify structure
    println!("\n[2/2] Verifying response structure...");
    if let Some(lc) = response.launch_configurations.first() {
        assert!(
            !lc.launch_configuration_name.is_empty(),
            "LC name should not be empty"
        );
        println!("  Launch configuration fields present and valid");
    } else {
        println!(
            "  No launch configurations found (expected for modern setups using launch templates)"
        );
    }

    Ok(())
}

// -- Write Operations (error cases) ------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_update_auto_scaling_group_error_case() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let asg = client.autoscaling();

    println!("\n=== UpdateAutoScalingGroup Error Cases ===");

    // 1. Non-existent ASG
    println!("\n[1/1] Updating non-existent ASG...");
    let body = aws_lite::types::autoscaling::UpdateAutoScalingGroupRequest {
        auto_scaling_group_name: "cloud-lite-test-ralph-nonexistent-asg".to_string(),
        desired_capacity: Some(1),
        ..Default::default()
    };
    let result = asg.update_auto_scaling_group(&body).await;
    assert!(result.is_err(), "non-existent ASG should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_start_instance_refresh_error_case() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let asg = client.autoscaling();

    println!("\n=== StartInstanceRefresh Error Cases ===");

    // 1. Non-existent ASG
    println!("\n[1/1] Starting instance refresh on non-existent ASG...");
    let body = aws_lite::types::autoscaling::StartInstanceRefreshRequest {
        auto_scaling_group_name: "cloud-lite-test-ralph-nonexistent-asg".to_string(),
        ..Default::default()
    };
    let result = asg.start_instance_refresh(&body).await;
    assert!(result.is_err(), "non-existent ASG should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}
