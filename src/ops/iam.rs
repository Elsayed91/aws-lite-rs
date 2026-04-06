//! Operation contracts for the AWS Identity and Access Management API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/iam.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::iam::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the AWS Identity and Access Management API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::iam::IamClient`] instead.
pub struct IamOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> IamOps<'a> {
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://iam.amazonaws.com"
    }

    /// Lists all IAM users in the account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListUsersRequest`]
    ///
    /// # Response
    /// [`ListUsersResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_users(&self, body: &ListUsersRequest) -> Result<ListUsersResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("ListUsers", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_users response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_users response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListUsersResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_users XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Lists all managed policies attached to the specified IAM user.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListAttachedUserPoliciesRequest`]
    ///
    /// # Response
    /// [`ListAttachedUserPoliciesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_attached_user_policies(
        &self,
        body: &ListAttachedUserPoliciesRequest,
    ) -> Result<ListAttachedUserPoliciesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("ListAttachedUserPolicies", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_attached_user_policies response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_attached_user_policies response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListAttachedUserPoliciesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_attached_user_policies XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Removes the specified managed policy from the specified IAM user.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DetachUserPolicyRequest`]
    #[allow(dead_code)]
    pub(crate) async fn detach_user_policy(&self, body: &DetachUserPolicyRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("DetachUserPolicy", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Deletes the access key pair associated with the specified IAM user.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteAccessKeyRequest`]
    #[allow(dead_code)]
    pub(crate) async fn delete_access_key(&self, body: &DeleteAccessKeyRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("DeleteAccessKey", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Lists the access key IDs associated with the specified IAM user.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListAccessKeysRequest`]
    ///
    /// # Response
    /// [`ListAccessKeysResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_access_keys(
        &self,
        body: &ListAccessKeysRequest,
    ) -> Result<ListAccessKeysResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("ListAccessKeys", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_access_keys response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_access_keys response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListAccessKeysResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_access_keys XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Retrieves information about when the specified access key was last used.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetAccessKeyLastUsedRequest`]
    ///
    /// # Response
    /// [`GetAccessKeyLastUsedResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_access_key_last_used(
        &self,
        body: &GetAccessKeyLastUsedRequest,
    ) -> Result<GetAccessKeyLastUsedResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("GetAccessKeyLastUsed", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_access_key_last_used response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_access_key_last_used response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetAccessKeyLastUsedResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_access_key_last_used XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Generates a credential report for the AWS account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Response
    /// [`GenerateCredentialReportResponse`]
    #[allow(dead_code)]
    pub(crate) async fn generate_credential_report(
        &self,
    ) -> Result<GenerateCredentialReportResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("GenerateCredentialReport", "2010-05-08", None::<&()>);
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read generate_credential_report response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in generate_credential_report response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GenerateCredentialReportResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse generate_credential_report XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Retrieves a credential report for the AWS account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Response
    /// [`GetCredentialReportResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_credential_report(&self) -> Result<GetCredentialReportResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("GetCredentialReport", "2010-05-08", None::<&()>);
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_credential_report response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_credential_report response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetCredentialReportResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_credential_report XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Changes the status of the specified access key from Active to Inactive, or vice versa.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`UpdateAccessKeyRequest`]
    #[allow(dead_code)]
    pub(crate) async fn update_access_key(&self, body: &UpdateAccessKeyRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("UpdateAccessKey", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Lists the MFA devices for an IAM user.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListMFADevicesRequest`]
    ///
    /// # Response
    /// [`ListMFADevicesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_mfa_devices(
        &self,
        body: &ListMFADevicesRequest,
    ) -> Result<ListMFADevicesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("ListMFADevices", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_mfa_devices response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_mfa_devices response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListMFADevicesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_mfa_devices XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Retrieves the login profile for the specified IAM user.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetLoginProfileRequest`]
    ///
    /// # Response
    /// [`GetLoginProfileResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_login_profile(
        &self,
        body: &GetLoginProfileRequest,
    ) -> Result<GetLoginProfileResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("GetLoginProfile", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_login_profile response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_login_profile response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetLoginProfileResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_login_profile XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Retrieves information about IAM entity usage and IAM quotas in the AWS account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Response
    /// [`GetAccountSummaryResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_account_summary(&self) -> Result<GetAccountSummaryResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("GetAccountSummary", "2010-05-08", None::<&()>);
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_account_summary response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_account_summary response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetAccountSummaryResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_account_summary XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Retrieves the password policy for the AWS account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Response
    /// [`GetAccountPasswordPolicyResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_account_password_policy(
        &self,
    ) -> Result<GetAccountPasswordPolicyResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("GetAccountPasswordPolicy", "2010-05-08", None::<&()>);
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_account_password_policy response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_account_password_policy response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetAccountPasswordPolicyResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_account_password_policy XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Updates the password policy settings for the AWS account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`UpdateAccountPasswordPolicyRequest`]
    #[allow(dead_code)]
    pub(crate) async fn update_account_password_policy(
        &self,
        body: &UpdateAccountPasswordPolicyRequest,
    ) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("UpdateAccountPasswordPolicy", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Lists the IAM roles that have the specified path prefix.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListRolesRequest`]
    ///
    /// # Response
    /// [`ListRolesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_roles(&self, body: &ListRolesRequest) -> Result<ListRolesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("ListRoles", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_roles response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_roles response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListRolesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_roles XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Lists the names of the inline policies embedded in the specified IAM user.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListUserPoliciesRequest`]
    ///
    /// # Response
    /// [`ListUserPoliciesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_user_policies(
        &self,
        body: &ListUserPoliciesRequest,
    ) -> Result<ListUserPoliciesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("ListUserPolicies", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_user_policies response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_user_policies response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListUserPoliciesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_user_policies XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Lists the IAM groups that the specified IAM user belongs to.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListGroupsForUserRequest`]
    ///
    /// # Response
    /// [`ListGroupsForUserResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_groups_for_user(
        &self,
        body: &ListGroupsForUserRequest,
    ) -> Result<ListGroupsForUserResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("ListGroupsForUser", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_groups_for_user response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_groups_for_user response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListGroupsForUserResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_groups_for_user XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Lists the server certificates stored in IAM.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListServerCertificatesRequest`]
    ///
    /// # Response
    /// [`ListServerCertificatesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_server_certificates(
        &self,
        body: &ListServerCertificatesRequest,
    ) -> Result<ListServerCertificatesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("ListServerCertificates", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_server_certificates response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_server_certificates response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListServerCertificatesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_server_certificates XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Deletes the specified inline policy from the specified IAM user.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteUserPolicyRequest`]
    #[allow(dead_code)]
    pub(crate) async fn delete_user_policy(&self, body: &DeleteUserPolicyRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("DeleteUserPolicy", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Attaches the specified managed policy to the specified IAM role.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`AttachRolePolicyRequest`]
    #[allow(dead_code)]
    pub(crate) async fn attach_role_policy(&self, body: &AttachRolePolicyRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("AttachRolePolicy", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Removes the specified managed policy from the specified IAM role.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DetachRolePolicyRequest`]
    #[allow(dead_code)]
    pub(crate) async fn detach_role_policy(&self, body: &DetachRolePolicyRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("DetachRolePolicy", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Creates an IAM role that is linked to a specific AWS service.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`CreateServiceLinkedRoleRequest`]
    ///
    /// # Response
    /// [`CreateServiceLinkedRoleResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_service_linked_role(
        &self,
        body: &CreateServiceLinkedRoleRequest,
    ) -> Result<CreateServiceLinkedRoleResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("CreateServiceLinkedRole", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read create_service_linked_role response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in create_service_linked_role response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<CreateServiceLinkedRoleResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse create_service_linked_role XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Retrieves the specified inline policy document embedded in the specified IAM user.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetUserPolicyRequest`]
    ///
    /// # Response
    /// [`GetUserPolicyResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_user_policy(
        &self,
        body: &GetUserPolicyRequest,
    ) -> Result<GetUserPolicyResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("GetUserPolicy", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_user_policy response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_user_policy response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetUserPolicyResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_user_policy XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Lists all managed policies that are attached to the specified IAM group.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListAttachedGroupPoliciesRequest`]
    ///
    /// # Response
    /// [`ListAttachedGroupPoliciesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_attached_group_policies(
        &self,
        body: &ListAttachedGroupPoliciesRequest,
    ) -> Result<ListAttachedGroupPoliciesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("ListAttachedGroupPolicies", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_attached_group_policies response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_attached_group_policies response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListAttachedGroupPoliciesResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_attached_group_policies XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Lists the virtual MFA devices defined in the AWS account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListVirtualMFADevicesRequest`]
    ///
    /// # Response
    /// [`ListVirtualMFADevicesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_virtual_mfa_devices(
        &self,
        body: &ListVirtualMFADevicesRequest,
    ) -> Result<ListVirtualMFADevicesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("ListVirtualMFADevices", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_virtual_mfa_devices response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_virtual_mfa_devices response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListVirtualMFADevicesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_virtual_mfa_devices XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Lists IAM policies, optionally filtered by scope (Local/AWS/All).
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListPoliciesRequest`]
    ///
    /// # Response
    /// [`ListPoliciesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_policies(
        &self,
        body: &ListPoliciesRequest,
    ) -> Result<ListPoliciesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("ListPolicies", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_policies response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_policies response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListPoliciesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_policies XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Retrieves information about the specified version of the specified managed policy.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetPolicyVersionRequest`]
    ///
    /// # Response
    /// [`GetPolicyVersionResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_policy_version(
        &self,
        body: &GetPolicyVersionRequest,
    ) -> Result<GetPolicyVersionResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_query_body("GetPolicyVersion", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_policy_version response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_policy_version response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetPolicyVersionResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_policy_version XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Lists all IAM users, groups, and roles that the specified managed policy is attached to.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListEntitiesForPolicyRequest`]
    ///
    /// # Response
    /// [`ListEntitiesForPolicyResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_entities_for_policy(
        &self,
        body: &ListEntitiesForPolicyRequest,
    ) -> Result<ListEntitiesForPolicyResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_query_body("ListEntitiesForPolicy", "2010-05-08", Some(body));
        let response = self
            .client
            .post(
                &url,
                "iam",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_entities_for_policy response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in list_entities_for_policy response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ListEntitiesForPolicyResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse list_entities_for_policy XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_users() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListUsersResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListUsersRequest::fixture();
        let result = ops.list_users(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_attached_user_policies() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListAttachedUserPoliciesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListAttachedUserPoliciesRequest::fixture();
        let result = ops.list_attached_user_policies(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_detach_user_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = DetachUserPolicyRequest::fixture();
        let result = ops.detach_user_policy(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_access_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = DeleteAccessKeyRequest::fixture();
        let result = ops.delete_access_key(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_access_keys() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListAccessKeysResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListAccessKeysRequest::fixture();
        let result = ops.list_access_keys(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_access_key_last_used() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetAccessKeyLastUsedResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = GetAccessKeyLastUsedRequest::fixture();
        let result = ops.get_access_key_last_used(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_credential_report() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GenerateCredentialReportResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let result = ops.generate_credential_report().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_credential_report() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetCredentialReportResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let result = ops.get_credential_report().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_access_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = UpdateAccessKeyRequest::fixture();
        let result = ops.update_access_key(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_mfa_devices() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListMFADevicesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListMFADevicesRequest::fixture();
        let result = ops.list_mfa_devices(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_login_profile() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetLoginProfileResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = GetLoginProfileRequest::fixture();
        let result = ops.get_login_profile(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_account_summary() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetAccountSummaryResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let result = ops.get_account_summary().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_account_password_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetAccountPasswordPolicyResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let result = ops.get_account_password_policy().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_account_password_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = UpdateAccountPasswordPolicyRequest::fixture();
        let result = ops.update_account_password_policy(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_roles() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListRolesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListRolesRequest::fixture();
        let result = ops.list_roles(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_user_policies() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListUserPoliciesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListUserPoliciesRequest::fixture();
        let result = ops.list_user_policies(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_groups_for_user() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListGroupsForUserResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListGroupsForUserRequest::fixture();
        let result = ops.list_groups_for_user(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_server_certificates() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListServerCertificatesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListServerCertificatesRequest::fixture();
        let result = ops.list_server_certificates(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_user_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = DeleteUserPolicyRequest::fixture();
        let result = ops.delete_user_policy(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_attach_role_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = AttachRolePolicyRequest::fixture();
        let result = ops.attach_role_policy(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_detach_role_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = DetachRolePolicyRequest::fixture();
        let result = ops.detach_role_policy(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_service_linked_role() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = CreateServiceLinkedRoleResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = CreateServiceLinkedRoleRequest::fixture();
        let result = ops.create_service_linked_role(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetUserPolicyResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = GetUserPolicyRequest::fixture();
        let result = ops.get_user_policy(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_attached_group_policies() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListAttachedGroupPoliciesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListAttachedGroupPoliciesRequest::fixture();
        let result = ops.list_attached_group_policies(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_virtual_mfa_devices() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListVirtualMFADevicesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListVirtualMFADevicesRequest::fixture();
        let result = ops.list_virtual_mfa_devices(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_policies() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListPoliciesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListPoliciesRequest::fixture();
        let result = ops.list_policies(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_policy_version() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetPolicyVersionResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = GetPolicyVersionRequest::fixture();
        let result = ops.get_policy_version(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_entities_for_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ListEntitiesForPolicyResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = ListEntitiesForPolicyRequest::fixture();
        let result = ops.list_entities_for_policy(&body).await;
        assert!(result.is_ok());
    }
}
