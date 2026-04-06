//! Types for the AWS Key Management Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `kms.v1.ListKeysRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListKeysRequest {
    /// Use this parameter to specify the maximum number of items to return. When this value is
    /// present, KMS does not return more than the specified number of items, but it might
    /// return fewer. This value is optional. If you include a value, it must be between 1 and
    /// 1000, inclusive. If you do not include a value, it defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Use this parameter in a subsequent request after you receive a response with truncated
    /// results. Set it to the value of NextMarker from the truncated response you just
    /// received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListKeysRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            limit: Some(100),
            marker: Some("test-marker".into()),
        }
    }
}

///
/// **AWS API**: `kms.v1.ListKeysResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListKeysResponse {
    /// A list of KMS keys.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<KeyListEntry>,

    /// When Truncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,

    /// A flag that indicates whether there are more items in the list. When this value is true,
    /// the list in this response is truncated. To get more items, pass the value of the
    /// NextMarker element in this response to the Marker parameter in a subsequent request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

impl ListKeysResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            keys: vec![],
            next_marker: Some("test-next_marker".into()),
            truncated: Some(false),
        }
    }
}

/// Contains information about each entry in the key list.
///
/// **AWS API**: `kms.v1.KeyListEntry`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeyListEntry {
    /// Unique identifier of the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,

    /// ARN of the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
}

impl KeyListEntry {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_id: Some("test-key_id".into()),
            key_arn: Some("test-key_arn".into()),
        }
    }
}

///
/// **AWS API**: `kms.v1.DescribeKeyRequest`
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `GrantTokens` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeKeyRequest {
    /// Describes the specified KMS key. If you specify a predefined Amazon Web Services alias
    /// (an Amazon Web Services alias with no key ID), KMS associates the alias with an Amazon
    /// Web Services managed key and returns its KeyId and Arn in the response. To specify a KMS
    /// key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix
    /// it with "alias/". To specify a KMS key in a different Amazon Web Services account, you
    /// must use the key ARN or alias ARN. For example: Key ID:
    /// 1234abcd-12ab-34cd-56ef-1234567890ab Key ARN: arn:aws:kms:us-
    /// east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab Alias name:
    /// alias/ExampleAlias Alias ARN: arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias To
    /// get the key ID and key ARN for a KMS key, use ListKeys or DescribeKey. To get the alias
    /// name and alias ARN, use ListAliases.
    pub key_id: String,
}

impl DescribeKeyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_id: "test-key_id".into(),
        }
    }
}

///
/// **AWS API**: `kms.v1.DescribeKeyResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeKeyResponse {
    /// Metadata associated with the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<KeyMetadata>,
}

impl DescribeKeyResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_metadata: Some(KeyMetadata::fixture()),
        }
    }
}

/// Contains metadata about a KMS key. This data type is used as a response element for the
/// CreateKey, DescribeKey, and ReplicateKey operations.
///
/// **AWS API**: `kms.v1.KeyMetadata`
///
/// ## Coverage
/// 11 of 26 fields included.
/// Omitted fields:
/// - `AWSAccountId` — not selected in manifest
/// - `ValidTo` — not selected in manifest
/// - `Origin` — not selected in manifest
/// - `CustomKeyStoreId` — not selected in manifest
/// - `CloudHsmClusterId` — not selected in manifest
/// - `ExpirationModel` — not selected in manifest
/// - `CustomerMasterKeySpec` — not selected in manifest
/// - `EncryptionAlgorithms` — not selected in manifest
/// - `SigningAlgorithms` — not selected in manifest
/// - `KeyAgreementAlgorithms` — not selected in manifest
/// - `MultiRegionConfiguration` — not selected in manifest
/// - `PendingDeletionWindowInDays` — not selected in manifest
/// - `MacAlgorithms` — not selected in manifest
/// - `XksKeyConfiguration` — not selected in manifest
/// - `CurrentKeyMaterialId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeyMetadata {
    /// The globally unique identifier for the KMS key.
    pub key_id: String,

    /// The Amazon Resource Name (ARN) of the KMS key. For examples, see Key Management Service
    /// (KMS) in the Example ARNs section of the Amazon Web Services General Reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The date and time when the KMS key was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,

    /// Specifies whether the KMS key is enabled. When KeyState is Enabled this value is true,
    /// otherwise it is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The description of the KMS key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The cryptographic operations for which you can use the KMS key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,

    /// The current status of the KMS key. For more information about how key state affects the
    /// use of a KMS key, see Key states of KMS keys in the Key Management Service Developer
    /// Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,

    /// The date and time after which KMS deletes this KMS key. This value is present only when
    /// the KMS key is scheduled for deletion, that is, when its KeyState is PendingDeletion.
    /// When the primary key in a multi-Region key is scheduled for deletion but still has
    /// replica keys, its key state is PendingReplicaDeletion and the length of its waiting
    /// period is displayed in the PendingDeletionWindowInDays field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,

    /// The manager of the KMS key. KMS keys in your Amazon Web Services account are either
    /// customer managed or Amazon Web Services managed. For more information about the
    /// difference, see KMS keys in the Key Management Service Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_manager: Option<String>,

    /// Describes the type of key material in the KMS key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,

    /// Indicates whether the KMS key is a multi-Region (True) or regional (False) key. This
    /// value is True for multi-Region primary and replica keys and False for regional KMS keys.
    /// For more information about multi-Region keys, see Multi-Region keys in KMS in the Key
    /// Management Service Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region: Option<bool>,
}

impl KeyMetadata {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_id: "test-key_id".into(),
            arn: Some("test-arn".into()),
            enabled: Some(false),
            description: Some("test-description".into()),
            key_usage: Some("test-key_usage".into()),
            key_state: Some("test-key_state".into()),
            key_manager: Some("test-key_manager".into()),
            key_spec: Some("test-key_spec".into()),
            multi_region: Some(false),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `kms.v1.GetKeyRotationStatusRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetKeyRotationStatusRequest {
    /// Gets the rotation status for the specified KMS key. Specify the key ID or key ARN of the
    /// KMS key. To specify a KMS key in a different Amazon Web Services account, you must use
    /// the key ARN. For example: Key ID: 1234abcd-12ab-34cd-56ef-1234567890ab Key ARN:
    /// arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab To get the
    /// key ID and key ARN for a KMS key, use ListKeys or DescribeKey.
    pub key_id: String,
}

impl GetKeyRotationStatusRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_id: "test-key_id".into(),
        }
    }
}

///
/// **AWS API**: `kms.v1.GetKeyRotationStatusResponse`
///
/// ## Coverage
/// 4 of 5 fields included.
/// Omitted fields:
/// - `OnDemandRotationStartDate` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetKeyRotationStatusResponse {
    /// A Boolean value that specifies whether key rotation is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_enabled: Option<bool>,

    /// Identifies the specified symmetric encryption KMS key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,

    /// The number of days between each automatic rotation. The default value is 365 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_period_in_days: Option<i32>,

    /// The next date that KMS will automatically rotate the key material.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_date: Option<f64>,
}

impl GetKeyRotationStatusResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_rotation_enabled: Some(false),
            key_id: Some("test-key_id".into()),
            rotation_period_in_days: Some(100),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `kms.v1.EnableKeyRotationRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnableKeyRotationRequest {
    /// Identifies a symmetric encryption KMS key. You cannot enable automatic rotation of
    /// asymmetric KMS keys, HMAC KMS keys, KMS keys with imported key material, or KMS keys in
    /// a custom key store. To enable or disable automatic rotation of a set of related multi-
    /// Region keys, set the property on the primary key. Specify the key ID or key ARN of the
    /// KMS key. For example: Key ID: 1234abcd-12ab-34cd-56ef-1234567890ab Key ARN:
    /// arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab To get the
    /// key ID and key ARN for a KMS key, use ListKeys or DescribeKey.
    pub key_id: String,

    /// Use this parameter to specify a custom period of time between each rotation date. If no
    /// value is specified, the default value is 365 days. The rotation period defines the
    /// number of days after you enable automatic key rotation that KMS will rotate your key
    /// material, and the number of days between each automatic rotation thereafter. You can use
    /// the kms:RotationPeriodInDays condition key to further constrain the values that
    /// principals can specify in the RotationPeriodInDays parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_period_in_days: Option<i32>,
}

impl EnableKeyRotationRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_id: "test-key_id".into(),
            rotation_period_in_days: Some(100),
        }
    }
}
