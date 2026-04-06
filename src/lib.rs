//! Lightweight HTTP client for Amazon Web Services APIs.
//!
//! Provides REST API access with automatic SigV4 signing,
//! retry, and rate limiting.

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
pub mod iam_policy;
#[cfg(any(test, feature = "test-support"))]
pub mod mock_client;
pub mod ops;
pub mod query;
pub(crate) mod serde_base64;
#[cfg(any(test, feature = "test-support"))]
pub mod test_support;
pub mod types;
pub mod xml;

pub use auth::AwsCredentials;
pub use client::{AwsHttpClient, AwsHttpClientBuilder, AwsResponse};
pub use error::{AwsError, Result};
#[cfg(any(test, feature = "test-support"))]
pub use mock_client::MockClient;

/// Build a URL query string from key-value pairs, omitting any with empty values.
pub(crate) fn append_query_params(url: String, params: &[(&str, &str)]) -> String {
    let qs: Vec<String> = params
        .iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect();
    if qs.is_empty() {
        url
    } else {
        format!("{}?{}", url, qs.join("&"))
    }
}
