//! MockClient helpers for Amazon Elastic File System API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon Elastic File System helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait EfsMockHelpers {
    /// Helper to expect `describe_file_systems`: Returns the description of a specific Amazon EFS
    /// file system if either the file system CreationToken or the FileSystemId is provided.
    /// Otherwise, it returns descriptions of all file systems owned by the caller's Amazon Web
    /// Services account in the Amazon Web Services Region of the endpoint that you're calling.
    fn expect_describe_file_systems(
        &mut self,
        max_items: &str,
        marker: &str,
        creation_token: &str,
        file_system_id: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl EfsMockHelpers for MockClient {
    /// Helper to expect `describe_file_systems`: Returns the description of a specific Amazon EFS
    /// file system if either the file system CreationToken or the FileSystemId is provided.
    /// Otherwise, it returns descriptions of all file systems owned by the caller's Amazon Web
    /// Services account in the Amazon Web Services Region of the endpoint that you're calling.
    fn expect_describe_file_systems(
        &mut self,
        max_items: &str,
        marker: &str,
        creation_token: &str,
        file_system_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = "/2015-02-01/file-systems".to_string();
        let mut __qp: Vec<String> = Vec::new();
        if !max_items.is_empty() {
            __qp.push(format!("MaxItems={}", max_items));
        }
        if !marker.is_empty() {
            __qp.push(format!("Marker={}", marker));
        }
        if !creation_token.is_empty() {
            __qp.push(format!("CreationToken={}", creation_token));
        }
        if !file_system_id.is_empty() {
            __qp.push(format!("FileSystemId={}", file_system_id));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }
}
