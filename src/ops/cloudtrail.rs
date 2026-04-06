//! Operation contracts for the AWS CloudTrail API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/cloudtrail.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::cloudtrail::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the AWS CloudTrail API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::cloudtrail::CloudtrailClient`] instead.
pub struct CloudtrailOps<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> CloudtrailOps<'a> {
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
        "https://cloudtrail.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Retrieves settings for one or more trails associated with the current region.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeTrailsRequest`]
    ///
    /// # Response
    /// [`DescribeTrailsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_trails(
        &self,
        body: &DescribeTrailsRequest,
    ) -> Result<DescribeTrailsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize describe_trails request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "cloudtrail",
                "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.DescribeTrails",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_trails response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse describe_trails response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Returns a JSON-formatted list of information about the specified trail.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetTrailStatusRequest`]
    ///
    /// # Response
    /// [`GetTrailStatusResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_trail_status(
        &self,
        body: &GetTrailStatusRequest,
    ) -> Result<GetTrailStatusResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize get_trail_status request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "cloudtrail",
                "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.GetTrailStatus",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_trail_status response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse get_trail_status response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Describes the settings for the event selectors configured for your trail.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetEventSelectorsRequest`]
    ///
    /// # Response
    /// [`GetEventSelectorsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_event_selectors(
        &self,
        body: &GetEventSelectorsRequest,
    ) -> Result<GetEventSelectorsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize get_event_selectors request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "cloudtrail",
                "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.GetEventSelectors",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_event_selectors response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse get_event_selectors response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a trail.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteTrailRequest`]
    ///
    /// # Response
    /// [`DeleteTrailResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_trail(
        &self,
        body: &DeleteTrailRequest,
    ) -> Result<DeleteTrailResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize delete_trail request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "cloudtrail",
                "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.DeleteTrail",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read delete_trail response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse delete_trail response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Updates trail settings that control what events you are logging.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`UpdateTrailRequest`]
    ///
    /// # Response
    /// [`UpdateTrailResponse`]
    #[allow(dead_code)]
    pub(crate) async fn update_trail(
        &self,
        body: &UpdateTrailRequest,
    ) -> Result<UpdateTrailResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Failed to serialize update_trail request: {e}"),
                body: None,
            })?;
        let response = self
            .client
            .post_json(
                &url,
                "cloudtrail",
                "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.UpdateTrail",
                "1.1",
                &body_bytes,
            )
            .await?;
        let response = response.error_for_status("json").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read update_trail response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
            message: format!("Failed to parse update_trail response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_trails() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DescribeTrailsResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudtrailOps::new(&client);

        let body = DescribeTrailsRequest::fixture();
        let result = ops.describe_trails(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_trail_status() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(GetTrailStatusResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudtrailOps::new(&client);

        let body = GetTrailStatusRequest::fixture();
        let result = ops.get_trail_status(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_event_selectors() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(GetEventSelectorsResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudtrailOps::new(&client);

        let body = GetEventSelectorsRequest::fixture();
        let result = ops.get_event_selectors(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_trail() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(DeleteTrailResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudtrailOps::new(&client);

        let body = DeleteTrailRequest::fixture();
        let result = ops.delete_trail(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_trail() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/")
            .returning_json(serde_json::to_value(UpdateTrailResponse::fixture()).unwrap());

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = CloudtrailOps::new(&client);

        let body = UpdateTrailRequest::fixture();
        let result = ops.update_trail(&body).await;
        assert!(result.is_ok());
    }
}
