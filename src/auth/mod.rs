//! AWS authentication and request signing.

pub mod credentials;
pub mod sigv4;

pub use credentials::AwsCredentials;
