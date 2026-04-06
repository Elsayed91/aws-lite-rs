//! Test support utilities and MockClient helpers.
//!
//! This module provides extension traits for `MockClient` that make test setup more ergonomic.
//! Each API has its own helper trait with `expect_*` methods for ergonomic test setup.
//!
//! # Example
//!
//! ```no_run
//! use aws_lite::MockClient;
//! use aws_lite::test_support::AccessanalyzerMockHelpers;
//!
//! let mut mock = MockClient::new();
//! ```

#[cfg(any(test, feature = "test-support"))]
pub mod accessanalyzer_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod apigateway_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod autoscaling_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod ce_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod cloudfront_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod cloudtrail_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod cloudwatch_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod config_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod dynamodb_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod ec2_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod ecr_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod ecs_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod efs_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod eks_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod elasticache_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod elbv2_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod emr_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod iam_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod kinesis_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod kms_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod lambda_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod logs_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod opensearch_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod organizations_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod rds_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod redshift_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod route53_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod s3_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod sagemaker_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod secretsmanager_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod securityhub_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod sts_mock_helpers;

// Re-export traits for convenience
#[cfg(any(test, feature = "test-support"))]
pub use accessanalyzer_mock_helpers::AccessanalyzerMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use apigateway_mock_helpers::ApigatewayMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use autoscaling_mock_helpers::AutoscalingMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use ce_mock_helpers::CeMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use cloudfront_mock_helpers::CloudfrontMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use cloudtrail_mock_helpers::CloudtrailMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use cloudwatch_mock_helpers::CloudwatchMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use config_mock_helpers::ConfigMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use dynamodb_mock_helpers::DynamodbMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use ec2_mock_helpers::Ec2MockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use ecr_mock_helpers::EcrMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use ecs_mock_helpers::EcsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use efs_mock_helpers::EfsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use eks_mock_helpers::EksMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use elasticache_mock_helpers::ElasticacheMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use elbv2_mock_helpers::Elbv2MockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use emr_mock_helpers::EmrMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use iam_mock_helpers::IamMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use kinesis_mock_helpers::KinesisMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use kms_mock_helpers::KmsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use lambda_mock_helpers::LambdaMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use logs_mock_helpers::LogsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use opensearch_mock_helpers::OpensearchMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use organizations_mock_helpers::OrganizationsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use rds_mock_helpers::RdsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use redshift_mock_helpers::RedshiftMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use route53_mock_helpers::Route53MockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use s3_mock_helpers::S3MockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use sagemaker_mock_helpers::SagemakerMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use secretsmanager_mock_helpers::SecretsmanagerMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use securityhub_mock_helpers::SecurityhubMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use sts_mock_helpers::StsMockHelpers;
