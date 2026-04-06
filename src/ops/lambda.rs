//! Operation contracts for the AWS Lambda API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/lambda.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::lambda::*;
use crate::{AwsHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the AWS Lambda API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::lambda::LambdaClient`] instead.
pub struct LambdaOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> LambdaOps<'a> {
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
        "https://lambda.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Returns a list of Lambda functions, with the version-specific configuration of each.
    ///
    /// **AWS API**: `GET /2015-03-31/functions`
    ///
    /// # Query Parameters
    /// - `MasterRegion` —
    /// - `FunctionVersion` —
    /// - `Marker` —
    /// - `MaxItems` —
    ///
    /// # Response
    /// [`ListFunctionsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_functions(
        &self,
        master_region: &str,
        function_version: &str,
        marker: &str,
        max_items: &str,
    ) -> Result<ListFunctionsResponse> {
        let url = format!("{}/2015-03-31/functions", self.base_url(),);
        let url = crate::append_query_params(
            url,
            &[
                ("MasterRegion", master_region),
                ("FunctionVersion", function_version),
                ("Marker", marker),
                ("MaxItems", max_items),
            ],
        );
        let response = self.client.get_json(&url, "lambda").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_functions response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_functions response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Returns the version-specific settings of a Lambda function or version.
    ///
    /// **AWS API**: `GET /2015-03-31/functions/{FunctionName}/configuration`
    ///
    /// # Path Parameters
    /// - `FunctionName` —  *(required)*
    ///
    /// # Query Parameters
    /// - `Qualifier` —
    ///
    /// # Response
    /// [`FunctionConfiguration`]
    #[allow(dead_code)]
    pub(crate) async fn get_function_configuration(
        &self,
        function_name: &str,
        qualifier: &str,
    ) -> Result<FunctionConfiguration> {
        let url = format!(
            "{}/2015-03-31/functions/{}/configuration",
            self.base_url(),
            encode(function_name),
        );
        let url = crate::append_query_params(url, &[("Qualifier", qualifier)]);
        let response = self.client.get_json(&url, "lambda").await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_function_configuration response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse get_function_configuration response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Modify the version-specific settings of a Lambda function.
    ///
    /// **AWS API**: `PUT /2015-03-31/functions/{FunctionName}/configuration`
    ///
    /// # Path Parameters
    /// - `FunctionName` —  *(required)*
    ///
    /// # Request Body
    /// [`UpdateFunctionConfigurationRequest`]
    ///
    /// # Response
    /// [`FunctionConfiguration`]
    #[allow(dead_code)]
    pub(crate) async fn update_function_configuration(
        &self,
        function_name: &str,
        body: &UpdateFunctionConfigurationRequest,
    ) -> Result<FunctionConfiguration> {
        let url = format!(
            "{}/2015-03-31/functions/{}/configuration",
            self.base_url(),
            encode(function_name),
        );
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize update_function_configuration request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .put(&url, "lambda", &body_bytes, "application/json")
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read update_function_configuration response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse update_function_configuration response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_functions() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/2015-03-31/functions?MasterRegion=test-MasterRegion&FunctionVersion=test-FunctionVersion&Marker=test-Marker&MaxItems=test-MaxItems")
            .returning_json(serde_json::to_value(ListFunctionsResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = LambdaOps::new(&client);

        let result = ops
            .list_functions(
                "test-MasterRegion",
                "test-FunctionVersion",
                "test-Marker",
                "test-MaxItems",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_function_configuration() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/2015-03-31/functions/test-FunctionName/configuration?Qualifier=test-Qualifier",
        )
        .returning_json(serde_json::to_value(FunctionConfiguration::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = LambdaOps::new(&client);

        let result = ops
            .get_function_configuration("test-FunctionName", "test-Qualifier")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_function_configuration() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/2015-03-31/functions/test-FunctionName/configuration")
            .returning_json(serde_json::to_value(FunctionConfiguration::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = LambdaOps::new(&client);

        let body = UpdateFunctionConfigurationRequest::fixture();
        let result = ops
            .update_function_configuration("test-FunctionName", &body)
            .await;
        assert!(result.is_ok());
    }
}
