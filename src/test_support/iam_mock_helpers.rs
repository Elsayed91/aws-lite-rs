//! MockClient helpers for AWS Identity and Access Management API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS Identity and Access Management helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait IamMockHelpers {
    /// Helper to expect `list_users`: Lists all IAM users in the account.
    fn expect_list_users(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_attached_user_policies`: Lists all managed policies attached to the
    /// specified IAM user.
    fn expect_list_attached_user_policies(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `detach_user_policy`: Removes the specified managed policy from the
    /// specified IAM user.
    fn expect_detach_user_policy(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_access_key`: Deletes the access key pair associated with the
    /// specified IAM user.
    fn expect_delete_access_key(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_access_keys`: Lists the access key IDs associated with the specified
    /// IAM user.
    fn expect_list_access_keys(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_access_key_last_used`: Retrieves information about when the specified
    /// access key was last used.
    fn expect_get_access_key_last_used(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `generate_credential_report`: Generates a credential report for the AWS
    /// account.
    fn expect_generate_credential_report(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_credential_report`: Retrieves a credential report for the AWS account.
    fn expect_get_credential_report(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_access_key`: Changes the status of the specified access key from
    /// Active to Inactive, or vice versa.
    fn expect_update_access_key(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_mfa_devices`: Lists the MFA devices for an IAM user.
    fn expect_list_mfa_devices(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_login_profile`: Retrieves the login profile for the specified IAM
    /// user.
    fn expect_get_login_profile(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_account_summary`: Retrieves information about IAM entity usage and IAM
    /// quotas in the AWS account.
    fn expect_get_account_summary(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_account_password_policy`: Retrieves the password policy for the AWS
    /// account.
    fn expect_get_account_password_policy(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_account_password_policy`: Updates the password policy settings for
    /// the AWS account.
    fn expect_update_account_password_policy(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_roles`: Lists the IAM roles that have the specified path prefix.
    fn expect_list_roles(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_user_policies`: Lists the names of the inline policies embedded in
    /// the specified IAM user.
    fn expect_list_user_policies(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_groups_for_user`: Lists the IAM groups that the specified IAM user
    /// belongs to.
    fn expect_list_groups_for_user(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_server_certificates`: Lists the server certificates stored in IAM.
    fn expect_list_server_certificates(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_user_policy`: Deletes the specified inline policy from the
    /// specified IAM user.
    fn expect_delete_user_policy(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `attach_role_policy`: Attaches the specified managed policy to the
    /// specified IAM role.
    fn expect_attach_role_policy(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `detach_role_policy`: Removes the specified managed policy from the
    /// specified IAM role.
    fn expect_detach_role_policy(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_service_linked_role`: Creates an IAM role that is linked to a
    /// specific AWS service.
    fn expect_create_service_linked_role(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_user_policy`: Retrieves the specified inline policy document embedded
    /// in the specified IAM user.
    fn expect_get_user_policy(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_attached_group_policies`: Lists all managed policies that are
    /// attached to the specified IAM group.
    fn expect_list_attached_group_policies(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_virtual_mfa_devices`: Lists the virtual MFA devices defined in the
    /// AWS account.
    fn expect_list_virtual_mfa_devices(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_policies`: Lists IAM policies, optionally filtered by scope
    /// (Local/AWS/All).
    fn expect_list_policies(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_policy_version`: Retrieves information about the specified version of
    /// the specified managed policy.
    fn expect_get_policy_version(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_entities_for_policy`: Lists all IAM users, groups, and roles that the
    /// specified managed policy is attached to.
    fn expect_list_entities_for_policy(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl IamMockHelpers for MockClient {
    /// Helper to expect `list_users`: Lists all IAM users in the account.
    fn expect_list_users(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_attached_user_policies`: Lists all managed policies attached to the
    /// specified IAM user.
    fn expect_list_attached_user_policies(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `detach_user_policy`: Removes the specified managed policy from the
    /// specified IAM user.
    fn expect_detach_user_policy(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_access_key`: Deletes the access key pair associated with the
    /// specified IAM user.
    fn expect_delete_access_key(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_access_keys`: Lists the access key IDs associated with the specified
    /// IAM user.
    fn expect_list_access_keys(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_access_key_last_used`: Retrieves information about when the specified
    /// access key was last used.
    fn expect_get_access_key_last_used(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `generate_credential_report`: Generates a credential report for the AWS
    /// account.
    fn expect_generate_credential_report(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_credential_report`: Retrieves a credential report for the AWS account.
    fn expect_get_credential_report(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `update_access_key`: Changes the status of the specified access key from
    /// Active to Inactive, or vice versa.
    fn expect_update_access_key(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_mfa_devices`: Lists the MFA devices for an IAM user.
    fn expect_list_mfa_devices(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_login_profile`: Retrieves the login profile for the specified IAM
    /// user.
    fn expect_get_login_profile(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_account_summary`: Retrieves information about IAM entity usage and IAM
    /// quotas in the AWS account.
    fn expect_get_account_summary(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_account_password_policy`: Retrieves the password policy for the AWS
    /// account.
    fn expect_get_account_password_policy(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `update_account_password_policy`: Updates the password policy settings for
    /// the AWS account.
    fn expect_update_account_password_policy(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_roles`: Lists the IAM roles that have the specified path prefix.
    fn expect_list_roles(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_user_policies`: Lists the names of the inline policies embedded in
    /// the specified IAM user.
    fn expect_list_user_policies(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_groups_for_user`: Lists the IAM groups that the specified IAM user
    /// belongs to.
    fn expect_list_groups_for_user(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_server_certificates`: Lists the server certificates stored in IAM.
    fn expect_list_server_certificates(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_user_policy`: Deletes the specified inline policy from the
    /// specified IAM user.
    fn expect_delete_user_policy(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `attach_role_policy`: Attaches the specified managed policy to the
    /// specified IAM role.
    fn expect_attach_role_policy(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `detach_role_policy`: Removes the specified managed policy from the
    /// specified IAM role.
    fn expect_detach_role_policy(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `create_service_linked_role`: Creates an IAM role that is linked to a
    /// specific AWS service.
    fn expect_create_service_linked_role(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_user_policy`: Retrieves the specified inline policy document embedded
    /// in the specified IAM user.
    fn expect_get_user_policy(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_attached_group_policies`: Lists all managed policies that are
    /// attached to the specified IAM group.
    fn expect_list_attached_group_policies(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_virtual_mfa_devices`: Lists the virtual MFA devices defined in the
    /// AWS account.
    fn expect_list_virtual_mfa_devices(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_policies`: Lists IAM policies, optionally filtered by scope
    /// (Local/AWS/All).
    fn expect_list_policies(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_policy_version`: Retrieves information about the specified version of
    /// the specified managed policy.
    fn expect_get_policy_version(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_entities_for_policy`: Lists all IAM users, groups, and roles that the
    /// specified managed policy is attached to.
    fn expect_list_entities_for_policy(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
