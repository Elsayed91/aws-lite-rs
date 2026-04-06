//! Operation contracts for the Amazon EC2 API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/ec2.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::ec2::*;
use crate::{AwsHttpClient, Result};

/// Raw HTTP operations for the Amazon EC2 API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the AWS Botocore Model.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::ec2::Ec2Client`] instead.
pub struct Ec2Ops<'a> {
    pub(crate) client: &'a AwsHttpClient,
}

impl<'a> Ec2Ops<'a> {
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
        "https://ec2.{region}.amazonaws.com".replace("{region}", self.client.region())
    }

    /// Describes the specified instances or all instances.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeInstancesRequest`]
    ///
    /// # Response
    /// [`DescribeInstancesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_instances(
        &self,
        body: &DescribeInstancesRequest,
    ) -> Result<DescribeInstancesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeInstances", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_instances response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_instances response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeInstancesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_instances XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified EBS volumes or all EBS volumes.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeVolumesRequest`]
    ///
    /// # Response
    /// [`DescribeVolumesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_volumes(
        &self,
        body: &DescribeVolumesRequest,
    ) -> Result<DescribeVolumesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeVolumes", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_volumes response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_volumes response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeVolumesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_volumes XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified EBS snapshots.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeSnapshotsRequest`]
    ///
    /// # Response
    /// [`DescribeSnapshotsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_snapshots(
        &self,
        body: &DescribeSnapshotsRequest,
    ) -> Result<DescribeSnapshotsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeSnapshots", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_snapshots response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_snapshots response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeSnapshotsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_snapshots XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified images (AMIs).
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeImagesRequest`]
    ///
    /// # Response
    /// [`DescribeImagesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_images(
        &self,
        body: &DescribeImagesRequest,
    ) -> Result<DescribeImagesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeImages", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_images response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_images response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeImagesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_images XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified security groups.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeSecurityGroupsRequest`]
    ///
    /// # Response
    /// [`DescribeSecurityGroupsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_security_groups(
        &self,
        body: &DescribeSecurityGroupsRequest,
    ) -> Result<DescribeSecurityGroupsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeSecurityGroups", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_security_groups response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_security_groups response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeSecurityGroupsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_security_groups XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified Elastic IP addresses.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeAddressesRequest`]
    ///
    /// # Response
    /// [`DescribeAddressesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_addresses(
        &self,
        body: &DescribeAddressesRequest,
    ) -> Result<DescribeAddressesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeAddresses", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_addresses response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_addresses response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeAddressesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_addresses XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified NAT gateways.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeNatGatewaysRequest`]
    ///
    /// # Response
    /// [`DescribeNatGatewaysResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_nat_gateways(
        &self,
        body: &DescribeNatGatewaysRequest,
    ) -> Result<DescribeNatGatewaysResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeNatGateways", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_nat_gateways response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_nat_gateways response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeNatGatewaysResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_nat_gateways XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified route tables.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeRouteTablesRequest`]
    ///
    /// # Response
    /// [`DescribeRouteTablesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_route_tables(
        &self,
        body: &DescribeRouteTablesRequest,
    ) -> Result<DescribeRouteTablesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeRouteTables", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_route_tables response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_route_tables response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeRouteTablesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_route_tables XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified network ACLs.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeNetworkAclsRequest`]
    ///
    /// # Response
    /// [`DescribeNetworkAclsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_network_acls(
        &self,
        body: &DescribeNetworkAclsRequest,
    ) -> Result<DescribeNetworkAclsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeNetworkAcls", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_network_acls response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_network_acls response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeNetworkAclsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_network_acls XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified flow logs.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeFlowLogsRequest`]
    ///
    /// # Response
    /// [`DescribeFlowLogsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_flow_logs(
        &self,
        body: &DescribeFlowLogsRequest,
    ) -> Result<DescribeFlowLogsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeFlowLogs", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_flow_logs response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_flow_logs response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeFlowLogsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_flow_logs XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified VPCs.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeVpcsRequest`]
    ///
    /// # Response
    /// [`DescribeVpcsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_vpcs(
        &self,
        body: &DescribeVpcsRequest,
    ) -> Result<DescribeVpcsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body("DescribeVpcs", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_vpcs response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_vpcs response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeVpcsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_vpcs XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified VPC endpoints.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeVpcEndpointsRequest`]
    ///
    /// # Response
    /// [`DescribeVpcEndpointsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_vpc_endpoints(
        &self,
        body: &DescribeVpcEndpointsRequest,
    ) -> Result<DescribeVpcEndpointsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeVpcEndpoints", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_vpc_endpoints response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_vpc_endpoints response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeVpcEndpointsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_vpc_endpoints XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified launch templates.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeLaunchTemplatesRequest`]
    ///
    /// # Response
    /// [`DescribeLaunchTemplatesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_launch_templates(
        &self,
        body: &DescribeLaunchTemplatesRequest,
    ) -> Result<DescribeLaunchTemplatesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DescribeLaunchTemplates", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_launch_templates response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_launch_templates response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeLaunchTemplatesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_launch_templates XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Describes the specified launch template versions.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeLaunchTemplateVersionsRequest`]
    ///
    /// # Response
    /// [`DescribeLaunchTemplateVersionsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_launch_template_versions(
        &self,
        body: &DescribeLaunchTemplateVersionsRequest,
    ) -> Result<DescribeLaunchTemplateVersionsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "DescribeLaunchTemplateVersions",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read describe_launch_template_versions response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Invalid UTF-8 in describe_launch_template_versions response: {e}"
                ),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeLaunchTemplateVersionsResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse describe_launch_template_versions XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Describes the specified attribute of the specified snapshot.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeSnapshotAttributeRequest`]
    ///
    /// # Response
    /// [`DescribeSnapshotAttributeResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_snapshot_attribute(
        &self,
        body: &DescribeSnapshotAttributeRequest,
    ) -> Result<DescribeSnapshotAttributeResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "DescribeSnapshotAttribute",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read describe_snapshot_attribute response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_snapshot_attribute response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeSnapshotAttributeResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse describe_snapshot_attribute XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Describes whether EBS encryption by default is enabled for the account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`GetEbsEncryptionByDefaultRequest`]
    ///
    /// # Response
    /// [`GetEbsEncryptionByDefaultResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_ebs_encryption_by_default(
        &self,
        body: &GetEbsEncryptionByDefaultRequest,
    ) -> Result<GetEbsEncryptionByDefaultResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "GetEbsEncryptionByDefault",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read get_ebs_encryption_by_default response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in get_ebs_encryption_by_default response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<GetEbsEncryptionByDefaultResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse get_ebs_encryption_by_default XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Shuts down the specified instances.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`TerminateInstancesRequest`]
    ///
    /// # Response
    /// [`TerminateInstancesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn terminate_instances(
        &self,
        body: &TerminateInstancesRequest,
    ) -> Result<TerminateInstancesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("TerminateInstances", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read terminate_instances response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in terminate_instances response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<TerminateInstancesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse terminate_instances XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Stops the specified instances.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`StopInstancesRequest`]
    ///
    /// # Response
    /// [`StopInstancesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn stop_instances(
        &self,
        body: &StopInstancesRequest,
    ) -> Result<StopInstancesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("StopInstances", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read stop_instances response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in stop_instances response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<StopInstancesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse stop_instances XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Starts the specified instances.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`StartInstancesRequest`]
    ///
    /// # Response
    /// [`StartInstancesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn start_instances(
        &self,
        body: &StartInstancesRequest,
    ) -> Result<StartInstancesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("StartInstances", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read start_instances response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in start_instances response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<StartInstancesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse start_instances XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Modifies the specified attribute of the specified instance.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ModifyInstanceAttributeRequest`]
    #[allow(dead_code)]
    pub(crate) async fn modify_instance_attribute(
        &self,
        body: &ModifyInstanceAttributeRequest,
    ) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("ModifyInstanceAttribute", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Modify the instance metadata parameters on a running or stopped instance.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ModifyInstanceMetadataOptionsRequest`]
    ///
    /// # Response
    /// [`ModifyInstanceMetadataOptionsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn modify_instance_metadata_options(
        &self,
        body: &ModifyInstanceMetadataOptionsRequest,
    ) -> Result<ModifyInstanceMetadataOptionsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "ModifyInstanceMetadataOptions",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read modify_instance_metadata_options response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in modify_instance_metadata_options response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ModifyInstanceMetadataOptionsResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse modify_instance_metadata_options XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Enables detailed monitoring for the specified instances.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`MonitorInstancesRequest`]
    ///
    /// # Response
    /// [`MonitorInstancesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn monitor_instances(
        &self,
        body: &MonitorInstancesRequest,
    ) -> Result<MonitorInstancesResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("MonitorInstances", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read monitor_instances response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in monitor_instances response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<MonitorInstancesResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse monitor_instances XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Associates an IAM instance profile with a running or stopped instance.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`AssociateIamInstanceProfileRequest`]
    ///
    /// # Response
    /// [`AssociateIamInstanceProfileResponse`]
    #[allow(dead_code)]
    pub(crate) async fn associate_iam_instance_profile(
        &self,
        body: &AssociateIamInstanceProfileRequest,
    ) -> Result<AssociateIamInstanceProfileResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "AssociateIamInstanceProfile",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read associate_iam_instance_profile response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in associate_iam_instance_profile response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<AssociateIamInstanceProfileResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse associate_iam_instance_profile XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Detaches an EBS volume from an instance.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DetachVolumeRequest`]
    ///
    /// # Response
    /// [`VolumeAttachment`]
    #[allow(dead_code)]
    pub(crate) async fn detach_volume(
        &self,
        body: &DetachVolumeRequest,
    ) -> Result<VolumeAttachment> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body("DetachVolume", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read detach_volume response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in detach_volume response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<VolumeAttachment>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse detach_volume XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Deletes the specified EBS volume.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteVolumeRequest`]
    #[allow(dead_code)]
    pub(crate) async fn delete_volume(&self, body: &DeleteVolumeRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body("DeleteVolume", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Modifies the size, IOPS, throughput, or type of an EBS volume.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ModifyVolumeRequest`]
    ///
    /// # Response
    /// [`ModifyVolumeResponse`]
    #[allow(dead_code)]
    pub(crate) async fn modify_volume(
        &self,
        body: &ModifyVolumeRequest,
    ) -> Result<ModifyVolumeResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body("ModifyVolume", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read modify_volume response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in modify_volume response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<ModifyVolumeResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse modify_volume XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Creates a snapshot of an EBS volume.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`CreateSnapshotRequest`]
    ///
    /// # Response
    /// [`Snapshot`]
    #[allow(dead_code)]
    pub(crate) async fn create_snapshot(&self, body: &CreateSnapshotRequest) -> Result<Snapshot> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("CreateSnapshot", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read create_snapshot response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in create_snapshot response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<Snapshot>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse create_snapshot XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Deletes the specified snapshot.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteSnapshotRequest`]
    #[allow(dead_code)]
    pub(crate) async fn delete_snapshot(&self, body: &DeleteSnapshotRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DeleteSnapshot", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Modifies the specified snapshot attribute.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ModifySnapshotAttributeRequest`]
    #[allow(dead_code)]
    pub(crate) async fn modify_snapshot_attribute(
        &self,
        body: &ModifySnapshotAttributeRequest,
    ) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("ModifySnapshotAttribute", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Enables the block public access for snapshots setting.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`EnableSnapshotBlockPublicAccessRequest`]
    ///
    /// # Response
    /// [`EnableSnapshotBlockPublicAccessResponse`]
    #[allow(dead_code)]
    pub(crate) async fn enable_snapshot_block_public_access(
        &self,
        body: &EnableSnapshotBlockPublicAccessRequest,
    ) -> Result<EnableSnapshotBlockPublicAccessResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "EnableSnapshotBlockPublicAccess",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read enable_snapshot_block_public_access response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Invalid UTF-8 in enable_snapshot_block_public_access response: {e}"
                ),
                body: None,
            })?;
        crate::xml::parse_xml_response::<EnableSnapshotBlockPublicAccessResponse>(body_text)
            .map_err(|e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse enable_snapshot_block_public_access XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            })
    }

    /// Deregisters the specified AMI.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeregisterImageRequest`]
    ///
    /// # Response
    /// [`DeregisterImageResponse`]
    #[allow(dead_code)]
    pub(crate) async fn deregister_image(
        &self,
        body: &DeregisterImageRequest,
    ) -> Result<DeregisterImageResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DeregisterImage", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read deregister_image response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in deregister_image response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeregisterImageResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse deregister_image XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Modifies the specified attribute of the specified AMI.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ModifyImageAttributeRequest`]
    #[allow(dead_code)]
    pub(crate) async fn modify_image_attribute(
        &self,
        body: &ModifyImageAttributeRequest,
    ) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("ModifyImageAttribute", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Enables the block public access for AMIs setting.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`EnableImageBlockPublicAccessRequest`]
    ///
    /// # Response
    /// [`EnableImageBlockPublicAccessResponse`]
    #[allow(dead_code)]
    pub(crate) async fn enable_image_block_public_access(
        &self,
        body: &EnableImageBlockPublicAccessRequest,
    ) -> Result<EnableImageBlockPublicAccessResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "EnableImageBlockPublicAccess",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read enable_image_block_public_access response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in enable_image_block_public_access response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<EnableImageBlockPublicAccessResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse enable_image_block_public_access XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Removes the specified inbound rules from a security group.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`RevokeSecurityGroupIngressRequest`]
    ///
    /// # Response
    /// [`RevokeSecurityGroupIngressResponse`]
    #[allow(dead_code)]
    pub(crate) async fn revoke_security_group_ingress(
        &self,
        body: &RevokeSecurityGroupIngressRequest,
    ) -> Result<RevokeSecurityGroupIngressResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "RevokeSecurityGroupIngress",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read revoke_security_group_ingress response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in revoke_security_group_ingress response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<RevokeSecurityGroupIngressResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse revoke_security_group_ingress XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Removes the specified outbound rules from a security group.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`RevokeSecurityGroupEgressRequest`]
    ///
    /// # Response
    /// [`RevokeSecurityGroupEgressResponse`]
    #[allow(dead_code)]
    pub(crate) async fn revoke_security_group_egress(
        &self,
        body: &RevokeSecurityGroupEgressRequest,
    ) -> Result<RevokeSecurityGroupEgressResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "RevokeSecurityGroupEgress",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read revoke_security_group_egress response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in revoke_security_group_egress response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<RevokeSecurityGroupEgressResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse revoke_security_group_egress XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Adds the specified inbound rules to a security group.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`AuthorizeSecurityGroupIngressRequest`]
    ///
    /// # Response
    /// [`AuthorizeSecurityGroupIngressResponse`]
    #[allow(dead_code)]
    pub(crate) async fn authorize_security_group_ingress(
        &self,
        body: &AuthorizeSecurityGroupIngressRequest,
    ) -> Result<AuthorizeSecurityGroupIngressResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "AuthorizeSecurityGroupIngress",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read authorize_security_group_ingress response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in authorize_security_group_ingress response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<AuthorizeSecurityGroupIngressResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse authorize_security_group_ingress XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Deletes the specified security group.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteSecurityGroupRequest`]
    ///
    /// # Response
    /// [`DeleteSecurityGroupResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_security_group(
        &self,
        body: &DeleteSecurityGroupRequest,
    ) -> Result<DeleteSecurityGroupResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DeleteSecurityGroup", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read delete_security_group response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in delete_security_group response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeleteSecurityGroupResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse delete_security_group XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Releases the specified Elastic IP address.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`ReleaseAddressRequest`]
    #[allow(dead_code)]
    pub(crate) async fn release_address(&self, body: &ReleaseAddressRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("ReleaseAddress", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Deletes the specified NAT gateway.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteNatGatewayRequest`]
    ///
    /// # Response
    /// [`DeleteNatGatewayResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_nat_gateway(
        &self,
        body: &DeleteNatGatewayRequest,
    ) -> Result<DeleteNatGatewayResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DeleteNatGateway", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read delete_nat_gateway response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in delete_nat_gateway response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeleteNatGatewayResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse delete_nat_gateway XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Deletes the specified VPC endpoints.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DeleteVpcEndpointsRequest`]
    ///
    /// # Response
    /// [`DeleteVpcEndpointsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_vpc_endpoints(
        &self,
        body: &DeleteVpcEndpointsRequest,
    ) -> Result<DeleteVpcEndpointsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("DeleteVpcEndpoints", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read delete_vpc_endpoints response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in delete_vpc_endpoints response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DeleteVpcEndpointsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse delete_vpc_endpoints XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Creates one or more flow logs.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`CreateFlowLogsRequest`]
    ///
    /// # Response
    /// [`CreateFlowLogsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_flow_logs(
        &self,
        body: &CreateFlowLogsRequest,
    ) -> Result<CreateFlowLogsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str =
            crate::query::build_ec2_query_body("CreateFlowLogs", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read create_flow_logs response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in create_flow_logs response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<CreateFlowLogsResponse>(body_text).map_err(|e| {
            crate::AwsError::InvalidResponse {
                message: format!("Failed to parse create_flow_logs XML response: {e}"),
                body: Some(body_text.to_string()),
            }
        })
    }

    /// Adds or overwrites only the specified tags for the specified resources.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`CreateTagsRequest`]
    #[allow(dead_code)]
    pub(crate) async fn create_tags(&self, body: &CreateTagsRequest) -> Result<()> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body("CreateTags", "2016-11-15", Some(body));
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        response.error_for_status("xml").await?;
        Ok(())
    }

    /// Enables EBS encryption by default for the account.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`EnableEbsEncryptionByDefaultRequest`]
    ///
    /// # Response
    /// [`EnableEbsEncryptionByDefaultResponse`]
    #[allow(dead_code)]
    pub(crate) async fn enable_ebs_encryption_by_default(
        &self,
        body: &EnableEbsEncryptionByDefaultRequest,
    ) -> Result<EnableEbsEncryptionByDefaultResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "EnableEbsEncryptionByDefault",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read enable_ebs_encryption_by_default response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in enable_ebs_encryption_by_default response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<EnableEbsEncryptionByDefaultResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse enable_ebs_encryption_by_default XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }

    /// Describes one or more VPC peering connections.
    ///
    /// **AWS API**: `POST /`
    ///
    /// # Request Body
    /// [`DescribeVpcPeeringConnectionsRequest`]
    ///
    /// # Response
    /// [`DescribeVpcPeeringConnectionsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn describe_vpc_peering_connections(
        &self,
        body: &DescribeVpcPeeringConnectionsRequest,
    ) -> Result<DescribeVpcPeeringConnectionsResponse> {
        let url = format!("{}/", self.base_url(),);
        let body_str = crate::query::build_ec2_query_body(
            "DescribeVpcPeeringConnections",
            "2016-11-15",
            Some(body),
        );
        let response = self
            .client
            .post(
                &url,
                "ec2",
                body_str.as_bytes(),
                "application/x-www-form-urlencoded",
            )
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!(
                        "Failed to read describe_vpc_peering_connections response: {e}"
                    ),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in describe_vpc_peering_connections response: {e}"),
                body: None,
            })?;
        crate::xml::parse_xml_response::<DescribeVpcPeeringConnectionsResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!(
                    "Failed to parse describe_vpc_peering_connections XML response: {e}"
                ),
                body: Some(body_text.to_string()),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeInstancesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeInstancesRequest::fixture();
        let result = ops.describe_instances(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_volumes() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeVolumesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeVolumesRequest::fixture();
        let result = ops.describe_volumes(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_snapshots() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeSnapshotsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeSnapshotsRequest::fixture();
        let result = ops.describe_snapshots(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_images() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeImagesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeImagesRequest::fixture();
        let result = ops.describe_images(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_security_groups() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeSecurityGroupsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeSecurityGroupsRequest::fixture();
        let result = ops.describe_security_groups(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_addresses() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeAddressesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeAddressesRequest::fixture();
        let result = ops.describe_addresses(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_nat_gateways() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeNatGatewaysResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeNatGatewaysRequest::fixture();
        let result = ops.describe_nat_gateways(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_route_tables() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeRouteTablesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeRouteTablesRequest::fixture();
        let result = ops.describe_route_tables(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_network_acls() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeNetworkAclsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeNetworkAclsRequest::fixture();
        let result = ops.describe_network_acls(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_flow_logs() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeFlowLogsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeFlowLogsRequest::fixture();
        let result = ops.describe_flow_logs(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_vpcs() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeVpcsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeVpcsRequest::fixture();
        let result = ops.describe_vpcs(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_vpc_endpoints() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeVpcEndpointsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeVpcEndpointsRequest::fixture();
        let result = ops.describe_vpc_endpoints(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_launch_templates() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeLaunchTemplatesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeLaunchTemplatesRequest::fixture();
        let result = ops.describe_launch_templates(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_launch_template_versions() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeLaunchTemplateVersionsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeLaunchTemplateVersionsRequest::fixture();
        let result = ops.describe_launch_template_versions(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_snapshot_attribute() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeSnapshotAttributeResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeSnapshotAttributeRequest::fixture();
        let result = ops.describe_snapshot_attribute(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_ebs_encryption_by_default() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = GetEbsEncryptionByDefaultResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = GetEbsEncryptionByDefaultRequest::fixture();
        let result = ops.get_ebs_encryption_by_default(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_terminate_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = TerminateInstancesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = TerminateInstancesRequest::fixture();
        let result = ops.terminate_instances(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stop_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = StopInstancesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = StopInstancesRequest::fixture();
        let result = ops.stop_instances(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = StartInstancesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = StartInstancesRequest::fixture();
        let result = ops.start_instances(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modify_instance_attribute() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = ModifyInstanceAttributeRequest::fixture();
        let result = ops.modify_instance_attribute(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modify_instance_metadata_options() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ModifyInstanceMetadataOptionsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = ModifyInstanceMetadataOptionsRequest::fixture();
        let result = ops.modify_instance_metadata_options(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_monitor_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = MonitorInstancesResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = MonitorInstancesRequest::fixture();
        let result = ops.monitor_instances(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_associate_iam_instance_profile() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = AssociateIamInstanceProfileResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = AssociateIamInstanceProfileRequest::fixture();
        let result = ops.associate_iam_instance_profile(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_detach_volume() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = VolumeAttachment::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DetachVolumeRequest::fixture();
        let result = ops.detach_volume(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_volume() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DeleteVolumeRequest::fixture();
        let result = ops.delete_volume(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modify_volume() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = ModifyVolumeResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = ModifyVolumeRequest::fixture();
        let result = ops.modify_volume(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_snapshot() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = Snapshot::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = CreateSnapshotRequest::fixture();
        let result = ops.create_snapshot(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_snapshot() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DeleteSnapshotRequest::fixture();
        let result = ops.delete_snapshot(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modify_snapshot_attribute() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = ModifySnapshotAttributeRequest::fixture();
        let result = ops.modify_snapshot_attribute(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enable_snapshot_block_public_access() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = EnableSnapshotBlockPublicAccessResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = EnableSnapshotBlockPublicAccessRequest::fixture();
        let result = ops.enable_snapshot_block_public_access(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_deregister_image() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeregisterImageResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DeregisterImageRequest::fixture();
        let result = ops.deregister_image(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modify_image_attribute() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = ModifyImageAttributeRequest::fixture();
        let result = ops.modify_image_attribute(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enable_image_block_public_access() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = EnableImageBlockPublicAccessResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = EnableImageBlockPublicAccessRequest::fixture();
        let result = ops.enable_image_block_public_access(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_revoke_security_group_ingress() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = RevokeSecurityGroupIngressResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = RevokeSecurityGroupIngressRequest::fixture();
        let result = ops.revoke_security_group_ingress(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_revoke_security_group_egress() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = RevokeSecurityGroupEgressResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = RevokeSecurityGroupEgressRequest::fixture();
        let result = ops.revoke_security_group_egress(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_security_group_ingress() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = AuthorizeSecurityGroupIngressResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = AuthorizeSecurityGroupIngressRequest::fixture();
        let result = ops.authorize_security_group_ingress(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_security_group() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeleteSecurityGroupResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DeleteSecurityGroupRequest::fixture();
        let result = ops.delete_security_group(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_release_address() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = ReleaseAddressRequest::fixture();
        let result = ops.release_address(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_nat_gateway() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeleteNatGatewayResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DeleteNatGatewayRequest::fixture();
        let result = ops.delete_nat_gateway(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_vpc_endpoints() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DeleteVpcEndpointsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DeleteVpcEndpointsRequest::fixture();
        let result = ops.delete_vpc_endpoints(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_flow_logs() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = CreateFlowLogsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = CreateFlowLogsRequest::fixture();
        let result = ops.create_flow_logs(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_tags() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_json(serde_json::json!({}));

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = CreateTagsRequest::fixture();
        let result = ops.create_tags(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enable_ebs_encryption_by_default() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = EnableEbsEncryptionByDefaultResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = EnableEbsEncryptionByDefaultRequest::fixture();
        let result = ops.enable_ebs_encryption_by_default(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_vpc_peering_connections() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/").returning_bytes({
            let fixture = DescribeVpcPeeringConnectionsResponse::fixture();
            let full_xml = quick_xml::se::to_string(&fixture).unwrap();
            // Strip the root element wrapper that quick_xml::se adds
            // (e.g. <GetLoginProfileResponse>inner</GetLoginProfileResponse>)
            // so only the inner fields remain for the test envelope.
            let inner = if let Some(gt) = full_xml.find('>') {
                let after_open = &full_xml[gt + 1..];
                if let Some(lt) = after_open.rfind("</") {
                    after_open[..lt].to_string()
                } else {
                    full_xml.clone()
                }
            } else {
                full_xml.clone()
            };
            let xml = format!("<TestResponse><TestResult>{inner}</TestResult></TestResponse>");
            xml.into_bytes()
        });

        let client = crate::AwsHttpClient::from_mock(mock);
        let ops = Ec2Ops::new(&client);

        let body = DescribeVpcPeeringConnectionsRequest::fixture();
        let result = ops.describe_vpc_peering_connections(&body).await;
        assert!(result.is_ok());
    }
}
