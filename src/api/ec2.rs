//! Amazon EC2 API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::ec2::Ec2Ops`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::ec2::Ec2Ops,
    types::ec2::{
        AssociateIamInstanceProfileRequest, AssociateIamInstanceProfileResponse,
        AuthorizeSecurityGroupIngressRequest, AuthorizeSecurityGroupIngressResponse,
        CreateFlowLogsRequest, CreateFlowLogsResponse, CreateSnapshotRequest, CreateTagsRequest,
        DeleteNatGatewayRequest, DeleteNatGatewayResponse, DeleteSecurityGroupRequest,
        DeleteSecurityGroupResponse, DeleteSnapshotRequest, DeleteVolumeRequest,
        DeleteVpcEndpointsRequest, DeleteVpcEndpointsResponse, DeregisterImageRequest,
        DeregisterImageResponse, DescribeAddressesRequest, DescribeAddressesResponse,
        DescribeFlowLogsRequest, DescribeFlowLogsResponse, DescribeImagesRequest,
        DescribeImagesResponse, DescribeInstancesRequest, DescribeInstancesResponse,
        DescribeLaunchTemplateVersionsRequest, DescribeLaunchTemplateVersionsResponse,
        DescribeLaunchTemplatesRequest, DescribeLaunchTemplatesResponse,
        DescribeNatGatewaysRequest, DescribeNatGatewaysResponse, DescribeNetworkAclsRequest,
        DescribeNetworkAclsResponse, DescribeRouteTablesRequest, DescribeRouteTablesResponse,
        DescribeSecurityGroupsRequest, DescribeSecurityGroupsResponse,
        DescribeSnapshotAttributeRequest, DescribeSnapshotAttributeResponse,
        DescribeSnapshotsRequest, DescribeSnapshotsResponse, DescribeVolumesRequest,
        DescribeVolumesResponse, DescribeVpcEndpointsRequest, DescribeVpcEndpointsResponse,
        DescribeVpcPeeringConnectionsRequest, DescribeVpcPeeringConnectionsResponse,
        DescribeVpcsRequest, DescribeVpcsResponse, DetachVolumeRequest,
        EnableEbsEncryptionByDefaultRequest, EnableEbsEncryptionByDefaultResponse,
        EnableImageBlockPublicAccessRequest, EnableImageBlockPublicAccessResponse,
        EnableSnapshotBlockPublicAccessRequest, EnableSnapshotBlockPublicAccessResponse,
        GetEbsEncryptionByDefaultRequest, GetEbsEncryptionByDefaultResponse,
        ModifyImageAttributeRequest, ModifyInstanceAttributeRequest,
        ModifyInstanceMetadataOptionsRequest, ModifyInstanceMetadataOptionsResponse,
        ModifySnapshotAttributeRequest, ModifyVolumeRequest, ModifyVolumeResponse,
        MonitorInstancesRequest, MonitorInstancesResponse, ReleaseAddressRequest,
        RevokeSecurityGroupEgressRequest, RevokeSecurityGroupEgressResponse,
        RevokeSecurityGroupIngressRequest, RevokeSecurityGroupIngressResponse, Snapshot,
        StartInstancesRequest, StartInstancesResponse, StopInstancesRequest, StopInstancesResponse,
        TerminateInstancesRequest, TerminateInstancesResponse, VolumeAttachment,
        VpcPeeringConnection,
    },
};

/// Client for the Amazon EC2 API
pub struct Ec2Client<'a> {
    ops: Ec2Ops<'a>,
}

impl<'a> Ec2Client<'a> {
    /// Create a new Amazon EC2 API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: Ec2Ops::new(client),
        }
    }

    /// Describes the specified instances or all instances.
    pub async fn describe_instances(
        &self,
        body: &DescribeInstancesRequest,
    ) -> Result<DescribeInstancesResponse> {
        self.ops.describe_instances(body).await
    }

    /// Describes the specified EBS volumes or all EBS volumes.
    pub async fn describe_volumes(
        &self,
        body: &DescribeVolumesRequest,
    ) -> Result<DescribeVolumesResponse> {
        self.ops.describe_volumes(body).await
    }

    /// Describes the specified EBS snapshots.
    pub async fn describe_snapshots(
        &self,
        body: &DescribeSnapshotsRequest,
    ) -> Result<DescribeSnapshotsResponse> {
        self.ops.describe_snapshots(body).await
    }

    /// Describes the specified images (AMIs).
    pub async fn describe_images(
        &self,
        body: &DescribeImagesRequest,
    ) -> Result<DescribeImagesResponse> {
        self.ops.describe_images(body).await
    }

    /// Describes the specified security groups.
    pub async fn describe_security_groups(
        &self,
        body: &DescribeSecurityGroupsRequest,
    ) -> Result<DescribeSecurityGroupsResponse> {
        self.ops.describe_security_groups(body).await
    }

    /// Describes the specified Elastic IP addresses.
    pub async fn describe_addresses(
        &self,
        body: &DescribeAddressesRequest,
    ) -> Result<DescribeAddressesResponse> {
        self.ops.describe_addresses(body).await
    }

    /// Describes the specified NAT gateways.
    pub async fn describe_nat_gateways(
        &self,
        body: &DescribeNatGatewaysRequest,
    ) -> Result<DescribeNatGatewaysResponse> {
        self.ops.describe_nat_gateways(body).await
    }

    /// Describes the specified route tables.
    pub async fn describe_route_tables(
        &self,
        body: &DescribeRouteTablesRequest,
    ) -> Result<DescribeRouteTablesResponse> {
        self.ops.describe_route_tables(body).await
    }

    /// Describes the specified network ACLs.
    pub async fn describe_network_acls(
        &self,
        body: &DescribeNetworkAclsRequest,
    ) -> Result<DescribeNetworkAclsResponse> {
        self.ops.describe_network_acls(body).await
    }

    /// Describes the specified flow logs.
    pub async fn describe_flow_logs(
        &self,
        body: &DescribeFlowLogsRequest,
    ) -> Result<DescribeFlowLogsResponse> {
        self.ops.describe_flow_logs(body).await
    }

    /// Describes the specified VPCs.
    pub async fn describe_vpcs(&self, body: &DescribeVpcsRequest) -> Result<DescribeVpcsResponse> {
        self.ops.describe_vpcs(body).await
    }

    /// Describes the specified VPC endpoints.
    pub async fn describe_vpc_endpoints(
        &self,
        body: &DescribeVpcEndpointsRequest,
    ) -> Result<DescribeVpcEndpointsResponse> {
        self.ops.describe_vpc_endpoints(body).await
    }

    // ── VPC Peering ────────────────────────────────────────────────────────

    /// Describes one or more VPC peering connections.
    ///
    /// Optionally filter by connection IDs. Passing an empty slice returns all peering
    /// connections in the current account/region.
    ///
    /// CIS 6.6: used to enumerate peering connections and cross-reference with
    /// `describe_route_tables` to verify that route tables don't enable direct access
    /// between peered VPCs where it shouldn't exist.
    pub async fn describe_vpc_peering_connections(
        &self,
        body: &DescribeVpcPeeringConnectionsRequest,
    ) -> Result<DescribeVpcPeeringConnectionsResponse> {
        self.ops.describe_vpc_peering_connections(body).await
    }

    /// Return all VPC peering connections as a flat `Vec`.
    pub async fn list_vpc_peering_connections(&self) -> Result<Vec<VpcPeeringConnection>> {
        let body = DescribeVpcPeeringConnectionsRequest {
            ..Default::default()
        };
        let resp = self.ops.describe_vpc_peering_connections(&body).await?;
        Ok(resp.vpc_peering_connections)
    }

    /// Describes the specified launch templates.
    pub async fn describe_launch_templates(
        &self,
        body: &DescribeLaunchTemplatesRequest,
    ) -> Result<DescribeLaunchTemplatesResponse> {
        self.ops.describe_launch_templates(body).await
    }

    /// Describes the specified launch template versions.
    pub async fn describe_launch_template_versions(
        &self,
        body: &DescribeLaunchTemplateVersionsRequest,
    ) -> Result<DescribeLaunchTemplateVersionsResponse> {
        self.ops.describe_launch_template_versions(body).await
    }

    /// Describes the specified attribute of the specified snapshot.
    pub async fn describe_snapshot_attribute(
        &self,
        body: &DescribeSnapshotAttributeRequest,
    ) -> Result<DescribeSnapshotAttributeResponse> {
        self.ops.describe_snapshot_attribute(body).await
    }

    /// Describes whether EBS encryption by default is enabled for the account.
    pub async fn get_ebs_encryption_by_default(
        &self,
        body: &GetEbsEncryptionByDefaultRequest,
    ) -> Result<GetEbsEncryptionByDefaultResponse> {
        self.ops.get_ebs_encryption_by_default(body).await
    }

    /// Shuts down the specified instances.
    pub async fn terminate_instances(
        &self,
        body: &TerminateInstancesRequest,
    ) -> Result<TerminateInstancesResponse> {
        self.ops.terminate_instances(body).await
    }

    /// Stops the specified instances.
    pub async fn stop_instances(
        &self,
        body: &StopInstancesRequest,
    ) -> Result<StopInstancesResponse> {
        self.ops.stop_instances(body).await
    }

    /// Starts the specified instances.
    pub async fn start_instances(
        &self,
        body: &StartInstancesRequest,
    ) -> Result<StartInstancesResponse> {
        self.ops.start_instances(body).await
    }

    /// Modifies the specified attribute of the specified instance.
    pub async fn modify_instance_attribute(
        &self,
        body: &ModifyInstanceAttributeRequest,
    ) -> Result<()> {
        self.ops.modify_instance_attribute(body).await
    }

    /// Modify the instance metadata parameters on a running or stopped instance.
    pub async fn modify_instance_metadata_options(
        &self,
        body: &ModifyInstanceMetadataOptionsRequest,
    ) -> Result<ModifyInstanceMetadataOptionsResponse> {
        self.ops.modify_instance_metadata_options(body).await
    }

    /// Enables detailed monitoring for the specified instances.
    pub async fn monitor_instances(
        &self,
        body: &MonitorInstancesRequest,
    ) -> Result<MonitorInstancesResponse> {
        self.ops.monitor_instances(body).await
    }

    /// Associates an IAM instance profile with a running or stopped instance.
    pub async fn associate_iam_instance_profile(
        &self,
        body: &AssociateIamInstanceProfileRequest,
    ) -> Result<AssociateIamInstanceProfileResponse> {
        self.ops.associate_iam_instance_profile(body).await
    }

    /// Detaches an EBS volume from an instance.
    pub async fn detach_volume(&self, body: &DetachVolumeRequest) -> Result<VolumeAttachment> {
        self.ops.detach_volume(body).await
    }

    /// Deletes the specified EBS volume.
    pub async fn delete_volume(&self, body: &DeleteVolumeRequest) -> Result<()> {
        self.ops.delete_volume(body).await
    }

    /// Modifies the size, IOPS, throughput, or type of an EBS volume.
    pub async fn modify_volume(&self, body: &ModifyVolumeRequest) -> Result<ModifyVolumeResponse> {
        self.ops.modify_volume(body).await
    }

    /// Creates a snapshot of an EBS volume.
    pub async fn create_snapshot(&self, body: &CreateSnapshotRequest) -> Result<Snapshot> {
        self.ops.create_snapshot(body).await
    }

    /// Deletes the specified snapshot.
    pub async fn delete_snapshot(&self, body: &DeleteSnapshotRequest) -> Result<()> {
        self.ops.delete_snapshot(body).await
    }

    /// Modifies the specified snapshot attribute.
    pub async fn modify_snapshot_attribute(
        &self,
        body: &ModifySnapshotAttributeRequest,
    ) -> Result<()> {
        self.ops.modify_snapshot_attribute(body).await
    }

    /// Enables the block public access for snapshots setting.
    pub async fn enable_snapshot_block_public_access(
        &self,
        body: &EnableSnapshotBlockPublicAccessRequest,
    ) -> Result<EnableSnapshotBlockPublicAccessResponse> {
        self.ops.enable_snapshot_block_public_access(body).await
    }

    /// Deregisters the specified AMI.
    pub async fn deregister_image(
        &self,
        body: &DeregisterImageRequest,
    ) -> Result<DeregisterImageResponse> {
        self.ops.deregister_image(body).await
    }

    /// Modifies the specified attribute of the specified AMI.
    pub async fn modify_image_attribute(&self, body: &ModifyImageAttributeRequest) -> Result<()> {
        self.ops.modify_image_attribute(body).await
    }

    /// Enables the block public access for AMIs setting.
    pub async fn enable_image_block_public_access(
        &self,
        body: &EnableImageBlockPublicAccessRequest,
    ) -> Result<EnableImageBlockPublicAccessResponse> {
        self.ops.enable_image_block_public_access(body).await
    }

    /// Removes the specified inbound rules from a security group.
    pub async fn revoke_security_group_ingress(
        &self,
        body: &RevokeSecurityGroupIngressRequest,
    ) -> Result<RevokeSecurityGroupIngressResponse> {
        self.ops.revoke_security_group_ingress(body).await
    }

    /// Removes the specified outbound rules from a security group.
    pub async fn revoke_security_group_egress(
        &self,
        body: &RevokeSecurityGroupEgressRequest,
    ) -> Result<RevokeSecurityGroupEgressResponse> {
        self.ops.revoke_security_group_egress(body).await
    }

    /// Adds the specified inbound rules to a security group.
    pub async fn authorize_security_group_ingress(
        &self,
        body: &AuthorizeSecurityGroupIngressRequest,
    ) -> Result<AuthorizeSecurityGroupIngressResponse> {
        self.ops.authorize_security_group_ingress(body).await
    }

    /// Deletes the specified security group.
    pub async fn delete_security_group(
        &self,
        body: &DeleteSecurityGroupRequest,
    ) -> Result<DeleteSecurityGroupResponse> {
        self.ops.delete_security_group(body).await
    }

    /// Releases the specified Elastic IP address.
    pub async fn release_address(&self, body: &ReleaseAddressRequest) -> Result<()> {
        self.ops.release_address(body).await
    }

    /// Deletes the specified NAT gateway.
    pub async fn delete_nat_gateway(
        &self,
        body: &DeleteNatGatewayRequest,
    ) -> Result<DeleteNatGatewayResponse> {
        self.ops.delete_nat_gateway(body).await
    }

    /// Deletes the specified VPC endpoints.
    pub async fn delete_vpc_endpoints(
        &self,
        body: &DeleteVpcEndpointsRequest,
    ) -> Result<DeleteVpcEndpointsResponse> {
        self.ops.delete_vpc_endpoints(body).await
    }

    /// Creates one or more flow logs.
    pub async fn create_flow_logs(
        &self,
        body: &CreateFlowLogsRequest,
    ) -> Result<CreateFlowLogsResponse> {
        self.ops.create_flow_logs(body).await
    }

    /// Adds or overwrites only the specified tags for the specified resources.
    pub async fn create_tags(&self, body: &CreateTagsRequest) -> Result<()> {
        self.ops.create_tags(body).await
    }

    /// Enables EBS encryption by default for the account.
    pub async fn enable_ebs_encryption_by_default(
        &self,
        body: &EnableEbsEncryptionByDefaultRequest,
    ) -> Result<EnableEbsEncryptionByDefaultResponse> {
        self.ops.enable_ebs_encryption_by_default(body).await
    }
}

#[cfg(test)]
mod tests {
    use crate::AwsHttpClient;
    use crate::mock_client::MockClient;
    use crate::test_support::ec2_mock_helpers::Ec2MockHelpers;

    /// EC2 responses don't use <ActionResult> wrapper. Data is directly inside
    /// <ActionResponse> with a <requestId> sibling element.
    fn ec2_response(action: &str, inner: &str) -> Vec<u8> {
        format!(
            "<{action}Response>\
               <requestId>test-request-id</requestId>\
               {inner}\
             </{action}Response>"
        )
        .into_bytes()
    }

    #[tokio::test]
    async fn describe_instances_returns_parsed_reservation() {
        let mut mock = MockClient::new();
        mock.expect_describe_instances()
            .returning_bytes(ec2_response(
                "DescribeInstances",
                "<reservationSet>\
                   <item>\
                     <reservationId>r-0123456789abcdef0</reservationId>\
                   </item>\
                 </reservationSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeInstancesRequest::default();
        let response = client.ec2().describe_instances(&body).await.unwrap();

        assert_eq!(response.reservations.len(), 1);
        assert_eq!(
            response.reservations[0].reservation_id.as_deref(),
            Some("r-0123456789abcdef0")
        );
    }

    #[tokio::test]
    async fn describe_instances_handles_empty_response() {
        let mut mock = MockClient::new();
        mock.expect_describe_instances()
            .returning_bytes(ec2_response("DescribeInstances", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeInstancesRequest::default();
        let response = client.ec2().describe_instances(&body).await.unwrap();
        assert!(response.reservations.is_empty());
    }

    #[tokio::test]
    async fn describe_launch_templates_returns_parsed_templates() {
        let mut mock = MockClient::new();
        mock.expect_describe_launch_templates()
            .returning_bytes(ec2_response(
                "DescribeLaunchTemplates",
                "<launchTemplates>\
                   <item>\
                     <launchTemplateId>lt-0123456789abcdef0</launchTemplateId>\
                     <launchTemplateName>my-template</launchTemplateName>\
                   </item>\
                 </launchTemplates>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeLaunchTemplatesRequest::default();
        let response = client.ec2().describe_launch_templates(&body).await.unwrap();

        assert_eq!(response.launch_templates.len(), 1);
        let lt = &response.launch_templates[0];
        assert_eq!(
            lt.launch_template_id.as_deref(),
            Some("lt-0123456789abcdef0")
        );
        assert_eq!(lt.launch_template_name.as_deref(), Some("my-template"));
    }

    #[tokio::test]
    async fn describe_launch_templates_handles_empty_response() {
        let mut mock = MockClient::new();
        mock.expect_describe_launch_templates()
            .returning_bytes(ec2_response("DescribeLaunchTemplates", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeLaunchTemplatesRequest::default();
        let response = client.ec2().describe_launch_templates(&body).await.unwrap();
        assert!(response.launch_templates.is_empty());
    }

    #[tokio::test]
    async fn describe_launch_template_versions_returns_parsed_versions() {
        let mut mock = MockClient::new();
        mock.expect_describe_launch_template_versions()
            .returning_bytes(ec2_response(
                "DescribeLaunchTemplateVersions",
                "<launchTemplateVersionSet>\
                   <item>\
                     <launchTemplateId>lt-0123456789abcdef0</launchTemplateId>\
                     <launchTemplateName>my-template</launchTemplateName>\
                     <versionNumber>1</versionNumber>\
                     <launchTemplateData>\
                       <imageId>ami-abcdef01</imageId>\
                       <instanceType>t3.micro</instanceType>\
                       <metadataOptions>\
                         <httpTokens>required</httpTokens>\
                       </metadataOptions>\
                     </launchTemplateData>\
                   </item>\
                 </launchTemplateVersionSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeLaunchTemplateVersionsRequest {
            launch_template_id: Some("lt-0123456789abcdef0".to_string()),
            ..Default::default()
        };
        let response = client
            .ec2()
            .describe_launch_template_versions(&body)
            .await
            .unwrap();

        assert_eq!(response.launch_template_versions.len(), 1);
        let ver = &response.launch_template_versions[0];
        assert_eq!(
            ver.launch_template_id.as_deref(),
            Some("lt-0123456789abcdef0")
        );
        assert_eq!(ver.launch_template_name.as_deref(), Some("my-template"));
        assert_eq!(ver.version_number, Some(1));

        let data = ver.launch_template_data.as_ref().unwrap();
        assert_eq!(data.image_id.as_deref(), Some("ami-abcdef01"));
        assert_eq!(data.instance_type.as_deref(), Some("t3.micro"));
        let metadata = data.metadata_options.as_ref().unwrap();
        assert_eq!(metadata.http_tokens.as_deref(), Some("required"));
    }

    #[tokio::test]
    async fn describe_launch_template_versions_handles_empty_response() {
        let mut mock = MockClient::new();
        mock.expect_describe_launch_template_versions()
            .returning_bytes(ec2_response("DescribeLaunchTemplateVersions", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeLaunchTemplateVersionsRequest::default();
        let response = client
            .ec2()
            .describe_launch_template_versions(&body)
            .await
            .unwrap();
        assert!(response.launch_template_versions.is_empty());
    }

    // =========================================================================
    // Task 3.3: DescribeVolumes, DetachVolume, DeleteVolume, ModifyVolume
    // =========================================================================

    #[tokio::test]
    async fn describe_volumes_returns_parsed_volumes() {
        let mut mock = MockClient::new();
        mock.expect_describe_volumes().returning_bytes(ec2_response(
            "DescribeVolumes",
            "<volumeSet>\
               <item>\
                 <volumeId>vol-0123456789abcdef0</volumeId>\
                 <size>100</size>\
                 <volumeType>gp3</volumeType>\
                 <status>available</status>\
                 <encrypted>true</encrypted>\
                 <availabilityZone>eu-central-1a</availabilityZone>\
                 <tagSet>\
                   <item>\
                     <key>Name</key>\
                     <value>test-volume</value>\
                   </item>\
                 </tagSet>\
               </item>\
             </volumeSet>\
             <nextToken>page-2-token</nextToken>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeVolumesRequest::default();
        let response = client.ec2().describe_volumes(&body).await.unwrap();

        assert_eq!(response.volumes.len(), 1);
        let vol = &response.volumes[0];
        assert_eq!(vol.volume_id.as_deref(), Some("vol-0123456789abcdef0"));
        assert_eq!(vol.size, Some(100));
        assert_eq!(vol.volume_type.as_deref(), Some("gp3"));
        assert_eq!(vol.state.as_deref(), Some("available"));
        assert_eq!(vol.encrypted, Some(true));
        assert_eq!(vol.availability_zone.as_deref(), Some("eu-central-1a"));
        assert_eq!(vol.tags.len(), 1);
        assert_eq!(vol.tags[0].key.as_deref(), Some("Name"));
        assert_eq!(vol.tags[0].value.as_deref(), Some("test-volume"));
        assert_eq!(response.next_token.as_deref(), Some("page-2-token"));
    }

    #[tokio::test]
    async fn describe_volumes_handles_empty_response() {
        let mut mock = MockClient::new();
        mock.expect_describe_volumes()
            .returning_bytes(ec2_response("DescribeVolumes", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeVolumesRequest::default();
        let response = client.ec2().describe_volumes(&body).await.unwrap();
        assert!(response.volumes.is_empty());
        assert!(response.next_token.is_none());
    }

    #[tokio::test]
    async fn modify_volume_returns_parsed_modification() {
        let mut mock = MockClient::new();
        mock.expect_modify_volume().returning_bytes(ec2_response(
            "ModifyVolume",
            "<volumeModification>\
               <volumeId>vol-0123456789abcdef0</volumeId>\
               <modificationState>modifying</modificationState>\
               <targetIops>4000</targetIops>\
               <targetVolumeType>gp3</targetVolumeType>\
               <targetThroughput>250</targetThroughput>\
             </volumeModification>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::ModifyVolumeRequest {
            volume_id: "vol-0123456789abcdef0".into(),
            iops: Some(4000),
            ..Default::default()
        };
        let response = client.ec2().modify_volume(&body).await.unwrap();

        let modification = response.volume_modification.unwrap();
        assert_eq!(
            modification.volume_id.as_deref(),
            Some("vol-0123456789abcdef0")
        );
        assert_eq!(
            modification.modification_state.as_deref(),
            Some("modifying")
        );
        assert_eq!(modification.target_iops, Some(4000));
        assert_eq!(modification.target_volume_type.as_deref(), Some("gp3"));
        assert_eq!(modification.target_throughput, Some(250));
    }

    #[tokio::test]
    async fn detach_volume_returns_parsed_attachment() {
        let mut mock = MockClient::new();
        mock.expect_detach_volume().returning_bytes(ec2_response(
            "DetachVolume",
            "<volumeId>vol-0123456789abcdef0</volumeId>\
             <instanceId>i-0123456789abcdef0</instanceId>\
             <device>/dev/sdf</device>\
             <status>detaching</status>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DetachVolumeRequest {
            volume_id: "vol-0123456789abcdef0".into(),
            instance_id: Some("i-0123456789abcdef0".into()),
        };
        let response = client.ec2().detach_volume(&body).await.unwrap();

        assert_eq!(response.volume_id.as_deref(), Some("vol-0123456789abcdef0"));
        assert_eq!(response.instance_id.as_deref(), Some("i-0123456789abcdef0"));
        assert_eq!(response.device.as_deref(), Some("/dev/sdf"));
        assert_eq!(response.state.as_deref(), Some("detaching"));
    }

    #[tokio::test]
    async fn delete_volume_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete_volume()
            .returning_bytes(ec2_response("DeleteVolume", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DeleteVolumeRequest {
            volume_id: "vol-0123456789abcdef0".into(),
        };
        let result = client.ec2().delete_volume(&body).await;
        assert!(result.is_ok());
    }

    // =========================================================================
    // Task 3.4: DescribeSnapshots, CreateSnapshot, DeleteSnapshot,
    //           DescribeSnapshotAttribute, ModifySnapshotAttribute
    // =========================================================================

    #[tokio::test]
    async fn describe_snapshots_returns_parsed_snapshots() {
        let mut mock = MockClient::new();
        mock.expect_describe_snapshots()
            .returning_bytes(ec2_response(
                "DescribeSnapshots",
                "<snapshotSet>\
                   <item>\
                     <snapshotId>snap-0123456789abcdef0</snapshotId>\
                     <volumeId>vol-0123456789abcdef0</volumeId>\
                     <volumeSize>100</volumeSize>\
                     <status>completed</status>\
                     <encrypted>false</encrypted>\
                     <startTime>2026-01-15T10:00:00.000Z</startTime>\
                     <tagSet>\
                       <item>\
                         <key>Name</key>\
                         <value>test-snapshot</value>\
                       </item>\
                     </tagSet>\
                   </item>\
                 </snapshotSet>\
                 <nextToken>page-2-token</nextToken>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeSnapshotsRequest::default();
        let response = client.ec2().describe_snapshots(&body).await.unwrap();

        assert_eq!(response.snapshots.len(), 1);
        let snap = &response.snapshots[0];
        assert_eq!(snap.snapshot_id.as_deref(), Some("snap-0123456789abcdef0"));
        assert_eq!(snap.volume_id.as_deref(), Some("vol-0123456789abcdef0"));
        assert_eq!(snap.volume_size, Some(100));
        assert_eq!(snap.state.as_deref(), Some("completed"));
        assert_eq!(snap.encrypted, Some(false));
        assert_eq!(snap.tags.len(), 1);
        assert_eq!(snap.tags[0].key.as_deref(), Some("Name"));
        assert_eq!(response.next_token.as_deref(), Some("page-2-token"));
    }

    #[tokio::test]
    async fn describe_snapshots_handles_empty_response() {
        let mut mock = MockClient::new();
        mock.expect_describe_snapshots()
            .returning_bytes(ec2_response("DescribeSnapshots", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeSnapshotsRequest::default();
        let response = client.ec2().describe_snapshots(&body).await.unwrap();
        assert!(response.snapshots.is_empty());
    }

    #[tokio::test]
    async fn create_snapshot_returns_parsed_snapshot() {
        let mut mock = MockClient::new();
        mock.expect_create_snapshot().returning_bytes(ec2_response(
            "CreateSnapshot",
            "<snapshotId>snap-0123456789abcdef0</snapshotId>\
                 <volumeId>vol-0123456789abcdef0</volumeId>\
                 <volumeSize>50</volumeSize>\
                 <status>pending</status>\
                 <encrypted>true</encrypted>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::CreateSnapshotRequest {
            volume_id: "vol-0123456789abcdef0".into(),
            description: Some("test snapshot".into()),
        };
        let response = client.ec2().create_snapshot(&body).await.unwrap();

        assert_eq!(
            response.snapshot_id.as_deref(),
            Some("snap-0123456789abcdef0")
        );
        assert_eq!(response.volume_id.as_deref(), Some("vol-0123456789abcdef0"));
        assert_eq!(response.volume_size, Some(50));
        assert_eq!(response.state.as_deref(), Some("pending"));
        assert_eq!(response.encrypted, Some(true));
    }

    #[tokio::test]
    async fn delete_snapshot_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete_snapshot()
            .returning_bytes(ec2_response("DeleteSnapshot", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DeleteSnapshotRequest {
            snapshot_id: "snap-0123456789abcdef0".into(),
        };
        let result = client.ec2().delete_snapshot(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn describe_snapshot_attribute_returns_permissions() {
        let mut mock = MockClient::new();
        mock.expect_describe_snapshot_attribute()
            .returning_bytes(ec2_response(
                "DescribeSnapshotAttribute",
                "<snapshotId>snap-0123456789abcdef0</snapshotId>\
                 <createVolumePermission>\
                   <item>\
                     <userId>123456789012</userId>\
                   </item>\
                 </createVolumePermission>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeSnapshotAttributeRequest {
            snapshot_id: "snap-0123456789abcdef0".into(),
            attribute: "createVolumePermission".into(),
        };
        let response = client
            .ec2()
            .describe_snapshot_attribute(&body)
            .await
            .unwrap();

        assert_eq!(
            response.snapshot_id.as_deref(),
            Some("snap-0123456789abcdef0")
        );
        assert_eq!(response.create_volume_permissions.len(), 1);
        assert_eq!(
            response.create_volume_permissions[0].user_id.as_deref(),
            Some("123456789012")
        );
    }

    #[tokio::test]
    async fn modify_snapshot_attribute_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_modify_snapshot_attribute()
            .returning_bytes(ec2_response("ModifySnapshotAttribute", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::ModifySnapshotAttributeRequest {
            snapshot_id: "snap-0123456789abcdef0".into(),
            attribute: Some("createVolumePermission".into()),
            ..Default::default()
        };
        let result = client.ec2().modify_snapshot_attribute(&body).await;
        assert!(result.is_ok());
    }

    // =========================================================================
    // Task 3.5: DescribeImages, DeregisterImage, ModifyImageAttribute
    // =========================================================================

    #[tokio::test]
    async fn describe_images_returns_parsed_images() {
        let mut mock = MockClient::new();
        mock.expect_describe_images().returning_bytes(ec2_response(
            "DescribeImages",
            "<imagesSet>\
                   <item>\
                     <imageId>ami-0123456789abcdef0</imageId>\
                     <name>test-ami</name>\
                     <imageState>available</imageState>\
                     <isPublic>false</isPublic>\
                     <imageType>machine</imageType>\
                     <platformDetails>Linux/UNIX</platformDetails>\
                     <creationDate>2026-01-15T10:00:00.000Z</creationDate>\
                     <description>A test AMI</description>\
                     <blockDeviceMapping>\
                       <item>\
                         <deviceName>/dev/xvda</deviceName>\
                       </item>\
                     </blockDeviceMapping>\
                     <tagSet>\
                       <item>\
                         <key>Name</key>\
                         <value>test-image</value>\
                       </item>\
                     </tagSet>\
                   </item>\
                 </imagesSet>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeImagesRequest {
            owners: vec!["self".into()],
            ..Default::default()
        };
        let response = client.ec2().describe_images(&body).await.unwrap();

        assert_eq!(response.images.len(), 1);
        let img = &response.images[0];
        assert_eq!(img.image_id.as_deref(), Some("ami-0123456789abcdef0"));
        assert_eq!(img.name.as_deref(), Some("test-ami"));
        assert_eq!(img.state.as_deref(), Some("available"));
        assert_eq!(img.public, Some(false));
        assert_eq!(img.image_type.as_deref(), Some("machine"));
        assert_eq!(img.platform_details.as_deref(), Some("Linux/UNIX"));
        assert_eq!(
            img.creation_date.as_deref(),
            Some("2026-01-15T10:00:00.000Z")
        );
        assert_eq!(img.description.as_deref(), Some("A test AMI"));
        assert_eq!(img.block_device_mappings.len(), 1);
        assert_eq!(
            img.block_device_mappings[0].device_name.as_deref(),
            Some("/dev/xvda")
        );
        assert_eq!(img.tags.len(), 1);
        assert_eq!(img.tags[0].key.as_deref(), Some("Name"));
        assert_eq!(img.tags[0].value.as_deref(), Some("test-image"));
    }

    #[tokio::test]
    async fn describe_images_handles_empty_response() {
        let mut mock = MockClient::new();
        mock.expect_describe_images()
            .returning_bytes(ec2_response("DescribeImages", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeImagesRequest::default();
        let response = client.ec2().describe_images(&body).await.unwrap();
        assert!(response.images.is_empty());
    }

    #[tokio::test]
    async fn deregister_image_returns_parsed_response() {
        let mut mock = MockClient::new();
        mock.expect_deregister_image().returning_bytes(ec2_response(
            "DeregisterImage",
            "<deleteSnapshotResultSet>\
                   <item>\
                     <snapshotId>snap-0123456789abcdef0</snapshotId>\
                     <return>true</return>\
                   </item>\
                 </deleteSnapshotResultSet>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DeregisterImageRequest {
            image_id: "ami-0123456789abcdef0".into(),
        };
        let response = client.ec2().deregister_image(&body).await.unwrap();

        assert_eq!(response.delete_snapshot_results.len(), 1);
    }

    #[tokio::test]
    async fn deregister_image_handles_empty_result() {
        let mut mock = MockClient::new();
        mock.expect_deregister_image()
            .returning_bytes(ec2_response("DeregisterImage", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DeregisterImageRequest {
            image_id: "ami-0123456789abcdef0".into(),
        };
        let response = client.ec2().deregister_image(&body).await.unwrap();
        assert!(response.delete_snapshot_results.is_empty());
    }

    #[tokio::test]
    async fn modify_image_attribute_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_modify_image_attribute()
            .returning_bytes(ec2_response("ModifyImageAttribute", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::ModifyImageAttributeRequest {
            image_id: "ami-0123456789abcdef0".into(),
            ..Default::default()
        };
        let result = client.ec2().modify_image_attribute(&body).await;
        assert!(result.is_ok());
    }

    // =========================================================================
    // Task 3.6: DescribeSecurityGroups, AuthorizeSecurityGroupIngress,
    //           RevokeSecurityGroupIngress, RevokeSecurityGroupEgress,
    //           DeleteSecurityGroup
    // =========================================================================

    #[tokio::test]
    async fn describe_security_groups_returns_parsed_groups() {
        let mut mock = MockClient::new();
        mock.expect_describe_security_groups()
            .returning_bytes(ec2_response(
                "DescribeSecurityGroups",
                "<securityGroupInfo>\
                   <item>\
                     <groupId>sg-0123456789abcdef0</groupId>\
                     <groupName>test-sg</groupName>\
                     <groupDescription>A test security group</groupDescription>\
                     <vpcId>vpc-abc123</vpcId>\
                     <ipPermissions>\
                       <item>\
                         <ipProtocol>tcp</ipProtocol>\
                         <fromPort>22</fromPort>\
                         <toPort>22</toPort>\
                         <ipRanges>\
                           <item>\
                             <cidrIp>10.0.0.0/8</cidrIp>\
                           </item>\
                         </ipRanges>\
                       </item>\
                     </ipPermissions>\
                     <ipPermissionsEgress>\
                       <item>\
                         <ipProtocol>-1</ipProtocol>\
                         <ipRanges>\
                           <item>\
                             <cidrIp>0.0.0.0/0</cidrIp>\
                           </item>\
                         </ipRanges>\
                       </item>\
                     </ipPermissionsEgress>\
                     <tagSet>\
                       <item>\
                         <key>Name</key>\
                         <value>my-sg</value>\
                       </item>\
                     </tagSet>\
                   </item>\
                 </securityGroupInfo>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeSecurityGroupsRequest::default();
        let response = client.ec2().describe_security_groups(&body).await.unwrap();

        assert_eq!(response.security_groups.len(), 1);
        let sg = &response.security_groups[0];
        assert_eq!(sg.group_id.as_deref(), Some("sg-0123456789abcdef0"));
        assert_eq!(sg.group_name.as_deref(), Some("test-sg"));
        assert_eq!(sg.description.as_deref(), Some("A test security group"));
        assert_eq!(sg.vpc_id.as_deref(), Some("vpc-abc123"));
        assert_eq!(sg.ip_permissions.len(), 1);
        let perm = &sg.ip_permissions[0];
        assert_eq!(perm.ip_protocol.as_deref(), Some("tcp"));
        assert_eq!(perm.from_port, Some(22));
        assert_eq!(perm.to_port, Some(22));
        assert_eq!(perm.ip_ranges.len(), 1);
        assert_eq!(perm.ip_ranges[0].cidr_ip.as_deref(), Some("10.0.0.0/8"));
        assert_eq!(sg.ip_permissions_egress.len(), 1);
        assert_eq!(sg.tags.len(), 1);
        assert_eq!(sg.tags[0].key.as_deref(), Some("Name"));
    }

    #[tokio::test]
    async fn describe_security_groups_handles_empty_response() {
        let mut mock = MockClient::new();
        mock.expect_describe_security_groups()
            .returning_bytes(ec2_response("DescribeSecurityGroups", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeSecurityGroupsRequest::default();
        let response = client.ec2().describe_security_groups(&body).await.unwrap();
        assert!(response.security_groups.is_empty());
    }

    #[tokio::test]
    async fn authorize_security_group_ingress_returns_rules() {
        let mut mock = MockClient::new();
        mock.expect_authorize_security_group_ingress()
            .returning_bytes(ec2_response(
                "AuthorizeSecurityGroupIngress",
                "<securityGroupRuleSet>\
                   <item>\
                     <securityGroupRuleId>sgr-0123456789abcdef0</securityGroupRuleId>\
                     <groupId>sg-0123456789abcdef0</groupId>\
                     <ipProtocol>tcp</ipProtocol>\
                     <fromPort>443</fromPort>\
                     <toPort>443</toPort>\
                     <cidrIpv4>0.0.0.0/0</cidrIpv4>\
                   </item>\
                 </securityGroupRuleSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::AuthorizeSecurityGroupIngressRequest {
            group_id: Some("sg-0123456789abcdef0".into()),
            ip_permissions: vec![crate::types::ec2::IpPermission {
                ip_protocol: Some("tcp".into()),
                from_port: Some(443),
                to_port: Some(443),
                ip_ranges: vec![crate::types::ec2::IpRange {
                    cidr_ip: Some("0.0.0.0/0".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        };
        let response = client
            .ec2()
            .authorize_security_group_ingress(&body)
            .await
            .unwrap();

        assert_eq!(response.security_group_rules.len(), 1);
        let rule = &response.security_group_rules[0];
        assert_eq!(
            rule.security_group_rule_id.as_deref(),
            Some("sgr-0123456789abcdef0")
        );
        assert_eq!(rule.group_id.as_deref(), Some("sg-0123456789abcdef0"));
        assert_eq!(rule.ip_protocol.as_deref(), Some("tcp"));
        assert_eq!(rule.from_port, Some(443));
        assert_eq!(rule.to_port, Some(443));
        assert_eq!(rule.cidr_ipv4.as_deref(), Some("0.0.0.0/0"));
    }

    #[tokio::test]
    async fn revoke_security_group_ingress_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_revoke_security_group_ingress()
            .returning_bytes(ec2_response("RevokeSecurityGroupIngress", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::RevokeSecurityGroupIngressRequest {
            group_id: Some("sg-0123456789abcdef0".into()),
            ip_permissions: vec![crate::types::ec2::IpPermission {
                ip_protocol: Some("tcp".into()),
                from_port: Some(22),
                to_port: Some(22),
                ..Default::default()
            }],
        };
        let result = client.ec2().revoke_security_group_ingress(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn revoke_security_group_egress_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_revoke_security_group_egress()
            .returning_bytes(ec2_response("RevokeSecurityGroupEgress", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::RevokeSecurityGroupEgressRequest {
            group_id: "sg-0123456789abcdef0".into(),
            ip_permissions: vec![crate::types::ec2::IpPermission {
                ip_protocol: Some("-1".into()),
                ..Default::default()
            }],
        };
        let result = client.ec2().revoke_security_group_egress(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_security_group_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete_security_group()
            .returning_bytes(ec2_response(
                "DeleteSecurityGroup",
                "<groupId>sg-0123456789abcdef0</groupId>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DeleteSecurityGroupRequest {
            group_id: Some("sg-0123456789abcdef0".into()),
        };
        let response = client.ec2().delete_security_group(&body).await.unwrap();
        assert_eq!(response.group_id.as_deref(), Some("sg-0123456789abcdef0"));
    }

    #[tokio::test]
    async fn describe_addresses_returns_parsed_addresses() {
        let mut mock = MockClient::new();
        mock.expect_describe_addresses()
            .returning_bytes(ec2_response(
                "DescribeAddresses",
                "<addressesSet>\
                   <item>\
                     <allocationId>eipalloc-0123456789abcdef0</allocationId>\
                     <publicIp>203.0.113.25</publicIp>\
                     <domain>vpc</domain>\
                   </item>\
                 </addressesSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeAddressesRequest::default();
        let response = client.ec2().describe_addresses(&body).await.unwrap();

        assert_eq!(response.addresses.len(), 1);
        let addr = &response.addresses[0];
        assert_eq!(
            addr.allocation_id.as_deref(),
            Some("eipalloc-0123456789abcdef0")
        );
        assert_eq!(addr.public_ip.as_deref(), Some("203.0.113.25"));
        assert_eq!(addr.domain.as_deref(), Some("vpc"));
    }

    #[tokio::test]
    async fn describe_addresses_handles_empty_response() {
        let mut mock = MockClient::new();
        mock.expect_describe_addresses()
            .returning_bytes(ec2_response("DescribeAddresses", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeAddressesRequest::default();
        let response = client.ec2().describe_addresses(&body).await.unwrap();
        assert!(response.addresses.is_empty());
    }

    #[tokio::test]
    async fn release_address_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_release_address()
            .returning_bytes(ec2_response("ReleaseAddress", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::ReleaseAddressRequest {
            allocation_id: Some("eipalloc-0123456789abcdef0".into()),
        };
        let result = client.ec2().release_address(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn describe_nat_gateways_returns_parsed_gateways() {
        let mut mock = MockClient::new();
        mock.expect_describe_nat_gateways()
            .returning_bytes(ec2_response(
                "DescribeNatGateways",
                "<natGatewaySet>\
                   <item>\
                     <natGatewayId>nat-0123456789abcdef0</natGatewayId>\
                     <state>available</state>\
                     <vpcId>vpc-0abc123</vpcId>\
                     <subnetId>subnet-0abc123</subnetId>\
                     <natGatewayAddressSet>\
                       <item>\
                         <allocationId>eipalloc-0abc123</allocationId>\
                         <publicIp>198.51.100.1</publicIp>\
                       </item>\
                     </natGatewayAddressSet>\
                   </item>\
                 </natGatewaySet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeNatGatewaysRequest::default();
        let response = client.ec2().describe_nat_gateways(&body).await.unwrap();

        assert_eq!(response.nat_gateways.len(), 1);
        let ngw = &response.nat_gateways[0];
        assert_eq!(ngw.nat_gateway_id.as_deref(), Some("nat-0123456789abcdef0"));
        assert_eq!(ngw.state.as_deref(), Some("available"));
        assert_eq!(ngw.vpc_id.as_deref(), Some("vpc-0abc123"));
        assert_eq!(ngw.nat_gateway_addresses.len(), 1);
        assert_eq!(
            ngw.nat_gateway_addresses[0].public_ip.as_deref(),
            Some("198.51.100.1")
        );
    }

    #[tokio::test]
    async fn delete_nat_gateway_returns_id() {
        let mut mock = MockClient::new();
        mock.expect_delete_nat_gateway()
            .returning_bytes(ec2_response(
                "DeleteNatGateway",
                "<natGatewayId>nat-0123456789abcdef0</natGatewayId>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DeleteNatGatewayRequest {
            nat_gateway_id: "nat-0123456789abcdef0".into(),
        };
        let response = client.ec2().delete_nat_gateway(&body).await.unwrap();
        assert_eq!(
            response.nat_gateway_id.as_deref(),
            Some("nat-0123456789abcdef0")
        );
    }

    #[tokio::test]
    async fn describe_vpc_endpoints_returns_parsed_endpoints() {
        let mut mock = MockClient::new();
        mock.expect_describe_vpc_endpoints()
            .returning_bytes(ec2_response(
                "DescribeVpcEndpoints",
                "<vpcEndpointSet>\
                   <item>\
                     <vpcEndpointId>vpce-0123456789abcdef0</vpcEndpointId>\
                     <vpcId>vpc-0abc123</vpcId>\
                     <serviceName>com.amazonaws.eu-central-1.s3</serviceName>\
                     <state>available</state>\
                   </item>\
                 </vpcEndpointSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeVpcEndpointsRequest::default();
        let response = client.ec2().describe_vpc_endpoints(&body).await.unwrap();

        assert_eq!(response.vpc_endpoints.len(), 1);
        let vpce = &response.vpc_endpoints[0];
        assert_eq!(
            vpce.vpc_endpoint_id.as_deref(),
            Some("vpce-0123456789abcdef0")
        );
        assert_eq!(vpce.vpc_id.as_deref(), Some("vpc-0abc123"));
        assert_eq!(
            vpce.service_name.as_deref(),
            Some("com.amazonaws.eu-central-1.s3")
        );
        assert_eq!(vpce.state.as_deref(), Some("available"));
    }

    #[tokio::test]
    async fn delete_vpc_endpoints_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete_vpc_endpoints()
            .returning_bytes(ec2_response("DeleteVpcEndpoints", "<unsuccessful/>"));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DeleteVpcEndpointsRequest {
            vpc_endpoint_ids: vec!["vpce-0123456789abcdef0".into()],
        };
        let response = client.ec2().delete_vpc_endpoints(&body).await.unwrap();
        assert!(response.unsuccessful.is_empty());
    }

    #[tokio::test]
    async fn describe_vpcs_returns_parsed_vpcs() {
        let mut mock = MockClient::new();
        mock.expect_describe_vpcs().returning_bytes(ec2_response(
            "DescribeVpcs",
            "<vpcSet>\
                   <item>\
                     <vpcId>vpc-0abc123</vpcId>\
                     <cidrBlock>10.0.0.0/16</cidrBlock>\
                     <state>available</state>\
                     <isDefault>true</isDefault>\
                   </item>\
                 </vpcSet>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeVpcsRequest::default();
        let response = client.ec2().describe_vpcs(&body).await.unwrap();

        assert_eq!(response.vpcs.len(), 1);
        let vpc = &response.vpcs[0];
        assert_eq!(vpc.vpc_id.as_deref(), Some("vpc-0abc123"));
        assert_eq!(vpc.cidr_block.as_deref(), Some("10.0.0.0/16"));
        assert_eq!(vpc.state.as_deref(), Some("available"));
        assert_eq!(vpc.is_default, Some(true));
    }

    #[tokio::test]
    async fn describe_route_tables_returns_parsed_tables() {
        let mut mock = MockClient::new();
        mock.expect_describe_route_tables()
            .returning_bytes(ec2_response(
                "DescribeRouteTables",
                "<routeTableSet>\
                   <item>\
                     <routeTableId>rtb-0abc123</routeTableId>\
                     <vpcId>vpc-0abc123</vpcId>\
                     <routeSet>\
                       <item>\
                         <destinationCidrBlock>10.0.0.0/16</destinationCidrBlock>\
                         <gatewayId>local</gatewayId>\
                         <state>active</state>\
                       </item>\
                     </routeSet>\
                   </item>\
                 </routeTableSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeRouteTablesRequest::default();
        let response = client.ec2().describe_route_tables(&body).await.unwrap();

        assert_eq!(response.route_tables.len(), 1);
        let rt = &response.route_tables[0];
        assert_eq!(rt.route_table_id.as_deref(), Some("rtb-0abc123"));
        assert_eq!(rt.vpc_id.as_deref(), Some("vpc-0abc123"));
        assert_eq!(rt.routes.len(), 1);
        assert_eq!(
            rt.routes[0].destination_cidr_block.as_deref(),
            Some("10.0.0.0/16")
        );
        assert_eq!(rt.routes[0].state.as_deref(), Some("active"));
    }

    #[tokio::test]
    async fn describe_network_acls_returns_parsed_acls() {
        let mut mock = MockClient::new();
        mock.expect_describe_network_acls()
            .returning_bytes(ec2_response(
                "DescribeNetworkAcls",
                "<networkAclSet>\
                   <item>\
                     <networkAclId>acl-0abc123</networkAclId>\
                     <vpcId>vpc-0abc123</vpcId>\
                     <default>true</default>\
                     <entrySet>\
                       <item>\
                         <ruleNumber>100</ruleNumber>\
                         <protocol>-1</protocol>\
                         <ruleAction>allow</ruleAction>\
                         <egress>false</egress>\
                         <cidrBlock>0.0.0.0/0</cidrBlock>\
                       </item>\
                     </entrySet>\
                   </item>\
                 </networkAclSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeNetworkAclsRequest::default();
        let response = client.ec2().describe_network_acls(&body).await.unwrap();

        assert_eq!(response.network_acls.len(), 1);
        let acl = &response.network_acls[0];
        assert_eq!(acl.network_acl_id.as_deref(), Some("acl-0abc123"));
        assert_eq!(acl.vpc_id.as_deref(), Some("vpc-0abc123"));
        assert_eq!(acl.is_default, Some(true));
        assert_eq!(acl.entries.len(), 1);
        assert_eq!(acl.entries[0].rule_number, Some(100));
        assert_eq!(acl.entries[0].rule_action.as_deref(), Some("allow"));
    }

    #[tokio::test]
    async fn describe_flow_logs_returns_parsed_logs() {
        let mut mock = MockClient::new();
        mock.expect_describe_flow_logs()
            .returning_bytes(ec2_response(
                "DescribeFlowLogs",
                "<flowLogSet>\
                   <item>\
                     <flowLogId>fl-0abc123</flowLogId>\
                     <resourceId>vpc-0abc123</resourceId>\
                     <trafficType>ALL</trafficType>\
                     <logGroupName>/vpc/flow-logs</logGroupName>\
                     <flowLogStatus>ACTIVE</flowLogStatus>\
                   </item>\
                 </flowLogSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeFlowLogsRequest::default();
        let response = client.ec2().describe_flow_logs(&body).await.unwrap();

        assert_eq!(response.flow_logs.len(), 1);
        let fl = &response.flow_logs[0];
        assert_eq!(fl.flow_log_id.as_deref(), Some("fl-0abc123"));
        assert_eq!(fl.resource_id.as_deref(), Some("vpc-0abc123"));
        assert_eq!(fl.traffic_type.as_deref(), Some("ALL"));
        assert_eq!(fl.log_group_name.as_deref(), Some("/vpc/flow-logs"));
        assert_eq!(fl.flow_log_status.as_deref(), Some("ACTIVE"));
    }

    #[tokio::test]
    async fn create_flow_logs_returns_ids() {
        let mut mock = MockClient::new();
        mock.expect_create_flow_logs().returning_bytes(ec2_response(
            "CreateFlowLogs",
            "<flowLogIdSet>\
                   <item>fl-0abc123</item>\
                 </flowLogIdSet>\
                 <unsuccessful/>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::CreateFlowLogsRequest {
            resource_ids: vec!["vpc-0abc123".into()],
            resource_type: "VPC".into(),
            traffic_type: "ALL".into(),
            log_group_name: Some("/vpc/flow-logs".into()),
            deliver_logs_permission_arn: Some("arn:aws:iam::123456789012:role/flow-logs".into()),
        };
        let response = client.ec2().create_flow_logs(&body).await.unwrap();
        assert_eq!(response.flow_log_ids.len(), 1);
        assert_eq!(response.flow_log_ids[0], "fl-0abc123");
        assert!(response.unsuccessful.is_empty());
    }

    #[tokio::test]
    async fn get_ebs_encryption_by_default_returns_state() {
        let mut mock = MockClient::new();
        mock.expect_get_ebs_encryption_by_default()
            .returning_bytes(ec2_response(
                "GetEbsEncryptionByDefault",
                "<ebsEncryptionByDefault>true</ebsEncryptionByDefault>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::GetEbsEncryptionByDefaultRequest::default();
        let response = client
            .ec2()
            .get_ebs_encryption_by_default(&body)
            .await
            .unwrap();
        assert_eq!(response.ebs_encryption_by_default, Some(true));
    }

    #[tokio::test]
    async fn enable_ebs_encryption_by_default_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_enable_ebs_encryption_by_default()
            .returning_bytes(ec2_response(
                "EnableEbsEncryptionByDefault",
                "<ebsEncryptionByDefault>true</ebsEncryptionByDefault>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::EnableEbsEncryptionByDefaultRequest::default();
        let response = client
            .ec2()
            .enable_ebs_encryption_by_default(&body)
            .await
            .unwrap();
        assert_eq!(response.ebs_encryption_by_default, Some(true));
    }

    #[tokio::test]
    async fn terminate_instances_returns_state_changes() {
        let mut mock = MockClient::new();
        mock.expect_terminate_instances()
            .returning_bytes(ec2_response(
                "TerminateInstances",
                "<instancesSet>\
                   <item>\
                     <instanceId>i-0123456789abcdef0</instanceId>\
                     <currentState><code>32</code><name>shutting-down</name></currentState>\
                     <previousState><code>16</code><name>running</name></previousState>\
                   </item>\
                 </instancesSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::TerminateInstancesRequest {
            instance_ids: vec!["i-0123456789abcdef0".into()],
        };
        let response = client.ec2().terminate_instances(&body).await.unwrap();

        assert_eq!(response.terminating_instances.len(), 1);
        let sc = &response.terminating_instances[0];
        assert_eq!(sc.instance_id.as_deref(), Some("i-0123456789abcdef0"));
        assert_eq!(
            sc.current_state.as_ref().unwrap().name.as_deref(),
            Some("shutting-down")
        );
        assert_eq!(
            sc.previous_state.as_ref().unwrap().name.as_deref(),
            Some("running")
        );
    }

    #[tokio::test]
    async fn stop_instances_returns_state_changes() {
        let mut mock = MockClient::new();
        mock.expect_stop_instances().returning_bytes(ec2_response(
            "StopInstances",
            "<instancesSet>\
                   <item>\
                     <instanceId>i-0123456789abcdef0</instanceId>\
                     <currentState><code>64</code><name>stopping</name></currentState>\
                     <previousState><code>16</code><name>running</name></previousState>\
                   </item>\
                 </instancesSet>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::StopInstancesRequest {
            instance_ids: vec!["i-0123456789abcdef0".into()],
        };
        let response = client.ec2().stop_instances(&body).await.unwrap();

        assert_eq!(response.stopping_instances.len(), 1);
        let sc = &response.stopping_instances[0];
        assert_eq!(sc.instance_id.as_deref(), Some("i-0123456789abcdef0"));
        assert_eq!(
            sc.current_state.as_ref().unwrap().name.as_deref(),
            Some("stopping")
        );
    }

    #[tokio::test]
    async fn start_instances_returns_state_changes() {
        let mut mock = MockClient::new();
        mock.expect_start_instances().returning_bytes(ec2_response(
            "StartInstances",
            "<instancesSet>\
                   <item>\
                     <instanceId>i-0123456789abcdef0</instanceId>\
                     <currentState><code>0</code><name>pending</name></currentState>\
                     <previousState><code>80</code><name>stopped</name></previousState>\
                   </item>\
                 </instancesSet>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::StartInstancesRequest {
            instance_ids: vec!["i-0123456789abcdef0".into()],
        };
        let response = client.ec2().start_instances(&body).await.unwrap();

        assert_eq!(response.starting_instances.len(), 1);
        let sc = &response.starting_instances[0];
        assert_eq!(sc.instance_id.as_deref(), Some("i-0123456789abcdef0"));
        assert_eq!(
            sc.current_state.as_ref().unwrap().name.as_deref(),
            Some("pending")
        );
        assert_eq!(
            sc.previous_state.as_ref().unwrap().name.as_deref(),
            Some("stopped")
        );
    }

    #[tokio::test]
    async fn modify_instance_metadata_options_returns_response() {
        let mut mock = MockClient::new();
        mock.expect_modify_instance_metadata_options()
            .returning_bytes(ec2_response(
                "ModifyInstanceMetadataOptions",
                "<instanceId>i-0123456789abcdef0</instanceId>\
                 <instanceMetadataOptions>\
                   <httpTokens>required</httpTokens>\
                   <httpEndpoint>enabled</httpEndpoint>\
                 </instanceMetadataOptions>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::ModifyInstanceMetadataOptionsRequest {
            instance_id: "i-0123456789abcdef0".into(),
            http_tokens: Some("required".into()),
            http_endpoint: Some("enabled".into()),
        };
        let response = client
            .ec2()
            .modify_instance_metadata_options(&body)
            .await
            .unwrap();

        assert_eq!(response.instance_id.as_deref(), Some("i-0123456789abcdef0"));
        let opts = response.instance_metadata_options.as_ref().unwrap();
        assert_eq!(opts.http_tokens.as_deref(), Some("required"));
        assert_eq!(opts.http_endpoint.as_deref(), Some("enabled"));
    }

    #[tokio::test]
    async fn monitor_instances_returns_monitorings() {
        let mut mock = MockClient::new();
        mock.expect_monitor_instances()
            .returning_bytes(ec2_response(
                "MonitorInstances",
                "<instancesSet>\
                   <item>\
                     <instanceId>i-0123456789abcdef0</instanceId>\
                     <monitoring><state>pending</state></monitoring>\
                   </item>\
                 </instancesSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::MonitorInstancesRequest {
            instance_ids: vec!["i-0123456789abcdef0".into()],
        };
        let response = client.ec2().monitor_instances(&body).await.unwrap();

        assert_eq!(response.instance_monitorings.len(), 1);
        let mon = &response.instance_monitorings[0];
        assert_eq!(mon.instance_id.as_deref(), Some("i-0123456789abcdef0"));
        assert_eq!(
            mon.monitoring.as_ref().unwrap().state.as_deref(),
            Some("pending")
        );
    }

    #[tokio::test]
    async fn associate_iam_instance_profile_returns_association() {
        let mut mock = MockClient::new();
        mock.expect_associate_iam_instance_profile()
            .returning_bytes(ec2_response(
                "AssociateIamInstanceProfile",
                "<iamInstanceProfileAssociation>\
                   <associationId>iip-assoc-0abc123</associationId>\
                   <instanceId>i-0123456789abcdef0</instanceId>\
                   <state>associating</state>\
                 </iamInstanceProfileAssociation>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::AssociateIamInstanceProfileRequest {
            iam_instance_profile: crate::types::ec2::IamInstanceProfileSpecification {
                arn: Some("arn:aws:iam::123456789012:instance-profile/test".into()),
                name: None,
            },
            instance_id: "i-0123456789abcdef0".into(),
        };
        let response = client
            .ec2()
            .associate_iam_instance_profile(&body)
            .await
            .unwrap();

        let assoc = response.iam_instance_profile_association.as_ref().unwrap();
        assert_eq!(assoc.association_id.as_deref(), Some("iip-assoc-0abc123"));
        assert_eq!(assoc.instance_id.as_deref(), Some("i-0123456789abcdef0"));
        assert_eq!(assoc.state.as_deref(), Some("associating"));
    }

    #[tokio::test]
    async fn create_tags_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_create_tags()
            .returning_bytes(ec2_response("CreateTags", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::CreateTagsRequest {
            resources: vec!["i-0123456789abcdef0".into()],
            tags: vec![crate::types::ec2::Tag {
                key: Some("Name".into()),
                value: Some("test".into()),
            }],
        };
        let result = client.ec2().create_tags(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn modify_instance_attribute_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_modify_instance_attribute()
            .returning_bytes(ec2_response("ModifyInstanceAttribute", ""));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::ModifyInstanceAttributeRequest {
            instance_id: "i-0123456789abcdef0".into(),
        };
        let result = client.ec2().modify_instance_attribute(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn enable_snapshot_block_public_access_returns_state() {
        let mut mock = MockClient::new();
        mock.expect_enable_snapshot_block_public_access()
            .returning_bytes(ec2_response(
                "EnableSnapshotBlockPublicAccess",
                "<state>block-all-sharing</state>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::EnableSnapshotBlockPublicAccessRequest {
            state: "block-all-sharing".into(),
        };
        let response = client
            .ec2()
            .enable_snapshot_block_public_access(&body)
            .await
            .unwrap();
        assert_eq!(response.state.as_deref(), Some("block-all-sharing"));
    }

    #[tokio::test]
    async fn enable_image_block_public_access_returns_state() {
        let mut mock = MockClient::new();
        mock.expect_enable_image_block_public_access()
            .returning_bytes(ec2_response(
                "EnableImageBlockPublicAccess",
                "<imageBlockPublicAccessState>block-new-sharing</imageBlockPublicAccessState>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::EnableImageBlockPublicAccessRequest {
            image_block_public_access_state: "block-new-sharing".into(),
        };
        let response = client
            .ec2()
            .enable_image_block_public_access(&body)
            .await
            .unwrap();
        assert_eq!(
            response.image_block_public_access_state.as_deref(),
            Some("block-new-sharing")
        );
    }

    // ── VPC Peering ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn describe_vpc_peering_connections_returns_connections() {
        let mut mock = MockClient::new();
        mock.expect_describe_vpc_peering_connections()
            .returning_bytes(ec2_response(
                "DescribeVpcPeeringConnections",
                "<vpcPeeringConnectionSet>\
                   <item>\
                     <vpcPeeringConnectionId>pcx-0a1b2c3d4e5f67890</vpcPeeringConnectionId>\
                     <status><code>active</code><message>Active</message></status>\
                     <accepterVpcInfo>\
                       <vpcId>vpc-0accepter</vpcId>\
                       <ownerId>111111111111</ownerId>\
                       <cidrBlock>10.1.0.0/16</cidrBlock>\
                       <region>eu-central-1</region>\
                     </accepterVpcInfo>\
                     <requesterVpcInfo>\
                       <vpcId>vpc-0requester</vpcId>\
                       <ownerId>222222222222</ownerId>\
                       <cidrBlock>10.2.0.0/16</cidrBlock>\
                       <region>eu-west-1</region>\
                     </requesterVpcInfo>\
                   </item>\
                 </vpcPeeringConnectionSet>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeVpcPeeringConnectionsRequest::default();
        let response = client
            .ec2()
            .describe_vpc_peering_connections(&body)
            .await
            .unwrap();

        assert_eq!(response.vpc_peering_connections.len(), 1);
        let pcx = &response.vpc_peering_connections[0];
        assert_eq!(
            pcx.vpc_peering_connection_id.as_deref(),
            Some("pcx-0a1b2c3d4e5f67890")
        );
        let status = pcx.status.as_ref().expect("status should be set");
        assert_eq!(status.code.as_deref(), Some("active"));
        let accepter = pcx.accepter_vpc_info.as_ref().expect("accepter_vpc_info");
        assert_eq!(accepter.vpc_id.as_deref(), Some("vpc-0accepter"));
        assert_eq!(accepter.cidr_block.as_deref(), Some("10.1.0.0/16"));
        let requester = pcx.requester_vpc_info.as_ref().expect("requester_vpc_info");
        assert_eq!(requester.vpc_id.as_deref(), Some("vpc-0requester"));
        assert_eq!(requester.cidr_block.as_deref(), Some("10.2.0.0/16"));
    }

    #[tokio::test]
    async fn describe_vpc_peering_connections_handles_empty() {
        let mut mock = MockClient::new();
        mock.expect_describe_vpc_peering_connections()
            .returning_bytes(ec2_response(
                "DescribeVpcPeeringConnections",
                "<vpcPeeringConnectionSet/>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::ec2::DescribeVpcPeeringConnectionsRequest::default();
        let response = client
            .ec2()
            .describe_vpc_peering_connections(&body)
            .await
            .unwrap();

        assert!(response.vpc_peering_connections.is_empty());
    }
}
