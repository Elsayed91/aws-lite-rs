//! Amazon DynamoDB API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::dynamodb::DynamodbOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::dynamodb::DynamodbOps,
    types::dynamodb::{
        DeleteTableInput, DeleteTableOutput, DescribeTableInput, DescribeTableOutput,
        ListTablesInput, ListTablesOutput, UpdateTableInput, UpdateTableOutput,
    },
};

/// Client for the Amazon DynamoDB API
pub struct DynamodbClient<'a> {
    ops: DynamodbOps<'a>,
}

impl<'a> DynamodbClient<'a> {
    /// Create a new Amazon DynamoDB API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: DynamodbOps::new(client),
        }
    }

    /// Returns an array of table names associated with the current account and endpoint.
    pub async fn list_tables(&self, body: &ListTablesInput) -> Result<ListTablesOutput> {
        self.ops.list_tables(body).await
    }

    /// Returns information about the table, including the current status of the table, when it was created, the primary key sch
    pub async fn describe_table(&self, body: &DescribeTableInput) -> Result<DescribeTableOutput> {
        self.ops.describe_table(body).await
    }

    /// Modifies the provisioned throughput settings, global secondary indexes, or DynamoDB Streams settings for a given table.
    pub async fn update_table(&self, body: &UpdateTableInput) -> Result<UpdateTableOutput> {
        self.ops.update_table(body).await
    }

    /// The DeleteTable operation deletes a table and all of its items.
    pub async fn delete_table(&self, body: &DeleteTableInput) -> Result<DeleteTableOutput> {
        self.ops.delete_table(body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::dynamodb::ProvisionedThroughput;

    #[tokio::test]
    async fn list_tables_returns_names() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "TableNames": ["orders", "users", "products"],
            "LastEvaluatedTableName": "products"
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .dynamodb()
            .list_tables(&ListTablesInput::default())
            .await
            .unwrap();
        assert_eq!(result.table_names, vec!["orders", "users", "products"]);
        assert_eq!(
            result.last_evaluated_table_name.as_deref(),
            Some("products")
        );
    }

    #[tokio::test]
    async fn describe_table_returns_details() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "Table": {
                "TableName": "orders",
                "TableArn": "arn:aws:dynamodb:us-east-1:123456789012:table/orders",
                "TableId": "abc-123",
                "TableStatus": "ACTIVE",
                "CreationDateTime": 1700000000.0,
                "ItemCount": 42,
                "TableSizeBytes": 8192,
                "BillingModeSummary": {
                    "BillingMode": "PAY_PER_REQUEST"
                },
                "KeySchema": [{
                    "AttributeName": "pk",
                    "KeyType": "HASH"
                }],
                "AttributeDefinitions": [{
                    "AttributeName": "pk",
                    "AttributeType": "S"
                }]
            }
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .dynamodb()
            .describe_table(&DescribeTableInput {
                table_name: "orders".to_string(),
            })
            .await
            .unwrap();
        let table = result.table.unwrap();
        assert_eq!(table.table_name.as_deref(), Some("orders"));
        assert_eq!(table.table_status.as_deref(), Some("ACTIVE"));
        assert_eq!(table.item_count, Some(42));
        assert_eq!(table.key_schema[0].attribute_name, "pk");
        assert_eq!(table.key_schema[0].key_type, "HASH");
        assert_eq!(
            table.billing_mode_summary.unwrap().billing_mode.as_deref(),
            Some("PAY_PER_REQUEST"),
        );
    }

    #[tokio::test]
    async fn update_table_changes_billing_mode() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "TableDescription": {
                "TableName": "orders",
                "TableStatus": "UPDATING",
                "ProvisionedThroughput": {
                    "ReadCapacityUnits": 10,
                    "WriteCapacityUnits": 5
                }
            }
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .dynamodb()
            .update_table(&UpdateTableInput {
                table_name: "orders".to_string(),
                billing_mode: Some("PROVISIONED".to_string()),
                provisioned_throughput: Some(ProvisionedThroughput {
                    read_capacity_units: 10,
                    write_capacity_units: 5,
                }),
                ..Default::default()
            })
            .await
            .unwrap();
        let desc = result.table_description.unwrap();
        assert_eq!(desc.table_name.as_deref(), Some("orders"));
        assert_eq!(desc.table_status.as_deref(), Some("UPDATING"));
        let tp = desc.provisioned_throughput.unwrap();
        assert_eq!(tp.read_capacity_units, Some(10));
        assert_eq!(tp.write_capacity_units, Some(5));
    }

    #[tokio::test]
    async fn delete_table_returns_deleting_status() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "TableDescription": {
                "TableName": "orders",
                "TableStatus": "DELETING",
                "TableArn": "arn:aws:dynamodb:us-east-1:123456789012:table/orders"
            }
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .dynamodb()
            .delete_table(&DeleteTableInput {
                table_name: "orders".to_string(),
            })
            .await
            .unwrap();
        let desc = result.table_description.unwrap();
        assert_eq!(desc.table_name.as_deref(), Some("orders"));
        assert_eq!(desc.table_status.as_deref(), Some("DELETING"));
    }
}
