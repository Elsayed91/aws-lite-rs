//! AWS CloudTrail API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::cloudtrail::CloudtrailOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::cloudtrail::CloudtrailOps,
    types::cloudtrail::{
        DeleteTrailRequest, DeleteTrailResponse, DescribeTrailsRequest, DescribeTrailsResponse,
        GetEventSelectorsRequest, GetEventSelectorsResponse, GetTrailStatusRequest,
        GetTrailStatusResponse, UpdateTrailRequest, UpdateTrailResponse,
    },
};

/// Client for the AWS CloudTrail API
pub struct CloudtrailClient<'a> {
    ops: CloudtrailOps<'a>,
}

impl<'a> CloudtrailClient<'a> {
    /// Create a new AWS CloudTrail API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: CloudtrailOps::new(client),
        }
    }

    /// Retrieves settings for one or more trails associated with the current region.
    pub async fn describe_trails(
        &self,
        body: &DescribeTrailsRequest,
    ) -> Result<DescribeTrailsResponse> {
        self.ops.describe_trails(body).await
    }

    /// Returns a JSON-formatted list of information about the specified trail.
    pub async fn get_trail_status(
        &self,
        body: &GetTrailStatusRequest,
    ) -> Result<GetTrailStatusResponse> {
        self.ops.get_trail_status(body).await
    }

    /// Describes the settings for the event selectors configured for your trail.
    pub async fn get_event_selectors(
        &self,
        body: &GetEventSelectorsRequest,
    ) -> Result<GetEventSelectorsResponse> {
        self.ops.get_event_selectors(body).await
    }

    /// Deletes a trail.
    pub async fn delete_trail(&self, body: &DeleteTrailRequest) -> Result<DeleteTrailResponse> {
        self.ops.delete_trail(body).await
    }

    /// Updates trail settings that control what events you are logging.
    pub async fn update_trail(&self, body: &UpdateTrailRequest) -> Result<UpdateTrailResponse> {
        self.ops.update_trail(body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn describe_trails_returns_trail_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "trailList": [
                {
                    "Name": "my-trail",
                    "TrailARN": "arn:aws:cloudtrail:us-east-1:123456789012:trail/my-trail",
                    "HomeRegion": "us-east-1",
                    "S3BucketName": "my-trail-logs",
                    "IsMultiRegionTrail": true,
                    "IsOrganizationTrail": false,
                    "HasCustomEventSelectors": false,
                    "HasInsightSelectors": false
                }
            ]
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .cloudtrail()
            .describe_trails(&DescribeTrailsRequest::default())
            .await
            .unwrap();
        assert_eq!(result.trail_list.len(), 1);
        let trail = &result.trail_list[0];
        assert_eq!(trail.name.as_deref(), Some("my-trail"));
        assert_eq!(
            trail.trail_arn.as_deref(),
            Some("arn:aws:cloudtrail:us-east-1:123456789012:trail/my-trail")
        );
        assert_eq!(trail.home_region.as_deref(), Some("us-east-1"));
        assert_eq!(trail.s3_bucket_name.as_deref(), Some("my-trail-logs"));
        assert_eq!(trail.is_multi_region_trail, Some(true));
    }

    #[tokio::test]
    async fn get_trail_status_returns_logging_state() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "IsLogging": true,
            "StartLoggingTime": 1700000000.0,
            "LatestDeliveryTime": 1700001000.0
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .cloudtrail()
            .get_trail_status(&GetTrailStatusRequest {
                name: "my-trail".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(result.is_logging, Some(true));
        assert_eq!(result.start_logging_time, Some(1700000000.0));
        assert_eq!(result.latest_delivery_time, Some(1700001000.0));
    }

    #[tokio::test]
    async fn update_trail_returns_updated_config() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "Name": "my-trail",
            "TrailARN": "arn:aws:cloudtrail:us-east-1:123456789012:trail/my-trail",
            "S3BucketName": "my-trail-logs",
            "IncludeGlobalServiceEvents": false,
            "IsMultiRegionTrail": false,
            "LogFileValidationEnabled": true
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .cloudtrail()
            .update_trail(&UpdateTrailRequest {
                name: "my-trail".to_string(),
                include_global_service_events: Some(false),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.name.as_deref(), Some("my-trail"));
        assert_eq!(
            result.trail_arn.as_deref(),
            Some("arn:aws:cloudtrail:us-east-1:123456789012:trail/my-trail")
        );
        assert_eq!(result.include_global_service_events, Some(false));
        assert_eq!(result.log_file_validation_enabled, Some(true));
    }

    #[tokio::test]
    async fn delete_trail_returns_empty_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({}));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .cloudtrail()
            .delete_trail(&DeleteTrailRequest {
                name: "my-trail".to_string(),
            })
            .await;
        assert!(result.is_ok(), "DeleteTrail should succeed with empty body");
    }

    #[tokio::test]
    async fn get_event_selectors_returns_selectors() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_json(serde_json::json!({
            "TrailARN": "arn:aws:cloudtrail:us-east-1:123456789012:trail/my-trail",
            "EventSelectors": [
                {
                    "ReadWriteType": "All",
                    "IncludeManagementEvents": true,
                    "DataResources": []
                }
            ],
            "AdvancedEventSelectors": []
        }));
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .cloudtrail()
            .get_event_selectors(&GetEventSelectorsRequest {
                trail_name: "my-trail".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(
            result.trail_arn.as_deref(),
            Some("arn:aws:cloudtrail:us-east-1:123456789012:trail/my-trail")
        );
        assert_eq!(result.event_selectors.len(), 1);
        assert_eq!(
            result.event_selectors[0].read_write_type.as_deref(),
            Some("All")
        );
        assert_eq!(
            result.event_selectors[0].include_management_events,
            Some(true)
        );
        assert!(result.advanced_event_selectors.is_empty());
    }
}
