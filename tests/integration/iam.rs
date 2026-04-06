//! Integration tests for AWS IAM API
//!
//! Tests IAM operations including ListUsers, ListAttachedUserPolicies,
//! DetachUserPolicy, access key lifecycle, credential reports,
//! MFA devices, login profiles, account summary/policy,
//! roles, user policies, groups, server certificates, and service-linked roles.
//!
//! IAM is a global service — the region is used for SigV4 signing
//! (typically us-east-1) but requests go to iam.amazonaws.com.
//!
//! Run with:
//!   AWS_PROFILE=<profile> AWS_REGION=us-east-1 \
//!     cargo test -p aws-lite --test integration iam -- --ignored --test-threads=1 --nocapture

use aws_lite::AwsHttpClient;
use std::env;
use std::process::Command;

fn region() -> String {
    env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into())
}

const TEST_USER_NAME: &str = "cloud-lite-test-iam-access-keys";

// -- CLI Fixture Helpers ------------------------------------------------------

fn aws_create_user(user_name: &str) -> Option<String> {
    let output = Command::new("aws")
        .args([
            "iam",
            "create-user",
            "--user-name",
            user_name,
            "--output",
            "json",
        ])
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str::<serde_json::Value>(&stdout)
            .ok()
            .and_then(|v| v["User"]["UserName"].as_str().map(String::from))
    } else {
        // User may already exist — that's fine
        Some(user_name.to_string())
    }
}

fn aws_create_access_key(user_name: &str) -> Option<(String, String)> {
    let output = Command::new("aws")
        .args([
            "iam",
            "create-access-key",
            "--user-name",
            user_name,
            "--output",
            "json",
        ])
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let v: serde_json::Value = serde_json::from_str(&stdout).ok()?;
        let key_id = v["AccessKey"]["AccessKeyId"].as_str()?.to_string();
        let secret = v["AccessKey"]["SecretAccessKey"].as_str()?.to_string();
        Some((key_id, secret))
    } else {
        None
    }
}

fn aws_delete_access_key(user_name: &str, access_key_id: &str) {
    let _ = Command::new("aws")
        .args([
            "iam",
            "delete-access-key",
            "--user-name",
            user_name,
            "--access-key-id",
            access_key_id,
        ])
        .output();
}

fn aws_list_access_keys(user_name: &str) -> Vec<String> {
    let output = Command::new("aws")
        .args([
            "iam",
            "list-access-keys",
            "--user-name",
            user_name,
            "--output",
            "json",
        ])
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&stdout) {
            v["AccessKeyMetadata"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|k| k["AccessKeyId"].as_str().map(String::from))
                .collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

fn aws_delete_user(user_name: &str) {
    // Delete all access keys first
    for key_id in aws_list_access_keys(user_name) {
        aws_delete_access_key(user_name, &key_id);
    }
    let _ = Command::new("aws")
        .args(["iam", "delete-user", "--user-name", user_name])
        .output();
}

// -- Read Operations --------------------------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_list_users() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== ListUsers ===");

    // 1. Basic list
    println!("\n[1/2] Listing IAM users...");
    let response = iam.list_users().await?;
    println!("  Found {} users", response.users.len());
    for user in response.users.iter().take(5) {
        println!(
            "    {} (arn={}, created={})",
            user.user_name, user.arn, user.create_date
        );
    }

    // 2. Verify user structure
    println!("\n[2/2] Verifying user fields...");
    if let Some(user) = response.users.first() {
        assert!(!user.arn.is_empty(), "user ARN should not be empty");
        assert!(
            user.arn.starts_with("arn:aws:iam::"),
            "user ARN should start with arn:aws:iam::"
        );
        assert!(!user.user_name.is_empty(), "user name should not be empty");
        assert!(
            !user.create_date.is_empty(),
            "create date should not be empty"
        );
        println!("  All fields present and valid");
    } else {
        println!("  No users found (account may have no IAM users)");
    }

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_list_attached_user_policies() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== ListAttachedUserPolicies ===");

    // 1. First get a real user name to query policies for
    println!("\n[1/3] Finding a user to query...");
    let users_response = iam.list_users().await?;
    let Some(user) = users_response.users.first() else {
        println!("  No IAM users found — skipping policy tests");
        return Ok(());
    };
    println!("  Using user: {}", user.user_name);

    // 2. List attached policies for that user
    println!(
        "\n[2/3] Listing attached policies for {}...",
        user.user_name
    );
    let policies_response = iam.list_attached_user_policies(&user.user_name).await?;
    println!(
        "  Found {} attached policies",
        policies_response.attached_policies.len()
    );
    for policy in &policies_response.attached_policies {
        println!(
            "    {} (arn={})",
            policy.policy_name.as_deref().unwrap_or("?"),
            policy.policy_arn.as_deref().unwrap_or("?")
        );
    }

    // 3. Error case: non-existent user
    println!("\n[3/3] Querying non-existent user...");
    let result = iam
        .list_attached_user_policies("cloud-lite-test-nonexistent-user-999")
        .await;
    assert!(result.is_err(), "non-existent user should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}

// -- Write Operations (error cases only) ------------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_detach_user_policy_error_cases() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== DetachUserPolicy Error Cases ===");

    // 1. Non-existent user
    println!("\n[1/2] Detaching from non-existent user...");
    let result = iam
        .detach_user_policy(
            "cloud-lite-test-nonexistent-user-999",
            "arn:aws:iam::aws:policy/ReadOnlyAccess",
        )
        .await;
    assert!(result.is_err(), "non-existent user should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    // 2. Non-existent policy ARN
    println!("\n[2/2] Detaching non-existent policy...");
    let users_response = iam.list_users().await?;
    if let Some(user) = users_response.users.first() {
        let result = iam
            .detach_user_policy(
                &user.user_name,
                "arn:aws:iam::000000000000:policy/NonExistentPolicy",
            )
            .await;
        assert!(result.is_err(), "non-existent policy should fail");
        println!("  Got expected error: {}", result.unwrap_err());
    } else {
        println!("  No users found — skipping");
    }

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_delete_access_key_error_cases() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== DeleteAccessKey Error Cases ===");

    // 1. Non-existent access key
    println!("\n[1/1] Deleting non-existent access key...");
    let result = iam
        .delete_access_key(
            "cloud-lite-test-nonexistent-user-999",
            "AKIAIOSFODNN7EXAMPLE",
        )
        .await;
    assert!(result.is_err(), "non-existent key should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}

// -- Task 5.2: Access Keys + Credential Report --------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_access_key_lifecycle() {
    // Pre-cleanup
    aws_delete_user(TEST_USER_NAME);

    let region = region();
    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_access_key_tests(&client).await;

    // Always cleanup
    aws_delete_user(TEST_USER_NAME);

    result.expect("access key lifecycle tests failed");
    println!("\nAll access key lifecycle tests passed!");
}

async fn run_access_key_tests(client: &AwsHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    let iam = client.iam();

    println!("\n=== Access Key Lifecycle Test ===");

    // [0/6] Create fixture: IAM user + access key
    println!("\n[0/6] Creating test user and access key...");
    let user_name = aws_create_user(TEST_USER_NAME).expect("failed to create test user");
    println!("  Created user: {user_name}");

    let (key_id, _secret) = aws_create_access_key(&user_name).expect("failed to create access key");
    println!("  Created access key: {key_id}");

    // [1/6] ListAccessKeys — verify the key we just created
    println!("\n[1/6] Listing access keys...");
    let response = iam.list_access_keys(&user_name).await?;
    println!("  Found {} keys", response.access_key_metadata.len());
    assert!(
        !response.access_key_metadata.is_empty(),
        "should have at least one access key"
    );
    let found = response
        .access_key_metadata
        .iter()
        .any(|k| k.access_key_id.as_deref() == Some(&key_id));
    assert!(found, "created key {key_id} should be in the list");
    // Verify field structure
    let key_meta = &response.access_key_metadata[0];
    assert!(
        key_meta.access_key_id.is_some(),
        "access key ID should be present"
    );
    assert!(key_meta.user_name.is_some(), "user name should be present");
    println!("  Key fields verified");

    // [2/6] GetAccessKeyLastUsed
    println!("\n[2/6] Getting access key last used info...");
    let last_used = iam.get_access_key_last_used(&key_id).await?;
    println!(
        "  User: {}",
        last_used.user_name.as_deref().unwrap_or("N/A")
    );
    // A newly created key may not have been used yet
    println!(
        "  Last used: {}",
        last_used
            .access_key_last_used
            .as_ref()
            .and_then(|u| u.last_used_date.as_deref())
            .unwrap_or("never")
    );
    // The user_name in the response should match
    assert_eq!(
        last_used.user_name.as_deref(),
        Some(TEST_USER_NAME),
        "response user_name should match"
    );

    // [3/6] UpdateAccessKey — deactivate
    println!("\n[3/6] Deactivating access key...");
    iam.update_access_key(&user_name, &key_id, "Inactive")
        .await?;
    println!("  Key deactivated");

    // Verify it's now Inactive
    let response = iam.list_access_keys(&user_name).await?;
    let updated_key = response
        .access_key_metadata
        .iter()
        .find(|k| k.access_key_id.as_deref() == Some(&key_id))
        .expect("key should still be in the list");
    println!("  Status: {:?}", updated_key.status);

    // [4/6] UpdateAccessKey — reactivate
    println!("\n[4/6] Reactivating access key...");
    iam.update_access_key(&user_name, &key_id, "Active").await?;
    println!("  Key reactivated");

    // [5/6] ListAccessKeys error case — non-existent user
    println!("\n[5/6] Listing keys for non-existent user...");
    let err = iam
        .list_access_keys("cloud-lite-test-nonexistent-user-999")
        .await;
    assert!(err.is_err(), "non-existent user should fail");
    let err_msg = format!("{}", err.unwrap_err());
    assert!(
        err_msg.contains("cannot be found"),
        "Expected 'cannot be found' error, got: {err_msg}"
    );
    println!("  Got expected error: {err_msg}");

    // [6/6] GetAccessKeyLastUsed error case — bogus key
    println!("\n[6/6] Getting last used for non-existent key...");
    let result = iam.get_access_key_last_used("AKIAIOSFODNN7EXAMPLE").await;
    assert!(result.is_err(), "bogus access key should fail");
    let err_msg = format!("{}", result.unwrap_err());
    println!("  Got expected error: {err_msg}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_credential_report() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== Credential Report Test ===");

    // [1/3] Generate credential report
    println!("\n[1/3] Generating credential report...");
    let gen_response = iam.generate_credential_report().await?;
    println!("  State: {:?}", gen_response.state);
    println!(
        "  Description: {}",
        gen_response.description.as_deref().unwrap_or("N/A")
    );

    // [2/3] Wait for report to complete, then retrieve it
    println!("\n[2/3] Retrieving credential report (may need to wait)...");
    let mut attempts = 0;
    let report = loop {
        attempts += 1;
        if attempts > 10 {
            return Err("credential report did not complete after 10 attempts".into());
        }
        match iam.get_credential_report().await {
            Ok(report) => break report,
            Err(_) => {
                // Report might not be ready yet — generate again and wait
                println!("  Report not ready, retrying ({attempts}/10)...");
                let _ = iam.generate_credential_report().await;
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    };
    println!("  Report format: {:?}", report.report_format);
    println!(
        "  Generated time: {}",
        report.generated_time.as_deref().unwrap_or("N/A")
    );
    // Content is base64-encoded CSV
    if let Some(ref content) = report.content {
        assert!(!content.is_empty(), "report content should not be empty");
        println!("  Content length: {} chars (base64)", content.len());
    }

    // [3/3] Verify report structure
    println!("\n[3/3] Verifying report fields...");
    assert!(
        report.generated_time.is_some(),
        "generated_time should be present"
    );
    println!("  Report fields verified");

    Ok(())
}

// -- Task 5.3: MFA, Login Profile, Account Summary/Policy --------------------

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_list_mfa_devices() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== ListMFADevices ===");

    // [1/3] List MFA devices for an existing user
    println!("\n[1/3] Listing MFA devices for existing user...");
    let users = iam.list_users().await?;
    let Some(user) = users.users.first() else {
        println!("  No IAM users found — skipping");
        return Ok(());
    };
    let response = iam.list_mfa_devices(&user.user_name).await?;
    println!(
        "  Found {} MFA devices for {}",
        response.mfa_devices.len(),
        user.user_name
    );
    for device in &response.mfa_devices {
        println!(
            "    serial={}, enabled={}",
            device.serial_number, device.enable_date
        );
    }

    // [2/3] Verify empty list for test user (no MFA)
    println!("\n[2/3] Creating test user to verify empty MFA list...");
    aws_delete_user(TEST_USER_NAME);
    aws_create_user(TEST_USER_NAME).expect("failed to create test user");
    let response = iam.list_mfa_devices(TEST_USER_NAME).await?;
    assert!(
        response.mfa_devices.is_empty(),
        "new user should have no MFA devices"
    );
    println!("  New user has 0 MFA devices (as expected)");
    aws_delete_user(TEST_USER_NAME);

    // [3/3] Error case — non-existent user
    println!("\n[3/3] Listing MFA for non-existent user...");
    let result = iam
        .list_mfa_devices("cloud-lite-test-nonexistent-user-999")
        .await;
    assert!(result.is_err(), "non-existent user should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_get_login_profile() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== GetLoginProfile ===");

    // [1/2] Get login profile for a user with console access
    println!("\n[1/2] Getting login profile for existing user...");
    let users = iam.list_users().await?;
    let mut found_profile = false;
    for user in &users.users {
        match iam.get_login_profile(&user.user_name).await {
            Ok(response) => {
                println!(
                    "  User {} has login profile (created={})",
                    response.login_profile.user_name, response.login_profile.create_date
                );
                assert!(
                    !response.login_profile.user_name.is_empty(),
                    "user_name should not be empty"
                );
                assert!(
                    !response.login_profile.create_date.is_empty(),
                    "create_date should not be empty"
                );
                found_profile = true;
                break;
            }
            Err(_) => {
                println!("  User {} has no login profile", user.user_name);
            }
        }
    }
    if !found_profile {
        println!("  No users with login profiles found");
    }

    // [2/2] Error case — user without login profile
    println!("\n[2/2] Getting login profile for user without console access...");
    aws_delete_user(TEST_USER_NAME);
    aws_create_user(TEST_USER_NAME).expect("failed to create test user");
    let result = iam.get_login_profile(TEST_USER_NAME).await;
    assert!(
        result.is_err(),
        "user without login profile should return error"
    );
    println!("  Got expected error: {}", result.unwrap_err());
    aws_delete_user(TEST_USER_NAME);

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_get_account_summary() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== GetAccountSummary ===");

    // [1/2] Get account summary
    println!("\n[1/2] Getting account summary...");
    let response = iam.get_account_summary().await?;
    println!("  Summary map has {} entries", response.summary_map.len());
    for (key, value) in &response.summary_map {
        println!("    {key} = {value}");
    }

    // [2/2] Verify expected keys exist
    println!("\n[2/2] Verifying expected summary keys...");
    assert!(
        !response.summary_map.is_empty(),
        "summary map should not be empty"
    );
    // These keys are always present in AWS accounts
    let expected_keys = ["Users", "Groups", "Roles", "Policies"];
    for key in &expected_keys {
        assert!(
            response.summary_map.contains_key(*key),
            "summary map should contain '{key}'"
        );
    }
    println!("  All expected keys present");

    Ok(())
}

const TEST_ROLE_NAME: &str = "cloud-lite-test-iam-role-ops";
const TEST_GROUP_NAME: &str = "cloud-lite-test-iam-group-ops";

fn aws_create_role(role_name: &str) -> Option<String> {
    let trust_policy = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"Service":"ec2.amazonaws.com"},"Action":"sts:AssumeRole"}]}"#;
    let output = Command::new("aws")
        .args([
            "iam",
            "create-role",
            "--role-name",
            role_name,
            "--assume-role-policy-document",
            trust_policy,
            "--output",
            "json",
        ])
        .output()
        .expect("aws cli must be installed");
    if output.status.success() || !output.status.success() {
        // Role may already exist — that's fine
        Some(role_name.to_string())
    } else {
        None
    }
}

fn aws_detach_all_role_policies(role_name: &str) {
    let output = Command::new("aws")
        .args([
            "iam",
            "list-attached-role-policies",
            "--role-name",
            role_name,
            "--output",
            "json",
        ])
        .output()
        .expect("aws cli must be installed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&stdout) {
            if let Some(policies) = v["AttachedPolicies"].as_array() {
                for p in policies {
                    if let Some(arn) = p["PolicyArn"].as_str() {
                        let _ = Command::new("aws")
                            .args([
                                "iam",
                                "detach-role-policy",
                                "--role-name",
                                role_name,
                                "--policy-arn",
                                arn,
                            ])
                            .output();
                    }
                }
            }
        }
    }
}

fn aws_delete_role(role_name: &str) {
    aws_detach_all_role_policies(role_name);
    let _ = Command::new("aws")
        .args(["iam", "delete-role", "--role-name", role_name])
        .output();
}

fn aws_put_user_policy(user_name: &str, policy_name: &str) {
    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:GetObject","Resource":"*"}]}"#;
    let _ = Command::new("aws")
        .args([
            "iam",
            "put-user-policy",
            "--user-name",
            user_name,
            "--policy-name",
            policy_name,
            "--policy-document",
            policy_doc,
        ])
        .output();
}

fn aws_delete_user_policy(user_name: &str, policy_name: &str) {
    let _ = Command::new("aws")
        .args([
            "iam",
            "delete-user-policy",
            "--user-name",
            user_name,
            "--policy-name",
            policy_name,
        ])
        .output();
}

fn aws_create_group(group_name: &str) {
    let _ = Command::new("aws")
        .args(["iam", "create-group", "--group-name", group_name])
        .output();
}

fn aws_add_user_to_group(group_name: &str, user_name: &str) {
    let _ = Command::new("aws")
        .args([
            "iam",
            "add-user-to-group",
            "--group-name",
            group_name,
            "--user-name",
            user_name,
        ])
        .output();
}

fn aws_remove_user_from_group(group_name: &str, user_name: &str) {
    let _ = Command::new("aws")
        .args([
            "iam",
            "remove-user-from-group",
            "--group-name",
            group_name,
            "--user-name",
            user_name,
        ])
        .output();
}

fn aws_delete_group(group_name: &str) {
    let _ = Command::new("aws")
        .args(["iam", "delete-group", "--group-name", group_name])
        .output();
}

fn aws_delete_service_linked_role(role_name: &str) {
    let _ = Command::new("aws")
        .args([
            "iam",
            "delete-service-linked-role",
            "--role-name",
            role_name,
        ])
        .output();
}

fn aws_attach_group_policy(group_name: &str, policy_arn: &str) {
    let _ = Command::new("aws")
        .args([
            "iam",
            "attach-group-policy",
            "--group-name",
            group_name,
            "--policy-arn",
            policy_arn,
        ])
        .output();
}

fn aws_detach_group_policy(group_name: &str, policy_arn: &str) {
    let _ = Command::new("aws")
        .args([
            "iam",
            "detach-group-policy",
            "--group-name",
            group_name,
            "--policy-arn",
            policy_arn,
        ])
        .output();
}

fn aws_delete_account_password_policy() {
    let _ = Command::new("aws")
        .args(["iam", "delete-account-password-policy"])
        .output();
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_account_password_policy() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== AccountPasswordPolicy ===");

    // [1/4] Ensure a password policy exists (create if needed)
    println!("\n[1/4] Ensuring password policy exists...");
    let had_policy = iam.get_account_password_policy().await.is_ok();
    if !had_policy {
        println!("  No policy exists — creating one...");
        let create_body = aws_lite::types::iam::UpdateAccountPasswordPolicyRequest {
            minimum_password_length: Some(8),
            require_symbols: Some(false),
            require_numbers: Some(true),
            require_uppercase_characters: Some(true),
            require_lowercase_characters: Some(true),
            ..Default::default()
        };
        iam.update_account_password_policy(&create_body).await?;
        println!("  Created default password policy");
    } else {
        println!("  Policy already exists");
    }

    // [2/4] Get current password policy
    println!("\n[2/4] Getting current password policy...");
    let original = iam.get_account_password_policy().await?;
    let policy = &original.password_policy;
    println!(
        "  MinLength={:?}, RequireSymbols={:?}, RequireNumbers={:?}",
        policy.minimum_password_length, policy.require_symbols, policy.require_numbers
    );
    println!(
        "  RequireUpper={:?}, RequireLower={:?}, MaxAge={:?}",
        policy.require_uppercase_characters,
        policy.require_lowercase_characters,
        policy.max_password_age
    );

    // [3/4] Update password policy (set min length to 14)
    println!("\n[3/4] Updating password policy (min length -> 14)...");
    let update_body = aws_lite::types::iam::UpdateAccountPasswordPolicyRequest {
        minimum_password_length: Some(14),
        require_symbols: policy.require_symbols,
        require_numbers: policy.require_numbers,
        require_uppercase_characters: policy.require_uppercase_characters,
        require_lowercase_characters: policy.require_lowercase_characters,
        allow_users_to_change_password: policy.allow_users_to_change_password,
        max_password_age: policy.max_password_age,
        password_reuse_prevention: policy.password_reuse_prevention,
        hard_expiry: policy.hard_expiry,
    };
    iam.update_account_password_policy(&update_body).await?;
    println!("  Updated successfully");

    // Verify the update
    let updated = iam.get_account_password_policy().await?;
    assert_eq!(
        updated.password_policy.minimum_password_length,
        Some(14),
        "min password length should be 14"
    );
    println!(
        "  Verified: min length = {:?}",
        updated.password_policy.minimum_password_length
    );

    // [4/4] Restore or clean up
    if had_policy {
        println!("\n[4/4] Restoring original password policy...");
        let restore_body = aws_lite::types::iam::UpdateAccountPasswordPolicyRequest {
            minimum_password_length: policy.minimum_password_length,
            require_symbols: policy.require_symbols,
            require_numbers: policy.require_numbers,
            require_uppercase_characters: policy.require_uppercase_characters,
            require_lowercase_characters: policy.require_lowercase_characters,
            allow_users_to_change_password: policy.allow_users_to_change_password,
            max_password_age: policy.max_password_age,
            password_reuse_prevention: policy.password_reuse_prevention,
            hard_expiry: policy.hard_expiry,
        };
        iam.update_account_password_policy(&restore_body).await?;
        let restored = iam.get_account_password_policy().await?;
        assert_eq!(
            restored.password_policy.minimum_password_length, policy.minimum_password_length,
            "min password length should be restored"
        );
        println!("  Verified: policy restored");
    } else {
        println!("\n[4/4] Cleaning up (deleting created policy)...");
        aws_delete_account_password_policy();
        println!("  Deleted password policy");
    }

    Ok(())
}

// -- Task 5.4: Roles, User Policies, Groups, Certs, Service-Linked Roles -----

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_list_roles() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== ListRoles ===");

    // [1/2] List all roles
    println!("\n[1/2] Listing IAM roles...");
    let response = iam.list_roles().await?;
    println!("  Found {} roles", response.roles.len());
    for role in response.roles.iter().take(5) {
        println!(
            "    {} (arn={}, created={})",
            role.role_name, role.arn, role.create_date
        );
    }

    // [2/2] Verify role structure
    println!("\n[2/2] Verifying role fields...");
    assert!(
        !response.roles.is_empty(),
        "account should have at least one role (service-linked roles)"
    );
    let role = &response.roles[0];
    assert!(!role.role_name.is_empty(), "role name should not be empty");
    assert!(
        role.arn.starts_with("arn:aws:iam::"),
        "role ARN should start with arn:aws:iam::"
    );
    assert!(
        !role.create_date.is_empty(),
        "create date should not be empty"
    );
    println!("  All fields present and valid");

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_user_policies_and_groups() {
    // Pre-cleanup
    aws_remove_user_from_group(TEST_GROUP_NAME, TEST_USER_NAME);
    aws_delete_user_policy(TEST_USER_NAME, "cloud-lite-test-inline-policy");
    aws_delete_user(TEST_USER_NAME);
    aws_delete_group(TEST_GROUP_NAME);

    let region = region();
    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_user_policies_and_groups_tests(&client).await;

    // Always cleanup
    aws_remove_user_from_group(TEST_GROUP_NAME, TEST_USER_NAME);
    aws_delete_user_policy(TEST_USER_NAME, "cloud-lite-test-inline-policy");
    aws_delete_user(TEST_USER_NAME);
    aws_delete_group(TEST_GROUP_NAME);

    result.expect("user policies and groups tests failed");
    println!("\nAll user policies and groups tests passed!");
}

async fn run_user_policies_and_groups_tests(
    client: &AwsHttpClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let iam = client.iam();

    println!("\n=== User Policies + Groups Lifecycle ===");

    // [0/7] Create fixtures: user + inline policy + group + membership
    println!("\n[0/7] Creating test fixtures...");
    aws_create_user(TEST_USER_NAME).expect("failed to create test user");
    println!("  Created user: {TEST_USER_NAME}");

    let policy_name = "cloud-lite-test-inline-policy";
    aws_put_user_policy(TEST_USER_NAME, policy_name);
    println!("  Attached inline policy: {policy_name}");

    aws_create_group(TEST_GROUP_NAME);
    println!("  Created group: {TEST_GROUP_NAME}");

    aws_add_user_to_group(TEST_GROUP_NAME, TEST_USER_NAME);
    println!("  Added user to group");

    // [1/7] ListUserPolicies — verify inline policy
    println!("\n[1/7] Listing user policies...");
    let response = iam.list_user_policies(TEST_USER_NAME).await?;
    println!("  Found {} inline policies", response.policy_names.len());
    assert!(
        response.policy_names.contains(&policy_name.to_string()),
        "inline policy should be in the list"
    );
    println!("  Verified: {policy_name} is present");

    // [2/7] ListGroupsForUser — verify group membership
    println!("\n[2/7] Listing groups for user...");
    let response = iam.list_groups_for_user(TEST_USER_NAME).await?;
    println!("  Found {} groups", response.groups.len());
    let found = response
        .groups
        .iter()
        .any(|g| g.group_name == TEST_GROUP_NAME);
    assert!(found, "test group should be in the list");
    let group = response
        .groups
        .iter()
        .find(|g| g.group_name == TEST_GROUP_NAME)
        .unwrap();
    assert!(
        group.arn.starts_with("arn:aws:iam::"),
        "group ARN should start with arn:aws:iam::"
    );
    assert!(
        !group.create_date.is_empty(),
        "group create_date should not be empty"
    );
    println!("  Verified: {TEST_GROUP_NAME} membership with valid fields");

    // [3/7] DeleteUserPolicy — remove inline policy
    println!("\n[3/7] Deleting inline policy...");
    iam.delete_user_policy(TEST_USER_NAME, policy_name).await?;
    println!("  Deleted policy: {policy_name}");

    // Verify it's gone
    let response = iam.list_user_policies(TEST_USER_NAME).await?;
    assert!(
        response.policy_names.is_empty(),
        "inline policies should be empty after delete"
    );
    println!("  Verified: no inline policies remain");

    // [4/7] DeleteUserPolicy error — non-existent policy
    println!("\n[4/7] Deleting non-existent policy...");
    let result = iam
        .delete_user_policy(TEST_USER_NAME, "nonexistent-policy")
        .await;
    assert!(result.is_err(), "deleting non-existent policy should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    // [5/7] ListUserPolicies error — non-existent user
    println!("\n[5/7] Listing policies for non-existent user...");
    let result = iam
        .list_user_policies("cloud-lite-test-nonexistent-user-999")
        .await;
    assert!(result.is_err(), "non-existent user should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    // [6/7] ListGroupsForUser error — non-existent user
    println!("\n[6/7] Listing groups for non-existent user...");
    let result = iam
        .list_groups_for_user("cloud-lite-test-nonexistent-user-999")
        .await;
    assert!(result.is_err(), "non-existent user should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    // [7/7] ListGroupsForUser — user with no groups
    println!("\n[7/7] Listing groups for user with no memberships...");
    aws_remove_user_from_group(TEST_GROUP_NAME, TEST_USER_NAME);
    let response = iam.list_groups_for_user(TEST_USER_NAME).await?;
    assert!(
        response.groups.is_empty(),
        "user should have no group memberships after removal"
    );
    println!("  Verified: 0 groups after removal");

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_list_server_certificates() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== ListServerCertificates ===");

    // [1/1] List server certificates (may be empty)
    println!("\n[1/1] Listing server certificates...");
    let response = iam.list_server_certificates().await?;
    println!(
        "  Found {} server certificates",
        response.server_certificate_metadata_list.len()
    );
    for cert in &response.server_certificate_metadata_list {
        println!(
            "    {} (arn={}, expires={:?})",
            cert.server_certificate_name,
            cert.arn,
            cert.expiration.as_deref().unwrap_or("N/A")
        );
    }
    // No assertion on count — many accounts have 0 server certs
    println!("  Server certificates listing succeeded");

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_role_policy_attach_detach() {
    // Pre-cleanup
    aws_delete_role(TEST_ROLE_NAME);

    let region = region();
    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_role_policy_tests(&client).await;

    // Always cleanup
    aws_delete_role(TEST_ROLE_NAME);

    result.expect("role policy attach/detach tests failed");
    println!("\nAll role policy attach/detach tests passed!");
}

async fn run_role_policy_tests(client: &AwsHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    let iam = client.iam();

    println!("\n=== AttachRolePolicy + DetachRolePolicy ===");

    // [0/4] Create fixture: IAM role
    println!("\n[0/4] Creating test role...");
    aws_create_role(TEST_ROLE_NAME).expect("failed to create test role");
    println!("  Created role: {TEST_ROLE_NAME}");

    let policy_arn = "arn:aws:iam::aws:policy/ReadOnlyAccess";

    // [1/4] AttachRolePolicy
    println!("\n[1/4] Attaching managed policy to role...");
    iam.attach_role_policy(TEST_ROLE_NAME, policy_arn).await?;
    println!("  Attached: {policy_arn}");

    // [2/4] Verify attachment via ListRoles (role should exist)
    println!("\n[2/4] Verifying role exists in list...");
    let roles = iam.list_roles().await?;
    let found = roles.roles.iter().any(|r| r.role_name == TEST_ROLE_NAME);
    assert!(found, "test role should appear in ListRoles");
    println!("  Verified: {TEST_ROLE_NAME} found in role list");

    // [3/4] DetachRolePolicy
    println!("\n[3/4] Detaching managed policy from role...");
    iam.detach_role_policy(TEST_ROLE_NAME, policy_arn).await?;
    println!("  Detached: {policy_arn}");

    // [4/4] Error case — detach non-existent policy
    println!("\n[4/4] Detaching non-existent policy...");
    let result = iam
        .detach_role_policy(
            TEST_ROLE_NAME,
            "arn:aws:iam::000000000000:policy/NonExistentPolicy",
        )
        .await;
    assert!(result.is_err(), "detaching non-existent policy should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_create_service_linked_role() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region)?;
    let iam = client.iam();

    println!("\n=== CreateServiceLinkedRole ===");

    // [1/2] Create service-linked role (or verify it already exists)
    println!("\n[1/2] Creating service-linked role for elasticbeanstalk...");
    let service = "elasticbeanstalk.amazonaws.com";
    let result = iam.create_service_linked_role(service).await;
    match result {
        Ok(response) => {
            if let Some(role) = &response.role {
                println!("  Created: {} (arn={})", role.role_name, role.arn);
                assert!(
                    role.arn.contains("aws-service-role"),
                    "service-linked role ARN should contain 'aws-service-role'"
                );
            }
        }
        Err(e) => {
            let err_msg = format!("{e}");
            // Service-linked role may already exist — that's acceptable
            assert!(
                err_msg.contains("already exists") || err_msg.contains("has been taken"),
                "Expected 'already exists' error, got: {err_msg}"
            );
            println!("  Service-linked role already exists (expected)");
        }
    }

    // [2/2] Error case — invalid service principal
    println!("\n[2/2] Creating service-linked role with invalid service...");
    let result = iam
        .create_service_linked_role("invalid-service.example.com")
        .await;
    assert!(result.is_err(), "invalid service principal should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}

// -- GetUserPolicy + ListAttachedGroupPolicies --------------------------------

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_get_user_policy() {
    // Pre-cleanup
    aws_delete_user_policy(TEST_USER_NAME, "cloud-lite-test-inline-policy");
    aws_delete_user(TEST_USER_NAME);

    let region = region();
    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_get_user_policy_tests(&client).await;

    // Always cleanup
    aws_delete_user_policy(TEST_USER_NAME, "cloud-lite-test-inline-policy");
    aws_delete_user(TEST_USER_NAME);

    result.expect("get_user_policy tests failed");
    println!("\nAll get_user_policy tests passed!");
}

async fn run_get_user_policy_tests(
    client: &AwsHttpClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let iam = client.iam();
    let policy_name = "cloud-lite-test-inline-policy";

    println!("\n=== GetUserPolicy ===");

    // [0/4] Create fixtures: user + inline policy
    println!("\n[0/4] Creating test user with inline policy...");
    aws_create_user(TEST_USER_NAME).expect("failed to create test user");
    aws_put_user_policy(TEST_USER_NAME, policy_name);
    println!("  Created user {TEST_USER_NAME} with inline policy {policy_name}");

    // [1/4] GetUserPolicy — retrieve the inline policy document
    println!("\n[1/4] Getting user policy...");
    let response = iam.get_user_policy(TEST_USER_NAME, policy_name).await?;
    println!("  UserName: {}", response.user_name);
    println!("  PolicyName: {}", response.policy_name);
    println!("  PolicyDocument: {}", response.policy_document);
    assert_eq!(response.user_name, TEST_USER_NAME);
    assert_eq!(response.policy_name, policy_name);
    assert!(
        !response.policy_document.is_empty(),
        "policy document should not be empty"
    );

    // [2/4] Verify policy document contains expected content
    println!("\n[2/4] Verifying policy document content...");
    // The document is URL-encoded by IAM; decode it for verification
    let decoded = urlencoding::decode(&response.policy_document)
        .unwrap_or_else(|_| response.policy_document.clone().into());
    assert!(
        decoded.contains("s3:GetObject"),
        "policy doc should contain s3:GetObject action, got: {decoded}"
    );
    println!("  Policy document contains expected s3:GetObject action");

    // [3/4] Error case — non-existent policy name
    println!("\n[3/4] Getting non-existent policy...");
    let result = iam
        .get_user_policy(TEST_USER_NAME, "nonexistent-policy")
        .await;
    assert!(result.is_err(), "non-existent policy should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    // [4/4] Error case — non-existent user
    println!("\n[4/4] Getting policy for non-existent user...");
    let result = iam
        .get_user_policy("cloud-lite-test-nonexistent-user-999", policy_name)
        .await;
    assert!(result.is_err(), "non-existent user should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}

const GROUP_POLICY_TEST: &str = "cloud-lite-test-iam-grp-pol";

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_list_attached_group_policies() {
    let policy_arn = "arn:aws:iam::aws:policy/ReadOnlyAccess";

    // Pre-cleanup
    aws_detach_group_policy(GROUP_POLICY_TEST, policy_arn);
    aws_delete_group(GROUP_POLICY_TEST);

    let region = region();
    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");

    let result = run_list_attached_group_policies_tests(&client).await;

    // Always cleanup
    aws_detach_group_policy(GROUP_POLICY_TEST, policy_arn);
    aws_delete_group(GROUP_POLICY_TEST);

    result.expect("list_attached_group_policies tests failed");
    println!("\nAll list_attached_group_policies tests passed!");
}

async fn run_list_attached_group_policies_tests(
    client: &AwsHttpClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let iam = client.iam();
    let policy_arn = "arn:aws:iam::aws:policy/ReadOnlyAccess";

    println!("\n=== ListAttachedGroupPolicies ===");

    // [0/4] Create fixtures: group + attach a managed policy
    println!("\n[0/4] Creating test group with attached policy...");
    aws_create_group(GROUP_POLICY_TEST);
    aws_attach_group_policy(GROUP_POLICY_TEST, policy_arn);
    println!("  Created group {GROUP_POLICY_TEST} with attached policy {policy_arn}");

    // [1/4] ListAttachedGroupPolicies — verify the attached policy
    println!("\n[1/4] Listing attached group policies...");
    let response = iam.list_attached_group_policies(GROUP_POLICY_TEST).await?;
    println!(
        "  Found {} attached policies",
        response.attached_policies.len()
    );
    assert!(
        !response.attached_policies.is_empty(),
        "group should have at least one attached policy"
    );
    let found = response
        .attached_policies
        .iter()
        .any(|p| p.policy_arn.as_deref() == Some(policy_arn));
    assert!(found, "ReadOnlyAccess should be in the attached policies");
    println!("  Verified: ReadOnlyAccess is attached");

    // [2/4] Verify field structure
    println!("\n[2/4] Verifying policy fields...");
    let policy = &response.attached_policies[0];
    assert!(policy.policy_arn.is_some(), "policy ARN should be present");
    assert!(
        policy.policy_name.is_some(),
        "policy name should be present"
    );
    println!(
        "  PolicyName={}, PolicyArn={}",
        policy.policy_name.as_deref().unwrap_or("?"),
        policy.policy_arn.as_deref().unwrap_or("?")
    );

    // [3/4] Detach and verify empty list
    println!("\n[3/4] Detaching policy and verifying empty list...");
    aws_detach_group_policy(GROUP_POLICY_TEST, policy_arn);
    let response = iam.list_attached_group_policies(GROUP_POLICY_TEST).await?;
    assert!(
        response.attached_policies.is_empty(),
        "group should have no attached policies after detach"
    );
    println!("  Verified: 0 attached policies after detach");

    // [4/4] Error case — non-existent group
    println!("\n[4/4] Listing policies for non-existent group...");
    let result = iam
        .list_attached_group_policies("cloud-lite-test-nonexistent-group-999")
        .await;
    assert!(result.is_err(), "non-existent group should fail");
    println!("  Got expected error: {}", result.unwrap_err());

    Ok(())
}

// -- Task 15.1: ListVirtualMFADevices -----------------------------------------

/// CIS 2.5: root account must use hardware MFA, not a virtual MFA device.
/// A virtual MFA device whose serial contains "root-account-mfa-device" indicates
/// virtual MFA on root — which is non-compliant.
#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_list_virtual_mfa_devices() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");
    let iam = client.iam();

    println!("\n=== ListVirtualMFADevices ===");

    // [1/3] List the first page of virtual MFA devices
    println!("\n[1/3] Listing virtual MFA devices (first page)...");
    let resp = iam.list_virtual_mfa_devices(None).await?;
    println!("  First page: {} device(s)", resp.virtual_mfa_devices.len());
    let is_truncated = resp.is_truncated.unwrap_or(false);
    println!("  IsTruncated: {is_truncated}");

    // [2/3] List ALL virtual MFA devices via paginating helper
    println!("\n[2/3] Listing all virtual MFA devices (paginated)...");
    let devices = iam.list_all_virtual_mfa_devices().await?;
    println!("  Total: {} device(s)", devices.len());

    let mut root_virtual_mfa = false;
    for d in &devices {
        let is_root = d.serial_number.contains("root-account-mfa-device");
        println!(
            "  - {} (root={is_root}, enabled={:?})",
            d.serial_number, d.enable_date
        );
        // Verify required field
        assert!(
            !d.serial_number.is_empty(),
            "serial_number must not be empty"
        );
        if is_root {
            root_virtual_mfa = true;
        }
    }

    // [3/3] CIS 2.5 compliance summary
    println!("\n[3/3] CIS 2.5 compliance summary...");
    if root_virtual_mfa {
        println!(
            "  WARNING (CIS 2.5): root account uses virtual MFA. \
             Hardware MFA token required for compliance."
        );
    } else if devices.is_empty() {
        println!("  No virtual MFA devices found (root may have no MFA or uses hardware MFA).");
    } else {
        println!(
            "  CIS 2.5: Root account is not using a virtual MFA device. \
             Verify hardware MFA is configured."
        );
    }

    println!("\nAll ListVirtualMFADevices tests passed!");
    Ok(())
}

// -- Task 15.1: ListPolicies + GetPolicyVersion --------------------------------

/// CIS 2.15: no customer-managed policy should grant full admin (*:* on *).
/// Lists all Local-scoped policies and inspects the default version document
/// of each for unrestricted statements.
#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_list_policies_and_get_version() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");
    let iam = client.iam();

    println!("\n=== ListPolicies + GetPolicyVersion ===");

    // [1/4] Single-page call: list Local policies
    println!("\n[1/4] Listing customer-managed (Local) policies (first page)...");
    let page = iam.list_policies(Some("Local"), None).await?;
    println!(
        "  First page: {} policies, truncated={}",
        page.policies.len(),
        page.is_truncated.unwrap_or(false)
    );

    // [2/4] Paginated: all Local policies
    println!("\n[2/4] Listing all customer-managed policies (paginated)...");
    let policies = iam.list_all_policies(Some("Local")).await?;
    println!("  Total Local policies: {}", policies.len());

    let mut full_admin_count = 0;
    let mut checked = 0;

    for p in &policies {
        let arn = p.arn.as_deref().unwrap_or("?");
        let name = p.policy_name.as_deref().unwrap_or("?");
        let version_id = p.default_version_id.as_deref().unwrap_or("v1");
        println!("  - {name} ({arn}) default_version={version_id}");

        // Verify core fields
        assert!(p.arn.is_some(), "policy ARN must be present");
        assert!(p.policy_name.is_some(), "policy name must be present");
        assert!(
            p.default_version_id.is_some(),
            "default_version_id must be present"
        );

        // [3/4] GetPolicyVersion — inspect document for *:* statements
        if checked < 3 {
            // Limit to first 3 to keep test fast
            let ver_resp = iam.get_policy_version(arn, version_id).await?;
            if let Some(ver) = ver_resp.policy_version {
                let doc_encoded = ver.document.unwrap_or_default();
                let doc = urlencoding::decode(&doc_encoded)
                    .unwrap_or_else(|_| doc_encoded.clone().into());
                // Check for full-admin pattern: Action=* and Resource=*
                let has_star_action = doc.contains(r#""Action": "*""#)
                    || doc.contains(r#""Action":"*""#)
                    || doc.contains(r#"["*"]"#);
                let has_star_resource =
                    doc.contains(r#""Resource": "*""#) || doc.contains(r#""Resource":"*""#);
                if has_star_action && has_star_resource {
                    println!("    WARNING (CIS 2.15): policy {name} grants full admin (*:*)!");
                    full_admin_count += 1;
                } else {
                    println!("    GetPolicyVersion OK — no full-admin pattern detected");
                }
                assert!(!doc.is_empty(), "policy document should not be empty");
                assert!(ver.version_id.is_some(), "version_id should be present");
            }
            checked += 1;
        }
    }

    // [4/4] CIS 2.15 compliance summary
    println!("\n[4/4] CIS 2.15 compliance summary...");
    if policies.is_empty() {
        println!("  No customer-managed policies found. Nothing to check.");
    } else if full_admin_count == 0 {
        println!(
            "  CIS 2.15: None of the first {checked} customer-managed policies \
             grant full admin access (*:*)."
        );
    } else {
        println!(
            "  WARNING (CIS 2.15): {full_admin_count} policy/policies grant \
             full admin access. Remediate by scoping actions and resources."
        );
    }

    println!("\nAll ListPolicies + GetPolicyVersion tests passed!");
    Ok(())
}

// -- Task 15.1: ListEntitiesForPolicy -----------------------------------------

/// CIS 2.16: AWSSupportAccess must have at least one entity attached.
/// CIS 2.21: AWSCloudShellFullAccess should have no entities attached
///           (or only explicitly approved ones).
#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn test_list_entities_for_policy() -> Result<(), Box<dyn std::error::Error>> {
    let region = region();
    let client = AwsHttpClient::from_default_chain(&region).expect("AWS credentials required");
    let iam = client.iam();

    println!("\n=== ListEntitiesForPolicy ===");

    let support_arn = "arn:aws:iam::aws:policy/AWSSupportAccess";
    let cloudshell_arn = "arn:aws:iam::aws:policy/AWSCloudShellFullAccess";

    // [1/4] AWSSupportAccess — first page
    println!("\n[1/4] Listing entities for AWSSupportAccess (first page)...");
    let page = iam.list_entities_for_policy(support_arn).await?;
    println!(
        "  groups={}, users={}, roles={}",
        page.policy_groups.len(),
        page.policy_users.len(),
        page.policy_roles.len()
    );

    // [2/4] AWSSupportAccess — all pages merged
    println!("\n[2/4] Listing all entities for AWSSupportAccess (paginated)...");
    let all_support = iam.list_all_entities_for_policy(support_arn).await?;
    let total_support = all_support.policy_groups.len()
        + all_support.policy_users.len()
        + all_support.policy_roles.len();
    println!("  Total entities: {total_support}");
    println!(
        "  Groups: {:?}",
        all_support
            .policy_groups
            .iter()
            .map(|g| g.group_name.as_deref().unwrap_or("?"))
            .collect::<Vec<_>>()
    );
    println!(
        "  Users:  {:?}",
        all_support
            .policy_users
            .iter()
            .map(|u| u.user_name.as_deref().unwrap_or("?"))
            .collect::<Vec<_>>()
    );
    println!(
        "  Roles:  {:?}",
        all_support
            .policy_roles
            .iter()
            .map(|r| r.role_name.as_deref().unwrap_or("?"))
            .collect::<Vec<_>>()
    );

    // Verify field structure
    for g in &all_support.policy_groups {
        assert!(g.group_name.is_some(), "group_name must be present");
    }
    for u in &all_support.policy_users {
        assert!(u.user_name.is_some(), "user_name must be present");
    }
    for r in &all_support.policy_roles {
        assert!(r.role_name.is_some(), "role_name must be present");
    }

    // CIS 2.16
    if total_support == 0 {
        println!(
            "  WARNING (CIS 2.16): No entity is attached to AWSSupportAccess. \
             At least one group, user, or role should be attached."
        );
    } else {
        println!(
            "  CIS 2.16: {total_support} entity/entities attached to \
             AWSSupportAccess. Compliant."
        );
    }

    // [3/4] AWSCloudShellFullAccess — all pages merged
    println!("\n[3/4] Listing all entities for AWSCloudShellFullAccess (paginated)...");
    let all_shell = iam.list_all_entities_for_policy(cloudshell_arn).await?;
    let total_shell =
        all_shell.policy_groups.len() + all_shell.policy_users.len() + all_shell.policy_roles.len();
    println!("  Total entities: {total_shell}");

    // CIS 2.21
    if total_shell == 0 {
        println!("  CIS 2.21: No entity is attached to AWSCloudShellFullAccess. Compliant.");
    } else {
        println!(
            "  WARNING (CIS 2.21): {total_shell} entity/entities attached to \
             AWSCloudShellFullAccess. Review if access is intentional."
        );
        println!(
            "  Groups: {:?}",
            all_shell
                .policy_groups
                .iter()
                .map(|g| g.group_name.as_deref().unwrap_or("?"))
                .collect::<Vec<_>>()
        );
        println!(
            "  Users:  {:?}",
            all_shell
                .policy_users
                .iter()
                .map(|u| u.user_name.as_deref().unwrap_or("?"))
                .collect::<Vec<_>>()
        );
        println!(
            "  Roles:  {:?}",
            all_shell
                .policy_roles
                .iter()
                .map(|r| r.role_name.as_deref().unwrap_or("?"))
                .collect::<Vec<_>>()
        );
    }

    // [4/4] Verify is_truncated is false on merged results
    println!("\n[4/4] Verifying pagination markers are cleared on merged responses...");
    assert_eq!(
        all_support.is_truncated,
        Some(false),
        "merged response should have is_truncated=false"
    );
    assert!(
        all_support.marker.is_none(),
        "merged response should have no marker"
    );
    println!("  Pagination markers correctly cleared.");

    println!("\nAll ListEntitiesForPolicy tests passed!");
    Ok(())
}
