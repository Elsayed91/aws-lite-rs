//! AWS Organizations API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::organizations::OrganizationsOps`. This layer adds:
//! - Ergonomic method signatures
//! - Pagination stream helpers

use crate::{
    AwsHttpClient, Result,
    ops::organizations::OrganizationsOps,
    types::organizations::{
        Account, ListAccountsForParentRequest, ListAccountsForParentResponse,
        ListOrganizationalUnitsForParentRequest, ListOrganizationalUnitsForParentResponse,
        OrganizationalUnit,
    },
};

/// Client for the AWS Organizations API
pub struct OrganizationsClient<'a> {
    ops: OrganizationsOps<'a>,
}

impl<'a> OrganizationsClient<'a> {
    /// Create a new AWS Organizations API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: OrganizationsOps::new(client),
        }
    }

    /// Lists the organizational units in a specified parent OU or root.
    pub async fn list_organizational_units_for_parent(
        &self,
        body: &ListOrganizationalUnitsForParentRequest,
    ) -> Result<ListOrganizationalUnitsForParentResponse> {
        self.ops.list_organizational_units_for_parent(body).await
    }

    /// Stream all organizational units for a parent, automatically handling pagination.
    pub fn list_organizational_units_for_parent_stream(
        &self,
        parent_id: &str,
    ) -> impl futures_core::Stream<Item = Result<OrganizationalUnit>> + '_ {
        let parent_id = parent_id.to_string();
        async_stream::try_stream! {
            let mut next_token: Option<String> = None;
            loop {
                let request = ListOrganizationalUnitsForParentRequest {
                    parent_id: parent_id.clone(),
                    next_token: next_token.clone(),
                    ..Default::default()
                };
                let response = self.ops.list_organizational_units_for_parent(&request).await?;
                for ou in response.organizational_units {
                    yield ou;
                }
                match response.next_token {
                    Some(token) if !token.is_empty() => next_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Lists the accounts in an organization that are contained by the specified parent OU or root.
    pub async fn list_accounts_for_parent(
        &self,
        body: &ListAccountsForParentRequest,
    ) -> Result<ListAccountsForParentResponse> {
        self.ops.list_accounts_for_parent(body).await
    }

    /// Stream all accounts for a parent, automatically handling pagination.
    pub fn list_accounts_for_parent_stream(
        &self,
        parent_id: &str,
    ) -> impl futures_core::Stream<Item = Result<Account>> + '_ {
        let parent_id = parent_id.to_string();
        async_stream::try_stream! {
            let mut next_token: Option<String> = None;
            loop {
                let request = ListAccountsForParentRequest {
                    parent_id: parent_id.clone(),
                    next_token: next_token.clone(),
                    ..Default::default()
                };
                let response = self.ops.list_accounts_for_parent(&request).await?;
                for account in response.accounts {
                    yield account;
                }
                match response.next_token {
                    Some(token) if !token.is_empty() => next_token = Some(token),
                    _ => break,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_client::MockClient;
    use crate::test_support::organizations_mock_helpers::OrganizationsMockHelpers;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn list_organizational_units_for_parent_returns_ous() {
        let mut mock = MockClient::new();
        mock.expect_list_organizational_units_for_parent()
            .returning_json(serde_json::json!({
                "OrganizationalUnits": [
                    {"Id": "ou-root-11111111", "Arn": "arn:aws:organizations::111:ou/o-abc/ou-root-11111111", "Name": "Engineering"}
                ]
            }));

        let client = crate::AwsHttpClient::from_mock(mock);
        let response = client
            .organizations()
            .list_organizational_units_for_parent(&ListOrganizationalUnitsForParentRequest {
                parent_id: "r-root".into(),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(response.organizational_units.len(), 1);
        assert_eq!(
            response.organizational_units[0].id.as_deref(),
            Some("ou-root-11111111")
        );
        assert_eq!(
            response.organizational_units[0].name.as_deref(),
            Some("Engineering")
        );
    }

    #[tokio::test]
    async fn list_organizational_units_for_parent_empty() {
        let mut mock = MockClient::new();
        mock.expect_list_organizational_units_for_parent()
            .returning_json(serde_json::json!({
                "OrganizationalUnits": []
            }));

        let client = crate::AwsHttpClient::from_mock(mock);
        let response = client
            .organizations()
            .list_organizational_units_for_parent(&ListOrganizationalUnitsForParentRequest {
                parent_id: "r-root".into(),
                ..Default::default()
            })
            .await
            .unwrap();

        assert!(response.organizational_units.is_empty());
    }

    #[tokio::test]
    async fn list_organizational_units_stream_paginates() {
        let mut mock = MockClient::new();
        mock.expect_list_organizational_units_for_parent()
            .returning_json_sequence(vec![
                serde_json::json!({
                    "OrganizationalUnits": [
                        {"Id": "ou-root-page1", "Name": "Page1"}
                    ],
                    "NextToken": "token-page2"
                }),
                serde_json::json!({
                    "OrganizationalUnits": [
                        {"Id": "ou-root-page2", "Name": "Page2"}
                    ]
                }),
            ])
            .times(2);

        let client = crate::AwsHttpClient::from_mock(mock);
        let ous: Vec<OrganizationalUnit> = client
            .organizations()
            .list_organizational_units_for_parent_stream("r-root")
            .map(|r| r.unwrap())
            .collect()
            .await;

        assert_eq!(ous.len(), 2);
        assert_eq!(ous[0].id.as_deref(), Some("ou-root-page1"));
        assert_eq!(ous[1].id.as_deref(), Some("ou-root-page2"));
    }

    #[tokio::test]
    async fn list_accounts_for_parent_returns_accounts() {
        let mut mock = MockClient::new();
        mock.expect_list_accounts_for_parent()
            .returning_json(serde_json::json!({
                "Accounts": [
                    {
                        "Id": "123456789012",
                        "Arn": "arn:aws:organizations::111:account/o-abc/123456789012",
                        "Email": "dev@example.com",
                        "Name": "Dev Account",
                        "Status": "ACTIVE",
                        "JoinedMethod": "CREATED",
                        "JoinedTimestamp": "2021-01-01T00:00:00Z"
                    }
                ]
            }));

        let client = crate::AwsHttpClient::from_mock(mock);
        let response = client
            .organizations()
            .list_accounts_for_parent(&ListAccountsForParentRequest {
                parent_id: "ou-root-11111111".into(),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(response.accounts.len(), 1);
        let acct = &response.accounts[0];
        assert_eq!(acct.id.as_deref(), Some("123456789012"));
        assert_eq!(acct.name.as_deref(), Some("Dev Account"));
        assert_eq!(
            acct.status,
            Some(crate::types::organizations::AccountStatus::Active)
        );
        assert_eq!(
            acct.joined_method,
            Some(crate::types::organizations::AccountJoinedMethod::Created)
        );
    }

    #[tokio::test]
    async fn list_accounts_for_parent_empty() {
        let mut mock = MockClient::new();
        mock.expect_list_accounts_for_parent()
            .returning_json(serde_json::json!({
                "Accounts": []
            }));

        let client = crate::AwsHttpClient::from_mock(mock);
        let response = client
            .organizations()
            .list_accounts_for_parent(&ListAccountsForParentRequest {
                parent_id: "ou-root-11111111".into(),
                ..Default::default()
            })
            .await
            .unwrap();

        assert!(response.accounts.is_empty());
    }

    #[tokio::test]
    async fn list_accounts_stream_paginates() {
        let mut mock = MockClient::new();
        mock.expect_list_accounts_for_parent()
            .returning_json_sequence(vec![
                serde_json::json!({
                    "Accounts": [
                        {"Id": "111111111111", "Name": "Account 1", "Status": "ACTIVE"}
                    ],
                    "NextToken": "token-page2"
                }),
                serde_json::json!({
                    "Accounts": [
                        {"Id": "222222222222", "Name": "Account 2", "Status": "ACTIVE"}
                    ]
                }),
            ])
            .times(2);

        let client = crate::AwsHttpClient::from_mock(mock);
        let accounts: Vec<Account> = client
            .organizations()
            .list_accounts_for_parent_stream("ou-root-11111111")
            .map(|r| r.unwrap())
            .collect()
            .await;

        assert_eq!(accounts.len(), 2);
        assert_eq!(accounts[0].id.as_deref(), Some("111111111111"));
        assert_eq!(accounts[1].id.as_deref(), Some("222222222222"));
    }
}
