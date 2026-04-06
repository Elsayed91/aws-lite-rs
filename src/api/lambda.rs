//! AWS Lambda API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::lambda::LambdaOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::lambda::LambdaOps,
    types::lambda::{
        FunctionConfiguration, ListFunctionsResponse, UpdateFunctionConfigurationRequest,
    },
};

/// Client for the AWS Lambda API
pub struct LambdaClient<'a> {
    ops: LambdaOps<'a>,
}

impl<'a> LambdaClient<'a> {
    /// Create a new AWS Lambda API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: LambdaOps::new(client),
        }
    }

    /// Returns a list of Lambda functions, with the version-specific configuration of each.
    pub async fn list_functions(
        &self,
        master_region: &str,
        function_version: &str,
        marker: &str,
        max_items: &str,
    ) -> Result<ListFunctionsResponse> {
        self.ops
            .list_functions(master_region, function_version, marker, max_items)
            .await
    }

    /// Returns the version-specific settings of a Lambda function or version.
    pub async fn get_function_configuration(
        &self,
        function_name: &str,
        qualifier: &str,
    ) -> Result<FunctionConfiguration> {
        self.ops
            .get_function_configuration(function_name, qualifier)
            .await
    }

    /// Modify the version-specific settings of a Lambda function.
    pub async fn update_function_configuration(
        &self,
        function_name: &str,
        body: &UpdateFunctionConfigurationRequest,
    ) -> Result<FunctionConfiguration> {
        self.ops
            .update_function_configuration(function_name, body)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;
    use crate::types::lambda::*;

    fn make_function_config(
        name: &str,
        arn: &str,
        runtime: &str,
        timeout: i32,
        memory: i32,
    ) -> FunctionConfiguration {
        FunctionConfiguration {
            function_name: name.to_string(),
            function_arn: arn.to_string(),
            runtime: Some(runtime.to_string()),
            timeout: Some(timeout),
            memory_size: Some(memory),
            architectures: vec!["x86_64".to_string()],
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_list_functions_returns_empty() {
        let mut mock = MockClient::new();
        mock.expect_get("/2015-03-31/functions")
            .returning_json(serde_json::json!({
                "Functions": [],
                "NextMarker": null
            }));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .lambda()
            .list_functions("", "", "", "")
            .await
            .unwrap();

        assert_eq!(result.functions.len(), 0);
        assert!(result.next_marker.is_none());
    }

    #[tokio::test]
    async fn test_list_functions_returns_functions_with_correct_fields() {
        let mut mock = MockClient::new();
        mock.expect_get("/2015-03-31/functions")
            .returning_json(serde_json::json!({
                "Functions": [
                    {
                        "FunctionName": "my-function",
                        "FunctionArn": "arn:aws:lambda:eu-central-1:123456789012:function:my-function",
                        "Runtime": "python3.12",
                        "Timeout": 30,
                        "MemorySize": 256,
                        "Architectures": ["arm64"]
                    }
                ]
            }));

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .lambda()
            .list_functions("", "", "", "")
            .await
            .unwrap();

        assert_eq!(result.functions.len(), 1);
        let func = &result.functions[0];
        assert_eq!(func.function_name, "my-function");
        assert_eq!(
            func.function_arn,
            "arn:aws:lambda:eu-central-1:123456789012:function:my-function"
        );
        assert_eq!(func.runtime.as_deref(), Some("python3.12"));
        assert_eq!(func.timeout, Some(30));
        assert_eq!(func.memory_size, Some(256));
        assert_eq!(func.architectures, vec!["arm64"]);
    }

    #[tokio::test]
    async fn test_get_function_configuration_returns_config() {
        let mut mock = MockClient::new();
        mock.expect_get("/2015-03-31/functions/my-function/configuration")
            .returning_json(
                serde_json::to_value(make_function_config(
                    "my-function",
                    "arn:aws:lambda:eu-central-1:123456789012:function:my-function",
                    "python3.12",
                    30,
                    256,
                ))
                .unwrap(),
            );

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .lambda()
            .get_function_configuration("my-function", "")
            .await
            .unwrap();

        assert_eq!(result.function_name, "my-function");
        assert_eq!(result.timeout, Some(30));
        assert_eq!(result.memory_size, Some(256));
        assert_eq!(result.architectures, vec!["x86_64"]);
    }

    #[tokio::test]
    async fn test_update_function_configuration_returns_updated_config() {
        let mut mock = MockClient::new();
        mock.expect_put("/2015-03-31/functions/my-function/configuration")
            .returning_json(
                serde_json::to_value(make_function_config(
                    "my-function",
                    "arn:aws:lambda:eu-central-1:123456789012:function:my-function",
                    "python3.12",
                    60,
                    512,
                ))
                .unwrap(),
            );

        let client = AwsHttpClient::from_mock(mock);
        let result = client
            .lambda()
            .update_function_configuration(
                "my-function",
                &UpdateFunctionConfigurationRequest {
                    function_name: "my-function".to_string(),
                    timeout: Some(60),
                    memory_size: Some(512),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        assert_eq!(result.function_name, "my-function");
        assert_eq!(result.timeout, Some(60));
        assert_eq!(result.memory_size, Some(512));
    }
}
