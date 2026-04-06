//! Types for the Amazon SageMaker API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `sagemaker.v1.ListNotebookInstancesInput`
///
/// ## Coverage
/// 6 of 13 fields included.
/// Omitted fields:
/// - `CreationTimeBefore` — not selected in manifest
/// - `CreationTimeAfter` — not selected in manifest
/// - `LastModifiedTimeBefore` — not selected in manifest
/// - `LastModifiedTimeAfter` — not selected in manifest
/// - `NotebookInstanceLifecycleConfigNameContains` — not selected in manifest
/// - `DefaultCodeRepositoryContains` — not selected in manifest
/// - `AdditionalCodeRepositoryEquals` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListNotebookInstancesInput {
    /// If the previous call to the ListNotebookInstances is truncated, the response includes a
    /// NextToken. You can use this token in your subsequent ListNotebookInstances request to
    /// fetch the next set of notebook instances. You might specify a filter or a sort order in
    /// your request. When response is truncated, you must use the same values for the filer and
    /// sort order in the next request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    /// The maximum number of notebook instances to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,

    /// The field to sort results by. The default is Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// The sort order for results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,

    /// A filter that returns only notebook instances with the specified status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,

    /// A string in the notebook instances' name. This filter returns only notebook instances
    /// whose name contains the specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
}

impl ListNotebookInstancesInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            next_token: Some("test-next_token".into()),
            max_results: Some(100),
            sort_by: Some("test-sort_by".into()),
            sort_order: Some("test-sort_order".into()),
            status_equals: Some("test-status_equals".into()),
            name_contains: Some("test-name_contains".into()),
        }
    }
}

///
/// **AWS API**: `sagemaker.v1.ListNotebookInstancesOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListNotebookInstancesOutput {
    /// An array of NotebookInstanceSummary objects, one for each notebook instance.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notebook_instances: Vec<NotebookInstanceSummary>,

    /// If the response to the previous ListNotebookInstances request was truncated, SageMaker
    /// AI returns this token. To retrieve the next set of notebook instances, use the token in
    /// the next request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListNotebookInstancesOutput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            notebook_instances: vec![],
            next_token: Some("test-next_token".into()),
        }
    }
}

/// Provides summary information for an SageMaker AI notebook instance.
///
/// **AWS API**: `sagemaker.v1.NotebookInstanceSummary`
///
/// ## Coverage
/// 7 of 10 fields included.
/// Omitted fields:
/// - `NotebookInstanceLifecycleConfigName` — not selected in manifest
/// - `DefaultCodeRepository` — not selected in manifest
/// - `AdditionalCodeRepositories` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NotebookInstanceSummary {
    /// The name of the notebook instance that you want a summary for.
    pub notebook_instance_name: String,

    /// The Amazon Resource Name (ARN) of the notebook instance.
    pub notebook_instance_arn: String,

    /// The status of the notebook instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_status: Option<String>,

    /// The type of ML compute instance that the notebook instance is running on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,

    /// A timestamp that shows when the notebook instance was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,

    /// A timestamp that shows when the notebook instance was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,

    /// The URL that you use to connect to the Jupyter notebook running in your notebook
    /// instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl NotebookInstanceSummary {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            notebook_instance_name: "test-notebook_instance_name".into(),
            notebook_instance_arn: "test-notebook_instance_arn".into(),
            notebook_instance_status: Some("test-notebook_instance_status".into()),
            instance_type: Some("test-instance_type".into()),
            url: Some("test-url".into()),
            ..Default::default()
        }
    }
}

///
/// **AWS API**: `sagemaker.v1.StopNotebookInstanceInput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopNotebookInstanceInput {
    /// The name of the notebook instance to terminate.
    pub notebook_instance_name: String,
}

impl StopNotebookInstanceInput {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            notebook_instance_name: "test-notebook_instance_name".into(),
        }
    }
}
