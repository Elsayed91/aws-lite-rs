//! Integration tests for SageMaker API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration sagemaker -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::sagemaker::*;
use std::env;

const TEST_REGION: &str = "eu-central-1";

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_sagemaker_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    println!("\n=== SageMaker Operations Test ===");
    println!("Region: {region}");

    run_sagemaker_tests(&client)
        .await
        .expect("SageMaker operations tests failed");
    println!("\nAll SageMaker operations tests passed!");
}

async fn run_sagemaker_tests(client: &AwsHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    // [1/3] ListNotebookInstances — verify API is accessible and response structure is correct
    println!("\n[1/3] ListNotebookInstances (all instances)...");
    let list_resp = client
        .sagemaker()
        .list_notebook_instances(&ListNotebookInstancesInput::default())
        .await?;

    println!(
        "  Found {} notebook instance(s)",
        list_resp.notebook_instances.len()
    );

    // Verify structure of any instances that exist
    for instance in &list_resp.notebook_instances {
        assert!(
            !instance.notebook_instance_name.is_empty(),
            "Expected non-empty NotebookInstanceName"
        );
        assert!(
            !instance.notebook_instance_arn.is_empty(),
            "Expected non-empty NotebookInstanceArn"
        );
        assert!(
            instance.notebook_instance_status.is_some(),
            "Expected NotebookInstanceStatus to be present"
        );
        println!(
            "  Instance: name={} arn={} status={:?} type={:?}",
            instance.notebook_instance_name,
            instance.notebook_instance_arn,
            instance.notebook_instance_status,
            instance.instance_type,
        );
    }

    // [2/3] ListNotebookInstances with StatusEquals filter
    println!("\n[2/3] ListNotebookInstances (filter by InService status)...");
    let list_filtered = client
        .sagemaker()
        .list_notebook_instances(&ListNotebookInstancesInput {
            status_equals: Some("InService".to_string()),
            ..Default::default()
        })
        .await?;
    println!(
        "  Found {} InService notebook instance(s)",
        list_filtered.notebook_instances.len()
    );
    // All returned instances should have InService status
    for instance in &list_filtered.notebook_instances {
        assert_eq!(
            instance.notebook_instance_status.as_deref(),
            Some("InService"),
            "Expected only InService instances in filtered list"
        );
    }

    // [3/3] StopNotebookInstance — test error path with non-existent instance
    // We don't create real notebook instances (they take ~5 min to provision and cost money),
    // but we verify the API is reachable and returns a proper error for an invalid name.
    println!("\n[3/3] StopNotebookInstance (error case — non-existent instance)...");
    let stop_err = client
        .sagemaker()
        .stop_notebook_instance(&StopNotebookInstanceInput {
            notebook_instance_name: "cloud-lite-test-ralph-nonexistent-notebook".to_string(),
        })
        .await;

    assert!(
        stop_err.is_err(),
        "Expected error when stopping non-existent notebook instance"
    );
    let err_msg = format!("{:?}", stop_err.unwrap_err());
    // SageMaker returns ValidationException for non-existent resources
    assert!(
        err_msg.contains("ValidationException")
            || err_msg.contains("RecordNotFound")
            || err_msg.contains("does not exist"),
        "Expected ValidationException or not-found error, got: {err_msg}"
    );
    println!(
        "  Got expected error for non-existent notebook: {}",
        &err_msg[..err_msg.len().min(120)]
    );

    Ok(())
}
