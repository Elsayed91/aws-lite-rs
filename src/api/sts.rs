//! AWS Security Token Service API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::sts::StsOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::sts::StsOps,
    types::sts::{
        AssumeRoleRequest, AssumeRoleResponse, GetCallerIdentityRequest, GetCallerIdentityResponse,
    },
};

/// Client for the AWS Security Token Service API.
pub struct StsClient<'a> {
    ops: StsOps<'a>,
}

impl<'a> StsClient<'a> {
    /// Create a new AWS Security Token Service API client.
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: StsOps::new(client),
        }
    }

    /// Returns details about the IAM user or role whose credentials are used
    /// to call the operation.
    pub async fn get_caller_identity(&self) -> Result<GetCallerIdentityResponse> {
        let body = GetCallerIdentityRequest::default();
        self.ops.get_caller_identity(&body).await
    }

    /// Returns a set of temporary security credentials for cross-account access.
    pub async fn assume_role(&self, body: &AssumeRoleRequest) -> Result<AssumeRoleResponse> {
        self.ops.assume_role(body).await
    }
}

#[cfg(test)]
mod tests {
    use crate::AwsHttpClient;
    use crate::mock_client::MockClient;
    use crate::test_support::sts_mock_helpers::StsMockHelpers;

    fn xml_envelope(action: &str, inner: &str) -> Vec<u8> {
        format!("<{action}Response><{action}Result>{inner}</{action}Result></{action}Response>")
            .into_bytes()
    }

    #[tokio::test]
    async fn get_caller_identity_returns_parsed_response() {
        let mut mock = MockClient::new();
        mock.expect_get_caller_identity()
            .returning_bytes(xml_envelope(
                "GetCallerIdentity",
                "<Account>265411104181</Account>\
             <Arn>arn:aws:iam::265411104181:root</Arn>\
             <UserId>265411104181</UserId>",
            ));

        let client = AwsHttpClient::from_mock(mock);
        let response = client.sts().get_caller_identity().await.unwrap();

        assert_eq!(response.account.as_deref(), Some("265411104181"));
        assert_eq!(
            response.arn.as_deref(),
            Some("arn:aws:iam::265411104181:root")
        );
        assert_eq!(response.user_id.as_deref(), Some("265411104181"));
    }

    #[tokio::test]
    async fn assume_role_returns_credentials() {
        let mut mock = MockClient::new();
        mock.expect_assume_role().returning_bytes(xml_envelope(
            "AssumeRole",
            "<Credentials>\
               <AccessKeyId>ASIATESTACCESSKEY</AccessKeyId>\
               <SecretAccessKey>testsecretkey123</SecretAccessKey>\
               <SessionToken>testsessiontoken456</SessionToken>\
               <Expiration>2024-01-15T12:00:00Z</Expiration>\
             </Credentials>\
             <AssumedRoleUser>\
               <AssumedRoleId>AROATESTROLE:test-session</AssumedRoleId>\
               <Arn>arn:aws:sts::123456789012:assumed-role/TestRole/test-session</Arn>\
             </AssumedRoleUser>",
        ));

        let client = AwsHttpClient::from_mock(mock);
        let body = crate::types::sts::AssumeRoleRequest {
            role_arn: "arn:aws:iam::123456789012:role/TestRole".to_string(),
            role_session_name: "test-session".to_string(),
            ..Default::default()
        };
        let response = client.sts().assume_role(&body).await.unwrap();

        let creds = response
            .credentials
            .as_ref()
            .expect("credentials should be set");
        assert_eq!(creds.access_key_id, "ASIATESTACCESSKEY");
        assert_eq!(creds.secret_access_key, "testsecretkey123");
        assert_eq!(creds.session_token, "testsessiontoken456");
        assert_eq!(creds.expiration, "2024-01-15T12:00:00Z");

        let assumed = response
            .assumed_role_user
            .as_ref()
            .expect("assumed role user should be set");
        assert_eq!(assumed.assumed_role_id, "AROATESTROLE:test-session");
        assert_eq!(
            assumed.arn,
            "arn:aws:sts::123456789012:assumed-role/TestRole/test-session"
        );
    }
}
