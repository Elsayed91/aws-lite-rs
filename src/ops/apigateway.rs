//! Operation contracts for the Amazon API Gateway API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/apigateway.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::apigateway::*;
use crate::{AwsHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Amazon API Gateway API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::apigateway::ApigatewayClient`] instead.
pub struct ApigatewayOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> ApigatewayOps<'a> {
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
        "https://apigateway.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Lists the RestApis resources for your collection.
    ///
    /// **AWS API**: `GET /restapis`
    ///
    /// # Query Parameters
    /// - `position` —
    /// - `limit` —
    ///
    /// # Response
    /// [`RestApis`]
    #[allow(dead_code)]
    pub(crate) async fn get_rest_apis(&self, position: &str, limit: &str) -> Result<RestApis> {
        let url = format!("{}/restapis", self.base_url(),);
        let url = crate::append_query_params(url, &[("position", position), ("limit", limit)]);
        let response = self.client.get_json(&url, "apigateway").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_rest_apis response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse get_rest_apis response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets information about one or more Stage resources.
    ///
    /// **AWS API**: `GET /restapis/{restapi_id}/stages`
    ///
    /// # Path Parameters
    /// - `restapi_id` —  *(required)*
    ///
    /// # Query Parameters
    /// - `deploymentId` —
    ///
    /// # Response
    /// [`Stages`]
    #[allow(dead_code)]
    pub(crate) async fn get_stages(&self, restapi_id: &str, deployment_id: &str) -> Result<Stages> {
        let url = format!("{}/restapis/{}/stages", self.base_url(), encode(restapi_id),);
        let url = crate::append_query_params(url, &[("deploymentId", deployment_id)]);
        let response = self.client.get_json(&url, "apigateway").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_stages response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse get_stages response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Changes information about a Stage resource.
    ///
    /// **AWS API**: `PATCH /restapis/{restapi_id}/stages/{stage_name}`
    ///
    /// # Path Parameters
    /// - `restapi_id` —  *(required)*
    /// - `stage_name` —  *(required)*
    ///
    /// # Request Body
    /// [`UpdateStageRequest`]
    ///
    /// # Response
    /// [`Stage`]
    #[allow(dead_code)]
    pub(crate) async fn update_stage(
        &self,
        restapi_id: &str,
        stage_name: &str,
        body: &UpdateStageRequest,
    ) -> Result<Stage> {
        let url = format!(
            "{}/restapis/{}/stages/{}",
            self.base_url(),
            encode(restapi_id),
            encode(stage_name),
        );
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize update_stage request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .patch(&url, "apigateway", &body_bytes, "application/json")
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read update_stage response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse update_stage response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_rest_apis() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/restapis?position=test-position&limit=test-limit")
            .returning_json(serde_json::to_value(RestApis::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ApigatewayOps::new(&client);

        let result = ops.get_rest_apis("test-position", "test-limit").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_stages() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/restapis/test-restapi_id/stages?deploymentId=test-deploymentId")
            .returning_json(serde_json::to_value(Stages::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ApigatewayOps::new(&client);

        let result = ops.get_stages("test-restapi_id", "test-deploymentId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_stage() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/restapis/test-restapi_id/stages/test-stage_name")
            .returning_json(serde_json::to_value(Stage::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = ApigatewayOps::new(&client);

        let body = UpdateStageRequest::fixture();
        let result = ops
            .update_stage("test-restapi_id", "test-stage_name", &body)
            .await;
        assert!(result.is_ok());
    }
}
