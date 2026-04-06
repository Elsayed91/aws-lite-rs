//! Amazon S3 API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::s3::S3Ops`. This layer adds:
//! - Ergonomic method signatures
//! - Custom XML serialization for lifecycle configuration (S3 expects
//!   non-standard XML element names that `quick_xml::se` cannot produce)

use crate::{
    AwsHttpClient, Result, iam_policy::PolicyDocument, ops::s3::S3Ops,
    types::s3::BucketLifecycleConfiguration, types::s3::BucketLoggingStatus,
    types::s3::GetBucketAclResponse, types::s3::GetBucketLifecycleConfigurationResponse,
    types::s3::GetBucketLoggingResponse, types::s3::GetBucketVersioningResponse,
    types::s3::Grantee, types::s3::ListBucketsResponse, types::s3::PublicAccessBlockConfiguration,
    types::s3::ServerSideEncryptionConfiguration, types::s3::VersioningConfiguration,
};
use urlencoding::encode;

/// Client for the Amazon S3 API
pub struct S3Client<'a> {
    ops: S3Ops<'a>,
}

impl<'a> S3Client<'a> {
    /// Create a new Amazon S3 API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: S3Ops::new(client),
        }
    }

    /// Applies an Amazon S3 bucket policy to an Amazon S3 bucket.
    pub async fn put_bucket_policy(&self, bucket: &str, body: &str) -> Result<()> {
        self.ops.put_bucket_policy(bucket, body).await
    }

    /// Deletes the policy of a specified bucket.
    pub async fn delete_bucket_policy(&self, bucket: &str) -> Result<()> {
        self.ops.delete_bucket_policy(bucket).await
    }

    /// Creates or modifies the PublicAccessBlock configuration for an Amazon S3 bucket.
    pub async fn put_public_access_block(
        &self,
        bucket: &str,
        body: &PublicAccessBlockConfiguration,
    ) -> Result<()> {
        self.ops.put_public_access_block(bucket, body).await
    }

    // ---- Read operations ----

    /// Returns a list of all buckets owned by the authenticated sender of the request.
    pub async fn list_buckets(&self) -> Result<ListBucketsResponse> {
        self.ops.list_buckets().await
    }

    /// Returns the versioning state of a bucket.
    pub async fn get_bucket_versioning(&self, bucket: &str) -> Result<GetBucketVersioningResponse> {
        self.ops.get_bucket_versioning(bucket).await
    }

    /// Returns the default encryption configuration for an Amazon S3 bucket.
    pub async fn get_bucket_encryption(
        &self,
        bucket: &str,
    ) -> Result<ServerSideEncryptionConfiguration> {
        self.ops.get_bucket_encryption(bucket).await
    }

    /// Returns the logging status of a bucket.
    pub async fn get_bucket_logging(&self, bucket: &str) -> Result<GetBucketLoggingResponse> {
        self.ops.get_bucket_logging(bucket).await
    }

    /// Returns the access control list (ACL) of a bucket.
    pub async fn get_bucket_acl(&self, bucket: &str) -> Result<GetBucketAclResponse> {
        self.ops.get_bucket_acl(bucket).await
    }

    /// Returns the lifecycle configuration information set on the bucket.
    pub async fn get_bucket_lifecycle_configuration(
        &self,
        bucket: &str,
    ) -> Result<GetBucketLifecycleConfigurationResponse> {
        self.ops.get_bucket_lifecycle_configuration(bucket).await
    }

    /// Retrieves the PublicAccessBlock configuration for an Amazon S3 bucket.
    pub async fn get_public_access_block(
        &self,
        bucket: &str,
    ) -> Result<PublicAccessBlockConfiguration> {
        self.ops.get_public_access_block(bucket).await
    }

    /// Returns the bucket policy as a parsed [`PolicyDocument`].
    ///
    /// This operation is implemented manually because S3 returns the policy as a raw JSON
    /// payload (not XML). The JSON is deserialized into a typed `PolicyDocument` so callers
    /// receive structured statements rather than a raw string to re-parse.
    pub async fn get_bucket_policy(&self, bucket: &str) -> Result<PolicyDocument> {
        let base = self.s3_base_url();
        let url = format!("{}/{}?policy", base, encode(bucket));
        let response = self.ops.client.get(&url, "s3").await?;
        let response = response.error_for_status("xml").await?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to read get_bucket_policy response: {e}"),
                body: None,
            })?;
        let json =
            String::from_utf8(bytes.to_vec()).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_bucket_policy response: {e}"),
                body: None,
            })?;
        PolicyDocument::from_json(&json).ok_or_else(|| crate::AwsError::InvalidResponse {
            message: "Failed to parse bucket policy JSON as PolicyDocument".into(),
            body: Some(json),
        })
    }

    // ---- Write operations ----

    /// Sets the versioning state of an existing bucket.
    pub async fn put_bucket_versioning(
        &self,
        bucket: &str,
        body: &VersioningConfiguration,
    ) -> Result<()> {
        self.ops.put_bucket_versioning(bucket, body).await
    }

    /// Creates the default encryption configuration for an Amazon S3 bucket.
    pub async fn put_bucket_encryption(
        &self,
        bucket: &str,
        body: &ServerSideEncryptionConfiguration,
    ) -> Result<()> {
        self.ops.put_bucket_encryption(bucket, body).await
    }

    /// Set the logging parameters for a bucket.
    pub async fn put_bucket_logging(&self, bucket: &str, body: &BucketLoggingStatus) -> Result<()> {
        self.ops.put_bucket_logging(bucket, body).await
    }

    /// Deletes the lifecycle configuration from the specified bucket.
    ///
    /// Returns 204 even if no lifecycle configuration exists (idempotent).
    pub async fn delete_bucket_lifecycle_configuration(&self, bucket: &str) -> Result<()> {
        self.ops.delete_bucket_lifecycle_configuration(bucket).await
    }

    /// Creates a new lifecycle configuration for the bucket or replaces an existing lifecycle
    /// configuration.
    ///
    /// Uses hand-built XML because S3 expects `<LifecycleConfiguration>` root with
    /// `<Rule>` elements (not `<BucketLifecycleConfiguration>` / `<Rules>`).
    pub async fn put_bucket_lifecycle_configuration(
        &self,
        bucket: &str,
        body: &BucketLifecycleConfiguration,
    ) -> Result<()> {
        let xml = build_lifecycle_xml(body);
        let base = self.s3_base_url();
        let url = format!("{}/{}?lifecycle", base, encode(bucket));
        let response = self
            .ops
            .client
            .put(&url, "s3", xml.as_bytes(), "application/xml")
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    fn s3_base_url(&self) -> String {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.ops.client.base_url {
                return base.trim_end_matches('/').to_string();
            }
        }
        format!("https://s3.{}.amazonaws.com", self.ops.client.region())
    }
}

impl Grantee {
    /// Infer the grantee type from the populated fields.
    ///
    /// S3 encodes the grantee type as an `xsi:type` XML attribute on `<Grantee>`, which
    /// the codegen cannot deserialize (it is a namespace-prefixed attribute, not an element).
    /// This method approximates it from the fields that *are* present:
    /// - `URI` is set only for group grantees (e.g. `AllUsers`, `AuthenticatedUsers`)
    /// - `ID` is set for canonical user grantees
    ///
    /// The URI value (e.g. `http://acs.amazonaws.com/groups/global/AllUsers`) is typically
    /// what callers match on anyway; this method provides the type label as a convenience.
    pub fn grantee_type(&self) -> &str {
        if self.uri.is_some() {
            "Group"
        } else if self.id.is_some() {
            "CanonicalUser"
        } else {
            "Unknown"
        }
    }
}

/// Build S3-compatible lifecycle configuration XML.
///
/// S3 expects specific element names that differ from the serde-derived names:
/// - Root: `<LifecycleConfiguration>` (not `<BucketLifecycleConfiguration>`)
/// - Each rule: `<Rule>` (not wrapped in `<Rules>`)
/// - Rule ID: `<ID>` (not `<Id>`)
fn build_lifecycle_xml(config: &BucketLifecycleConfiguration) -> String {
    use std::fmt::Write;

    let mut xml = String::new();
    write!(
        xml,
        r#"<LifecycleConfiguration xmlns="http://s3.amazonaws.com/doc/2006-03-01/">"#
    )
    .unwrap();

    for rule in &config.rules {
        xml.push_str("<Rule>");

        if let Some(ref id) = rule.id {
            write!(xml, "<ID>{}</ID>", escape_xml(id)).unwrap();
        }
        if let Some(ref prefix) = rule.prefix {
            write!(xml, "<Prefix>{}</Prefix>", escape_xml(prefix)).unwrap();
        }
        if let Some(ref filter) = rule.filter {
            xml.push_str("<Filter>");
            if let Some(ref p) = filter.prefix {
                write!(xml, "<Prefix>{}</Prefix>", escape_xml(p)).unwrap();
            }
            if let Some(v) = filter.object_size_greater_than {
                write!(xml, "<ObjectSizeGreaterThan>{v}</ObjectSizeGreaterThan>").unwrap();
            }
            if let Some(v) = filter.object_size_less_than {
                write!(xml, "<ObjectSizeLessThan>{v}</ObjectSizeLessThan>").unwrap();
            }
            xml.push_str("</Filter>");
        }

        write!(xml, "<Status>{}</Status>", escape_xml(&rule.status)).unwrap();

        if let Some(ref exp) = rule.expiration {
            xml.push_str("<Expiration>");
            if let Some(ref date) = exp.date {
                write!(xml, "<Date>{}</Date>", escape_xml(date)).unwrap();
            }
            if let Some(days) = exp.days {
                write!(xml, "<Days>{days}</Days>").unwrap();
            }
            if let Some(marker) = exp.expired_object_delete_marker {
                write!(
                    xml,
                    "<ExpiredObjectDeleteMarker>{marker}</ExpiredObjectDeleteMarker>"
                )
                .unwrap();
            }
            xml.push_str("</Expiration>");
        }

        for transition in &rule.transitions {
            xml.push_str("<Transition>");
            if let Some(ref date) = transition.date {
                write!(xml, "<Date>{}</Date>", escape_xml(date)).unwrap();
            }
            if let Some(days) = transition.days {
                write!(xml, "<Days>{days}</Days>").unwrap();
            }
            if let Some(ref sc) = transition.storage_class {
                write!(xml, "<StorageClass>{}</StorageClass>", escape_xml(sc)).unwrap();
            }
            xml.push_str("</Transition>");
        }

        for nvt in &rule.noncurrent_version_transitions {
            xml.push_str("<NoncurrentVersionTransition>");
            if let Some(days) = nvt.noncurrent_days {
                write!(xml, "<NoncurrentDays>{days}</NoncurrentDays>").unwrap();
            }
            if let Some(ref sc) = nvt.storage_class {
                write!(xml, "<StorageClass>{}</StorageClass>", escape_xml(sc)).unwrap();
            }
            if let Some(n) = nvt.newer_noncurrent_versions {
                write!(
                    xml,
                    "<NewerNoncurrentVersions>{n}</NewerNoncurrentVersions>"
                )
                .unwrap();
            }
            xml.push_str("</NoncurrentVersionTransition>");
        }

        if let Some(ref nve) = rule.noncurrent_version_expiration {
            xml.push_str("<NoncurrentVersionExpiration>");
            if let Some(days) = nve.noncurrent_days {
                write!(xml, "<NoncurrentDays>{days}</NoncurrentDays>").unwrap();
            }
            if let Some(n) = nve.newer_noncurrent_versions {
                write!(
                    xml,
                    "<NewerNoncurrentVersions>{n}</NewerNoncurrentVersions>"
                )
                .unwrap();
            }
            xml.push_str("</NoncurrentVersionExpiration>");
        }

        if let Some(ref abort) = rule.abort_incomplete_multipart_upload {
            xml.push_str("<AbortIncompleteMultipartUpload>");
            if let Some(days) = abort.days_after_initiation {
                write!(xml, "<DaysAfterInitiation>{days}</DaysAfterInitiation>").unwrap();
            }
            xml.push_str("</AbortIncompleteMultipartUpload>");
        }

        xml.push_str("</Rule>");
    }

    xml.push_str("</LifecycleConfiguration>");
    xml
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::s3::*;

    fn make_client_with_mock(mock: crate::MockClient) -> crate::AwsHttpClient {
        crate::AwsHttpClient::from_mock(mock)
    }

    #[tokio::test]
    async fn list_buckets_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/")
            .returning_bytes(
                r#"<ListAllMyBucketsResult>
                    <Buckets><Bucket><Name>my-bucket</Name><CreationDate>2026-01-01T00:00:00Z</CreationDate></Bucket></Buckets>
                    <Owner><ID>owner-id</ID></Owner>
                </ListAllMyBucketsResult>"#.as_bytes().to_vec(),
            );
        let client = make_client_with_mock(mock);
        let result = client.s3().list_buckets().await.unwrap();
        assert_eq!(result.buckets.len(), 1);
        assert_eq!(result.buckets[0].name.as_deref(), Some("my-bucket"));
        assert_eq!(
            result.owner.as_ref().unwrap().id.as_deref(),
            Some("owner-id")
        );
    }

    #[tokio::test]
    async fn get_bucket_versioning_returns_status() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/test-bucket?versioning").returning_bytes(
            "<VersioningConfiguration><Status>Enabled</Status></VersioningConfiguration>"
                .as_bytes()
                .to_vec(),
        );
        let client = make_client_with_mock(mock);
        let result = client
            .s3()
            .get_bucket_versioning("test-bucket")
            .await
            .unwrap();
        assert_eq!(result.status.as_deref(), Some("Enabled"));
    }

    #[tokio::test]
    async fn get_bucket_encryption_returns_rules() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/test-bucket?encryption")
            .returning_bytes(
                r#"<ServerSideEncryptionConfiguration>
                    <Rule><ApplyServerSideEncryptionByDefault><SSEAlgorithm>AES256</SSEAlgorithm></ApplyServerSideEncryptionByDefault><BucketKeyEnabled>false</BucketKeyEnabled></Rule>
                </ServerSideEncryptionConfiguration>"#.as_bytes().to_vec(),
            );
        let client = make_client_with_mock(mock);
        let result = client
            .s3()
            .get_bucket_encryption("test-bucket")
            .await
            .unwrap();
        assert_eq!(result.rules.len(), 1);
        assert_eq!(
            result.rules[0]
                .apply_server_side_encryption_by_default
                .as_ref()
                .unwrap()
                .sse_algorithm,
            "AES256"
        );
    }

    #[tokio::test]
    async fn get_bucket_logging_returns_config() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/test-bucket?logging")
            .returning_bytes(
                r#"<BucketLoggingStatus>
                    <LoggingEnabled><TargetBucket>log-bucket</TargetBucket><TargetPrefix>logs/</TargetPrefix></LoggingEnabled>
                </BucketLoggingStatus>"#.as_bytes().to_vec(),
            );
        let client = make_client_with_mock(mock);
        let result = client.s3().get_bucket_logging("test-bucket").await.unwrap();
        let logging = result.logging_enabled.unwrap();
        assert_eq!(logging.target_bucket, "log-bucket");
        assert_eq!(logging.target_prefix, "logs/");
    }

    #[tokio::test]
    async fn get_bucket_acl_returns_grants() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/test-bucket?acl")
            .returning_bytes(
                r#"<AccessControlPolicy>
                    <Owner><ID>owner-id</ID></Owner>
                    <AccessControlList><Grant><Grantee><ID>grantee-id</ID></Grantee><Permission>FULL_CONTROL</Permission></Grant></AccessControlList>
                </AccessControlPolicy>"#.as_bytes().to_vec(),
            );
        let client = make_client_with_mock(mock);
        let result = client.s3().get_bucket_acl("test-bucket").await.unwrap();
        assert_eq!(
            result.owner.as_ref().unwrap().id.as_deref(),
            Some("owner-id")
        );
        assert_eq!(result.grants.len(), 1);
        assert_eq!(result.grants[0].permission.as_deref(), Some("FULL_CONTROL"));
    }

    #[tokio::test]
    async fn get_bucket_lifecycle_configuration_returns_rules() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/test-bucket?lifecycle").returning_bytes(
            r#"<LifecycleConfiguration>
                    <Rule><ID>test-rule</ID><Status>Enabled</Status></Rule>
                </LifecycleConfiguration>"#
                .as_bytes()
                .to_vec(),
        );
        let client = make_client_with_mock(mock);
        let result = client
            .s3()
            .get_bucket_lifecycle_configuration("test-bucket")
            .await
            .unwrap();
        assert_eq!(result.rules.len(), 1);
        assert_eq!(result.rules[0].id.as_deref(), Some("test-rule"));
        assert_eq!(result.rules[0].status, "Enabled");
    }

    #[tokio::test]
    async fn get_public_access_block_returns_config() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/test-bucket?publicAccessBlock")
            .returning_bytes(
                r#"<PublicAccessBlockConfiguration>
                    <BlockPublicAcls>true</BlockPublicAcls>
                    <IgnorePublicAcls>true</IgnorePublicAcls>
                    <BlockPublicPolicy>false</BlockPublicPolicy>
                    <RestrictPublicBuckets>false</RestrictPublicBuckets>
                </PublicAccessBlockConfiguration>"#
                    .as_bytes()
                    .to_vec(),
            );
        let client = make_client_with_mock(mock);
        let result = client
            .s3()
            .get_public_access_block("test-bucket")
            .await
            .unwrap();
        assert_eq!(result.block_public_acls, Some(true));
        assert_eq!(result.ignore_public_acls, Some(true));
        assert_eq!(result.block_public_policy, Some(false));
        assert_eq!(result.restrict_public_buckets, Some(false));
    }

    #[tokio::test]
    async fn get_bucket_policy_returns_parsed_document() {
        let mut mock = crate::MockClient::new();
        let policy_json = r#"{"Version":"2012-10-17","Statement":[]}"#;
        mock.expect_get("/test-bucket?policy")
            .returning_bytes(policy_json.as_bytes().to_vec());
        let client = make_client_with_mock(mock);
        let result = client.s3().get_bucket_policy("test-bucket").await.unwrap();
        assert_eq!(result.version.as_deref(), Some("2012-10-17"));
        assert!(result.statement.is_empty());
    }

    #[tokio::test]
    async fn get_bucket_policy_returns_parsed_statement() {
        let mut mock = crate::MockClient::new();
        let policy_json = r#"{
            "Version": "2012-10-17",
            "Statement": [{
                "Effect": "Allow",
                "Principal": "*",
                "Action": "s3:GetObject",
                "Resource": "arn:aws:s3:::test-bucket/*"
            }]
        }"#;
        mock.expect_get("/test-bucket?policy")
            .returning_bytes(policy_json.as_bytes().to_vec());
        let client = make_client_with_mock(mock);
        let result = client.s3().get_bucket_policy("test-bucket").await.unwrap();
        assert_eq!(result.statement.len(), 1);
        assert_eq!(result.statement[0].effect, crate::iam_policy::Effect::Allow);
        assert_eq!(result.statement[0].action.as_slice(), vec!["s3:GetObject"]);
    }

    #[test]
    fn grantee_type_infers_group_from_uri() {
        let g = Grantee {
            uri: Some("http://acs.amazonaws.com/groups/global/AllUsers".into()),
            id: None,
            display_name: None,
        };
        assert_eq!(g.grantee_type(), "Group");
    }

    #[test]
    fn grantee_type_infers_canonical_user_from_id() {
        let g = Grantee {
            uri: None,
            id: Some("abc123canonical".into()),
            display_name: None,
        };
        assert_eq!(g.grantee_type(), "CanonicalUser");
    }

    #[test]
    fn grantee_type_returns_unknown_when_no_uri_or_id() {
        let g = Grantee {
            uri: None,
            id: None,
            display_name: Some("display".into()),
        };
        assert_eq!(g.grantee_type(), "Unknown");
    }

    #[tokio::test]
    async fn put_bucket_versioning_succeeds() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/test-bucket?versioning")
            .returning_bytes(vec![]);
        let client = make_client_with_mock(mock);
        let config = VersioningConfiguration {
            status: Some("Enabled".into()),
            ..Default::default()
        };
        client
            .s3()
            .put_bucket_versioning("test-bucket", &config)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn put_bucket_encryption_succeeds() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/test-bucket?encryption")
            .returning_bytes(vec![]);
        let client = make_client_with_mock(mock);
        let config = ServerSideEncryptionConfiguration {
            rules: vec![ServerSideEncryptionRule {
                apply_server_side_encryption_by_default: Some(ServerSideEncryptionByDefault {
                    sse_algorithm: "AES256".into(),
                    kms_master_key_id: None,
                }),
                bucket_key_enabled: Some(false),
            }],
        };
        client
            .s3()
            .put_bucket_encryption("test-bucket", &config)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn put_bucket_logging_succeeds() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/test-bucket?logging")
            .returning_bytes(vec![]);
        let client = make_client_with_mock(mock);
        let logging = BucketLoggingStatus {
            logging_enabled: Some(LoggingEnabled {
                target_bucket: "log-bucket".into(),
                target_prefix: "logs/".into(),
            }),
        };
        client
            .s3()
            .put_bucket_logging("test-bucket", &logging)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn delete_bucket_lifecycle_configuration_succeeds() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/test-bucket?lifecycle")
            .returning_bytes(vec![]);
        let client = make_client_with_mock(mock);
        client
            .s3()
            .delete_bucket_lifecycle_configuration("test-bucket")
            .await
            .unwrap();
    }

    #[test]
    fn lifecycle_xml_matches_s3_format() {
        let lifecycle = BucketLifecycleConfiguration {
            rules: vec![LifecycleRule {
                id: Some("test-rule".into()),
                status: "Enabled".into(),
                filter: Some(LifecycleRuleFilter {
                    prefix: Some("logs/".into()),
                    ..Default::default()
                }),
                expiration: Some(LifecycleExpiration {
                    days: Some(30),
                    ..Default::default()
                }),
                ..Default::default()
            }],
        };
        let xml = build_lifecycle_xml(&lifecycle);

        assert!(xml.starts_with(
            r#"<LifecycleConfiguration xmlns="http://s3.amazonaws.com/doc/2006-03-01/">"#
        ));
        assert!(xml.contains("<Rule>"));
        assert!(xml.contains("<ID>test-rule</ID>"));
        assert!(xml.contains("<Filter><Prefix>logs/</Prefix></Filter>"));
        assert!(xml.contains("<Status>Enabled</Status>"));
        assert!(xml.contains("<Expiration><Days>30</Days></Expiration>"));
        assert!(xml.ends_with("</LifecycleConfiguration>"));
    }
}
