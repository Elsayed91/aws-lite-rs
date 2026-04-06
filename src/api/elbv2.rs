//! Elastic Load Balancing v2 API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::elbv2::Elbv2Ops`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::elbv2::Elbv2Ops,
    types::elbv2::{
        DeleteLoadBalancerRequest, DeleteLoadBalancerResponse, DeleteTargetGroupRequest,
        DeleteTargetGroupResponse, DescribeListenersRequest, DescribeListenersResponse,
        DescribeLoadBalancerAttributesRequest, DescribeLoadBalancerAttributesResponse,
        DescribeLoadBalancersRequest, DescribeLoadBalancersResponse, DescribeTargetGroupsRequest,
        DescribeTargetGroupsResponse, DescribeTargetHealthRequest, DescribeTargetHealthResponse,
        ModifyLoadBalancerAttributesRequest, ModifyLoadBalancerAttributesResponse,
    },
};

/// Client for the Elastic Load Balancing v2 API
pub struct Elbv2Client<'a> {
    ops: Elbv2Ops<'a>,
}

impl<'a> Elbv2Client<'a> {
    /// Create a new Elastic Load Balancing v2 API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: Elbv2Ops::new(client),
        }
    }

    /// Describes the specified load balancers or all of your load balancers.
    pub async fn describe_load_balancers(
        &self,
        body: &DescribeLoadBalancersRequest,
    ) -> Result<DescribeLoadBalancersResponse> {
        self.ops.describe_load_balancers(body).await
    }

    /// Describes the specified target groups or all of your target groups. By default, all target groups are described. Alterna
    pub async fn describe_target_groups(
        &self,
        body: &DescribeTargetGroupsRequest,
    ) -> Result<DescribeTargetGroupsResponse> {
        self.ops.describe_target_groups(body).await
    }

    /// Describes the health of the specified targets or all of your targets.
    pub async fn describe_target_health(
        &self,
        body: &DescribeTargetHealthRequest,
    ) -> Result<DescribeTargetHealthResponse> {
        self.ops.describe_target_health(body).await
    }

    /// Describes the specified listeners or the listeners for the specified Application Load Balancer, Network Load Balancer, o
    pub async fn describe_listeners(
        &self,
        body: &DescribeListenersRequest,
    ) -> Result<DescribeListenersResponse> {
        self.ops.describe_listeners(body).await
    }

    /// Describes the attributes for the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. F
    pub async fn describe_load_balancer_attributes(
        &self,
        body: &DescribeLoadBalancerAttributesRequest,
    ) -> Result<DescribeLoadBalancerAttributesResponse> {
        self.ops.describe_load_balancer_attributes(body).await
    }

    /// Deletes the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. Deleting a load balanc
    pub async fn delete_load_balancer(
        &self,
        body: &DeleteLoadBalancerRequest,
    ) -> Result<DeleteLoadBalancerResponse> {
        self.ops.delete_load_balancer(body).await
    }

    /// Deletes the specified target group. You can delete a target group if it is not referenced by any actions. Deleting a tar
    pub async fn delete_target_group(
        &self,
        body: &DeleteTargetGroupRequest,
    ) -> Result<DeleteTargetGroupResponse> {
        self.ops.delete_target_group(body).await
    }

    /// Modifies the specified attributes of the specified Application Load Balancer, Network Load Balancer, or Gateway Load Bal
    pub async fn modify_load_balancer_attributes(
        &self,
        body: &ModifyLoadBalancerAttributesRequest,
    ) -> Result<ModifyLoadBalancerAttributesResponse> {
        self.ops.modify_load_balancer_attributes(body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::elbv2::LoadBalancerAttribute;

    #[tokio::test]
    async fn describe_load_balancers_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeLoadBalancersResponse><DescribeLoadBalancersResult>
                <LoadBalancers>
                    <member>
                        <LoadBalancerArn>arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/abc123</LoadBalancerArn>
                        <DNSName>internal-my-alb-123.us-east-1.elb.amazonaws.com</DNSName>
                        <LoadBalancerName>my-alb</LoadBalancerName>
                        <Scheme>internal</Scheme>
                        <VpcId>vpc-12345</VpcId>
                        <State>
                            <Code>active</Code>
                        </State>
                        <Type>application</Type>
                        <AvailabilityZones>
                            <member>
                                <ZoneName>us-east-1a</ZoneName>
                                <SubnetId>subnet-aaa</SubnetId>
                            </member>
                            <member>
                                <ZoneName>us-east-1b</ZoneName>
                                <SubnetId>subnet-bbb</SubnetId>
                            </member>
                        </AvailabilityZones>
                        <SecurityGroups>
                            <member>sg-12345</member>
                        </SecurityGroups>
                        <IpAddressType>ipv4</IpAddressType>
                    </member>
                </LoadBalancers>
            </DescribeLoadBalancersResult></DescribeLoadBalancersResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elbv2()
            .describe_load_balancers(&DescribeLoadBalancersRequest {
                names: vec!["my-alb".into()],
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.load_balancers.len(), 1);
        let lb = &result.load_balancers[0];
        assert_eq!(lb.load_balancer_name.as_deref(), Some("my-alb"));
        assert_eq!(lb.scheme.as_deref(), Some("internal"));
        assert_eq!(lb.r#type.as_deref(), Some("application"));
        assert_eq!(lb.vpc_id.as_deref(), Some("vpc-12345"));
        assert_eq!(lb.ip_address_type.as_deref(), Some("ipv4"));
        let state = lb.state.as_ref().unwrap();
        assert_eq!(state.code.as_deref(), Some("active"));
        assert_eq!(lb.availability_zones.len(), 2);
        assert_eq!(
            lb.availability_zones[0].zone_name.as_deref(),
            Some("us-east-1a")
        );
        assert_eq!(lb.security_groups.len(), 1);
        assert_eq!(lb.security_groups[0], "sg-12345");
    }

    #[tokio::test]
    async fn describe_target_groups_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeTargetGroupsResponse><DescribeTargetGroupsResult>
                <TargetGroups>
                    <member>
                        <TargetGroupArn>arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/my-tg/abc123</TargetGroupArn>
                        <TargetGroupName>my-tg</TargetGroupName>
                        <Protocol>HTTP</Protocol>
                        <Port>80</Port>
                        <VpcId>vpc-12345</VpcId>
                        <TargetType>ip</TargetType>
                        <HealthCheckEnabled>true</HealthCheckEnabled>
                        <HealthCheckProtocol>HTTP</HealthCheckProtocol>
                        <HealthCheckPort>traffic-port</HealthCheckPort>
                        <HealthCheckPath>/</HealthCheckPath>
                        <HealthCheckIntervalSeconds>30</HealthCheckIntervalSeconds>
                        <HealthCheckTimeoutSeconds>5</HealthCheckTimeoutSeconds>
                        <HealthyThresholdCount>5</HealthyThresholdCount>
                        <UnhealthyThresholdCount>2</UnhealthyThresholdCount>
                        <LoadBalancerArns>
                            <member>arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/abc123</member>
                        </LoadBalancerArns>
                    </member>
                </TargetGroups>
            </DescribeTargetGroupsResult></DescribeTargetGroupsResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elbv2()
            .describe_target_groups(&DescribeTargetGroupsRequest {
                names: vec!["my-tg".into()],
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.target_groups.len(), 1);
        let tg = &result.target_groups[0];
        assert_eq!(tg.target_group_name.as_deref(), Some("my-tg"));
        assert_eq!(tg.protocol.as_deref(), Some("HTTP"));
        assert_eq!(tg.port, Some(80));
        assert_eq!(tg.target_type.as_deref(), Some("ip"));
        assert_eq!(tg.health_check_enabled, Some(true));
        assert_eq!(tg.health_check_path.as_deref(), Some("/"));
        assert_eq!(tg.healthy_threshold_count, Some(5));
        assert_eq!(tg.unhealthy_threshold_count, Some(2));
        assert_eq!(tg.load_balancer_arns.len(), 1);
    }

    #[tokio::test]
    async fn describe_target_health_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeTargetHealthResponse><DescribeTargetHealthResult>
                <TargetHealthDescriptions>
                    <member>
                        <Target>
                            <Id>10.0.1.5</Id>
                            <Port>80</Port>
                        </Target>
                        <HealthCheckPort>80</HealthCheckPort>
                        <TargetHealth>
                            <State>healthy</State>
                        </TargetHealth>
                    </member>
                </TargetHealthDescriptions>
            </DescribeTargetHealthResult></DescribeTargetHealthResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elbv2()
            .describe_target_health(&DescribeTargetHealthRequest {
                target_group_arn:
                    "arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/my-tg/abc"
                        .into(),
            })
            .await
            .unwrap();
        assert_eq!(result.target_health_descriptions.len(), 1);
        let thd = &result.target_health_descriptions[0];
        let target = thd.target.as_ref().unwrap();
        assert_eq!(target.id, "10.0.1.5");
        assert_eq!(target.port, Some(80));
        assert_eq!(thd.health_check_port.as_deref(), Some("80"));
        let health = thd.target_health.as_ref().unwrap();
        assert_eq!(health.state.as_deref(), Some("healthy"));
    }

    #[tokio::test]
    async fn describe_listeners_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeListenersResponse><DescribeListenersResult>
                <Listeners>
                    <member>
                        <ListenerArn>arn:aws:elasticloadbalancing:us-east-1:123456789012:listener/app/my-alb/abc/def</ListenerArn>
                        <LoadBalancerArn>arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/abc123</LoadBalancerArn>
                        <Port>443</Port>
                        <Protocol>HTTPS</Protocol>
                        <SslPolicy>ELBSecurityPolicy-2016-08</SslPolicy>
                    </member>
                </Listeners>
            </DescribeListenersResult></DescribeListenersResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elbv2()
            .describe_listeners(&DescribeListenersRequest {
                load_balancer_arn: Some("arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/abc123".into()),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.listeners.len(), 1);
        let l = &result.listeners[0];
        assert_eq!(l.protocol.as_deref(), Some("HTTPS"));
        assert_eq!(l.port, Some(443));
        assert_eq!(l.ssl_policy.as_deref(), Some("ELBSecurityPolicy-2016-08"));
        assert!(l.listener_arn.is_some());
        assert!(l.load_balancer_arn.is_some());
    }

    #[tokio::test]
    async fn describe_load_balancer_attributes_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeLoadBalancerAttributesResponse><DescribeLoadBalancerAttributesResult>
                <Attributes>
                    <member>
                        <Key>deletion_protection.enabled</Key>
                        <Value>false</Value>
                    </member>
                    <member>
                        <Key>access_logs.s3.enabled</Key>
                        <Value>true</Value>
                    </member>
                    <member>
                        <Key>access_logs.s3.bucket</Key>
                        <Value>my-logs-bucket</Value>
                    </member>
                </Attributes>
            </DescribeLoadBalancerAttributesResult></DescribeLoadBalancerAttributesResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elbv2()
            .describe_load_balancer_attributes(&DescribeLoadBalancerAttributesRequest {
                load_balancer_arn: "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/abc123".into(),
            })
            .await
            .unwrap();
        assert_eq!(result.attributes.len(), 3);
        assert_eq!(
            result.attributes[0].key.as_deref(),
            Some("deletion_protection.enabled")
        );
        assert_eq!(result.attributes[0].value.as_deref(), Some("false"));
        assert_eq!(
            result.attributes[1].key.as_deref(),
            Some("access_logs.s3.enabled")
        );
        assert_eq!(result.attributes[1].value.as_deref(), Some("true"));
        assert_eq!(
            result.attributes[2].key.as_deref(),
            Some("access_logs.s3.bucket")
        );
        assert_eq!(
            result.attributes[2].value.as_deref(),
            Some("my-logs-bucket")
        );
    }

    #[tokio::test]
    async fn delete_load_balancer_returns_success() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DeleteLoadBalancerResponse><DeleteLoadBalancerResult>
            </DeleteLoadBalancerResult></DeleteLoadBalancerResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elbv2()
            .delete_load_balancer(&DeleteLoadBalancerRequest {
                load_balancer_arn: "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/abc123".into(),
            })
            .await;
        assert!(result.is_ok(), "delete_load_balancer should succeed");
    }

    #[tokio::test]
    async fn delete_target_group_returns_success() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DeleteTargetGroupResponse><DeleteTargetGroupResult>
            </DeleteTargetGroupResult></DeleteTargetGroupResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elbv2()
            .delete_target_group(&DeleteTargetGroupRequest {
                target_group_arn:
                    "arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/my-tg/abc123"
                        .into(),
            })
            .await;
        assert!(result.is_ok(), "delete_target_group should succeed");
    }

    #[tokio::test]
    async fn modify_load_balancer_attributes_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<ModifyLoadBalancerAttributesResponse><ModifyLoadBalancerAttributesResult>
                <Attributes>
                    <member>
                        <Key>idle_timeout.timeout_seconds</Key>
                        <Value>120</Value>
                    </member>
                    <member>
                        <Key>deletion_protection.enabled</Key>
                        <Value>false</Value>
                    </member>
                </Attributes>
            </ModifyLoadBalancerAttributesResult></ModifyLoadBalancerAttributesResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .elbv2()
            .modify_load_balancer_attributes(&ModifyLoadBalancerAttributesRequest {
                load_balancer_arn: "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/abc123".into(),
                attributes: vec![LoadBalancerAttribute {
                    key: Some("idle_timeout.timeout_seconds".into()),
                    value: Some("120".into()),
                }],
            })
            .await
            .unwrap();
        assert_eq!(result.attributes.len(), 2);
        assert_eq!(
            result.attributes[0].key.as_deref(),
            Some("idle_timeout.timeout_seconds")
        );
        assert_eq!(result.attributes[0].value.as_deref(), Some("120"));
        assert_eq!(
            result.attributes[1].key.as_deref(),
            Some("deletion_protection.enabled")
        );
        assert_eq!(result.attributes[1].value.as_deref(), Some("false"));
    }
}
