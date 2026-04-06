//! MockClient helpers for AWS Organizations API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS Organizations helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait OrganizationsMockHelpers {
    /// Helper to expect `list_organizational_units_for_parent`: Lists the organizational units in a
    /// specified parent OU or root.
    fn expect_list_organizational_units_for_parent(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_accounts_for_parent`: Lists the accounts in an organization that are
    /// contained by the specified parent OU or root.
    fn expect_list_accounts_for_parent(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl OrganizationsMockHelpers for MockClient {
    /// Helper to expect `list_organizational_units_for_parent`: Lists the organizational units in a
    /// specified parent OU or root.
    fn expect_list_organizational_units_for_parent(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `list_accounts_for_parent`: Lists the accounts in an organization that are
    /// contained by the specified parent OU or root.
    fn expect_list_accounts_for_parent(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
