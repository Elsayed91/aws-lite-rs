//! Types for the AWS Security Token Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Contains the response to a successful GetCallerIdentity request, including information about
/// the entity making the request.
///
/// **AWS API**: `sts.v1.GetCallerIdentityResponse`
/// **Reference**: <https://docs.aws.amazon.com/STS/latest/APIReference//GetCallerIdentityResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCallerIdentityResponse {
    /// The unique identifier of the calling entity. The exact value depends on the type of
    /// entity that is making the call. The values returned are those listed in the aws:userid
    /// column in the Principal table found on the Policy Variables reference page in the IAM
    /// User Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The Amazon Web Services account ID number of the account that owns or contains the
    /// calling entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// The Amazon Web Services ARN associated with the calling entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

impl GetCallerIdentityResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_id: Some("test-user_id".into()),
            account: Some("test-account".into()),
            arn: Some("test-arn".into()),
        }
    }
}

/// Contains the response to a successful AssumeRole request, including temporary Amazon Web
/// Services credentials that can be used to make Amazon Web Services requests.
///
/// **AWS API**: `sts.v1.AssumeRoleResponse`
/// **Reference**: <https://docs.aws.amazon.com/STS/latest/APIReference//AssumeRoleResponse>
///
/// ## Coverage
/// 2 of 4 fields included.
/// Omitted fields:
/// - `PackedPolicySize` — not selected in manifest
/// - `SourceIdentity` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssumeRoleResponse {
    /// The temporary security credentials, which include an access key ID, a secret access key,
    /// and a security (or session) token. The size of the security token that STS API
    /// operations return is not fixed. We strongly recommend that you make no assumptions about
    /// the maximum size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,

    /// The Amazon Resource Name (ARN) and the assumed role ID, which are identifiers that you
    /// can use to refer to the resulting temporary security credentials. For example, you can
    /// reference these credentials as a principal in a resource-based policy by using the ARN
    /// or assumed role ID. The ARN and ID include the RoleSessionName that you specified when
    /// you called AssumeRole.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_user: Option<AssumedRoleUser>,
}

impl AssumeRoleResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            credentials: Some(Credentials::fixture()),
            assumed_role_user: Some(AssumedRoleUser::fixture()),
        }
    }
}

/// Amazon Web Services credentials for API authentication.
///
/// **AWS API**: `sts.v1.Credentials`
/// **Reference**: <https://docs.aws.amazon.com/STS/latest/APIReference//Credentials>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Credentials {
    /// The access key ID that identifies the temporary security credentials.
    pub access_key_id: String,

    /// The secret access key that can be used to sign requests.
    pub secret_access_key: String,

    /// The token that users must pass to the service API to use the temporary credentials.
    pub session_token: String,

    /// The date on which the current credentials expire.
    pub expiration: String,
}

impl Credentials {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            access_key_id: "test-access_key_id".into(),
            secret_access_key: "test-secret_access_key".into(),
            session_token: "test-session_token".into(),
            expiration: "test-expiration".into(),
        }
    }
}

/// The identifiers for the temporary security credentials that the operation returns.
///
/// **AWS API**: `sts.v1.AssumedRoleUser`
/// **Reference**: <https://docs.aws.amazon.com/STS/latest/APIReference//AssumedRoleUser>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssumedRoleUser {
    /// A unique identifier that contains the role ID and the role session name of the role that
    /// is being assumed. The role ID is generated by Amazon Web Services when the role is
    /// created.
    pub assumed_role_id: String,

    /// The ARN of the temporary security credentials that are returned from the AssumeRole
    /// action. For more information about ARNs and how to use them in policies, see IAM
    /// Identifiers in the IAM User Guide.
    pub arn: String,
}

impl AssumedRoleUser {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            assumed_role_id: "test-assumed_role_id".into(),
            arn: "test-arn".into(),
        }
    }
}

///
/// **AWS API**: `sts.v1.AssumeRoleRequest`
/// **Reference**: <https://docs.aws.amazon.com/STS/latest/APIReference//AssumeRoleRequest>
///
/// ## Coverage
/// 3 of 12 fields included.
/// Omitted fields:
/// - `PolicyArns` — not selected in manifest
/// - `Policy` — not selected in manifest
/// - `DurationSeconds` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `TransitiveTagKeys` — not selected in manifest
/// - `SerialNumber` — not selected in manifest
/// - `TokenCode` — not selected in manifest
/// - `SourceIdentity` — not selected in manifest
/// - `ProvidedContexts` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssumeRoleRequest {
    /// The Amazon Resource Name (ARN) of the role to assume.
    pub role_arn: String,

    /// An identifier for the assumed role session. Use the role session name to uniquely
    /// identify a session when the same role is assumed by different principals or for
    /// different reasons. In cross-account scenarios, the role session name is visible to, and
    /// can be logged by the account that owns the role. The role session name is also used in
    /// the ARN of the assumed role principal. This means that subsequent cross-account API
    /// requests that use the temporary security credentials will expose the role session name
    /// to the external account in their CloudTrail logs. For security purposes, administrators
    /// can view this field in CloudTrail logs to help identify who performed an action in
    /// Amazon Web Services. Your administrator might require that you specify your user name as
    /// the session name when you assume the role. For more information, see sts:RoleSessionName
    /// . The regex used to validate this parameter is a string of characters consisting of
    /// upper- and lower-case alphanumeric characters with no spaces. You can also include
    /// underscores or any of the following characters: +=,.@-
    pub role_session_name: String,

    /// A unique identifier that might be required when you assume a role in another account. If
    /// the administrator of the account to which the role belongs provided you with an external
    /// ID, then provide that value in the ExternalId parameter. This value can be any string,
    /// such as a passphrase or account number. A cross-account role is usually set up to trust
    /// everyone in an account. Therefore, the administrator of the trusting account might send
    /// an external ID to the administrator of the trusted account. That way, only someone with
    /// the ID can assume the role, rather than everyone in the account. For more information
    /// about the external ID, see How to Use an External ID When Granting Access to Your Amazon
    /// Web Services Resources to a Third Party in the IAM User Guide. The regex used to
    /// validate this parameter is a string of characters consisting of upper- and lower-case
    /// alphanumeric characters with no spaces. You can also include underscores or any of the
    /// following characters: +=,.@:\/-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl AssumeRoleRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role_arn: "test-role_arn".into(),
            role_session_name: "test-role_session_name".into(),
            external_id: Some("test-external_id".into()),
        }
    }
}

///
/// **AWS API**: `sts.v1.GetCallerIdentityRequest`
/// **Reference**: <https://docs.aws.amazon.com/STS/latest/APIReference//GetCallerIdentityRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCallerIdentityRequest {}

impl GetCallerIdentityRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}
