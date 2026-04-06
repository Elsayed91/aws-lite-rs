//! Amazon CloudFront API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::cloudfront::CloudfrontOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::cloudfront::CloudfrontOps,
    types::cloudfront::{
        Distribution, DistributionConfig, DistributionList, OriginAccessControl,
        OriginAccessControlConfig,
    },
};

/// Client for the Amazon CloudFront API
pub struct CloudfrontClient<'a> {
    ops: CloudfrontOps<'a>,
}

impl<'a> CloudfrontClient<'a> {
    /// Create a new Amazon CloudFront API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: CloudfrontOps::new(client),
        }
    }

    /// List CloudFront distributions.
    pub async fn list_distributions(&self) -> Result<DistributionList> {
        self.ops.list_distributions().await
    }

    /// Get the configuration for a CloudFront distribution.
    pub async fn get_distribution_config(&self, id: &str) -> Result<DistributionConfig> {
        self.ops.get_distribution_config(id).await
    }

    /// Update a CloudFront distribution configuration.
    pub async fn update_distribution(
        &self,
        id: &str,
        body: &DistributionConfig,
    ) -> Result<Distribution> {
        self.ops.update_distribution(id, body).await
    }

    /// Creates a new origin access control in CloudFront.
    pub async fn create_origin_access_control(
        &self,
        body: &OriginAccessControlConfig,
    ) -> Result<OriginAccessControl> {
        self.ops.create_origin_access_control(body).await
    }

    /// Creates a new CloudFront distribution.
    pub async fn create_distribution(&self, body: &DistributionConfig) -> Result<Distribution> {
        self.ops.create_distribution(body).await
    }
}

#[cfg(test)]
mod tests {
    use crate::AwsHttpClient;
    use crate::mock_client::MockClient;
    use crate::test_support::cloudfront_mock_helpers::CloudfrontMockHelpers;

    #[tokio::test]
    async fn list_distributions_returns_parsed_response() {
        let mut mock = MockClient::new();
        mock.expect_list_distributions().returning_bytes(
            b"<DistributionList>\
                <Marker></Marker>\
                <MaxItems>100</MaxItems>\
                <IsTruncated>false</IsTruncated>\
                <Quantity>1</Quantity>\
                <Items>\
                  <DistributionSummary>\
                    <Id>E1ABC2DEF3GHIJ</Id>\
                    <ARN>arn:aws:cloudfront::123456789012:distribution/E1ABC2DEF3GHIJ</ARN>\
                    <Status>Deployed</Status>\
                    <DomainName>d111111abcdef8.cloudfront.net</DomainName>\
                    <Origins><Quantity>1</Quantity></Origins>\
                    <DefaultCacheBehavior>\
                      <TargetOriginId>myS3Origin</TargetOriginId>\
                      <ViewerProtocolPolicy>redirect-to-https</ViewerProtocolPolicy>\
                    </DefaultCacheBehavior>\
                    <PriceClass>PriceClass_100</PriceClass>\
                    <Enabled>true</Enabled>\
                    <Comment>Test distribution</Comment>\
                  </DistributionSummary>\
                </Items>\
              </DistributionList>"
                .to_vec(),
        );

        let client = AwsHttpClient::from_mock(mock);
        let list = client.cloudfront().list_distributions().await.unwrap();

        assert_eq!(list.quantity, 1);
        assert!(!list.is_truncated);
        assert_eq!(list.items.len(), 1);

        let dist = &list.items[0];
        assert_eq!(dist.id, "E1ABC2DEF3GHIJ");
        assert_eq!(
            dist.arn,
            "arn:aws:cloudfront::123456789012:distribution/E1ABC2DEF3GHIJ"
        );
        assert_eq!(dist.status, "Deployed");
        assert_eq!(dist.domain_name, "d111111abcdef8.cloudfront.net");
        assert_eq!(dist.price_class, "PriceClass_100");
        assert!(dist.enabled);
        assert_eq!(
            dist.default_cache_behavior.viewer_protocol_policy,
            "redirect-to-https"
        );
    }

    #[tokio::test]
    async fn list_distributions_empty_returns_zero_items() {
        let mut mock = MockClient::new();
        mock.expect_list_distributions().returning_bytes(
            b"<DistributionList>\
                <Marker></Marker>\
                <MaxItems>100</MaxItems>\
                <IsTruncated>false</IsTruncated>\
                <Quantity>0</Quantity>\
              </DistributionList>"
                .to_vec(),
        );

        let client = AwsHttpClient::from_mock(mock);
        let list = client.cloudfront().list_distributions().await.unwrap();

        assert_eq!(list.quantity, 0);
        assert!(list.items.is_empty());
    }

    #[tokio::test]
    async fn get_distribution_config_returns_parsed_response() {
        let mut mock = MockClient::new();
        mock.expect_get_distribution_config("E1ABC2DEF3GHIJ")
            .returning_bytes(
                b"<DistributionConfig>\
                    <CallerReference>unique-ref-123</CallerReference>\
                    <Origins>\
                      <Quantity>1</Quantity>\
                      <Items>\
                        <Origin>\
                          <Id>myS3Origin</Id>\
                          <DomainName>mybucket.s3.amazonaws.com</DomainName>\
                        </Origin>\
                      </Items>\
                    </Origins>\
                    <DefaultCacheBehavior>\
                      <TargetOriginId>myS3Origin</TargetOriginId>\
                      <ViewerProtocolPolicy>https-only</ViewerProtocolPolicy>\
                    </DefaultCacheBehavior>\
                    <Comment>My CloudFront distribution</Comment>\
                    <Enabled>true</Enabled>\
                  </DistributionConfig>"
                    .to_vec(),
            );

        let client = AwsHttpClient::from_mock(mock);
        let config = client
            .cloudfront()
            .get_distribution_config("E1ABC2DEF3GHIJ")
            .await
            .unwrap();

        assert_eq!(config.caller_reference, "unique-ref-123");
        assert_eq!(config.comment, "My CloudFront distribution");
        assert!(config.enabled);
        assert_eq!(config.origins.quantity, 1);
        assert_eq!(config.origins.items.len(), 1);
        assert_eq!(config.origins.items[0].id, "myS3Origin");
        assert_eq!(
            config.origins.items[0].domain_name,
            "mybucket.s3.amazonaws.com"
        );
        assert_eq!(
            config.default_cache_behavior.viewer_protocol_policy,
            "https-only"
        );
    }

    #[tokio::test]
    async fn get_distribution_config_parses_aliases_default_root_object_viewer_certificate() {
        let mut mock = MockClient::new();
        mock.expect_get_distribution_config("E2XYZ3CUSTOM456")
            .returning_bytes(
                b"<DistributionConfig>\
                    <CallerReference>ref-with-custom-domain</CallerReference>\
                    <Aliases>\
                      <Quantity>2</Quantity>\
                      <Items>\
                        <CNAME>www.example.com</CNAME>\
                        <CNAME>example.com</CNAME>\
                      </Items>\
                    </Aliases>\
                    <DefaultRootObject>index.html</DefaultRootObject>\
                    <Origins>\
                      <Quantity>1</Quantity>\
                    </Origins>\
                    <DefaultCacheBehavior>\
                      <TargetOriginId>myOrigin</TargetOriginId>\
                      <ViewerProtocolPolicy>redirect-to-https</ViewerProtocolPolicy>\
                    </DefaultCacheBehavior>\
                    <Comment>Custom domain distribution</Comment>\
                    <Enabled>true</Enabled>\
                    <ViewerCertificate>\
                      <ACMCertificateArn>arn:aws:acm:us-east-1:123456789012:certificate/abc123</ACMCertificateArn>\
                      <SSLSupportMethod>sni-only</SSLSupportMethod>\
                      <MinimumProtocolVersion>TLSv1.2_2021</MinimumProtocolVersion>\
                      <CloudFrontDefaultCertificate>false</CloudFrontDefaultCertificate>\
                    </ViewerCertificate>\
                  </DistributionConfig>"
                    .to_vec(),
            );

        let client = AwsHttpClient::from_mock(mock);
        let config = client
            .cloudfront()
            .get_distribution_config("E2XYZ3CUSTOM456")
            .await
            .unwrap();

        // Aliases
        let aliases = config.aliases.as_ref().expect("aliases should be present");
        assert_eq!(aliases.quantity, 2);
        assert_eq!(aliases.items, vec!["www.example.com", "example.com"]);

        // DefaultRootObject
        assert_eq!(config.default_root_object.as_deref(), Some("index.html"));

        // ViewerCertificate
        let vc = config
            .viewer_certificate
            .as_ref()
            .expect("viewer_certificate should be present");
        assert_eq!(
            vc.acm_certificate_arn.as_deref(),
            Some("arn:aws:acm:us-east-1:123456789012:certificate/abc123")
        );
        assert_eq!(vc.ssl_support_method.as_deref(), Some("sni-only"));
        assert_eq!(vc.minimum_protocol_version.as_deref(), Some("TLSv1.2_2021"));
        assert_eq!(vc.cloud_front_default_certificate, Some(false));
    }

    #[tokio::test]
    async fn get_distribution_config_uses_cloudfront_default_cert_when_no_custom_domain() {
        let mut mock = MockClient::new();
        mock.expect_get_distribution_config("E3DEFAULT789")
            .returning_bytes(
                b"<DistributionConfig>\
                    <CallerReference>ref-default-cert</CallerReference>\
                    <Origins><Quantity>1</Quantity></Origins>\
                    <DefaultCacheBehavior>\
                      <TargetOriginId>origin</TargetOriginId>\
                      <ViewerProtocolPolicy>allow-all</ViewerProtocolPolicy>\
                    </DefaultCacheBehavior>\
                    <Comment></Comment>\
                    <Enabled>true</Enabled>\
                    <ViewerCertificate>\
                      <CloudFrontDefaultCertificate>true</CloudFrontDefaultCertificate>\
                    </ViewerCertificate>\
                  </DistributionConfig>"
                    .to_vec(),
            );

        let client = AwsHttpClient::from_mock(mock);
        let config = client
            .cloudfront()
            .get_distribution_config("E3DEFAULT789")
            .await
            .unwrap();

        // No aliases or default root object
        assert!(config.aliases.is_none());
        assert!(config.default_root_object.is_none());

        // CloudFront default certificate only
        let vc = config
            .viewer_certificate
            .as_ref()
            .expect("viewer_certificate should be present");
        assert_eq!(vc.cloud_front_default_certificate, Some(true));
        assert!(vc.acm_certificate_arn.is_none());
        assert!(vc.ssl_support_method.is_none());
    }

    #[tokio::test]
    async fn create_origin_access_control_returns_parsed_response() {
        let mut mock = MockClient::new();
        mock.expect_create_origin_access_control().returning_bytes(
            b"<OriginAccessControl>\
                <Id>E1XYZ2ABC3DEFG</Id>\
                <OriginAccessControlConfig>\
                  <Name>test-oac</Name>\
                  <Description>Test OAC</Description>\
                  <SigningProtocol>sigv4</SigningProtocol>\
                  <SigningBehavior>always</SigningBehavior>\
                  <OriginAccessControlOriginType>s3</OriginAccessControlOriginType>\
                </OriginAccessControlConfig>\
              </OriginAccessControl>"
                .to_vec(),
        );

        let client = AwsHttpClient::from_mock(mock);
        let oac_config = crate::types::cloudfront::OriginAccessControlConfig {
            name: "test-oac".to_string(),
            description: Some("Test OAC".to_string()),
            signing_protocol: "sigv4".to_string(),
            signing_behavior: "always".to_string(),
            origin_access_control_origin_type: "s3".to_string(),
        };
        let oac = client
            .cloudfront()
            .create_origin_access_control(&oac_config)
            .await
            .unwrap();

        assert_eq!(oac.id, "E1XYZ2ABC3DEFG");
        let config = oac
            .origin_access_control_config
            .as_ref()
            .expect("OAC config should be present");
        assert_eq!(config.name, "test-oac");
        assert_eq!(config.signing_protocol, "sigv4");
        assert_eq!(config.signing_behavior, "always");
        assert_eq!(config.origin_access_control_origin_type, "s3");
    }
}
