//! Amazon Kinesis API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::kinesis::KinesisOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::kinesis::KinesisOps,
    types::kinesis::{
        DeleteStreamInput, DescribeStreamSummaryInput, DescribeStreamSummaryOutput,
        ListStreamsInput, ListStreamsOutput, UpdateStreamModeInput,
    },
};

/// Client for the Amazon Kinesis API
pub struct KinesisClient<'a> {
    ops: KinesisOps<'a>,
}

impl<'a> KinesisClient<'a> {
    /// Create a new Amazon Kinesis API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: KinesisOps::new(client),
        }
    }

    /// Lists the Kinesis data streams associated with the AWS account.
    pub async fn list_streams(&self, body: &ListStreamsInput) -> Result<ListStreamsOutput> {
        self.ops.list_streams(body).await
    }

    /// Provides a summarized description of the specified Kinesis data stream.
    pub async fn describe_stream_summary(
        &self,
        body: &DescribeStreamSummaryInput,
    ) -> Result<DescribeStreamSummaryOutput> {
        self.ops.describe_stream_summary(body).await
    }

    /// Deletes a Kinesis data stream and all its shards and data.
    pub async fn delete_stream(&self, body: &DeleteStreamInput) -> Result<()> {
        self.ops.delete_stream(body).await
    }

    /// Updates the capacity mode of a Kinesis data stream (PROVISIONED or ON_DEMAND).
    pub async fn update_stream_mode(&self, body: &UpdateStreamModeInput) -> Result<()> {
        self.ops.update_stream_mode(body).await
    }
}
