//! Integration tests for Amazon API Gateway API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration apigateway -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::apigateway::{PatchOperation, UpdateStageRequest};
use std::env;
use std::process::Command;

const TEST_REGION: &str = "eu-central-1";
const TEST_API_NAME: &str = "cloud-lite-test-ralph-apigateway";
const TEST_STAGE_NAME: &str = "cloud-lite-test-ralph-stage";

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

fn aws_cleanup(region: &str, service: &str, args: &[&str]) {
    let mut cmd_args: Vec<&str> = vec![service];
    cmd_args.extend_from_slice(args);
    cmd_args.extend_from_slice(&["--region", region]);
    let _ = Command::new("aws").args(&cmd_args).output();
}

/// Find and delete any existing test REST APIs with our test name.
fn cleanup_test_apis(region: &str) {
    if let Some(val) = aws(region, "apigateway", &["get-rest-apis"]) {
        if let Some(items) = val["items"].as_array() {
            for item in items {
                if item["name"].as_str() == Some(TEST_API_NAME) {
                    if let Some(id) = item["id"].as_str() {
                        eprintln!("  Deleting leftover test REST API: {id}");
                        aws_cleanup(
                            region,
                            "apigateway",
                            &["delete-rest-api", "--rest-api-id", id],
                        );
                        // API Gateway enforces a 30s throttle between deletes
                        std::thread::sleep(std::time::Duration::from_secs(30));
                    }
                }
            }
        }
    }
}

/// Create a REST API with a MOCK method and a deployment+stage.
/// Returns (api_id, deployment_id) on success.
fn create_test_api_with_stage(region: &str) -> Option<(String, String)> {
    // Create the REST API
    let api = aws(
        region,
        "apigateway",
        &["create-rest-api", "--name", TEST_API_NAME],
    )?;
    let api_id = api["id"].as_str()?.to_string();
    let root_resource_id = api["rootResourceId"].as_str()?.to_string();
    eprintln!("  Created REST API: id={api_id} rootResource={root_resource_id}");

    // Add a GET method on the root resource
    aws(
        region,
        "apigateway",
        &[
            "put-method",
            "--rest-api-id",
            &api_id,
            "--resource-id",
            &root_resource_id,
            "--http-method",
            "GET",
            "--authorization-type",
            "NONE",
        ],
    )?;

    // Add a MOCK integration (required for deployment)
    aws(
        region,
        "apigateway",
        &[
            "put-integration",
            "--rest-api-id",
            &api_id,
            "--resource-id",
            &root_resource_id,
            "--http-method",
            "GET",
            "--type",
            "MOCK",
            "--request-templates",
            r#"{"application/json":"{\"statusCode\": 200}"}"#,
        ],
    )?;

    // Create a deployment with a stage
    let deployment = aws(
        region,
        "apigateway",
        &[
            "create-deployment",
            "--rest-api-id",
            &api_id,
            "--stage-name",
            TEST_STAGE_NAME,
        ],
    )?;
    let deployment_id = deployment["id"].as_str()?.to_string();
    eprintln!("  Created deployment: id={deployment_id}, stage={TEST_STAGE_NAME}");

    Some((api_id, deployment_id))
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_apigateway_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    println!("\n=== AWS API Gateway Operations Test ===");
    println!("Region: {region}");

    // Pre-cleanup
    println!("[pre] Cleaning up leftover test resources...");
    cleanup_test_apis(&region);

    let result = run_apigateway_tests(&client, &region).await;

    // Always cleanup
    println!("[post] Cleaning up test resources...");
    cleanup_test_apis(&region);

    result.expect("API Gateway operations tests failed");
    println!("\nAll API Gateway operations tests passed!");
}

async fn run_apigateway_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // [0/5] Create test fixtures via CLI
    println!("\n[0/5] Creating test REST API with stage via CLI...");
    let (api_id, _deployment_id) =
        create_test_api_with_stage(region).ok_or("Failed to create test REST API with stage")?;
    println!("  REST API ID: {api_id}");

    // [1/5] GetRestApis — list all APIs, verify our test API is present
    println!("\n[1/5] GetRestApis...");
    let list_resp = client.apigateway().get_rest_apis("", "").await?;

    let found = list_resp
        .items
        .iter()
        .find(|a| a.id.as_deref() == Some(&api_id));
    assert!(
        found.is_some(),
        "Expected to find test REST API '{api_id}' in GetRestApis response"
    );
    let api = found.unwrap();
    assert_eq!(
        api.name.as_deref(),
        Some(TEST_API_NAME),
        "Expected API name to match"
    );
    println!(
        "  Found {} API(s), test API: id={:?} name={:?}",
        list_resp.items.len(),
        api.id,
        api.name
    );

    // [2/5] GetStages — list stages for our test API
    println!("\n[2/5] GetStages (api_id={api_id})...");
    let stages_resp = client.apigateway().get_stages(&api_id, "").await?;

    assert!(!stages_resp.item.is_empty(), "Expected at least one stage");
    let stage = stages_resp
        .item
        .iter()
        .find(|s| s.stage_name.as_deref() == Some(TEST_STAGE_NAME));
    assert!(
        stage.is_some(),
        "Expected to find stage '{TEST_STAGE_NAME}'"
    );
    let stage = stage.unwrap();
    assert!(
        stage
            .deployment_id
            .as_deref()
            .is_some_and(|s| !s.is_empty()),
        "Expected non-empty deploymentId"
    );
    assert_eq!(
        stage.cache_cluster_enabled,
        Some(false),
        "Expected cacheClusterEnabled=false by default"
    );
    println!(
        "  Found {} stage(s), test stage: name={:?} deploymentId={:?} cacheEnabled={:?} tracing={:?}",
        stages_resp.item.len(),
        stage.stage_name,
        stage.deployment_id,
        stage.cache_cluster_enabled,
        stage.tracing_enabled
    );

    // [3/5] UpdateStage — enable tracing via PATCH
    println!("\n[3/5] UpdateStage (enable tracing on stage={TEST_STAGE_NAME})...");
    let updated_stage = client
        .apigateway()
        .update_stage(
            &api_id,
            TEST_STAGE_NAME,
            &UpdateStageRequest {
                rest_api_id: api_id.clone(),
                stage_name: TEST_STAGE_NAME.to_string(),
                patch_operations: vec![PatchOperation {
                    op: Some("replace".to_string()),
                    path: Some("/tracingEnabled".to_string()),
                    value: Some("true".to_string()),
                    ..Default::default()
                }],
            },
        )
        .await?;

    assert_eq!(
        updated_stage.tracing_enabled,
        Some(true),
        "Expected tracing to be enabled after update"
    );
    assert_eq!(
        updated_stage.stage_name.as_deref(),
        Some(TEST_STAGE_NAME),
        "Expected stage name to be preserved"
    );
    println!(
        "  Updated stage: name={:?} tracing={:?} cacheEnabled={:?}",
        updated_stage.stage_name,
        updated_stage.tracing_enabled,
        updated_stage.cache_cluster_enabled
    );

    // [4/5] UpdateStage — restore tracing=false (cleanup-friendly)
    println!("\n[4/5] UpdateStage (restore tracing=false)...");
    let restored_stage = client
        .apigateway()
        .update_stage(
            &api_id,
            TEST_STAGE_NAME,
            &UpdateStageRequest {
                rest_api_id: api_id.clone(),
                stage_name: TEST_STAGE_NAME.to_string(),
                patch_operations: vec![PatchOperation {
                    op: Some("replace".to_string()),
                    path: Some("/tracingEnabled".to_string()),
                    value: Some("false".to_string()),
                    ..Default::default()
                }],
            },
        )
        .await?;

    assert_eq!(
        restored_stage.tracing_enabled,
        Some(false),
        "Expected tracing to be restored to false"
    );
    println!(
        "  Restored stage: tracing={:?}",
        restored_stage.tracing_enabled
    );

    // [5/5] GetStages error case — non-existent API
    println!("\n[5/5] GetStages error case (non-existent API)...");
    let err = client
        .apigateway()
        .get_stages("nonexistent123abc", "")
        .await;

    assert!(err.is_err(), "Expected error for non-existent REST API");
    let err_msg = format!("{:?}", err.unwrap_err());
    assert!(
        err_msg.contains("NotFoundException")
            || err_msg.contains("404")
            || err_msg.contains("Invalid API identifier"),
        "Expected NotFoundException or 404, got: {err_msg}"
    );
    println!(
        "  Got expected error: {}",
        &err_msg[..err_msg.len().min(120)]
    );

    Ok(())
}
