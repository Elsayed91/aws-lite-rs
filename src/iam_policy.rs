//! IAM policy document types.
//!
//! These types are shared across S3 (bucket policies), IAM (inline/managed policies),
//! and AWS Config (`supplementaryConfiguration`). They are hand-written because the AWS
//! IAM policy JSON format uses polymorphic fields — `Principal`, `Action`, and `Resource`
//! can each be a string or an array, and `Principal` can additionally be a typed object
//! (`{"AWS": [...], "Service": [...], "Federated": [...]}`) — which requires custom serde
//! handling that the codegen pipeline cannot produce.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A parsed AWS IAM policy document.
///
/// ```json
/// {
///   "Version": "2012-10-17",
///   "Statement": [{ "Effect": "Allow", "Principal": "*", "Action": "s3:GetObject", ... }]
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyDocument {
    /// Policy language version. Almost always `"2012-10-17"`.
    #[serde(default)]
    pub version: Option<String>,

    /// The list of policy statements.
    pub statement: Vec<PolicyStatement>,
}

impl PolicyDocument {
    /// Parse a `PolicyDocument` from a raw AWS IAM policy JSON string.
    ///
    /// Returns `None` if the string is not valid JSON or cannot be deserialized as a
    /// policy document. Useful for parsing the raw JSON returned by S3's `GetBucketPolicy`
    /// and by AWS Config's `supplementaryConfiguration.BucketPolicy`.
    pub fn from_json(json_str: &str) -> Option<Self> {
        serde_json::from_str(json_str).ok()
    }
}

/// A single statement within an IAM policy document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyStatement {
    /// Optional statement identifier.
    #[serde(default)]
    pub sid: Option<String>,

    /// Whether this statement allows or denies the specified actions.
    pub effect: Effect,

    /// The principal(s) the statement applies to.
    ///
    /// AWS uses three distinct JSON shapes here — see [`PrincipalValue`].
    #[serde(default)]
    pub principal: PrincipalValue,

    /// The action(s) this statement covers.
    ///
    /// Can be a single string (`"s3:GetObject"`) or an array (`["s3:GetObject", "s3:PutObject"]`).
    #[serde(default)]
    pub action: StringOrArray,

    /// The resource(s) this statement applies to.
    ///
    /// Can be a single string (`"arn:aws:s3:::my-bucket/*"`) or an array.
    #[serde(default)]
    pub resource: StringOrArray,

    /// Optional condition block. The structure varies widely by condition operator and key,
    /// so it is kept as a raw JSON value.
    ///
    /// Example: `{"StringEquals": {"aws:PrincipalOrgID": ["o-abc123"]}}`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<serde_json::Value>,
}

/// Whether a policy statement allows or denies access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Effect {
    Allow,
    Deny,
}

/// AWS IAM `Principal` field — three possible JSON shapes.
///
/// AWS serializes the `Principal` field in three different ways:
///
/// | Shape | Example | Variant |
/// |-------|---------|---------|
/// | Wildcard string | `"*"` | `Wildcard` |
/// | Single ARN string | `"arn:aws:iam::123456789012:role/Foo"` | `Wildcard` |
/// | Typed map | `{"AWS": ["arn:..."], "Service": ["s3.amazonaws.com"]}` | `Typed` |
///
/// `#[serde(untagged)]` tries variants in declaration order. `Typed` is listed first so
/// that JSON objects bind to it before the string fallback (`Wildcard`) is tried.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrincipalValue {
    /// A typed principal map with one or more key categories:
    /// `"AWS"`, `"Service"`, or `"Federated"`, each mapping to one or more ARN strings.
    Typed(HashMap<String, StringOrArray>),

    /// A wildcard `"*"` or a single principal ARN string.
    Wildcard(String),
}

impl PrincipalValue {
    /// Flatten all principal strings into a single `Vec<String>`.
    ///
    /// - `Wildcard(s)` → `[s]`
    /// - `Typed(map)` → all values from all categories concatenated
    pub fn flatten(&self) -> Vec<String> {
        match self {
            PrincipalValue::Wildcard(s) => vec![s.clone()],
            PrincipalValue::Typed(map) => map
                .values()
                .flat_map(|v| v.as_slice().into_iter().map(|s| s.to_string()))
                .collect(),
        }
    }
}

impl Default for PrincipalValue {
    fn default() -> Self {
        PrincipalValue::Wildcard(String::new())
    }
}

/// A JSON value that can be either a single string or an array of strings.
///
/// Used for `Action` and `Resource` fields in IAM policy statements. AWS serializes these
/// as either `"s3:GetObject"` (single) or `["s3:GetObject", "s3:PutObject"]` (multiple).
///
/// `#[serde(untagged)]` tries `Single` first. A JSON string matches `Single`; a JSON array
/// fails `Single` and falls through to `Multiple`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrArray {
    /// A single string value.
    Single(String),
    /// An array of string values.
    Multiple(Vec<String>),
}

impl StringOrArray {
    /// Return all values as `Vec<&str>`, normalizing `Single` into a one-element vec.
    pub fn as_slice(&self) -> Vec<&str> {
        match self {
            StringOrArray::Single(s) => vec![s.as_str()],
            StringOrArray::Multiple(v) => v.iter().map(|s| s.as_str()).collect(),
        }
    }
}

impl Default for StringOrArray {
    fn default() -> Self {
        StringOrArray::Multiple(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── PolicyDocument::from_json ─────────────────────────────────────────────

    #[test]
    fn from_json_parses_minimal_policy() {
        let json = r#"{"Version":"2012-10-17","Statement":[]}"#;
        let doc = PolicyDocument::from_json(json).unwrap();
        assert_eq!(doc.version.as_deref(), Some("2012-10-17"));
        assert!(doc.statement.is_empty());
    }

    #[test]
    fn from_json_returns_none_for_invalid_json() {
        assert!(PolicyDocument::from_json("not json").is_none());
        assert!(PolicyDocument::from_json(r#"{"no_statement_key": true}"#).is_none());
    }

    #[test]
    fn from_json_parses_wildcard_principal_and_string_action() {
        let json = r#"{
            "Version": "2012-10-17",
            "Statement": [{
                "Effect": "Allow",
                "Principal": "*",
                "Action": "s3:GetObject",
                "Resource": "arn:aws:s3:::my-bucket/*"
            }]
        }"#;
        let doc = PolicyDocument::from_json(json).unwrap();
        assert_eq!(doc.statement.len(), 1);

        let stmt = &doc.statement[0];
        assert_eq!(stmt.effect, Effect::Allow);
        assert!(matches!(&stmt.principal, PrincipalValue::Wildcard(s) if s == "*"));
        assert_eq!(stmt.action.as_slice(), vec!["s3:GetObject"]);
        assert_eq!(stmt.resource.as_slice(), vec!["arn:aws:s3:::my-bucket/*"]);
    }

    #[test]
    fn from_json_parses_typed_principal_and_array_action() {
        let json = r#"{
            "Version": "2012-10-17",
            "Statement": [{
                "Effect": "Deny",
                "Principal": {"AWS": ["arn:aws:iam::123456789012:root", "arn:aws:iam::111111111111:role/Foo"]},
                "Action": ["s3:DeleteObject", "s3:DeleteBucket"],
                "Resource": "*"
            }]
        }"#;
        let doc = PolicyDocument::from_json(json).unwrap();
        let stmt = &doc.statement[0];
        assert_eq!(stmt.effect, Effect::Deny);

        let principals = stmt.principal.flatten();
        assert_eq!(principals.len(), 2);
        assert!(principals.contains(&"arn:aws:iam::123456789012:root".to_string()));

        assert_eq!(stmt.action.as_slice().len(), 2);
        assert!(stmt.action.as_slice().contains(&"s3:DeleteObject"));
    }

    #[test]
    fn from_json_parses_service_principal() {
        let json = r#"{
            "Version": "2012-10-17",
            "Statement": [{
                "Effect": "Allow",
                "Principal": {"Service": "cloudtrail.amazonaws.com"},
                "Action": "s3:PutObject",
                "Resource": "arn:aws:s3:::my-bucket/AWSLogs/*"
            }]
        }"#;
        let doc = PolicyDocument::from_json(json).unwrap();
        let stmt = &doc.statement[0];
        let principals = stmt.principal.flatten();
        assert_eq!(principals, vec!["cloudtrail.amazonaws.com"]);
    }

    #[test]
    fn from_json_parses_condition_block() {
        let json = r#"{
            "Version": "2012-10-17",
            "Statement": [{
                "Effect": "Allow",
                "Principal": "*",
                "Action": "s3:GetObject",
                "Resource": "*",
                "Condition": {"StringEquals": {"aws:PrincipalOrgID": ["o-abc123"]}}
            }]
        }"#;
        let doc = PolicyDocument::from_json(json).unwrap();
        assert!(doc.statement[0].condition.is_some());
    }

    #[test]
    fn from_json_parses_sid() {
        let json = r#"{
            "Version": "2012-10-17",
            "Statement": [{"Sid": "AllowPublicRead", "Effect": "Allow", "Principal": "*", "Action": "s3:GetObject", "Resource": "*"}]
        }"#;
        let doc = PolicyDocument::from_json(json).unwrap();
        assert_eq!(doc.statement[0].sid.as_deref(), Some("AllowPublicRead"));
    }

    // ── PrincipalValue ────────────────────────────────────────────────────────

    #[test]
    fn principal_wildcard_flatten() {
        let p = PrincipalValue::Wildcard("*".into());
        assert_eq!(p.flatten(), vec!["*"]);
    }

    #[test]
    fn principal_typed_flatten_multiple_categories() {
        let mut map = HashMap::new();
        map.insert(
            "AWS".into(),
            StringOrArray::Multiple(vec!["arn:aws:iam::1:root".into()]),
        );
        map.insert(
            "Service".into(),
            StringOrArray::Single("lambda.amazonaws.com".into()),
        );
        let p = PrincipalValue::Typed(map);
        let mut flat = p.flatten();
        flat.sort();
        assert_eq!(flat.len(), 2);
        assert!(flat.contains(&"arn:aws:iam::1:root".to_string()));
        assert!(flat.contains(&"lambda.amazonaws.com".to_string()));
    }

    // ── StringOrArray ─────────────────────────────────────────────────────────

    #[test]
    fn string_or_array_single_as_slice() {
        let s = StringOrArray::Single("s3:GetObject".into());
        assert_eq!(s.as_slice(), vec!["s3:GetObject"]);
    }

    #[test]
    fn string_or_array_multiple_as_slice() {
        let s = StringOrArray::Multiple(vec!["s3:GetObject".into(), "s3:PutObject".into()]);
        assert_eq!(s.as_slice(), vec!["s3:GetObject", "s3:PutObject"]);
    }

    #[test]
    fn string_or_array_default_is_empty_multiple() {
        let s = StringOrArray::default();
        assert!(s.as_slice().is_empty());
    }
}
