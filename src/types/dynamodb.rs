//! Types for the Amazon DynamoDB API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Represents the input of a ListTables operation.
///
/// **AWS API**: `dynamodb.v1.ListTablesInput`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//ListTablesInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTablesInput {
    /// The first table name that this operation will evaluate. Use the value that was returned
    /// for LastEvaluatedTableName in a previous operation, so that you can obtain the next page
    /// of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_table_name: Option<String>,

    /// A maximum number of table names to return. If this parameter is not specified, the limit
    /// is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl ListTablesInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            exclusive_start_table_name: Some("test-exclusive_start_table_name".into()),
            limit: Some(100),
        }
    }
}

/// Represents the output of a ListTables operation.
///
/// **AWS API**: `dynamodb.v1.ListTablesOutput`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//ListTablesOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTablesOutput {
    /// The names of the tables associated with the current account at the current endpoint. The
    /// maximum size of this array is 100. If LastEvaluatedTableName also appears in the output,
    /// you can use this value as the ExclusiveStartTableName parameter in a subsequent
    /// ListTables request and obtain the next page of results.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub table_names: Vec<String>,

    /// The name of the last table in the current page of results. Use this value as the
    /// ExclusiveStartTableName in a new request to obtain the next page of results, until all
    /// the table names are returned. If you do not receive a LastEvaluatedTableName value in
    /// the response, this means that there are no more table names to be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_table_name: Option<String>,
}

impl ListTablesOutput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            table_names: vec![],
            last_evaluated_table_name: Some("test-last_evaluated_table_name".into()),
        }
    }
}

/// Represents the input of a DescribeTable operation.
///
/// **AWS API**: `dynamodb.v1.DescribeTableInput`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//DescribeTableInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeTableInput {
    /// The name of the table to describe. You can also provide the Amazon Resource Name (ARN)
    /// of the table in this parameter.
    pub table_name: String,
}

impl DescribeTableInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            table_name: "test-table_name".into(),
        }
    }
}

/// Represents the output of a DescribeTable operation.
///
/// **AWS API**: `dynamodb.v1.DescribeTableOutput`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//DescribeTableOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeTableOutput {
    /// The properties of the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableDescription>,
}

impl DescribeTableOutput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            table: Some(TableDescription::fixture()),
        }
    }
}

/// Represents the input of an UpdateTable operation.
///
/// **AWS API**: `dynamodb.v1.UpdateTableInput`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//UpdateTableInput>
///
/// ## Coverage
/// 6 of 14 fields included.
/// Omitted fields:
/// - `AttributeDefinitions` — not selected in manifest
/// - `GlobalSecondaryIndexUpdates` — not selected in manifest
/// - `SSESpecification` — not selected in manifest
/// - `ReplicaUpdates` — not selected in manifest
/// - `MultiRegionConsistency` — not selected in manifest
/// - `GlobalTableWitnessUpdates` — not selected in manifest
/// - `OnDemandThroughput` — not selected in manifest
/// - `WarmThroughput` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTableInput {
    /// The name of the table to be updated. You can also provide the Amazon Resource Name (ARN)
    /// of the table in this parameter.
    pub table_name: String,

    /// Controls how you are charged for read and write throughput and how you manage capacity.
    /// When switching from pay-per-request to provisioned capacity, initial provisioned
    /// capacity values must be set. The initial provisioned capacity values are estimated based
    /// on the consumed read and write capacity of your table and global secondary indexes over
    /// the past 30 minutes. PAY_PER_REQUEST
    /// - We recommend using PAY_PER_REQUEST for most DynamoDB workloads. PAY_PER_REQUEST sets
    ///   the billing mode to On-demand capacity mode. PROVISIONED
    /// - We recommend using PROVISIONED for steady workloads with predictable growth where
    ///   capacity requirements can be reliably forecasted. PROVISIONED sets the billing mode to
    ///   Provisioned capacity mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,

    /// The new provisioned throughput settings for the specified table or index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,

    /// Indicates whether deletion protection is to be enabled (true) or disabled (false) on the
    /// table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,

    /// The table class of the table to be updated. Valid values are STANDARD and
    /// STANDARD_INFREQUENT_ACCESS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_class: Option<String>,

    /// Represents the DynamoDB Streams configuration for the table. You receive a
    /// ValidationException if you try to enable a stream on a table that already has a stream,
    /// or if you try to disable a stream on a table that doesn't have a stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,
}

impl UpdateTableInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            table_name: "test-table_name".into(),
            billing_mode: Some("test-billing_mode".into()),
            provisioned_throughput: Some(ProvisionedThroughput::fixture()),
            deletion_protection_enabled: Some(false),
            table_class: Some("test-table_class".into()),
            stream_specification: Some(StreamSpecification::fixture()),
        }
    }
}

/// Represents the output of an UpdateTable operation.
///
/// **AWS API**: `dynamodb.v1.UpdateTableOutput`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//UpdateTableOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTableOutput {
    /// Represents the properties of the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

impl UpdateTableOutput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            table_description: Some(TableDescription::fixture()),
        }
    }
}

/// Represents the input of a DeleteTable operation.
///
/// **AWS API**: `dynamodb.v1.DeleteTableInput`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//DeleteTableInput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTableInput {
    /// The name of the table to delete. You can also provide the Amazon Resource Name (ARN) of
    /// the table in this parameter.
    pub table_name: String,
}

impl DeleteTableInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            table_name: "test-table_name".into(),
        }
    }
}

/// Represents the output of a DeleteTable operation.
///
/// **AWS API**: `dynamodb.v1.DeleteTableOutput`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//DeleteTableOutput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTableOutput {
    /// Represents the properties of a table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

impl DeleteTableOutput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            table_description: Some(TableDescription::fixture()),
        }
    }
}

/// Represents the properties of a table.
///
/// **AWS API**: `dynamodb.v1.TableDescription`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//TableDescription>
///
/// ## Coverage
/// 16 of 28 fields included.
/// Omitted fields:
/// - `LocalSecondaryIndexes` — not selected in manifest
/// - `GlobalSecondaryIndexes` — not selected in manifest
/// - `LatestStreamLabel` — not selected in manifest
/// - `LatestStreamArn` — not selected in manifest
/// - `GlobalTableVersion` — not selected in manifest
/// - `Replicas` — not selected in manifest
/// - `GlobalTableWitnesses` — not selected in manifest
/// - `GlobalTableSettingsReplicationMode` — not selected in manifest
/// - `RestoreSummary` — not selected in manifest
/// - `OnDemandThroughput` — not selected in manifest
/// - `WarmThroughput` — not selected in manifest
/// - `MultiRegionConsistency` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TableDescription {
    /// The name of the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,

    /// The Amazon Resource Name (ARN) that uniquely identifies the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,

    /// Unique identifier for the table for which the backup was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,

    /// The current state of the table: CREATING
    /// - The table is being created. UPDATING
    /// - The table/index configuration is being updated. The table/index remains available for
    ///   data operations when UPDATING. DELETING
    /// - The table is being deleted. ACTIVE
    /// - The table is ready for use. INACCESSIBLE_ENCRYPTION_CREDENTIALS
    /// - The KMS key used to encrypt the table in inaccessible. Table operations may fail due
    ///   to failure to use the KMS key. DynamoDB will initiate the table archival process when
    ///   a table's KMS key remains inaccessible for more than seven days. ARCHIVING
    /// - The table is being archived. Operations are not allowed until archival is complete.
    ///   ARCHIVED
    /// - The table has been archived. See the ArchivalReason for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<String>,

    /// The date and time when the table was created, in UNIX epoch time format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,

    /// The number of items in the specified table. DynamoDB updates this value approximately
    /// every six hours. Recent changes might not be reflected in this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,

    /// The total size of the specified table, in bytes. DynamoDB updates this value
    /// approximately every six hours. Recent changes might not be reflected in this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size_bytes: Option<i64>,

    /// Contains the details for the read/write capacity mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode_summary: Option<BillingModeSummary>,

    /// The provisioned throughput settings for the table, consisting of read and write capacity
    /// units, along with data about increases and decreases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughputDescription>,

    /// The primary key structure for the table. Each KeySchemaElement consists of:
    /// AttributeName
    /// - The name of the attribute. KeyType
    /// - The role of the attribute: HASH
    /// - partition key RANGE
    /// - sort key The partition key of an item is also known as its hash attribute. The term
    ///   "hash attribute" derives from DynamoDB's usage of an internal hash function to evenly
    ///   distribute data items across partitions, based on their partition key values. The sort
    ///   key of an item is also known as its range attribute. The term "range attribute"
    ///   derives from the way DynamoDB stores items with the same partition key physically
    ///   close together, in sorted order by the sort key value. For more information about
    ///   primary keys, see Primary Key in the Amazon DynamoDB Developer Guide.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub key_schema: Vec<KeySchemaElement>,

    /// An array of AttributeDefinition objects. Each of these objects describes one attribute
    /// in the table and index key schema. Each AttributeDefinition object in this array is
    /// composed of: AttributeName
    /// - The name of the attribute. AttributeType
    /// - The data type for the attribute.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attribute_definitions: Vec<AttributeDefinition>,

    /// The current DynamoDB Streams configuration for the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,

    /// The description of the server-side encryption status on the specified table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_description: Option<SSEDescription>,

    /// Contains details of the table class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_class_summary: Option<TableClassSummary>,

    /// Indicates whether deletion protection is enabled (true) or disabled (false) on the
    /// table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,

    /// Contains information about the table archive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_summary: Option<ArchivalSummary>,
}

impl TableDescription {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            table_name: Some("test-table_name".into()),
            table_arn: Some("test-table_arn".into()),
            table_id: Some("test-table_id".into()),
            table_status: Some("test-table_status".into()),
            item_count: Some(100),
            table_size_bytes: Some(100),
            billing_mode_summary: Some(BillingModeSummary::fixture()),
            provisioned_throughput: Some(ProvisionedThroughputDescription::fixture()),
            key_schema: vec![],
            attribute_definitions: vec![],
            stream_specification: Some(StreamSpecification::fixture()),
            sse_description: Some(SSEDescription::fixture()),
            table_class_summary: Some(TableClassSummary::fixture()),
            deletion_protection_enabled: Some(false),
            archival_summary: Some(ArchivalSummary::fixture()),
            ..Default::default()
        }
    }
}

/// Contains the details for the read/write capacity mode. This page talks about PROVISIONED and
/// PAY_PER_REQUEST billing modes. For more information about these modes, see Read/write
/// capacity mode. You may need to switch to on-demand mode at least once in order to return a
/// BillingModeSummary response.
///
/// **AWS API**: `dynamodb.v1.BillingModeSummary`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//BillingModeSummary>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BillingModeSummary {
    /// Controls how you are charged for read and write throughput and how you manage capacity.
    /// This setting can be changed later. PROVISIONED
    /// - Sets the read/write capacity mode to PROVISIONED. We recommend using PROVISIONED for
    ///   predictable workloads. PAY_PER_REQUEST
    /// - Sets the read/write capacity mode to PAY_PER_REQUEST. We recommend using
    ///   PAY_PER_REQUEST for unpredictable workloads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,

    /// Represents the time when PAY_PER_REQUEST was last set as the read/write capacity mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_to_pay_per_request_date_time: Option<f64>,
}

impl BillingModeSummary {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            billing_mode: Some("test-billing_mode".into()),
            ..Default::default()
        }
    }
}

/// Represents the provisioned throughput settings for the table, consisting of read and write
/// capacity units, along with data about increases and decreases.
///
/// **AWS API**: `dynamodb.v1.ProvisionedThroughputDescription`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//ProvisionedThroughputDescription>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProvisionedThroughputDescription {
    /// The maximum number of strongly consistent reads consumed per second before DynamoDB
    /// returns a ThrottlingException. Eventually consistent reads require less effort than
    /// strongly consistent reads, so a setting of 50 ReadCapacityUnits per second provides 100
    /// eventually consistent ReadCapacityUnits per second.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i64>,

    /// The maximum number of writes consumed per second before DynamoDB returns a
    /// ThrottlingException.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<i64>,

    /// The number of provisioned throughput decreases for this table during this UTC calendar
    /// day. For current maximums on provisioned throughput decreases, see Service, Account, and
    /// Table Quotas in the Amazon DynamoDB Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_decreases_today: Option<i64>,

    /// The date and time of the last provisioned throughput increase for this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_increase_date_time: Option<f64>,

    /// The date and time of the last provisioned throughput decrease for this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_decrease_date_time: Option<f64>,
}

impl ProvisionedThroughputDescription {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            read_capacity_units: Some(100),
            write_capacity_units: Some(100),
            number_of_decreases_today: Some(100),
            ..Default::default()
        }
    }
}

/// Represents the provisioned throughput settings for the specified global secondary index. You
/// must use ProvisionedThroughput or OnDemandThroughput based on your table’s capacity mode.
/// For current minimum and maximum provisioned throughput values, see Service, Account, and
/// Table Quotas in the Amazon DynamoDB Developer Guide.
///
/// **AWS API**: `dynamodb.v1.ProvisionedThroughput`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//ProvisionedThroughput>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProvisionedThroughput {
    /// The maximum number of strongly consistent reads consumed per second before DynamoDB
    /// returns a ThrottlingException. For more information, see Specifying Read and Write
    /// Requirements in the Amazon DynamoDB Developer Guide. If read/write capacity mode is
    /// PAY_PER_REQUEST the value is set to 0.
    pub read_capacity_units: i64,

    /// The maximum number of writes consumed per second before DynamoDB returns a
    /// ThrottlingException. For more information, see Specifying Read and Write Requirements in
    /// the Amazon DynamoDB Developer Guide. If read/write capacity mode is PAY_PER_REQUEST the
    /// value is set to 0.
    pub write_capacity_units: i64,
}

impl ProvisionedThroughput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            read_capacity_units: 100,
            write_capacity_units: 100,
        }
    }
}

/// Represents a single element of a key schema. A key schema specifies the attributes that make
/// up the primary key of a table, or the key attributes of an index. A KeySchemaElement
/// represents exactly one attribute of the primary key. For example, a simple primary key would
/// be represented by one KeySchemaElement (for the partition key). A composite primary key
/// would require one KeySchemaElement for the partition key, and another KeySchemaElement for
/// the sort key. A KeySchemaElement must be a scalar, top-level attribute (not a nested
/// attribute). The data type must be one of String, Number, or Binary. The attribute cannot be
/// nested within a List or a Map.
///
/// **AWS API**: `dynamodb.v1.KeySchemaElement`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//KeySchemaElement>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeySchemaElement {
    /// The name of a key attribute.
    pub attribute_name: String,

    /// The role that this key attribute will assume: HASH
    /// - partition key RANGE
    /// - sort key The partition key of an item is also known as its hash attribute. The term
    ///   "hash attribute" derives from DynamoDB's usage of an internal hash function to evenly
    ///   distribute data items across partitions, based on their partition key values. The sort
    ///   key of an item is also known as its range attribute. The term "range attribute"
    ///   derives from the way DynamoDB stores items with the same partition key physically
    ///   close together, in sorted order by the sort key value.
    pub key_type: String,
}

impl KeySchemaElement {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            attribute_name: "test-attribute_name".into(),
            key_type: "test-key_type".into(),
        }
    }
}

/// Represents an attribute for describing the schema for the table and indexes.
///
/// **AWS API**: `dynamodb.v1.AttributeDefinition`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//AttributeDefinition>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttributeDefinition {
    /// A name for the attribute.
    pub attribute_name: String,

    /// The data type for the attribute, where: S
    /// - the attribute is of type String N
    /// - the attribute is of type Number B
    /// - the attribute is of type Binary
    pub attribute_type: String,
}

impl AttributeDefinition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            attribute_name: "test-attribute_name".into(),
            attribute_type: "test-attribute_type".into(),
        }
    }
}

/// Represents the DynamoDB Streams configuration for a table in DynamoDB.
///
/// **AWS API**: `dynamodb.v1.StreamSpecification`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//StreamSpecification>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StreamSpecification {
    /// Indicates whether DynamoDB Streams is enabled (true) or disabled (false) on the table.
    #[serde(default)]
    pub stream_enabled: bool,

    /// When an item in the table is modified, StreamViewType determines what information is
    /// written to the stream for this table. Valid values for StreamViewType are: KEYS_ONLY
    /// - Only the key attributes of the modified item are written to the stream. NEW_IMAGE
    /// - The entire item, as it appears after it was modified, is written to the stream.
    ///   OLD_IMAGE
    /// - The entire item, as it appeared before it was modified, is written to the stream.
    ///   NEW_AND_OLD_IMAGES
    /// - Both the new and the old item images of the item are written to the stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view_type: Option<String>,
}

impl StreamSpecification {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stream_enabled: false,
            stream_view_type: Some("test-stream_view_type".into()),
        }
    }
}

/// The description of the server-side encryption status on the specified table.
///
/// **AWS API**: `dynamodb.v1.SSEDescription`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//SSEDescription>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SSEDescription {
    /// Represents the current state of server-side encryption. The only supported values are:
    /// ENABLED
    /// - Server-side encryption is enabled. UPDATING
    /// - Server-side encryption is being updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Server-side encryption type. The only supported value is: KMS
    /// - Server-side encryption that uses Key Management Service. The key is stored in your
    ///   account and is managed by KMS (KMS charges apply).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_type: Option<String>,

    /// The KMS key ARN used for the KMS encryption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_arn: Option<String>,

    /// Indicates the time, in UNIX epoch date format, when DynamoDB detected that the table's
    /// KMS key was inaccessible. This attribute will automatically be cleared when DynamoDB
    /// detects that the table's KMS key is accessible again. DynamoDB will initiate the table
    /// archival process when table's KMS key remains inaccessible for more than seven days from
    /// this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inaccessible_encryption_date_time: Option<f64>,
}

impl SSEDescription {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            status: Some("test-status".into()),
            sse_type: Some("test-sse_type".into()),
            kms_master_key_arn: Some("test-kms_master_key_arn".into()),
            ..Default::default()
        }
    }
}

/// Contains details of the table class.
///
/// **AWS API**: `dynamodb.v1.TableClassSummary`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//TableClassSummary>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TableClassSummary {
    /// The table class of the specified table. Valid values are STANDARD and
    /// STANDARD_INFREQUENT_ACCESS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_class: Option<String>,

    /// The date and time at which the table class was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<f64>,
}

impl TableClassSummary {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            table_class: Some("test-table_class".into()),
            ..Default::default()
        }
    }
}

/// Contains details of a table archival operation.
///
/// **AWS API**: `dynamodb.v1.ArchivalSummary`
/// **Reference**: <https://docs.aws.amazon.com/amazondynamodb/latest/APIReference//ArchivalSummary>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArchivalSummary {
    /// The date and time when table archival was initiated by DynamoDB, in UNIX epoch time
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_date_time: Option<f64>,

    /// The reason DynamoDB archived the table. Currently, the only possible value is:
    /// INACCESSIBLE_ENCRYPTION_CREDENTIALS
    /// - The table was archived due to the table's KMS key being inaccessible for more than
    ///   seven days. An On-Demand backup was created at the archival time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_reason: Option<String>,

    /// The Amazon Resource Name (ARN) of the backup the table was archived to, when applicable
    /// in the archival reason. If you wish to restore this backup to the same table name, you
    /// will need to delete the original table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_backup_arn: Option<String>,
}

impl ArchivalSummary {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            archival_reason: Some("test-archival_reason".into()),
            archival_backup_arn: Some("test-archival_backup_arn".into()),
            ..Default::default()
        }
    }
}
