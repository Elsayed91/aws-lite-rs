//! Types for the Amazon Kinesis API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Represents the input for ListStreams.
///
/// **AWS API**: `kinesis.v1.ListStreamsInput`
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `ExclusiveStartStreamName` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListStreamsInput {
    /// The maximum number of streams to list. The default value is 100. If you specify a value
    /// greater than 100, at most 100 results are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// The `NextToken` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListStreamsInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            limit: Some(100),
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Represents the output for ListStreams.
///
/// **AWS API**: `kinesis.v1.ListStreamsOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListStreamsOutput {
    /// The names of the streams that are associated with the Amazon Web Services account making
    /// the ListStreams request.
    #[serde(default)]
    pub stream_names: Vec<String>,

    /// The `StreamSummaries` field.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stream_summaries: Vec<StreamSummary>,

    /// The `NextToken` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// If set to true, there are more streams available to list.
    #[serde(default)]
    pub has_more_streams: bool,
}

impl ListStreamsOutput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stream_names: vec![],
            stream_summaries: vec![],
            next_token: Some("test-next_token".into()),
            has_more_streams: false,
        }
    }
}

/// The summary of a stream.
///
/// **AWS API**: `kinesis.v1.StreamSummary`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StreamSummary {
    /// The name of a stream.
    pub stream_name: String,

    /// The ARN of the stream.
    pub stream_arn: String,

    /// The status of the stream.
    pub stream_status: String,

    /// The `StreamModeDetails` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_mode_details: Option<StreamModeDetails>,

    /// The timestamp at which the stream was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_creation_timestamp: Option<f64>,
}

impl StreamSummary {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stream_name: "test-stream_name".into(),
            stream_arn: "test-stream_arn".into(),
            stream_status: "test-stream_status".into(),
            stream_mode_details: Some(StreamModeDetails::fixture()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `kinesis.v1.DescribeStreamSummaryInput`
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `StreamId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeStreamSummaryInput {
    /// The name of the stream to describe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,

    /// The ARN of the stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
}

impl DescribeStreamSummaryInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stream_name: Some("test-stream_name".into()),
            stream_arn: Some("test-stream_arn".into()),
        }
    }
}

///
/// **AWS API**: `kinesis.v1.DescribeStreamSummaryOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeStreamSummaryOutput {
    /// A StreamDescriptionSummary containing information about the stream.
    pub stream_description_summary: StreamDescriptionSummary,
}

impl DescribeStreamSummaryOutput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stream_description_summary: StreamDescriptionSummary::fixture(),
        }
    }
}

/// Represents the output for DescribeStreamSummary
///
/// **AWS API**: `kinesis.v1.StreamDescriptionSummary`
///
/// ## Coverage
/// 10 of 14 fields included.
/// Omitted fields:
/// - `StreamId` — not selected in manifest
/// - `EnhancedMonitoring` — not selected in manifest
/// - `WarmThroughput` — not selected in manifest
/// - `MaxRecordSizeInKiB` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StreamDescriptionSummary {
    /// The name of the stream being described.
    pub stream_name: String,

    /// The Amazon Resource Name (ARN) for the stream being described.
    pub stream_arn: String,

    /// The current status of the stream being described. The stream status is one of the
    /// following states: CREATING
    /// - The stream is being created. Kinesis Data Streams immediately returns and sets
    ///   StreamStatus to CREATING. DELETING
    /// - The stream is being deleted. The specified stream is in the DELETING state until
    ///   Kinesis Data Streams completes the deletion. ACTIVE
    /// - The stream exists and is ready for read and write operations or deletion. You should
    ///   perform read and write operations only on an ACTIVE stream. UPDATING
    /// - Shards in the stream are being merged or split. Read and write operations continue to
    ///   work while the stream is in the UPDATING state.
    pub stream_status: String,

    /// Specifies the capacity mode to which you want to set your data stream. Currently, in
    /// Kinesis Data Streams, you can choose between an on-demand ycapacity mode and a
    /// provisioned capacity mode for your data streams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_mode_details: Option<StreamModeDetails>,

    /// The current retention period, in hours.
    pub retention_period_hours: i32,

    /// The approximate time that the stream was created.
    pub stream_creation_timestamp: f64,

    /// The number of open shards in the stream.
    pub open_shard_count: i32,

    /// The number of enhanced fan-out consumers registered with the stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_count: Option<i32>,

    /// The encryption type used. This value is one of the following: KMS NONE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,

    /// The GUID for the customer-managed Amazon Web Services KMS key to use for encryption.
    /// This value can be a globally unique identifier, a fully specified ARN to either an alias
    /// or a key, or an alias name prefixed by "alias/".You can also use a master key owned by
    /// Kinesis Data Streams by specifying the alias aws/kinesis. Key ARN example:
    /// arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012 Alias ARN
    /// example: arn:aws:kms:us-east-1:123456789012:alias/MyAliasName Globally unique key ID
    /// example: 12345678-1234-1234-1234-123456789012 Alias name example: alias/MyAliasName
    /// Master key owned by Kinesis Data Streams: alias/aws/kinesis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

impl StreamDescriptionSummary {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stream_name: "test-stream_name".into(),
            stream_arn: "test-stream_arn".into(),
            stream_status: "test-stream_status".into(),
            stream_mode_details: Some(StreamModeDetails::fixture()),
            retention_period_hours: 100,
            open_shard_count: 100,
            consumer_count: Some(100),
            encryption_type: Some("test-encryption_type".into()),
            key_id: Some("test-key_id".into()),
            ..Default::default()
        }
    }
}

/// Specifies the capacity mode to which you want to set your data stream. Currently, in Kinesis
/// Data Streams, you can choose between an on-demand capacity mode and a provisioned capacity
/// mode for your data streams.
///
/// **AWS API**: `kinesis.v1.StreamModeDetails`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StreamModeDetails {
    /// Specifies the capacity mode to which you want to set your data stream. Currently, in
    /// Kinesis Data Streams, you can choose between an on-demand capacity mode and a
    /// provisioned capacity mode for your data streams.
    pub stream_mode: String,
}

impl StreamModeDetails {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stream_mode: "test-stream_mode".into(),
        }
    }
}

/// Represents the input for DeleteStream.
///
/// **AWS API**: `kinesis.v1.DeleteStreamInput`
///
/// ## Coverage
/// 3 of 4 fields included.
/// Omitted fields:
/// - `StreamId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteStreamInput {
    /// The name of the stream to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,

    /// The ARN of the stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,

    /// If this parameter is unset (null) or if you set it to false, and the stream has
    /// registered consumers, the call to DeleteStream fails with a ResourceInUseException.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_consumer_deletion: Option<bool>,
}

impl DeleteStreamInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stream_name: Some("test-stream_name".into()),
            stream_arn: Some("test-stream_arn".into()),
            enforce_consumer_deletion: Some(false),
        }
    }
}

///
/// **AWS API**: `kinesis.v1.UpdateStreamModeInput`
///
/// ## Coverage
/// 2 of 4 fields included.
/// Omitted fields:
/// - `StreamId` — not selected in manifest
/// - `WarmThroughputMiBps` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateStreamModeInput {
    /// Specifies the ARN of the data stream whose capacity mode you want to update.
    pub stream_arn: String,

    /// Specifies the capacity mode to which you want to set your data stream. Currently, in
    /// Kinesis Data Streams, you can choose between an on-demand capacity mode and a
    /// provisioned capacity mode for your data streams.
    pub stream_mode_details: StreamModeDetails,
}

impl UpdateStreamModeInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            stream_arn: "test-stream_arn".into(),
            stream_mode_details: StreamModeDetails::fixture(),
        }
    }
}
