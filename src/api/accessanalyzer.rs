//! AWS IAM Access Analyzer API client.
//!
//! Thin wrapper over generated ops. Adds ergonomic method signatures
//! and auto-pagination.
//!
//! Needed by AWS CIS benchmark checks:
//!   - CIS 2.19 (aws_iam_access_analyzer): confirm at least one ACTIVE
//!     analyzer exists per region (ACCOUNT or ORGANIZATION type).

use crate::{
    AwsHttpClient, Result, ops::accessanalyzer::AccessanalyzerOps,
    types::accessanalyzer::AnalyzerSummary,
};

/// Client for the AWS IAM Access Analyzer API.
pub struct AccessAnalyzerClient<'a> {
    ops: AccessanalyzerOps<'a>,
}

impl<'a> AccessAnalyzerClient<'a> {
    /// Create a new Access Analyzer client.
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: AccessanalyzerOps::new(client),
        }
    }

    // ── Analyzers ─────────────────────────────────────────────────────

    /// List all IAM Access Analyzers in the current region (auto-paginated).
    ///
    /// CIS 2.19: at least one analyzer with `status == "ACTIVE"` must exist
    /// in every region. Check the returned list's `status` fields.
    pub async fn list_analyzers(&self) -> Result<Vec<AnalyzerSummary>> {
        let mut all = Vec::new();
        let mut next_token = String::new();
        loop {
            let resp = self.ops.list_analyzers(&next_token, "", "").await?;
            all.extend(resp.analyzers);
            match resp.next_token {
                Some(tok) if !tok.is_empty() => next_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_list_analyzers_empty() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/analyzer")
            .returning_json(json!({"analyzers": []}))
            .times(1);

        let client = crate::AwsHttpClient::from_mock(mock);
        let aa = client.accessanalyzer();
        let result = aa.list_analyzers().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_list_analyzers_active() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/analyzer")
            .returning_json(json!({
                "analyzers": [
                    {
                        "arn": "arn:aws:access-analyzer:us-east-1:123456789012:analyzer/my-analyzer",
                        "name": "my-analyzer",
                        "type": "ACCOUNT",
                        "status": "ACTIVE",
                        "createdAt": "2024-01-01T00:00:00Z"
                    }
                ]
            }))
            .times(1);

        let client = crate::AwsHttpClient::from_mock(mock);
        let aa = client.accessanalyzer();
        let result = aa.list_analyzers().await;
        assert!(result.is_ok());
        let analyzers = result.unwrap();
        assert_eq!(analyzers.len(), 1);
        assert_eq!(analyzers[0].name, "my-analyzer");
        assert_eq!(analyzers[0].status, "ACTIVE");
        assert_eq!(analyzers[0].r#type, "ACCOUNT");
        assert_eq!(
            analyzers[0].arn,
            "arn:aws:access-analyzer:us-east-1:123456789012:analyzer/my-analyzer"
        );
    }

    #[tokio::test]
    async fn test_list_analyzers_paginated() {
        let mut mock = crate::MockClient::new();
        // Register second page first (more-specific URL)
        mock.expect_get("/analyzer?nextToken=tok1")
            .returning_json(json!({
                "analyzers": [{"arn": "arn:aws:access-analyzer:us-east-1:123456789012:analyzer/analyzer-2", "name": "analyzer-2", "type": "ACCOUNT", "status": "ACTIVE", "createdAt": "2024-01-02T00:00:00Z"}]
            }))
            .times(1);
        // First page
        mock.expect_get("/analyzer")
            .returning_json(json!({
                "analyzers": [{"arn": "arn:aws:access-analyzer:us-east-1:123456789012:analyzer/analyzer-1", "name": "analyzer-1", "type": "ACCOUNT", "status": "ACTIVE", "createdAt": "2024-01-01T00:00:00Z"}],
                "nextToken": "tok1"
            }))
            .times(1);

        let client = crate::AwsHttpClient::from_mock(mock);
        let aa = client.accessanalyzer();
        let result = aa.list_analyzers().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }
}
