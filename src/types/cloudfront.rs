//! Types for the Amazon CloudFront API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// A distribution list.
///
/// **AWS API**: `cloudfront.v1.DistributionList`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/DistributionList>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DistributionList {
    /// The value you provided for the Marker request parameter.
    pub marker: String,

    /// If IsTruncated is true, this element is present and contains the value you can use for
    /// the Marker request parameter to continue listing your distributions where they left off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,

    /// The value you provided for the MaxItems request parameter.
    pub max_items: i32,

    /// A flag that indicates whether more distributions remain to be listed. If your results
    /// were truncated, you can make a follow-up pagination request using the Marker request
    /// parameter to retrieve more distributions in the list.
    #[serde(default)]
    pub is_truncated: bool,

    /// The number of distributions that were created by the current Amazon Web Services
    /// account.
    pub quantity: i32,

    /// A complex type that contains one DistributionSummary element for each distribution that
    /// was created by the current Amazon Web Services account.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DistributionSummary>,
}

impl DistributionList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            marker: "test-marker".into(),
            next_marker: Some("test-next_marker".into()),
            max_items: 100,
            is_truncated: false,
            quantity: 100,
            items: vec![],
        }
    }
}

/// A summary of the information about a CloudFront distribution.
///
/// **AWS API**: `cloudfront.v1.DistributionSummary`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/DistributionSummary>
///
/// ## Coverage
/// 9 of 26 fields included.
/// Omitted fields:
/// - `ETag` — not selected in manifest
/// - `LastModifiedTime` — not selected in manifest
/// - `Aliases` — not selected in manifest
/// - `OriginGroups` — not selected in manifest
/// - `CacheBehaviors` — not selected in manifest
/// - `CustomErrorResponses` — not selected in manifest
/// - `ViewerCertificate` — not selected in manifest
/// - `Restrictions` — not selected in manifest
/// - `WebACLId` — not selected in manifest
/// - `HttpVersion` — not selected in manifest
/// - `IsIPV6Enabled` — not selected in manifest
/// - `AliasICPRecordals` — not selected in manifest
/// - `Staging` — not selected in manifest
/// - `ConnectionMode` — not selected in manifest
/// - `AnycastIpListId` — not selected in manifest
/// - `ViewerMtlsConfig` — not selected in manifest
/// - `ConnectionFunctionAssociation` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DistributionSummary {
    /// The identifier for the distribution. For example: EDFDVBD632BHDS5.
    pub id: String,

    /// The ARN (Amazon Resource Name) for the distribution. For example:
    /// arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5, where 123456789012 is
    /// your Amazon Web Services account ID.
    #[serde(rename = "ARN")]
    pub arn: String,

    /// The current status of the distribution. When the status is Deployed, the distribution's
    /// information is propagated to all CloudFront edge locations.
    pub status: String,

    /// The domain name that corresponds to the distribution, for example,
    /// d111111abcdef8.cloudfront.net.
    pub domain_name: String,

    /// A complex type that contains information about origins for this distribution.
    pub origins: Origins,

    /// A complex type that describes the default cache behavior if you don't specify a
    /// CacheBehavior element or if files don't match any of the values of PathPattern in
    /// CacheBehavior elements. You must create exactly one default cache behavior.
    pub default_cache_behavior: DefaultCacheBehavior,

    /// This field only supports standard distributions. You can't specify this field for multi-
    /// tenant distributions. For more information, see Unsupported features for SaaS Manager
    /// for Amazon CloudFront in the Amazon CloudFront Developer Guide. A complex type that
    /// contains information about price class for this streaming distribution.
    pub price_class: String,

    /// Whether the distribution is enabled to accept user requests for content.
    #[serde(default)]
    pub enabled: bool,

    /// The comment originally specified when this distribution was created.
    pub comment: String,
}

impl DistributionSummary {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            arn: "test-arn".into(),
            status: "test-status".into(),
            domain_name: "test-domain_name".into(),
            origins: Origins::fixture(),
            default_cache_behavior: DefaultCacheBehavior::fixture(),
            price_class: "test-price_class".into(),
            enabled: false,
            comment: "test-comment".into(),
        }
    }
}

/// A distribution tells CloudFront where you want content to be delivered from, and the details
/// about how to track and manage content delivery.
///
/// **AWS API**: `cloudfront.v1.Distribution`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/Distribution>
///
/// ## Coverage
/// 5 of 10 fields included.
/// Omitted fields:
/// - `LastModifiedTime` — not selected in manifest
/// - `InProgressInvalidationBatches` — not selected in manifest
/// - `ActiveTrustedSigners` — not selected in manifest
/// - `ActiveTrustedKeyGroups` — not selected in manifest
/// - `AliasICPRecordals` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Distribution {
    /// The distribution's identifier. For example: E1U5RQF7T870K0.
    pub id: String,

    /// The distribution's Amazon Resource Name (ARN).
    #[serde(rename = "ARN")]
    pub arn: String,

    /// The distribution's status. When the status is Deployed, the distribution's information
    /// is fully propagated to all CloudFront edge locations.
    pub status: String,

    /// The distribution's CloudFront domain name. For example: d111111abcdef8.cloudfront.net.
    pub domain_name: String,

    /// The distribution's configuration.
    pub distribution_config: DistributionConfig,
}

impl Distribution {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            arn: "test-arn".into(),
            status: "test-status".into(),
            domain_name: "test-domain_name".into(),
            distribution_config: DistributionConfig::fixture(),
        }
    }
}

/// A distribution configuration.
///
/// **AWS API**: `cloudfront.v1.DistributionConfig`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/DistributionConfig>
///
/// ## Coverage
/// 9 of 24 fields included.
/// Omitted fields:
/// - `OriginGroups` — not selected in manifest
/// - `CacheBehaviors` — not selected in manifest
/// - `CustomErrorResponses` — not selected in manifest
/// - `Logging` — not selected in manifest
/// - `Restrictions` — not selected in manifest
/// - `WebACLId` — not selected in manifest
/// - `HttpVersion` — not selected in manifest
/// - `IsIPV6Enabled` — not selected in manifest
/// - `ContinuousDeploymentPolicyId` — not selected in manifest
/// - `Staging` — not selected in manifest
/// - `AnycastIpListId` — not selected in manifest
/// - `TenantConfig` — not selected in manifest
/// - `ConnectionMode` — not selected in manifest
/// - `ViewerMtlsConfig` — not selected in manifest
/// - `ConnectionFunctionAssociation` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DistributionConfig {
    /// A unique value (for example, a date-time stamp) that ensures that the request can't be
    /// replayed. If the value of CallerReference is new (regardless of the content of the
    /// DistributionConfig object), CloudFront creates a new distribution. If CallerReference is
    /// a value that you already sent in a previous request to create a distribution, CloudFront
    /// returns a DistributionAlreadyExists error.
    pub caller_reference: String,

    /// This field only supports standard distributions. You can't specify this field for multi-
    /// tenant distributions. For more information, see Unsupported features for SaaS Manager
    /// for Amazon CloudFront in the Amazon CloudFront Developer Guide. A complex type that
    /// contains information about CNAMEs (alternate domain names), if any, for this
    /// distribution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Aliases>,

    /// When a viewer requests the root URL for your distribution, the default root object is
    /// the object that you want CloudFront to request from your origin. For example, if your
    /// root URL is https://www.example.com, you can specify CloudFront to return the index.html
    /// file as the default root object. You can specify a default root object so that viewers
    /// see a specific file or object, instead of another object in your distribution (for
    /// example, https://www.example.com/product-description.html). A default root object avoids
    /// exposing the contents of your distribution. You can specify the object name or a path to
    /// the object name (for example, index.html or exampleFolderName/index.html). Your string
    /// can't begin with a forward slash (/). Only specify the object name or the path to the
    /// object. If you don't want to specify a default root object when you create a
    /// distribution, include an empty DefaultRootObject element. To delete the default root
    /// object from an existing distribution, update the distribution configuration and include
    /// an empty DefaultRootObject element. To replace the default root object, update the
    /// distribution configuration and specify the new object. For more information about the
    /// default root object, see Specify a default root object in the Amazon CloudFront
    /// Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root_object: Option<String>,

    /// A complex type that contains information about origins for this distribution.
    pub origins: Origins,

    /// A complex type that describes the default cache behavior if you don't specify a
    /// CacheBehavior element or if files don't match any of the values of PathPattern in
    /// CacheBehavior elements. You must create exactly one default cache behavior.
    pub default_cache_behavior: DefaultCacheBehavior,

    /// A comment to describe the distribution. The comment cannot be longer than 128
    /// characters.
    pub comment: String,

    /// This field only supports standard distributions. You can't specify this field for multi-
    /// tenant distributions. For more information, see Unsupported features for SaaS Manager
    /// for Amazon CloudFront in the Amazon CloudFront Developer Guide. The price class that
    /// corresponds with the maximum price that you want to pay for CloudFront service. If you
    /// specify PriceClass_All, CloudFront responds to requests for your objects from all
    /// CloudFront edge locations. If you specify a price class other than PriceClass_All,
    /// CloudFront serves your objects from the CloudFront edge location that has the lowest
    /// latency among the edge locations in your price class. Viewers who are in or near regions
    /// that are excluded from your specified price class may encounter slower performance. For
    /// more information about price classes, see Choosing the Price Class for a CloudFront
    /// Distribution in the Amazon CloudFront Developer Guide. For information about CloudFront
    /// pricing, including how price classes (such as Price Class 100) map to CloudFront
    /// regions, see Amazon CloudFront Pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_class: Option<String>,

    /// From this field, you can enable or disable the selected distribution.
    #[serde(default)]
    pub enabled: bool,

    /// A complex type that determines the distribution's SSL/TLS configuration for
    /// communicating with viewers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_certificate: Option<ViewerCertificate>,
}

impl DistributionConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            caller_reference: "test-caller_reference".into(),
            aliases: Some(Aliases::fixture()),
            default_root_object: Some("test-default_root_object".into()),
            origins: Origins::fixture(),
            default_cache_behavior: DefaultCacheBehavior::fixture(),
            comment: "test-comment".into(),
            price_class: Some("test-price_class".into()),
            enabled: false,
            viewer_certificate: Some(ViewerCertificate::fixture()),
        }
    }
}

/// A complex type that contains information about CNAMEs (alternate domain names), if any, for
/// this distribution.
///
/// **AWS API**: `cloudfront.v1.Aliases`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/Aliases>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Aliases {
    /// The number of CNAME aliases, if any, that you want to associate with this distribution.
    pub quantity: i32,

    /// A complex type that contains the CNAME aliases, if any, that you want to associate with
    /// this distribution.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}

impl Aliases {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            quantity: 100,
            items: vec![],
        }
    }
}

/// A complex type that determines the distribution's SSL/TLS configuration for communicating
/// with viewers. If the distribution doesn't use Aliases (also known as alternate domain names
/// or CNAMEs)—that is, if the distribution uses the CloudFront domain name such as
/// d111111abcdef8.cloudfront.net—set CloudFrontDefaultCertificate to true and leave all other
/// fields empty. If the distribution uses Aliases (alternate domain names or CNAMEs), use the
/// fields in this type to specify the following settings: Which viewers the distribution
/// accepts HTTPS connections from: only viewers that support server name indication (SNI)
/// (recommended), or all viewers including those that don't support SNI. To accept HTTPS
/// connections from only viewers that support SNI, set SSLSupportMethod to sni-only. This is
/// recommended. Most browsers and clients support SNI. To accept HTTPS connections from all
/// viewers, including those that don't support SNI, set SSLSupportMethod to vip. This is not
/// recommended, and results in additional monthly charges from CloudFront. The minimum SSL/TLS
/// protocol version that the distribution can use to communicate with viewers. To specify a
/// minimum version, choose a value for MinimumProtocolVersion. For more information, see
/// Security Policy in the Amazon CloudFront Developer Guide. The location of the SSL/TLS
/// certificate, Certificate Manager (ACM) (recommended) or Identity and Access Management
/// (IAM). You specify the location by setting a value in one of the following fields (not
/// both): ACMCertificateArn IAMCertificateId All distributions support HTTPS connections from
/// viewers. To require viewers to use HTTPS only, or to redirect them from HTTP to HTTPS, use
/// ViewerProtocolPolicy in the CacheBehavior or DefaultCacheBehavior. To specify how CloudFront
/// should use SSL/TLS to communicate with your custom origin, use CustomOriginConfig. For more
/// information, see Using HTTPS with CloudFront and Using Alternate Domain Names and HTTPS in
/// the Amazon CloudFront Developer Guide.
///
/// **AWS API**: `cloudfront.v1.ViewerCertificate`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/ViewerCertificate>
///
/// ## Coverage
/// 5 of 7 fields included.
/// Omitted fields:
/// - `Certificate` — not selected in manifest
/// - `CertificateSource` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ViewerCertificate {
    /// If the distribution uses the CloudFront domain name such as
    /// d111111abcdef8.cloudfront.net, set this field to true. If the distribution uses Aliases
    /// (alternate domain names or CNAMEs), set this field to false and specify values for the
    /// following fields: ACMCertificateArn or IAMCertificateId (specify a value for one, not
    /// both) MinimumProtocolVersion SSLSupportMethod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_default_certificate: Option<bool>,

    /// This field only supports standard distributions. You can't specify this field for multi-
    /// tenant distributions. For more information, see Unsupported features for SaaS Manager
    /// for Amazon CloudFront in the Amazon CloudFront Developer Guide. If the distribution uses
    /// Aliases (alternate domain names or CNAMEs) and the SSL/TLS certificate is stored in
    /// Identity and Access Management (IAM), provide the ID of the IAM certificate. If you
    /// specify an IAM certificate ID, you must also specify values for MinimumProtocolVersion
    /// and SSLSupportMethod.
    #[serde(rename = "IAMCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_certificate_id: Option<String>,

    /// If the distribution uses Aliases (alternate domain names or CNAMEs) and the SSL/TLS
    /// certificate is stored in Certificate Manager (ACM), provide the Amazon Resource Name
    /// (ARN) of the ACM certificate. CloudFront only supports ACM certificates in the US East
    /// (N. Virginia) Region (us-east-1). If you specify an ACM certificate ARN, you must also
    /// specify values for MinimumProtocolVersion and SSLSupportMethod.
    #[serde(rename = "ACMCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_certificate_arn: Option<String>,

    /// If the distribution uses Aliases (alternate domain names or CNAMEs), specify which
    /// viewers the distribution accepts HTTPS connections from. sni-only – The distribution
    /// accepts HTTPS connections from only viewers that support server name indication (SNI).
    /// This is recommended. Most browsers and clients support SNI. vip – The distribution
    /// accepts HTTPS connections from all viewers including those that don't support SNI. This
    /// is not recommended, and results in additional monthly charges from CloudFront. static-ip
    /// - Do not specify this value unless your distribution has been enabled for this feature
    ///   by the CloudFront team. If you have a use case that requires static IP addresses for a
    ///   distribution, contact CloudFront through the Amazon Web ServicesSupport Center. If the
    ///   distribution uses the CloudFront domain name such as d111111abcdef8.cloudfront.net,
    ///   don't set a value for this field.
    #[serde(rename = "SSLSupportMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_support_method: Option<String>,

    /// If the distribution uses Aliases (alternate domain names or CNAMEs), specify the
    /// security policy that you want CloudFront to use for HTTPS connections with viewers. The
    /// security policy determines two settings: The minimum SSL/TLS protocol that CloudFront
    /// can use to communicate with viewers. The ciphers that CloudFront can use to encrypt the
    /// content that it returns to viewers. For more information, see Security Policy and
    /// Supported Protocols and Ciphers Between Viewers and CloudFront in the Amazon CloudFront
    /// Developer Guide. On the CloudFront console, this setting is called Security Policy. When
    /// you're using SNI only (you set SSLSupportMethod to sni-only), you must specify TLSv1 or
    /// higher. If the distribution uses the CloudFront domain name such as
    /// d111111abcdef8.cloudfront.net (you set CloudFrontDefaultCertificate to true), CloudFront
    /// automatically sets the security policy to TLSv1 regardless of the value that you set
    /// here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_protocol_version: Option<String>,
}

impl ViewerCertificate {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cloud_front_default_certificate: Some(false),
            iam_certificate_id: Some("test-iam_certificate_id".into()),
            acm_certificate_arn: Some("test-acm_certificate_arn".into()),
            ssl_support_method: Some("test-ssl_support_method".into()),
            minimum_protocol_version: Some("test-minimum_protocol_version".into()),
        }
    }
}

/// Contains information about the origins for this distribution.
///
/// **AWS API**: `cloudfront.v1.Origins`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/Origins>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Origins {
    /// The number of origins for this distribution.
    pub quantity: i32,

    /// A list of origins.
    #[serde(default)]
    pub items: Vec<Origin>,
}

impl Origins {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            quantity: 100,
            items: vec![],
        }
    }
}

/// An origin. An origin is the location where content is stored, and from which CloudFront gets
/// content to serve to viewers. To specify an origin: Use S3OriginConfig to specify an Amazon
/// S3 bucket that is not configured with static website hosting. Use VpcOriginConfig to specify
/// a VPC origin. Use CustomOriginConfig to specify all other kinds of origins, including: An
/// Amazon S3 bucket that is configured with static website hosting An Elastic Load Balancing
/// load balancer An Elemental MediaPackage endpoint An Elemental MediaStore container Any other
/// HTTP server, running on an Amazon EC2 instance or any other kind of host For the current
/// maximum number of origins that you can specify per distribution, see General Quotas on Web
/// Distributions in the Amazon CloudFront Developer Guide (quotas were formerly referred to as
/// limits).
///
/// **AWS API**: `cloudfront.v1.Origin`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/Origin>
///
/// ## Coverage
/// 4 of 12 fields included.
/// Omitted fields:
/// - `OriginPath` — not selected in manifest
/// - `CustomHeaders` — not selected in manifest
/// - `CustomOriginConfig` — not selected in manifest
/// - `VpcOriginConfig` — not selected in manifest
/// - `ConnectionAttempts` — not selected in manifest
/// - `ConnectionTimeout` — not selected in manifest
/// - `ResponseCompletionTimeout` — not selected in manifest
/// - `OriginShield` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Origin {
    /// A unique identifier for the origin. This value must be unique within the distribution.
    /// Use this value to specify the TargetOriginId in a CacheBehavior or DefaultCacheBehavior.
    pub id: String,

    /// The domain name for the origin. For more information, see Origin Domain Name in the
    /// Amazon CloudFront Developer Guide.
    pub domain_name: String,

    /// Use this type to specify an origin that is an Amazon S3 bucket that is not configured
    /// with static website hosting. To specify any other type of origin, including an Amazon S3
    /// bucket that is configured with static website hosting, use the CustomOriginConfig type
    /// instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_origin_config: Option<S3OriginConfig>,

    /// The unique identifier of an origin access control for this origin. For more information,
    /// see Restricting access to an Amazon S3 origin in the Amazon CloudFront Developer Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control_id: Option<String>,
}

impl Origin {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            domain_name: "test-domain_name".into(),
            s3_origin_config: Some(S3OriginConfig::fixture()),
            origin_access_control_id: Some("test-origin_access_control_id".into()),
        }
    }
}

/// A complex type that contains information about the Amazon S3 origin. If the origin is a
/// custom origin or an S3 bucket that is configured as a website endpoint, use the
/// CustomOriginConfig element instead.
///
/// **AWS API**: `cloudfront.v1.S3OriginConfig`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/S3OriginConfig>
///
/// ## Coverage
/// 1 of 2 fields included.
/// Omitted fields:
/// - `OriginReadTimeout` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct S3OriginConfig {
    /// If you're using origin access control (OAC) instead of origin access identity, specify
    /// an empty OriginAccessIdentity element. For more information, see Restricting access to
    /// an Amazon Web Services in the Amazon CloudFront Developer Guide. The CloudFront origin
    /// access identity to associate with the origin. Use an origin access identity to configure
    /// the origin so that viewers can only access objects in an Amazon S3 bucket through
    /// CloudFront. The format of the value is: origin-access-identity/cloudfront/ID-of-origin-
    /// access-identity The ID-of-origin-access-identity is the value that CloudFront returned
    /// in the ID element when you created the origin access identity. If you want viewers to be
    /// able to access objects using either the CloudFront URL or the Amazon S3 URL, specify an
    /// empty OriginAccessIdentity element. To delete the origin access identity from an
    /// existing distribution, update the distribution configuration and include an empty
    /// OriginAccessIdentity element. To replace the origin access identity, update the
    /// distribution configuration and specify the new origin access identity. For more
    /// information about the origin access identity, see Serving Private Content through
    /// CloudFront in the Amazon CloudFront Developer Guide.
    pub origin_access_identity: String,
}

impl S3OriginConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            origin_access_identity: "test-origin_access_identity".into(),
        }
    }
}

/// A complex type that describes the default cache behavior if you don't specify a
/// CacheBehavior element or if request URLs don't match any of the values of PathPattern in
/// CacheBehavior elements. You must create exactly one default cache behavior. If your minimum
/// TTL is greater than 0, CloudFront will cache content for at least the duration specified in
/// the cache policy's minimum TTL, even if the Cache-Control: no-cache, no-store, or private
/// directives are present in the origin headers.
///
/// **AWS API**: `cloudfront.v1.DefaultCacheBehavior`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/DefaultCacheBehavior>
///
/// ## Coverage
/// 2 of 19 fields included.
/// Omitted fields:
/// - `TrustedSigners` — not selected in manifest
/// - `TrustedKeyGroups` — not selected in manifest
/// - `AllowedMethods` — not selected in manifest
/// - `SmoothStreaming` — not selected in manifest
/// - `Compress` — not selected in manifest
/// - `LambdaFunctionAssociations` — not selected in manifest
/// - `FunctionAssociations` — not selected in manifest
/// - `FieldLevelEncryptionId` — not selected in manifest
/// - `RealtimeLogConfigArn` — not selected in manifest
/// - `CachePolicyId` — not selected in manifest
/// - `OriginRequestPolicyId` — not selected in manifest
/// - `ResponseHeadersPolicyId` — not selected in manifest
/// - `GrpcConfig` — not selected in manifest
/// - `ForwardedValues` — not selected in manifest
/// - `MinTTL` — not selected in manifest
/// - `DefaultTTL` — not selected in manifest
/// - `MaxTTL` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DefaultCacheBehavior {
    /// The value of ID for the origin that you want CloudFront to route requests to when they
    /// use the default cache behavior.
    pub target_origin_id: String,

    /// The protocol that viewers can use to access the files in the origin specified by
    /// TargetOriginId when a request matches the path pattern in PathPattern. You can specify
    /// the following options: allow-all: Viewers can use HTTP or HTTPS. redirect-to-https: If a
    /// viewer submits an HTTP request, CloudFront returns an HTTP status code of 301 (Moved
    /// Permanently) to the viewer along with the HTTPS URL. The viewer then resubmits the
    /// request using the new URL. https-only: If a viewer sends an HTTP request, CloudFront
    /// returns an HTTP status code of 403 (Forbidden). For more information about requiring the
    /// HTTPS protocol, see Requiring HTTPS Between Viewers and CloudFront in the Amazon
    /// CloudFront Developer Guide. The only way to guarantee that viewers retrieve an object
    /// that was fetched from the origin using HTTPS is never to use any other protocol to fetch
    /// the object. If you have recently changed from HTTP to HTTPS, we recommend that you clear
    /// your objects' cache because cached objects are protocol agnostic. That means that an
    /// edge location will return an object from the cache regardless of whether the current
    /// request protocol matches the protocol used previously. For more information, see
    /// Managing Cache Expiration in the Amazon CloudFront Developer Guide.
    pub viewer_protocol_policy: String,
}

impl DefaultCacheBehavior {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            target_origin_id: "test-target_origin_id".into(),
            viewer_protocol_policy: "test-viewer_protocol_policy".into(),
        }
    }
}

/// A CloudFront origin access control, including its unique identifier.
///
/// **AWS API**: `cloudfront.v1.OriginAccessControl`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/OriginAccessControl>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OriginAccessControl {
    /// The unique identifier of the origin access control.
    pub id: String,

    /// The origin access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control_config: Option<OriginAccessControlConfig>,
}

impl OriginAccessControl {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            origin_access_control_config: Some(OriginAccessControlConfig::fixture()),
        }
    }
}

/// A CloudFront origin access control configuration.
///
/// **AWS API**: `cloudfront.v1.OriginAccessControlConfig`
/// **Reference**: <https://docs.aws.amazon.com/cloudfront/latest/APIReference/OriginAccessControlConfig>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OriginAccessControlConfig {
    /// A name to identify the origin access control. You can specify up to 64 characters.
    pub name: String,

    /// A description of the origin access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The signing protocol of the origin access control, which determines how CloudFront signs
    /// (authenticates) requests. The only valid value is sigv4.
    pub signing_protocol: String,

    /// Specifies which requests CloudFront signs (adds authentication information to). Specify
    /// always for the most common use case. For more information, see origin access control
    /// advanced settings in the Amazon CloudFront Developer Guide. This field can have one of
    /// the following values: always – CloudFront signs all origin requests, overwriting the
    /// Authorization header from the viewer request if one exists. never – CloudFront doesn't
    /// sign any origin requests. This value turns off origin access control for all origins in
    /// all distributions that use this origin access control. no-override – If the viewer
    /// request doesn't contain the Authorization header, then CloudFront signs the origin
    /// request. If the viewer request contains the Authorization header, then CloudFront
    /// doesn't sign the origin request and instead passes along the Authorization header from
    /// the viewer request. WARNING: To pass along the Authorization header from the viewer
    /// request, you must add the Authorization header to a cache policy for all cache behaviors
    /// that use origins associated with this origin access control.
    pub signing_behavior: String,

    /// The type of origin that this origin access control is for.
    pub origin_access_control_origin_type: String,
}

impl OriginAccessControlConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-name".into(),
            description: Some("test-description".into()),
            signing_protocol: "test-signing_protocol".into(),
            signing_behavior: "test-signing_behavior".into(),
            origin_access_control_origin_type: "test-origin_access_control_origin_type".into(),
        }
    }
}
