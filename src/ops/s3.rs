//! Operation contracts for the Amazon S3 API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/s3.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::s3::*;
use crate::{AwsHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Amazon S3 API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::s3::S3Client`] instead.
pub struct S3Ops<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> S3Ops<'a> {
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> String {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/').to_string();
            }
        }
        "https://s3.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Applies an Amazon S3 bucket policy to an Amazon S3 bucket.
    ///
    /// **AWS API**: `PUT /{Bucket}?policy`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Request Body
    /// [`str`]
    #[allow(dead_code)]
    pub(crate) async fn put_bucket_policy(&self, bucket: &str, body: &str) -> Result<()> {
        let url = format!("{}/{}?policy", self.base_url(), encode(bucket),);
        let response = self.client.put(&url, "s3", body.as_bytes(), "").await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Deletes the policy of a specified bucket.
    ///
    /// **AWS API**: `DELETE /{Bucket}?policy`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_bucket_policy(&self, bucket: &str) -> Result<()> {
        let url = format!("{}/{}?policy", self.base_url(), encode(bucket),);
        let response = self.client.delete(&url, "s3").await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Creates or modifies the PublicAccessBlock configuration for an Amazon S3 bucket.
    ///
    /// **AWS API**: `PUT /{Bucket}?publicAccessBlock`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Request Body
    /// [`PublicAccessBlockConfiguration`]
    #[allow(dead_code)]
    pub(crate) async fn put_public_access_block(
        &self,
        bucket: &str,
        body: &PublicAccessBlockConfiguration,
    ) -> Result<()> {
        let url = format!("{}/{}?publicAccessBlock", self.base_url(), encode(bucket),);
        let body_xml =
            quick_xml::se::to_string(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize put_public_access_block request to XML: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .put(&url, "s3", body_xml.as_bytes(), "application/xml")
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Creates a new lifecycle configuration for the bucket or replaces an existing lifecycle
    /// configuration.
    ///
    /// **AWS API**: `PUT /{Bucket}?lifecycle`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Request Body
    /// [`BucketLifecycleConfiguration`]
    #[allow(dead_code)]
    pub(crate) async fn put_bucket_lifecycle_configuration(
        &self,
        bucket: &str,
        body: &BucketLifecycleConfiguration,
    ) -> Result<()> {
        let url = format!("{}/{}?lifecycle", self.base_url(), encode(bucket),);
        let body_xml =
            quick_xml::se::to_string(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to serialize put_bucket_lifecycle_configuration request to XML: {e}"
                ),
                body: None,
            })?;
        let response = self
            .client
            .put(&url, "s3", body_xml.as_bytes(), "application/xml")
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Returns a list of all buckets owned by the authenticated sender of the request.
    ///
    /// **AWS API**: `GET /`
    ///
    /// # Response
    /// [`ListBucketsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_buckets(&self) -> Result<ListBucketsResponse> {
        let url = format!("{}/", self.base_url(),);
        let response = self.client.get(&url, "s3").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_buckets response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_buckets response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<ListBucketsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_buckets XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Returns the versioning state of a bucket.
    ///
    /// **AWS API**: `GET /{Bucket}?versioning`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Response
    /// [`GetBucketVersioningResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_bucket_versioning(
        &self,
        bucket: &str,
    ) -> Result<GetBucketVersioningResponse> {
        let url = format!("{}/{}?versioning", self.base_url(), encode(bucket),);
        let response = self.client.get(&url, "s3").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_bucket_versioning response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_bucket_versioning response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<GetBucketVersioningResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_bucket_versioning XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Returns the default encryption configuration for an Amazon S3 bucket.
    ///
    /// **AWS API**: `GET /{Bucket}?encryption`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Response
    /// [`ServerSideEncryptionConfiguration`]
    #[allow(dead_code)]
    pub(crate) async fn get_bucket_encryption(
        &self,
        bucket: &str,
    ) -> Result<ServerSideEncryptionConfiguration> {
        let url = format!("{}/{}?encryption", self.base_url(), encode(bucket),);
        let response = self.client.get(&url, "s3").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_bucket_encryption response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_bucket_encryption response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<ServerSideEncryptionConfiguration>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_bucket_encryption XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Returns the logging status of a bucket and the permissions users have to view and modify
    /// that status.
    ///
    /// **AWS API**: `GET /{Bucket}?logging`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Response
    /// [`GetBucketLoggingResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_bucket_logging(
        &self,
        bucket: &str,
    ) -> Result<GetBucketLoggingResponse> {
        let url = format!("{}/{}?logging", self.base_url(), encode(bucket),);
        let response = self.client.get(&url, "s3").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_bucket_logging response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_bucket_logging response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<GetBucketLoggingResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_bucket_logging XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Returns the access control list (ACL) of a bucket.
    ///
    /// **AWS API**: `GET /{Bucket}?acl`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Response
    /// [`GetBucketAclResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_bucket_acl(&self, bucket: &str) -> Result<GetBucketAclResponse> {
        let url = format!("{}/{}?acl", self.base_url(), encode(bucket),);
        let response = self.client.get(&url, "s3").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_bucket_acl response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_bucket_acl response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<GetBucketAclResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_bucket_acl XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Returns the lifecycle configuration information set on the bucket.
    ///
    /// **AWS API**: `GET /{Bucket}?lifecycle`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Response
    /// [`GetBucketLifecycleConfigurationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_bucket_lifecycle_configuration(
        &self,
        bucket: &str,
    ) -> Result<GetBucketLifecycleConfigurationResponse> {
        let url = format!("{}/{}?lifecycle", self.base_url(), encode(bucket),);
        let response = self.client.get(&url, "s3").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read get_bucket_lifecycle_configuration response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Invalid UTF-8 in get_bucket_lifecycle_configuration response: {e}"
                ),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<GetBucketLifecycleConfigurationResponse>(body_text)
            .map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse get_bucket_lifecycle_configuration XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            })
    }

    /// Retrieves the PublicAccessBlock configuration for an Amazon S3 bucket.
    ///
    /// **AWS API**: `GET /{Bucket}?publicAccessBlock`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Response
    /// [`PublicAccessBlockConfiguration`]
    #[allow(dead_code)]
    pub(crate) async fn get_public_access_block(
        &self,
        bucket: &str,
    ) -> Result<PublicAccessBlockConfiguration> {
        let url = format!("{}/{}?publicAccessBlock", self.base_url(), encode(bucket),);
        let response = self.client.get(&url, "s3").await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_public_access_block response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_public_access_block response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<PublicAccessBlockConfiguration>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_public_access_block XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Deletes the lifecycle configuration from the specified bucket.
    ///
    /// **AWS API**: `DELETE /{Bucket}?lifecycle`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_bucket_lifecycle_configuration(&self, bucket: &str) -> Result<()> {
        let url = format!("{}/{}?lifecycle", self.base_url(), encode(bucket),);
        let response = self.client.delete(&url, "s3").await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Sets the versioning state of an existing bucket.
    ///
    /// **AWS API**: `PUT /{Bucket}?versioning`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Request Body
    /// [`VersioningConfiguration`]
    #[allow(dead_code)]
    pub(crate) async fn put_bucket_versioning(
        &self,
        bucket: &str,
        body: &VersioningConfiguration,
    ) -> Result<()> {
        let url = format!("{}/{}?versioning", self.base_url(), encode(bucket),);
        let body_xml =
            quick_xml::se::to_string(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize put_bucket_versioning request to XML: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .put(&url, "s3", body_xml.as_bytes(), "application/xml")
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Creates the default encryption configuration for an Amazon S3 bucket.
    ///
    /// **AWS API**: `PUT /{Bucket}?encryption`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Request Body
    /// [`ServerSideEncryptionConfiguration`]
    #[allow(dead_code)]
    pub(crate) async fn put_bucket_encryption(
        &self,
        bucket: &str,
        body: &ServerSideEncryptionConfiguration,
    ) -> Result<()> {
        let url = format!("{}/{}?encryption", self.base_url(), encode(bucket),);
        let body_xml =
            quick_xml::se::to_string(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize put_bucket_encryption request to XML: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .put(&url, "s3", body_xml.as_bytes(), "application/xml")
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Set the logging parameters for a bucket.
    ///
    /// **AWS API**: `PUT /{Bucket}?logging`
    ///
    /// # Path Parameters
    /// - `Bucket` —  *(required)*
    ///
    /// # Request Body
    /// [`BucketLoggingStatus`]
    #[allow(dead_code)]
    pub(crate) async fn put_bucket_logging(
        &self,
        bucket: &str,
        body: &BucketLoggingStatus,
    ) -> Result<()> {
        let url = format!("{}/{}?logging", self.base_url(), encode(bucket),);
        let body_xml =
            quick_xml::se::to_string(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize put_bucket_logging request to XML: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .put(&url, "s3", body_xml.as_bytes(), "application/xml")
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_put_bucket_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/test-Bucket?policy")
            .returning_bytes(vec![]);

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let body = "test-body";
        let result = ops.put_bucket_policy("test-Bucket", body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_bucket_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/test-Bucket?policy")
            .returning_bytes(vec![]);

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let result = ops.delete_bucket_policy("test-Bucket").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_public_access_block() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/test-Bucket?publicAccessBlock")
            .returning_bytes(vec![]);

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let body = PublicAccessBlockConfiguration::fixture();
        let result = ops.put_public_access_block("test-Bucket", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_bucket_lifecycle_configuration() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/test-Bucket?lifecycle")
            .returning_bytes(vec![]);

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let body = BucketLifecycleConfiguration::fixture();
        let result = ops
            .put_bucket_lifecycle_configuration("test-Bucket", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_buckets() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/").returning_bytes({
            let fixture = ListBucketsResponse::fixture();
            quick_xml::se::to_string(&fixture).unwrap().into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let result = ops.list_buckets().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_bucket_versioning() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/test-Bucket?versioning").returning_bytes({
            let fixture = GetBucketVersioningResponse::fixture();
            quick_xml::se::to_string(&fixture).unwrap().into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let result = ops.get_bucket_versioning("test-Bucket").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_bucket_encryption() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/test-Bucket?encryption").returning_bytes({
            let fixture = ServerSideEncryptionConfiguration::fixture();
            quick_xml::se::to_string(&fixture).unwrap().into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let result = ops.get_bucket_encryption("test-Bucket").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_bucket_logging() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/test-Bucket?logging").returning_bytes({
            let fixture = GetBucketLoggingResponse::fixture();
            quick_xml::se::to_string(&fixture).unwrap().into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let result = ops.get_bucket_logging("test-Bucket").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_bucket_acl() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/test-Bucket?acl").returning_bytes({
            let fixture = GetBucketAclResponse::fixture();
            quick_xml::se::to_string(&fixture).unwrap().into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let result = ops.get_bucket_acl("test-Bucket").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_bucket_lifecycle_configuration() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/test-Bucket?lifecycle").returning_bytes({
            let fixture = GetBucketLifecycleConfigurationResponse::fixture();
            quick_xml::se::to_string(&fixture).unwrap().into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let result = ops.get_bucket_lifecycle_configuration("test-Bucket").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_public_access_block() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/test-Bucket?publicAccessBlock")
            .returning_bytes({
                let fixture = PublicAccessBlockConfiguration::fixture();
                quick_xml::se::to_string(&fixture).unwrap().into_bytes()
            });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let result = ops.get_public_access_block("test-Bucket").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_bucket_lifecycle_configuration() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/test-Bucket?lifecycle")
            .returning_bytes(vec![]);

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let result = ops
            .delete_bucket_lifecycle_configuration("test-Bucket")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_bucket_versioning() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/test-Bucket?versioning")
            .returning_bytes(vec![]);

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let body = VersioningConfiguration::fixture();
        let result = ops.put_bucket_versioning("test-Bucket", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_bucket_encryption() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/test-Bucket?encryption")
            .returning_bytes(vec![]);

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let body = ServerSideEncryptionConfiguration::fixture();
        let result = ops.put_bucket_encryption("test-Bucket", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_bucket_logging() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/test-Bucket?logging")
            .returning_bytes(vec![]);

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = S3Ops::new(&client);

        let body = BucketLoggingStatus::fixture();
        let result = ops.put_bucket_logging("test-Bucket", &body).await;
        assert!(result.is_ok());
    }
}
