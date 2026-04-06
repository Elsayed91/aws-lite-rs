//! Amazon Route 53 API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::route53::Route53Ops`. This layer adds:
//! - Ergonomic method signatures

use serde::Serialize;

use crate::{
    AwsHttpClient, Result,
    ops::route53::Route53Ops,
    types::route53::{
        AliasTarget, Change, ChangeAction, ChangeResourceRecordSetsRequest,
        ChangeResourceRecordSetsResponse, CreateHealthCheckRequest, CreateHealthCheckResponse,
        GetHealthCheckStatusResponse, ListHealthChecksResponse, ListHostedZonesResponse,
        ListResourceRecordSetsResponse, RRType, ResourceRecord,
    },
};

// ── Local XML serialization helpers for ChangeResourceRecordSets ──────────────
//
// quick_xml serializes `Vec<Change>` as repeated <Changes>item</Changes> tags:
//   <Changes><Action>CREATE</Action>...</Changes>
// But Route53 requires wrapped elements:
//   <Changes><Change><Action>CREATE</Action>...</Change></Changes>
//
// Similarly ResourceRecords needs:
//   <ResourceRecords><ResourceRecord><Value>x</Value></ResourceRecord></ResourceRecords>
//
// These local structs produce the correct nesting.  They mirror the public
// `types::route53` structs but have an extra level of wrapping for the lists.

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct XmlResourceRecord<'a> {
    value: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct XmlResourceRecordsWrapper<'a> {
    resource_record: Vec<XmlResourceRecord<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct XmlResourceRecordSet<'a> {
    name: &'a str,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    r#type: Option<RRType>,
    #[serde(rename = "TTL", skip_serializing_if = "Option::is_none")]
    ttl: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_records: Option<XmlResourceRecordsWrapper<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias_target: Option<&'a AliasTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_identifier: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct XmlChange<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ChangeAction>,
    resource_record_set: XmlResourceRecordSet<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct XmlChangesWrapper<'a> {
    change: Vec<XmlChange<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct XmlChangeBatch<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<&'a str>,
    changes: XmlChangesWrapper<'a>,
}

#[derive(Serialize)]
#[serde(rename = "ChangeResourceRecordSetsRequest", rename_all = "PascalCase")]
struct XmlChangeResourceRecordSetsRequest<'a> {
    change_batch: XmlChangeBatch<'a>,
}

fn build_change_rrset_xml(body: &ChangeResourceRecordSetsRequest) -> Result<String> {
    let xml_changes: Vec<XmlChange<'_>> = body
        .change_batch
        .changes
        .iter()
        .map(|c: &Change| {
            let rrs = &c.resource_record_set;
            let resource_records = if rrs.resource_records.is_empty() {
                None
            } else {
                Some(XmlResourceRecordsWrapper {
                    resource_record: rrs
                        .resource_records
                        .iter()
                        .map(|r: &ResourceRecord| XmlResourceRecord { value: &r.value })
                        .collect(),
                })
            };
            XmlChange {
                action: c.action,
                resource_record_set: XmlResourceRecordSet {
                    name: &rrs.name,
                    r#type: rrs.r#type,
                    ttl: rrs.ttl,
                    resource_records,
                    alias_target: rrs.alias_target.as_ref(),
                    health_check_id: rrs.health_check_id.as_deref(),
                    set_identifier: rrs.set_identifier.as_deref(),
                    weight: rrs.weight,
                },
            }
        })
        .collect();

    let xml_body = XmlChangeResourceRecordSetsRequest {
        change_batch: XmlChangeBatch {
            comment: body.change_batch.comment.as_deref(),
            changes: XmlChangesWrapper {
                change: xml_changes,
            },
        },
    };

    quick_xml::se::to_string(&xml_body).map_err(|e| crate::AwsError::InvalidResponse {
        message: format!("Failed to serialize ChangeResourceRecordSets request to XML: {e}"),
        body: None,
    })
}

/// Client for the Amazon Route 53 API
pub struct Route53Client<'a> {
    ops: Route53Ops<'a>,
}

impl<'a> Route53Client<'a> {
    /// Create a new Amazon Route 53 API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: Route53Ops::new(client),
        }
    }

    /// Retrieves a list of the public and private hosted zones associated with the current AWS account.
    pub async fn list_hosted_zones(&self) -> Result<ListHostedZonesResponse> {
        self.ops.list_hosted_zones().await
    }

    /// Lists the resource record sets in a specified hosted zone.
    pub async fn list_resource_record_sets(
        &self,
        id: &str,
    ) -> Result<ListResourceRecordSetsResponse> {
        self.ops.list_resource_record_sets(id).await
    }

    /// Retrieve a list of the health checks associated with the current AWS account.
    pub async fn list_health_checks(&self) -> Result<ListHealthChecksResponse> {
        self.ops.list_health_checks().await
    }

    /// Gets status of a health check based on the most recent checker observations.
    pub async fn get_health_check_status(
        &self,
        health_check_id: &str,
    ) -> Result<GetHealthCheckStatusResponse> {
        self.ops.get_health_check_status(health_check_id).await
    }

    /// Creates a new health check.
    pub async fn create_health_check(
        &self,
        body: &CreateHealthCheckRequest,
    ) -> Result<CreateHealthCheckResponse> {
        self.ops.create_health_check(body).await
    }

    /// Deletes a health check.
    pub async fn delete_health_check(&self, health_check_id: &str) -> Result<()> {
        self.ops.delete_health_check(health_check_id).await
    }

    /// Creates, changes, or deletes a resource record set.
    ///
    /// Route53 requires the `Changes` list to be wrapped with individual `<Change>` elements:
    /// `<Changes><Change>...</Change></Changes>`. This method constructs that XML manually
    /// because quick_xml's default Vec serialization produces flat repeated tags.
    pub async fn change_resource_record_sets(
        &self,
        id: &str,
        body: &ChangeResourceRecordSetsRequest,
    ) -> Result<ChangeResourceRecordSetsResponse> {
        use urlencoding::encode;
        #[cfg(any(test, feature = "test-support"))]
        let base_url = self
            .ops
            .client
            .base_url
            .as_deref()
            .map(|u| u.trim_end_matches('/'))
            .unwrap_or("https://route53.amazonaws.com");
        #[cfg(not(any(test, feature = "test-support")))]
        let base_url = "https://route53.amazonaws.com";
        let url = format!("{base_url}/2013-04-01/hostedzone/{}/rrset/", encode(id));
        let body_xml = build_change_rrset_xml(body)?;
        let body_xml = crate::xml::inject_xml_namespace(
            &body_xml,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        let response = self
            .ops
            .client
            .post(&url, "route53", body_xml.as_bytes(), "application/xml")
            .await?;
        let response = response.error_for_status("xml").await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AwsError::InvalidResponse {
                    message: format!("Failed to read change_resource_record_sets response: {e}"),
                    body: None,
                })?;
        let body_text =
            std::str::from_utf8(&response_bytes).map_err(|e| crate::AwsError::InvalidResponse {
                message: format!("Invalid UTF-8 in change_resource_record_sets response: {e}"),
                body: None,
            })?;
        crate::xml::parse_rest_xml_response::<ChangeResourceRecordSetsResponse>(body_text).map_err(
            |e| crate::AwsError::InvalidResponse {
                message: format!("Failed to parse change_resource_record_sets XML response: {e}"),
                body: Some(body_text.to_string()),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Route53 REST-XML responses use PascalCase field names and namespace-stripped XML.
    // Proven behavior from integration testing (2026-02-18):
    // - Marker field is absent when there are no more pages (Option<String>)
    // - IsTruncated is always present
    // - MaxItems is always present
    // - HostedZone.Id returns the full "/hostedzone/XXXXX" format
    // - HealthCheck.Id is a UUID without prefix

    #[tokio::test]
    async fn test_list_hosted_zones() {
        let mut mock = crate::MockClient::new();
        let xml = r#"<?xml version="1.0"?>
<ListHostedZonesResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/">
  <HostedZones>
    <HostedZone>
      <Id>/hostedzone/Z08460922J0JNOMET4IK5</Id>
      <Name>cloud-lite-test-ralph.internal.</Name>
      <CallerReference>cloud-lite-test-ralph-1771394386</CallerReference>
      <Config><PrivateZone>true</PrivateZone></Config>
      <ResourceRecordSetCount>2</ResourceRecordSetCount>
    </HostedZone>
  </HostedZones>
  <IsTruncated>false</IsTruncated>
  <MaxItems>100</MaxItems>
</ListHostedZonesResponse>"#;
        mock.expect_get("/2013-04-01/hostedzone")
            .returning_bytes(xml.as_bytes().to_vec());

        let client = AwsHttpClient::from_mock(mock);
        let r53 = client.route53();
        let result = r53.list_hosted_zones().await.unwrap();

        assert_eq!(result.hosted_zones.len(), 1);
        assert_eq!(
            result.hosted_zones[0].id,
            "/hostedzone/Z08460922J0JNOMET4IK5"
        );
        assert_eq!(
            result.hosted_zones[0].name,
            "cloud-lite-test-ralph.internal."
        );
        assert_eq!(
            result.hosted_zones[0].caller_reference,
            "cloud-lite-test-ralph-1771394386"
        );
        assert_eq!(result.hosted_zones[0].resource_record_set_count, Some(2));
        assert!(!result.is_truncated);
        assert_eq!(result.max_items, "100");
        assert_eq!(result.marker, None); // absent when no more pages
    }

    #[tokio::test]
    async fn test_list_resource_record_sets() {
        let mut mock = crate::MockClient::new();
        let xml = r#"<?xml version="1.0"?>
<ListResourceRecordSetsResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/">
  <ResourceRecordSets>
    <ResourceRecordSet>
      <Name>cloud-lite-test-ralph.internal.</Name>
      <Type>NS</Type>
      <TTL>172800</TTL>
      <ResourceRecords>
        <ResourceRecord><Value>ns-1.awsdns-01.com.</Value></ResourceRecord>
      </ResourceRecords>
    </ResourceRecordSet>
    <ResourceRecordSet>
      <Name>cloud-lite-test-ralph.internal.</Name>
      <Type>SOA</Type>
      <TTL>900</TTL>
      <ResourceRecords>
        <ResourceRecord><Value>ns-1.awsdns-01.com.</Value></ResourceRecord>
      </ResourceRecords>
    </ResourceRecordSet>
  </ResourceRecordSets>
  <IsTruncated>false</IsTruncated>
  <MaxItems>300</MaxItems>
</ListResourceRecordSetsResponse>"#;
        mock.expect_get("/2013-04-01/hostedzone/Z08460922J0JNOMET4IK5/rrset")
            .returning_bytes(xml.as_bytes().to_vec());

        let client = AwsHttpClient::from_mock(mock);
        let r53 = client.route53();
        let result = r53
            .list_resource_record_sets("Z08460922J0JNOMET4IK5")
            .await
            .unwrap();

        assert_eq!(result.resource_record_sets.len(), 2);
        assert_eq!(
            result.resource_record_sets[0].name,
            "cloud-lite-test-ralph.internal."
        );
        assert!(!result.is_truncated);
        assert_eq!(result.max_items, "300");
    }

    #[tokio::test]
    async fn test_list_health_checks() {
        let mut mock = crate::MockClient::new();
        let xml = r#"<?xml version="1.0"?>
<ListHealthChecksResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/">
  <HealthChecks>
    <HealthCheck>
      <Id>0e548232-8f42-4e8c-bbda-81b6bf5e71ca</Id>
      <CallerReference>cloud-lite-test-ralph-1771394175</CallerReference>
      <HealthCheckConfig>
        <Type>HTTP</Type>
        <FullyQualifiedDomainName>cloud-lite-test-ralph-healthcheck.example.com</FullyQualifiedDomainName>
        <Port>80</Port>
        <ResourcePath>/health</ResourcePath>
        <RequestInterval>30</RequestInterval>
        <FailureThreshold>3</FailureThreshold>
      </HealthCheckConfig>
      <HealthCheckVersion>1</HealthCheckVersion>
    </HealthCheck>
  </HealthChecks>
  <IsTruncated>false</IsTruncated>
  <MaxItems>100</MaxItems>
</ListHealthChecksResponse>"#;
        mock.expect_get("/2013-04-01/healthcheck")
            .returning_bytes(xml.as_bytes().to_vec());

        let client = AwsHttpClient::from_mock(mock);
        let r53 = client.route53();
        let result = r53.list_health_checks().await.unwrap();

        assert_eq!(result.health_checks.len(), 1);
        let hc = &result.health_checks[0];
        assert_eq!(hc.id, "0e548232-8f42-4e8c-bbda-81b6bf5e71ca");
        assert_eq!(hc.caller_reference, "cloud-lite-test-ralph-1771394175");
        assert_eq!(hc.health_check_version, 1);
        assert_eq!(
            hc.health_check_config
                .fully_qualified_domain_name
                .as_deref(),
            Some("cloud-lite-test-ralph-healthcheck.example.com")
        );
        assert_eq!(hc.health_check_config.port, Some(80));
        assert!(!result.is_truncated);
        assert_eq!(result.max_items, "100");
        assert_eq!(result.marker, None); // absent when no more pages
    }

    #[tokio::test]
    async fn test_get_health_check_status() {
        let mut mock = crate::MockClient::new();
        let xml = r#"<?xml version="1.0"?>
<GetHealthCheckStatusResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/">
  <HealthCheckObservations>
    <HealthCheckObservation>
      <Region>us-east-1</Region>
      <IPAddress>15.177.4.1</IPAddress>
      <StatusReport>
        <Status>Success: HTTP Status Code 200</Status>
        <CheckedTime>2026-02-18T08:00:00.000Z</CheckedTime>
      </StatusReport>
    </HealthCheckObservation>
  </HealthCheckObservations>
</GetHealthCheckStatusResponse>"#;
        mock.expect_get("/2013-04-01/healthcheck/0e548232-8f42-4e8c-bbda-81b6bf5e71ca/status")
            .returning_bytes(xml.as_bytes().to_vec());

        let client = AwsHttpClient::from_mock(mock);
        let r53 = client.route53();
        let result = r53
            .get_health_check_status("0e548232-8f42-4e8c-bbda-81b6bf5e71ca")
            .await
            .unwrap();

        assert_eq!(result.health_check_observations.len(), 1);
        let obs = &result.health_check_observations[0];
        assert_eq!(obs.region.as_deref(), Some("us-east-1"));
        assert_eq!(obs.ip_address.as_deref(), Some("15.177.4.1"));
        let status = obs.status_report.as_ref().unwrap();
        assert!(status.status.as_deref().unwrap().contains("200"));
    }

    // DeleteHealthCheck: proven behavior from integration testing (2026-02-18):
    // - DELETE /2013-04-01/healthcheck/{id} returns empty body on success
    // - Route53 returns HTTP 200 with no body when delete succeeds
    #[tokio::test]
    async fn test_delete_health_check() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/2013-04-01/healthcheck/8271fa7a-bffa-4cc5-9d33-70d9cf71a032")
            .returning_bytes(vec![]);

        let client = AwsHttpClient::from_mock(mock);
        let r53 = client.route53();
        let result = r53
            .delete_health_check("8271fa7a-bffa-4cc5-9d33-70d9cf71a032")
            .await;
        assert!(
            result.is_ok(),
            "delete_health_check should return Ok(()) on success: {result:?}"
        );
    }

    // ChangeResourceRecordSets: proven behavior from integration testing (2026-02-18):
    // - POST /2013-04-01/hostedzone/{id}/rrset/ with XML body
    // - Body must be <ChangeResourceRecordSetsRequest><ChangeBatch><Changes><Change>...</Change></Changes></ChangeBatch></ChangeResourceRecordSetsRequest>
    // - ResourceRecords must be wrapped: <ResourceRecords><ResourceRecord><Value>x</Value></ResourceRecord></ResourceRecords>
    // - Response is <ChangeResourceRecordSetsResponse><ChangeInfo><Id>...</Id><Status>PENDING</Status><SubmittedAt>...</SubmittedAt></ChangeInfo></ChangeResourceRecordSetsResponse>
    // - Status is PENDING immediately after submission
    // - ChangeInfo.Id has "/change/" prefix (e.g. "/change/C03401643O9NFZYR50ZKY")
    // - SubmittedAt is an ISO 8601 timestamp
    #[tokio::test]
    async fn test_change_resource_record_sets() {
        use crate::types::route53::{
            Change, ChangeAction, ChangeBatch, ChangeResourceRecordSetsRequest, RRType,
            ResourceRecord, ResourceRecordSet,
        };

        let mut mock = crate::MockClient::new();
        let xml = r#"<?xml version="1.0"?>
<ChangeResourceRecordSetsResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/">
  <ChangeInfo>
    <Id>/change/C03401643O9NFZYR50ZKY</Id>
    <Status>PENDING</Status>
    <SubmittedAt>2026-02-18T06:24:24.579Z</SubmittedAt>
  </ChangeInfo>
</ChangeResourceRecordSetsResponse>"#;
        mock.expect_post("/2013-04-01/hostedzone/Z08643462LIHG4ESDGE73/rrset/")
            .returning_bytes(xml.as_bytes().to_vec());

        let client = AwsHttpClient::from_mock(mock);
        let r53 = client.route53();
        let body = ChangeResourceRecordSetsRequest {
            change_batch: ChangeBatch {
                comment: Some("test change".to_string()),
                changes: vec![Change {
                    action: Some(ChangeAction::Create),
                    resource_record_set: ResourceRecordSet {
                        name: "test-txt.cloud-lite-test-ralph.internal.".to_string(),
                        r#type: Some(RRType::Txt),
                        ttl: Some(300),
                        resource_records: vec![ResourceRecord {
                            value: "\"cloud-lite-test-value\"".to_string(),
                        }],
                        ..Default::default()
                    },
                }],
            },
        };
        let result = r53
            .change_resource_record_sets("Z08643462LIHG4ESDGE73", &body)
            .await
            .unwrap();

        // ChangeInfo.Id has "/change/" prefix
        assert_eq!(result.change_info.id, "/change/C03401643O9NFZYR50ZKY");
        // Status is PENDING immediately after submission
        assert_eq!(
            result.change_info.status,
            Some(crate::types::route53::ChangeStatus::Pending)
        );
        // SubmittedAt is an ISO 8601 timestamp
        assert_eq!(result.change_info.submitted_at, "2026-02-18T06:24:24.579Z");
    }

    // ChangeResourceRecordSets body serialization: verify XML structure
    // Proven: quick_xml flat Vec serialization produces wrong structure;
    // build_change_rrset_xml wraps correctly for Route53.
    #[test]
    fn test_change_resource_record_sets_xml_body_structure() {
        use crate::types::route53::{
            Change, ChangeAction, ChangeBatch, ChangeResourceRecordSetsRequest, RRType,
            ResourceRecord, ResourceRecordSet,
        };

        let body = ChangeResourceRecordSetsRequest {
            change_batch: ChangeBatch {
                comment: None,
                changes: vec![Change {
                    action: Some(ChangeAction::Create),
                    resource_record_set: ResourceRecordSet {
                        name: "test.example.com.".to_string(),
                        r#type: Some(RRType::Txt),
                        ttl: Some(300),
                        resource_records: vec![ResourceRecord {
                            value: "\"hello\"".to_string(),
                        }],
                        ..Default::default()
                    },
                }],
            },
        };

        let xml = build_change_rrset_xml(&body).unwrap();
        // Root element must be ChangeResourceRecordSetsRequest (not XmlChangeResourceRecordSetsRequest)
        assert!(
            xml.contains("<ChangeResourceRecordSetsRequest"),
            "root element must be ChangeResourceRecordSetsRequest, got: {xml}"
        );
        // Changes wraps Change element (not repeated Changes)
        assert!(
            xml.contains("<Changes><Change>")
                || xml.contains("<Changes>\n<Change>")
                || xml.contains("<Changes> <Change>"),
            "Changes must contain <Change> element, got: {xml}"
        );
        // ResourceRecord is wrapped
        assert!(
            xml.contains("<ResourceRecords><ResourceRecord>")
                || xml.contains("<ResourceRecords>\n<ResourceRecord>"),
            "ResourceRecords must contain <ResourceRecord> element, got: {xml}"
        );
        // TTL uses correct casing
        assert!(
            xml.contains("<TTL>300</TTL>"),
            "TTL field name must be <TTL>, got: {xml}"
        );
        // Type uses correct casing
        assert!(
            xml.contains("<Type>"),
            "Type field must be present, got: {xml}"
        );
    }
}
