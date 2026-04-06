//! AWS Signature Version 4 request signing.
//!
//! Implements the signing algorithm described at:
//! <https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_sigv.html>

use crate::auth::credentials::AwsCredentials;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::fmt::Write;

type HmacSha256 = Hmac<Sha256>;

/// Sign an HTTP request using AWS SigV4.
///
/// Returns the Authorization header value and additional headers
/// (X-Amz-Date, X-Amz-Content-Sha256, X-Amz-Security-Token if applicable).
pub fn sign_request(
    method: &str,
    url: &str,
    headers: &[(&str, &str)],
    body: &[u8],
    service: &str,
    credentials: &AwsCredentials,
    timestamp: &chrono::DateTime<chrono::Utc>,
) -> SignedHeaders {
    let date_stamp = timestamp.format("%Y%m%d").to_string();
    let amz_date = timestamp.format("%Y%m%dT%H%M%SZ").to_string();

    let parsed = parse_url(url);
    let payload_hash = hex_sha256(body);

    // Build sorted signed headers list
    let mut signed_headers_list: Vec<(String, String)> = headers
        .iter()
        .map(|(k, v)| (k.to_lowercase(), (*v).to_string()))
        .collect();
    signed_headers_list.push(("host".to_string(), parsed.host.clone()));
    signed_headers_list.push(("x-amz-date".to_string(), amz_date.clone()));
    if let Some(ref token) = credentials.session_token {
        signed_headers_list.push(("x-amz-security-token".to_string(), token.clone()));
    }
    signed_headers_list.sort_by(|a, b| a.0.cmp(&b.0));

    let signed_headers_str: String = signed_headers_list
        .iter()
        .map(|(k, _)| k.as_str())
        .collect::<Vec<_>>()
        .join(";");

    let canonical_headers: String = signed_headers_list
        .iter()
        .map(|(k, v)| format!("{}:{}\n", k, v.trim()))
        .collect();

    // Step 1: Canonical Request
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        method,
        parsed.canonical_path,
        parsed.canonical_query,
        canonical_headers,
        signed_headers_str,
        payload_hash
    );

    // Step 2: String to Sign
    let credential_scope = format!(
        "{}/{}/{}/aws4_request",
        date_stamp, credentials.region, service
    );
    let string_to_sign = format!(
        "AWS4-HMAC-SHA256\n{}\n{}\n{}",
        amz_date,
        credential_scope,
        hex_sha256(canonical_request.as_bytes())
    );

    // Step 3: Signing Key
    let signing_key = derive_signing_key(
        &credentials.secret_access_key,
        &date_stamp,
        &credentials.region,
        service,
    );

    // Step 4: Signature
    let signature = hex::encode(hmac_sha256(&signing_key, string_to_sign.as_bytes()));

    // Step 5: Authorization Header
    let authorization = format!(
        "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
        credentials.access_key_id, credential_scope, signed_headers_str, signature
    );

    let mut extra_headers = vec![
        ("X-Amz-Date".to_string(), amz_date),
        ("X-Amz-Content-Sha256".to_string(), payload_hash),
    ];
    if let Some(ref token) = credentials.session_token {
        extra_headers.push(("X-Amz-Security-Token".to_string(), token.clone()));
    }

    SignedHeaders {
        authorization,
        extra_headers,
    }
}

/// Result of signing a request.
pub struct SignedHeaders {
    /// The Authorization header value.
    pub authorization: String,
    /// Additional headers to include (X-Amz-Date, X-Amz-Content-Sha256, etc.).
    pub extra_headers: Vec<(String, String)>,
}

struct ParsedUrl {
    host: String,
    canonical_path: String,
    canonical_query: String,
}

/// RFC 3986 percent-encode a string, encoding everything except unreserved characters.
/// Unreserved: A-Z a-z 0-9 - _ . ~
fn uri_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            _ => {
                write!(result, "%{byte:02X}").unwrap();
            }
        }
    }
    result
}

/// Percent-encode a URI path. Each segment is decoded then re-encoded per RFC 3986,
/// preserving '/' separators.
///
/// NOTE: S3 uses single encoding for paths. Most other AWS services require double encoding
/// (encode the already-encoded path again). This implements single encoding, which is correct
/// for S3 and for services where the path is already normalized (like IAM's "/").
/// Double encoding support can be added when needed via a `double_encode` parameter.
fn uri_encode_path(path: &str) -> String {
    path.split('/')
        .map(|segment| {
            let decoded =
                urlencoding::decode(segment).unwrap_or(std::borrow::Cow::Borrowed(segment));
            uri_encode(&decoded)
        })
        .collect::<Vec<_>>()
        .join("/")
}

/// Parse a URL into components needed for SigV4 canonical request.
///
/// Query params are decoded, re-encoded per RFC 3986, sorted by encoded key,
/// then joined. Path segments are percent-encoded preserving '/'.
fn parse_url(url: &str) -> ParsedUrl {
    let after_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    let (host_and_path, raw_query) = after_scheme.split_once('?').unwrap_or((after_scheme, ""));
    let (host, path) = host_and_path
        .split_once('/')
        .map(|(h, p)| (h.to_string(), format!("/{p}")))
        .unwrap_or((host_and_path.to_string(), "/".to_string()));

    // Encode path segments (preserving '/')
    let canonical_path = uri_encode_path(&path);

    // Decode, re-encode, sort query params per SigV4 spec
    let canonical_query = if raw_query.is_empty() {
        String::new()
    } else {
        let mut params: Vec<(String, String)> = raw_query
            .split('&')
            .map(|pair| {
                let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
                // Decode any existing percent-encoding, then re-encode per RFC 3986
                let dk = urlencoding::decode(k).unwrap_or(std::borrow::Cow::Borrowed(k));
                let dv = urlencoding::decode(v).unwrap_or(std::borrow::Cow::Borrowed(v));
                (uri_encode(&dk), uri_encode(&dv))
            })
            .collect();
        params.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&")
    };

    ParsedUrl {
        host,
        canonical_path,
        canonical_query,
    }
}

fn hex_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts any key length");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

fn derive_signing_key(secret: &str, date: &str, region: &str, service: &str) -> Vec<u8> {
    let k_secret = format!("AWS4{secret}");
    let k_date = hmac_sha256(k_secret.as_bytes(), date.as_bytes());
    let k_region = hmac_sha256(&k_date, region.as_bytes());
    let k_service = hmac_sha256(&k_region, service.as_bytes());
    hmac_sha256(&k_service, b"aws4_request")
}

#[cfg(test)]
mod tests {
    use super::*;

    // AWS published test vector for signing key derivation:
    // https://docs.aws.amazon.com/general/latest/gr/sigv4-calculate-signature.html
    #[test]
    fn derive_signing_key_matches_aws_example() {
        let key = derive_signing_key(
            "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY",
            "20150830",
            "us-east-1",
            "iam",
        );
        assert_eq!(key.len(), 32);
        // Known expected value from AWS docs
        assert_eq!(
            hex::encode(&key),
            "c4afb1cc5771d871763a393e44b703571b55cc28424d1a5e86da6ed3c154a4b9"
        );
    }

    #[test]
    fn hex_sha256_empty_payload() {
        let hash = hex_sha256(b"");
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn hex_sha256_nonempty() {
        let hash = hex_sha256(b"hello");
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn parse_url_with_query() {
        let parsed = parse_url("https://iam.amazonaws.com/?Action=ListUsers&Version=2010-05-08");
        assert_eq!(parsed.host, "iam.amazonaws.com");
        assert_eq!(parsed.canonical_path, "/");
        assert_eq!(
            parsed.canonical_query,
            "Action=ListUsers&Version=2010-05-08"
        );
    }

    #[test]
    fn parse_url_without_query() {
        let parsed = parse_url("https://s3.amazonaws.com/my-bucket/my-key");
        assert_eq!(parsed.host, "s3.amazonaws.com");
        assert_eq!(parsed.canonical_path, "/my-bucket/my-key");
        assert_eq!(parsed.canonical_query, "");
    }

    #[test]
    fn parse_url_sorts_query_params() {
        let parsed =
            parse_url("https://example.com/?Zebra=1&Action=ListUsers&Alpha=2&Action=GetUser");
        // Sorted by key, then by value for duplicate keys
        assert_eq!(
            parsed.canonical_query,
            "Action=GetUser&Action=ListUsers&Alpha=2&Zebra=1"
        );
    }

    #[test]
    fn uri_encode_unreserved_chars_unchanged() {
        assert_eq!(uri_encode("ABCabc012-_.~"), "ABCabc012-_.~");
    }

    #[test]
    fn uri_encode_special_chars() {
        assert_eq!(uri_encode("hello world"), "hello%20world");
        assert_eq!(uri_encode("key=val"), "key%3Dval");
        assert_eq!(uri_encode("a+b"), "a%2Bb");
        assert_eq!(uri_encode("/"), "%2F");
    }

    #[test]
    fn uri_encode_path_preserves_slashes() {
        assert_eq!(uri_encode_path("/my-bucket/my key"), "/my-bucket/my%20key");
        assert_eq!(uri_encode_path("/"), "/");
        assert_eq!(
            uri_encode_path("/bucket/dir/file name.txt"),
            "/bucket/dir/file%20name.txt"
        );
    }

    #[test]
    fn parse_url_encodes_query_param_special_chars() {
        // Space encoded as %20 in the query param value
        let parsed = parse_url("https://example.com/?Prefix=my%20folder&Action=List");
        assert_eq!(parsed.canonical_query, "Action=List&Prefix=my%20folder");
    }

    #[test]
    fn parse_url_encodes_plus_in_query() {
        // Plus sign in query values must be encoded as %2B (not treated as space)
        let parsed = parse_url("https://example.com/?Tag=a%2Bb");
        assert_eq!(parsed.canonical_query, "Tag=a%2Bb");
    }

    #[test]
    fn parse_url_encodes_path_segments() {
        let parsed = parse_url("https://s3.amazonaws.com/my-bucket/my%20key");
        assert_eq!(parsed.canonical_path, "/my-bucket/my%20key");
    }

    #[test]
    fn sign_request_produces_valid_authorization_header() {
        let creds = AwsCredentials::new(
            "AKIDEXAMPLE".into(),
            "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY".into(),
            None,
            "us-east-1".into(),
        );
        let timestamp = chrono::DateTime::parse_from_rfc3339("2015-08-30T12:36:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);

        let signed = sign_request(
            "GET",
            "https://iam.amazonaws.com/?Action=ListUsers&Version=2010-05-08",
            &[],
            b"",
            "iam",
            &creds,
            &timestamp,
        );

        // Verify structure of authorization header
        assert!(
            signed
                .authorization
                .starts_with("AWS4-HMAC-SHA256 Credential=AKIDEXAMPLE/")
        );
        assert!(
            signed
                .authorization
                .contains("SignedHeaders=host;x-amz-date")
        );
        assert!(signed.authorization.contains("Signature="));

        // Verify extra headers
        let amz_date = signed.extra_headers.iter().find(|(k, _)| k == "X-Amz-Date");
        assert_eq!(amz_date.unwrap().1, "20150830T123600Z");

        let content_sha = signed
            .extra_headers
            .iter()
            .find(|(k, _)| k == "X-Amz-Content-Sha256");
        assert_eq!(
            content_sha.unwrap().1,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn sign_request_includes_session_token() {
        let creds = AwsCredentials::new(
            "AKID".into(),
            "SECRET".into(),
            Some("SESSION_TOKEN".into()),
            "us-east-1".into(),
        );
        let timestamp = chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);

        let signed = sign_request(
            "GET",
            "https://iam.amazonaws.com/?Action=ListUsers&Version=2010-05-08",
            &[],
            b"",
            "iam",
            &creds,
            &timestamp,
        );

        // Session token should be in signed headers and extra headers
        assert!(signed.authorization.contains("x-amz-security-token"));
        let token_header = signed
            .extra_headers
            .iter()
            .find(|(k, _)| k == "X-Amz-Security-Token");
        assert_eq!(token_header.unwrap().1, "SESSION_TOKEN");
    }

    #[test]
    fn sign_post_request_with_body() {
        let creds = AwsCredentials::new("AKID".into(), "SECRET".into(), None, "us-east-1".into());
        let timestamp = chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);

        let body = b"Action=CreateUser&UserName=test";
        let headers = [("content-type", "application/x-www-form-urlencoded")];

        let signed = sign_request(
            "POST",
            "https://iam.amazonaws.com/",
            &headers,
            body,
            "iam",
            &creds,
            &timestamp,
        );

        // Content-type should be in signed headers
        assert!(signed.authorization.contains("content-type"));
        // Body hash should NOT be the empty-string hash
        let content_sha = signed
            .extra_headers
            .iter()
            .find(|(k, _)| k == "X-Amz-Content-Sha256");
        assert_ne!(
            content_sha.unwrap().1,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
