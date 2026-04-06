//! Types for the Amazon Elastic File System API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `efs.v1.DescribeFileSystemsRequest`
/// **Reference**: <https://docs.aws.amazon.com/efs/latest/ug/API_Operations.html/DescribeFileSystemsRequest>
///
/// ## Coverage
/// 3 of 4 fields included.
/// Omitted fields:
/// - `CreationToken` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeFileSystemsRequest {
    /// (Optional) Specifies the maximum number of file systems to return in the response
    /// (integer). This number is automatically set to 100. The response is paginated at 100 per
    /// page if you have more than 100 file systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,

    /// (Optional) Opaque pagination token returned from a previous DescribeFileSystems
    /// operation (String). If present, specifies to continue the list from where the returning
    /// call had left off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// (Optional) ID of the file system whose description you want to retrieve (String).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
}

impl DescribeFileSystemsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            max_items: Some(100),
            marker: Some("test-marker".into()),
            file_system_id: Some("test-file_system_id".into()),
        }
    }
}

///
/// **AWS API**: `efs.v1.DescribeFileSystemsResponse`
/// **Reference**: <https://docs.aws.amazon.com/efs/latest/ug/API_Operations.html/DescribeFileSystemsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeFileSystemsResponse {
    /// An array of file system descriptions.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_systems: Vec<FileSystemDescription>,

    /// Present if there are more file systems than returned in the response (String). You can
    /// use the NextMarker in the subsequent request to fetch the descriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,

    /// Present if provided by caller in the request (String).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeFileSystemsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            file_systems: vec![],
            next_marker: Some("test-next_marker".into()),
            marker: Some("test-marker".into()),
        }
    }
}

/// A description of the file system.
///
/// **AWS API**: `efs.v1.FileSystemDescription`
/// **Reference**: <https://docs.aws.amazon.com/efs/latest/ug/API_Operations.html/FileSystemDescription>
///
/// ## Coverage
/// 12 of 18 fields included.
/// Omitted fields:
/// - `CreationToken` — not selected in manifest
/// - `SizeInBytes` — not selected in manifest
/// - `ProvisionedThroughputInMibps` — not selected in manifest
/// - `AvailabilityZoneName` — not selected in manifest
/// - `AvailabilityZoneId` — not selected in manifest
/// - `FileSystemProtection` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileSystemDescription {
    /// The Amazon Web Services account that created the file system.
    pub owner_id: String,

    /// The ID of the file system, assigned by Amazon EFS.
    pub file_system_id: String,

    /// The Amazon Resource Name (ARN) for the EFS file system, in the format
    /// arn:aws:elasticfilesystem:region:account-id:file-system/file-system-id . Example with
    /// sample data: arn:aws:elasticfilesystem:us-west-2:1111333322228888:file-
    /// system/fs-01234567
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_arn: Option<String>,

    /// The time that the file system was created, in seconds (since 1970-01-01T00:00:00Z).
    pub creation_time: String,

    /// The lifecycle phase of the file system.
    pub life_cycle_state: String,

    /// You can add tags to a file system, including a Name tag. For more information, see
    /// CreateFileSystem. If the file system has a Name tag, Amazon EFS returns the value in
    /// this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The current number of mount targets that the file system has. For more information, see
    /// CreateMountTarget.
    pub number_of_mount_targets: i32,

    /// The performance mode of the file system.
    pub performance_mode: String,

    /// A Boolean value that, if true, indicates that the file system is encrypted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,

    /// The ID of an KMS key used to protect the encrypted file system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    /// Displays the file system's throughput mode. For more information, see Throughput modes
    /// in the Amazon EFS User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,

    /// The tags associated with the file system, presented as an array of Tag objects.
    #[serde(default)]
    pub tags: Vec<Tag>,
}

impl FileSystemDescription {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            owner_id: "test-owner_id".into(),
            file_system_id: "test-file_system_id".into(),
            file_system_arn: Some("test-file_system_arn".into()),
            creation_time: "test-creation_time".into(),
            life_cycle_state: "test-life_cycle_state".into(),
            name: Some("test-name".into()),
            number_of_mount_targets: 100,
            performance_mode: "test-performance_mode".into(),
            encrypted: Some(false),
            kms_key_id: Some("test-kms_key_id".into()),
            throughput_mode: Some("test-throughput_mode".into()),
            tags: vec![],
        }
    }
}

/// A tag is a key-value pair. Allowed characters are letters, white space, and numbers that can
/// be represented in UTF-8, and the following characters: + - = . _ : /.
///
/// **AWS API**: `efs.v1.Tag`
/// **Reference**: <https://docs.aws.amazon.com/efs/latest/ug/API_Operations.html/Tag>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    /// The tag key (String). The key can't start with aws:.
    pub key: String,

    /// The value of the tag key.
    pub value: String,
}

impl Tag {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key: "test-key".into(),
            value: "test-value".into(),
        }
    }
}
