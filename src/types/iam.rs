//! Types for the AWS Identity and Access Management API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Possible values for `iam.AccessKeyMetadata.Status`.
///
/// **AWS API**: `iam.AccessKeyMetadata.Status`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessKeyStatus {
    #[serde(rename = "Active")]
    Active,

    #[serde(rename = "Inactive")]
    Inactive,

    #[serde(rename = "Expired")]
    Expired,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `iam.GenerateCredentialReportResponse.State`.
///
/// **AWS API**: `iam.GenerateCredentialReportResponse.State`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportStateType {
    Started,

    Inprogress,

    Complete,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `iam.GetCredentialReportResponse.ReportFormat`.
///
/// **AWS API**: `iam.GetCredentialReportResponse.ReportFormat`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportFormatType {
    #[serde(rename = "text/csv")]
    TextPercsv,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Contains the response to a successful ListUsers request.
///
/// **AWS API**: `iam.v1.ListUsersResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListUsersResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUsersResponse {
    /// A list of users.
    #[serde(default)]
    pub users: Vec<User>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListUsersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            users: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains information about an IAM user entity. This data type is used as a response element
/// in the following operations: CreateUser GetUser ListUsers
///
/// **AWS API**: `iam.v1.User`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//User>
///
/// ## Coverage
/// 3 of 8 fields included.
/// Omitted fields:
/// - `Path` — not selected in manifest
/// - `UserId` — not selected in manifest
/// - `PasswordLastUsed` — not selected in manifest
/// - `PermissionsBoundary` — not selected in manifest
/// - `Tags` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    /// The Amazon Resource Name (ARN) that identifies the user. For more information about ARNs
    /// and how to use ARNs in policies, see IAM Identifiers in the IAM User Guide.
    pub arn: String,

    /// The friendly name identifying the user.
    pub user_name: String,

    /// The date and time, in ISO 8601 date-time format, when the user was created.
    pub create_date: String,
}

impl User {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            arn: "test-arn".into(),
            user_name: "test-user_name".into(),
            create_date: "test-create_date".into(),
        }
    }
}

/// Contains the response to a successful ListAttachedUserPolicies request.
///
/// **AWS API**: `iam.v1.ListAttachedUserPoliciesResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListAttachedUserPoliciesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAttachedUserPoliciesResponse {
    /// A list of the attached policies.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attached_policies: Vec<AttachedPolicy>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListAttachedUserPoliciesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            attached_policies: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains information about an attached policy. An attached policy is a managed policy that
/// has been attached to a user, group, or role. This data type is used as a response element in
/// the ListAttachedGroupPolicies, ListAttachedRolePolicies, ListAttachedUserPolicies, and
/// GetAccountAuthorizationDetails operations. For more information about managed policies,
/// refer to Managed policies and inline policies in the IAM User Guide.
///
/// **AWS API**: `iam.v1.AttachedPolicy`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//AttachedPolicy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttachedPolicy {
    /// The `PolicyArn` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,

    /// The friendly name of the attached policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

impl AttachedPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policy_arn: Some("test-policy_arn".into()),
            policy_name: Some("test-policy_name".into()),
        }
    }
}

/// Contains the response to a successful ListAccessKeys request.
///
/// **AWS API**: `iam.v1.ListAccessKeysResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListAccessKeysResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAccessKeysResponse {
    /// A list of objects containing metadata about the access keys.
    #[serde(default)]
    pub access_key_metadata: Vec<AccessKeyMetadata>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListAccessKeysResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            access_key_metadata: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains information about an Amazon Web Services access key, without its secret key. This
/// data type is used as a response element in the ListAccessKeys operation.
///
/// **AWS API**: `iam.v1.AccessKeyMetadata`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//AccessKeyMetadata>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccessKeyMetadata {
    /// The name of the IAM user that the key is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The ID for this access key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    /// The status of the access key. Active means that the key is valid for API calls; Inactive
    /// means it is not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AccessKeyStatus>,

    /// The date when the access key was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl AccessKeyMetadata {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: Some("test-user_name".into()),
            access_key_id: Some("test-access_key_id".into()),
            create_date: Some("test-create_date".into()),
            ..Default::default()
        }
    }
}

/// Contains the response to a successful GetAccessKeyLastUsed request. It is also returned as a
/// member of the AccessKeyMetaData structure returned by the ListAccessKeys action.
///
/// **AWS API**: `iam.v1.GetAccessKeyLastUsedResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetAccessKeyLastUsedResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetAccessKeyLastUsedResponse {
    /// The name of the IAM user that owns this access key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// Contains information about the last time the access key was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_last_used: Option<AccessKeyLastUsed>,
}

impl GetAccessKeyLastUsedResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: Some("test-user_name".into()),
            access_key_last_used: Some(AccessKeyLastUsed::fixture()),
        }
    }
}

/// Contains information about the last time an Amazon Web Services access key was used since
/// IAM began tracking this information on April 22, 2015. This data type is used as a response
/// element in the GetAccessKeyLastUsed operation.
///
/// **AWS API**: `iam.v1.AccessKeyLastUsed`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//AccessKeyLastUsed>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccessKeyLastUsed {
    /// The date and time, in ISO 8601 date-time format, when the access key was most recently
    /// used. This field is null in the following situations: The user does not have an access
    /// key. An access key exists but has not been used since IAM began tracking this
    /// information. There is no sign-in data associated with the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_date: Option<String>,

    /// The name of the Amazon Web Services service with which this access key was most recently
    /// used. The value of this field is "N/A" in the following situations: The user does not
    /// have an access key. An access key exists but has not been used since IAM started
    /// tracking this information. There is no sign-in data associated with the user.
    pub service_name: String,

    /// The Amazon Web Services Region where this access key was most recently used. The value
    /// for this field is "N/A" in the following situations: The user does not have an access
    /// key. An access key exists but has not been used since IAM began tracking this
    /// information. There is no sign-in data associated with the user. For more information
    /// about Amazon Web Services Regions, see Regions and endpoints in the Amazon Web Services
    /// General Reference.
    pub region: String,
}

impl AccessKeyLastUsed {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            last_used_date: Some("test-last_used_date".into()),
            service_name: "test-service_name".into(),
            region: "test-region".into(),
        }
    }
}

/// Contains the response to a successful GenerateCredentialReport request.
///
/// **AWS API**: `iam.v1.GenerateCredentialReportResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GenerateCredentialReportResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenerateCredentialReportResponse {
    /// Information about the state of the credential report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ReportStateType>,

    /// Information about the credential report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl GenerateCredentialReportResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            description: Some("test-description".into()),
            ..Default::default()
        }
    }
}

/// Contains the response to a successful GetCredentialReport request.
///
/// **AWS API**: `iam.v1.GetCredentialReportResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetCredentialReportResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCredentialReportResponse {
    /// Contains the credential report. The report is Base64-encoded.
    ///
    /// *Wire format: base64-encoded. Serde handles encoding/decoding automatically.*
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serde_base64::serialize_base64_opt",
        deserialize_with = "crate::serde_base64::deserialize_base64_opt"
    )]
    pub content: Option<String>,

    /// The format (MIME type) of the credential report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_format: Option<ReportFormatType>,

    /// The date and time when the credential report was created, in ISO 8601 date-time format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_time: Option<String>,
}

impl GetCredentialReportResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            content: Some("test-content".into()),
            generated_time: Some("test-generated_time".into()),
            ..Default::default()
        }
    }
}

/// Contains the response to a successful ListMFADevices request.
///
/// **AWS API**: `iam.v1.ListMFADevicesResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListMFADevicesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListMFADevicesResponse {
    /// A list of MFA devices.
    #[serde(rename = "MFADevices")]
    #[serde(default)]
    pub mfa_devices: Vec<MFADevice>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListMFADevicesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            mfa_devices: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains information about an MFA device. This data type is used as a response element in
/// the ListMFADevices operation.
///
/// **AWS API**: `iam.v1.MFADevice`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//MFADevice>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MFADevice {
    /// The user with whom the MFA device is associated.
    pub user_name: String,

    /// The serial number that uniquely identifies the MFA device. For virtual MFA devices, the
    /// serial number is the device ARN.
    pub serial_number: String,

    /// The date when the MFA device was enabled for the user.
    pub enable_date: String,
}

impl MFADevice {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: "test-user_name".into(),
            serial_number: "test-serial_number".into(),
            enable_date: "test-enable_date".into(),
        }
    }
}

/// Contains the response to a successful GetLoginProfile request.
///
/// **AWS API**: `iam.v1.GetLoginProfileResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetLoginProfileResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetLoginProfileResponse {
    /// A structure containing the user name and the profile creation date for the user.
    pub login_profile: LoginProfile,
}

impl GetLoginProfileResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            login_profile: LoginProfile::fixture(),
        }
    }
}

/// Contains the user name and password create date for a user. This data type is used as a
/// response element in the CreateLoginProfile and GetLoginProfile operations.
///
/// **AWS API**: `iam.v1.LoginProfile`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//LoginProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginProfile {
    /// The name of the user, which can be used for signing in to the Amazon Web Services
    /// Management Console.
    pub user_name: String,

    /// The date when the password for the user was created.
    pub create_date: String,

    /// Specifies whether the user is required to set a new password on next sign-in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reset_required: Option<bool>,
}

impl LoginProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: "test-user_name".into(),
            create_date: "test-create_date".into(),
            password_reset_required: Some(false),
        }
    }
}

/// Contains the response to a successful GetAccountSummary request.
///
/// **AWS API**: `iam.v1.GetAccountSummaryResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetAccountSummaryResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetAccountSummaryResponse {
    /// A set of key–value pairs containing information about IAM entity usage and IAM quotas.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub summary_map: HashMap<String, i32>,
}

impl GetAccountSummaryResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            summary_map: Default::default(),
        }
    }
}

/// Contains the response to a successful GetAccountPasswordPolicy request.
///
/// **AWS API**: `iam.v1.GetAccountPasswordPolicyResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetAccountPasswordPolicyResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetAccountPasswordPolicyResponse {
    /// A structure that contains details about the account's password policy.
    pub password_policy: PasswordPolicy,
}

impl GetAccountPasswordPolicyResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            password_policy: PasswordPolicy::fixture(),
        }
    }
}

/// Contains information about the account password policy. This data type is used as a response
/// element in the GetAccountPasswordPolicy operation.
///
/// **AWS API**: `iam.v1.PasswordPolicy`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//PasswordPolicy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PasswordPolicy {
    /// Minimum length to require for IAM user passwords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_password_length: Option<i32>,

    /// Specifies whether IAM user passwords must contain at least one of the following symbols:
    /// ! @ # $ % ^ &amp; * ( ) _ + - = [ ] { } | '
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_symbols: Option<bool>,

    /// Specifies whether IAM user passwords must contain at least one numeric character (0 to
    /// 9).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,

    /// Specifies whether IAM user passwords must contain at least one uppercase character (A to
    /// Z).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase_characters: Option<bool>,

    /// Specifies whether IAM user passwords must contain at least one lowercase character (a to
    /// z).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase_characters: Option<bool>,

    /// Specifies whether IAM users are allowed to change their own password. Gives IAM users
    /// permissions to iam:ChangePassword for only their user and to the
    /// iam:GetAccountPasswordPolicy action. This option does not attach a permissions policy to
    /// each user, rather the permissions are applied at the account-level for all users by IAM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_users_to_change_password: Option<bool>,

    /// Indicates whether passwords in the account expire. Returns true if MaxPasswordAge
    /// contains a value greater than 0. Returns false if MaxPasswordAge is 0 or not present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_passwords: Option<bool>,

    /// The number of days that an IAM user password is valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_password_age: Option<i32>,

    /// Specifies the number of previous passwords that IAM users are prevented from reusing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reuse_prevention: Option<i32>,

    /// Specifies whether IAM users are prevented from setting a new password via the Amazon Web
    /// Services Management Console after their password has expired. The IAM user cannot access
    /// the console until an administrator resets the password. IAM users with
    /// iam:ChangePassword permission and active access keys can reset their own expired console
    /// password using the CLI or API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_expiry: Option<bool>,
}

impl PasswordPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            minimum_password_length: Some(100),
            require_symbols: Some(false),
            require_numbers: Some(false),
            require_uppercase_characters: Some(false),
            require_lowercase_characters: Some(false),
            allow_users_to_change_password: Some(false),
            expire_passwords: Some(false),
            max_password_age: Some(100),
            password_reuse_prevention: Some(100),
            hard_expiry: Some(false),
        }
    }
}

/// Contains the response to a successful ListRoles request.
///
/// **AWS API**: `iam.v1.ListRolesResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListRolesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListRolesResponse {
    /// A list of roles.
    #[serde(default)]
    pub roles: Vec<Role>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListRolesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            roles: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains information about an IAM role. This structure is returned as a response element in
/// several API operations that interact with roles.
///
/// **AWS API**: `iam.v1.Role`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//Role>
///
/// ## Coverage
/// 4 of 11 fields included.
/// Omitted fields:
/// - `Path` — not selected in manifest
/// - `RoleId` — not selected in manifest
/// - `AssumeRolePolicyDocument` — not selected in manifest
/// - `MaxSessionDuration` — not selected in manifest
/// - `PermissionsBoundary` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `RoleLastUsed` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Role {
    /// The friendly name that identifies the role.
    pub role_name: String,

    /// The Amazon Resource Name (ARN) specifying the role. For more information about ARNs and
    /// how to use them in policies, see IAM identifiers in the IAM User Guide guide.
    pub arn: String,

    /// The date and time, in ISO 8601 date-time format, when the role was created.
    pub create_date: String,

    /// A description of the role that you provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Role {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role_name: "test-role_name".into(),
            arn: "test-arn".into(),
            create_date: "test-create_date".into(),
            description: Some("test-description".into()),
        }
    }
}

/// Contains the response to a successful ListUserPolicies request.
///
/// **AWS API**: `iam.v1.ListUserPoliciesResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListUserPoliciesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUserPoliciesResponse {
    /// A list of policy names.
    #[serde(default)]
    pub policy_names: Vec<String>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListUserPoliciesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policy_names: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains the response to a successful ListGroupsForUser request.
///
/// **AWS API**: `iam.v1.ListGroupsForUserResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListGroupsForUserResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListGroupsForUserResponse {
    /// A list of groups.
    #[serde(default)]
    pub groups: Vec<Group>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListGroupsForUserResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            groups: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains information about an IAM group entity. This data type is used as a response element
/// in the following operations: CreateGroup GetGroup ListGroups
///
/// **AWS API**: `iam.v1.Group`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//Group>
///
/// ## Coverage
/// 3 of 5 fields included.
/// Omitted fields:
/// - `Path` — not selected in manifest
/// - `GroupId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Group {
    /// The friendly name that identifies the group.
    pub group_name: String,

    /// The Amazon Resource Name (ARN) specifying the group. For more information about ARNs and
    /// how to use them in policies, see IAM identifiers in the IAM User Guide.
    pub arn: String,

    /// The date and time, in ISO 8601 date-time format, when the group was created.
    pub create_date: String,
}

impl Group {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_name: "test-group_name".into(),
            arn: "test-arn".into(),
            create_date: "test-create_date".into(),
        }
    }
}

/// Contains the response to a successful ListServerCertificates request.
///
/// **AWS API**: `iam.v1.ListServerCertificatesResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListServerCertificatesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListServerCertificatesResponse {
    /// A list of server certificates.
    #[serde(default)]
    pub server_certificate_metadata_list: Vec<ServerCertificateMetadata>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListServerCertificatesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            server_certificate_metadata_list: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains information about a server certificate without its certificate body, certificate
/// chain, and private key. This data type is used as a response element in the
/// UploadServerCertificate and ListServerCertificates operations.
///
/// **AWS API**: `iam.v1.ServerCertificateMetadata`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ServerCertificateMetadata>
///
/// ## Coverage
/// 4 of 6 fields included.
/// Omitted fields:
/// - `Path` — not selected in manifest
/// - `ServerCertificateId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerCertificateMetadata {
    /// The name that identifies the server certificate.
    pub server_certificate_name: String,

    /// The Amazon Resource Name (ARN) specifying the server certificate. For more information
    /// about ARNs and how to use them in policies, see IAM identifiers in the IAM User Guide.
    pub arn: String,

    /// The date on which the certificate is set to expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,

    /// The date when the server certificate was uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_date: Option<String>,
}

impl ServerCertificateMetadata {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            server_certificate_name: "test-server_certificate_name".into(),
            arn: "test-arn".into(),
            expiration: Some("test-expiration".into()),
            upload_date: Some("test-upload_date".into()),
        }
    }
}

///
/// **AWS API**: `iam.v1.CreateServiceLinkedRoleRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//CreateServiceLinkedRoleRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateServiceLinkedRoleRequest {
    /// The service principal for the Amazon Web Services service to which this role is
    /// attached. You use a string similar to a URL but without the http:// in front. For
    /// example: elasticbeanstalk.amazonaws.com. Service principals are unique and case-
    /// sensitive. To find the exact service principal for your service-linked role, see Amazon
    /// Web Services services that work with IAM in the IAM User Guide. Look for the services
    /// that have Yes in the Service-Linked Role column. Choose the Yes link to view the
    /// service-linked role documentation for that service.
    #[serde(rename = "AWSServiceName")]
    pub aws_service_name: String,

    /// The description of the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A string that you provide, which is combined with the service-provided prefix to form
    /// the complete role name. If you make multiple requests for the same service, then you
    /// must supply a different CustomSuffix for each request. Otherwise the request fails with
    /// a duplicate role name error. For example, you could add -1 or -debug to the suffix. Some
    /// services do not support the CustomSuffix parameter. If you provide an optional suffix
    /// and the operation fails, try the operation again without the suffix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_suffix: Option<String>,
}

impl CreateServiceLinkedRoleRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            aws_service_name: "test-aws_service_name".into(),
            description: Some("test-description".into()),
            custom_suffix: Some("test-custom_suffix".into()),
        }
    }
}

///
/// **AWS API**: `iam.v1.CreateServiceLinkedRoleResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//CreateServiceLinkedRoleResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateServiceLinkedRoleResponse {
    /// A Role object that contains details about the newly created role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

impl CreateServiceLinkedRoleResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role: Some(Role::fixture()),
        }
    }
}

/// Contains the response to a successful GetUserPolicy request.
///
/// **AWS API**: `iam.v1.GetUserPolicyResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetUserPolicyResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetUserPolicyResponse {
    /// The user the policy is associated with.
    pub user_name: String,

    /// The name of the policy.
    pub policy_name: String,

    /// The policy document. IAM stores policies in JSON format. However, resources that were
    /// created using CloudFormation templates can be formatted in YAML. CloudFormation always
    /// converts a YAML policy to JSON format before submitting it to IAM.
    pub policy_document: String,
}

impl GetUserPolicyResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: "test-user_name".into(),
            policy_name: "test-policy_name".into(),
            policy_document: "test-policy_document".into(),
        }
    }
}

/// Contains the response to a successful ListAttachedGroupPolicies request.
///
/// **AWS API**: `iam.v1.ListAttachedGroupPoliciesResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListAttachedGroupPoliciesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAttachedGroupPoliciesResponse {
    /// A list of the attached policies.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attached_policies: Vec<AttachedPolicy>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListAttachedGroupPoliciesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            attached_policies: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains the response to a successful ListVirtualMFADevices request.
///
/// **AWS API**: `iam.v1.ListVirtualMFADevicesResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListVirtualMFADevicesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListVirtualMFADevicesResponse {
    /// The list of virtual MFA devices in the current account that match the AssignmentStatus
    /// value that was passed in the request.
    #[serde(rename = "VirtualMFADevices")]
    #[serde(default)]
    pub virtual_mfa_devices: Vec<VirtualMFADevice>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListVirtualMFADevicesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            virtual_mfa_devices: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains information about a virtual MFA device.
///
/// **AWS API**: `iam.v1.VirtualMFADevice`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//VirtualMFADevice>
///
/// ## Coverage
/// 3 of 6 fields included.
/// Omitted fields:
/// - `Base32StringSeed` — not selected in manifest
/// - `QRCodePNG` — not selected in manifest
/// - `Tags` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VirtualMFADevice {
    /// The serial number associated with VirtualMFADevice.
    pub serial_number: String,

    /// The date and time on which the virtual MFA device was enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_date: Option<String>,

    /// The IAM user associated with this virtual MFA device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl VirtualMFADevice {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            serial_number: "test-serial_number".into(),
            enable_date: Some("test-enable_date".into()),
            user: Some(User::fixture()),
        }
    }
}

/// Contains the response to a successful ListPolicies request.
///
/// **AWS API**: `iam.v1.ListPoliciesResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListPoliciesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListPoliciesResponse {
    /// A list of policies.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<Policy>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListPoliciesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policies: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains information about a managed policy. This data type is used as a response element in
/// the CreatePolicy, GetPolicy, and ListPolicies operations. For more information about managed
/// policies, refer to Managed policies and inline policies in the IAM User Guide.
///
/// **AWS API**: `iam.v1.Policy`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//Policy>
///
/// ## Coverage
/// 7 of 12 fields included.
/// Omitted fields:
/// - `Path` — not selected in manifest
/// - `AttachmentCount` — not selected in manifest
/// - `PermissionsBoundaryUsageCount` — not selected in manifest
/// - `Description` — not selected in manifest
/// - `Tags` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    /// The friendly name (not ARN) identifying the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,

    /// The stable and unique string identifying the policy. For more information about IDs, see
    /// IAM identifiers in the IAM User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,

    /// The `Arn` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The identifier for the version of the policy that is set as the default version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,

    /// Specifies whether the policy can be attached to an IAM user, group, or role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attachable: Option<bool>,

    /// The date and time, in ISO 8601 date-time format, when the policy was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,

    /// The date and time, in ISO 8601 date-time format, when the policy was last updated. When
    /// a policy has only one version, this field contains the date and time when the policy was
    /// created. When a policy has more than one version, this field contains the date and time
    /// when the most recent policy version was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

impl Policy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policy_name: Some("test-policy_name".into()),
            policy_id: Some("test-policy_id".into()),
            arn: Some("test-arn".into()),
            default_version_id: Some("test-default_version_id".into()),
            is_attachable: Some(false),
            create_date: Some("test-create_date".into()),
            update_date: Some("test-update_date".into()),
        }
    }
}

/// Contains the response to a successful GetPolicyVersion request.
///
/// **AWS API**: `iam.v1.GetPolicyVersionResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetPolicyVersionResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetPolicyVersionResponse {
    /// A structure containing details about the policy version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<PolicyVersion>,
}

impl GetPolicyVersionResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policy_version: Some(PolicyVersion::fixture()),
        }
    }
}

/// Contains information about a version of a managed policy. This data type is used as a
/// response element in the CreatePolicyVersion, GetPolicyVersion, ListPolicyVersions, and
/// GetAccountAuthorizationDetails operations. For more information about managed policies,
/// refer to Managed policies and inline policies in the IAM User Guide.
///
/// **AWS API**: `iam.v1.PolicyVersion`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//PolicyVersion>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyVersion {
    /// The policy document. The policy document is returned in the response to the
    /// GetPolicyVersion and GetAccountAuthorizationDetails operations. It is not returned in
    /// the response to the CreatePolicyVersion or ListPolicyVersions operations. The policy
    /// document returned in this structure is URL-encoded compliant with RFC 3986. You can use
    /// a URL decoding method to convert the policy back to plain JSON text. For example, if you
    /// use Java, you can use the decode method of the java.net.URLDecoder utility class in the
    /// Java SDK. Other languages and SDKs provide similar functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,

    /// The identifier for the policy version. Policy version identifiers always begin with v
    /// (always lowercase). When a policy is created, the first policy version is v1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,

    /// Specifies whether the policy version is set as the policy's default version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,

    /// The date and time, in ISO 8601 date-time format, when the policy version was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl PolicyVersion {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            document: Some("test-document".into()),
            version_id: Some("test-version_id".into()),
            is_default_version: Some(false),
            create_date: Some("test-create_date".into()),
        }
    }
}

/// Contains the response to a successful ListEntitiesForPolicy request.
///
/// **AWS API**: `iam.v1.ListEntitiesForPolicyResponse`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListEntitiesForPolicyResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListEntitiesForPolicyResponse {
    /// A list of IAM groups that the policy is attached to.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub policy_groups: Vec<PolicyGroup>,

    /// A list of IAM users that the policy is attached to.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub policy_users: Vec<PolicyUser>,

    /// A list of IAM roles that the policy is attached to.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub policy_roles: Vec<PolicyRole>,

    /// A flag that indicates whether there are more items to return. If your results were
    /// truncated, you can make a subsequent pagination request using the Marker request
    /// parameter to retrieve more items. Note that IAM might return fewer than the MaxItems
    /// number of results even when there are more results available. We recommend that you
    /// check IsTruncated after every call to ensure that you receive all your results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,

    /// When IsTruncated is true, this element is present and contains the value to use for the
    /// Marker parameter in a subsequent pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl ListEntitiesForPolicyResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policy_groups: vec![],
            policy_users: vec![],
            policy_roles: vec![],
            is_truncated: Some(false),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains information about a group that a managed policy is attached to. This data type is
/// used as a response element in the ListEntitiesForPolicy operation. For more information
/// about managed policies, refer to Managed policies and inline policies in the IAM User Guide.
///
/// **AWS API**: `iam.v1.PolicyGroup`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//PolicyGroup>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyGroup {
    /// The name (friendly name, not ARN) identifying the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The stable and unique string identifying the group. For more information about IDs, see
    /// IAM identifiers in the IAM User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl PolicyGroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_name: Some("test-group_name".into()),
            group_id: Some("test-group_id".into()),
        }
    }
}

/// Contains information about a user that a managed policy is attached to. This data type is
/// used as a response element in the ListEntitiesForPolicy operation. For more information
/// about managed policies, refer to Managed policies and inline policies in the IAM User Guide.
///
/// **AWS API**: `iam.v1.PolicyUser`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//PolicyUser>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyUser {
    /// The name (friendly name, not ARN) identifying the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The stable and unique string identifying the user. For more information about IDs, see
    /// IAM identifiers in the IAM User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl PolicyUser {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: Some("test-user_name".into()),
            user_id: Some("test-user_id".into()),
        }
    }
}

/// Contains information about a role that a managed policy is attached to. This data type is
/// used as a response element in the ListEntitiesForPolicy operation. For more information
/// about managed policies, refer to Managed policies and inline policies in the IAM User Guide.
///
/// **AWS API**: `iam.v1.PolicyRole`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//PolicyRole>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyRole {
    /// The name (friendly name, not ARN) identifying the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,

    /// The stable and unique string identifying the role. For more information about IDs, see
    /// IAM identifiers in the IAM User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
}

impl PolicyRole {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role_name: Some("test-role_name".into()),
            role_id: Some("test-role_id".into()),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListUsersRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListUsersRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUsersRequest {
    /// The path prefix for filtering the results. For example: /division_abc/subdivision_xyz/,
    /// which would get all user names whose path starts with /division_abc/subdivision_xyz/.
    /// This parameter is optional. If it is not included, it defaults to a slash (/), listing
    /// all user names. This parameter allows (through its regex pattern) a string of characters
    /// consisting of either a forward slash (/) by itself or a string that must begin and end
    /// with forward slashes. In addition, it can contain any ASCII character from the !
    /// (\u0021) through the DEL character (\u007F), including most punctuation characters,
    /// digits, and upper and lowercased letters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListUsersRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            path_prefix: Some("test-path_prefix".into()),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListAttachedUserPoliciesRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListAttachedUserPoliciesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAttachedUserPoliciesRequest {
    /// The name (friendly name, not ARN) of the user to list attached policies for. This
    /// parameter allows (through its regex pattern) a string of characters consisting of upper
    /// and lowercase alphanumeric characters with no spaces. You can also include any of the
    /// following characters: _+=,.@-
    pub user_name: String,

    /// The path prefix for filtering the results. This parameter is optional. If it is not
    /// included, it defaults to a slash (/), listing all policies. This parameter allows
    /// (through its regex pattern) a string of characters consisting of either a forward slash
    /// (/) by itself or a string that must begin and end with forward slashes. In addition, it
    /// can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F),
    /// including most punctuation characters, digits, and upper and lowercased letters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListAttachedUserPoliciesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: "test-user_name".into(),
            path_prefix: Some("test-path_prefix".into()),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.DetachUserPolicyRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//DetachUserPolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetachUserPolicyRequest {
    /// The name (friendly name, not ARN) of the IAM user to detach the policy from. This
    /// parameter allows (through its regex pattern) a string of characters consisting of upper
    /// and lowercase alphanumeric characters with no spaces. You can also include any of the
    /// following characters: _+=,.@-
    pub user_name: String,

    /// The Amazon Resource Name (ARN) of the IAM policy you want to detach. For more
    /// information about ARNs, see Amazon Resource Names (ARNs) in the Amazon Web Services
    /// General Reference.
    pub policy_arn: String,
}

impl DetachUserPolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: "test-user_name".into(),
            policy_arn: "test-policy_arn".into(),
        }
    }
}

///
/// **AWS API**: `iam.v1.DeleteAccessKeyRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//DeleteAccessKeyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteAccessKeyRequest {
    /// The name of the user whose access key pair you want to delete. This parameter allows
    /// (through its regex pattern) a string of characters consisting of upper and lowercase
    /// alphanumeric characters with no spaces. You can also include any of the following
    /// characters: _+=,.@-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The access key ID for the access key ID and secret access key you want to delete. This
    /// parameter allows (through its regex pattern) a string of characters that can consist of
    /// any upper or lowercased letter or digit.
    pub access_key_id: String,
}

impl DeleteAccessKeyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: Some("test-user_name".into()),
            access_key_id: "test-access_key_id".into(),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListAccessKeysRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListAccessKeysRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAccessKeysRequest {
    /// The name of the user. This parameter allows (through its regex pattern) a string of
    /// characters consisting of upper and lowercase alphanumeric characters with no spaces. You
    /// can also include any of the following characters: _+=,.@-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListAccessKeysRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: Some("test-user_name".into()),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.GetAccessKeyLastUsedRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetAccessKeyLastUsedRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetAccessKeyLastUsedRequest {
    /// The identifier of an access key. This parameter allows (through its regex pattern) a
    /// string of characters that can consist of any upper or lowercased letter or digit.
    pub access_key_id: String,
}

impl GetAccessKeyLastUsedRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            access_key_id: "test-access_key_id".into(),
        }
    }
}

///
/// **AWS API**: `iam.v1.UpdateAccessKeyRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//UpdateAccessKeyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateAccessKeyRequest {
    /// The name of the user whose key you want to update. This parameter allows (through its
    /// regex pattern) a string of characters consisting of upper and lowercase alphanumeric
    /// characters with no spaces. You can also include any of the following characters: _+=,.@-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The access key ID of the secret access key you want to update. This parameter allows
    /// (through its regex pattern) a string of characters that can consist of any upper or
    /// lowercased letter or digit.
    pub access_key_id: String,

    /// The status you want to assign to the secret access key. Active means that the key can be
    /// used for programmatic calls to Amazon Web Services, while Inactive means that the key
    /// cannot be used.
    pub status: String,
}

impl UpdateAccessKeyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: Some("test-user_name".into()),
            access_key_id: "test-access_key_id".into(),
            status: "test-status".into(),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListMFADevicesRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListMFADevicesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListMFADevicesRequest {
    /// The name of the user whose MFA devices you want to list. This parameter allows (through
    /// its regex pattern) a string of characters consisting of upper and lowercase alphanumeric
    /// characters with no spaces. You can also include any of the following characters: _+=,.@-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListMFADevicesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: Some("test-user_name".into()),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.GetLoginProfileRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetLoginProfileRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetLoginProfileRequest {
    /// The name of the user whose login profile you want to retrieve. This parameter is
    /// optional. If no user name is included, it defaults to the principal making the request.
    /// When you make this request with root user credentials, you must use an AssumeRoot
    /// session to omit the user name. This parameter allows (through its regex pattern) a
    /// string of characters consisting of upper and lowercase alphanumeric characters with no
    /// spaces. You can also include any of the following characters: _+=,.@-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl GetLoginProfileRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: Some("test-user_name".into()),
        }
    }
}

///
/// **AWS API**: `iam.v1.UpdateAccountPasswordPolicyRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//UpdateAccountPasswordPolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateAccountPasswordPolicyRequest {
    /// The minimum number of characters allowed in an IAM user password. If you do not specify
    /// a value for this parameter, then the operation uses the default value of 6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_password_length: Option<i32>,

    /// Specifies whether IAM user passwords must contain at least one of the following non-
    /// alphanumeric characters: ! @ # $ % ^ &amp; * ( ) _ + - = [ ] { } | ' If you do not
    /// specify a value for this parameter, then the operation uses the default value of false.
    /// The result is that passwords do not require at least one symbol character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_symbols: Option<bool>,

    /// Specifies whether IAM user passwords must contain at least one numeric character (0 to
    /// 9). If you do not specify a value for this parameter, then the operation uses the
    /// default value of false. The result is that passwords do not require at least one numeric
    /// character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,

    /// Specifies whether IAM user passwords must contain at least one uppercase character from
    /// the ISO basic Latin alphabet (A to Z). If you do not specify a value for this parameter,
    /// then the operation uses the default value of false. The result is that passwords do not
    /// require at least one uppercase character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase_characters: Option<bool>,

    /// Specifies whether IAM user passwords must contain at least one lowercase character from
    /// the ISO basic Latin alphabet (a to z). If you do not specify a value for this parameter,
    /// then the operation uses the default value of false. The result is that passwords do not
    /// require at least one lowercase character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase_characters: Option<bool>,

    /// Allows all IAM users in your account to use the Amazon Web Services Management Console
    /// to change their own passwords. For more information, see Permitting IAM users to change
    /// their own passwords in the IAM User Guide. If you do not specify a value for this
    /// parameter, then the operation uses the default value of false. The result is that IAM
    /// users in the account do not automatically have permissions to change their own password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_users_to_change_password: Option<bool>,

    /// The number of days that an IAM user password is valid. If you do not specify a value for
    /// this parameter, then the operation uses the default value of 0. The result is that IAM
    /// user passwords never expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_password_age: Option<i32>,

    /// Specifies the number of previous passwords that IAM users are prevented from reusing. If
    /// you do not specify a value for this parameter, then the operation uses the default value
    /// of 0. The result is that IAM users are not prevented from reusing previous passwords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reuse_prevention: Option<i32>,

    /// Prevents IAM users who are accessing the account via the Amazon Web Services Management
    /// Console from setting a new console password after their password has expired. The IAM
    /// user cannot access the console until an administrator resets the password. If you do not
    /// specify a value for this parameter, then the operation uses the default value of false.
    /// The result is that IAM users can change their passwords after they expire and continue
    /// to sign in as the user. In the Amazon Web Services Management Console, the custom
    /// password policy option Allow users to change their own password gives IAM users
    /// permissions to iam:ChangePassword for only their user and to the
    /// iam:GetAccountPasswordPolicy action. This option does not attach a permissions policy to
    /// each user, rather the permissions are applied at the account-level for all users by IAM.
    /// IAM users with iam:ChangePassword permission and active access keys can reset their own
    /// expired console password using the CLI or API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_expiry: Option<bool>,
}

impl UpdateAccountPasswordPolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            minimum_password_length: Some(100),
            require_symbols: Some(false),
            require_numbers: Some(false),
            require_uppercase_characters: Some(false),
            require_lowercase_characters: Some(false),
            allow_users_to_change_password: Some(false),
            max_password_age: Some(100),
            password_reuse_prevention: Some(100),
            hard_expiry: Some(false),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListRolesRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListRolesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListRolesRequest {
    /// The path prefix for filtering the results. For example, the prefix
    /// /application_abc/component_xyz/ gets all roles whose path starts with
    /// /application_abc/component_xyz/. This parameter is optional. If it is not included, it
    /// defaults to a slash (/), listing all roles. This parameter allows (through its regex
    /// pattern) a string of characters consisting of either a forward slash (/) by itself or a
    /// string that must begin and end with forward slashes. In addition, it can contain any
    /// ASCII character from the ! (\u0021) through the DEL character (\u007F), including most
    /// punctuation characters, digits, and upper and lowercased letters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListRolesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            path_prefix: Some("test-path_prefix".into()),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListUserPoliciesRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListUserPoliciesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUserPoliciesRequest {
    /// The name of the user to list policies for. This parameter allows (through its regex
    /// pattern) a string of characters consisting of upper and lowercase alphanumeric
    /// characters with no spaces. You can also include any of the following characters: _+=,.@-
    pub user_name: String,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListUserPoliciesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: "test-user_name".into(),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListGroupsForUserRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListGroupsForUserRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListGroupsForUserRequest {
    /// The name of the user to list groups for. This parameter allows (through its regex
    /// pattern) a string of characters consisting of upper and lowercase alphanumeric
    /// characters with no spaces. You can also include any of the following characters: _+=,.@-
    pub user_name: String,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListGroupsForUserRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: "test-user_name".into(),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListServerCertificatesRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListServerCertificatesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListServerCertificatesRequest {
    /// The path prefix for filtering the results. For example: /company/servercerts would get
    /// all server certificates for which the path starts with /company/servercerts. This
    /// parameter is optional. If it is not included, it defaults to a slash (/), listing all
    /// server certificates. This parameter allows (through its regex pattern) a string of
    /// characters consisting of either a forward slash (/) by itself or a string that must
    /// begin and end with forward slashes. In addition, it can contain any ASCII character from
    /// the ! (\u0021) through the DEL character (\u007F), including most punctuation
    /// characters, digits, and upper and lowercased letters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListServerCertificatesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            path_prefix: Some("test-path_prefix".into()),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.DeleteUserPolicyRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//DeleteUserPolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteUserPolicyRequest {
    /// The name (friendly name, not ARN) identifying the user that the policy is embedded in.
    /// This parameter allows (through its regex pattern) a string of characters consisting of
    /// upper and lowercase alphanumeric characters with no spaces. You can also include any of
    /// the following characters: _+=,.@-
    pub user_name: String,

    /// The name identifying the policy document to delete. This parameter allows (through its
    /// regex pattern) a string of characters consisting of upper and lowercase alphanumeric
    /// characters with no spaces. You can also include any of the following characters: _+=,.@-
    pub policy_name: String,
}

impl DeleteUserPolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: "test-user_name".into(),
            policy_name: "test-policy_name".into(),
        }
    }
}

///
/// **AWS API**: `iam.v1.AttachRolePolicyRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//AttachRolePolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttachRolePolicyRequest {
    /// The name (friendly name, not ARN) of the role to attach the policy to. This parameter
    /// allows (through its regex pattern) a string of characters consisting of upper and
    /// lowercase alphanumeric characters with no spaces. You can also include any of the
    /// following characters: _+=,.@-
    pub role_name: String,

    /// The Amazon Resource Name (ARN) of the IAM policy you want to attach. For more
    /// information about ARNs, see Amazon Resource Names (ARNs) in the Amazon Web Services
    /// General Reference.
    pub policy_arn: String,
}

impl AttachRolePolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role_name: "test-role_name".into(),
            policy_arn: "test-policy_arn".into(),
        }
    }
}

///
/// **AWS API**: `iam.v1.DetachRolePolicyRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//DetachRolePolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetachRolePolicyRequest {
    /// The name (friendly name, not ARN) of the IAM role to detach the policy from. This
    /// parameter allows (through its regex pattern) a string of characters consisting of upper
    /// and lowercase alphanumeric characters with no spaces. You can also include any of the
    /// following characters: _+=,.@-
    pub role_name: String,

    /// The Amazon Resource Name (ARN) of the IAM policy you want to detach. For more
    /// information about ARNs, see Amazon Resource Names (ARNs) in the Amazon Web Services
    /// General Reference.
    pub policy_arn: String,
}

impl DetachRolePolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role_name: "test-role_name".into(),
            policy_arn: "test-policy_arn".into(),
        }
    }
}

///
/// **AWS API**: `iam.v1.GetUserPolicyRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetUserPolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetUserPolicyRequest {
    /// The name of the user who the policy is associated with. This parameter allows (through
    /// its regex pattern) a string of characters consisting of upper and lowercase alphanumeric
    /// characters with no spaces. You can also include any of the following characters: _+=,.@-
    pub user_name: String,

    /// The name of the policy document to get. This parameter allows (through its regex
    /// pattern) a string of characters consisting of upper and lowercase alphanumeric
    /// characters with no spaces. You can also include any of the following characters: _+=,.@-
    pub policy_name: String,
}

impl GetUserPolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_name: "test-user_name".into(),
            policy_name: "test-policy_name".into(),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListAttachedGroupPoliciesRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListAttachedGroupPoliciesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAttachedGroupPoliciesRequest {
    /// The name (friendly name, not ARN) of the group to list attached policies for. This
    /// parameter allows (through its regex pattern) a string of characters consisting of upper
    /// and lowercase alphanumeric characters with no spaces. You can also include any of the
    /// following characters: _+=,.@-
    pub group_name: String,

    /// The path prefix for filtering the results. This parameter is optional. If it is not
    /// included, it defaults to a slash (/), listing all policies. This parameter allows
    /// (through its regex pattern) a string of characters consisting of either a forward slash
    /// (/) by itself or a string that must begin and end with forward slashes. In addition, it
    /// can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F),
    /// including most punctuation characters, digits, and upper and lowercased letters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListAttachedGroupPoliciesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            group_name: "test-group_name".into(),
            path_prefix: Some("test-path_prefix".into()),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListVirtualMFADevicesRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListVirtualMFADevicesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListVirtualMFADevicesRequest {
    /// The status (Unassigned or Assigned) of the devices to list. If you do not specify an
    /// AssignmentStatus, the operation defaults to Any, which lists both assigned and
    /// unassigned virtual MFA devices.,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListVirtualMFADevicesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            assignment_status: Some("test-assignment_status".into()),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListPoliciesRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListPoliciesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListPoliciesRequest {
    /// The scope to use for filtering the results. To list only Amazon Web Services managed
    /// policies, set Scope to AWS. To list only the customer managed policies in your Amazon
    /// Web Services account, set Scope to Local. This parameter is optional. If it is not
    /// included, or if it is set to All, all policies are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// A flag to filter the results to only the attached policies. When OnlyAttached is true,
    /// the returned list contains only the policies that are attached to an IAM user, group, or
    /// role. When OnlyAttached is false, or when the parameter is not included, all policies
    /// are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_attached: Option<bool>,

    /// The path prefix for filtering the results. This parameter is optional. If it is not
    /// included, it defaults to a slash (/), listing all policies. This parameter allows
    /// (through its regex pattern) a string of characters consisting of either a forward slash
    /// (/) by itself or a string that must begin and end with forward slashes. In addition, it
    /// can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F),
    /// including most punctuation characters, digits, and upper and lowercased letters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,

    /// The policy usage method to use for filtering the results. To list only permissions
    /// policies, set PolicyUsageFilter to PermissionsPolicy. To list only the policies used to
    /// set permissions boundaries, set the value to PermissionsBoundary. This parameter is
    /// optional. If it is not included, all policies are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_usage_filter: Option<String>,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListPoliciesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            scope: Some("test-scope".into()),
            only_attached: Some(false),
            path_prefix: Some("test-path_prefix".into()),
            policy_usage_filter: Some("test-policy_usage_filter".into()),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}

///
/// **AWS API**: `iam.v1.GetPolicyVersionRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//GetPolicyVersionRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetPolicyVersionRequest {
    /// The Amazon Resource Name (ARN) of the managed policy that you want information about.
    /// For more information about ARNs, see Amazon Resource Names (ARNs) in the Amazon Web
    /// Services General Reference.
    pub policy_arn: String,

    /// Identifies the policy version to retrieve. This parameter allows (through its regex
    /// pattern) a string of characters that consists of the lowercase letter 'v' followed by
    /// one or two digits, and optionally followed by a period '.' and a string of letters and
    /// digits.
    pub version_id: String,
}

impl GetPolicyVersionRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policy_arn: "test-policy_arn".into(),
            version_id: "test-version_id".into(),
        }
    }
}

///
/// **AWS API**: `iam.v1.ListEntitiesForPolicyRequest`
/// **Reference**: <https://docs.aws.amazon.com/IAM/latest/APIReference//ListEntitiesForPolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListEntitiesForPolicyRequest {
    /// The Amazon Resource Name (ARN) of the IAM policy for which you want the versions. For
    /// more information about ARNs, see Amazon Resource Names (ARNs) in the Amazon Web Services
    /// General Reference.
    pub policy_arn: String,

    /// The entity type to use for filtering the results. For example, when EntityFilter is
    /// Role, only the roles that are attached to the specified policy are returned. This
    /// parameter is optional. If it is not included, all attached entities (users, groups, and
    /// roles) are returned. The argument for this parameter must be one of the valid values
    /// listed below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_filter: Option<String>,

    /// The path prefix for filtering the results. This parameter is optional. If it is not
    /// included, it defaults to a slash (/), listing all entities. This parameter allows
    /// (through its regex pattern) a string of characters consisting of either a forward slash
    /// (/) by itself or a string that must begin and end with forward slashes. In addition, it
    /// can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F),
    /// including most punctuation characters, digits, and upper and lowercased letters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,

    /// The policy usage method to use for filtering the results. To list only permissions
    /// policies, set PolicyUsageFilter to PermissionsPolicy. To list only the policies used to
    /// set permissions boundaries, set the value to PermissionsBoundary. This parameter is
    /// optional. If it is not included, all policies are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_usage_filter: Option<String>,

    /// Use this parameter only when paginating results and only after you receive a response
    /// indicating that the results are truncated. Set it to the value of the Marker element in
    /// the response that you received to indicate where the next call should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Use this only when paginating results to indicate the maximum number of items you want
    /// in the response. If additional items exist beyond the maximum you specify, the
    /// IsTruncated response element is true. If you do not include this parameter, the number
    /// of items defaults to 100. Note that IAM might return fewer results, even when there are
    /// more results available. In that case, the IsTruncated response element returns true, and
    /// Marker contains a value to include in the subsequent call that tells the service where
    /// to continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

impl ListEntitiesForPolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policy_arn: "test-policy_arn".into(),
            entity_filter: Some("test-entity_filter".into()),
            path_prefix: Some("test-path_prefix".into()),
            policy_usage_filter: Some("test-policy_usage_filter".into()),
            marker: Some("test-marker".into()),
            max_items: Some(100),
        }
    }
}
