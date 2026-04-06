//! MockClient helpers for Amazon EC2 API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon EC2 helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait Ec2MockHelpers {
    /// Helper to expect `describe_instances`: Describes the specified instances or all instances.
    fn expect_describe_instances(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_volumes`: Describes the specified EBS volumes or all EBS volumes.
    fn expect_describe_volumes(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_snapshots`: Describes the specified EBS snapshots.
    fn expect_describe_snapshots(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_images`: Describes the specified images (AMIs).
    fn expect_describe_images(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_security_groups`: Describes the specified security groups.
    fn expect_describe_security_groups(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_addresses`: Describes the specified Elastic IP addresses.
    fn expect_describe_addresses(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_nat_gateways`: Describes the specified NAT gateways.
    fn expect_describe_nat_gateways(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_route_tables`: Describes the specified route tables.
    fn expect_describe_route_tables(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_network_acls`: Describes the specified network ACLs.
    fn expect_describe_network_acls(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_flow_logs`: Describes the specified flow logs.
    fn expect_describe_flow_logs(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_vpcs`: Describes the specified VPCs.
    fn expect_describe_vpcs(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_vpc_endpoints`: Describes the specified VPC endpoints.
    fn expect_describe_vpc_endpoints(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_launch_templates`: Describes the specified launch templates.
    fn expect_describe_launch_templates(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_launch_template_versions`: Describes the specified launch
    /// template versions.
    fn expect_describe_launch_template_versions(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_snapshot_attribute`: Describes the specified attribute of the
    /// specified snapshot.
    fn expect_describe_snapshot_attribute(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_ebs_encryption_by_default`: Describes whether EBS encryption by
    /// default is enabled for the account.
    fn expect_get_ebs_encryption_by_default(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `terminate_instances`: Shuts down the specified instances.
    fn expect_terminate_instances(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `stop_instances`: Stops the specified instances.
    fn expect_stop_instances(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `start_instances`: Starts the specified instances.
    fn expect_start_instances(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `modify_instance_attribute`: Modifies the specified attribute of the
    /// specified instance.
    fn expect_modify_instance_attribute(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `modify_instance_metadata_options`: Modify the instance metadata parameters
    /// on a running or stopped instance.
    fn expect_modify_instance_metadata_options(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `monitor_instances`: Enables detailed monitoring for the specified
    /// instances.
    fn expect_monitor_instances(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `associate_iam_instance_profile`: Associates an IAM instance profile with a
    /// running or stopped instance.
    fn expect_associate_iam_instance_profile(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `detach_volume`: Detaches an EBS volume from an instance.
    fn expect_detach_volume(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_volume`: Deletes the specified EBS volume.
    fn expect_delete_volume(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `modify_volume`: Modifies the size, IOPS, throughput, or type of an EBS
    /// volume.
    fn expect_modify_volume(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_snapshot`: Creates a snapshot of an EBS volume.
    fn expect_create_snapshot(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_snapshot`: Deletes the specified snapshot.
    fn expect_delete_snapshot(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `modify_snapshot_attribute`: Modifies the specified snapshot attribute.
    fn expect_modify_snapshot_attribute(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `enable_snapshot_block_public_access`: Enables the block public access for
    /// snapshots setting.
    fn expect_enable_snapshot_block_public_access(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `deregister_image`: Deregisters the specified AMI.
    fn expect_deregister_image(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `modify_image_attribute`: Modifies the specified attribute of the specified
    /// AMI.
    fn expect_modify_image_attribute(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `enable_image_block_public_access`: Enables the block public access for
    /// AMIs setting.
    fn expect_enable_image_block_public_access(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `revoke_security_group_ingress`: Removes the specified inbound rules from a
    /// security group.
    fn expect_revoke_security_group_ingress(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `revoke_security_group_egress`: Removes the specified outbound rules from a
    /// security group.
    fn expect_revoke_security_group_egress(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `authorize_security_group_ingress`: Adds the specified inbound rules to a
    /// security group.
    fn expect_authorize_security_group_ingress(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_security_group`: Deletes the specified security group.
    fn expect_delete_security_group(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `release_address`: Releases the specified Elastic IP address.
    fn expect_release_address(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_nat_gateway`: Deletes the specified NAT gateway.
    fn expect_delete_nat_gateway(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_vpc_endpoints`: Deletes the specified VPC endpoints.
    fn expect_delete_vpc_endpoints(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_flow_logs`: Creates one or more flow logs.
    fn expect_create_flow_logs(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_tags`: Adds or overwrites only the specified tags for the specified
    /// resources.
    fn expect_create_tags(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `enable_ebs_encryption_by_default`: Enables EBS encryption by default for
    /// the account.
    fn expect_enable_ebs_encryption_by_default(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `describe_vpc_peering_connections`: Describes one or more VPC peering
    /// connections.
    fn expect_describe_vpc_peering_connections(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl Ec2MockHelpers for MockClient {
    /// Helper to expect `describe_instances`: Describes the specified instances or all instances.
    fn expect_describe_instances(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_volumes`: Describes the specified EBS volumes or all EBS volumes.
    fn expect_describe_volumes(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_snapshots`: Describes the specified EBS snapshots.
    fn expect_describe_snapshots(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_images`: Describes the specified images (AMIs).
    fn expect_describe_images(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_security_groups`: Describes the specified security groups.
    fn expect_describe_security_groups(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_addresses`: Describes the specified Elastic IP addresses.
    fn expect_describe_addresses(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_nat_gateways`: Describes the specified NAT gateways.
    fn expect_describe_nat_gateways(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_route_tables`: Describes the specified route tables.
    fn expect_describe_route_tables(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_network_acls`: Describes the specified network ACLs.
    fn expect_describe_network_acls(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_flow_logs`: Describes the specified flow logs.
    fn expect_describe_flow_logs(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_vpcs`: Describes the specified VPCs.
    fn expect_describe_vpcs(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_vpc_endpoints`: Describes the specified VPC endpoints.
    fn expect_describe_vpc_endpoints(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_launch_templates`: Describes the specified launch templates.
    fn expect_describe_launch_templates(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_launch_template_versions`: Describes the specified launch
    /// template versions.
    fn expect_describe_launch_template_versions(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_snapshot_attribute`: Describes the specified attribute of the
    /// specified snapshot.
    fn expect_describe_snapshot_attribute(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_ebs_encryption_by_default`: Describes whether EBS encryption by
    /// default is enabled for the account.
    fn expect_get_ebs_encryption_by_default(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `terminate_instances`: Shuts down the specified instances.
    fn expect_terminate_instances(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `stop_instances`: Stops the specified instances.
    fn expect_stop_instances(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `start_instances`: Starts the specified instances.
    fn expect_start_instances(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `modify_instance_attribute`: Modifies the specified attribute of the
    /// specified instance.
    fn expect_modify_instance_attribute(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `modify_instance_metadata_options`: Modify the instance metadata parameters
    /// on a running or stopped instance.
    fn expect_modify_instance_metadata_options(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `monitor_instances`: Enables detailed monitoring for the specified
    /// instances.
    fn expect_monitor_instances(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `associate_iam_instance_profile`: Associates an IAM instance profile with a
    /// running or stopped instance.
    fn expect_associate_iam_instance_profile(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `detach_volume`: Detaches an EBS volume from an instance.
    fn expect_detach_volume(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_volume`: Deletes the specified EBS volume.
    fn expect_delete_volume(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `modify_volume`: Modifies the size, IOPS, throughput, or type of an EBS
    /// volume.
    fn expect_modify_volume(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `create_snapshot`: Creates a snapshot of an EBS volume.
    fn expect_create_snapshot(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_snapshot`: Deletes the specified snapshot.
    fn expect_delete_snapshot(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `modify_snapshot_attribute`: Modifies the specified snapshot attribute.
    fn expect_modify_snapshot_attribute(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `enable_snapshot_block_public_access`: Enables the block public access for
    /// snapshots setting.
    fn expect_enable_snapshot_block_public_access(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `deregister_image`: Deregisters the specified AMI.
    fn expect_deregister_image(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `modify_image_attribute`: Modifies the specified attribute of the specified
    /// AMI.
    fn expect_modify_image_attribute(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `enable_image_block_public_access`: Enables the block public access for
    /// AMIs setting.
    fn expect_enable_image_block_public_access(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `revoke_security_group_ingress`: Removes the specified inbound rules from a
    /// security group.
    fn expect_revoke_security_group_ingress(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `revoke_security_group_egress`: Removes the specified outbound rules from a
    /// security group.
    fn expect_revoke_security_group_egress(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `authorize_security_group_ingress`: Adds the specified inbound rules to a
    /// security group.
    fn expect_authorize_security_group_ingress(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_security_group`: Deletes the specified security group.
    fn expect_delete_security_group(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `release_address`: Releases the specified Elastic IP address.
    fn expect_release_address(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_nat_gateway`: Deletes the specified NAT gateway.
    fn expect_delete_nat_gateway(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_vpc_endpoints`: Deletes the specified VPC endpoints.
    fn expect_delete_vpc_endpoints(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `create_flow_logs`: Creates one or more flow logs.
    fn expect_create_flow_logs(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `create_tags`: Adds or overwrites only the specified tags for the specified
    /// resources.
    fn expect_create_tags(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `enable_ebs_encryption_by_default`: Enables EBS encryption by default for
    /// the account.
    fn expect_enable_ebs_encryption_by_default(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `describe_vpc_peering_connections`: Describes one or more VPC peering
    /// connections.
    fn expect_describe_vpc_peering_connections(
        &mut self,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
