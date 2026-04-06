//! Base64 serde helpers for AWS `blob` fields.
//!
//! Re-exports [`cloud_lite_core::serde_base64`]. Generated types reference these
//! functions via `crate::serde_base64::` paths in serde `with` attributes.

#[allow(unused_imports)]
pub use cloud_lite_core::serde_base64::{deserialize_base64_opt, serialize_base64_opt};
