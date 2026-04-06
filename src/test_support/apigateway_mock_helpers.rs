//! MockClient helpers for Amazon API Gateway API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon API Gateway helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait ApigatewayMockHelpers {
    /// Helper to expect `get_rest_apis`: Lists the RestApis resources for your collection.
    fn expect_get_rest_apis(&mut self, position: &str, limit: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_stages`: Gets information about one or more Stage resources.
    fn expect_get_stages(
        &mut self,
        restapi_id: &str,
        deployment_id: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_stage`: Changes information about a Stage resource.
    fn expect_update_stage(&mut self, restapi_id: &str, stage_name: &str)
    -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl ApigatewayMockHelpers for MockClient {
    /// Helper to expect `get_rest_apis`: Lists the RestApis resources for your collection.
    fn expect_get_rest_apis(
        &mut self,
        position: &str,
        limit: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = "/restapis".to_string();
        let mut __qp: Vec<String> = Vec::new();
        if !position.is_empty() {
            __qp.push(format!("position={}", position));
        }
        if !limit.is_empty() {
            __qp.push(format!("limit={}", limit));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `get_stages`: Gets information about one or more Stage resources.
    fn expect_get_stages(
        &mut self,
        restapi_id: &str,
        deployment_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/restapis/{restapi_id}/stages");
        let mut __qp: Vec<String> = Vec::new();
        if !deployment_id.is_empty() {
            __qp.push(format!("deploymentId={}", deployment_id));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `update_stage`: Changes information about a Stage resource.
    fn expect_update_stage(
        &mut self,
        restapi_id: &str,
        stage_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/restapis/{restapi_id}/stages/{stage_name}");
        self.expect_patch(&path)
    }
}
