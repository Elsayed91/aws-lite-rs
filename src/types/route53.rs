//! Types for the Amazon Route 53 API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Possible values for `route53.ResourceRecordSet.Type`.
///
/// **AWS API**: `route53.ResourceRecordSet.Type`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RRType {
    Soa,

    A,

    Txt,

    Ns,

    Cname,

    Mx,

    Naptr,

    Ptr,

    Srv,

    Spf,

    Aaaa,

    Caa,

    Ds,

    Tlsa,

    Sshfp,

    Svcb,

    Https,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `route53.HealthCheckConfig.Type`.
///
/// **AWS API**: `route53.HealthCheckConfig.Type`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HealthCheckType {
    Http,

    Https,

    HttpStrMatch,

    HttpsStrMatch,

    Tcp,

    Calculated,

    CloudwatchMetric,

    RecoveryControl,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `route53.Change.Action`.
///
/// **AWS API**: `route53.Change.Action`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChangeAction {
    Create,

    Delete,

    Upsert,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `route53.ChangeInfo.Status`.
///
/// **AWS API**: `route53.ChangeInfo.Status`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChangeStatus {
    Pending,

    Insync,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

///
/// **AWS API**: `route53.v1.ListHostedZonesResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListHostedZonesResponse {
    /// A complex type that contains general information about the hosted zone.
    #[serde(default)]
    pub hosted_zones: Vec<HostedZone>,

    /// For the second and subsequent calls to ListHostedZones, Marker is the value that you
    /// specified for the marker parameter in the request that produced the current response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// A flag indicating whether there are more hosted zones to be listed. If the response was
    /// truncated, you can get more hosted zones by submitting another ListHostedZones request
    /// and specifying the value of NextMarker in the marker parameter.
    #[serde(default)]
    pub is_truncated: bool,

    /// If IsTruncated is true, the value of NextMarker identifies the first hosted zone in the
    /// next group of hosted zones. Submit another ListHostedZones request, and specify the
    /// value of NextMarker from the response in the marker parameter. This element is present
    /// only if IsTruncated is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,

    /// The value that you specified for the maxitems parameter in the call to ListHostedZones
    /// that produced the current response.
    pub max_items: String,
}

impl ListHostedZonesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            hosted_zones: vec![],
            marker: Some("test-marker".into()),
            is_truncated: false,
            next_marker: Some("test-next_marker".into()),
            max_items: "test-max_items".into(),
        }
    }
}

/// A complex type that contains general information about the hosted zone.
///
/// **AWS API**: `route53.v1.HostedZone`
///
/// ## Coverage
/// 5 of 7 fields included.
/// Omitted fields:
/// - `LinkedService` — not selected in manifest
/// - `Features` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostedZone {
    /// The ID that Amazon Route 53 assigned to the hosted zone when you created it.
    pub id: String,

    /// The name of the domain. For public hosted zones, this is the name that you have
    /// registered with your DNS registrar. For information about how to specify characters
    /// other than a-z, 0-9, and - (hyphen) and how to specify internationalized domain names,
    /// see CreateHostedZone.
    pub name: String,

    /// The value that you specified for CallerReference when you created the hosted zone.
    pub caller_reference: String,

    /// A complex type that includes the Comment and PrivateZone elements. If you omitted the
    /// HostedZoneConfig and Comment elements from the request, the Config and Comment elements
    /// don't appear in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HostedZoneConfig>,

    /// The number of resource record sets in the hosted zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_record_set_count: Option<i64>,
}

impl HostedZone {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            name: "test-name".into(),
            caller_reference: "test-caller_reference".into(),
            config: Some(HostedZoneConfig::fixture()),
            resource_record_set_count: Some(100),
        }
    }
}

/// A complex type that contains an optional comment about your hosted zone. If you don't want
/// to specify a comment, omit both the HostedZoneConfig and Comment elements.
///
/// **AWS API**: `route53.v1.HostedZoneConfig`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostedZoneConfig {
    /// Any comments that you want to include about the hosted zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A value that indicates whether this is a private hosted zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_zone: Option<bool>,
}

impl HostedZoneConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            comment: Some("test-comment".into()),
            private_zone: Some(false),
        }
    }
}

/// A complex type that contains list information for the resource record set.
///
/// **AWS API**: `route53.v1.ListResourceRecordSetsResponse`
///
/// ## Coverage
/// 5 of 6 fields included.
/// Omitted fields:
/// - `NextRecordIdentifier` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListResourceRecordSetsResponse {
    /// Information about multiple resource record sets.
    #[serde(default)]
    pub resource_record_sets: Vec<ResourceRecordSet>,

    /// A flag that indicates whether more resource record sets remain to be listed. If your
    /// results were truncated, you can make a follow-up pagination request by using the
    /// NextRecordName element.
    #[serde(default)]
    pub is_truncated: bool,

    /// If the results were truncated, the name of the next record in the list. This element is
    /// present only if IsTruncated is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_record_name: Option<String>,

    /// If the results were truncated, the type of the next record in the list. This element is
    /// present only if IsTruncated is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_record_type: Option<String>,

    /// The maximum number of records you requested.
    pub max_items: String,
}

impl ListResourceRecordSetsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource_record_sets: vec![],
            is_truncated: false,
            next_record_name: Some("test-next_record_name".into()),
            next_record_type: Some("test-next_record_type".into()),
            max_items: "test-max_items".into(),
        }
    }
}

/// Information about the resource record set to create or delete.
///
/// **AWS API**: `route53.v1.ResourceRecordSet`
///
/// ## Coverage
/// 8 of 15 fields included.
/// Omitted fields:
/// - `Region` — not selected in manifest
/// - `GeoLocation` — not selected in manifest
/// - `Failover` — not selected in manifest
/// - `MultiValueAnswer` — not selected in manifest
/// - `TrafficPolicyInstanceId` — not selected in manifest
/// - `CidrRoutingConfig` — not selected in manifest
/// - `GeoProximityLocation` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceRecordSet {
    /// For ChangeResourceRecordSets requests, the name of the record that you want to create,
    /// update, or delete. For ListResourceRecordSets responses, the name of a record in the
    /// specified hosted zone. ChangeResourceRecordSets Only Enter a fully qualified domain
    /// name, for example, www.example.com. You can optionally include a trailing dot. If you
    /// omit the trailing dot, Amazon Route 53 assumes that the domain name that you specify is
    /// fully qualified. This means that Route 53 treats www.example.com (without a trailing
    /// dot) and www.example.com. (with a trailing dot) as identical. For information about how
    /// to specify characters other than a-z, 0-9, and - (hyphen) and how to specify
    /// internationalized domain names, see DNS Domain Name Format in the Amazon Route 53
    /// Developer Guide. You can use the asterisk (*) wildcard to replace the leftmost label in
    /// a domain name, for example, *.example.com. Note the following: The
    /// * must replace the entire label. For example, you can't specify *prod.example.com or
    ///   prod*.example.com. The
    /// * can't replace any of the middle labels, for example, marketing.*.example.com. If you
    ///   include
    /// * in any position other than the leftmost label in a domain name, DNS treats it as an
    /// * character (ASCII 42), not as a wildcard. You can't use the
    /// * wildcard for resource records sets that have a type of NS.
    pub name: String,

    /// The DNS record type. For information about different record types and how data is
    /// encoded for them, see Supported DNS Resource Record Types in the Amazon Route 53
    /// Developer Guide. Valid values for basic resource record sets: A | AAAA | CAA | CNAME |
    /// DS |MX | NAPTR | NS | PTR | SOA | SPF | SRV | TXT| TLSA| SSHFP| SVCB| HTTPS Values for
    /// weighted, latency, geolocation, and failover resource record sets: A | AAAA | CAA |
    /// CNAME | MX | NAPTR | PTR | SPF | SRV | TXT| TLSA| SSHFP| SVCB| HTTPS. When creating a
    /// group of weighted, latency, geolocation, or failover resource record sets, specify the
    /// same value for all of the resource record sets in the group. Valid values for multivalue
    /// answer resource record sets: A | AAAA | MX | NAPTR | PTR | SPF | SRV | TXT| CAA| TLSA|
    /// SSHFP| SVCB| HTTPS SPF records were formerly used to verify the identity of the sender
    /// of email messages. However, we no longer recommend that you create resource record sets
    /// for which the value of Type is SPF. RFC 7208, Sender Policy Framework (SPF) for
    /// Authorizing Use of Domains in Email, Version 1, has been updated to say, "...[I]ts
    /// existence and mechanism defined in [RFC4408] have led to some interoperability issues.
    /// Accordingly, its use is no longer appropriate for SPF version 1; implementations are not
    /// to use it." In RFC 7208, see section 14.1, The SPF DNS Record Type. Values for alias
    /// resource record sets: Amazon API Gateway custom regional APIs and edge-optimized APIs: A
    /// CloudFront distributions: A If IPv6 is enabled for the distribution, create two resource
    /// record sets to route traffic to your distribution, one with a value of A and one with a
    /// value of AAAA. Amazon API Gateway environment that has a regionalized subdomain: A ELB
    /// load balancers: A | AAAA Amazon S3 buckets: A Amazon Virtual Private Cloud interface VPC
    /// endpoints A Another resource record set in this hosted zone: Specify the type of the
    /// resource record set that you're creating the alias for. All values are supported except
    /// NS and SOA. If you're creating an alias record that has the same name as the hosted zone
    /// (known as the zone apex), you can't route traffic to a record for which the value of
    /// Type is CNAME. This is because the alias record must have the same type as the record
    /// you're routing traffic to, and creating a CNAME record for the zone apex isn't supported
    /// even for an alias record.
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RRType>,

    /// The resource record cache time to live (TTL), in seconds. Note the following: If you're
    /// creating or updating an alias resource record set, omit TTL. Amazon Route 53 uses the
    /// value of TTL for the alias target. If you're associating this resource record set with a
    /// health check (if you're adding a HealthCheckId element), we recommend that you specify a
    /// TTL of 60 seconds or less so clients respond quickly to changes in health status. All of
    /// the resource record sets in a group of weighted resource record sets must have the same
    /// value for TTL. If a group of weighted resource record sets includes one or more weighted
    /// alias resource record sets for which the alias target is an ELB load balancer, we
    /// recommend that you specify a TTL of 60 seconds for all of the non-alias weighted
    /// resource record sets that have the same name and type. Values other than 60 seconds (the
    /// TTL for load balancers) will change the effect of the values that you specify for
    /// Weight.
    #[serde(rename = "TTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,

    /// Information about the resource records to act upon. If you're creating an alias resource
    /// record set, omit ResourceRecords.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_records: Vec<ResourceRecord>,

    /// If you want Amazon Route 53 to return this resource record set in response to a DNS
    /// query only when the status of a health check is healthy, include the HealthCheckId
    /// element and specify the ID of the applicable health check. Route 53 determines whether a
    /// resource record set is healthy based on one of the following: By periodically sending a
    /// request to the endpoint that is specified in the health check By aggregating the status
    /// of a specified group of health checks (calculated health checks) By determining the
    /// current state of a CloudWatch alarm (CloudWatch metric health checks) Route 53 doesn't
    /// check the health of the endpoint that is specified in the resource record set, for
    /// example, the endpoint specified by the IP address in the Value element. When you add a
    /// HealthCheckId element to a resource record set, Route 53 checks the health of the
    /// endpoint that you specified in the health check. For more information, see the following
    /// topics in the Amazon Route 53 Developer Guide: How Amazon Route 53 Determines Whether an
    /// Endpoint Is Healthy Route 53 Health Checks and DNS Failover Configuring Failover in a
    /// Private Hosted Zone When to Specify HealthCheckId Specifying a value for HealthCheckId
    /// is useful only when Route 53 is choosing between two or more resource record sets to
    /// respond to a DNS query, and you want Route 53 to base the choice in part on the status
    /// of a health check. Configuring health checks makes sense only in the following
    /// configurations: Non-alias resource record sets: You're checking the health of a group of
    /// non-alias resource record sets that have the same routing policy, name, and type (such
    /// as multiple weighted records named www.example.com with a type of A) and you specify
    /// health check IDs for all the resource record sets. If the health check status for a
    /// resource record set is healthy, Route 53 includes the record among the records that it
    /// responds to DNS queries with. If the health check status for a resource record set is
    /// unhealthy, Route 53 stops responding to DNS queries using the value for that resource
    /// record set. If the health check status for all resource record sets in the group is
    /// unhealthy, Route 53 considers all resource record sets in the group healthy and responds
    /// to DNS queries accordingly. Alias resource record sets: You specify the following
    /// settings: You set EvaluateTargetHealth to true for an alias resource record set in a
    /// group of resource record sets that have the same routing policy, name, and type (such as
    /// multiple weighted records named www.example.com with a type of A). You configure the
    /// alias resource record set to route traffic to a non-alias resource record set in the
    /// same hosted zone. You specify a health check ID for the non-alias resource record set.
    /// If the health check status is healthy, Route 53 considers the alias resource record set
    /// to be healthy and includes the alias record among the records that it responds to DNS
    /// queries with. If the health check status is unhealthy, Route 53 stops responding to DNS
    /// queries using the alias resource record set. The alias resource record set can also
    /// route traffic to a group of non-alias resource record sets that have the same routing
    /// policy, name, and type. In that configuration, associate health checks with all of the
    /// resource record sets in the group of non-alias resource record sets. Geolocation Routing
    /// For geolocation resource record sets, if an endpoint is unhealthy, Route 53 looks for a
    /// resource record set for the larger, associated geographic region. For example, suppose
    /// you have resource record sets for a state in the United States, for the entire United
    /// States, for North America, and a resource record set that has
    /// * for CountryCode is *, which applies to all locations. If the endpoint for the state
    ///   resource record set is unhealthy, Route 53 checks for healthy resource record sets in
    ///   the following order until it finds a resource record set for which the endpoint is
    ///   healthy: The United States North America The default resource record set Specifying
    ///   the Health Check Endpoint by Domain Name If your health checks specify the endpoint
    ///   only by domain name, we recommend that you create a separate health check for each
    ///   endpoint. For example, create a health check for each HTTP server that is serving
    ///   content for www.example.com. For the value of FullyQualifiedDomainName, specify the
    ///   domain name of the server (such as us-east-2-www.example.com), not the name of the
    ///   resource record sets (www.example.com). Health check results will be unpredictable if
    ///   you do the following: Create a health check that has the same value for
    ///   FullyQualifiedDomainName as the name of a resource record set. Associate that health
    ///   check with the resource record set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_id: Option<String>,

    /// Resource record sets that have a routing policy other than simple: An identifier that
    /// differentiates among multiple resource record sets that have the same combination of
    /// name and type, such as multiple weighted resource record sets named acme.example.com
    /// that have a type of A. In a group of resource record sets that have the same name and
    /// type, the value of SetIdentifier must be unique for each resource record set. For
    /// information about routing policies, see Choosing a Routing Policy in the Amazon Route 53
    /// Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_identifier: Option<String>,

    /// Weighted resource record sets only: Among resource record sets that have the same
    /// combination of DNS name and type, a value that determines the proportion of DNS queries
    /// that Amazon Route 53 responds to using the current resource record set. Route 53
    /// calculates the sum of the weights for the resource record sets that have the same
    /// combination of DNS name and type. Route 53 then responds to queries based on the ratio
    /// of a resource's weight to the total. Note the following: You must specify a value for
    /// the Weight element for every weighted resource record set. You can only specify one
    /// ResourceRecord per weighted resource record set. You can't create latency, failover, or
    /// geolocation resource record sets that have the same values for the Name and Type
    /// elements as weighted resource record sets. You can create a maximum of 100 weighted
    /// resource record sets that have the same values for the Name and Type elements. For
    /// weighted (but not weighted alias) resource record sets, if you set Weight to 0 for a
    /// resource record set, Route 53 never responds to queries with the applicable value for
    /// that resource record set. However, if you set Weight to 0 for all resource record sets
    /// that have the same combination of DNS name and type, traffic is routed to all resources
    /// with equal probability. The effect of setting Weight to 0 is different when you
    /// associate health checks with weighted resource record sets. For more information, see
    /// Options for Configuring Route 53 Active-Active and Active-Passive Failover in the Amazon
    /// Route 53 Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,

    /// Alias resource record sets only: Information about the Amazon Web Services resource,
    /// such as a CloudFront distribution or an Amazon S3 bucket, that you want to route traffic
    /// to. If you're creating resource records sets for a private hosted zone, note the
    /// following: You can't create an alias resource record set in a private hosted zone to
    /// route traffic to a CloudFront distribution. For information about creating failover
    /// resource record sets in a private hosted zone, see Configuring Failover in a Private
    /// Hosted Zone in the Amazon Route 53 Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_target: Option<AliasTarget>,
}

impl ResourceRecordSet {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-name".into(),
            ttl: Some(100),
            resource_records: vec![],
            health_check_id: Some("test-health_check_id".into()),
            set_identifier: Some("test-set_identifier".into()),
            weight: Some(100),
            alias_target: Some(AliasTarget::fixture()),
            ..Default::default()
        }
    }
}

/// Information specific to the resource record. If you're creating an alias resource record
/// set, omit ResourceRecord.
///
/// **AWS API**: `route53.v1.ResourceRecord`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceRecord {
    /// The current or new DNS record value, not to exceed 4,000 characters. In the case of a
    /// DELETE action, if the current value does not match the actual value, an error is
    /// returned. For descriptions about how to format Value for different record types, see
    /// Supported DNS Resource Record Types in the Amazon Route 53 Developer Guide. You can
    /// specify more than one value for all record types except CNAME and SOA. If you're
    /// creating an alias resource record set, omit Value.
    pub value: String,
}

impl ResourceRecord {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: "test-value".into(),
        }
    }
}

/// Alias resource record sets only: Information about the Amazon Web Services resource, such as
/// a CloudFront distribution or an Amazon S3 bucket, that you want to route traffic to. When
/// creating resource record sets for a private hosted zone, note the following: For information
/// about creating failover resource record sets in a private hosted zone, see Configuring
/// Failover in a Private Hosted Zone.
///
/// **AWS API**: `route53.v1.AliasTarget`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AliasTarget {
    /// Alias resource records sets only: The value used depends on where you want to route
    /// traffic: Amazon API Gateway custom regional APIs and edge-optimized APIs Specify the
    /// hosted zone ID for your API. You can get the applicable value using the CLI command get-
    /// domain-names: For regional APIs, specify the value of regionalHostedZoneId. For edge-
    /// optimized APIs, specify the value of distributionHostedZoneId. Amazon Virtual Private
    /// Cloud interface VPC endpoint Specify the hosted zone ID for your interface endpoint. You
    /// can get the value of HostedZoneId using the CLI command describe-vpc-endpoints.
    /// CloudFront distribution Specify Z2FDTNDATAQYW2. Alias resource record sets for
    /// CloudFront can't be created in a private zone. Elastic Beanstalk environment Specify the
    /// hosted zone ID for the region that you created the environment in. The environment must
    /// have a regionalized subdomain. For a list of regions and the corresponding hosted zone
    /// IDs, see Elastic Beanstalk endpoints and quotas in the Amazon Web Services General
    /// Reference. ELB load balancer Specify the value of the hosted zone ID for the load
    /// balancer. Use the following methods to get the hosted zone ID: Elastic Load Balancing
    /// endpoints and quotas topic in the Amazon Web Services General Reference: Use the value
    /// that corresponds with the region that you created your load balancer in. Note that there
    /// are separate columns for Application and Classic Load Balancers and for Network Load
    /// Balancers. Amazon Web Services Management Console: Go to the Amazon EC2 page, choose
    /// Load Balancers in the navigation pane, select the load balancer, and get the value of
    /// the Hosted zone field on the Description tab. Elastic Load Balancing API: Use
    /// DescribeLoadBalancers to get the applicable value. For more information, see the
    /// applicable guide: Classic Load Balancers: Use DescribeLoadBalancers to get the value of
    /// CanonicalHostedZoneNameId. Application and Network Load Balancers: Use
    /// DescribeLoadBalancers to get the value of CanonicalHostedZoneId. CLI: Use describe-load-
    /// balancers to get the applicable value. For more information, see the applicable guide:
    /// Classic Load Balancers: Use describe-load-balancers to get the value of
    /// CanonicalHostedZoneNameId. Application and Network Load Balancers: Use describe-load-
    /// balancers to get the value of CanonicalHostedZoneId. Global Accelerator accelerator
    /// Specify Z2BJ6XQ5FK7U4H. An Amazon S3 bucket configured as a static website Specify the
    /// hosted zone ID for the region that you created the bucket in. For more information about
    /// valid values, see the table Amazon S3 Website Endpoints in the Amazon Web Services
    /// General Reference. Another Route 53 resource record set in your hosted zone Specify the
    /// hosted zone ID of your hosted zone. (An alias resource record set can't reference a
    /// resource record set in a different hosted zone.)
    pub hosted_zone_id: String,

    /// Alias resource record sets only: The value that you specify depends on where you want to
    /// route queries: Amazon API Gateway custom regional APIs and edge-optimized APIs Specify
    /// the applicable domain name for your API. You can get the applicable value using the CLI
    /// command get-domain-names: For regional APIs, specify the value of regionalDomainName.
    /// For edge-optimized APIs, specify the value of distributionDomainName. This is the name
    /// of the associated CloudFront distribution, such as da1b2c3d4e5.cloudfront.net. The name
    /// of the record that you're creating must match a custom domain name for your API, such as
    /// api.example.com. Amazon Virtual Private Cloud interface VPC endpoint Enter the API
    /// endpoint for the interface endpoint, such as vpce-123456789abcdef01-example-us-
    /// east-1a.elasticloadbalancing.us-east-1.vpce.amazonaws.com. For edge-optimized APIs, this
    /// is the domain name for the corresponding CloudFront distribution. You can get the value
    /// of DnsName using the CLI command describe-vpc-endpoints. CloudFront distribution Specify
    /// the domain name that CloudFront assigned when you created your distribution. Your
    /// CloudFront distribution must include an alternate domain name that matches the name of
    /// the resource record set. For example, if the name of the resource record set is
    /// acme.example.com, your CloudFront distribution must include acme.example.com as one of
    /// the alternate domain names. For more information, see Using Alternate Domain Names
    /// (CNAMEs) in the Amazon CloudFront Developer Guide. You can't create a resource record
    /// set in a private hosted zone to route traffic to a CloudFront distribution. For failover
    /// alias records, you can't specify a CloudFront distribution for both the primary and
    /// secondary records. A distribution must include an alternate domain name that matches the
    /// name of the record. However, the primary and secondary records have the same name, and
    /// you can't include the same alternate domain name in more than one distribution. Elastic
    /// Beanstalk environment If the domain name for your Elastic Beanstalk environment includes
    /// the region that you deployed the environment in, you can create an alias record that
    /// routes traffic to the environment. For example, the domain name my-environment.us-
    /// west-2.elasticbeanstalk.com is a regionalized domain name. For environments that were
    /// created before early 2016, the domain name doesn't include the region. To route traffic
    /// to these environments, you must create a CNAME record instead of an alias record. Note
    /// that you can't create a CNAME record for the root domain name. For example, if your
    /// domain name is example.com, you can create a record that routes traffic for
    /// acme.example.com to your Elastic Beanstalk environment, but you can't create a record
    /// that routes traffic for example.com to your Elastic Beanstalk environment. For Elastic
    /// Beanstalk environments that have regionalized subdomains, specify the CNAME attribute
    /// for the environment. You can use the following methods to get the value of the CNAME
    /// attribute: Amazon Web Services Management Console: For information about how to get the
    /// value by using the console, see Using Custom Domains with Elastic Beanstalk in the
    /// Elastic Beanstalk Developer Guide. Elastic Beanstalk API: Use the DescribeEnvironments
    /// action to get the value of the CNAME attribute. For more information, see
    /// DescribeEnvironments in the Elastic Beanstalk API Reference. CLI: Use the describe-
    /// environments command to get the value of the CNAME attribute. For more information, see
    /// describe-environments in the CLI Command Reference. ELB load balancer Specify the DNS
    /// name that is associated with the load balancer. Get the DNS name by using the Amazon Web
    /// Services Management Console, the ELB API, or the CLI. Amazon Web Services Management
    /// Console: Go to the EC2 page, choose Load Balancers in the navigation pane, choose the
    /// load balancer, choose the Description tab, and get the value of the DNS name field. If
    /// you're routing traffic to a Classic Load Balancer, get the value that begins with
    /// dualstack. If you're routing traffic to another type of load balancer, get the value
    /// that applies to the record type, A or AAAA. Elastic Load Balancing API: Use
    /// DescribeLoadBalancers to get the value of DNSName. For more information, see the
    /// applicable guide: Classic Load Balancers: DescribeLoadBalancers Application and Network
    /// Load Balancers: DescribeLoadBalancers CLI: Use describe-load-balancers to get the value
    /// of DNSName. For more information, see the applicable guide: Classic Load Balancers:
    /// describe-load-balancers Application and Network Load Balancers: describe-load-balancers
    /// Global Accelerator accelerator Specify the DNS name for your accelerator: Global
    /// Accelerator API: To get the DNS name, use DescribeAccelerator. CLI: To get the DNS name,
    /// use describe-accelerator. Amazon S3 bucket that is configured as a static website
    /// Specify the domain name of the Amazon S3 website endpoint that you created the bucket
    /// in, for example, s3-website.us-east-2.amazonaws.com. For more information about valid
    /// values, see the table Amazon S3 Website Endpoints in the Amazon Web Services General
    /// Reference. For more information about using S3 buckets for websites, see Getting Started
    /// with Amazon Route 53 in the Amazon Route 53 Developer Guide. Another Route 53 resource
    /// record set Specify the value of the Name element for a resource record set in the
    /// current hosted zone. If you're creating an alias record that has the same name as the
    /// hosted zone (known as the zone apex), you can't specify the domain name for a record for
    /// which the value of Type is CNAME. This is because the alias record must have the same
    /// type as the record that you're routing traffic to, and creating a CNAME record for the
    /// zone apex isn't supported even for an alias record.
    #[serde(rename = "DNSName")]
    pub dns_name: String,

    /// Applies only to alias, failover alias, geolocation alias, latency alias, and weighted
    /// alias resource record sets: When EvaluateTargetHealth is true, an alias resource record
    /// set inherits the health of the referenced Amazon Web Services resource, such as an ELB
    /// load balancer or another resource record set in the hosted zone. Note the following:
    /// CloudFront distributions You can't set EvaluateTargetHealth to true when the alias
    /// target is a CloudFront distribution. Elastic Beanstalk environments that have
    /// regionalized subdomains If you specify an Elastic Beanstalk environment in DNSName and
    /// the environment contains an ELB load balancer, Elastic Load Balancing routes queries
    /// only to the healthy Amazon EC2 instances that are registered with the load balancer. (An
    /// environment automatically contains an ELB load balancer if it includes more than one
    /// Amazon EC2 instance.) If you set EvaluateTargetHealth to true and either no Amazon EC2
    /// instances are healthy or the load balancer itself is unhealthy, Route 53 routes queries
    /// to other available resources that are healthy, if any. If the environment contains a
    /// single Amazon EC2 instance, there are no special requirements. ELB load balancers Health
    /// checking behavior depends on the type of load balancer: Classic Load Balancers: If you
    /// specify an ELB Classic Load Balancer in DNSName, Elastic Load Balancing routes queries
    /// only to the healthy Amazon EC2 instances that are registered with the load balancer. If
    /// you set EvaluateTargetHealth to true and either no EC2 instances are healthy or the load
    /// balancer itself is unhealthy, Route 53 routes queries to other resources. Application
    /// and Network Load Balancers: If you specify an ELB Application or Network Load Balancer
    /// and you set EvaluateTargetHealth to true, Route 53 routes queries to the load balancer
    /// based on the health of the target groups that are associated with the load balancer: For
    /// an Application or Network Load Balancer to be considered healthy, every target group
    /// that contains targets must contain at least one healthy target. If any target group
    /// contains only unhealthy targets, the load balancer is considered unhealthy, and Route 53
    /// routes queries to other resources. A target group that has no registered targets is
    /// considered unhealthy. When you create a load balancer, you configure settings for
    /// Elastic Load Balancing health checks; they're not Route 53 health checks, but they
    /// perform a similar function. Do not create Route 53 health checks for the EC2 instances
    /// that you register with an ELB load balancer. API Gateway APIs There are no special
    /// requirements for setting EvaluateTargetHealth to true when the alias target is an API
    /// Gateway API. However, because API Gateway is highly available by design,
    /// EvaluateTargetHealth provides no operational benefit and Route 53 health checks are
    /// recommended instead for failover scenarios. S3 buckets There are no special requirements
    /// for setting EvaluateTargetHealth to true when the alias target is an S3 bucket. However,
    /// because S3 buckets are highly available by design, EvaluateTargetHealth provides no
    /// operational benefit and Route 53 health checks are recommended instead for failover
    /// scenarios. VPC interface endpoints There are no special requirements for setting
    /// EvaluateTargetHealth to true when the alias target is a VPC interface endpoint. However,
    /// because VPC interface endpoints are highly available by design, EvaluateTargetHealth
    /// provides no operational benefit and Route 53 health checks are recommended instead for
    /// failover scenarios. Other records in the same hosted zone If the Amazon Web Services
    /// resource that you specify in DNSName is a record or a group of records (for example, a
    /// group of weighted records) but is not another alias record, we recommend that you
    /// associate a health check with all of the records in the alias target. For more
    /// information, see What Happens When You Omit Health Checks? in the Amazon Route 53
    /// Developer Guide. While EvaluateTargetHealth can be set to true for highly available
    /// Amazon Web Services services (such as S3 buckets, VPC interface endpoints, and API
    /// Gateway), these services are designed for high availability and rarely experience
    /// outages that would be detected by this feature. For failover scenarios with these
    /// services, consider using Route 53 health checks that monitor your application's ability
    /// to access the service instead. For more information and examples, see Amazon Route 53
    /// Health Checks and DNS Failover in the Amazon Route 53 Developer Guide.
    #[serde(default)]
    pub evaluate_target_health: bool,
}

impl AliasTarget {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            hosted_zone_id: "test-hosted_zone_id".into(),
            dns_name: "test-dns_name".into(),
            evaluate_target_health: false,
        }
    }
}

/// A complex type that contains the response to a ListHealthChecks request.
///
/// **AWS API**: `route53.v1.ListHealthChecksResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListHealthChecksResponse {
    /// A complex type that contains one HealthCheck element for each health check that is
    /// associated with the current Amazon Web Services account.
    #[serde(default)]
    pub health_checks: Vec<HealthCheck>,

    /// For the second and subsequent calls to ListHealthChecks, Marker is the value that you
    /// specified for the marker parameter in the previous request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// A flag that indicates whether there are more health checks to be listed. If the response
    /// was truncated, you can get the next group of health checks by submitting another
    /// ListHealthChecks request and specifying the value of NextMarker in the marker parameter.
    #[serde(default)]
    pub is_truncated: bool,

    /// If IsTruncated is true, the value of NextMarker identifies the first health check that
    /// Amazon Route 53 returns if you submit another ListHealthChecks request and specify the
    /// value of NextMarker in the marker parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,

    /// The value that you specified for the maxitems parameter in the call to ListHealthChecks
    /// that produced the current response.
    pub max_items: String,
}

impl ListHealthChecksResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            health_checks: vec![],
            marker: Some("test-marker".into()),
            is_truncated: false,
            next_marker: Some("test-next_marker".into()),
            max_items: "test-max_items".into(),
        }
    }
}

/// A complex type that contains information about one health check that is associated with the
/// current Amazon Web Services account.
///
/// **AWS API**: `route53.v1.HealthCheck`
///
/// ## Coverage
/// 4 of 6 fields included.
/// Omitted fields:
/// - `LinkedService` — not selected in manifest
/// - `CloudWatchAlarmConfiguration` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthCheck {
    /// The identifier that Amazon Route 53 assigned to the health check when you created it.
    /// When you add or update a resource record set, you use this value to specify which health
    /// check to use. The value can be up to 64 characters long.
    pub id: String,

    /// A unique string that you specified when you created the health check.
    pub caller_reference: String,

    /// A complex type that contains detailed information about one health check.
    pub health_check_config: HealthCheckConfig,

    /// The version of the health check. You can optionally pass this value in a call to
    /// UpdateHealthCheck to prevent overwriting another change to the health check.
    pub health_check_version: i64,
}

impl HealthCheck {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            caller_reference: "test-caller_reference".into(),
            health_check_config: HealthCheckConfig::fixture(),
            health_check_version: 100,
        }
    }
}

/// A complex type that contains information about the health check.
///
/// **AWS API**: `route53.v1.HealthCheckConfig`
///
/// ## Coverage
/// 7 of 18 fields included.
/// Omitted fields:
/// - `SearchString` — not selected in manifest
/// - `MeasureLatency` — not selected in manifest
/// - `Inverted` — not selected in manifest
/// - `Disabled` — not selected in manifest
/// - `HealthThreshold` — not selected in manifest
/// - `ChildHealthChecks` — not selected in manifest
/// - `EnableSNI` — not selected in manifest
/// - `Regions` — not selected in manifest
/// - `AlarmIdentifier` — not selected in manifest
/// - `InsufficientDataHealthStatus` — not selected in manifest
/// - `RoutingControlArn` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthCheckConfig {
    /// The type of health check that you want to create, which indicates how Amazon Route 53
    /// determines whether an endpoint is healthy. You can't change the value of Type after you
    /// create a health check. You can create the following types of health checks: HTTP: Route
    /// 53 tries to establish a TCP connection. If successful, Route 53 submits an HTTP request
    /// and waits for an HTTP status code of 200 or greater and less than 400. HTTPS: Route 53
    /// tries to establish a TCP connection. If successful, Route 53 submits an HTTPS request
    /// and waits for an HTTP status code of 200 or greater and less than 400. If you specify
    /// HTTPS for the value of Type, the endpoint must support TLS v1.0, v1.1, or v1.2.
    /// HTTP_STR_MATCH: Route 53 tries to establish a TCP connection. If successful, Route 53
    /// submits an HTTP request and searches the first 5,120 bytes of the response body for the
    /// string that you specify in SearchString. HTTPS_STR_MATCH: Route 53 tries to establish a
    /// TCP connection. If successful, Route 53 submits an HTTPS request and searches the first
    /// 5,120 bytes of the response body for the string that you specify in SearchString. TCP:
    /// Route 53 tries to establish a TCP connection. CLOUDWATCH_METRIC: The health check is
    /// associated with a CloudWatch alarm. If the state of the alarm is OK, the health check is
    /// considered healthy. If the state is ALARM, the health check is considered unhealthy. If
    /// CloudWatch doesn't have sufficient data to determine whether the state is OK or ALARM,
    /// the health check status depends on the setting for InsufficientDataHealthStatus:
    /// Healthy, Unhealthy, or LastKnownStatus. CALCULATED: For health checks that monitor the
    /// status of other health checks, Route 53 adds up the number of health checks that Route
    /// 53 health checkers consider to be healthy and compares that number with the value of
    /// HealthThreshold. RECOVERY_CONTROL: The health check is associated with a Route53
    /// Application Recovery Controller routing control. If the routing control state is ON, the
    /// health check is considered healthy. If the state is OFF, the health check is considered
    /// unhealthy. For more information, see How Route 53 Determines Whether an Endpoint Is
    /// Healthy in the Amazon Route 53 Developer Guide.
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<HealthCheckType>,

    /// The IPv4 or IPv6 IP address of the endpoint that you want Amazon Route 53 to perform
    /// health checks on. If you don't specify a value for IPAddress, Route 53 sends a DNS
    /// request to resolve the domain name that you specify in FullyQualifiedDomainName at the
    /// interval that you specify in RequestInterval. Using an IP address returned by DNS, Route
    /// 53 then checks the health of the endpoint. Use one of the following formats for the
    /// value of IPAddress: IPv4 address: four values between 0 and 255, separated by periods
    /// (.), for example, 192.0.2.44. IPv6 address: eight groups of four hexadecimal values,
    /// separated by colons (:), for example, 2001:0db8:85a3:0000:0000:abcd:0001:2345. You can
    /// also shorten IPv6 addresses as described in RFC 5952, for example,
    /// 2001:db8:85a3::abcd:1:2345. If the endpoint is an EC2 instance, we recommend that you
    /// create an Elastic IP address, associate it with your EC2 instance, and specify the
    /// Elastic IP address for IPAddress. This ensures that the IP address of your instance will
    /// never change. For more information, see FullyQualifiedDomainName. Constraints: Route 53
    /// can't check the health of endpoints for which the IP address is in local, private, non-
    /// routable, or multicast ranges. For more information about IP addresses for which you
    /// can't create health checks, see the following documents: RFC 5735, Special Use IPv4
    /// Addresses RFC 6598, IANA-Reserved IPv4 Prefix for Shared Address Space RFC 5156,
    /// Special-Use IPv6 Addresses When the value of Type is CALCULATED or CLOUDWATCH_METRIC,
    /// omit IPAddress.
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// The port on the endpoint that you want Amazon Route 53 to perform health checks on.
    /// Don't specify a value for Port when you specify a value for Type of CLOUDWATCH_METRIC or
    /// CALCULATED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// Amazon Route 53 behavior depends on whether you specify a value for IPAddress. If you
    /// specify a value for IPAddress: Amazon Route 53 sends health check requests to the
    /// specified IPv4 or IPv6 address and passes the value of FullyQualifiedDomainName in the
    /// Host header for all health checks except TCP health checks. This is typically the fully
    /// qualified DNS name of the endpoint on which you want Route 53 to perform health checks.
    /// When Route 53 checks the health of an endpoint, here is how it constructs the Host
    /// header: If you specify a value of 80 for Port and HTTP or HTTP_STR_MATCH for Type, Route
    /// 53 passes the value of FullyQualifiedDomainName to the endpoint in the Host header. If
    /// you specify a value of 443 for Port and HTTPS or HTTPS_STR_MATCH for Type, Route 53
    /// passes the value of FullyQualifiedDomainName to the endpoint in the Host header. If you
    /// specify another value for Port and any value except TCP for Type, Route 53 passes
    /// FullyQualifiedDomainName:Port to the endpoint in the Host header. If you don't specify a
    /// value for FullyQualifiedDomainName, Route 53 substitutes the value of IPAddress in the
    /// Host header in each of the preceding cases. If you don't specify a value for IPAddress:
    /// Route 53 sends a DNS request to the domain that you specify for FullyQualifiedDomainName
    /// at the interval that you specify for RequestInterval. Using an IPv4 address that DNS
    /// returns, Route 53 then checks the health of the endpoint. If you don't specify a value
    /// for IPAddress, Route 53 uses only IPv4 to send health checks to the endpoint. If there's
    /// no resource record set with a type of A for the name that you specify for
    /// FullyQualifiedDomainName, the health check fails with a "DNS resolution failed" error.
    /// If you want to check the health of weighted, latency, or failover resource record sets
    /// and you choose to specify the endpoint only by FullyQualifiedDomainName, we recommend
    /// that you create a separate health check for each endpoint. For example, create a health
    /// check for each HTTP server that is serving content for www.example.com. For the value of
    /// FullyQualifiedDomainName, specify the domain name of the server (such as us-
    /// east-2-www.example.com), not the name of the resource record sets (www.example.com). In
    /// this configuration, if you create a health check for which the value of
    /// FullyQualifiedDomainName matches the name of the resource record sets and you then
    /// associate the health check with those resource record sets, health check results will be
    /// unpredictable. In addition, if the value that you specify for Type is HTTP, HTTPS,
    /// HTTP_STR_MATCH, or HTTPS_STR_MATCH, Route 53 passes the value of
    /// FullyQualifiedDomainName in the Host header, as it does when you specify a value for
    /// IPAddress. If the value of Type is TCP, Route 53 doesn't pass a Host header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,

    /// The path, if any, that you want Amazon Route 53 to request when performing health
    /// checks. The path can be any value for which your endpoint will return an HTTP status
    /// code of 2xx or 3xx when the endpoint is healthy, for example, the file
    /// /docs/route53-health-check.html. You can also include query string parameters, for
    /// example, /welcome.html?language=jp&amp;login=y.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,

    /// The number of seconds between the time that Amazon Route 53 gets a response from your
    /// endpoint and the time that it sends the next health check request. Each Route 53 health
    /// checker makes requests at this interval. RequestInterval is not supported when you
    /// specify a value for Type of RECOVERY_CONTROL. You can't change the value of
    /// RequestInterval after you create a health check. If you don't specify a value for
    /// RequestInterval, the default value is 30 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_interval: Option<i32>,

    /// The number of consecutive health checks that an endpoint must pass or fail for Amazon
    /// Route 53 to change the current status of the endpoint from unhealthy to healthy or vice
    /// versa. For more information, see How Amazon Route 53 Determines Whether an Endpoint Is
    /// Healthy in the Amazon Route 53 Developer Guide. FailureThreshold is not supported when
    /// you specify a value for Type of RECOVERY_CONTROL. Otherwise, if you don't specify a
    /// value for FailureThreshold, the default value is three health checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
}

impl HealthCheckConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ip_address: Some("test-ip_address".into()),
            port: Some(100),
            fully_qualified_domain_name: Some("test-fully_qualified_domain_name".into()),
            resource_path: Some("test-resource_path".into()),
            request_interval: Some(100),
            failure_threshold: Some(100),
            ..Default::default()
        }
    }
}

/// A complex type that contains the response to a GetHealthCheck request.
///
/// **AWS API**: `route53.v1.GetHealthCheckStatusResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetHealthCheckStatusResponse {
    /// A list that contains one HealthCheckObservation element for each Amazon Route 53 health
    /// checker that is reporting a status about the health check endpoint.
    #[serde(default)]
    pub health_check_observations: Vec<HealthCheckObservation>,
}

impl GetHealthCheckStatusResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            health_check_observations: vec![],
        }
    }
}

/// A complex type that contains the last failure reason as reported by one Amazon Route 53
/// health checker.
///
/// **AWS API**: `route53.v1.HealthCheckObservation`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthCheckObservation {
    /// The region of the Amazon Route 53 health checker that provided the status in
    /// StatusReport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// The IP address of the Amazon Route 53 health checker that provided the failure reason in
    /// StatusReport.
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// A complex type that contains the last failure reason as reported by one Amazon Route 53
    /// health checker and the time of the failed health check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_report: Option<StatusReport>,
}

impl HealthCheckObservation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            region: Some("test-region".into()),
            ip_address: Some("test-ip_address".into()),
            status_report: Some(StatusReport::fixture()),
        }
    }
}

/// A complex type that contains the status that one Amazon Route 53 health checker reports and
/// the time of the health check.
///
/// **AWS API**: `route53.v1.StatusReport`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatusReport {
    /// A description of the status of the health check endpoint as reported by one of the
    /// Amazon Route 53 health checkers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The date and time that the health checker performed the health check in ISO 8601 format
    /// and Coordinated Universal Time (UTC). For example, the value 2017-03-27T17:48:16.751Z
    /// represents March 27, 2017 at 17:48:16.751 UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_time: Option<String>,
}

impl StatusReport {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            status: Some("test-status".into()),
            checked_time: Some("test-checked_time".into()),
        }
    }
}

/// A complex type that contains the health check request information.
///
/// **AWS API**: `route53.v1.CreateHealthCheckRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateHealthCheckRequest {
    /// A unique string that identifies the request and that allows you to retry a failed
    /// CreateHealthCheck request without the risk of creating two identical health checks: If
    /// you send a CreateHealthCheck request with the same CallerReference and settings as a
    /// previous request, and if the health check doesn't exist, Amazon Route 53 creates the
    /// health check. If the health check does exist, Route 53 returns the health check
    /// configuration in the response. If you send a CreateHealthCheck request with the same
    /// CallerReference as a deleted health check, regardless of the settings, Route 53 returns
    /// a HealthCheckAlreadyExists error. If you send a CreateHealthCheck request with the same
    /// CallerReference as an existing health check but with different settings, Route 53
    /// returns a HealthCheckAlreadyExists error. If you send a CreateHealthCheck request with a
    /// unique CallerReference but settings identical to an existing health check, Route 53
    /// creates the health check. Route 53 does not store the CallerReference for a deleted
    /// health check indefinitely. The CallerReference for a deleted health check will be
    /// deleted after a number of days.
    pub caller_reference: String,

    /// A complex type that contains settings for a new health check.
    pub health_check_config: HealthCheckConfig,
}

impl CreateHealthCheckRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            caller_reference: "test-caller_reference".into(),
            health_check_config: HealthCheckConfig::fixture(),
        }
    }
}

/// A complex type containing the response information for the new health check.
///
/// **AWS API**: `route53.v1.CreateHealthCheckResponse`
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `Location` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateHealthCheckResponse {
    /// A complex type that contains identifying information about the health check.
    pub health_check: HealthCheck,
}

impl CreateHealthCheckResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            health_check: HealthCheck::fixture(),
        }
    }
}

/// A complex type that contains change information for the resource record set.
///
/// **AWS API**: `route53.v1.ChangeResourceRecordSetsRequest`
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `HostedZoneId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeResourceRecordSetsRequest {
    /// A complex type that contains an optional comment and the Changes element.
    pub change_batch: ChangeBatch,
}

impl ChangeResourceRecordSetsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            change_batch: ChangeBatch::fixture(),
        }
    }
}

/// The information for a change request.
///
/// **AWS API**: `route53.v1.ChangeBatch`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeBatch {
    /// Optional: Any comments you want to include about a change batch request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Information about the changes to make to the record sets.
    #[serde(default)]
    pub changes: Vec<Change>,
}

impl ChangeBatch {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            comment: Some("test-comment".into()),
            changes: vec![],
        }
    }
}

/// The information for each resource record set that you want to change.
///
/// **AWS API**: `route53.v1.Change`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Change {
    /// The action to perform: CREATE: Creates a resource record set that has the specified
    /// values. DELETE: Deletes a existing resource record set. To delete the resource record
    /// set that is associated with a traffic policy instance, use DeleteTrafficPolicyInstance.
    /// Amazon Route 53 will delete the resource record set automatically. If you delete the
    /// resource record set by using ChangeResourceRecordSets, Route 53 doesn't automatically
    /// delete the traffic policy instance, and you'll continue to be charged for it even though
    /// it's no longer in use. UPSERT: If a resource record set doesn't already exist, Route 53
    /// creates it. If a resource record set does exist, Route 53 updates it with the values in
    /// the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ChangeAction>,

    /// Information about the resource record set to create, delete, or update.
    pub resource_record_set: ResourceRecordSet,
}

impl Change {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource_record_set: ResourceRecordSet::fixture(),
            ..Default::default()
        }
    }
}

/// A complex type containing the response for the request.
///
/// **AWS API**: `route53.v1.ChangeResourceRecordSetsResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeResourceRecordSetsResponse {
    /// A complex type that contains information about changes made to your hosted zone. This
    /// element contains an ID that you use when performing a GetChange action to get detailed
    /// information about the change.
    pub change_info: ChangeInfo,
}

impl ChangeResourceRecordSetsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            change_info: ChangeInfo::fixture(),
        }
    }
}

/// A complex type that describes change information about changes made to your hosted zone.
///
/// **AWS API**: `route53.v1.ChangeInfo`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeInfo {
    /// This element contains an ID that you use when performing a GetChange action to get
    /// detailed information about the change.
    pub id: String,

    /// The current state of the request. PENDING indicates that this request has not yet been
    /// applied to all Amazon Route 53 DNS servers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStatus>,

    /// The date and time that the change request was submitted in ISO 8601 format and
    /// Coordinated Universal Time (UTC). For example, the value 2017-03-27T17:48:16.751Z
    /// represents March 27, 2017 at 17:48:16.751 UTC.
    pub submitted_at: String,

    /// A comment you can provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ChangeInfo {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            submitted_at: "test-submitted_at".into(),
            comment: Some("test-comment".into()),
            ..Default::default()
        }
    }
}
