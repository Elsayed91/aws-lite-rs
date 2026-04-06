//! Integration tests for ECR (Elastic Container Registry) API
//!
//! Run with:
//!   cargo test -p aws-lite --test integration ecr -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use aws_lite::types::ecr::*;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_REGION: &str = "eu-central-1";
const TEST_REPO_NAME: &str = "cloud-lite-test-ralph-ecr";

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

fn aws_create_repository(region: &str) -> Option<String> {
    let v = aws(&[
        "ecr",
        "create-repository",
        "--repository-name",
        TEST_REPO_NAME,
        "--region",
        region,
        "--output",
        "json",
    ])?;
    v["repository"]["repositoryArn"].as_str().map(String::from)
}

fn aws_delete_repository(region: &str) {
    aws_cleanup(&[
        "ecr",
        "delete-repository",
        "--repository-name",
        TEST_REPO_NAME,
        "--force",
        "--region",
        region,
    ]);
}

/// Push a minimal Docker image to the ECR repo so DescribeImages has data.
/// Returns the image digest if successful.
fn push_test_image(region: &str, repo_uri: &str) -> Option<String> {
    // Get ECR login password
    let login_output = Command::new("aws")
        .args(["ecr", "get-login-password", "--region", region])
        .output()
        .ok()?;
    if !login_output.status.success() {
        eprintln!("  Warning: failed to get ECR login password");
        return None;
    }
    let password = String::from_utf8_lossy(&login_output.stdout)
        .trim()
        .to_string();

    // Extract registry URL from repo URI (everything before the first /)
    let registry = repo_uri.split('/').next()?;

    // Docker login
    let login = Command::new("docker")
        .args(["login", "--username", "AWS", "--password-stdin", registry])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();
    match login {
        Ok(mut child) => {
            use std::io::Write;
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(password.as_bytes());
            }
            let output = child.wait_with_output().ok()?;
            if !output.status.success() {
                eprintln!("  Warning: docker login failed");
                return None;
            }
        }
        Err(_) => {
            eprintln!("  Warning: docker not available, skipping image push");
            return None;
        }
    }

    // Pull a tiny image
    let pull = Command::new("docker")
        .args(["pull", "hello-world:latest"])
        .output()
        .ok()?;
    if !pull.status.success() {
        eprintln!("  Warning: docker pull failed");
        return None;
    }

    // Tag for ECR
    let ecr_tag = format!("{repo_uri}:test-image");
    let tag = Command::new("docker")
        .args(["tag", "hello-world:latest", &ecr_tag])
        .output()
        .ok()?;
    if !tag.status.success() {
        eprintln!("  Warning: docker tag failed");
        return None;
    }

    // Push to ECR
    let push = Command::new("docker")
        .args(["push", &ecr_tag])
        .output()
        .ok()?;
    if !push.status.success() {
        eprintln!(
            "  Warning: docker push failed: {}",
            String::from_utf8_lossy(&push.stderr)
        );
        return None;
    }

    // Get the digest from DescribeImages via CLI
    std::thread::sleep(Duration::from_secs(2));
    let v = aws(&[
        "ecr",
        "describe-images",
        "--repository-name",
        TEST_REPO_NAME,
        "--region",
        region,
        "--output",
        "json",
    ])?;
    v["imageDetails"][0]["imageDigest"]
        .as_str()
        .map(String::from)
}

fn cleanup_all(region: &str) {
    aws_delete_repository(region);
}

// --- Tests ---

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_ecr_operations() {
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| TEST_REGION.to_string());

    // Pre-cleanup
    cleanup_all(&region);
    tokio::time::sleep(Duration::from_secs(2)).await;

    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_ecr_tests(&client, &region).await;

    // Always cleanup
    cleanup_all(&region);

    result.expect("ECR operations tests failed");
    println!("\nAll ECR operations tests passed!");
}

async fn run_ecr_tests(
    client: &AwsHttpClient,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== ECR Operations Test ===");
    println!("Region: {region}");

    // [0/6] Create fixtures
    println!("\n[0/6] Creating test fixtures...");
    let repo_arn = aws_create_repository(region).expect("Failed to create ECR repository");
    println!("  Created repository: {repo_arn}");
    tokio::time::sleep(Duration::from_secs(2)).await;

    // [1/6] DescribeRepositories
    println!("\n[1/6] DescribeRepositories...");
    let desc_repos = client
        .ecr()
        .describe_repositories(&DescribeRepositoriesRequest {
            repository_names: vec![TEST_REPO_NAME.to_string()],
            ..Default::default()
        })
        .await?;
    assert_eq!(desc_repos.repositories.len(), 1, "Expected 1 repository");
    let repo = &desc_repos.repositories[0];
    assert_eq!(repo.repository_name.as_deref(), Some(TEST_REPO_NAME));
    assert!(repo.repository_arn.is_some());
    assert!(repo.repository_uri.is_some());
    assert!(repo.created_at.is_some());
    let repo_uri = repo.repository_uri.clone().unwrap();
    println!(
        "  Repo: {} (uri={}, tag_mutability={})",
        repo.repository_name.as_deref().unwrap_or("?"),
        repo_uri,
        repo.image_tag_mutability.as_deref().unwrap_or("?"),
    );

    // [2/6] DescribeImages (empty repo)
    println!("\n[2/6] DescribeImages (empty repo)...");
    let desc_images_empty = client
        .ecr()
        .describe_images(&DescribeImagesRequest {
            repository_name: TEST_REPO_NAME.to_string(),
            ..Default::default()
        })
        .await?;
    println!(
        "  Images in empty repo: {}",
        desc_images_empty.image_details.len()
    );
    // Empty repo should have 0 images
    assert!(
        desc_images_empty.image_details.is_empty(),
        "Expected empty repo to have no images, got: {}",
        desc_images_empty.image_details.len(),
    );

    // [3/6] Push test image and DescribeImages
    println!("\n[3/6] Pushing test image and DescribeImages...");
    let image_digest = push_test_image(region, &repo_uri);
    if let Some(ref digest) = image_digest {
        println!("  Pushed image with digest: {digest}");
        let desc_images = client
            .ecr()
            .describe_images(&DescribeImagesRequest {
                repository_name: TEST_REPO_NAME.to_string(),
                ..Default::default()
            })
            .await?;
        assert!(
            !desc_images.image_details.is_empty(),
            "Expected at least 1 image after push"
        );
        let img = &desc_images.image_details[0];
        assert_eq!(img.image_digest.as_deref(), Some(digest.as_str()));
        assert!(img.image_pushed_at.is_some());
        assert!(img.image_size_in_bytes.is_some());
        println!(
            "  Image: digest={}, tags={:?}, size={} bytes",
            img.image_digest.as_deref().unwrap_or("?"),
            img.image_tags,
            img.image_size_in_bytes.unwrap_or(0),
        );
    } else {
        println!("  Skipped image push (docker not available) — testing with empty repo");
    }

    // [4/6] PutLifecyclePolicy
    println!("\n[4/6] PutLifecyclePolicy...");
    let lifecycle_policy = serde_json::json!({
        "rules": [{
            "rulePriority": 1,
            "description": "Expire untagged images older than 1 day",
            "selection": {
                "tagStatus": "untagged",
                "countType": "sinceImagePushed",
                "countUnit": "days",
                "countNumber": 1
            },
            "action": {
                "type": "expire"
            }
        }]
    });
    let policy_text = serde_json::to_string(&lifecycle_policy)?;
    let put_policy = client
        .ecr()
        .put_lifecycle_policy(&PutLifecyclePolicyRequest {
            repository_name: TEST_REPO_NAME.to_string(),
            lifecycle_policy_text: policy_text.clone(),
            ..Default::default()
        })
        .await?;
    assert_eq!(put_policy.repository_name.as_deref(), Some(TEST_REPO_NAME));
    assert!(put_policy.lifecycle_policy_text.is_some());
    println!(
        "  Set lifecycle policy on {} (policy length={})",
        put_policy.repository_name.as_deref().unwrap_or("?"),
        put_policy
            .lifecycle_policy_text
            .as_deref()
            .unwrap_or("")
            .len(),
    );

    // [5/6] BatchDeleteImage
    println!("\n[5/6] BatchDeleteImage...");
    if let Some(ref digest) = image_digest {
        // Delete the pushed image by digest
        let delete_resp = client
            .ecr()
            .batch_delete_image(&BatchDeleteImageRequest {
                repository_name: TEST_REPO_NAME.to_string(),
                image_ids: vec![ImageIdentifier {
                    image_digest: Some(digest.clone()),
                    ..Default::default()
                }],
                ..Default::default()
            })
            .await?;
        assert!(
            !delete_resp.image_ids.is_empty(),
            "Expected deleted image IDs"
        );
        assert!(
            delete_resp.failures.is_empty(),
            "Expected no failures, got: {:?}",
            delete_resp.failures,
        );
        println!(
            "  Deleted {} image(s), {} failure(s)",
            delete_resp.image_ids.len(),
            delete_resp.failures.len(),
        );
    } else {
        // Test with a non-existent digest — should return failure
        let delete_resp = client
            .ecr()
            .batch_delete_image(&BatchDeleteImageRequest {
                repository_name: TEST_REPO_NAME.to_string(),
                image_ids: vec![ImageIdentifier {
                    image_digest: Some(
                        "sha256:0000000000000000000000000000000000000000000000000000000000000000"
                            .to_string(),
                    ),
                    ..Default::default()
                }],
                ..Default::default()
            })
            .await?;
        assert!(
            !delete_resp.failures.is_empty(),
            "Expected failures for non-existent image"
        );
        let failure = &delete_resp.failures[0];
        println!(
            "  Got expected failure: code={}, reason={}",
            failure.failure_code.as_deref().unwrap_or("?"),
            failure.failure_reason.as_deref().unwrap_or("?"),
        );
    }

    // [6/6] DescribeRepositories with non-existent repo
    println!("\n[6/6] DescribeRepositories error case...");
    let err = client
        .ecr()
        .describe_repositories(&DescribeRepositoriesRequest {
            repository_names: vec!["cloud-lite-test-nonexistent-repo".to_string()],
            ..Default::default()
        })
        .await;
    assert!(err.is_err(), "Expected error for non-existent repository");
    let err_msg = format!("{}", err.unwrap_err());
    assert!(
        err_msg.contains("RepositoryNotFoundException") || err_msg.contains("does not exist"),
        "Expected RepositoryNotFoundException, got: {err_msg}",
    );
    println!("  Got expected error: {err_msg}");

    Ok(())
}
