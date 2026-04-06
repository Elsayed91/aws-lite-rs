//! MockClient helpers for AWS CloudTrail API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with AWS CloudTrail helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CloudtrailMockHelpers {
    /// Helper to expect `describe_trails`: Retrieves settings for one or more trails associated
    /// with the current region.
    fn expect_describe_trails(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_trail_status`: Returns a JSON-formatted list of information about the
    /// specified trail.
    fn expect_get_trail_status(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_event_selectors`: Describes the settings for the event selectors
    /// configured for your trail.
    fn expect_get_event_selectors(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_trail`: Deletes a trail.
    fn expect_delete_trail(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_trail`: Updates trail settings that control what events you are
    /// logging.
    fn expect_update_trail(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CloudtrailMockHelpers for MockClient {
    /// Helper to expect `describe_trails`: Retrieves settings for one or more trails associated
    /// with the current region.
    fn expect_describe_trails(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_trail_status`: Returns a JSON-formatted list of information about the
    /// specified trail.
    fn expect_get_trail_status(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `get_event_selectors`: Describes the settings for the event selectors
    /// configured for your trail.
    fn expect_get_event_selectors(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_trail`: Deletes a trail.
    fn expect_delete_trail(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `update_trail`: Updates trail settings that control what events you are
    /// logging.
    fn expect_update_trail(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/".to_string();
        self.expect_post(&path)
    }
}
