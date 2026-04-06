//! MockClient helpers for AWS Lambda API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS Lambda helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait LambdaMockHelpers {
    /// Helper to expect `list_functions`: Returns a list of Lambda functions, with the version-
    /// specific configuration of each.
    fn expect_list_functions(
        &mut self,
        master_region: &str,
        function_version: &str,
        marker: &str,
        max_items: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_function_configuration`: Returns the version-specific settings of a
    /// Lambda function or version.
    fn expect_get_function_configuration(
        &mut self,
        function_name: &str,
        qualifier: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_function_configuration`: Modify the version-specific settings of a
    /// Lambda function.
    fn expect_update_function_configuration(
        &mut self,
        function_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl LambdaMockHelpers for MockClient {
    /// Helper to expect `list_functions`: Returns a list of Lambda functions, with the version-
    /// specific configuration of each.
    fn expect_list_functions(
        &mut self,
        master_region: &str,
        function_version: &str,
        marker: &str,
        max_items: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = "/2015-03-31/functions".to_string();
        let mut __qp: Vec<String> = Vec::new();
        if !master_region.is_empty() {
            __qp.push(format!("MasterRegion={}", master_region));
        }
        if !function_version.is_empty() {
            __qp.push(format!("FunctionVersion={}", function_version));
        }
        if !marker.is_empty() {
            __qp.push(format!("Marker={}", marker));
        }
        if !max_items.is_empty() {
            __qp.push(format!("MaxItems={}", max_items));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `get_function_configuration`: Returns the version-specific settings of a
    /// Lambda function or version.
    fn expect_get_function_configuration(
        &mut self,
        function_name: &str,
        qualifier: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/2015-03-31/functions/{function_name}/configuration");
        let mut __qp: Vec<String> = Vec::new();
        if !qualifier.is_empty() {
            __qp.push(format!("Qualifier={}", qualifier));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `update_function_configuration`: Modify the version-specific settings of a
    /// Lambda function.
    fn expect_update_function_configuration(
        &mut self,
        function_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/2015-03-31/functions/{function_name}/configuration");
        self.expect_put(&path)
    }
}
