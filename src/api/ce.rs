//! AWS Cost Explorer API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::ce::CeOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::ce::CeOps,
    types::ce::{GetCostAndUsageRequest, GetCostAndUsageResponse},
};

/// Client for the AWS Cost Explorer API
pub struct CeClient<'a> {
    ops: CeOps<'a>,
}

impl<'a> CeClient<'a> {
    /// Create a new AWS Cost Explorer API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: CeOps::new(client),
        }
    }

    /// Retrieves cost and usage metrics for your account.
    pub async fn get_cost_and_usage(
        &self,
        body: &GetCostAndUsageRequest,
    ) -> Result<GetCostAndUsageResponse> {
        self.ops.get_cost_and_usage(body).await
    }
}
