//! Amazon Relational Database Service API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::rds::RdsOps`. This layer adds:
//! - Ergonomic method signatures

use crate::{
    AwsHttpClient, Result,
    ops::rds::RdsOps,
    types::rds::{
        CreateDBSnapshotRequest, CreateDBSnapshotResponse, DeleteDBInstanceRequest,
        DeleteDBInstanceResponse, DeleteDBSnapshotRequest, DeleteDBSnapshotResponse,
        DescribeDBInstancesRequest, DescribeDBInstancesResponse,
        DescribeDBSnapshotAttributesRequest, DescribeDBSnapshotAttributesResponse,
        DescribeDBSnapshotsRequest, DescribeDBSnapshotsResponse, ModifyDBInstanceRequest,
        ModifyDBInstanceResponse, ModifyDBSnapshotAttributeRequest,
        ModifyDBSnapshotAttributeResponse, StartDBInstanceRequest, StartDBInstanceResponse,
        StopDBInstanceRequest, StopDBInstanceResponse,
    },
};

/// Client for the Amazon Relational Database Service API
pub struct RdsClient<'a> {
    ops: RdsOps<'a>,
}

impl<'a> RdsClient<'a> {
    /// Create a new Amazon Relational Database Service API client
    pub(crate) fn new(client: &'a AwsHttpClient) -> Self {
        Self {
            ops: RdsOps::new(client),
        }
    }

    /// Describes provisioned RDS instances.
    pub async fn describe_db_instances(
        &self,
        body: &DescribeDBInstancesRequest,
    ) -> Result<DescribeDBInstancesResponse> {
        self.ops.describe_db_instances(body).await
    }

    /// Returns information about DB snapshots.
    pub async fn describe_db_snapshots(
        &self,
        body: &DescribeDBSnapshotsRequest,
    ) -> Result<DescribeDBSnapshotsResponse> {
        self.ops.describe_db_snapshots(body).await
    }

    /// Returns a list of DB snapshot attribute names and values for a manual DB snapshot.
    pub async fn describe_db_snapshot_attributes(
        &self,
        body: &DescribeDBSnapshotAttributesRequest,
    ) -> Result<DescribeDBSnapshotAttributesResponse> {
        self.ops.describe_db_snapshot_attributes(body).await
    }

    /// Modifies settings for a DB instance.
    pub async fn modify_db_instance(
        &self,
        body: &ModifyDBInstanceRequest,
    ) -> Result<ModifyDBInstanceResponse> {
        self.ops.modify_db_instance(body).await
    }

    /// Stops an Amazon RDS DB instance.
    pub async fn stop_db_instance(
        &self,
        body: &StopDBInstanceRequest,
    ) -> Result<StopDBInstanceResponse> {
        self.ops.stop_db_instance(body).await
    }

    /// Starts an Amazon RDS DB instance that was stopped.
    pub async fn start_db_instance(
        &self,
        body: &StartDBInstanceRequest,
    ) -> Result<StartDBInstanceResponse> {
        self.ops.start_db_instance(body).await
    }

    /// Deletes a previously provisioned DB instance.
    pub async fn delete_db_instance(
        &self,
        body: &DeleteDBInstanceRequest,
    ) -> Result<DeleteDBInstanceResponse> {
        self.ops.delete_db_instance(body).await
    }

    /// Creates a snapshot of a DB instance.
    pub async fn create_db_snapshot(
        &self,
        body: &CreateDBSnapshotRequest,
    ) -> Result<CreateDBSnapshotResponse> {
        self.ops.create_db_snapshot(body).await
    }

    /// Deletes a DB snapshot.
    pub async fn delete_db_snapshot(
        &self,
        body: &DeleteDBSnapshotRequest,
    ) -> Result<DeleteDBSnapshotResponse> {
        self.ops.delete_db_snapshot(body).await
    }

    /// Adds an attribute and values to, or removes an attribute and values from, a manual DB snapshot.
    pub async fn modify_db_snapshot_attribute(
        &self,
        body: &ModifyDBSnapshotAttributeRequest,
    ) -> Result<ModifyDBSnapshotAttributeResponse> {
        self.ops.modify_db_snapshot_attribute(body).await
    }
}

#[cfg(test)]
mod tests {
    use crate::types::rds::*;

    #[tokio::test]
    async fn describe_db_instances_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeDBInstancesResponse><DescribeDBInstancesResult>
                <DBInstances>
                    <DBInstance>
                        <DBInstanceIdentifier>my-db</DBInstanceIdentifier>
                        <DBInstanceClass>db.t3.micro</DBInstanceClass>
                        <Engine>mysql</Engine>
                        <DBInstanceStatus>available</DBInstanceStatus>
                        <AllocatedStorage>20</AllocatedStorage>
                        <Endpoint>
                            <Address>my-db.abc.us-east-1.rds.amazonaws.com</Address>
                            <Port>3306</Port>
                        </Endpoint>
                    </DBInstance>
                </DBInstances>
            </DescribeDBInstancesResult></DescribeDBInstancesResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .rds()
            .describe_db_instances(&DescribeDBInstancesRequest {
                db_instance_identifier: Some("my-db".into()),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.db_instances.len(), 1);
        let inst = &result.db_instances[0];
        assert_eq!(inst.db_instance_identifier.as_deref(), Some("my-db"));
        assert_eq!(inst.engine.as_deref(), Some("mysql"));
        assert_eq!(inst.db_instance_status.as_deref(), Some("available"));
        assert_eq!(inst.allocated_storage, Some(20));
        let ep = inst.endpoint.as_ref().unwrap();
        assert_eq!(
            ep.address.as_deref(),
            Some("my-db.abc.us-east-1.rds.amazonaws.com")
        );
        assert_eq!(ep.port, Some(3306));
    }

    #[tokio::test]
    async fn describe_db_snapshots_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeDBSnapshotsResponse><DescribeDBSnapshotsResult>
                <DBSnapshots>
                    <DBSnapshot>
                        <DBSnapshotIdentifier>my-snap</DBSnapshotIdentifier>
                        <DBInstanceIdentifier>my-db</DBInstanceIdentifier>
                        <Engine>mysql</Engine>
                        <Status>available</Status>
                        <AllocatedStorage>20</AllocatedStorage>
                    </DBSnapshot>
                </DBSnapshots>
            </DescribeDBSnapshotsResult></DescribeDBSnapshotsResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .rds()
            .describe_db_snapshots(&DescribeDBSnapshotsRequest {
                db_snapshot_identifier: Some("my-snap".into()),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(result.db_snapshots.len(), 1);
        let snap = &result.db_snapshots[0];
        assert_eq!(snap.db_snapshot_identifier.as_deref(), Some("my-snap"));
        assert_eq!(snap.db_instance_identifier.as_deref(), Some("my-db"));
        assert_eq!(snap.engine.as_deref(), Some("mysql"));
        assert_eq!(snap.status.as_deref(), Some("available"));
    }

    #[tokio::test]
    async fn describe_db_snapshot_attributes_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DescribeDBSnapshotAttributesResponse><DescribeDBSnapshotAttributesResult>
                <DBSnapshotAttributesResult>
                    <DBSnapshotIdentifier>my-snap</DBSnapshotIdentifier>
                    <DBSnapshotAttributes>
                        <DBSnapshotAttribute>
                            <AttributeName>restore</AttributeName>
                            <AttributeValues/>
                        </DBSnapshotAttribute>
                    </DBSnapshotAttributes>
                </DBSnapshotAttributesResult>
            </DescribeDBSnapshotAttributesResult></DescribeDBSnapshotAttributesResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .rds()
            .describe_db_snapshot_attributes(&DescribeDBSnapshotAttributesRequest {
                db_snapshot_identifier: "my-snap".into(),
            })
            .await
            .unwrap();
        let attrs = result.db_snapshot_attributes_result.as_ref().unwrap();
        assert_eq!(attrs.db_snapshot_identifier.as_deref(), Some("my-snap"));
        assert_eq!(attrs.db_snapshot_attributes.len(), 1);
        assert_eq!(
            attrs.db_snapshot_attributes[0].attribute_name.as_deref(),
            Some("restore")
        );
    }

    #[tokio::test]
    async fn modify_db_instance_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<ModifyDBInstanceResponse><ModifyDBInstanceResult>
                <DBInstance>
                    <DBInstanceIdentifier>my-db</DBInstanceIdentifier>
                    <DBInstanceClass>db.t3.micro</DBInstanceClass>
                    <Engine>mysql</Engine>
                    <DBInstanceStatus>available</DBInstanceStatus>
                    <BackupRetentionPeriod>1</BackupRetentionPeriod>
                </DBInstance>
            </ModifyDBInstanceResult></ModifyDBInstanceResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .rds()
            .modify_db_instance(&ModifyDBInstanceRequest {
                db_instance_identifier: "my-db".into(),
                backup_retention_period: Some(1),
                apply_immediately: Some(true),
                ..Default::default()
            })
            .await
            .unwrap();
        let inst = result.db_instance.as_ref().unwrap();
        assert_eq!(inst.db_instance_identifier.as_deref(), Some("my-db"));
        assert_eq!(inst.db_instance_status.as_deref(), Some("available"));
        assert_eq!(inst.backup_retention_period, Some(1));
    }

    #[tokio::test]
    async fn stop_db_instance_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<StopDBInstanceResponse><StopDBInstanceResult>
                <DBInstance>
                    <DBInstanceIdentifier>my-db</DBInstanceIdentifier>
                    <DBInstanceStatus>stopping</DBInstanceStatus>
                    <Engine>mysql</Engine>
                </DBInstance>
            </StopDBInstanceResult></StopDBInstanceResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .rds()
            .stop_db_instance(&StopDBInstanceRequest {
                db_instance_identifier: "my-db".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        let inst = result.db_instance.as_ref().unwrap();
        assert_eq!(inst.db_instance_identifier.as_deref(), Some("my-db"));
        assert_eq!(inst.db_instance_status.as_deref(), Some("stopping"));
    }

    #[tokio::test]
    async fn start_db_instance_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<StartDBInstanceResponse><StartDBInstanceResult>
                <DBInstance>
                    <DBInstanceIdentifier>my-db</DBInstanceIdentifier>
                    <DBInstanceStatus>starting</DBInstanceStatus>
                    <Engine>mysql</Engine>
                </DBInstance>
            </StartDBInstanceResult></StartDBInstanceResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .rds()
            .start_db_instance(&StartDBInstanceRequest {
                db_instance_identifier: "my-db".into(),
            })
            .await
            .unwrap();
        let inst = result.db_instance.as_ref().unwrap();
        assert_eq!(inst.db_instance_identifier.as_deref(), Some("my-db"));
        assert_eq!(inst.db_instance_status.as_deref(), Some("starting"));
    }

    #[tokio::test]
    async fn delete_db_instance_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DeleteDBInstanceResponse><DeleteDBInstanceResult>
                <DBInstance>
                    <DBInstanceIdentifier>my-db</DBInstanceIdentifier>
                    <DBInstanceStatus>deleting</DBInstanceStatus>
                    <Engine>mysql</Engine>
                </DBInstance>
            </DeleteDBInstanceResult></DeleteDBInstanceResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .rds()
            .delete_db_instance(&DeleteDBInstanceRequest {
                db_instance_identifier: "my-db".into(),
                skip_final_snapshot: Some(true),
                delete_automated_backups: Some(true),
                ..Default::default()
            })
            .await
            .unwrap();
        let inst = result.db_instance.as_ref().unwrap();
        assert_eq!(inst.db_instance_identifier.as_deref(), Some("my-db"));
        assert_eq!(inst.db_instance_status.as_deref(), Some("deleting"));
    }

    #[tokio::test]
    async fn create_db_snapshot_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<CreateDBSnapshotResponse><CreateDBSnapshotResult>
                <DBSnapshot>
                    <DBSnapshotIdentifier>my-snap</DBSnapshotIdentifier>
                    <DBInstanceIdentifier>my-db</DBInstanceIdentifier>
                    <Engine>mysql</Engine>
                    <Status>creating</Status>
                    <AllocatedStorage>20</AllocatedStorage>
                </DBSnapshot>
            </CreateDBSnapshotResult></CreateDBSnapshotResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .rds()
            .create_db_snapshot(&CreateDBSnapshotRequest {
                db_snapshot_identifier: "my-snap".into(),
                db_instance_identifier: "my-db".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        let snap = result.db_snapshot.as_ref().unwrap();
        assert_eq!(snap.db_snapshot_identifier.as_deref(), Some("my-snap"));
        assert_eq!(snap.db_instance_identifier.as_deref(), Some("my-db"));
        assert_eq!(snap.status.as_deref(), Some("creating"));
    }

    #[tokio::test]
    async fn delete_db_snapshot_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<DeleteDBSnapshotResponse><DeleteDBSnapshotResult>
                <DBSnapshot>
                    <DBSnapshotIdentifier>my-snap</DBSnapshotIdentifier>
                    <DBInstanceIdentifier>my-db</DBInstanceIdentifier>
                    <Status>deleted</Status>
                </DBSnapshot>
            </DeleteDBSnapshotResult></DeleteDBSnapshotResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .rds()
            .delete_db_snapshot(&DeleteDBSnapshotRequest {
                db_snapshot_identifier: "my-snap".into(),
            })
            .await
            .unwrap();
        let snap = result.db_snapshot.as_ref().unwrap();
        assert_eq!(snap.db_snapshot_identifier.as_deref(), Some("my-snap"));
        assert_eq!(snap.status.as_deref(), Some("deleted"));
    }

    #[tokio::test]
    async fn modify_db_snapshot_attribute_returns_parsed_response() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/").returning_bytes(
            r#"<ModifyDBSnapshotAttributeResponse><ModifyDBSnapshotAttributeResult>
                <DBSnapshotAttributesResult>
                    <DBSnapshotIdentifier>my-snap</DBSnapshotIdentifier>
                    <DBSnapshotAttributes>
                        <DBSnapshotAttribute>
                            <AttributeName>restore</AttributeName>
                            <AttributeValues>
                                <AttributeValue>123456789012</AttributeValue>
                            </AttributeValues>
                        </DBSnapshotAttribute>
                    </DBSnapshotAttributes>
                </DBSnapshotAttributesResult>
            </ModifyDBSnapshotAttributeResult></ModifyDBSnapshotAttributeResponse>"#
                .as_bytes()
                .to_vec(),
        );
        let client = crate::AwsHttpClient::from_mock(mock);
        let result = client
            .rds()
            .modify_db_snapshot_attribute(&ModifyDBSnapshotAttributeRequest {
                db_snapshot_identifier: "my-snap".into(),
                attribute_name: "restore".into(),
                values_to_add: vec!["123456789012".into()],
                ..Default::default()
            })
            .await
            .unwrap();
        let attrs = result.db_snapshot_attributes_result.as_ref().unwrap();
        assert_eq!(attrs.db_snapshot_identifier.as_deref(), Some("my-snap"));
        assert_eq!(attrs.db_snapshot_attributes.len(), 1);
        assert_eq!(
            attrs.db_snapshot_attributes[0].attribute_name.as_deref(),
            Some("restore")
        );
        assert!(
            attrs.db_snapshot_attributes[0]
                .attribute_values
                .contains(&"123456789012".to_string())
        );
    }
}
