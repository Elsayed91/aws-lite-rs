# Amazon CloudFront Usage Examples

## List Distributions

```rust
use aws_lite::AwsHttpClient;

let client = AwsHttpClient::from_default_chain("us-east-1")?;
let cf = client.cloudfront();

let list = cf.list_distributions().await?;
println!("Found {} distributions", list.quantity);

for dist in &list.items {
    println!("  {} — {} ({})", dist.id, dist.domain_name, dist.status);
}
```

## Get Distribution Config

```rust
let config = cf.get_distribution_config("E1ABC2DEF3GHIJ").await?;

println!("Enabled: {}", config.enabled);
println!("Origins: {}", config.origins.quantity);
for origin in &config.origins.items {
    println!("  {} → {}", origin.id, origin.domain_name);
}
```

## Create an Origin Access Control

```rust
use aws_lite::types::cloudfront::OriginAccessControlConfig;

let oac_config = OriginAccessControlConfig {
    name: "my-s3-oac".to_string(),
    description: Some("OAC for S3 bucket".to_string()),
    signing_protocol: "sigv4".to_string(),
    signing_behavior: "always".to_string(),
    origin_access_control_origin_type: "s3".to_string(),
};

let oac = cf.create_origin_access_control(&oac_config).await?;
println!("Created OAC: {}", oac.id);
```

## Testing

```rust
use aws_lite::{AwsHttpClient, mock_client::MockClient};
use aws_lite::test_support::cloudfront_mock_helpers::CloudfrontMockHelpers;

#[tokio::test]
async fn test_list_distributions() {
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
                <Comment>My distribution</Comment>\
              </DistributionSummary>\
            </Items>\
          </DistributionList>"
            .to_vec(),
    );

    let client = AwsHttpClient::from_mock(mock);
    let list = client.cloudfront().list_distributions().await.unwrap();

    assert_eq!(list.quantity, 1);
    assert_eq!(list.items[0].id, "E1ABC2DEF3GHIJ");
}
```
