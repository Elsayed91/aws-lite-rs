//! Types for the Amazon S3 API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// The PublicAccessBlock configuration that you want to apply to this Amazon S3 bucket. You can
/// enable the configuration options in any combination. Bucket-level settings work alongside
/// account-level settings (which may inherit from organization-level policies). For more
/// information about when Amazon S3 considers a bucket or object public, see The Meaning of
/// "Public" in the Amazon S3 User Guide.
///
/// **AWS API**: `s3.v1.PublicAccessBlockConfiguration`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicAccessBlockConfiguration {
    /// Specifies whether Amazon S3 should block public access control lists (ACLs) for this
    /// bucket and objects in this bucket. Setting this element to TRUE causes the following
    /// behavior: PUT Bucket ACL and PUT Object ACL calls fail if the specified ACL is public.
    /// PUT Object calls fail if the request includes a public ACL. PUT Bucket calls fail if the
    /// request includes a public ACL. Enabling this setting doesn't affect existing policies or
    /// ACLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_acls: Option<bool>,

    /// Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in
    /// this bucket. Setting this element to TRUE causes Amazon S3 to ignore all public ACLs on
    /// this bucket and objects in this bucket. Enabling this setting doesn't affect the
    /// persistence of any existing ACLs and doesn't prevent new public ACLs from being set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_public_acls: Option<bool>,

    /// Specifies whether Amazon S3 should block public bucket policies for this bucket. Setting
    /// this element to TRUE causes Amazon S3 to reject calls to PUT Bucket policy if the
    /// specified bucket policy allows public access. Enabling this setting doesn't affect
    /// existing bucket policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<bool>,

    /// Specifies whether Amazon S3 should restrict public bucket policies for this bucket.
    /// Setting this element to TRUE restricts access to this bucket to only Amazon Web Services
    /// service principals and authorized users within this account if the bucket has a public
    /// policy. Enabling this setting doesn't affect previously stored bucket policies, except
    /// that public and cross-account access within any public bucket policy, including non-
    /// public delegation to specific accounts, is blocked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_public_buckets: Option<bool>,
}

impl PublicAccessBlockConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            block_public_acls: Some(false),
            ignore_public_acls: Some(false),
            block_public_policy: Some(false),
            restrict_public_buckets: Some(false),
        }
    }
}

/// Specifies the lifecycle configuration for objects in an Amazon S3 bucket. For more
/// information, see Object Lifecycle Management in the Amazon S3 User Guide.
///
/// **AWS API**: `s3.v1.BucketLifecycleConfiguration`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketLifecycleConfiguration {
    /// A lifecycle rule for individual objects in an Amazon S3 bucket.
    #[serde(default)]
    pub rules: Vec<LifecycleRule>,
}

impl BucketLifecycleConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { rules: vec![] }
    }
}

/// The Filter is used to identify objects that a Lifecycle Rule applies to. A Filter can have
/// exactly one of Prefix, Tag, ObjectSizeGreaterThan, ObjectSizeLessThan, or And specified. If
/// the Filter element is left empty, the Lifecycle Rule applies to all objects in the bucket.
///
/// **AWS API**: `s3.v1.LifecycleRuleFilter`
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `Tag` — not selected in manifest
/// - `And` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleRuleFilter {
    /// Prefix identifying one or more objects to which the rule applies. Replacement must be
    /// made for object keys containing special characters (such as carriage returns) when using
    /// XML requests. For more information, see XML related object key constraints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// Minimum object size to which the rule applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size_greater_than: Option<i64>,

    /// Maximum object size to which the rule applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size_less_than: Option<i64>,
}

impl LifecycleRuleFilter {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            prefix: Some("test-prefix".into()),
            object_size_greater_than: Some(100),
            object_size_less_than: Some(100),
        }
    }
}

/// A lifecycle rule for individual objects in an Amazon S3 bucket. For more information see,
/// Managing your storage lifecycle in the Amazon S3 User Guide.
///
/// **AWS API**: `s3.v1.LifecycleRule`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleRule {
    /// Specifies the expiration for the lifecycle of the object in the form of date, days and,
    /// whether the object has a delete marker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<LifecycleExpiration>,

    /// Unique identifier for the rule. The value cannot be longer than 255 characters.
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The general purpose bucket prefix that identifies one or more objects to which the rule
    /// applies. We recommend using Filter instead of Prefix for new PUTs. Previous
    /// configurations where a prefix is defined will continue to operate as before. Replacement
    /// must be made for object keys containing special characters (such as carriage returns)
    /// when using XML requests. For more information, see XML related object key constraints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// The Filter is used to identify objects that a Lifecycle Rule applies to. A Filter must
    /// have exactly one of Prefix, Tag, ObjectSizeGreaterThan, ObjectSizeLessThan, or And
    /// specified. Filter is required if the LifecycleRule does not contain a Prefix element.
    /// For more information about Tag filters, see Adding filters to Lifecycle rules in the
    /// Amazon S3 User Guide. Tag filters are not supported for directory buckets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<LifecycleRuleFilter>,

    /// If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not
    /// currently being applied.
    pub status: String,

    /// Specifies when an Amazon S3 object transitions to a specified storage class. This
    /// parameter applies to general purpose buckets only. It is not supported for directory
    /// bucket lifecycle configurations.
    #[serde(rename = "Transition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transitions: Vec<Transition>,

    /// Specifies the transition rule for the lifecycle rule that describes when noncurrent
    /// objects transition to a specific storage class. If your bucket is versioning-enabled (or
    /// versioning is suspended), you can set this action to request that Amazon S3 transition
    /// noncurrent object versions to a specific storage class at a set period in the object's
    /// lifetime. This parameter applies to general purpose buckets only. It is not supported
    /// for directory bucket lifecycle configurations.
    #[serde(rename = "NoncurrentVersionTransition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub noncurrent_version_transitions: Vec<NoncurrentVersionTransition>,

    /// The `NoncurrentVersionExpiration` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,

    /// The `AbortIncompleteMultipartUpload` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
}

impl LifecycleRule {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            expiration: Some(LifecycleExpiration::fixture()),
            id: Some("test-id".into()),
            prefix: Some("test-prefix".into()),
            filter: Some(LifecycleRuleFilter::fixture()),
            status: "test-status".into(),
            transitions: vec![],
            noncurrent_version_transitions: vec![],
            noncurrent_version_expiration: Some(NoncurrentVersionExpiration::fixture()),
            abort_incomplete_multipart_upload: Some(AbortIncompleteMultipartUpload::fixture()),
        }
    }
}

///
/// **AWS API**: `s3.v1.ListBucketsOutput`
///
/// ## Coverage
/// 2 of 4 fields included.
/// Omitted fields:
/// - `ContinuationToken` — not selected in manifest
/// - `Prefix` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBucketsResponse {
    /// The list of buckets owned by the requester.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub buckets: Vec<Bucket>,

    /// The owner of the buckets listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
}

impl ListBucketsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            buckets: vec![],
            owner: Some(Owner::fixture()),
        }
    }
}

/// In terms of implementation, a Bucket is a resource.
///
/// **AWS API**: `s3.v1.Bucket`
///
/// ## Coverage
/// 2 of 4 fields included.
/// Omitted fields:
/// - `BucketRegion` — not selected in manifest
/// - `BucketArn` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bucket {
    /// The name of the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Date the bucket was created. This date can change when making changes to your bucket,
    /// such as editing its bucket policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
}

impl Bucket {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-name".into()),
            creation_date: Some("test-creation_date".into()),
        }
    }
}

/// Container for the owner's display name and ID.
///
/// **AWS API**: `s3.v1.Owner`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    /// The `DisplayName` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Container for the ID of the owner.
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Owner {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            display_name: Some("test-display_name".into()),
            id: Some("test-id".into()),
        }
    }
}

///
/// **AWS API**: `s3.v1.GetBucketVersioningOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBucketVersioningResponse {
    /// The versioning state of the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Specifies whether MFA delete is enabled in the bucket versioning configuration. This
    /// element is only returned if the bucket has been configured with MFA delete. If the
    /// bucket has never been so configured, this element is not returned.
    #[serde(rename = "MfaDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_delete: Option<String>,
}

impl GetBucketVersioningResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            status: Some("test-status".into()),
            mfa_delete: Some("test-mfa_delete".into()),
        }
    }
}

///
/// **AWS API**: `s3.v1.GetBucketEncryptionOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBucketEncryptionResponse {
    /// The `ServerSideEncryptionConfiguration` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

impl GetBucketEncryptionResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            server_side_encryption_configuration: Some(ServerSideEncryptionConfiguration::fixture()),
        }
    }
}

/// Specifies the default server-side-encryption configuration.
///
/// **AWS API**: `s3.v1.ServerSideEncryptionConfiguration`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerSideEncryptionConfiguration {
    /// Container for information about a particular server-side encryption configuration rule.
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rules: Vec<ServerSideEncryptionRule>,
}

impl ServerSideEncryptionConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { rules: vec![] }
    }
}

/// Specifies the default server-side encryption configuration. General purpose buckets
/// - If you're specifying a customer managed KMS key, we recommend using a fully qualified KMS
///   key ARN. If you use a KMS key alias instead, then KMS resolves the key within the
///   requester’s account. This behavior can result in data that's encrypted with a KMS key that
///   belongs to the requester, and not the bucket owner. Directory buckets
/// - When you specify an KMS customer managed key for encryption in your directory bucket, only
///   use the key ID or key ARN. The key alias format of the KMS key isn't supported.
///
/// **AWS API**: `s3.v1.ServerSideEncryptionRule`
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `BlockedEncryptionTypes` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerSideEncryptionRule {
    /// Specifies the default server-side encryption to apply to new objects in the bucket. If a
    /// PUT Object request doesn't specify any server-side encryption, this default encryption
    /// will be applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,

    /// Specifies whether Amazon S3 should use an S3 Bucket Key with server-side encryption
    /// using KMS (SSE-KMS) for new objects in the bucket. Existing objects are not affected.
    /// Setting the BucketKeyEnabled element to true causes Amazon S3 to use an S3 Bucket Key.
    /// General purpose buckets
    /// - By default, S3 Bucket Key is not enabled. For more information, see Amazon S3 Bucket
    ///   Keys in the Amazon S3 User Guide. Directory buckets
    /// - S3 Bucket Keys are always enabled for GET and PUT operations in a directory bucket and
    ///   can’t be disabled. S3 Bucket Keys aren't supported, when you copy SSE-KMS encrypted
    ///   objects from general purpose buckets to directory buckets, from directory buckets to
    ///   general purpose buckets, or between directory buckets, through CopyObject,
    ///   UploadPartCopy, the Copy operation in Batch Operations, or the import jobs. In this
    ///   case, Amazon S3 makes a call to KMS every time a copy request is made for a KMS-
    ///   encrypted object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
}

impl ServerSideEncryptionRule {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            apply_server_side_encryption_by_default: Some(ServerSideEncryptionByDefault::fixture()),
            bucket_key_enabled: Some(false),
        }
    }
}

/// Describes the default server-side encryption to apply to new objects in the bucket. If a PUT
/// Object request doesn't specify any server-side encryption, this default encryption will be
/// applied. For more information, see PutBucketEncryption. General purpose buckets
/// - If you don't specify a customer managed key at configuration, Amazon S3 automatically
///   creates an Amazon Web Services KMS key (aws/s3) in your Amazon Web Services account the
///   first time that you add an object encrypted with SSE-KMS to a bucket. By default, Amazon
///   S3 uses this KMS key for SSE-KMS. Directory buckets
/// - Your SSE-KMS configuration can only support 1 customer managed key per directory bucket's
///   lifetime. The Amazon Web Services managed key (aws/s3) isn't supported. Directory buckets
/// - For directory buckets, there are only two supported options for server-side encryption:
///   SSE-S3 and SSE-KMS.
///
/// **AWS API**: `s3.v1.ServerSideEncryptionByDefault`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerSideEncryptionByDefault {
    /// Server-side encryption algorithm to use for the default encryption. For directory
    /// buckets, there are only two supported values for server-side encryption: AES256 and
    /// aws:kms.
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: String,

    /// Amazon Web Services Key Management Service (KMS) customer managed key ID to use for the
    /// default encryption. General purpose buckets
    /// - This parameter is allowed if and only if SSEAlgorithm is set to aws:kms or
    ///   aws:kms:dsse. Directory buckets
    /// - This parameter is allowed if and only if SSEAlgorithm is set to aws:kms. You can
    ///   specify the key ID, key alias, or the Amazon Resource Name (ARN) of the KMS key. Key
    ///   ID: 1234abcd-12ab-34cd-56ef-1234567890ab Key ARN: arn:aws:kms:us-
    ///   east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab Key Alias: alias/alias-
    ///   name If you are using encryption with cross-account or Amazon Web Services service
    ///   operations, you must use a fully qualified KMS key ARN. For more information, see
    ///   Using encryption for cross-account operations. General purpose buckets
    /// - If you're specifying a customer managed KMS key, we recommend using a fully qualified
    ///   KMS key ARN. If you use a KMS key alias instead, then KMS resolves the key within the
    ///   requester’s account. This behavior can result in data that's encrypted with a KMS key
    ///   that belongs to the requester, and not the bucket owner. Also, if you use a key ID,
    ///   you can run into a LogDestination undeliverable error when creating a VPC flow log.
    ///   Directory buckets
    /// - When you specify an KMS customer managed key for encryption in your directory bucket,
    ///   only use the key ID or key ARN. The key alias format of the KMS key isn't supported.
    ///   Amazon S3 only supports symmetric encryption KMS keys. For more information, see
    ///   Asymmetric keys in Amazon Web Services KMS in the Amazon Web Services Key Management
    ///   Service Developer Guide.
    #[serde(rename = "KMSMasterKeyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
}

impl ServerSideEncryptionByDefault {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            sse_algorithm: "test-sse_algorithm".into(),
            kms_master_key_id: Some("test-kms_master_key_id".into()),
        }
    }
}

///
/// **AWS API**: `s3.v1.GetBucketLoggingOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBucketLoggingResponse {
    /// The `LoggingEnabled` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_enabled: Option<LoggingEnabled>,
}

impl GetBucketLoggingResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            logging_enabled: Some(LoggingEnabled::fixture()),
        }
    }
}

/// Describes where logs are stored and the prefix that Amazon S3 assigns to all log object keys
/// for a bucket. For more information, see PUT Bucket logging in the Amazon S3 API Reference.
///
/// **AWS API**: `s3.v1.LoggingEnabled`
///
/// ## Coverage
/// 2 of 4 fields included.
/// Omitted fields:
/// - `TargetGrants` — not selected in manifest
/// - `TargetObjectKeyFormat` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoggingEnabled {
    /// Specifies the bucket where you want Amazon S3 to store server access logs. You can have
    /// your logs delivered to any bucket that you own, including the same bucket that is being
    /// logged. You can also configure multiple buckets to deliver their logs to the same target
    /// bucket. In this case, you should choose a different TargetPrefix for each source bucket
    /// so that the delivered log files can be distinguished by key.
    pub target_bucket: String,

    /// A prefix for all log object keys. If you store log files from multiple Amazon S3 buckets
    /// in a single bucket, you can use a prefix to distinguish which log files came from which
    /// bucket.
    pub target_prefix: String,
}

impl LoggingEnabled {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            target_bucket: "test-target_bucket".into(),
            target_prefix: "test-target_prefix".into(),
        }
    }
}

///
/// **AWS API**: `s3.v1.GetBucketAclOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBucketAclResponse {
    /// Container for the bucket owner's ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,

    /// A list of grants.
    #[serde(rename = "AccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub grants: Vec<Grant>,
}

impl GetBucketAclResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            owner: Some(Owner::fixture()),
            grants: vec![],
        }
    }
}

/// Container for grant information.
///
/// **AWS API**: `s3.v1.Grant`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Grant {
    /// The person being granted permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<Grantee>,

    /// Specifies the permission given to the grantee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

impl Grant {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            grantee: Some(Grantee::fixture()),
            permission: Some("test-permission".into()),
        }
    }
}

/// Container for the person being granted permissions.
///
/// **AWS API**: `s3.v1.Grantee`
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `EmailAddress` — not selected in manifest
/// - `Type` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Grantee {
    /// The `DisplayName` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The canonical user ID of the grantee.
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// URI of the grantee group.
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl Grantee {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            display_name: Some("test-display_name".into()),
            id: Some("test-id".into()),
            uri: Some("test-uri".into()),
        }
    }
}

///
/// **AWS API**: `s3.v1.GetBucketLifecycleConfigurationOutput`
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `TransitionDefaultMinimumObjectSize` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBucketLifecycleConfigurationResponse {
    /// Container for a lifecycle rule.
    #[serde(rename = "Rule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<LifecycleRule>,
}

impl GetBucketLifecycleConfigurationResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { rules: vec![] }
    }
}

///
/// **AWS API**: `s3.v1.GetPublicAccessBlockOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetPublicAccessBlockResponse {
    /// The PublicAccessBlock configuration currently in effect for this Amazon S3 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

impl GetPublicAccessBlockResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            public_access_block_configuration: Some(PublicAccessBlockConfiguration::fixture()),
        }
    }
}

/// Describes the versioning state of an Amazon S3 bucket. For more information, see PUT Bucket
/// versioning in the Amazon S3 API Reference.
///
/// **AWS API**: `s3.v1.VersioningConfiguration`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VersioningConfiguration {
    /// Specifies whether MFA delete is enabled in the bucket versioning configuration. This
    /// element is only returned if the bucket has been configured with MFA delete. If the
    /// bucket has never been so configured, this element is not returned.
    #[serde(rename = "MfaDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_delete: Option<String>,

    /// The versioning state of the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl VersioningConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            mfa_delete: Some("test-mfa_delete".into()),
            status: Some("test-status".into()),
        }
    }
}

/// Container for logging status information.
///
/// **AWS API**: `s3.v1.BucketLoggingStatus`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketLoggingStatus {
    /// The `LoggingEnabled` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_enabled: Option<LoggingEnabled>,
}

impl BucketLoggingStatus {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            logging_enabled: Some(LoggingEnabled::fixture()),
        }
    }
}

// ======================================================================
// Auto-generated dependency types (referenced via $ref)
// ======================================================================

/// Specifies the days since the initiation of an incomplete multipart upload that Amazon S3
/// will wait before permanently removing all parts of the upload. For more information, see
/// Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration in the Amazon
/// S3 User Guide.
///
/// **AWS API**: `s3.v1.AbortIncompleteMultipartUpload`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbortIncompleteMultipartUpload {
    /// Specifies the number of days after which Amazon S3 aborts an incomplete multipart
    /// upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_after_initiation: Option<i32>,
}

impl AbortIncompleteMultipartUpload {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            days_after_initiation: Some(100),
        }
    }
}

/// Container for the expiration for the lifecycle of the object. For more information see,
/// Managing your storage lifecycle in the Amazon S3 User Guide.
///
/// **AWS API**: `s3.v1.LifecycleExpiration`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleExpiration {
    /// Indicates at what date the object is to be moved or deleted. The date value must conform
    /// to the ISO 8601 format. The time is always midnight UTC. This parameter applies to
    /// general purpose buckets only. It is not supported for directory bucket lifecycle
    /// configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    /// Indicates the lifetime, in days, of the objects that are subject to the rule. The value
    /// must be a non-zero positive integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,

    /// Indicates whether Amazon S3 will remove a delete marker with no noncurrent versions. If
    /// set to true, the delete marker will be expired; if set to false the policy takes no
    /// action. This cannot be specified with Days or Date in a Lifecycle Expiration Policy.
    /// This parameter applies to general purpose buckets only. It is not supported for
    /// directory bucket lifecycle configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_object_delete_marker: Option<bool>,
}

impl LifecycleExpiration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            date: Some("test-date".into()),
            days: Some(100),
            expired_object_delete_marker: Some(false),
        }
    }
}

/// Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently
/// deletes the noncurrent object versions. You set this lifecycle configuration action on a
/// bucket that has versioning enabled (or suspended) to request that Amazon S3 delete
/// noncurrent object versions at a specific period in the object's lifetime. This parameter
/// applies to general purpose buckets only. It is not supported for directory bucket lifecycle
/// configurations.
///
/// **AWS API**: `s3.v1.NoncurrentVersionExpiration`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NoncurrentVersionExpiration {
    /// Specifies the number of days an object is noncurrent before Amazon S3 can perform the
    /// associated action. The value must be a non-zero positive integer. For information about
    /// the noncurrent days calculations, see How Amazon S3 Calculates When an Object Became
    /// Noncurrent in the Amazon S3 User Guide. This parameter applies to general purpose
    /// buckets only. It is not supported for directory bucket lifecycle configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_days: Option<i32>,

    /// Specifies how many noncurrent versions Amazon S3 will retain. You can specify up to 100
    /// noncurrent versions to retain. Amazon S3 will permanently delete any additional
    /// noncurrent versions beyond the specified number to retain. For more information about
    /// noncurrent versions, see Lifecycle configuration elements in the Amazon S3 User Guide.
    /// This parameter applies to general purpose buckets only. It is not supported for
    /// directory bucket lifecycle configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newer_noncurrent_versions: Option<i32>,
}

impl NoncurrentVersionExpiration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            noncurrent_days: Some(100),
            newer_noncurrent_versions: Some(100),
        }
    }
}

/// Container for the transition rule that describes when noncurrent objects transition to the
/// STANDARD_IA, ONEZONE_IA, INTELLIGENT_TIERING, GLACIER_IR, GLACIER, or DEEP_ARCHIVE storage
/// class. If your bucket is versioning-enabled (or versioning is suspended), you can set this
/// action to request that Amazon S3 transition noncurrent object versions to the STANDARD_IA,
/// ONEZONE_IA, INTELLIGENT_TIERING, GLACIER_IR, GLACIER, or DEEP_ARCHIVE storage class at a
/// specific period in the object's lifetime.
///
/// **AWS API**: `s3.v1.NoncurrentVersionTransition`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NoncurrentVersionTransition {
    /// Specifies the number of days an object is noncurrent before Amazon S3 can perform the
    /// associated action. For information about the noncurrent days calculations, see How
    /// Amazon S3 Calculates How Long an Object Has Been Noncurrent in the Amazon S3 User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_days: Option<i32>,

    /// The class of storage used to store the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,

    /// Specifies how many noncurrent versions Amazon S3 will retain in the same storage class
    /// before transitioning objects. You can specify up to 100 noncurrent versions to retain.
    /// Amazon S3 will transition any additional noncurrent versions beyond the specified number
    /// to retain. For more information about noncurrent versions, see Lifecycle configuration
    /// elements in the Amazon S3 User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newer_noncurrent_versions: Option<i32>,
}

impl NoncurrentVersionTransition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            noncurrent_days: Some(100),
            storage_class: Some("test-storage_class".into()),
            newer_noncurrent_versions: Some(100),
        }
    }
}

/// Specifies when an object transitions to a specified storage class. For more information
/// about Amazon S3 lifecycle configuration rules, see Transitioning Objects Using Amazon S3
/// Lifecycle in the Amazon S3 User Guide.
///
/// **AWS API**: `s3.v1.Transition`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transition {
    /// Indicates when objects are transitioned to the specified storage class. The date value
    /// must be in ISO 8601 format. The time is always midnight UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    /// Indicates the number of days after creation when objects are transitioned to the
    /// specified storage class. If the specified storage class is INTELLIGENT_TIERING,
    /// GLACIER_IR, GLACIER, or DEEP_ARCHIVE, valid values are 0 or positive integers. If the
    /// specified storage class is STANDARD_IA or ONEZONE_IA, valid values are positive integers
    /// greater than 30. Be aware that some storage classes have a minimum storage duration and
    /// that you're charged for transitioning objects before their minimum storage duration. For
    /// more information, see Constraints and considerations for transitions in the Amazon S3
    /// User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,

    /// The storage class to which you want the object to transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

impl Transition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            date: Some("test-date".into()),
            days: Some(100),
            storage_class: Some("test-storage_class".into()),
        }
    }
}
