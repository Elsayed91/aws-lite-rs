//! Integration tests for AWS Lambda API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration lambda -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::lambda::*;
use std::env;

const TEST_REGION: &str = "eu-central-1";
const TEST_FUNCTION_NAME: &str = "cloud-lite-test-ralph-lambda";

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_lambda_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    println!("\n=== AWS Lambda Operations Test ===");
    println!("Region: {region}");

    run_lambda_tests(&client)
        .await
        .expect("Lambda operations tests failed");
    println!("\nAll Lambda operations tests passed!");
}

async fn run_lambda_tests(client: &AwsHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    // [1/4] ListFunctions — verify API is accessible and response structure is correct
    println!("\n[1/4] ListFunctions (all functions)...");
    let list_resp = client.lambda().list_functions("", "", "", "").await?;

    println!("  Found {} Lambda function(s)", list_resp.functions.len());
    for func in &list_resp.functions {
        assert!(
            !func.function_name.is_empty(),
            "Expected non-empty FunctionName"
        );
        assert!(
            !func.function_arn.is_empty(),
            "Expected non-empty FunctionArn"
        );
        println!(
            "  Function: name={} arn={} runtime={:?} timeout={:?}ms architectures={:?}",
            func.function_name, func.function_arn, func.runtime, func.timeout, func.architectures,
        );
    }

    // Find a suitable function to test GetFunctionConfiguration and UpdateFunctionConfiguration.
    // If we have any functions, use the first one. Otherwise test error path.
    if let Some(first_func) = list_resp.functions.first() {
        let function_name = first_func.function_name.clone();

        // [2/4] GetFunctionConfiguration — verify detailed config for an existing function
        println!("\n[2/4] GetFunctionConfiguration (function={function_name})...");
        let config = client
            .lambda()
            .get_function_configuration(&function_name, "")
            .await?;

        assert_eq!(
            config.function_name, function_name,
            "Expected FunctionName to match"
        );
        assert!(
            !config.function_arn.is_empty(),
            "Expected non-empty FunctionArn"
        );
        println!(
            "  Config: name={} arn={} timeout={:?} memory={:?} runtime={:?}",
            config.function_name,
            config.function_arn,
            config.timeout,
            config.memory_size,
            config.runtime,
        );

        // [3/4] UpdateFunctionConfiguration — update an existing function (only touch timeout,
        // restoring it to the same value to avoid any unintended side effects)
        let original_timeout = config.timeout.unwrap_or(3);
        println!(
            "\n[3/4] UpdateFunctionConfiguration (function={function_name}, restoring timeout={original_timeout}s)..."
        );
        let updated = client
            .lambda()
            .update_function_configuration(
                &function_name,
                &UpdateFunctionConfigurationRequest {
                    function_name: function_name.clone(),
                    timeout: Some(original_timeout),
                    ..Default::default()
                },
            )
            .await?;

        assert_eq!(
            updated.function_name, function_name,
            "Expected FunctionName to match after update"
        );
        assert_eq!(
            updated.timeout,
            Some(original_timeout),
            "Expected timeout to remain at {original_timeout}s after update"
        );
        println!(
            "  Updated function: name={} timeout={:?}s memory={:?}",
            updated.function_name, updated.timeout, updated.memory_size,
        );
    } else {
        println!("  No existing functions found — testing error paths instead");

        // [2/4] GetFunctionConfiguration — error path with non-existent function
        println!("\n[2/4] GetFunctionConfiguration (error case — non-existent function)...");
        let get_err = client
            .lambda()
            .get_function_configuration(TEST_FUNCTION_NAME, "")
            .await;

        assert!(
            get_err.is_err(),
            "Expected error when getting non-existent function configuration"
        );
        let err_msg = format!("{:?}", get_err.unwrap_err());
        assert!(
            err_msg.contains("ResourceNotFoundException")
                || err_msg.contains("404")
                || err_msg.contains("Function not found")
                || err_msg.contains(TEST_FUNCTION_NAME),
            "Expected not-found error, got: {err_msg}"
        );
        println!(
            "  Got expected error: {}",
            &err_msg[..err_msg.len().min(120)]
        );

        // [3/4] UpdateFunctionConfiguration — error path with non-existent function
        println!("\n[3/4] UpdateFunctionConfiguration (error case — non-existent function)...");
        let update_err = client
            .lambda()
            .update_function_configuration(
                TEST_FUNCTION_NAME,
                &UpdateFunctionConfigurationRequest {
                    function_name: TEST_FUNCTION_NAME.to_string(),
                    timeout: Some(10),
                    ..Default::default()
                },
            )
            .await;

        assert!(
            update_err.is_err(),
            "Expected error when updating non-existent function"
        );
        let err_msg = format!("{:?}", update_err.unwrap_err());
        assert!(
            err_msg.contains("ResourceNotFoundException")
                || err_msg.contains("404")
                || err_msg.contains("Function not found")
                || err_msg.contains(TEST_FUNCTION_NAME),
            "Expected not-found error for update, got: {err_msg}"
        );
        println!(
            "  Got expected error: {}",
            &err_msg[..err_msg.len().min(120)]
        );
    }

    // [4/4] ListFunctions with Marker pagination — verify pagination parameter works
    println!("\n[4/4] ListFunctions with MaxItems=1 (pagination test)...");
    let paginated = client.lambda().list_functions("", "", "", "1").await?;
    assert!(
        paginated.functions.len() <= 1,
        "Expected at most 1 function with MaxItems=1, got {}",
        paginated.functions.len()
    );
    println!(
        "  Paginated result: {} function(s), next_marker={:?}",
        paginated.functions.len(),
        paginated.next_marker,
    );

    Ok(())
}
