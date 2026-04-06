//! Integration tests for AWS Config API
//!
//! Tests SelectResourceConfig against the real AWS Config API.
//! Uses AWS CLI to create test fixtures (SNS topic) for querying.
//!
//! Run with:
//!   AWS_PROFILE=<profile> AWS_REGION=eu-central-1 \
//!     cargo test --test integration config -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::config::*;
use std::env;
use std::pin::pin;
use std::process::Command;
use tokio_stream::StreamExt;

const TEST_TOPIC_NAME: &str = "cloud-lite-test-config-integration";

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "eu-central-1".into())
}

/// Create an SNS topic via AWS CLI so Config has something to discover.
fn create_sns_topic(region: &str) -> Option<String> {
    let output = Command::new("aws")
        .args([
            "sns",
            "create-topic",
            "--name",
            TEST_TOPIC_NAME,
            "--region",
            region,
        ])
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Extract TopicArn from JSON output
        serde_json::from_str::<serde_json::Value>(&stdout)
            .ok()
            .and_then(|v| v["TopicArn"].as_str().map(String::from))
    } else {
        eprintln!(
            "  Warning: failed to create SNS topic: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        None
    }
}

/// Delete the test SNS topic via AWS CLI.
fn delete_sns_topic(arn: &str, region: &str) {
    let _ = Command::new("aws")
        .args([
            "sns",
            "delete-topic",
            "--topic-arn",
            arn,
            "--region",
            region,
        ])
        .output();
}

// -- SelectResourceConfig ---------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials and AWS Config enabled"]
async fn test_select_resource_config() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let config = client.config();

    // Pre-cleanup: delete leftover test topic
    println!("\n=== Setup: creating SNS topic fixture ===");
    let account_id = env::var("AWS_ACCOUNT_ID").unwrap_or_default();
    if !account_id.is_empty() {
        let arn = format!("arn:aws:sns:{region}:{account_id}:{TEST_TOPIC_NAME}");
        delete_sns_topic(&arn, &region);
    }

    // Create test fixture
    let topic_arn = create_sns_topic(&region);
    if let Some(ref arn) = topic_arn {
        println!("  Created SNS topic: {arn}");
    } else {
        println!("  Skipping SNS fixture (creation failed), testing with existing resources");
    }

    let result = run_select_tests(&config).await;

    // Always cleanup
    if let Some(ref arn) = topic_arn {
        println!("\n=== Cleanup: deleting SNS topic ===");
        delete_sns_topic(arn, &region);
        println!("  Deleted {arn}");
    }

    result
}

async fn run_select_tests(
    config: &aws_lite::api::ConfigClient<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== SelectResourceConfig ===");

    // 1. Basic query — multiple resource types exist (Config recorder tracks all)
    println!("\n[1/8] Querying all resource types with COUNT aggregation...");
    let request = SelectResourceConfigRequest {
        expression: "SELECT resourceType, COUNT(*) WHERE resourceType LIKE 'AWS::%' \
                     GROUP BY resourceType"
            .into(),
        ..Default::default()
    };
    let response = config.select_resource_config(&request).await?;
    println!("  Resource type groups: {}", response.results.len());
    assert!(
        !response.results.is_empty(),
        "Config should have discovered at least some resources"
    );
    for (i, result) in response.results.iter().enumerate().take(5) {
        println!("  [{i}]: {result}");
    }

    // 2. Query with specific fields — validate QueryInfo reflects SELECT columns
    println!("\n[2/8] Querying EC2 Subnets with specific fields...");
    let request = SelectResourceConfigRequest {
        expression: "SELECT resourceId, resourceType, accountId, awsRegion, tags \
                     WHERE resourceType = 'AWS::EC2::Subnet'"
            .into(),
        ..Default::default()
    };
    let response = config.select_resource_config(&request).await?;
    println!("  Subnet results: {}", response.results.len());
    let query_fields: Vec<&str> = response
        .query_info
        .as_ref()
        .map(|q| {
            q.select_fields
                .iter()
                .filter_map(|f| f.name.as_deref())
                .collect()
        })
        .unwrap_or_default();
    println!("  QueryInfo fields: {query_fields:?}");
    assert!(
        query_fields.contains(&"resourceId"),
        "QueryInfo should include resourceId"
    );
    assert!(
        query_fields.contains(&"tags"),
        "QueryInfo should include tags"
    );

    // 3. Pagination — use limit=1 to force multiple pages on a type with >1 resource
    println!("\n[3/8] Testing pagination with limit=1 on CodeDeploy configs...");
    let request = SelectResourceConfigRequest {
        expression: "SELECT resourceId, resourceType \
                     WHERE resourceType = 'AWS::CodeDeploy::DeploymentConfig'"
            .into(),
        limit: Some(1),
        ..Default::default()
    };
    let page1 = config.select_resource_config(&request).await?;
    println!("  Page 1 results: {}", page1.results.len());
    println!(
        "  Page 1 NextToken: {:?}",
        page1.next_token.as_deref().map(|t| &t[..t.len().min(30)])
    );
    assert_eq!(
        page1.results.len(),
        1,
        "limit=1 should return exactly 1 result"
    );

    if let Some(ref token) = page1.next_token {
        println!("  Fetching page 2 with NextToken...");
        let request_p2 = SelectResourceConfigRequest {
            expression: "SELECT resourceId, resourceType \
                         WHERE resourceType = 'AWS::CodeDeploy::DeploymentConfig'"
                .into(),
            limit: Some(1),
            next_token: Some(token.clone()),
            ..Default::default()
        };
        let page2 = config.select_resource_config(&request_p2).await?;
        println!("  Page 2 results: {}", page2.results.len());
        assert_eq!(page2.results.len(), 1, "page 2 should have 1 result");

        // Verify pages return different resources
        assert_ne!(
            page1.results[0], page2.results[0],
            "page 1 and page 2 should return different resources"
        );
        println!("  Confirmed: pages return different resources");
    } else {
        println!("  Warning: no NextToken, cannot test pagination (only 1 resource?)");
    }

    // 4. Pagination stream — collect all CodeDeploy configs via stream
    println!("\n[4/8] Testing pagination stream on CodeDeploy configs...");
    let mut stream = pin!(config.select_resource_config_stream(
        "SELECT resourceId WHERE resourceType = 'AWS::CodeDeploy::DeploymentConfig'",
    ));
    let mut streamed: Vec<String> = Vec::new();
    while let Some(result) = stream.next().await {
        streamed.push(result?);
    }
    println!("  Streamed {} total results", streamed.len());
    assert!(
        streamed.len() > 1,
        "stream should collect multiple CodeDeploy configs"
    );

    // 5. Query with WHERE filter — non-existent resource
    println!("\n[5/8] Querying non-existent resource ID...");
    let request = SelectResourceConfigRequest {
        expression: "SELECT resourceId WHERE resourceType = 'AWS::EC2::Subnet' \
                     AND resourceId = 'subnet-nonexistent-cloud-lite-test-00000'"
            .into(),
        ..Default::default()
    };
    let response = config.select_resource_config(&request).await?;
    println!("  Results: {}", response.results.len());
    assert!(
        response.results.is_empty(),
        "non-existent resource should return empty results"
    );

    // 6. Query with configuration field — returns nested JSON
    println!("\n[6/8] Querying with configuration field...");
    let request = SelectResourceConfigRequest {
        expression: "SELECT resourceId, resourceType, configuration \
                     WHERE resourceType = 'AWS::EC2::VPC'"
            .into(),
        ..Default::default()
    };
    let response = config.select_resource_config(&request).await?;
    println!("  VPC results: {}", response.results.len());
    if let Some(first) = response.results.first() {
        println!(
            "  First result (truncated): {}...",
            &first[..first.len().min(200)]
        );
        // Parse to verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(first)?;
        assert!(
            parsed.get("configuration").is_some(),
            "result should contain configuration field"
        );
        println!("  Confirmed: configuration field is present and parseable");
    }

    // 7. Empty expression body (just SELECT * equivalent)
    println!("\n[7/8] Querying with SELECT * (all fields, single resource type)...");
    let request = SelectResourceConfigRequest {
        expression: "SELECT * WHERE resourceType = 'AWS::EC2::VPC'".into(),
        limit: Some(1),
        ..Default::default()
    };
    let response = config.select_resource_config(&request).await?;
    println!("  SELECT * results: {}", response.results.len());
    if let Some(first) = response.results.first() {
        let parsed: serde_json::Value = serde_json::from_str(first)?;
        let keys: Vec<&str> = parsed
            .as_object()
            .map_or(vec![], |m| m.keys().map(|k| k.as_str()).collect());
        println!("  Fields returned by SELECT *: {keys:?}");
        // SELECT * should return more fields than a targeted SELECT
        assert!(keys.len() > 3, "SELECT * should return many fields");
    }

    // 8. Query with LIKE operator
    println!("\n[8/8] Querying with LIKE operator...");
    let request = SelectResourceConfigRequest {
        expression: "SELECT resourceId, resourceType WHERE resourceType LIKE 'AWS::EC2::%'".into(),
        ..Default::default()
    };
    let response = config.select_resource_config(&request).await?;
    println!("  EC2 resources (all types): {}", response.results.len());
    assert!(
        !response.results.is_empty(),
        "should find at least some EC2 resources"
    );
    // Verify all results are EC2 resources
    for result in &response.results {
        let parsed: serde_json::Value = serde_json::from_str(result)?;
        let rt = parsed["resourceType"].as_str().unwrap_or("");
        assert!(
            rt.starts_with("AWS::EC2::"),
            "LIKE filter should only return EC2 resources, got: {rt}"
        );
    }
    println!("  Confirmed: all results match LIKE 'AWS::EC2::%'");

    println!("\nAll SelectResourceConfig tests passed!");
    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials and AWS Config enabled"]
async fn test_select_resource_config_error_cases() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let config = client.config();

    println!("\n=== SelectResourceConfig Error Cases ===");

    // 1. Completely invalid SQL syntax
    println!("\n[1/4] Invalid SQL syntax...");
    let request = SelectResourceConfigRequest {
        expression: "THIS IS NOT VALID SQL".into(),
        ..Default::default()
    };
    let result = config.select_resource_config(&request).await;
    assert!(result.is_err(), "invalid SQL should fail");
    let err = result.unwrap_err();
    println!("  Error: {err}");
    assert!(
        format!("{err}").contains("InvalidExpressionException"),
        "should be InvalidExpressionException"
    );

    // 2. SQL LIMIT keyword (not supported — must use request body limit)
    println!("\n[2/4] SQL LIMIT keyword (unsupported)...");
    let request = SelectResourceConfigRequest {
        expression: "SELECT resourceId WHERE resourceType = 'AWS::EC2::VPC' LIMIT 1".into(),
        ..Default::default()
    };
    let result = config.select_resource_config(&request).await;
    assert!(result.is_err(), "SQL LIMIT should be rejected");
    let err = result.unwrap_err();
    println!("  Error: {err}");
    assert!(
        format!("{err}").contains("LIMIT"),
        "error message should mention LIMIT"
    );

    // 3. Empty expression
    println!("\n[3/4] Empty expression...");
    let request = SelectResourceConfigRequest {
        expression: "".into(),
        ..Default::default()
    };
    let result = config.select_resource_config(&request).await;
    assert!(result.is_err(), "empty expression should fail");
    println!("  Error: {}", result.unwrap_err());

    // 4. Invalid column name in SELECT
    println!("\n[4/4] Invalid column name...");
    let request = SelectResourceConfigRequest {
        expression: "SELECT nonExistentField WHERE resourceType = 'AWS::EC2::VPC'".into(),
        ..Default::default()
    };
    let result = config.select_resource_config(&request).await;
    // This might succeed with empty field or fail — let's see what the real API does
    match result {
        Ok(resp) => println!(
            "  Surprisingly succeeded with {} results",
            resp.results.len()
        ),
        Err(err) => println!("  Error: {err}"),
    }

    println!("\nAll error case tests passed!");
    Ok(())
}

// -- DescribeConfigurationRecorders / DescribeConfigurationRecorderStatus ----

#[tokio::test]
#[ignore = "requires AWS credentials and AWS Config enabled"]
async fn test_describe_configuration_recorders() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let config = client.config();

    println!("\n=== DescribeConfigurationRecorders Integration Test ===");
    println!("Region: {}", region);

    // [1/3] List all recorders via describe_configuration_recorders
    println!("\n[1/3] Describing all configuration recorders...");
    let resp = config.describe_configuration_recorders(&[]).await?;
    println!("  Found {} recorder(s)", resp.configuration_recorders.len());

    for r in &resp.configuration_recorders {
        println!(
            "  - name={:?} arn={:?} role_arn={:?}",
            r.name,
            r.arn,
            r.role_arn.as_deref().map(|s| &s[..s.len().min(60)])
        );
        assert!(r.name.is_some(), "recorder should have a name");
    }

    // [2/3] list_configuration_recorders convenience wrapper
    println!("\n[2/3] list_configuration_recorders convenience method...");
    let recorders = config.list_configuration_recorders().await?;
    assert_eq!(
        recorders.len(),
        resp.configuration_recorders.len(),
        "convenience method should return same count"
    );
    println!("  Confirmed: {} recorder(s)", recorders.len());

    // [3/3] CIS 4.3 compliance report
    if recorders.is_empty() {
        println!("\n  WARNING (CIS 4.3): No configuration recorder found in region {region}.");
        println!("  To remediate: create a configuration recorder in this region.");
    } else {
        println!(
            "\n  CIS 4.3: {} recorder(s) found in region {region}. Recorder exists.",
            recorders.len()
        );
    }

    println!("\nAll DescribeConfigurationRecorders tests passed!");
    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials and AWS Config enabled"]
async fn test_describe_configuration_recorder_status() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let config = client.config();

    println!("\n=== DescribeConfigurationRecorderStatus Integration Test ===");
    println!("Region: {}", region);

    // [1/3] Describe status for all recorders
    println!("\n[1/3] Describing configuration recorder status...");
    let resp = config.describe_configuration_recorder_status(&[]).await?;
    println!(
        "  Found {} recorder status entry(ies)",
        resp.configuration_recorders_status.len()
    );

    for s in &resp.configuration_recorders_status {
        println!(
            "  - name={:?} recording={:?} last_status={:?}",
            s.name, s.recording, s.last_status
        );
        assert!(s.name.is_some(), "status entry should have a name");
    }

    // [2/3] list_configuration_recorder_statuses convenience wrapper
    println!("\n[2/3] list_configuration_recorder_statuses convenience method...");
    let statuses = config.list_configuration_recorder_statuses().await?;
    assert_eq!(
        statuses.len(),
        resp.configuration_recorders_status.len(),
        "convenience method should return same count"
    );
    println!("  Confirmed: {} status entry(ies)", statuses.len());

    // [3/3] CIS 4.3 compliance report: recorder must be recording and not in Failure state
    println!("\n[3/3] CIS 4.3 compliance check...");
    let mut compliant_count = 0;
    for s in &statuses {
        let recording = s.recording.unwrap_or(false);
        let is_failure = matches!(
            s.last_status,
            Some(aws_lite::types::config::RecorderStatus::Failure)
        );
        let name = s.name.as_deref().unwrap_or("<unknown>");

        if recording && !is_failure {
            println!("  COMPLIANT: recorder '{name}' is actively recording.");
            compliant_count += 1;
        } else if !recording {
            println!("  NON-COMPLIANT (CIS 4.3): recorder '{name}' is NOT recording.");
        } else {
            println!("  NON-COMPLIANT (CIS 4.3): recorder '{name}' has last_status=Failure.");
        }
    }

    if statuses.is_empty() {
        println!("  WARNING (CIS 4.3): No recorder status found in region {region}.");
    } else {
        println!(
            "  {compliant_count}/{} recorders compliant.",
            statuses.len()
        );
    }

    println!("\nAll DescribeConfigurationRecorderStatus tests passed!");
    Ok(())
}
