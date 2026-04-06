//! Amazon Auto Scaling API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::autoscaling::AutoscalingOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::autoscaling::AutoscalingOps,
    types::autoscaling::{
        DescribeAutoScalingGroupsRequest, DescribeAutoScalingGroupsResponse,
        DescribeLaunchConfigurationsRequest, DescribeLaunchConfigurationsResponse,
        StartInstanceRefreshRequest, StartInstanceRefreshResponse, UpdateAutoScalingGroupRequest,
    },
};

/// Client for the Amazon Auto Scaling API.
pub struct AutoScalingClient<'a> {
    ops: AutoscalingOps<'a>,
}

impl<'a> AutoScalingClient<'a> {
    /// Create a new Auto Scaling API client.
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: AutoscalingOps::new(client),
        }
    }

    /// Describes all Auto Scaling groups.
    pub async fn describe_auto_scaling_groups(&self) -> Result<DescribeAutoScalingGroupsResponse> {
        let body = DescribeAutoScalingGroupsRequest::default();
        self.ops.describe_auto_scaling_groups(&body).await
    }

    /// Describes all launch configurations.
    pub async fn describe_launch_configurations(
        &self,
    ) -> Result<DescribeLaunchConfigurationsResponse> {
        let body = DescribeLaunchConfigurationsRequest::default();
        self.ops.describe_launch_configurations(&body).await
    }

    /// Updates the configuration for the specified Auto Scaling group.
    pub async fn update_auto_scaling_group(
        &self,
        body: &UpdateAutoScalingGroupRequest,
    ) -> Result<()> {
        self.ops.update_auto_scaling_group(body).await
    }

    /// Starts an instance refresh for the specified Auto Scaling group.
    pub async fn start_instance_refresh(
        &self,
        body: &StartInstanceRefreshRequest,
    ) -> Result<StartInstanceRefreshResponse> {
        self.ops.start_instance_refresh(body).await
    }
}

#[cfg(test)]
mod tests {
    use crate::AwsHttpClient;
    use crate::mock_client::MockClient;
    use crate::test_support::autoscaling_mock_helpers::AutoscalingMockHelpers;

    fn xml_envelope(action: &str, inner: &str) -> Vec<u8> {
        format!("<{action}Response><{action}Result>{inner}</{action}Result></{action}Response>")
            .into_bytes()
    }

    #[tokio::test]
    async fn describe_auto_scaling_groups_returns_empty_list() {
        let mut mock = MockClient::new();
        mock.expect_describe_auto_scaling_groups()
            .returning_bytes(xml_envelope(
                "DescribeAutoScalingGroups",
                "<AutoScalingGroups/>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .autoscaling()
            .describe_auto_scaling_groups()
            .await
            .unwrap();
        assert!(response.auto_scaling_groups.is_empty());
    }

    #[tokio::test]
    async fn describe_auto_scaling_groups_returns_groups() {
        let mut mock = MockClient::new();
        mock.expect_describe_auto_scaling_groups()
            .returning_bytes(xml_envelope(
                "DescribeAutoScalingGroups",
                "<AutoScalingGroups>\
                   <member>\
                     <AutoScalingGroupName>my-asg</AutoScalingGroupName>\
                     <MinSize>1</MinSize>\
                     <MaxSize>10</MaxSize>\
                     <DesiredCapacity>3</DesiredCapacity>\
                   </member>\
                 </AutoScalingGroups>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .autoscaling()
            .describe_auto_scaling_groups()
            .await
            .unwrap();

        assert_eq!(response.auto_scaling_groups.len(), 1);
        let group = &response.auto_scaling_groups[0];
        assert_eq!(group.auto_scaling_group_name, "my-asg");
        assert_eq!(group.min_size, 1);
        assert_eq!(group.max_size, 10);
        assert_eq!(group.desired_capacity, 3);
    }

    #[tokio::test]
    async fn describe_launch_configurations_returns_empty_list() {
        let mut mock = MockClient::new();
        mock.expect_describe_launch_configurations()
            .returning_bytes(xml_envelope(
                "DescribeLaunchConfigurations",
                "<LaunchConfigurations/>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client
            .autoscaling()
            .describe_launch_configurations()
            .await
            .unwrap();
        assert!(response.launch_configurations.is_empty());
    }

    #[tokio::test]
    async fn update_auto_scaling_group_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_update_auto_scaling_group()
            .returning_bytes(xml_envelope("UpdateAutoScalingGroup", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::autoscaling::UpdateAutoScalingGroupRequest {
            auto_scaling_group_name: "my-asg".to_string(),
            desired_capacity: Some(5),
            ..Default::default()
        };
        let result = client.autoscaling().update_auto_scaling_group(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn start_instance_refresh_returns_id() {
        let mut mock = MockClient::new();
        mock.expect_start_instance_refresh()
            .returning_bytes(xml_envelope(
                "StartInstanceRefresh",
                "<InstanceRefreshId>abc-123-def</InstanceRefreshId>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::autoscaling::StartInstanceRefreshRequest {
            auto_scaling_group_name: "my-asg".to_string(),
            ..Default::default()
        };
        let response = client
            .autoscaling()
            .start_instance_refresh(&body)
            .await
            .unwrap();
        assert_eq!(response.instance_refresh_id.as_deref(), Some("abc-123-def"));
    }
}
