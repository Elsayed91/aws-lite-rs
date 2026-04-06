//! MockClient helpers for Amazon S3 API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Amazon S3 helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait S3MockHelpers {
    /// Helper to expect `put_bucket_policy`: Applies an Amazon S3 bucket policy to an Amazon S3
    /// bucket.
    fn expect_put_bucket_policy(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_bucket_policy`: Deletes the policy of a specified bucket.
    fn expect_delete_bucket_policy(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `put_public_access_block`: Creates or modifies the PublicAccessBlock
    /// configuration for an Amazon S3 bucket.
    fn expect_put_public_access_block(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `put_bucket_lifecycle_configuration`: Creates a new lifecycle configuration
    /// for the bucket or replaces an existing lifecycle configuration.
    fn expect_put_bucket_lifecycle_configuration(&mut self, bucket: &str)
    -> ExpectationBuilder<'_>;

    /// Helper to expect `list_buckets`: Returns a list of all buckets owned by the authenticated
    /// sender of the request.
    fn expect_list_buckets(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_bucket_versioning`: Returns the versioning state of a bucket.
    fn expect_get_bucket_versioning(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_bucket_encryption`: Returns the default encryption configuration for
    /// an Amazon S3 bucket.
    fn expect_get_bucket_encryption(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_bucket_logging`: Returns the logging status of a bucket and the
    /// permissions users have to view and modify that status.
    fn expect_get_bucket_logging(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_bucket_acl`: Returns the access control list (ACL) of a bucket.
    fn expect_get_bucket_acl(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_bucket_lifecycle_configuration`: Returns the lifecycle configuration
    /// information set on the bucket.
    fn expect_get_bucket_lifecycle_configuration(&mut self, bucket: &str)
    -> ExpectationBuilder<'_>;

    /// Helper to expect `get_public_access_block`: Retrieves the PublicAccessBlock configuration
    /// for an Amazon S3 bucket.
    fn expect_get_public_access_block(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_bucket_lifecycle_configuration`: Deletes the lifecycle
    /// configuration from the specified bucket.
    fn expect_delete_bucket_lifecycle_configuration(
        &mut self,
        bucket: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `put_bucket_versioning`: Sets the versioning state of an existing bucket.
    fn expect_put_bucket_versioning(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `put_bucket_encryption`: Creates the default encryption configuration for
    /// an Amazon S3 bucket.
    fn expect_put_bucket_encryption(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `put_bucket_logging`: Set the logging parameters for a bucket.
    fn expect_put_bucket_logging(&mut self, bucket: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl S3MockHelpers for MockClient {
    /// Helper to expect `put_bucket_policy`: Applies an Amazon S3 bucket policy to an Amazon S3
    /// bucket.
    fn expect_put_bucket_policy(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?policy");
        self.expect_put(&path)
    }

    /// Helper to expect `delete_bucket_policy`: Deletes the policy of a specified bucket.
    fn expect_delete_bucket_policy(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?policy");
        self.expect_delete(&path)
    }

    /// Helper to expect `put_public_access_block`: Creates or modifies the PublicAccessBlock
    /// configuration for an Amazon S3 bucket.
    fn expect_put_public_access_block(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?publicAccessBlock");
        self.expect_put(&path)
    }

    /// Helper to expect `put_bucket_lifecycle_configuration`: Creates a new lifecycle configuration
    /// for the bucket or replaces an existing lifecycle configuration.
    fn expect_put_bucket_lifecycle_configuration(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?lifecycle");
        self.expect_put(&path)
    }

    /// Helper to expect `list_buckets`: Returns a list of all buckets owned by the authenticated
    /// sender of the request.
    fn expect_list_buckets(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_get(&path)
    }

    /// Helper to expect `get_bucket_versioning`: Returns the versioning state of a bucket.
    fn expect_get_bucket_versioning(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?versioning");
        self.expect_get(&path)
    }

    /// Helper to expect `get_bucket_encryption`: Returns the default encryption configuration for
    /// an Amazon S3 bucket.
    fn expect_get_bucket_encryption(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?encryption");
        self.expect_get(&path)
    }

    /// Helper to expect `get_bucket_logging`: Returns the logging status of a bucket and the
    /// permissions users have to view and modify that status.
    fn expect_get_bucket_logging(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?logging");
        self.expect_get(&path)
    }

    /// Helper to expect `get_bucket_acl`: Returns the access control list (ACL) of a bucket.
    fn expect_get_bucket_acl(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?acl");
        self.expect_get(&path)
    }

    /// Helper to expect `get_bucket_lifecycle_configuration`: Returns the lifecycle configuration
    /// information set on the bucket.
    fn expect_get_bucket_lifecycle_configuration(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?lifecycle");
        self.expect_get(&path)
    }

    /// Helper to expect `get_public_access_block`: Retrieves the PublicAccessBlock configuration
    /// for an Amazon S3 bucket.
    fn expect_get_public_access_block(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?publicAccessBlock");
        self.expect_get(&path)
    }

    /// Helper to expect `delete_bucket_lifecycle_configuration`: Deletes the lifecycle
    /// configuration from the specified bucket.
    fn expect_delete_bucket_lifecycle_configuration(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?lifecycle");
        self.expect_delete(&path)
    }

    /// Helper to expect `put_bucket_versioning`: Sets the versioning state of an existing bucket.
    fn expect_put_bucket_versioning(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?versioning");
        self.expect_put(&path)
    }

    /// Helper to expect `put_bucket_encryption`: Creates the default encryption configuration for
    /// an Amazon S3 bucket.
    fn expect_put_bucket_encryption(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?encryption");
        self.expect_put(&path)
    }

    /// Helper to expect `put_bucket_logging`: Set the logging parameters for a bucket.
    fn expect_put_bucket_logging(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{bucket}?logging");
        self.expect_put(&path)
    }
}
