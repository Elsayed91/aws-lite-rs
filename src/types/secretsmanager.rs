//! Types for the AWS Secrets Manager API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `secretsmanager.v1.ListSecretsRequest`
///
/// ## Coverage
/// 4 of 6 fields included.
/// Omitted fields:
/// - `IncludePlannedDeletion` — not selected in manifest
/// - `Filters` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListSecretsRequest {
    /// The number of results to include in the response. If there are more results available,
    /// in the response, Secrets Manager includes NextToken. To get the next results, call
    /// ListSecrets again with the value from NextToken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,

    /// A token that indicates where the output should continue from, if a previous call did not
    /// show all results. To get the next results, call ListSecrets again with this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// Secrets are listed by CreatedDate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,

    /// If not specified, secrets are listed by CreatedDate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListSecretsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            max_results: Some(100),
            next_token: Some("test-next_token".into()),
            sort_order: Some("test-sort_order".into()),
            sort_by: Some("test-sort_by".into()),
        }
    }
}

///
/// **AWS API**: `secretsmanager.v1.ListSecretsResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListSecretsResponse {
    /// A list of the secrets in the account.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub secret_list: Vec<SecretListEntry>,

    /// Secrets Manager includes this value if there's more output available than what is
    /// included in the current response. This can occur even when the response includes no
    /// values at all, such as when you ask for a filtered view of a long list. To get the next
    /// results, call ListSecrets again with this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListSecretsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            secret_list: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// A structure that contains the details about a secret. It does not include the encrypted
/// SecretString and SecretBinary values. To get those values, use GetSecretValue .
///
/// **AWS API**: `secretsmanager.v1.SecretListEntry`
///
/// ## Coverage
/// 10 of 20 fields included.
/// Omitted fields:
/// - `Type` — not selected in manifest
/// - `KmsKeyId` — not selected in manifest
/// - `RotationLambdaARN` — not selected in manifest
/// - `ExternalSecretRotationMetadata` — not selected in manifest
/// - `ExternalSecretRotationRoleArn` — not selected in manifest
/// - `NextRotationDate` — not selected in manifest
/// - `Tags` — not selected in manifest
/// - `SecretVersionsToStages` — not selected in manifest
/// - `OwningService` — not selected in manifest
/// - `PrimaryRegion` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretListEntry {
    /// The Amazon Resource Name (ARN) of the secret.
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The friendly name of the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The user-provided description of the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Indicates whether automatic, scheduled rotation is enabled for this secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_enabled: Option<bool>,

    /// A structure that defines the rotation configuration for the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,

    /// The most recent date and time that the Secrets Manager rotation process was successfully
    /// completed. This value is null if the secret hasn't ever rotated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotated_date: Option<f64>,

    /// The last date and time that this secret was modified in any way.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_date: Option<f64>,

    /// The date that the secret was last accessed in the Region. This field is omitted if the
    /// secret has never been retrieved in the Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<f64>,

    /// The date and time when a secret was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,

    /// The date and time the deletion of the secret occurred. Not present on active secrets.
    /// The secret can be recovered until the number of days in the recovery window has passed,
    /// as specified in the RecoveryWindowInDays parameter of the DeleteSecret operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<f64>,
}

impl SecretListEntry {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            arn: Some("test-arn".into()),
            name: Some("test-name".into()),
            description: Some("test-description".into()),
            rotation_enabled: Some(false),
            rotation_rules: Some(RotationRulesType::fixture()),
            ..Default::default()
        }
    }
}

/// A structure that defines the rotation configuration for the secret.
///
/// **AWS API**: `secretsmanager.v1.RotationRulesType`
///
/// ## Coverage
/// 2 of 3 fields included.
/// Omitted fields:
/// - `Duration` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RotationRulesType {
    /// The number of days between rotations of the secret. You can use this value to check that
    /// your secret meets your compliance guidelines for how often secrets must be rotated. If
    /// you use this field to set the rotation schedule, Secrets Manager calculates the next
    /// rotation date based on the previous rotation. Manually updating the secret value by
    /// calling PutSecretValue or UpdateSecret is considered a valid rotation. In DescribeSecret
    /// and ListSecrets, this value is calculated from the rotation schedule after every
    /// successful rotation. In RotateSecret, you can set the rotation schedule in RotationRules
    /// with AutomaticallyAfterDays or ScheduleExpression, but not both. To set a rotation
    /// schedule in hours, use ScheduleExpression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_after_days: Option<i64>,

    /// A cron() or rate() expression that defines the schedule for rotating your secret.
    /// Secrets Manager rotation schedules use UTC time zone. Secrets Manager rotates your
    /// secret any time during a rotation window. Secrets Manager rate() expressions represent
    /// the interval in hours or days that you want to rotate your secret, for example rate(12
    /// hours) or rate(10 days). You can rotate a secret as often as every four hours. If you
    /// use a rate() expression, the rotation window starts at midnight. For a rate in hours,
    /// the default rotation window closes after one hour. For a rate in days, the default
    /// rotation window closes at the end of the day. You can set the Duration to change the
    /// rotation window. The rotation window must not extend into the next UTC day or into the
    /// next rotation window. You can use a cron() expression to create a rotation schedule that
    /// is more detailed than a rotation interval. For more information, including examples, see
    /// Schedule expressions in Secrets Manager rotation in the Secrets Manager Users Guide. For
    /// a cron expression that represents a schedule in hours, the default rotation window
    /// closes after one hour. For a cron expression that represents a schedule in days, the
    /// default rotation window closes at the end of the day. You can set the Duration to change
    /// the rotation window. The rotation window must not extend into the next UTC day or into
    /// the next rotation window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
}

impl RotationRulesType {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            automatically_after_days: Some(100),
            schedule_expression: Some("test-schedule_expression".into()),
        }
    }
}

///
/// **AWS API**: `secretsmanager.v1.DeleteSecretRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSecretRequest {
    /// The ARN or name of the secret to delete. For an ARN, we recommend that you specify a
    /// complete ARN rather than a partial ARN. See Finding a secret from a partial ARN.
    pub secret_id: String,

    /// The number of days from 7 to 30 that Secrets Manager waits before permanently deleting
    /// the secret. You can't use both this parameter and ForceDeleteWithoutRecovery in the same
    /// call. If you don't use either, then by default Secrets Manager uses a 30 day recovery
    /// window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window_in_days: Option<i64>,

    /// Specifies whether to delete the secret without any recovery window. You can't use both
    /// this parameter and RecoveryWindowInDays in the same call. If you don't use either, then
    /// by default Secrets Manager uses a 30 day recovery window. Secrets Manager performs the
    /// actual deletion with an asynchronous background process, so there might be a short delay
    /// before the secret is permanently deleted. If you delete a secret and then immediately
    /// create a secret with the same name, use appropriate back off and retry logic. If you
    /// forcibly delete an already deleted or nonexistent secret, the operation does not return
    /// ResourceNotFoundException. Use this parameter with caution. This parameter causes the
    /// operation to skip the normal recovery window before the permanent deletion that Secrets
    /// Manager would normally impose with the RecoveryWindowInDays parameter. If you delete a
    /// secret with the ForceDeleteWithoutRecovery parameter, then you have no opportunity to
    /// recover the secret. You lose the secret permanently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_without_recovery: Option<bool>,
}

impl DeleteSecretRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            secret_id: "test-secret_id".into(),
            recovery_window_in_days: Some(100),
            force_delete_without_recovery: Some(false),
        }
    }
}

///
/// **AWS API**: `secretsmanager.v1.DeleteSecretResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSecretResponse {
    /// The ARN of the secret.
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The name of the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The date and time after which this secret Secrets Manager can permanently delete this
    /// secret, and it can no longer be restored. This value is the date and time of the delete
    /// request plus the number of days in RecoveryWindowInDays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
}

impl DeleteSecretResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            arn: Some("test-arn".into()),
            name: Some("test-name".into()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `secretsmanager.v1.RotateSecretRequest`
///
/// ## Coverage
/// 4 of 7 fields included.
/// Omitted fields:
/// - `ClientRequestToken` — not selected in manifest
/// - `ExternalSecretRotationMetadata` — not selected in manifest
/// - `ExternalSecretRotationRoleArn` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RotateSecretRequest {
    /// The ARN or name of the secret to rotate. For an ARN, we recommend that you specify a
    /// complete ARN rather than a partial ARN. See Finding a secret from a partial ARN.
    pub secret_id: String,

    /// For secrets that use a Lambda rotation function to rotate, the ARN of the Lambda
    /// rotation function. For secrets that use managed rotation, omit this field. For more
    /// information, see Managed rotation in the Secrets Manager User Guide.
    #[serde(rename = "RotationLambdaARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_arn: Option<String>,

    /// A structure that defines the rotation configuration for this secret. When changing an
    /// existing rotation schedule and setting RotateImmediately to false: If using
    /// AutomaticallyAfterDays or a ScheduleExpression with rate(), the previously scheduled
    /// rotation might still occur. To prevent unintended rotations, use a ScheduleExpression
    /// with cron() for granular control over rotation windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,

    /// Specifies whether to rotate the secret immediately or wait until the next scheduled
    /// rotation window. The rotation schedule is defined in RotateSecretRequest$RotationRules.
    /// The default for RotateImmediately is true. If you don't specify this value, Secrets
    /// Manager rotates the secret immediately. If you set RotateImmediately to false, Secrets
    /// Manager tests the rotation configuration by running the testSecret step of the Lambda
    /// rotation function. This test creates an AWSPENDING version of the secret and then
    /// removes it. When changing an existing rotation schedule and setting RotateImmediately to
    /// false: If using AutomaticallyAfterDays or a ScheduleExpression with rate(), the
    /// previously scheduled rotation might still occur. To prevent unintended rotations, use a
    /// ScheduleExpression with cron() for granular control over rotation windows. Rotation is
    /// an asynchronous process. For more information, see How rotation works.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_immediately: Option<bool>,
}

impl RotateSecretRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            secret_id: "test-secret_id".into(),
            rotation_lambda_arn: Some("test-rotation_lambda_arn".into()),
            rotation_rules: Some(RotationRulesType::fixture()),
            rotate_immediately: Some(false),
        }
    }
}

///
/// **AWS API**: `secretsmanager.v1.RotateSecretResponse`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RotateSecretResponse {
    /// The ARN of the secret.
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,

    /// The name of the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The ID of the new version of the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl RotateSecretResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            arn: Some("test-arn".into()),
            name: Some("test-name".into()),
            version_id: Some("test-version_id".into()),
        }
    }
}
