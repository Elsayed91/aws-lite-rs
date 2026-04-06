# AWS CloudTrail Usage Examples

## Setup

Before creating a trail, the target S3 bucket must have a bucket policy granting CloudTrail write access:

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AWSCloudTrailAclCheck",
      "Effect": "Allow",
      "Principal": { "Service": "cloudtrail.amazonaws.com" },
      "Action": "s3:GetBucketAcl",
      "Resource": "arn:aws:s3:::my-trail-logs"
    },
    {
      "Sid": "AWSCloudTrailWrite",
      "Effect": "Allow",
      "Principal": { "Service": "cloudtrail.amazonaws.com" },
      "Action": "s3:PutObject",
      "Resource": "arn:aws:s3:::my-trail-logs/AWSLogs/123456789012/*",
      "Condition": {
        "StringEquals": { "s3:x-amz-acl": "bucket-owner-full-control" }
      }
    }
  ]
}
```

## Listing Trails

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::cloudtrail::DescribeTrailsRequest;

let client = AwsHttpClient::from_default_chain("us-east-1")?;

// List all trails in the region
let resp = client.cloudtrail().describe_trails(&DescribeTrailsRequest::default()).await?;
for trail in &resp.trail_list {
    println!("Trail: {} ({})", trail.name.as_deref().unwrap_or("?"), trail.trail_arn.as_deref().unwrap_or("?"));
}

// Filter to specific trail(s)
let resp = client.cloudtrail().describe_trails(&DescribeTrailsRequest {
    trail_name_list: vec!["my-trail".to_string()],
    ..Default::default()
}).await?;
```

## Checking Trail Status

```rust
use aws_lite::types::cloudtrail::GetTrailStatusRequest;

let status = client.cloudtrail().get_trail_status(&GetTrailStatusRequest {
    name: "my-trail".to_string(),
}).await?;

println!("Logging: {:?}", status.is_logging);
println!("Last delivery: {:?}", status.latest_delivery_time);
println!("Last notification: {:?}", status.latest_notification_time);
```

## Reading Event Selectors

```rust
use aws_lite::types::cloudtrail::GetEventSelectorsRequest;

let resp = client.cloudtrail().get_event_selectors(&GetEventSelectorsRequest {
    trail_name: "my-trail".to_string(),
}).await?;

for selector in &resp.event_selectors {
    println!("ReadWriteType: {:?}", selector.read_write_type);
    println!("Management events: {:?}", selector.include_management_events);
}
```

## Updating Trail Configuration

```rust
use aws_lite::types::cloudtrail::UpdateTrailRequest;

// Enable multi-region logging
let resp = client.cloudtrail().update_trail(&UpdateTrailRequest {
    name: "my-trail".to_string(),
    is_multi_region_trail: Some(true),
    include_global_service_events: Some(true),
    enable_log_file_validation: Some(true),
    ..Default::default()
}).await?;

println!("Updated trail ARN: {:?}", resp.trail_arn);
```

## Deleting a Trail

```rust
use aws_lite::types::cloudtrail::DeleteTrailRequest;

client.cloudtrail().delete_trail(&DeleteTrailRequest {
    name: "my-trail".to_string(),
}).await?;
```

## Testing with MockClient

```rust
use aws_lite::{AwsHttpClient, MockClient};
use aws_lite::types::cloudtrail::*;

#[tokio::test]
async fn test_describe_trails() {
    let mut mock = MockClient::new();
    mock.expect_post("/").returning_json(serde_json::json!({
        "trailList": [{
            "Name": "my-trail",
            "TrailARN": "arn:aws:cloudtrail:us-east-1:123456789012:trail/my-trail",
            "HomeRegion": "us-east-1",
            "S3BucketName": "my-trail-logs",
            "IsMultiRegionTrail": false
        }]
    }));
    let client = AwsHttpClient::from_mock(mock);
    let result = client.cloudtrail()
        .describe_trails(&DescribeTrailsRequest::default())
        .await
        .unwrap();
    assert_eq!(result.trail_list.len(), 1);
    assert_eq!(result.trail_list[0].name.as_deref(), Some("my-trail"));
}
```
