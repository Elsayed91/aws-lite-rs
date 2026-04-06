//! Types for the AWS Organizations API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Possible values for `organizations.Account.Status`.
///
/// **AWS API**: `organizations.Account.Status`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    Active,

    Suspended,

    PendingClosure,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `organizations.Account.JoinedMethod`.
///
/// **AWS API**: `organizations.Account.JoinedMethod`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountJoinedMethod {
    Invited,

    Created,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Contains details about an organizational unit (OU). An OU is a container of Amazon Web
/// Services accounts within a root of an organization. Policies that are attached to an OU
/// apply to all accounts contained in that OU and in any child OUs.
///
/// **AWS API**: `organizations.v1.OrganizationalUnit`
/// **Reference**: <https://docs.aws.amazon.com/organizations/latest/APIReference//OrganizationalUnit>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrganizationalUnit {
    /// The unique identifier (ID) associated with this OU. The ID is unique to the organization
    /// only. The regex pattern for an organizational unit ID string requires "ou-" followed by
    /// from 4 to 32 lowercase letters or digits (the ID of the root that contains the OU). This
    /// string is followed by a second "-" dash and from 8 to 32 additional lowercase letters or
    /// digits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The Amazon Resource Name (ARN) of this OU. For more information about ARNs in
    /// Organizations, see ARN Formats Supported by Organizations in the Amazon Web Services
    /// Service Authorization Reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The friendly name of this OU. The regex pattern that is used to validate this parameter
    /// is a string of any of the characters in the ASCII character range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl OrganizationalUnit {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            arn: Some("test-arn".into()),
            name: Some("test-name".into()),
        }
    }
}

///
/// **AWS API**: `organizations.v1.ListOrganizationalUnitsForParentResponse`
/// **Reference**: <https://docs.aws.amazon.com/organizations/latest/APIReference//ListOrganizationalUnitsForParentResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListOrganizationalUnitsForParentResponse {
    /// A list of the OUs in the specified root or parent OU.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub organizational_units: Vec<OrganizationalUnit>,

    /// If present, indicates that more output is available than is included in the current
    /// response. Use this value in the NextToken request parameter in a subsequent call to the
    /// operation to get the next part of the output. You should repeat this until the NextToken
    /// response element comes back as null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListOrganizationalUnitsForParentResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            organizational_units: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Contains information about an Amazon Web Services account that is a member of an
/// organization.
///
/// **AWS API**: `organizations.v1.Account`
/// **Reference**: <https://docs.aws.amazon.com/organizations/latest/APIReference//Account>
///
/// ## Coverage
/// 7 of 8 fields included.
/// Omitted fields:
/// - `State` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    /// The unique identifier (ID) of the account. The regex pattern for an account ID string
    /// requires exactly 12 digits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The Amazon Resource Name (ARN) of the account. For more information about ARNs in
    /// Organizations, see ARN Formats Supported by Organizations in the Amazon Web Services
    /// Service Authorization Reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The email address associated with the Amazon Web Services account. The regex pattern for
    /// this parameter is a string of characters that represents a standard internet email
    /// address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The friendly name of the account. The regex pattern that is used to validate this
    /// parameter is a string of any of the characters in the ASCII character range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The status of the account in the organization. The Status parameter in the Account
    /// object will be retired on September 9, 2026. Although both the account State and account
    /// Status parameters are currently available in the Organizations APIs (DescribeAccount,
    /// ListAccounts, ListAccountsForParent), we recommend that you update your scripts or other
    /// code to use the State parameter instead of Status before September 9, 2026.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AccountStatus>,

    /// The method by which the account joined the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_method: Option<AccountJoinedMethod>,

    /// The date the account became a part of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_timestamp: Option<String>,
}

impl Account {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            arn: Some("test-arn".into()),
            email: Some("test-email".into()),
            name: Some("test-name".into()),
            joined_timestamp: Some("test-joined_timestamp".into()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `organizations.v1.ListAccountsForParentResponse`
/// **Reference**: <https://docs.aws.amazon.com/organizations/latest/APIReference//ListAccountsForParentResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAccountsForParentResponse {
    /// A list of the accounts in the specified root or OU. The Status parameter in the API
    /// response will be retired on September 9, 2026. Although both the account State and
    /// account Status parameters are currently available in the Organizations APIs
    /// (DescribeAccount, ListAccounts, ListAccountsForParent), we recommend that you update
    /// your scripts or other code to use the State parameter instead of Status before September
    /// 9, 2026.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<Account>,

    /// If present, indicates that more output is available than is included in the current
    /// response. Use this value in the NextToken request parameter in a subsequent call to the
    /// operation to get the next part of the output. You should repeat this until the NextToken
    /// response element comes back as null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListAccountsForParentResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            accounts: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

///
/// **AWS API**: `organizations.v1.ListOrganizationalUnitsForParentRequest`
/// **Reference**: <https://docs.aws.amazon.com/organizations/latest/APIReference//ListOrganizationalUnitsForParentRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListOrganizationalUnitsForParentRequest {
    /// ID for the root or OU whose child OUs you want to list. The regex pattern for a parent
    /// ID string requires one of the following: Root
    /// - A string that begins with "r-" followed by from 4 to 32 lowercase letters or digits.
    ///   Organizational unit (OU)
    /// - A string that begins with "ou-" followed by from 4 to 32 lowercase letters or digits
    ///   (the ID of the root that the OU is in). This string is followed by a second "-" dash
    ///   and from 8 to 32 additional lowercase letters or digits.
    pub parent_id: String,

    /// The parameter for receiving additional results if you receive a NextToken response in a
    /// previous request. A NextToken response indicates that more output is available. Set this
    /// parameter to the value of the previous call's NextToken response to indicate where the
    /// output should continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return in the response. If more results exist than the
    /// specified MaxResults value, a token is included in the response so that you can retrieve
    /// the remaining results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl ListOrganizationalUnitsForParentRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            parent_id: "test-parent_id".into(),
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}

///
/// **AWS API**: `organizations.v1.ListAccountsForParentRequest`
/// **Reference**: <https://docs.aws.amazon.com/organizations/latest/APIReference//ListAccountsForParentRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAccountsForParentRequest {
    /// The unique identifier (ID) for the parent root or organization unit (OU) whose accounts
    /// you want to list.
    pub parent_id: String,

    /// The parameter for receiving additional results if you receive a NextToken response in a
    /// previous request. A NextToken response indicates that more output is available. Set this
    /// parameter to the value of the previous call's NextToken response to indicate where the
    /// output should continue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of items to return in the response. If more results exist than the
    /// specified MaxResults value, a token is included in the response so that you can retrieve
    /// the remaining results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl ListAccountsForParentRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            parent_id: "test-parent_id".into(),
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
        }
    }
}
