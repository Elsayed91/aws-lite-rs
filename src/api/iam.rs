//! AWS Identity and Access Management (IAM) API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::iam::IamOps`. This layer adds:
//! - Ergonomic method signatures
//! - Convenience parameters for common operations

use crate::{
    AwsHttpClient, Result,
    ops::iam::IamOps,
    types::iam::{
        AttachRolePolicyRequest, CreateServiceLinkedRoleRequest, CreateServiceLinkedRoleResponse,
        DeleteAccessKeyRequest, DeleteUserPolicyRequest, DetachRolePolicyRequest,
        DetachUserPolicyRequest, GenerateCredentialReportResponse, GetAccessKeyLastUsedRequest,
        GetAccessKeyLastUsedResponse, GetAccountPasswordPolicyResponse, GetAccountSummaryResponse,
        GetCredentialReportResponse, GetLoginProfileRequest, GetLoginProfileResponse,
        GetPolicyVersionRequest, GetPolicyVersionResponse, GetUserPolicyRequest,
        GetUserPolicyResponse, ListAccessKeysRequest, ListAccessKeysResponse,
        ListAttachedGroupPoliciesRequest, ListAttachedGroupPoliciesResponse,
        ListAttachedUserPoliciesRequest, ListAttachedUserPoliciesResponse,
        ListEntitiesForPolicyRequest, ListEntitiesForPolicyResponse, ListGroupsForUserRequest,
        ListGroupsForUserResponse, ListMFADevicesRequest, ListMFADevicesResponse,
        ListPoliciesRequest, ListPoliciesResponse, ListRolesRequest, ListRolesResponse,
        ListServerCertificatesRequest, ListServerCertificatesResponse, ListUserPoliciesRequest,
        ListUserPoliciesResponse, ListUsersRequest, ListUsersResponse,
        ListVirtualMFADevicesRequest, ListVirtualMFADevicesResponse, Policy,
        UpdateAccessKeyRequest, UpdateAccountPasswordPolicyRequest, VirtualMFADevice,
    },
};

/// Client for the AWS Identity and Access Management API.
pub struct IamClient<'a> {
    ops: IamOps<'a>,
}

impl<'a> IamClient<'a> {
    /// Create a new IAM API client.
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: IamOps::new(client),
        }
    }

    /// List all IAM users in the account.
    pub async fn list_users(&self) -> Result<ListUsersResponse> {
        let body = ListUsersRequest::default();
        self.ops.list_users(&body).await
    }

    /// List all managed policies attached to the specified IAM user.
    pub async fn list_attached_user_policies(
        &self,
        user_name: &str,
    ) -> Result<ListAttachedUserPoliciesResponse> {
        let body = ListAttachedUserPoliciesRequest {
            user_name: user_name.to_string(),
            ..Default::default()
        };
        self.ops.list_attached_user_policies(&body).await
    }

    /// Remove a managed policy from an IAM user.
    pub async fn detach_user_policy(&self, user_name: &str, policy_arn: &str) -> Result<()> {
        let body = DetachUserPolicyRequest {
            user_name: user_name.to_string(),
            policy_arn: policy_arn.to_string(),
        };
        self.ops.detach_user_policy(&body).await
    }

    /// Delete an access key pair for an IAM user.
    pub async fn delete_access_key(&self, user_name: &str, access_key_id: &str) -> Result<()> {
        let body = DeleteAccessKeyRequest {
            user_name: Some(user_name.to_string()),
            access_key_id: access_key_id.to_string(),
        };
        self.ops.delete_access_key(&body).await
    }

    /// List access keys for an IAM user.
    pub async fn list_access_keys(&self, user_name: &str) -> Result<ListAccessKeysResponse> {
        let body = ListAccessKeysRequest {
            user_name: Some(user_name.to_string()),
            ..Default::default()
        };
        self.ops.list_access_keys(&body).await
    }

    /// Retrieve information about when an access key was last used.
    pub async fn get_access_key_last_used(
        &self,
        access_key_id: &str,
    ) -> Result<GetAccessKeyLastUsedResponse> {
        let body = GetAccessKeyLastUsedRequest {
            access_key_id: access_key_id.to_string(),
        };
        self.ops.get_access_key_last_used(&body).await
    }

    /// Generate a credential report for the AWS account.
    pub async fn generate_credential_report(&self) -> Result<GenerateCredentialReportResponse> {
        self.ops.generate_credential_report().await
    }

    /// Retrieve a credential report for the AWS account.
    pub async fn get_credential_report(&self) -> Result<GetCredentialReportResponse> {
        self.ops.get_credential_report().await
    }

    /// Change the status of an access key from Active to Inactive, or vice versa.
    pub async fn update_access_key(
        &self,
        user_name: &str,
        access_key_id: &str,
        status: &str,
    ) -> Result<()> {
        let body = UpdateAccessKeyRequest {
            user_name: Some(user_name.to_string()),
            access_key_id: access_key_id.to_string(),
            status: status.to_string(),
        };
        self.ops.update_access_key(&body).await
    }

    /// List MFA devices for an IAM user.
    pub async fn list_mfa_devices(&self, user_name: &str) -> Result<ListMFADevicesResponse> {
        let body = ListMFADevicesRequest {
            user_name: Some(user_name.to_string()),
            ..Default::default()
        };
        self.ops.list_mfa_devices(&body).await
    }

    /// Get the login profile (console password) for an IAM user.
    pub async fn get_login_profile(&self, user_name: &str) -> Result<GetLoginProfileResponse> {
        let body = GetLoginProfileRequest {
            user_name: Some(user_name.to_string()),
        };
        self.ops.get_login_profile(&body).await
    }

    /// Get the account-level summary of IAM entity usage and quotas.
    pub async fn get_account_summary(&self) -> Result<GetAccountSummaryResponse> {
        self.ops.get_account_summary().await
    }

    /// Get the account password policy.
    pub async fn get_account_password_policy(&self) -> Result<GetAccountPasswordPolicyResponse> {
        self.ops.get_account_password_policy().await
    }

    /// Update the account password policy.
    pub async fn update_account_password_policy(
        &self,
        body: &UpdateAccountPasswordPolicyRequest,
    ) -> Result<()> {
        self.ops.update_account_password_policy(body).await
    }

    /// List all IAM roles in the account.
    pub async fn list_roles(&self) -> Result<ListRolesResponse> {
        let body = ListRolesRequest::default();
        self.ops.list_roles(&body).await
    }

    /// List inline policy names for an IAM user.
    pub async fn list_user_policies(&self, user_name: &str) -> Result<ListUserPoliciesResponse> {
        let body = ListUserPoliciesRequest {
            user_name: user_name.to_string(),
            ..Default::default()
        };
        self.ops.list_user_policies(&body).await
    }

    /// List the groups that an IAM user belongs to.
    pub async fn list_groups_for_user(&self, user_name: &str) -> Result<ListGroupsForUserResponse> {
        let body = ListGroupsForUserRequest {
            user_name: user_name.to_string(),
            ..Default::default()
        };
        self.ops.list_groups_for_user(&body).await
    }

    /// List all server certificates in the account.
    pub async fn list_server_certificates(&self) -> Result<ListServerCertificatesResponse> {
        let body = ListServerCertificatesRequest::default();
        self.ops.list_server_certificates(&body).await
    }

    /// Delete an inline policy from an IAM user.
    pub async fn delete_user_policy(&self, user_name: &str, policy_name: &str) -> Result<()> {
        let body = DeleteUserPolicyRequest {
            user_name: user_name.to_string(),
            policy_name: policy_name.to_string(),
        };
        self.ops.delete_user_policy(&body).await
    }

    /// Attach a managed policy to an IAM role.
    pub async fn attach_role_policy(&self, role_name: &str, policy_arn: &str) -> Result<()> {
        let body = AttachRolePolicyRequest {
            role_name: role_name.to_string(),
            policy_arn: policy_arn.to_string(),
        };
        self.ops.attach_role_policy(&body).await
    }

    /// Detach a managed policy from an IAM role.
    pub async fn detach_role_policy(&self, role_name: &str, policy_arn: &str) -> Result<()> {
        let body = DetachRolePolicyRequest {
            role_name: role_name.to_string(),
            policy_arn: policy_arn.to_string(),
        };
        self.ops.detach_role_policy(&body).await
    }

    /// Create a service-linked role for an AWS service.
    pub async fn create_service_linked_role(
        &self,
        aws_service_name: &str,
    ) -> Result<CreateServiceLinkedRoleResponse> {
        let body = CreateServiceLinkedRoleRequest {
            aws_service_name: aws_service_name.to_string(),
            ..Default::default()
        };
        self.ops.create_service_linked_role(&body).await
    }

    /// Retrieve an inline policy document embedded in an IAM user.
    pub async fn get_user_policy(
        &self,
        user_name: &str,
        policy_name: &str,
    ) -> Result<GetUserPolicyResponse> {
        let body = GetUserPolicyRequest {
            user_name: user_name.to_string(),
            policy_name: policy_name.to_string(),
        };
        self.ops.get_user_policy(&body).await
    }

    /// List all managed policies attached to an IAM group.
    pub async fn list_attached_group_policies(
        &self,
        group_name: &str,
    ) -> Result<ListAttachedGroupPoliciesResponse> {
        let body = ListAttachedGroupPoliciesRequest {
            group_name: group_name.to_string(),
            ..Default::default()
        };
        self.ops.list_attached_group_policies(&body).await
    }

    // ── Virtual MFA Devices ────────────────────────────────────────────

    /// Return the first page of virtual MFA devices.
    ///
    /// `assignment_status`: optional filter — `"Assigned"`, `"Unassigned"`, or
    /// `"Any"` (default when `None`).
    pub async fn list_virtual_mfa_devices(
        &self,
        assignment_status: Option<&str>,
    ) -> Result<ListVirtualMFADevicesResponse> {
        let body = ListVirtualMFADevicesRequest {
            assignment_status: assignment_status.map(str::to_string),
            ..Default::default()
        };
        self.ops.list_virtual_mfa_devices(&body).await
    }

    /// Return all virtual MFA devices in the account, following pagination.
    ///
    /// CIS 2.5: the root account should use a hardware MFA device, not a
    /// virtual one. Any `VirtualMFADevice` whose serial number contains
    /// `"root-account-mfa-device"` indicates virtual MFA on root.
    pub async fn list_all_virtual_mfa_devices(&self) -> Result<Vec<VirtualMFADevice>> {
        let mut all = Vec::new();
        let mut marker: Option<String> = None;
        loop {
            let body = ListVirtualMFADevicesRequest {
                marker: marker.clone(),
                ..Default::default()
            };
            let resp = self.ops.list_virtual_mfa_devices(&body).await?;
            all.extend(resp.virtual_mfa_devices);
            match resp.marker {
                Some(m) if !m.is_empty() && resp.is_truncated == Some(true) => {
                    marker = Some(m);
                }
                _ => break,
            }
        }
        Ok(all)
    }

    // ── Managed Policies ───────────────────────────────────────────────

    /// Return the first page of IAM policies.
    ///
    /// `scope`: `"Local"` (customer-managed), `"AWS"` (AWS-managed), or
    /// `"All"` (default when `None`).
    pub async fn list_policies(
        &self,
        scope: Option<&str>,
        marker: Option<&str>,
    ) -> Result<ListPoliciesResponse> {
        let body = ListPoliciesRequest {
            scope: scope.map(str::to_string),
            marker: marker.map(str::to_string),
            ..Default::default()
        };
        self.ops.list_policies(&body).await
    }

    /// Return all IAM policies for the given scope, following pagination.
    ///
    /// CIS 2.15: pass `scope = Some("Local")` to enumerate customer-managed
    /// policies, then call `get_policy_version` on each to inspect for
    /// unrestricted `"*:*"` statements.
    pub async fn list_all_policies(&self, scope: Option<&str>) -> Result<Vec<Policy>> {
        let mut all = Vec::new();
        let mut marker: Option<String> = None;
        loop {
            let body = ListPoliciesRequest {
                scope: scope.map(str::to_string),
                marker: marker.clone(),
                ..Default::default()
            };
            let resp = self.ops.list_policies(&body).await?;
            all.extend(resp.policies);
            match resp.marker {
                Some(m) if !m.is_empty() && resp.is_truncated == Some(true) => {
                    marker = Some(m);
                }
                _ => break,
            }
        }
        Ok(all)
    }

    /// Retrieve a specific version of a managed policy.
    ///
    /// The `document` field in the response is URL-encoded JSON. Decode it
    /// with `urlencoding::decode` before parsing.
    ///
    /// CIS 2.15: inspect the document for statements allowing `"*"` actions
    /// on `"*"` resources to detect policies with full admin access.
    pub async fn get_policy_version(
        &self,
        policy_arn: &str,
        version_id: &str,
    ) -> Result<GetPolicyVersionResponse> {
        let body = GetPolicyVersionRequest {
            policy_arn: policy_arn.to_string(),
            version_id: version_id.to_string(),
        };
        self.ops.get_policy_version(&body).await
    }

    // ── Entities For Policy ────────────────────────────────────────────

    /// Return the first page of entities (groups, users, roles) attached to a
    /// managed policy.
    pub async fn list_entities_for_policy(
        &self,
        policy_arn: &str,
    ) -> Result<ListEntitiesForPolicyResponse> {
        let body = ListEntitiesForPolicyRequest {
            policy_arn: policy_arn.to_string(),
            ..Default::default()
        };
        self.ops.list_entities_for_policy(&body).await
    }

    /// Return all entities attached to a managed policy, following pagination.
    ///
    /// Merges `PolicyGroups`, `PolicyUsers`, and `PolicyRoles` across all
    /// pages into a single response.
    ///
    /// CIS 2.16: verify at least one entity is attached to `AWSSupportAccess`.
    /// CIS 2.21: verify no entity is attached to `AWSCloudShellFullAccess`
    /// (or only approved entities).
    pub async fn list_all_entities_for_policy(
        &self,
        policy_arn: &str,
    ) -> Result<ListEntitiesForPolicyResponse> {
        let mut groups = Vec::new();
        let mut users = Vec::new();
        let mut roles = Vec::new();
        let mut marker: Option<String> = None;
        loop {
            let body = ListEntitiesForPolicyRequest {
                policy_arn: policy_arn.to_string(),
                marker: marker.clone(),
                ..Default::default()
            };
            let resp = self.ops.list_entities_for_policy(&body).await?;
            groups.extend(resp.policy_groups);
            users.extend(resp.policy_users);
            roles.extend(resp.policy_roles);
            match resp.marker {
                Some(m) if !m.is_empty() && resp.is_truncated == Some(true) => {
                    marker = Some(m);
                }
                _ => break,
            }
        }
        Ok(ListEntitiesForPolicyResponse {
            policy_groups: groups,
            policy_users: users,
            policy_roles: roles,
            is_truncated: Some(false),
            marker: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::AwsHttpClient;
    use crate::mock_client::MockClient;
    use crate::test_support::iam_mock_helpers::IamMockHelpers;

    fn xml_envelope(action: &str, inner: &str) -> Vec<u8> {
        format!("<{action}Response><{action}Result>{inner}</{action}Result></{action}Response>")
            .into_bytes()
    }

    #[tokio::test]
    async fn list_users_returns_parsed_users() {
        let mut mock = MockClient::new();
        mock.expect_list_users().returning_bytes(xml_envelope(
            "ListUsers",
            "<Users>\
               <member>\
                 <Arn>arn:aws:iam::123456789012:user/alice</Arn>\
                 <UserName>alice</UserName>\
                 <CreateDate>2024-01-15T10:30:00Z</CreateDate>\
               </member>\
             </Users>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_users().await.unwrap();

        assert_eq!(response.users.len(), 1);
        assert_eq!(response.users[0].user_name, "alice");
        assert_eq!(
            response.users[0].arn,
            "arn:aws:iam::123456789012:user/alice"
        );
        assert_eq!(response.users[0].create_date, "2024-01-15T10:30:00Z");
    }

    #[tokio::test]
    async fn list_users_handles_empty_response() {
        let mut mock = MockClient::new();
        mock.expect_list_users()
            .returning_bytes(xml_envelope("ListUsers", "<Users/>"));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_users().await.unwrap();
        assert!(response.users.is_empty());
    }

    #[tokio::test]
    async fn list_attached_user_policies_returns_policies() {
        let mut mock = MockClient::new();
        mock.expect_list_attached_user_policies()
            .returning_bytes(xml_envelope(
                "ListAttachedUserPolicies",
                "<AttachedPolicies>\
                   <member>\
                     <PolicyArn>arn:aws:iam::aws:policy/ReadOnlyAccess</PolicyArn>\
                     <PolicyName>ReadOnlyAccess</PolicyName>\
                   </member>\
                 </AttachedPolicies>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .iam()
            .list_attached_user_policies("alice")
            .await
            .unwrap();

        assert_eq!(response.attached_policies.len(), 1);
        assert_eq!(
            response.attached_policies[0].policy_arn.as_deref(),
            Some("arn:aws:iam::aws:policy/ReadOnlyAccess")
        );
        assert_eq!(
            response.attached_policies[0].policy_name.as_deref(),
            Some("ReadOnlyAccess")
        );
    }

    #[tokio::test]
    async fn detach_user_policy_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_detach_user_policy()
            .returning_bytes(xml_envelope("DetachUserPolicy", ""));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .iam()
            .detach_user_policy("alice", "arn:aws:iam::aws:policy/ReadOnlyAccess")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_access_key_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete_access_key()
            .returning_bytes(xml_envelope("DeleteAccessKey", ""));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .iam()
            .delete_access_key("alice", "AKIAIOSFODNN7EXAMPLE")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn list_access_keys_returns_keys() {
        let mut mock = MockClient::new();
        mock.expect_list_access_keys().returning_bytes(xml_envelope(
            "ListAccessKeys",
            "<AccessKeyMetadata>\
               <member>\
                 <UserName>alice</UserName>\
                 <AccessKeyId>AKIAIOSFODNN7EXAMPLE</AccessKeyId>\
                 <Status>Active</Status>\
                 <CreateDate>2024-03-01T12:00:00Z</CreateDate>\
               </member>\
             </AccessKeyMetadata>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_access_keys("alice").await.unwrap();

        assert_eq!(response.access_key_metadata.len(), 1);
        assert_eq!(
            response.access_key_metadata[0].access_key_id.as_deref(),
            Some("AKIAIOSFODNN7EXAMPLE")
        );
        assert_eq!(
            response.access_key_metadata[0].user_name.as_deref(),
            Some("alice")
        );
    }

    #[tokio::test]
    async fn get_access_key_last_used_returns_info() {
        let mut mock = MockClient::new();
        mock.expect_get_access_key_last_used()
            .returning_bytes(xml_envelope(
                "GetAccessKeyLastUsed",
                "<UserName>alice</UserName>\
                 <AccessKeyLastUsed>\
                   <ServiceName>s3</ServiceName>\
                   <Region>us-east-1</Region>\
                   <LastUsedDate>2024-06-15T08:00:00Z</LastUsedDate>\
                 </AccessKeyLastUsed>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .iam()
            .get_access_key_last_used("AKIAIOSFODNN7EXAMPLE")
            .await
            .unwrap();

        assert_eq!(response.user_name.as_deref(), Some("alice"));
        let last_used = response.access_key_last_used.unwrap();
        assert_eq!(last_used.service_name, "s3");
        assert_eq!(last_used.region, "us-east-1");
        assert_eq!(
            last_used.last_used_date.as_deref(),
            Some("2024-06-15T08:00:00Z")
        );
    }

    #[tokio::test]
    async fn generate_credential_report_returns_state() {
        let mut mock = MockClient::new();
        mock.expect_generate_credential_report()
            .returning_bytes(xml_envelope(
                "GenerateCredentialReport",
                "<State>STARTED</State>\
                 <Description>Starting report generation</Description>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().generate_credential_report().await.unwrap();

        assert_eq!(
            response.state,
            Some(crate::types::iam::ReportStateType::Started)
        );
        assert_eq!(
            response.description.as_deref(),
            Some("Starting report generation")
        );
    }

    #[tokio::test]
    async fn get_credential_report_returns_content() {
        let mut mock = MockClient::new();
        mock.expect_get_credential_report()
            .returning_bytes(xml_envelope(
                "GetCredentialReport",
                "<Content>dXNlcixhcm4K</Content>\
                 <ReportFormat>text/csv</ReportFormat>\
                 <GeneratedTime>2024-06-15T10:00:00Z</GeneratedTime>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().get_credential_report().await.unwrap();

        // Content is base64 decoded automatically by serde
        assert_eq!(response.content.as_deref(), Some("user,arn\n"));
        assert_eq!(
            response.report_format,
            Some(crate::types::iam::ReportFormatType::TextPercsv)
        );
        assert_eq!(
            response.generated_time.as_deref(),
            Some("2024-06-15T10:00:00Z")
        );
    }

    #[tokio::test]
    async fn update_access_key_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_update_access_key()
            .returning_bytes(xml_envelope("UpdateAccessKey", ""));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .iam()
            .update_access_key("alice", "AKIAIOSFODNN7EXAMPLE", "Inactive")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn list_mfa_devices_returns_devices() {
        let mut mock = MockClient::new();
        mock.expect_list_mfa_devices().returning_bytes(xml_envelope(
            "ListMFADevices",
            "<MFADevices>\
               <member>\
                 <UserName>alice</UserName>\
                 <SerialNumber>arn:aws:iam::123456789012:mfa/alice</SerialNumber>\
                 <EnableDate>2024-01-15T10:00:00Z</EnableDate>\
               </member>\
             </MFADevices>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_mfa_devices("alice").await.unwrap();

        assert_eq!(response.mfa_devices.len(), 1);
        assert_eq!(response.mfa_devices[0].user_name, "alice");
        assert_eq!(
            response.mfa_devices[0].serial_number,
            "arn:aws:iam::123456789012:mfa/alice"
        );
        assert_eq!(response.mfa_devices[0].enable_date, "2024-01-15T10:00:00Z");
    }

    #[tokio::test]
    async fn get_login_profile_returns_profile() {
        let mut mock = MockClient::new();
        mock.expect_get_login_profile()
            .returning_bytes(xml_envelope(
                "GetLoginProfile",
                "<LoginProfile>\
                   <UserName>alice</UserName>\
                   <CreateDate>2024-01-15T10:30:00Z</CreateDate>\
                   <PasswordResetRequired>false</PasswordResetRequired>\
                 </LoginProfile>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().get_login_profile("alice").await.unwrap();

        assert_eq!(response.login_profile.user_name, "alice");
        assert_eq!(response.login_profile.create_date, "2024-01-15T10:30:00Z");
        assert_eq!(response.login_profile.password_reset_required, Some(false));
    }

    #[tokio::test]
    async fn get_account_summary_returns_map() {
        let mut mock = MockClient::new();
        mock.expect_get_account_summary()
            .returning_bytes(xml_envelope(
                "GetAccountSummary",
                "<SummaryMap>\
                   <entry><key>Users</key><value>5</value></entry>\
                   <entry><key>Roles</key><value>12</value></entry>\
                   <entry><key>AccountMFAEnabled</key><value>1</value></entry>\
                 </SummaryMap>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().get_account_summary().await.unwrap();

        assert_eq!(response.summary_map.len(), 3);
        assert_eq!(response.summary_map.get("Users"), Some(&5));
        assert_eq!(response.summary_map.get("Roles"), Some(&12));
        assert_eq!(response.summary_map.get("AccountMFAEnabled"), Some(&1));
    }

    #[tokio::test]
    async fn get_account_password_policy_returns_policy() {
        let mut mock = MockClient::new();
        mock.expect_get_account_password_policy()
            .returning_bytes(xml_envelope(
                "GetAccountPasswordPolicy",
                "<PasswordPolicy>\
                   <MinimumPasswordLength>14</MinimumPasswordLength>\
                   <RequireSymbols>true</RequireSymbols>\
                   <RequireNumbers>true</RequireNumbers>\
                   <RequireUppercaseCharacters>true</RequireUppercaseCharacters>\
                   <RequireLowercaseCharacters>true</RequireLowercaseCharacters>\
                   <MaxPasswordAge>90</MaxPasswordAge>\
                 </PasswordPolicy>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().get_account_password_policy().await.unwrap();

        let policy = &response.password_policy;
        assert_eq!(policy.minimum_password_length, Some(14));
        assert_eq!(policy.require_symbols, Some(true));
        assert_eq!(policy.require_numbers, Some(true));
        assert_eq!(policy.require_uppercase_characters, Some(true));
        assert_eq!(policy.require_lowercase_characters, Some(true));
        assert_eq!(policy.max_password_age, Some(90));
    }

    #[tokio::test]
    async fn update_account_password_policy_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_update_account_password_policy()
            .returning_bytes(xml_envelope("UpdateAccountPasswordPolicy", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::iam::UpdateAccountPasswordPolicyRequest {
            minimum_password_length: Some(14),
            ..Default::default()
        };
        let result = client.iam().update_account_password_policy(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn list_roles_returns_roles() {
        let mut mock = MockClient::new();
        mock.expect_list_roles().returning_bytes(xml_envelope(
            "ListRoles",
            "<Roles>\
               <member>\
                 <RoleName>admin-role</RoleName>\
                 <Arn>arn:aws:iam::123456789012:role/admin-role</Arn>\
                 <CreateDate>2024-01-15T10:30:00Z</CreateDate>\
                 <Description>Admin role</Description>\
               </member>\
             </Roles>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_roles().await.unwrap();

        assert_eq!(response.roles.len(), 1);
        assert_eq!(response.roles[0].role_name, "admin-role");
        assert_eq!(
            response.roles[0].arn,
            "arn:aws:iam::123456789012:role/admin-role"
        );
        assert_eq!(response.roles[0].create_date, "2024-01-15T10:30:00Z");
        assert_eq!(response.roles[0].description.as_deref(), Some("Admin role"));
    }

    #[tokio::test]
    async fn list_user_policies_returns_names() {
        let mut mock = MockClient::new();
        mock.expect_list_user_policies()
            .returning_bytes(xml_envelope(
                "ListUserPolicies",
                "<PolicyNames>\
                   <member>s3-read-policy</member>\
                   <member>ec2-describe-policy</member>\
                 </PolicyNames>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_user_policies("alice").await.unwrap();

        assert_eq!(response.policy_names.len(), 2);
        assert_eq!(response.policy_names[0], "s3-read-policy");
        assert_eq!(response.policy_names[1], "ec2-describe-policy");
    }

    #[tokio::test]
    async fn list_groups_for_user_returns_groups() {
        let mut mock = MockClient::new();
        mock.expect_list_groups_for_user()
            .returning_bytes(xml_envelope(
                "ListGroupsForUser",
                "<Groups>\
                   <member>\
                     <GroupName>developers</GroupName>\
                     <Arn>arn:aws:iam::123456789012:group/developers</Arn>\
                     <CreateDate>2024-02-01T09:00:00Z</CreateDate>\
                   </member>\
                 </Groups>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_groups_for_user("alice").await.unwrap();

        assert_eq!(response.groups.len(), 1);
        assert_eq!(response.groups[0].group_name, "developers");
        assert_eq!(
            response.groups[0].arn,
            "arn:aws:iam::123456789012:group/developers"
        );
        assert_eq!(response.groups[0].create_date, "2024-02-01T09:00:00Z");
    }

    #[tokio::test]
    async fn list_server_certificates_returns_certs() {
        let mut mock = MockClient::new();
        mock.expect_list_server_certificates()
            .returning_bytes(xml_envelope(
                "ListServerCertificates",
                "<ServerCertificateMetadataList>\
                   <member>\
                     <ServerCertificateName>my-cert</ServerCertificateName>\
                     <Arn>arn:aws:iam::123456789012:server-certificate/my-cert</Arn>\
                     <Expiration>2025-12-31T23:59:59Z</Expiration>\
                     <UploadDate>2024-01-01T00:00:00Z</UploadDate>\
                   </member>\
                 </ServerCertificateMetadataList>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_server_certificates().await.unwrap();

        assert_eq!(response.server_certificate_metadata_list.len(), 1);
        let cert = &response.server_certificate_metadata_list[0];
        assert_eq!(cert.server_certificate_name, "my-cert");
        assert_eq!(
            cert.arn,
            "arn:aws:iam::123456789012:server-certificate/my-cert"
        );
        assert_eq!(cert.expiration.as_deref(), Some("2025-12-31T23:59:59Z"));
        assert_eq!(cert.upload_date.as_deref(), Some("2024-01-01T00:00:00Z"));
    }

    #[tokio::test]
    async fn delete_user_policy_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete_user_policy()
            .returning_bytes(xml_envelope("DeleteUserPolicy", ""));

        let client = AwsHttpClient::from_mock(mock);
        let result = client.iam().delete_user_policy("alice", "my-policy").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn attach_role_policy_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_attach_role_policy()
            .returning_bytes(xml_envelope("AttachRolePolicy", ""));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .iam()
            .attach_role_policy("my-role", "arn:aws:iam::aws:policy/ReadOnlyAccess")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn detach_role_policy_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_detach_role_policy()
            .returning_bytes(xml_envelope("DetachRolePolicy", ""));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .iam()
            .detach_role_policy("my-role", "arn:aws:iam::aws:policy/ReadOnlyAccess")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_service_linked_role_returns_role() {
        let mut mock = MockClient::new();
        mock.expect_create_service_linked_role()
            .returning_bytes(xml_envelope(
                "CreateServiceLinkedRole",
                "<Role>\
                   <RoleName>AWSServiceRoleForElasticBeanstalk</RoleName>\
                   <Arn>arn:aws:iam::123456789012:role/aws-service-role/elasticbeanstalk.amazonaws.com/AWSServiceRoleForElasticBeanstalk</Arn>\
                   <CreateDate>2024-06-15T12:00:00Z</CreateDate>\
                   <Description>Service-linked role for Elastic Beanstalk</Description>\
                 </Role>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .iam()
            .create_service_linked_role("elasticbeanstalk.amazonaws.com")
            .await
            .unwrap();

        let role = response.role.unwrap();
        assert_eq!(role.role_name, "AWSServiceRoleForElasticBeanstalk");
        assert!(role.arn.contains("aws-service-role"));
        assert_eq!(role.create_date, "2024-06-15T12:00:00Z");
        assert_eq!(
            role.description.as_deref(),
            Some("Service-linked role for Elastic Beanstalk")
        );
    }

    #[tokio::test]
    async fn get_user_policy_returns_document() {
        let mut mock = MockClient::new();
        mock.expect_get_user_policy().returning_bytes(xml_envelope(
            "GetUserPolicy",
            "<UserName>alice</UserName>\
             <PolicyName>s3-read-policy</PolicyName>\
             <PolicyDocument>%7B%22Version%22%3A%222012-10-17%22%7D</PolicyDocument>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .iam()
            .get_user_policy("alice", "s3-read-policy")
            .await
            .unwrap();

        assert_eq!(response.user_name, "alice");
        assert_eq!(response.policy_name, "s3-read-policy");
        assert_eq!(
            response.policy_document,
            "%7B%22Version%22%3A%222012-10-17%22%7D"
        );
    }

    #[tokio::test]
    async fn list_attached_group_policies_returns_policies() {
        let mut mock = MockClient::new();
        mock.expect_list_attached_group_policies()
            .returning_bytes(xml_envelope(
                "ListAttachedGroupPolicies",
                "<AttachedPolicies>\
                   <member>\
                     <PolicyArn>arn:aws:iam::aws:policy/ReadOnlyAccess</PolicyArn>\
                     <PolicyName>ReadOnlyAccess</PolicyName>\
                   </member>\
                 </AttachedPolicies>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .iam()
            .list_attached_group_policies("developers")
            .await
            .unwrap();

        assert_eq!(response.attached_policies.len(), 1);
        assert_eq!(
            response.attached_policies[0].policy_arn.as_deref(),
            Some("arn:aws:iam::aws:policy/ReadOnlyAccess")
        );
        assert_eq!(
            response.attached_policies[0].policy_name.as_deref(),
            Some("ReadOnlyAccess")
        );
    }

    #[tokio::test]
    async fn list_attached_group_policies_handles_empty() {
        let mut mock = MockClient::new();
        mock.expect_list_attached_group_policies()
            .returning_bytes(xml_envelope(
                "ListAttachedGroupPolicies",
                "<AttachedPolicies/>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .iam()
            .list_attached_group_policies("empty-group")
            .await
            .unwrap();
        assert!(response.attached_policies.is_empty());
    }

    // ── Virtual MFA Devices ────────────────────────────────────────────

    #[tokio::test]
    async fn list_virtual_mfa_devices_returns_devices() {
        let mut mock = MockClient::new();
        mock.expect_list_virtual_mfa_devices()
            .returning_bytes(xml_envelope(
                "ListVirtualMFADevices",
                "<VirtualMFADevices>\
                   <member>\
                     <SerialNumber>arn:aws:iam::123456789012:mfa/root-account-mfa-device</SerialNumber>\
                     <EnableDate>2024-01-01T00:00:00Z</EnableDate>\
                   </member>\
                 </VirtualMFADevices>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_virtual_mfa_devices(None).await.unwrap();

        assert_eq!(response.virtual_mfa_devices.len(), 1);
        assert_eq!(
            response.virtual_mfa_devices[0].serial_number,
            "arn:aws:iam::123456789012:mfa/root-account-mfa-device"
        );
        assert_eq!(
            response.virtual_mfa_devices[0].enable_date.as_deref(),
            Some("2024-01-01T00:00:00Z")
        );
    }

    #[tokio::test]
    async fn list_virtual_mfa_devices_handles_empty() {
        let mut mock = MockClient::new();
        mock.expect_list_virtual_mfa_devices()
            .returning_bytes(xml_envelope(
                "ListVirtualMFADevices",
                "<VirtualMFADevices/>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_virtual_mfa_devices(None).await.unwrap();
        assert!(response.virtual_mfa_devices.is_empty());
    }

    // ── Managed Policies ───────────────────────────────────────────────

    #[tokio::test]
    async fn list_policies_returns_local_policies() {
        let mut mock = MockClient::new();
        mock.expect_list_policies().returning_bytes(xml_envelope(
            "ListPolicies",
            "<Policies>\
               <member>\
                 <PolicyName>FullAdminPolicy</PolicyName>\
                 <PolicyId>ANPA000000000EXAMPLE</PolicyId>\
                 <Arn>arn:aws:iam::123456789012:policy/FullAdminPolicy</Arn>\
                 <DefaultVersionId>v1</DefaultVersionId>\
                 <IsAttachable>true</IsAttachable>\
                 <CreateDate>2024-01-15T10:00:00Z</CreateDate>\
                 <UpdateDate>2024-01-15T10:00:00Z</UpdateDate>\
               </member>\
             </Policies>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .iam()
            .list_policies(Some("Local"), None)
            .await
            .unwrap();

        assert_eq!(response.policies.len(), 1);
        assert_eq!(
            response.policies[0].policy_name.as_deref(),
            Some("FullAdminPolicy")
        );
        assert_eq!(
            response.policies[0].arn.as_deref(),
            Some("arn:aws:iam::123456789012:policy/FullAdminPolicy")
        );
        assert_eq!(
            response.policies[0].default_version_id.as_deref(),
            Some("v1")
        );
        assert_eq!(response.policies[0].is_attachable, Some(true));
    }

    #[tokio::test]
    async fn list_policies_handles_empty() {
        let mut mock = MockClient::new();
        mock.expect_list_policies()
            .returning_bytes(xml_envelope("ListPolicies", "<Policies/>"));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.iam().list_policies(None, None).await.unwrap();
        assert!(response.policies.is_empty());
    }

    #[tokio::test]
    async fn get_policy_version_returns_document() {
        let mut mock = MockClient::new();
        mock.expect_get_policy_version().returning_bytes(xml_envelope(
            "GetPolicyVersion",
            "<PolicyVersion>\
               <Document>%7B%22Version%22%3A%222012-10-17%22%2C%22Statement%22%3A%5B%7B%22Effect%22%3A%22Allow%22%2C%22Action%22%3A%22%2A%22%2C%22Resource%22%3A%22%2A%22%7D%5D%7D</Document>\
               <VersionId>v1</VersionId>\
               <IsDefaultVersion>true</IsDefaultVersion>\
               <CreateDate>2024-01-15T10:00:00Z</CreateDate>\
             </PolicyVersion>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .iam()
            .get_policy_version("arn:aws:iam::123456789012:policy/FullAdminPolicy", "v1")
            .await
            .unwrap();

        let version = response.policy_version.unwrap();
        assert_eq!(version.version_id.as_deref(), Some("v1"));
        assert_eq!(version.is_default_version, Some(true));
        // Document is URL-encoded JSON — just verify it's non-empty
        assert!(version.document.as_deref().unwrap_or("").contains("%7B"));
    }

    // ── Entities For Policy ────────────────────────────────────────────

    #[tokio::test]
    async fn list_entities_for_policy_returns_all_entity_types() {
        let mut mock = MockClient::new();
        mock.expect_list_entities_for_policy()
            .returning_bytes(xml_envelope(
                "ListEntitiesForPolicy",
                "<PolicyGroups>\
                   <member>\
                     <GroupName>SupportTeam</GroupName>\
                     <GroupId>AGPA000000000EXAMPLE</GroupId>\
                   </member>\
                 </PolicyGroups>\
                 <PolicyUsers>\
                   <member>\
                     <UserName>support-user</UserName>\
                     <UserId>AIDA000000000EXAMPLE</UserId>\
                   </member>\
                 </PolicyUsers>\
                 <PolicyRoles>\
                   <member>\
                     <RoleName>SupportRole</RoleName>\
                     <RoleId>AROA000000000EXAMPLE</RoleId>\
                   </member>\
                 </PolicyRoles>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .iam()
            .list_entities_for_policy("arn:aws:iam::aws:policy/AWSSupportAccess")
            .await
            .unwrap();

        assert_eq!(response.policy_groups.len(), 1);
        assert_eq!(
            response.policy_groups[0].group_name.as_deref(),
            Some("SupportTeam")
        );
        assert_eq!(response.policy_users.len(), 1);
        assert_eq!(
            response.policy_users[0].user_name.as_deref(),
            Some("support-user")
        );
        assert_eq!(response.policy_roles.len(), 1);
        assert_eq!(
            response.policy_roles[0].role_name.as_deref(),
            Some("SupportRole")
        );
    }

    #[tokio::test]
    async fn list_entities_for_policy_handles_empty() {
        let mut mock = MockClient::new();
        mock.expect_list_entities_for_policy()
            .returning_bytes(xml_envelope(
                "ListEntitiesForPolicy",
                "<PolicyGroups/><PolicyUsers/><PolicyRoles/>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .iam()
            .list_entities_for_policy("arn:aws:iam::aws:policy/AWSCloudShellFullAccess")
            .await
            .unwrap();
        assert!(response.policy_groups.is_empty());
        assert!(response.policy_users.is_empty());
        assert!(response.policy_roles.is_empty());
    }
}
