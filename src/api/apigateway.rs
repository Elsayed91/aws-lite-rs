//! Amazon API Gateway API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::apigateway::ApigatewayOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::apigateway::ApigatewayOps,
    types::apigateway::{RestApis, Stage, Stages, UpdateStageRequest},
};

/// Client for the Amazon API Gateway API
pub struct ApigatewayClient<'a> {
    ops: ApigatewayOps<'a>,
}

impl<'a> ApigatewayClient<'a> {
    /// Create a new Amazon API Gateway API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: ApigatewayOps::new(client),
        }
    }

    /// Lists the RestApis resources for your collection.
    pub async fn get_rest_apis(&self, position: &str, limit: &str) -> Result<RestApis> {
        self.ops.get_rest_apis(position, limit).await
    }

    /// Gets information about one or more Stage resources.
    pub async fn get_stages(&self, restapi_id: &str, deployment_id: &str) -> Result<Stages> {
        self.ops.get_stages(restapi_id, deployment_id).await
    }

    /// Changes information about a Stage resource.
    pub async fn update_stage(
        &self,
        restapi_id: &str,
        stage_name: &str,
        body: &UpdateStageRequest,
    ) -> Result<Stage> {
        self.ops.update_stage(restapi_id, stage_name, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;
    use crate::types::apigateway::*;

    // RestApis response uses "item" key (locationName) not "items" (memberName)
    // This was verified via integration testing against the real API.

    #[tokio::test]
    async fn test_get_rest_apis_deserializes_item_key() {
        let mut mock = MockClient::new();
        mock.expect_get("/restapis")
            .returning_json(serde_json::json!({
                "item": [
                    {"id": "abc123", "name": "my-api", "description": "A test API"}
                ]
            }));

        let client = AwsHttpClient::from_mock(mock);
        let result = client.apigateway().get_rest_apis("", "").await.unwrap();

        assert_eq!(result.items.len(), 1);
        let api = &result.items[0];
        assert_eq!(api.id.as_deref(), Some("abc123"));
        assert_eq!(api.name.as_deref(), Some("my-api"));
        assert_eq!(api.description.as_deref(), Some("A test API"));
    }

    #[tokio::test]
    async fn test_get_rest_apis_returns_empty_when_no_apis() {
        let mut mock = MockClient::new();
        mock.expect_get("/restapis")
            .returning_json(serde_json::json!({}));

        let client = AwsHttpClient::from_mock(mock);
        let result = client.apigateway().get_rest_apis("", "").await.unwrap();

        assert_eq!(result.items.len(), 0);
        assert!(result.position.is_none());
    }

    #[tokio::test]
    async fn test_get_stages_returns_stage_fields() {
        let mut mock = MockClient::new();
        mock.expect_get("/restapis/api-id-123/stages")
            .returning_json(serde_json::json!({
                "item": [
                    {
                        "stageName": "prod",
                        "deploymentId": "deploy-abc",
                        "cacheClusterEnabled": false,
                        "tracingEnabled": true
                    }
                ]
            }));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .apigateway()
            .get_stages("api-id-123", "")
            .await
            .unwrap();

        assert_eq!(result.item.len(), 1);
        let stage = &result.item[0];
        assert_eq!(stage.stage_name.as_deref(), Some("prod"));
        assert_eq!(stage.deployment_id.as_deref(), Some("deploy-abc"));
        assert_eq!(stage.cache_cluster_enabled, Some(false));
        assert_eq!(stage.tracing_enabled, Some(true));
    }

    #[tokio::test]
    async fn test_update_stage_returns_updated_fields() {
        let mut mock = MockClient::new();
        mock.expect_patch("/restapis/api-id-123/stages/prod")
            .returning_json(serde_json::json!({
                "stageName": "prod",
                "deploymentId": "deploy-abc",
                "cacheClusterEnabled": false,
                "tracingEnabled": true
            }));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .apigateway()
            .update_stage(
                "api-id-123",
                "prod",
                &UpdateStageRequest {
                    rest_api_id: "api-id-123".to_string(),
                    stage_name: "prod".to_string(),
                    patch_operations: vec![PatchOperation {
                        op: Some("replace".to_string()),
                        path: Some("/tracingEnabled".to_string()),
                        value: Some("true".to_string()),
                        ..Default::default()
                    }],
                },
            )
            .await
            .unwrap();

        assert_eq!(result.stage_name.as_deref(), Some("prod"));
        assert_eq!(result.tracing_enabled, Some(true));
        assert_eq!(result.cache_cluster_enabled, Some(false));
    }
}
