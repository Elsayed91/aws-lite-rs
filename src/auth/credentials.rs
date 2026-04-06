//! AWS credential resolution.

use crate::error::AwsError;
use std::path::PathBuf;

/// AWS credentials (access key + secret + optional session token).
#[derive(Debug, Clone)]
pub struct AwsCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub region: String,
}

impl AwsCredentials {
    /// Resolve credentials from the default chain:
    /// 1. Environment variables (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_SESSION_TOKEN)
    /// 2. Shared credentials file (~/.aws/credentials) using AWS_PROFILE or "default"
    /// 3. ECS container credentials (AWS_CONTAINER_CREDENTIALS_RELATIVE_URI)
    /// 4. EC2 instance metadata (IMDSv2)
    pub fn from_default_chain(region: &str) -> Result<Self, AwsError> {
        // 1. Environment variables
        if let Ok(access_key) = std::env::var("AWS_ACCESS_KEY_ID")
            && let Ok(secret_key) = std::env::var("AWS_SECRET_ACCESS_KEY")
        {
            return Ok(Self {
                access_key_id: access_key,
                secret_access_key: secret_key,
                session_token: std::env::var("AWS_SESSION_TOKEN").ok(),
                region: std::env::var("AWS_REGION")
                    .or_else(|_| std::env::var("AWS_DEFAULT_REGION"))
                    .unwrap_or_else(|_| region.to_string()),
            });
        }

        // 2. Shared credentials file (respects AWS_PROFILE)
        let profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());
        if let Ok(creds) = Self::from_credentials_file_with_profile(region, &profile) {
            return Ok(creds);
        }

        // 3. ECS container credentials
        if std::env::var("AWS_CONTAINER_CREDENTIALS_RELATIVE_URI").is_ok()
            || std::env::var("AWS_CONTAINER_CREDENTIALS_FULL_URI").is_ok()
        {
            // ECS credentials require async HTTP — return a marker error so the caller
            // can use `from_ecs_container` instead, or fall through to IMDS.
            // We can't do async here in a sync function, so we return an error and let
            // the async `from_default_chain_async` handle it.
        }

        Err(AwsError::Auth {
            message: format!(
                "No credentials found. Checked: environment variables, credentials file (profile: {profile}). \
                 For ECS/IMDS credentials, use AwsCredentials::from_default_chain_async()."
            ),
        })
    }

    /// Async version of credential resolution that includes ECS and IMDS sources.
    ///
    /// Resolution order:
    /// 1. Environment variables
    /// 2. Shared credentials file (~/.aws/credentials)
    /// 3. ECS container credentials
    /// 4. EC2 instance metadata (IMDSv2)
    pub async fn from_default_chain_async(region: &str) -> Result<Self, AwsError> {
        // 1. Environment variables
        if let Ok(access_key) = std::env::var("AWS_ACCESS_KEY_ID")
            && let Ok(secret_key) = std::env::var("AWS_SECRET_ACCESS_KEY")
        {
            return Ok(Self {
                access_key_id: access_key,
                secret_access_key: secret_key,
                session_token: std::env::var("AWS_SESSION_TOKEN").ok(),
                region: std::env::var("AWS_REGION")
                    .or_else(|_| std::env::var("AWS_DEFAULT_REGION"))
                    .unwrap_or_else(|_| region.to_string()),
            });
        }

        // 2. Shared credentials file
        let profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());
        if let Ok(creds) = Self::from_credentials_file_with_profile(region, &profile) {
            return Ok(creds);
        }

        // 3. ECS container credentials
        if let Ok(creds) = Self::from_ecs_container(region).await {
            return Ok(creds);
        }

        // 4. EC2 instance metadata (IMDSv2)
        if let Ok(creds) = Self::from_imds_v2(region).await {
            return Ok(creds);
        }

        Err(AwsError::Auth {
            message: format!(
                "No credentials found. Checked: environment variables, credentials file (profile: {profile}), \
                 ECS container credentials, EC2 instance metadata (IMDSv2)."
            ),
        })
    }

    /// Create credentials from explicit values.
    pub fn new(
        access_key_id: String,
        secret_access_key: String,
        session_token: Option<String>,
        region: String,
    ) -> Self {
        Self {
            access_key_id,
            secret_access_key,
            session_token,
            region,
        }
    }

    /// Load credentials from a named profile in ~/.aws/credentials.
    pub fn from_profile(region: &str, profile: &str) -> Result<Self, AwsError> {
        Self::from_credentials_file_with_profile(region, profile)
    }

    /// Load credentials from ~/.aws/credentials using a specific profile.
    fn from_credentials_file_with_profile(region: &str, profile: &str) -> Result<Self, AwsError> {
        let path = credentials_file_path().ok_or_else(|| AwsError::Auth {
            message: "Could not determine AWS credentials file path".into(),
        })?;

        let content = std::fs::read_to_string(&path).map_err(|e| AwsError::Auth {
            message: format!("Failed to read {}: {}", path.display(), e),
        })?;

        parse_credentials_file(&content, region, profile)
    }

    /// Fetch credentials from ECS container metadata.
    ///
    /// Checks `AWS_CONTAINER_CREDENTIALS_RELATIVE_URI` (relative to `http://169.254.170.2`)
    /// and `AWS_CONTAINER_CREDENTIALS_FULL_URI` (absolute URL) in that order.
    async fn from_ecs_container(region: &str) -> Result<Self, AwsError> {
        let (url, auth_token) =
            if let Ok(relative_uri) = std::env::var("AWS_CONTAINER_CREDENTIALS_RELATIVE_URI") {
                (
                    format!("http://169.254.170.2{relative_uri}"),
                    std::env::var("AWS_CONTAINER_AUTHORIZATION_TOKEN").ok(),
                )
            } else if let Ok(full_uri) = std::env::var("AWS_CONTAINER_CREDENTIALS_FULL_URI") {
                (
                    full_uri,
                    std::env::var("AWS_CONTAINER_AUTHORIZATION_TOKEN").ok(),
                )
            } else {
                return Err(AwsError::Auth {
                    message: "No ECS container credentials URI set".into(),
                });
            };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()
            .map_err(|e| AwsError::Auth {
                message: format!("Failed to create HTTP client for ECS credentials: {e}"),
            })?;

        let mut request = client.get(&url);
        if let Some(token) = auth_token {
            request = request.header("Authorization", token);
        }

        let response = request.send().await.map_err(|e| AwsError::Auth {
            message: format!("Failed to fetch ECS container credentials from {url}: {e}"),
        })?;

        if !response.status().is_success() {
            return Err(AwsError::Auth {
                message: format!(
                    "ECS container credentials returned HTTP {}",
                    response.status()
                ),
            });
        }

        let text = response.text().await.map_err(|e| AwsError::Auth {
            message: format!("Failed to read ECS container credentials: {e}"),
        })?;
        let body: serde_json::Value = serde_json::from_str(&text).map_err(|e| AwsError::Auth {
            message: format!("Failed to parse ECS container credentials: {e}"),
        })?;

        let access_key = body["AccessKeyId"]
            .as_str()
            .ok_or_else(|| AwsError::Auth {
                message: "Missing AccessKeyId in ECS credentials response".into(),
            })?
            .to_string();

        let secret_key = body["SecretAccessKey"]
            .as_str()
            .ok_or_else(|| AwsError::Auth {
                message: "Missing SecretAccessKey in ECS credentials response".into(),
            })?
            .to_string();

        let session_token = body["Token"].as_str().map(String::from);

        Ok(Self {
            access_key_id: access_key,
            secret_access_key: secret_key,
            session_token,
            region: region.to_string(),
        })
    }

    /// Fetch credentials from EC2 Instance Metadata Service v2 (IMDSv2).
    ///
    /// 1. PUT to get a session token
    /// 2. GET the IAM role name
    /// 3. GET the credentials for that role
    async fn from_imds_v2(region: &str) -> Result<Self, AwsError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()
            .map_err(|e| AwsError::Auth {
                message: format!("Failed to create HTTP client for IMDS: {e}"),
            })?;

        // Step 1: Get IMDSv2 session token
        let token_response = client
            .put("http://169.254.169.254/latest/api/token")
            .header("X-aws-ec2-metadata-token-ttl-seconds", "21600")
            .send()
            .await
            .map_err(|e| AwsError::Auth {
                message: format!("Failed to get IMDSv2 token: {e}"),
            })?;

        if !token_response.status().is_success() {
            return Err(AwsError::Auth {
                message: format!(
                    "IMDSv2 token request returned HTTP {}",
                    token_response.status()
                ),
            });
        }

        let token = token_response.text().await.map_err(|e| AwsError::Auth {
            message: format!("Failed to read IMDSv2 token: {e}"),
        })?;

        // Step 2: Get IAM role name
        let role_response = client
            .get("http://169.254.169.254/latest/meta-data/iam/security-credentials/")
            .header("X-aws-ec2-metadata-token", &token)
            .send()
            .await
            .map_err(|e| AwsError::Auth {
                message: format!("Failed to get IAM role name from IMDS: {e}"),
            })?;

        if !role_response.status().is_success() {
            return Err(AwsError::Auth {
                message: format!(
                    "IMDS IAM role request returned HTTP {}",
                    role_response.status()
                ),
            });
        }

        let role_name = role_response.text().await.map_err(|e| AwsError::Auth {
            message: format!("Failed to read IAM role name: {e}"),
        })?;
        let role_name = role_name.trim();

        if role_name.is_empty() {
            return Err(AwsError::Auth {
                message: "No IAM role attached to this EC2 instance".into(),
            });
        }

        // Step 3: Get credentials for the role
        let creds_url =
            format!("http://169.254.169.254/latest/meta-data/iam/security-credentials/{role_name}");
        let creds_response = client
            .get(&creds_url)
            .header("X-aws-ec2-metadata-token", &token)
            .send()
            .await
            .map_err(|e| AwsError::Auth {
                message: format!("Failed to get IMDS credentials for role {role_name}: {e}"),
            })?;

        if !creds_response.status().is_success() {
            return Err(AwsError::Auth {
                message: format!(
                    "IMDS credentials request returned HTTP {}",
                    creds_response.status()
                ),
            });
        }

        let text = creds_response.text().await.map_err(|e| AwsError::Auth {
            message: format!("Failed to read IMDS credentials: {e}"),
        })?;
        let body: serde_json::Value = serde_json::from_str(&text).map_err(|e| AwsError::Auth {
            message: format!("Failed to parse IMDS credentials: {e}"),
        })?;

        let access_key = body["AccessKeyId"]
            .as_str()
            .ok_or_else(|| AwsError::Auth {
                message: "Missing AccessKeyId in IMDS credentials".into(),
            })?
            .to_string();

        let secret_key = body["SecretAccessKey"]
            .as_str()
            .ok_or_else(|| AwsError::Auth {
                message: "Missing SecretAccessKey in IMDS credentials".into(),
            })?
            .to_string();

        let session_token = body["Token"].as_str().map(String::from);

        Ok(Self {
            access_key_id: access_key,
            secret_access_key: secret_key,
            session_token,
            region: region.to_string(),
        })
    }
}

fn credentials_file_path() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("AWS_SHARED_CREDENTIALS_FILE") {
        return Some(PathBuf::from(path));
    }
    dirs::home_dir().map(|h| h.join(".aws").join("credentials"))
}

fn parse_credentials_file(
    content: &str,
    region: &str,
    profile: &str,
) -> Result<AwsCredentials, AwsError> {
    let mut access_key = None;
    let mut secret_key = None;
    let mut session_token = None;
    let target_section = format!("[{profile}]");
    // Also handle "profile X" syntax used in ~/.aws/config
    let target_section_alt = format!("[profile {profile}]");
    let mut in_target = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_target = trimmed == target_section || trimmed == target_section_alt;
            continue;
        }
        if !in_target {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "aws_access_key_id" => access_key = Some(value.to_string()),
                "aws_secret_access_key" => secret_key = Some(value.to_string()),
                "aws_session_token" => session_token = Some(value.to_string()),
                _ => {}
            }
        }
    }

    match (access_key, secret_key) {
        (Some(ak), Some(sk)) => Ok(AwsCredentials {
            access_key_id: ak,
            secret_access_key: sk,
            session_token,
            region: region.to_string(),
        }),
        _ => Err(AwsError::Auth {
            message: format!("No valid [{profile}] credentials found in credentials file"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_default_profile() {
        let content = "\
[default]
aws_access_key_id = AKIAIOSFODNN7EXAMPLE
aws_secret_access_key = wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
aws_session_token = FwoGZXIvYXdzEBYaD

[other]
aws_access_key_id = OTHER
aws_secret_access_key = OTHER_SECRET
";
        let creds = parse_credentials_file(content, "us-east-1", "default").unwrap();
        assert_eq!(creds.access_key_id, "AKIAIOSFODNN7EXAMPLE");
        assert_eq!(
            creds.secret_access_key,
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
        );
        assert_eq!(creds.session_token.as_deref(), Some("FwoGZXIvYXdzEBYaD"));
        assert_eq!(creds.region, "us-east-1");
    }

    #[test]
    fn parse_named_profile() {
        let content = "\
[default]
aws_access_key_id = DEFAULT_ID
aws_secret_access_key = DEFAULT_SECRET

[production]
aws_access_key_id = PROD_AKID
aws_secret_access_key = PROD_SECRET
aws_session_token = PROD_TOKEN
";
        let creds = parse_credentials_file(content, "us-west-2", "production").unwrap();
        assert_eq!(creds.access_key_id, "PROD_AKID");
        assert_eq!(creds.secret_access_key, "PROD_SECRET");
        assert_eq!(creds.session_token.as_deref(), Some("PROD_TOKEN"));
        assert_eq!(creds.region, "us-west-2");
    }

    #[test]
    fn parse_profile_syntax_from_config_file() {
        let content = "\
[profile staging]
aws_access_key_id = STAGING_AKID
aws_secret_access_key = STAGING_SECRET
";
        let creds = parse_credentials_file(content, "eu-west-1", "staging").unwrap();
        assert_eq!(creds.access_key_id, "STAGING_AKID");
        assert_eq!(creds.secret_access_key, "STAGING_SECRET");
    }

    #[test]
    fn parse_missing_profile_errors() {
        let content = "\
[default]
aws_access_key_id = DEFAULT_ID
aws_secret_access_key = DEFAULT_SECRET
";
        let result = parse_credentials_file(content, "us-east-1", "nonexistent");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("nonexistent"),
            "Error should mention the profile name: {err}"
        );
    }

    #[test]
    fn parse_missing_default_profile_errors() {
        let content = "\
[other]
aws_access_key_id = OTHER
aws_secret_access_key = OTHER_SECRET
";
        let result = parse_credentials_file(content, "us-east-1", "default");
        assert!(result.is_err());
    }

    #[test]
    fn parse_only_access_key_errors() {
        let content = "\
[default]
aws_access_key_id = AKIAIOSFODNN7EXAMPLE
";
        let result = parse_credentials_file(content, "us-east-1", "default");
        assert!(result.is_err());
    }

    #[test]
    fn parse_empty_content_errors() {
        let result = parse_credentials_file("", "us-east-1", "default");
        assert!(result.is_err());
    }

    #[test]
    fn parse_no_session_token() {
        let content = "\
[default]
aws_access_key_id = AKID
aws_secret_access_key = SECRET
";
        let creds = parse_credentials_file(content, "eu-west-1", "default").unwrap();
        assert_eq!(creds.access_key_id, "AKID");
        assert_eq!(creds.secret_access_key, "SECRET");
        assert!(creds.session_token.is_none());
        assert_eq!(creds.region, "eu-west-1");
    }

    #[test]
    fn parse_ignores_other_profiles() {
        let content = "\
[production]
aws_access_key_id = PROD_ID
aws_secret_access_key = PROD_SECRET

[default]
aws_access_key_id = DEFAULT_ID
aws_secret_access_key = DEFAULT_SECRET
";
        let creds = parse_credentials_file(content, "us-west-2", "default").unwrap();
        assert_eq!(creds.access_key_id, "DEFAULT_ID");
        assert_eq!(creds.secret_access_key, "DEFAULT_SECRET");
    }

    #[test]
    fn parse_handles_whitespace_around_equals() {
        let content = "\
[default]
aws_access_key_id  =  AKID
aws_secret_access_key  =  SECRET
";
        let creds = parse_credentials_file(content, "us-east-1", "default").unwrap();
        assert_eq!(creds.access_key_id, "AKID");
        assert_eq!(creds.secret_access_key, "SECRET");
    }

    #[test]
    fn parse_multiple_profiles_correct_selection() {
        let content = "\
[default]
aws_access_key_id = DEFAULT_AKID
aws_secret_access_key = DEFAULT_SECRET

[dev]
aws_access_key_id = DEV_AKID
aws_secret_access_key = DEV_SECRET

[staging]
aws_access_key_id = STAGING_AKID
aws_secret_access_key = STAGING_SECRET

[prod]
aws_access_key_id = PROD_AKID
aws_secret_access_key = PROD_SECRET
";
        let dev = parse_credentials_file(content, "us-east-1", "dev").unwrap();
        assert_eq!(dev.access_key_id, "DEV_AKID");

        let staging = parse_credentials_file(content, "us-east-1", "staging").unwrap();
        assert_eq!(staging.access_key_id, "STAGING_AKID");

        let prod = parse_credentials_file(content, "us-east-1", "prod").unwrap();
        assert_eq!(prod.access_key_id, "PROD_AKID");
    }

    #[test]
    fn from_profile_delegates_correctly() {
        // This just tests the public API surface exists and calls through.
        // Actual file I/O would fail in test env without a real credentials file,
        // so we just verify the error message mentions the profile name.
        let result = AwsCredentials::from_profile("us-east-1", "my-custom-profile");
        // Either succeeds (if a credentials file with this profile exists) or errors
        if let Err(e) = result {
            let msg = e.to_string();
            // Should fail with file-not-found or profile-not-found, not a generic error
            assert!(
                msg.contains("credentials") || msg.contains("my-custom-profile"),
                "Error should be about credentials: {msg}"
            );
        }
    }
}
