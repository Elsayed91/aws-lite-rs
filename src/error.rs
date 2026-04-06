//! Error types for AWS HTTP client operations.

use std::time::Duration;
use thiserror::Error;

/// Result type alias using AwsError.
pub type Result<T> = std::result::Result<T, AwsError>;

/// Errors that can occur during AWS API operations.
#[derive(Debug, Error, Clone)]
pub enum AwsError {
    /// Authentication failed (invalid credentials, expired token).
    #[error("Authentication failed: {message}")]
    Auth { message: String },

    /// Access denied (insufficient IAM permissions).
    #[error("Access denied: {message}")]
    AccessDenied { message: String },

    /// Resource not found.
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    /// Request throttled.
    #[error("Throttled (retry after {retry_after:?})")]
    Throttled {
        retry_after: Option<Duration>,
        message: String,
    },

    /// AWS service error.
    #[error("Service error ({code}): {message}")]
    ServiceError {
        code: String,
        message: String,
        status: u16,
    },

    /// Network error.
    #[error("Network error: {0}")]
    Network(String),

    /// Invalid response.
    #[error("Invalid response: {message}")]
    InvalidResponse {
        message: String,
        body: Option<String>,
    },

    /// XML parsing error.
    #[error("XML parse error: {message}")]
    XmlParse { message: String },
}

impl From<reqwest::Error> for AwsError {
    fn from(err: reqwest::Error) -> Self {
        Self::Network(err.to_string())
    }
}

impl AwsError {
    /// Returns true if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Throttled { .. } | Self::Network(_) => true,
            Self::ServiceError { status, .. } => matches!(status, 500 | 502 | 503 | 504),
            _ => false,
        }
    }

    /// Extract retry-after duration if present.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::Throttled {
                retry_after: Some(duration),
                ..
            } => Some(*duration),
            _ => None,
        }
    }
}

/// Map an AWS error code + HTTP status to a typed `AwsError`.
#[allow(dead_code)]
fn classify_error(status: u16, code: &str, message: &str) -> AwsError {
    match status {
        401 => AwsError::Auth {
            message: format!("{code}: {message}"),
        },
        403 if code.contains("ExpiredToken") || code.contains("InvalidClientTokenId") => {
            AwsError::Auth {
                message: message.to_string(),
            }
        }
        403 => AwsError::AccessDenied {
            message: format!("{code}: {message}"),
        },
        404 => AwsError::NotFound {
            resource: message.to_string(),
        },
        429 => AwsError::Throttled {
            retry_after: None,
            message: message.to_string(),
        },
        _ if code == "Throttling"
            || code == "ThrottlingException"
            || code == "TooManyRequestsException" =>
        {
            AwsError::Throttled {
                retry_after: None,
                message: message.to_string(),
            }
        }
        _ => AwsError::ServiceError {
            code: code.to_string(),
            message: message.to_string(),
            status,
        },
    }
}

/// Parse an AWS XML error response (Query/XML protocol).
///
/// Expected format:
/// ```xml
/// <ErrorResponse>
///   <Error>
///     <Code>InvalidParameterValue</Code>
///     <Message>The value supplied is not valid.</Message>
///   </Error>
/// </ErrorResponse>
/// ```
#[allow(dead_code)]
pub(crate) fn parse_xml_error(status: u16, body: &str) -> AwsError {
    // Try to extract <Code> and <Message> from the XML body
    let code = extract_xml_tag(body, "Code").unwrap_or_default();
    let message = extract_xml_tag(body, "Message").unwrap_or_default();

    if code.is_empty() {
        return AwsError::ServiceError {
            code: format!("HttpError{status}"),
            message: truncate_body(body),
            status,
        };
    }

    classify_error(status, &code, &message)
}

/// Parse an AWS JSON error response (JSON protocol).
///
/// Expected format:
/// ```json
/// {"__type": "ResourceNotFoundException", "message": "The specified log group does not exist."}
/// ```
/// Some services use `Message` (capital M) instead of `message`.
#[allow(dead_code)]
pub(crate) fn parse_json_error(status: u16, body: &str) -> AwsError {
    let parsed: std::result::Result<serde_json::Value, _> = serde_json::from_str(body);
    let (code, message) = match parsed {
        Ok(val) => {
            let code = val
                .get("__type")
                .and_then(|v| v.as_str())
                .map(|s| {
                    // __type can be a full URI like "com.amazonaws.logs#ResourceNotFoundException"
                    s.rsplit_once('#').map(|(_, c)| c).unwrap_or(s).to_string()
                })
                .or_else(|| {
                    val.get("code")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .unwrap_or_default();
            let message = val
                .get("message")
                .or_else(|| val.get("Message"))
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            (code, message)
        }
        Err(_) => (String::new(), truncate_body(body)),
    };

    if code.is_empty() {
        return AwsError::ServiceError {
            code: format!("HttpError{status}"),
            message,
            status,
        };
    }

    classify_error(status, &code, &message)
}

/// Truncate a body string for error messages, avoiding unreadable HTML pages.
fn truncate_body(body: &str) -> String {
    if body.len() > 200 {
        let end = body.floor_char_boundary(200);
        format!("{}...", &body[..end])
    } else {
        body.to_string()
    }
}

/// Extract text content from a simple XML tag (no attributes, no nesting).
#[allow(dead_code)]
fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)? + start;
    Some(xml[start..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn throttled_is_retryable() {
        let err = AwsError::Throttled {
            retry_after: None,
            message: "slow down".into(),
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn network_is_retryable() {
        let err = AwsError::Network("timeout".into());
        assert!(err.is_retryable());
    }

    #[test]
    fn auth_is_not_retryable() {
        let err = AwsError::Auth {
            message: "bad creds".into(),
        };
        assert!(!err.is_retryable());
    }

    #[test]
    fn service_error_4xx_is_not_retryable() {
        let err = AwsError::ServiceError {
            code: "ValidationError".into(),
            message: "bad param".into(),
            status: 400,
        };
        assert!(!err.is_retryable());
    }

    #[test]
    fn service_error_500_is_retryable() {
        let err = AwsError::ServiceError {
            code: "InternalError".into(),
            message: "internal".into(),
            status: 500,
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn service_error_503_is_retryable() {
        let err = AwsError::ServiceError {
            code: "ServiceUnavailable".into(),
            message: "unavailable".into(),
            status: 503,
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn service_error_502_504_are_retryable() {
        for status in [502, 504] {
            let err = AwsError::ServiceError {
                code: "ServerError".into(),
                message: "error".into(),
                status,
            };
            assert!(err.is_retryable(), "status {status} should be retryable");
        }
    }

    #[test]
    fn parse_xml_error_extracts_code_and_message() {
        let body = r#"<ErrorResponse><Error><Code>InvalidParameterValue</Code><Message>Bad param</Message></Error></ErrorResponse>"#;
        let err = parse_xml_error(400, body);
        match err {
            AwsError::ServiceError {
                code,
                message,
                status,
            } => {
                assert_eq!(code, "InvalidParameterValue");
                assert_eq!(message, "Bad param");
                assert_eq!(status, 400);
            }
            other => panic!("expected ServiceError, got: {other}"),
        }
    }

    #[test]
    fn parse_xml_error_access_denied() {
        let body = r#"<ErrorResponse><Error><Code>AccessDenied</Code><Message>not allowed</Message></Error></ErrorResponse>"#;
        let err = parse_xml_error(403, body);
        assert!(matches!(err, AwsError::AccessDenied { .. }));
    }

    #[test]
    fn parse_xml_error_fallback_on_invalid_xml() {
        let err = parse_xml_error(500, "not xml at all");
        match err {
            AwsError::ServiceError { code, status, .. } => {
                assert_eq!(code, "HttpError500");
                assert_eq!(status, 500);
            }
            other => panic!("expected ServiceError, got: {other}"),
        }
    }

    #[test]
    fn parse_json_error_extracts_type_and_message() {
        let body = r#"{"__type": "ResourceNotFoundException", "message": "Log group not found"}"#;
        let err = parse_json_error(404, body);
        assert!(matches!(err, AwsError::NotFound { .. }));
    }

    #[test]
    fn parse_json_error_strips_uri_prefix() {
        let body =
            r#"{"__type": "com.amazonaws.logs#ResourceNotFoundException", "message": "not found"}"#;
        let err = parse_json_error(404, body);
        assert!(matches!(err, AwsError::NotFound { .. }));
    }

    #[test]
    fn parse_json_error_handles_capital_message() {
        let body = r#"{"__type": "ThrottlingException", "Message": "Rate exceeded"}"#;
        let err = parse_json_error(429, body);
        match err {
            AwsError::Throttled { message, .. } => {
                assert_eq!(message, "Rate exceeded");
            }
            other => panic!("expected Throttled, got: {other}"),
        }
    }

    #[test]
    fn parse_json_error_fallback_on_invalid_json() {
        let err = parse_json_error(500, "not json");
        match err {
            AwsError::ServiceError { code, status, .. } => {
                assert_eq!(code, "HttpError500");
                assert_eq!(status, 500);
            }
            other => panic!("expected ServiceError, got: {other}"),
        }
    }

    #[test]
    fn parse_xml_error_throttling() {
        let body = r#"<ErrorResponse><Error><Code>Throttling</Code><Message>Rate exceeded</Message></Error></ErrorResponse>"#;
        let err = parse_xml_error(400, body);
        assert!(matches!(err, AwsError::Throttled { .. }));
    }

    #[test]
    fn classify_401_unconditionally_as_auth() {
        // 401 with any code should map to Auth, not ServiceError
        let err = classify_error(401, "SignatureDoesNotMatch", "bad sig");
        assert!(matches!(err, AwsError::Auth { .. }), "got: {err}");

        let err = classify_error(401, "MissingAuthenticationToken", "no token");
        assert!(matches!(err, AwsError::Auth { .. }), "got: {err}");
    }

    #[test]
    fn classify_403_expired_token_as_auth() {
        let err = classify_error(403, "ExpiredToken", "token expired");
        assert!(matches!(err, AwsError::Auth { .. }), "got: {err}");

        let err = classify_error(403, "InvalidClientTokenId", "bad token");
        assert!(matches!(err, AwsError::Auth { .. }), "got: {err}");
    }

    #[test]
    fn classify_403_other_as_access_denied() {
        let err = classify_error(403, "AccessDenied", "not allowed");
        assert!(matches!(err, AwsError::AccessDenied { .. }), "got: {err}");
    }

    #[test]
    fn parse_xml_error_truncates_html_body() {
        let html = "<html><body>".to_string() + &"x".repeat(500) + "</body></html>";
        let err = parse_xml_error(502, &html);
        match err {
            AwsError::ServiceError { message, .. } => {
                assert!(
                    message.len() <= 203,
                    "message should be truncated, got {} chars",
                    message.len()
                );
                assert!(message.ends_with("..."));
            }
            other => panic!("expected ServiceError, got: {other}"),
        }
    }

    #[test]
    fn retry_after_returns_duration_for_throttled() {
        let err = AwsError::Throttled {
            retry_after: Some(Duration::from_secs(5)),
            message: "slow down".into(),
        };
        assert_eq!(err.retry_after(), Some(Duration::from_secs(5)));
    }

    #[test]
    fn retry_after_returns_none_for_non_throttled() {
        let err = AwsError::Auth {
            message: "bad creds".into(),
        };
        assert_eq!(err.retry_after(), None);
    }

    #[test]
    fn retry_after_returns_none_for_throttled_without_duration() {
        let err = AwsError::Throttled {
            retry_after: None,
            message: "slow down".into(),
        };
        assert_eq!(err.retry_after(), None);
    }

    #[test]
    fn truncate_body_handles_multibyte_utf8() {
        // 'é' is 2 bytes in UTF-8; build a string where byte 200 falls mid-character
        let body = "a".repeat(199) + "é" + &"b".repeat(100);
        // byte len: 199 + 2 + 100 = 301, so truncation triggers
        // byte 200 is inside 'é', floor_char_boundary should back up to 199
        let truncated = truncate_body(&body);
        assert!(truncated.ends_with("..."));
        assert!(truncated.len() <= 203); // 200 + "..."
    }

    #[test]
    fn parse_json_error_truncates_html_body() {
        let html = "<html><body>".to_string() + &"x".repeat(500) + "</body></html>";
        let err = parse_json_error(502, &html);
        match err {
            AwsError::ServiceError { message, .. } => {
                assert!(
                    message.len() <= 203,
                    "message should be truncated, got {} chars",
                    message.len()
                );
                assert!(message.ends_with("..."));
            }
            other => panic!("expected ServiceError, got: {other}"),
        }
    }
}
