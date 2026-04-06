//! Types for the AWS CloudTrail API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Returns information about the trail.
///
/// **AWS API**: `cloudtrail.v1.DescribeTrailsRequest`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//DescribeTrailsRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeTrailsRequest {
    /// Specifies a list of trail names, trail ARNs, or both, of the trails to describe. The
    /// format of a trail ARN is: arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail If an
    /// empty list is specified, information for the trail in the current Region is returned. If
    /// an empty list is specified and IncludeShadowTrails is false, then information for all
    /// trails in the current Region is returned. If an empty list is specified and
    /// IncludeShadowTrails is null or true, then information for all trails in the current
    /// Region and any associated shadow trails in other Regions is returned. If one or more
    /// trail names are specified, information is returned only if the names match the names of
    /// trails belonging only to the current Region and current account. To return information
    /// about a trail in another Region, you must specify its trail ARN.
    #[serde(rename = "trailNameList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trail_name_list: Vec<String>,

    /// Specifies whether to include shadow trails in the response. A shadow trail is the
    /// replication in a Region of a trail that was created in a different Region, or in the
    /// case of an organization trail, the replication of an organization trail in member
    /// accounts. If you do not include shadow trails, organization trails in a member account
    /// and Region replication trails will not be returned. The default is true.
    #[serde(rename = "includeShadowTrails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shadow_trails: Option<bool>,
}

impl DescribeTrailsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            trail_name_list: vec![],
            include_shadow_trails: Some(false),
        }
    }
}

/// Returns the objects or data listed below if successful. Otherwise, returns an error.
///
/// **AWS API**: `cloudtrail.v1.DescribeTrailsResponse`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//DescribeTrailsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeTrailsResponse {
    /// The list of trail objects. Trail objects with string values are only returned if values
    /// for the objects exist in a trail's configuration. For example, SNSTopicName and
    /// SNSTopicARN are only returned in results if a trail is configured to send SNS
    /// notifications. Similarly, KMSKeyId only appears in results if a trail's log files are
    /// encrypted with KMS customer managed keys.
    #[serde(rename = "trailList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trail_list: Vec<Trail>,
}

impl DescribeTrailsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { trail_list: vec![] }
    }
}

/// The settings for a trail.
///
/// **AWS API**: `cloudtrail.v1.Trail`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//Trail>
///
/// ## Coverage
/// 15 of 16 fields included.
/// Omitted fields:
/// - `SnsTopicName` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Trail {
    /// Name of the trail set by calling CreateTrail. The maximum length is 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Name of the Amazon S3 bucket into which CloudTrail delivers your trail files. See Amazon
    /// S3 Bucket naming rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,

    /// Specifies the Amazon S3 key prefix that comes after the name of the bucket you have
    /// designated for log file delivery. For more information, see Finding Your CloudTrail Log
    /// Files. The maximum length is 200 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,

    /// Specifies the ARN of the Amazon SNS topic that CloudTrail uses to send notifications
    /// when log files are delivered. The following is the format of a topic ARN.
    /// arn:aws:sns:us-east-2:123456789012:MyTopic
    #[serde(rename = "SnsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,

    /// Set to True to include Amazon Web Services API calls from Amazon Web Services global
    /// services such as IAM. Otherwise, False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,

    /// Specifies whether the trail exists only in one Region or exists in all Regions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,

    /// The Region in which the trail was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,

    /// Specifies the ARN of the trail. The following is the format of a trail ARN.
    /// arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,

    /// Specifies whether log file validation is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_validation_enabled: Option<bool>,

    /// Specifies an Amazon Resource Name (ARN), a unique identifier that represents the log
    /// group to which CloudTrail logs will be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,

    /// Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log
    /// group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,

    /// Specifies the KMS key ID that encrypts the logs and digest files delivered by
    /// CloudTrail. The value is a fully specified ARN to a KMS key in the following format.
    /// arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    /// Specifies if the trail has custom event selectors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_custom_event_selectors: Option<bool>,

    /// Specifies whether a trail has insight types specified in an InsightSelector list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_insight_selectors: Option<bool>,

    /// Specifies whether the trail is an organization trail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
}

impl Trail {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-name".into()),
            s3_bucket_name: Some("test-s3_bucket_name".into()),
            s3_key_prefix: Some("test-s3_key_prefix".into()),
            sns_topic_arn: Some("test-sns_topic_arn".into()),
            include_global_service_events: Some(false),
            is_multi_region_trail: Some(false),
            home_region: Some("test-home_region".into()),
            trail_arn: Some("test-trail_arn".into()),
            log_file_validation_enabled: Some(false),
            cloud_watch_logs_log_group_arn: Some("test-cloud_watch_logs_log_group_arn".into()),
            cloud_watch_logs_role_arn: Some("test-cloud_watch_logs_role_arn".into()),
            kms_key_id: Some("test-kms_key_id".into()),
            has_custom_event_selectors: Some(false),
            has_insight_selectors: Some(false),
            is_organization_trail: Some(false),
        }
    }
}

/// The name of a trail about which you want the current status.
///
/// **AWS API**: `cloudtrail.v1.GetTrailStatusRequest`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//GetTrailStatusRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTrailStatusRequest {
    /// Specifies the name or the CloudTrail ARN of the trail for which you are requesting
    /// status. To get the status of a shadow trail (a replication of the trail in another
    /// Region), you must specify its ARN. The following is the format of a trail ARN:
    /// arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail If the trail is an organization
    /// trail and you are a member account in the organization in Organizations, you must
    /// provide the full ARN of that trail, and not just the name.
    pub name: String,
}

impl GetTrailStatusRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-name".into(),
        }
    }
}

/// Returns the objects or data listed below if successful. Otherwise, returns an error.
///
/// **AWS API**: `cloudtrail.v1.GetTrailStatusResponse`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//GetTrailStatusResponse>
///
/// ## Coverage
/// 11 of 17 fields included.
/// Omitted fields:
/// - `LatestDeliveryAttemptTime` — not selected in manifest
/// - `LatestNotificationAttemptTime` — not selected in manifest
/// - `LatestNotificationAttemptSucceeded` — not selected in manifest
/// - `LatestDeliveryAttemptSucceeded` — not selected in manifest
/// - `TimeLoggingStarted` — not selected in manifest
/// - `TimeLoggingStopped` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTrailStatusResponse {
    /// Whether the CloudTrail trail is currently logging Amazon Web Services API calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_logging: Option<bool>,

    /// Displays any Amazon S3 error that CloudTrail encountered when attempting to deliver log
    /// files to the designated bucket. For more information, see Error Responses in the Amazon
    /// S3 API Reference. This error occurs only when there is a problem with the destination S3
    /// bucket, and does not occur for requests that time out. To resolve the issue, fix the
    /// bucket policy so that CloudTrail can write to the bucket; or create a new bucket and
    /// call UpdateTrail to specify the new bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_error: Option<String>,

    /// Displays any Amazon SNS error that CloudTrail encountered when attempting to send a
    /// notification. For more information about Amazon SNS errors, see the Amazon SNS Developer
    /// Guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_notification_error: Option<String>,

    /// Specifies the date and time that CloudTrail last delivered log files to an account's
    /// Amazon S3 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_time: Option<f64>,

    /// Specifies the date and time of the most recent Amazon SNS notification that CloudTrail
    /// has written a new log file to an account's Amazon S3 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_notification_time: Option<f64>,

    /// Specifies the most recent date and time when CloudTrail started recording API calls for
    /// an Amazon Web Services account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_logging_time: Option<f64>,

    /// Specifies the most recent date and time when CloudTrail stopped recording API calls for
    /// an Amazon Web Services account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_logging_time: Option<f64>,

    /// Displays any CloudWatch Logs error that CloudTrail encountered when attempting to
    /// deliver logs to CloudWatch Logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_cloud_watch_logs_delivery_error: Option<String>,

    /// Displays the most recent date and time when CloudTrail delivered logs to CloudWatch
    /// Logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_cloud_watch_logs_delivery_time: Option<f64>,

    /// Specifies the date and time that CloudTrail last delivered a digest file to an account's
    /// Amazon S3 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_digest_delivery_time: Option<f64>,

    /// Displays any Amazon S3 error that CloudTrail encountered when attempting to deliver a
    /// digest file to the designated bucket. For more information, see Error Responses in the
    /// Amazon S3 API Reference. This error occurs only when there is a problem with the
    /// destination S3 bucket, and does not occur for requests that time out. To resolve the
    /// issue, fix the bucket policy so that CloudTrail can write to the bucket; or create a new
    /// bucket and call UpdateTrail to specify the new bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_digest_delivery_error: Option<String>,
}

impl GetTrailStatusResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            is_logging: Some(false),
            latest_delivery_error: Some("test-latest_delivery_error".into()),
            latest_notification_error: Some("test-latest_notification_error".into()),
            latest_cloud_watch_logs_delivery_error: Some(
                "test-latest_cloud_watch_logs_delivery_error".into(),
            ),
            latest_digest_delivery_error: Some("test-latest_digest_delivery_error".into()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `cloudtrail.v1.GetEventSelectorsRequest`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//GetEventSelectorsRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetEventSelectorsRequest {
    /// Specifies the name of the trail or trail ARN. If you specify a trail name, the string
    /// must meet the following requirements: Contain only ASCII letters (a-z, A-Z), numbers
    /// (0-9), periods (.), underscores (_), or dashes (-) Start with a letter or number, and
    /// end with a letter or number Be between 3 and 128 characters Have no adjacent periods,
    /// underscores or dashes. Names like my-_namespace and my--namespace are not valid. Not be
    /// in IP address format (for example, 192.168.5.4) If you specify a trail ARN, it must be
    /// in the format: arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail
    pub trail_name: String,
}

impl GetEventSelectorsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            trail_name: "test-trail_name".into(),
        }
    }
}

///
/// **AWS API**: `cloudtrail.v1.GetEventSelectorsResponse`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//GetEventSelectorsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetEventSelectorsResponse {
    /// The specified trail ARN that has the event selectors.
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,

    /// The event selectors that are configured for the trail.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event_selectors: Vec<EventSelector>,

    /// The advanced event selectors that are configured for the trail.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub advanced_event_selectors: Vec<AdvancedEventSelector>,
}

impl GetEventSelectorsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            trail_arn: Some("test-trail_arn".into()),
            event_selectors: vec![],
            advanced_event_selectors: vec![],
        }
    }
}

/// Use event selectors to further specify the management and data event settings for your
/// trail. By default, trails created without specific event selectors will be configured to log
/// all read and write management events, and no data events. When an event occurs in your
/// account, CloudTrail evaluates the event selector for all trails. For each trail, if the
/// event matches any event selector, the trail processes and logs the event. If the event
/// doesn't match any event selector, the trail doesn't log the event. You can configure up to
/// five event selectors for a trail. You cannot apply both event selectors and advanced event
/// selectors to a trail.
///
/// **AWS API**: `cloudtrail.v1.EventSelector`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//EventSelector>
///
/// ## Coverage
/// 3 of 4 fields included.
/// Omitted fields:
/// - `ExcludeManagementEventSources` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventSelector {
    /// Specify if you want your trail to log read-only events, write-only events, or all. For
    /// example, the EC2 GetConsoleOutput is a read-only API operation and RunInstances is a
    /// write-only API operation. By default, the value is All.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_write_type: Option<String>,

    /// Specify if you want your event selector to include management events for your trail. For
    /// more information, see Management Events in the CloudTrail User Guide. By default, the
    /// value is true. The first copy of management events is free. You are charged for
    /// additional copies of management events that you are logging on any subsequent trail in
    /// the same Region. For more information about CloudTrail pricing, see CloudTrail Pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_management_events: Option<bool>,

    /// CloudTrail supports data event logging for Amazon S3 objects in standard S3 buckets,
    /// Lambda functions, and Amazon DynamoDB tables with basic event selectors. You can specify
    /// up to 250 resources for an individual event selector, but the total number of data
    /// resources cannot exceed 250 across all event selectors in a trail. This limit does not
    /// apply if you configure resource logging for all data events. For more information, see
    /// Data Events and Limits in CloudTrail in the CloudTrail User Guide. To log data events
    /// for all other resource types including objects stored in directory buckets, you must use
    /// AdvancedEventSelectors. You must also use AdvancedEventSelectors if you want to filter
    /// on the eventName field.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data_resources: Vec<DataResource>,
}

impl EventSelector {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            read_write_type: Some("test-read_write_type".into()),
            include_management_events: Some(false),
            data_resources: vec![],
        }
    }
}

/// You can configure the DataResource in an EventSelector to log data events for the following
/// three resource types: AWS::DynamoDB::Table AWS::Lambda::Function AWS::S3::Object To log data
/// events for all other resource types including objects stored in directory buckets, you must
/// use AdvancedEventSelectors. You must also use AdvancedEventSelectors if you want to filter
/// on the eventName field. Configure the DataResource to specify the resource type and resource
/// ARNs for which you want to log data events. The total number of allowed data resources is
/// 250. This number can be distributed between 1 and 5 event selectors, but the total cannot
/// exceed 250 across all selectors for the trail. The following example demonstrates how
/// logging works when you configure logging of all data events for a general purpose bucket
/// named amzn-s3-demo-bucket1. In this example, the CloudTrail user specified an empty prefix,
/// and the option to log both Read and Write data events. A user uploads an image file to
/// amzn-s3-demo-bucket1. The PutObject API operation is an Amazon S3 object-level API. It is
/// recorded as a data event in CloudTrail. Because the CloudTrail user specified an S3 bucket
/// with an empty prefix, events that occur on any object in that bucket are logged. The trail
/// processes and logs the event. A user uploads an object to an Amazon S3 bucket named
/// arn:aws:s3:::amzn-s3-demo-bucket1. The PutObject API operation occurred for an object in an
/// S3 bucket that the CloudTrail user didn't specify for the trail. The trail doesn’t log the
/// event. The following example demonstrates how logging works when you configure logging of
/// Lambda data events for a Lambda function named MyLambdaFunction, but not for all Lambda
/// functions. A user runs a script that includes a call to the MyLambdaFunction function and
/// the MyOtherLambdaFunction function. The Invoke API operation on MyLambdaFunction is an
/// Lambda API. It is recorded as a data event in CloudTrail. Because the CloudTrail user
/// specified logging data events for MyLambdaFunction, any invocations of that function are
/// logged. The trail processes and logs the event. The Invoke API operation on
/// MyOtherLambdaFunction is an Lambda API. Because the CloudTrail user did not specify logging
/// data events for all Lambda functions, the Invoke operation for MyOtherLambdaFunction does
/// not match the function specified for the trail. The trail doesn’t log the event.
///
/// **AWS API**: `cloudtrail.v1.DataResource`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//DataResource>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataResource {
    /// The resource type in which you want to log data events. You can specify the following
    /// basic event selector resource types: AWS::DynamoDB::Table AWS::Lambda::Function
    /// AWS::S3::Object Additional resource types are available through advanced event
    /// selectors. For more information, see AdvancedEventSelector.
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// An array of Amazon Resource Name (ARN) strings or partial ARN strings for the specified
    /// resource type. To log data events for all objects in all S3 buckets in your Amazon Web
    /// Services account, specify the prefix as arn:aws:s3. This also enables logging of data
    /// event activity performed by any user or role in your Amazon Web Services account, even
    /// if that activity is performed on a bucket that belongs to another Amazon Web Services
    /// account. To log data events for all objects in an S3 bucket, specify the bucket and an
    /// empty object prefix such as arn:aws:s3:::amzn-s3-demo-bucket1/. The trail logs data
    /// events for all objects in this S3 bucket. To log data events for specific objects,
    /// specify the S3 bucket and object prefix such as arn:aws:s3:::amzn-s3-demo-
    /// bucket1/example-images. The trail logs data events for objects in this S3 bucket that
    /// match the prefix. To log data events for all Lambda functions in your Amazon Web
    /// Services account, specify the prefix as arn:aws:lambda. This also enables logging of
    /// Invoke activity performed by any user or role in your Amazon Web Services account, even
    /// if that activity is performed on a function that belongs to another Amazon Web Services
    /// account. To log data events for a specific Lambda function, specify the function ARN.
    /// Lambda function ARNs are exact. For example, if you specify a function ARN
    /// arn:aws:lambda:us-west-2:111111111111:function:helloworld, data events will only be
    /// logged for arn:aws:lambda:us-west-2:111111111111:function:helloworld. They will not be
    /// logged for arn:aws:lambda:us-west-2:111111111111:function:helloworld2. To log data
    /// events for all DynamoDB tables in your Amazon Web Services account, specify the prefix
    /// as arn:aws:dynamodb.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

impl DataResource {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            r#type: Some("test-type".into()),
            values: vec![],
        }
    }
}

/// Advanced event selectors let you create fine-grained selectors for CloudTrail management,
/// data, and network activity events. They help you control costs by logging only those events
/// that are important to you. For more information about configuring advanced event selectors,
/// see the Logging data events, Logging network activity events, and Logging management events
/// topics in the CloudTrail User Guide. You cannot apply both event selectors and advanced
/// event selectors to a trail. For information about configurable advanced event selector
/// fields, see AdvancedEventSelector in the CloudTrail API Reference.
///
/// **AWS API**: `cloudtrail.v1.AdvancedEventSelector`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//AdvancedEventSelector>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdvancedEventSelector {
    /// An optional, descriptive name for an advanced event selector, such as "Log data events
    /// for only two S3 buckets".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Contains all selector statements in an advanced event selector.
    #[serde(default)]
    pub field_selectors: Vec<AdvancedFieldSelector>,
}

impl AdvancedEventSelector {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-name".into()),
            field_selectors: vec![],
        }
    }
}

/// A single selector statement in an advanced event selector.
///
/// **AWS API**: `cloudtrail.v1.AdvancedFieldSelector`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//AdvancedFieldSelector>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdvancedFieldSelector {
    /// A field in a CloudTrail event record on which to filter events to be logged. For event
    /// data stores for CloudTrail Insights events, Config configuration items, Audit Manager
    /// evidence, or events outside of Amazon Web Services, the field is used only for selecting
    /// events as filtering is not supported. For more information, see AdvancedFieldSelector in
    /// the CloudTrail API Reference. Selectors don't support the use of wildcards like * . To
    /// match multiple values with a single condition, you may use StartsWith, EndsWith,
    /// NotStartsWith, or NotEndsWith to explicitly match the beginning or end of the event
    /// field.
    pub field: String,

    /// An operator that includes events that match the exact value of the event record field
    /// specified as the value of Field. This is the only valid operator that you can use with
    /// the readOnly, eventCategory, and resources.type fields.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub equals: Vec<String>,

    /// An operator that includes events that match the first few characters of the event record
    /// field specified as the value of Field.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub starts_with: Vec<String>,

    /// An operator that includes events that match the last few characters of the event record
    /// field specified as the value of Field.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ends_with: Vec<String>,

    /// An operator that excludes events that match the exact value of the event record field
    /// specified as the value of Field.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub not_equals: Vec<String>,

    /// An operator that excludes events that match the first few characters of the event record
    /// field specified as the value of Field.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub not_starts_with: Vec<String>,

    /// An operator that excludes events that match the last few characters of the event record
    /// field specified as the value of Field.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub not_ends_with: Vec<String>,
}

impl AdvancedFieldSelector {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            field: "test-field".into(),
            equals: vec![],
            starts_with: vec![],
            ends_with: vec![],
            not_equals: vec![],
            not_starts_with: vec![],
            not_ends_with: vec![],
        }
    }
}

/// The request that specifies the name of a trail to delete.
///
/// **AWS API**: `cloudtrail.v1.DeleteTrailRequest`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//DeleteTrailRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTrailRequest {
    /// Specifies the name or the CloudTrail ARN of the trail to be deleted. The following is
    /// the format of a trail ARN. arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail
    pub name: String,
}

impl DeleteTrailRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-name".into(),
        }
    }
}

/// Returns the objects or data listed below if successful. Otherwise, returns an error.
///
/// **AWS API**: `cloudtrail.v1.DeleteTrailResponse`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//DeleteTrailResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTrailResponse {}

impl DeleteTrailResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}

/// Specifies settings to update for the trail.
///
/// **AWS API**: `cloudtrail.v1.UpdateTrailRequest`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//UpdateTrailRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTrailRequest {
    /// Specifies the name of the trail or trail ARN. If Name is a trail name, the string must
    /// meet the following requirements: Contain only ASCII letters (a-z, A-Z), numbers (0-9),
    /// periods (.), underscores (_), or dashes (-) Start with a letter or number, and end with
    /// a letter or number Be between 3 and 128 characters Have no adjacent periods, underscores
    /// or dashes. Names like my-_namespace and my--namespace are not valid. Not be in IP
    /// address format (for example, 192.168.5.4) If Name is a trail ARN, it must be in the
    /// following format. arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail
    pub name: String,

    /// Specifies the name of the Amazon S3 bucket designated for publishing log files. See
    /// Amazon S3 Bucket naming rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,

    /// Specifies the Amazon S3 key prefix that comes after the name of the bucket you have
    /// designated for log file delivery. For more information, see Finding Your CloudTrail Log
    /// Files. The maximum length is 200 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,

    /// Specifies the name or ARN of the Amazon SNS topic defined for notification of log file
    /// delivery. The maximum length is 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,

    /// Specifies whether the trail is publishing events from global services such as IAM to the
    /// log files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,

    /// Specifies whether the trail applies only to the current Region or to all Regions. The
    /// default is false. If the trail exists only in the current Region and this value is set
    /// to true, shadow trails (replications of the trail) will be created in the other Regions.
    /// If the trail exists in all Regions and this value is set to false, the trail will remain
    /// in the Region where it was created, and its shadow trails in other Regions will be
    /// deleted. As a best practice, consider using trails that log events in all Regions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,

    /// Specifies whether log file validation is enabled. The default is false. When you disable
    /// log file integrity validation, the chain of digest files is broken after one hour.
    /// CloudTrail does not create digest files for log files that were delivered during a
    /// period in which log file integrity validation was disabled. For example, if you enable
    /// log file integrity validation at noon on January 1, disable it at noon on January 2, and
    /// re-enable it at noon on January 10, digest files will not be created for the log files
    /// delivered from noon on January 2 to noon on January 10. The same applies whenever you
    /// stop CloudTrail logging or delete a trail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_log_file_validation: Option<bool>,

    /// Specifies a log group name using an Amazon Resource Name (ARN), a unique identifier that
    /// represents the log group to which CloudTrail logs are delivered. You must use a log
    /// group that exists in your account. Not required unless you specify
    /// CloudWatchLogsRoleArn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,

    /// Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log
    /// group. You must use a role that exists in your account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,

    /// Specifies the KMS key ID to use to encrypt the logs and digest files delivered by
    /// CloudTrail. The value can be an alias name prefixed by "alias/", a fully specified ARN
    /// to an alias, a fully specified ARN to a key, or a globally unique identifier. CloudTrail
    /// also supports KMS multi-Region keys. For more information about multi-Region keys, see
    /// Using multi-Region keys in the Key Management Service Developer Guide. Examples:
    /// alias/MyAliasName arn:aws:kms:us-east-2:123456789012:alias/MyAliasName arn:aws:kms:us-
    /// east-2:123456789012:key/12345678-1234-1234-1234-123456789012
    /// 12345678-1234-1234-1234-123456789012
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    /// Specifies whether the trail is applied to all accounts in an organization in
    /// Organizations, or only for the current Amazon Web Services account. The default is
    /// false, and cannot be true unless the call is made on behalf of an Amazon Web Services
    /// account that is the management account for an organization in Organizations. If the
    /// trail is not an organization trail and this is set to true, the trail will be created in
    /// all Amazon Web Services accounts that belong to the organization. If the trail is an
    /// organization trail and this is set to false, the trail will remain in the current Amazon
    /// Web Services account but be deleted from all member accounts in the organization. Only
    /// the management account for the organization can convert an organization trail to a non-
    /// organization trail, or convert a non-organization trail to an organization trail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
}

impl UpdateTrailRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-name".into(),
            s3_bucket_name: Some("test-s3_bucket_name".into()),
            s3_key_prefix: Some("test-s3_key_prefix".into()),
            sns_topic_name: Some("test-sns_topic_name".into()),
            include_global_service_events: Some(false),
            is_multi_region_trail: Some(false),
            enable_log_file_validation: Some(false),
            cloud_watch_logs_log_group_arn: Some("test-cloud_watch_logs_log_group_arn".into()),
            cloud_watch_logs_role_arn: Some("test-cloud_watch_logs_role_arn".into()),
            kms_key_id: Some("test-kms_key_id".into()),
            is_organization_trail: Some(false),
        }
    }
}

/// Returns the objects or data listed below if successful. Otherwise, returns an error.
///
/// **AWS API**: `cloudtrail.v1.UpdateTrailResponse`
/// **Reference**: <https://docs.aws.amazon.com/awscloudtrail/latest/APIReference//UpdateTrailResponse>
///
/// ## Coverage
/// 12 of 13 fields included.
/// Omitted fields:
/// - `SnsTopicName` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTrailResponse {
    /// Specifies the name of the trail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Specifies the name of the Amazon S3 bucket designated for publishing log files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,

    /// Specifies the Amazon S3 key prefix that comes after the name of the bucket you have
    /// designated for log file delivery. For more information, see Finding Your IAM Log Files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,

    /// Specifies the ARN of the Amazon SNS topic that CloudTrail uses to send notifications
    /// when log files are delivered. The following is the format of a topic ARN.
    /// arn:aws:sns:us-east-2:123456789012:MyTopic
    #[serde(rename = "SnsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,

    /// Specifies whether the trail is publishing events from global services such as IAM to the
    /// log files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,

    /// Specifies whether the trail exists in one Region or in all Regions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,

    /// Specifies the ARN of the trail that was updated. The following is the format of a trail
    /// ARN. arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,

    /// Specifies whether log file integrity validation is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_validation_enabled: Option<bool>,

    /// Specifies the Amazon Resource Name (ARN) of the log group to which CloudTrail logs are
    /// delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,

    /// Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log
    /// group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,

    /// Specifies the KMS key ID that encrypts the logs and digest files delivered by
    /// CloudTrail. The value is a fully specified ARN to a KMS key in the following format.
    /// arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    /// Specifies whether the trail is an organization trail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
}

impl UpdateTrailResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-name".into()),
            s3_bucket_name: Some("test-s3_bucket_name".into()),
            s3_key_prefix: Some("test-s3_key_prefix".into()),
            sns_topic_arn: Some("test-sns_topic_arn".into()),
            include_global_service_events: Some(false),
            is_multi_region_trail: Some(false),
            trail_arn: Some("test-trail_arn".into()),
            log_file_validation_enabled: Some(false),
            cloud_watch_logs_log_group_arn: Some("test-cloud_watch_logs_log_group_arn".into()),
            cloud_watch_logs_role_arn: Some("test-cloud_watch_logs_role_arn".into()),
            kms_key_id: Some("test-kms_key_id".into()),
            is_organization_trail: Some(false),
        }
    }
}
