//! Core HTTP client for AWS API access.

use crate::auth::credentials::AwsCredentials;
use crate::auth::sigv4;
use crate::error::AwsError;
use base64::Engine;
use cloud_lite_core::rate_limit::{RateLimitConfig, RateLimiter};
use cloud_lite_core::retry::RetryConfig;
use md5::{Digest, Md5};

/// HTTP client for AWS API operations.
///
/// Provides automatic SigV4 signing, retry, and rate limiting.
pub struct AwsHttpClient {
    http: reqwest::Client,
    credentials: AwsCredentials,
    retry_config: RetryConfig,
    rate_limit_config: RateLimitConfig,
    rate_limiter: RateLimiter,
    /// Override base URL for testing.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) base_url: Option<String>,
    /// Mock client for testing.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) mock: Option<std::sync::Arc<crate::mock_client::MockClient>>,
}

/// Response wrapper that abstracts over real and mock responses.
pub struct AwsResponse {
    data: ResponseData,
}

enum ResponseData {
    Real(reqwest::Response),
    #[cfg(any(test, feature = "test-support"))]
    Mock(Vec<u8>),
}

impl AwsResponse {
    /// Get the HTTP status code of the response.
    pub fn status(&self) -> u16 {
        match &self.data {
            ResponseData::Real(response) => response.status().as_u16(),
            #[cfg(any(test, feature = "test-support"))]
            ResponseData::Mock(_) => 200,
        }
    }

    /// Check for HTTP error status and parse the error body.
    ///
    /// For 4xx/5xx responses, reads the body and parses the AWS error format
    /// (XML or JSON) into a typed `AwsError`. For 2xx responses, returns self
    /// so the caller can proceed with body parsing.
    pub async fn error_for_status(self, content_type: &str) -> Result<Self, AwsError> {
        let status = self.status();
        if status < 400 {
            return Ok(self);
        }

        let body_bytes = self
            .bytes()
            .await
            .unwrap_or_else(|_| bytes::Bytes::from_static(b""));
        let body_text = std::str::from_utf8(&body_bytes).unwrap_or("");

        if content_type.contains("json") {
            Err(crate::error::parse_json_error(status, body_text))
        } else {
            Err(crate::error::parse_xml_error(status, body_text))
        }
    }

    /// Read the response body as bytes.
    pub async fn bytes(self) -> Result<bytes::Bytes, AwsError> {
        match self.data {
            ResponseData::Real(response) => response
                .bytes()
                .await
                .map_err(|e| AwsError::Network(e.to_string())),
            #[cfg(any(test, feature = "test-support"))]
            ResponseData::Mock(data) => Ok(bytes::Bytes::from(data)),
        }
    }
}

/// Builder for [`AwsHttpClient`].
pub struct AwsHttpClientBuilder {
    credentials: Option<AwsCredentials>,
    retry_config: RetryConfig,
    rate_limit: RateLimitConfig,
}

impl Default for AwsHttpClientBuilder {
    fn default() -> Self {
        Self {
            credentials: None,
            retry_config: RetryConfig::default(),
            rate_limit: RateLimitConfig::new(20),
        }
    }
}

impl AwsHttpClientBuilder {
    /// Set AWS credentials.
    pub fn credentials(mut self, credentials: AwsCredentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Set retry configuration.
    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Set rate limiting configuration.
    pub fn rate_limit(mut self, config: RateLimitConfig) -> Self {
        self.rate_limit = config;
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<AwsHttpClient, AwsError> {
        let credentials = self.credentials.ok_or(AwsError::Auth {
            message: "Credentials required".into(),
        })?;

        let http = reqwest::Client::builder()
            .build()
            .map_err(|e| AwsError::Network(e.to_string()))?;

        Ok(AwsHttpClient {
            http,
            credentials,
            retry_config: self.retry_config,
            rate_limit_config: self.rate_limit.clone(),
            rate_limiter: RateLimiter::new(self.rate_limit),
            #[cfg(any(test, feature = "test-support"))]
            base_url: None,
            #[cfg(any(test, feature = "test-support"))]
            mock: None,
        })
    }
}

impl AwsHttpClient {
    /// Create a new builder.
    pub fn builder() -> AwsHttpClientBuilder {
        AwsHttpClientBuilder::default()
    }

    /// Create a client from the default credential chain (sync).
    ///
    /// Checks environment variables and credentials file. For ECS/IMDS support,
    /// use [`from_default_chain_async`](Self::from_default_chain_async).
    pub fn from_default_chain(region: &str) -> Result<Self, AwsError> {
        let credentials = AwsCredentials::from_default_chain(region)?;
        Self::builder().credentials(credentials).build()
    }

    /// Create a client from the full async credential chain.
    ///
    /// Checks environment variables, credentials file, ECS container credentials,
    /// and EC2 instance metadata (IMDSv2) in order.
    pub async fn from_default_chain_async(region: &str) -> Result<Self, AwsError> {
        let credentials = AwsCredentials::from_default_chain_async(region).await?;
        Self::builder().credentials(credentials).build()
    }

    /// Create a client from a mock for testing.
    #[cfg(any(test, feature = "test-support"))]
    pub fn from_mock(mock: crate::mock_client::MockClient) -> Self {
        let credentials =
            AwsCredentials::new("AKID".into(), "SECRET".into(), None, "us-east-1".into());
        Self {
            http: reqwest::Client::new(),
            credentials,
            retry_config: RetryConfig::default(),
            rate_limit_config: RateLimitConfig::disabled(),
            rate_limiter: RateLimiter::new(RateLimitConfig::disabled()),
            base_url: Some("http://mock".into()),
            mock: Some(std::sync::Arc::new(mock)),
        }
    }

    /// Get the configured region.
    pub fn region(&self) -> &str {
        &self.credentials.region
    }

    /// Create a new client targeting a different region.
    ///
    /// Clones the credentials with the new region and builds a fresh client
    /// preserving the same retry and rate-limit configuration. Each regional
    /// client gets its own rate limiter — AWS limits are per-account-per-region.
    pub fn with_region(&self, region: &str) -> Result<Self, AwsError> {
        let mut credentials = self.credentials.clone();
        credentials.region = region.to_string();

        let http = reqwest::Client::builder()
            .build()
            .map_err(|e| AwsError::Network(e.to_string()))?;

        Ok(Self {
            http,
            credentials,
            retry_config: self.retry_config.clone(),
            rate_limit_config: self.rate_limit_config.clone(),
            rate_limiter: RateLimiter::new(self.rate_limit_config.clone()),
            #[cfg(any(test, feature = "test-support"))]
            base_url: None,
            #[cfg(any(test, feature = "test-support"))]
            mock: None,
        })
    }

    // === Generated API Accessors (do not edit) ===

    /// Access the AWS IAM Access Analyzer API
    pub fn accessanalyzer(&self) -> crate::api::AccessAnalyzerClient<'_> {
        crate::api::AccessAnalyzerClient::new(self)
    }

    /// Access the Amazon API Gateway API
    pub fn apigateway(&self) -> crate::api::ApigatewayClient<'_> {
        crate::api::ApigatewayClient::new(self)
    }

    /// Access the Amazon Auto Scaling API
    pub fn autoscaling(&self) -> crate::api::AutoScalingClient<'_> {
        crate::api::AutoScalingClient::new(self)
    }

    /// Access the AWS Cost Explorer API
    pub fn ce(&self) -> crate::api::CeClient<'_> {
        crate::api::CeClient::new(self)
    }

    /// Access the Amazon CloudFront API
    pub fn cloudfront(&self) -> crate::api::CloudfrontClient<'_> {
        crate::api::CloudfrontClient::new(self)
    }

    /// Access the AWS CloudTrail API
    pub fn cloudtrail(&self) -> crate::api::CloudtrailClient<'_> {
        crate::api::CloudtrailClient::new(self)
    }

    /// Access the Amazon CloudWatch API
    pub fn cloudwatch(&self) -> crate::api::CloudWatchClient<'_> {
        crate::api::CloudWatchClient::new(self)
    }

    /// Access the AWS Config API
    pub fn config(&self) -> crate::api::ConfigClient<'_> {
        crate::api::ConfigClient::new(self)
    }

    /// Access the Amazon DynamoDB API
    pub fn dynamodb(&self) -> crate::api::DynamodbClient<'_> {
        crate::api::DynamodbClient::new(self)
    }

    /// Access the Amazon EC2 API
    pub fn ec2(&self) -> crate::api::Ec2Client<'_> {
        crate::api::Ec2Client::new(self)
    }

    /// Access the Amazon Elastic Container Registry API
    pub fn ecr(&self) -> crate::api::EcrClient<'_> {
        crate::api::EcrClient::new(self)
    }

    /// Access the Amazon Elastic Container Service API
    pub fn ecs(&self) -> crate::api::EcsClient<'_> {
        crate::api::EcsClient::new(self)
    }

    /// Access the Amazon Elastic File System API
    pub fn efs(&self) -> crate::api::EfsClient<'_> {
        crate::api::EfsClient::new(self)
    }

    /// Access the Amazon Elastic Kubernetes Service API
    pub fn eks(&self) -> crate::api::EksClient<'_> {
        crate::api::EksClient::new(self)
    }

    /// Access the Amazon ElastiCache API
    pub fn elasticache(&self) -> crate::api::ElasticacheClient<'_> {
        crate::api::ElasticacheClient::new(self)
    }

    /// Access the Elastic Load Balancing v2 API
    pub fn elbv2(&self) -> crate::api::Elbv2Client<'_> {
        crate::api::Elbv2Client::new(self)
    }

    /// Access the Amazon EMR API
    pub fn emr(&self) -> crate::api::EmrClient<'_> {
        crate::api::EmrClient::new(self)
    }

    /// Access the AWS Identity and Access Management API
    pub fn iam(&self) -> crate::api::IamClient<'_> {
        crate::api::IamClient::new(self)
    }

    /// Access the Amazon Kinesis API
    pub fn kinesis(&self) -> crate::api::KinesisClient<'_> {
        crate::api::KinesisClient::new(self)
    }

    /// Access the AWS Key Management Service API
    pub fn kms(&self) -> crate::api::KmsClient<'_> {
        crate::api::KmsClient::new(self)
    }

    /// Access the AWS Lambda API
    pub fn lambda(&self) -> crate::api::LambdaClient<'_> {
        crate::api::LambdaClient::new(self)
    }

    /// Access the Amazon CloudWatch Logs API
    pub fn logs(&self) -> crate::api::LogsClient<'_> {
        crate::api::LogsClient::new(self)
    }

    /// Access the Amazon OpenSearch Service API
    pub fn opensearch(&self) -> crate::api::OpensearchClient<'_> {
        crate::api::OpensearchClient::new(self)
    }

    /// Access the AWS Organizations API
    pub fn organizations(&self) -> crate::api::OrganizationsClient<'_> {
        crate::api::OrganizationsClient::new(self)
    }

    /// Access the Amazon Relational Database Service API
    pub fn rds(&self) -> crate::api::RdsClient<'_> {
        crate::api::RdsClient::new(self)
    }

    /// Access the Amazon Redshift API
    pub fn redshift(&self) -> crate::api::RedshiftClient<'_> {
        crate::api::RedshiftClient::new(self)
    }

    /// Access the Amazon Route 53 API
    pub fn route53(&self) -> crate::api::Route53Client<'_> {
        crate::api::Route53Client::new(self)
    }

    /// Access the Amazon S3 API
    pub fn s3(&self) -> crate::api::S3Client<'_> {
        crate::api::S3Client::new(self)
    }

    /// Access the Amazon SageMaker API
    pub fn sagemaker(&self) -> crate::api::SagemakerClient<'_> {
        crate::api::SagemakerClient::new(self)
    }

    /// Access the AWS Secrets Manager API
    pub fn secretsmanager(&self) -> crate::api::SecretsmanagerClient<'_> {
        crate::api::SecretsmanagerClient::new(self)
    }

    /// Access the AWS Security Hub API
    pub fn securityhub(&self) -> crate::api::SecurityHubClient<'_> {
        crate::api::SecurityHubClient::new(self)
    }

    /// Access the AWS Security Token Service API
    pub fn sts(&self) -> crate::api::StsClient<'_> {
        crate::api::StsClient::new(self)
    }
    // === End Generated API Accessors ===

    /// Make a signed GET request with automatic retry.
    pub async fn get(&self, url: &str, service: &str) -> Result<AwsResponse, AwsError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("GET", url, None).await?;
            return Ok(AwsResponse {
                data: ResponseData::Mock(result),
            });
        }

        let response = self.signed_request("GET", url, service, b"", &[]).await?;
        Ok(AwsResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a signed GET request with `Accept: application/json`.
    ///
    /// Used for REST-JSON services (e.g. API Gateway) that return HAL format
    /// by default and require `Accept: application/json` to get plain JSON.
    pub async fn get_json(&self, url: &str, service: &str) -> Result<AwsResponse, AwsError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("GET", url, None).await?;
            return Ok(AwsResponse {
                data: ResponseData::Mock(result),
            });
        }

        let response = self
            .signed_request("GET", url, service, b"", &[("accept", "application/json")])
            .await?;
        Ok(AwsResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a signed POST request with a body.
    pub async fn post(
        &self,
        url: &str,
        service: &str,
        body: &[u8],
        content_type: &str,
    ) -> Result<AwsResponse, AwsError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("POST", url, None).await?;
            return Ok(AwsResponse {
                data: ResponseData::Mock(result),
            });
        }

        let response = self
            .signed_request(
                "POST",
                url,
                service,
                body,
                &[("content-type", content_type)],
            )
            .await?;
        Ok(AwsResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a signed POST request for AWS JSON protocol services.
    ///
    /// Automatically sets `Content-Type: application/x-amz-json-{json_version}`
    /// and `X-Amz-Target: {target}`, with both headers included in the SigV4
    /// signature.
    pub async fn post_json(
        &self,
        url: &str,
        service: &str,
        target: &str,
        json_version: &str,
        body: &[u8],
    ) -> Result<AwsResponse, AwsError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("POST", url, None).await?;
            return Ok(AwsResponse {
                data: ResponseData::Mock(result),
            });
        }

        let content_type = format!("application/x-amz-json-{json_version}");
        let extra = [
            ("content-type", content_type.as_str()),
            ("x-amz-target", target),
        ];
        let response = self
            .signed_request("POST", url, service, body, &extra)
            .await?;
        Ok(AwsResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a signed PUT request with a body.
    pub async fn put(
        &self,
        url: &str,
        service: &str,
        body: &[u8],
        content_type: &str,
    ) -> Result<AwsResponse, AwsError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("PUT", url, None).await?;
            return Ok(AwsResponse {
                data: ResponseData::Mock(result),
            });
        }

        // AWS requires Content-MD5 for certain PUT operations (e.g., S3 lifecycle)
        let md5_b64;
        let extra: Vec<(&str, &str)> = if body.is_empty() {
            vec![("content-type", content_type)]
        } else {
            let digest = Md5::digest(body);
            md5_b64 = base64::engine::general_purpose::STANDARD.encode(digest);
            vec![("content-type", content_type), ("content-md5", &md5_b64)]
        };
        let response = self
            .signed_request("PUT", url, service, body, &extra)
            .await?;
        Ok(AwsResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a signed DELETE request.
    pub async fn delete(&self, url: &str, service: &str) -> Result<AwsResponse, AwsError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("DELETE", url, None).await?;
            return Ok(AwsResponse {
                data: ResponseData::Mock(result),
            });
        }

        let response = self
            .signed_request("DELETE", url, service, b"", &[])
            .await?;
        Ok(AwsResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a signed PATCH request.
    pub async fn patch(
        &self,
        url: &str,
        service: &str,
        body: &[u8],
        content_type: &str,
    ) -> Result<AwsResponse, AwsError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("PATCH", url, None).await?;
            return Ok(AwsResponse {
                data: ResponseData::Mock(result),
            });
        }

        let response = self
            .signed_request(
                "PATCH",
                url,
                service,
                body,
                &[("content-type", content_type)],
            )
            .await?;
        Ok(AwsResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Internal: signed request with arbitrary method, headers, and automatic retry.
    async fn signed_request(
        &self,
        method: &str,
        url: &str,
        service: &str,
        body: &[u8],
        extra_headers: &[(&str, &str)],
    ) -> Result<reqwest::Response, AwsError> {
        let _permit = self.rate_limiter.acquire(url).await;

        let mut attempt = 0u32;
        let mut backoff = self.retry_config.initial_backoff;
        let body_bytes = if body.is_empty() {
            None
        } else {
            Some(bytes::Bytes::copy_from_slice(body))
        };

        loop {
            let now = chrono::Utc::now();
            let signed = sigv4::sign_request(
                method,
                url,
                extra_headers,
                body,
                service,
                &self.credentials,
                &now,
            );

            let mut request = self
                .http
                .request(method.parse().expect("invalid HTTP method"), url);
            if let Some(ref b) = body_bytes {
                request = request.body(b.clone());
            }
            for &(key, value) in extra_headers {
                request = request.header(key, value);
            }
            request = request.header("Authorization", &signed.authorization);
            for (key, value) in &signed.extra_headers {
                request = request.header(key, value);
            }

            let result = match request.send().await {
                Ok(response) => Self::classify_response(response).await,
                Err(e) => Err(AwsError::from(e)),
            };

            match result {
                Ok(response) => return Ok(response),
                Err(aws_err) => {
                    if aws_err.is_retryable() && attempt < self.retry_config.max_retries {
                        let delay = self
                            .retry_config
                            .compute_backoff(backoff, aws_err.retry_after());
                        tokio::time::sleep(delay).await;
                        backoff = std::time::Duration::from_secs_f64(
                            backoff.as_secs_f64() * self.retry_config.backoff_multiplier,
                        );
                        attempt += 1;
                        continue;
                    }
                    return Err(aws_err);
                }
            }
        }
    }

    /// Classify an HTTP response: return Ok for 2xx, parse and return Err for 4xx/5xx.
    ///
    /// Extracts `Retry-After` header for throttled responses. Tries JSON then XML
    /// parsing for error bodies based on Content-Type.
    async fn classify_response(response: reqwest::Response) -> Result<reqwest::Response, AwsError> {
        let status = response.status().as_u16();
        if status < 400 {
            return Ok(response);
        }

        // Extract Retry-After header before consuming the response body
        let retry_after_secs: Option<u64> = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok());

        // Detect content type for error parsing
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let body_text = response.text().await.unwrap_or_default();

        let mut err = if content_type.contains("json") {
            crate::error::parse_json_error(status, &body_text)
        } else {
            crate::error::parse_xml_error(status, &body_text)
        };

        // Inject Retry-After duration if the header was present and the error is Throttled
        if let Some(secs) = retry_after_secs
            && let AwsError::Throttled { retry_after, .. } = &mut err
        {
            *retry_after = Some(std::time::Duration::from_secs(secs));
        }

        Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_requires_credentials() {
        let result = AwsHttpClient::builder().build();
        match result {
            Err(AwsError::Auth { .. }) => {} // expected
            Err(other) => panic!("expected Auth error, got: {other}"),
            Ok(_) => panic!("expected error, got Ok"),
        }
    }

    #[test]
    fn builder_succeeds_with_credentials() {
        let creds = AwsCredentials::new("AKID".into(), "SECRET".into(), None, "us-east-1".into());
        let client = AwsHttpClient::builder().credentials(creds).build();
        assert!(client.is_ok());
        assert_eq!(client.unwrap().region(), "us-east-1");
    }

    #[test]
    fn builder_accepts_custom_retry_config() {
        let creds = AwsCredentials::new("AKID".into(), "SECRET".into(), None, "us-east-1".into());
        let client = AwsHttpClient::builder()
            .credentials(creds)
            .retry_config(RetryConfig {
                max_retries: 5,
                ..RetryConfig::default()
            })
            .build()
            .unwrap();
        assert_eq!(client.retry_config.max_retries, 5);
    }

    #[test]
    fn builder_accepts_custom_rate_limit() {
        let creds = AwsCredentials::new("AKID".into(), "SECRET".into(), None, "us-east-1".into());
        let client = AwsHttpClient::builder()
            .credentials(creds)
            .rate_limit(RateLimitConfig::disabled())
            .build();
        assert!(client.is_ok());
    }
}
