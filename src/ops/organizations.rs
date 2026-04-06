//! Operation contracts for the AWS Organizations API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/organizations.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::organizations::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the AWS Organizations API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::organizations::OrganizationsClient`] instead.
pub struct OrganizationsOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> OrganizationsOps<'a> {
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
        "https://organizations.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Lists the organizational units in a specified parent OU or root.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListOrganizationalUnitsForParentRequest`]
    ///
    /// # Response
    /// [`ListOrganizationalUnitsForParentResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_organizational_units_for_parent(
        &self,
        body: &ListOrganizationalUnitsForParentRequest,
    ) -> Result<ListOrganizationalUnitsForParentResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to serialize list_organizational_units_for_parent request: {e}"
                ),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "organizations",
                "AWSOrganizationsV20161128.ListOrganizationalUnitsForParent",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read list_organizational_units_for_parent response: {e}"
                    ),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_organizational_units_for_parent response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists the accounts in an organization that are contained by the specified parent OU or
    /// root.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListAccountsForParentRequest`]
    ///
    /// # Response
    /// [`ListAccountsForParentResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_accounts_for_parent(
        &self,
        body: &ListAccountsForParentRequest,
    ) -> Result<ListAccountsForParentResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize list_accounts_for_parent request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "organizations",
                "AWSOrganizationsV20161128.ListAccountsForParent",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_accounts_for_parent response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_accounts_for_parent response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_organizational_units_for_parent() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(
            serde_json::to_value(ListOrganizationalUnitsForParentResponse::fixture()).unwrap(),
        );

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = OrganizationsOps::new(&client);

        let body = ListOrganizationalUnitsForParentRequest::fixture();
        let result = ops.list_organizational_units_for_parent(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_accounts_for_parent() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(
            serde_json::to_value(ListAccountsForParentResponse::fixture()).unwrap(),
        );

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = OrganizationsOps::new(&client);

        let body = ListAccountsForParentRequest::fixture();
        let result = ops.list_accounts_for_parent(&body).await;
        assert!(result.is_ok());
    }
}
