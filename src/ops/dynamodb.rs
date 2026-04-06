//! Operation contracts for the Amazon DynamoDB API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/dynamodb.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::dynamodb::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon DynamoDB API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::dynamodb::DynamodbClient`] instead.
pub struct DynamodbOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> DynamodbOps<'a> {
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
        "https://dynamodb.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Returns an array of table names associated with the current account and endpoint.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ListTablesInput`]
    ///
    /// # Response
    /// [`ListTablesOutput`]
    #[allow(dead_code)]
    pub(crate) async fn list_tables(&self, body: &ListTablesInput) -> Result<ListTablesOutput> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize list_tables request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "dynamodb",
                "DynamoDB_20120810.ListTables",
                "1.0",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read list_tables response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse list_tables response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Returns information about the table, including the current status of the table, when it
    /// was created, the primary key schema, and any indexes on the table.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeTableInput`]
    ///
    /// # Response
    /// [`DescribeTableOutput`]
    #[allow(dead_code)]
    pub(crate) async fn describe_table(
        &self,
        body: &DescribeTableInput,
    ) -> Result<DescribeTableOutput> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_table request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "dynamodb",
                "DynamoDB_20120810.DescribeTable",
                "1.0",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_table response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_table response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Modifies the provisioned throughput settings, global secondary indexes, or DynamoDB
    /// Streams settings for a given table.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`UpdateTableInput`]
    ///
    /// # Response
    /// [`UpdateTableOutput`]
    #[allow(dead_code)]
    pub(crate) async fn update_table(&self, body: &UpdateTableInput) -> Result<UpdateTableOutput> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize update_table request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "dynamodb",
                "DynamoDB_20120810.UpdateTable",
                "1.0",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read update_table response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse update_table response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// The DeleteTable operation deletes a table and all of its items.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteTableInput`]
    ///
    /// # Response
    /// [`DeleteTableOutput`]
    #[allow(dead_code)]
    pub(crate) async fn delete_table(&self, body: &DeleteTableInput) -> Result<DeleteTableOutput> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize delete_table request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "dynamodb",
                "DynamoDB_20120810.DeleteTable",
                "1.0",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read delete_table response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse delete_table response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_tables() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(ListTablesOutput::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = DynamodbOps::new(&client);

        let body = ListTablesInput::fixture();
        let result = ops.list_tables(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeTableOutput::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = DynamodbOps::new(&client);

        let body = DescribeTableInput::fixture();
        let result = ops.describe_table(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(UpdateTableOutput::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = DynamodbOps::new(&client);

        let body = UpdateTableInput::fixture();
        let result = ops.update_table(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DeleteTableOutput::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = DynamodbOps::new(&client);

        let body = DeleteTableInput::fixture();
        let result = ops.delete_table(&body).await;
        assert!(result.is_ok());
    }
}
