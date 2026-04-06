//! Types for the Amazon Relational Database Service API (v1).
//!
//! Auto-generated from the AWS Botocore Model.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

///
/// **AWS API**: `rds.v1.DescribeDBInstancesMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DescribeDBInstancesMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDBInstancesRequest {
    /// The user-supplied instance identifier or the Amazon Resource Name (ARN) of the DB
    /// instance. If this parameter is specified, information from only the specific DB instance
    /// is returned. This parameter isn't case-sensitive. Constraints: If supplied, must match
    /// the identifier of an existing DB instance.
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,

    /// A filter that specifies one or more DB instances to describe. Supported Filters: db-
    /// cluster-id
    /// - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The
    ///   results list only includes information about the DB instances associated with the DB
    ///   clusters identified by these ARNs. db-instance-id
    /// - Accepts DB instance identifiers and DB instance Amazon Resource Names (ARNs). The
    ///   results list only includes information about the DB instances identified by these
    ///   ARNs. dbi-resource-id
    /// - Accepts DB instance resource identifiers. The results list only includes information
    ///   about the DB instances identified by these DB instance resource identifiers. domain
    /// - Accepts Active Directory directory IDs. The results list only includes information
    ///   about the DB instances associated with these domains. engine
    /// - Accepts engine names. The results list only includes information about the DB
    ///   instances for these engines.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<Filter>,

    /// The maximum number of records to include in the response. If more records exist than the
    /// specified MaxRecords value, a pagination token called a marker is included in the
    /// response so that you can retrieve the remaining results. Default: 100 Constraints:
    /// Minimum 20, maximum 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,

    /// An optional pagination token provided by a previous DescribeDBInstances request. If this
    /// parameter is specified, the response includes only records beyond the marker, up to the
    /// value specified by MaxRecords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeDBInstancesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance_identifier: Some("test-db_instance_identifier".into()),
            filters: vec![],
            max_records: Some(100),
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains the result of a successful invocation of the DescribeDBInstances action.
///
/// **AWS API**: `rds.v1.DBInstanceMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DBInstanceMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDBInstancesResponse {
    /// A list of DBInstance instances.
    #[serde(rename = "DBInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub db_instances: Vec<DBInstance>,

    /// An optional pagination token provided by a previous request. If this parameter is
    /// specified, the response includes only records beyond the marker, up to the value
    /// specified by MaxRecords .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeDBInstancesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instances: vec![],
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains the details of an Amazon RDS DB instance. This data type is used as a response
/// element in the operations CreateDBInstance, CreateDBInstanceReadReplica, DeleteDBInstance,
/// DescribeDBInstances, ModifyDBInstance, PromoteReadReplica, RebootDBInstance,
/// RestoreDBInstanceFromDBSnapshot, RestoreDBInstanceFromS3, RestoreDBInstanceToPointInTime,
/// StartDBInstance, and StopDBInstance.
///
/// **AWS API**: `rds.v1.DBInstance`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DBInstance>
///
/// ## Coverage
/// 24 of 90 fields included.
/// Omitted fields:
/// - `DBSecurityGroups` — not selected in manifest
/// - `VpcSecurityGroups` — not selected in manifest
/// - `DBParameterGroups` — not selected in manifest
/// - `DBSubnetGroup` — not selected in manifest
/// - `UpgradeRolloutOrder` — not selected in manifest
/// - `PendingModifiedValues` — not selected in manifest
/// - `LatestRestorableTime` — not selected in manifest
/// - `ReadReplicaSourceDBInstanceIdentifier` — not selected in manifest
/// - `ReadReplicaDBInstanceIdentifiers` — not selected in manifest
/// - `ReadReplicaDBClusterIdentifiers` — not selected in manifest
/// - `ReplicaMode` — not selected in manifest
/// - `LicenseModel` — not selected in manifest
/// - `OptionGroupMemberships` — not selected in manifest
/// - `CharacterSetName` — not selected in manifest
/// - `NcharCharacterSetName` — not selected in manifest
/// - `SecondaryAvailabilityZone` — not selected in manifest
/// - `StatusInfos` — not selected in manifest
/// - `TdeCredentialArn` — not selected in manifest
/// - `DbInstancePort` — not selected in manifest
/// - `DBClusterIdentifier` — not selected in manifest
/// - `CACertificateIdentifier` — not selected in manifest
/// - `DomainMemberships` — not selected in manifest
/// - `CopyTagsToSnapshot` — not selected in manifest
/// - `MonitoringInterval` — not selected in manifest
/// - `EnhancedMonitoringResourceArn` — not selected in manifest
/// - `MonitoringRoleArn` — not selected in manifest
/// - `PromotionTier` — not selected in manifest
/// - `Timezone` — not selected in manifest
/// - `IAMDatabaseAuthenticationEnabled` — not selected in manifest
/// - `DatabaseInsightsMode` — not selected in manifest
/// - `PerformanceInsightsEnabled` — not selected in manifest
/// - `PerformanceInsightsKMSKeyId` — not selected in manifest
/// - `PerformanceInsightsRetentionPeriod` — not selected in manifest
/// - `EnabledCloudwatchLogsExports` — not selected in manifest
/// - `ProcessorFeatures` — not selected in manifest
/// - `DeletionProtection` — not selected in manifest
/// - `AssociatedRoles` — not selected in manifest
/// - `ListenerEndpoint` — not selected in manifest
/// - `MaxAllocatedStorage` — not selected in manifest
/// - `TagList` — not selected in manifest
/// - `AutomationMode` — not selected in manifest
/// - `ResumeFullAutomationModeTime` — not selected in manifest
/// - `CustomerOwnedIpEnabled` — not selected in manifest
/// - `NetworkType` — not selected in manifest
/// - `ActivityStreamStatus` — not selected in manifest
/// - `ActivityStreamKmsKeyId` — not selected in manifest
/// - `ActivityStreamKinesisStreamName` — not selected in manifest
/// - `ActivityStreamMode` — not selected in manifest
/// - `ActivityStreamEngineNativeAuditFieldsIncluded` — not selected in manifest
/// - `AwsBackupRecoveryPointArn` — not selected in manifest
/// - `DBInstanceAutomatedBackupsReplications` — not selected in manifest
/// - `BackupTarget` — not selected in manifest
/// - `AutomaticRestartTime` — not selected in manifest
/// - `CustomIamInstanceProfile` — not selected in manifest
/// - `ActivityStreamPolicyStatus` — not selected in manifest
/// - `CertificateDetails` — not selected in manifest
/// - `DBSystemId` — not selected in manifest
/// - `MasterUserSecret` — not selected in manifest
/// - `ReadReplicaSourceDBClusterIdentifier` — not selected in manifest
/// - `PercentProgress` — not selected in manifest
/// - `MultiTenant` — not selected in manifest
/// - `DedicatedLogVolume` — not selected in manifest
/// - `IsStorageConfigUpgradeAvailable` — not selected in manifest
/// - `EngineLifecycleSupport` — not selected in manifest
/// - `AdditionalStorageVolumes` — not selected in manifest
/// - `StorageVolumeStatus` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DBInstance {
    /// The user-supplied database identifier. This identifier is the unique key that identifies
    /// a DB instance.
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,

    /// The name of the compute and memory capacity class of the DB instance.
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,

    /// The database engine used for this DB instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,

    /// The current state of this database. For information about DB instance statuses, see
    /// Viewing DB instance status in the Amazon RDS User Guide.
    #[serde(rename = "DBInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_status: Option<String>,

    /// The master username for the DB instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,

    /// The initial database name that you provided (if required) when you created the DB
    /// instance. This name is returned for the life of your DB instance. For an RDS for Oracle
    /// CDB instance, the name identifies the PDB rather than the CDB.
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,

    /// The connection endpoint for the DB instance. The endpoint might not be shown for
    /// instances with the status of creating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,

    /// The amount of storage in gibibytes (GiB) allocated for the DB instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,

    /// The date and time when the DB instance was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,

    /// The daily time range during which automated backups are created if automated backups are
    /// enabled, as determined by the BackupRetentionPeriod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,

    /// The number of days for which automatic DB snapshots are retained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,

    /// The name of the Availability Zone where the DB instance is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,

    /// The weekly time range during which system maintenance can occur, in Universal
    /// Coordinated Time (UTC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,

    /// Indicates whether the DB instance is a Multi-AZ deployment. This setting doesn't apply
    /// to RDS Custom DB instances.
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,

    /// The version of the database engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,

    /// Indicates whether minor version patches are applied automatically. For more information
    /// about automatic minor version upgrades, see Automatically upgrading the minor engine
    /// version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,

    /// Indicates whether the DB instance is publicly accessible. When the DB instance is
    /// publicly accessible and you connect from outside of the DB instance's virtual private
    /// cloud (VPC), its Domain Name System (DNS) endpoint resolves to the public IP address.
    /// When you connect from within the same VPC as the DB instance, the endpoint resolves to
    /// the private IP address. Access to the DB cluster is ultimately controlled by the
    /// security group it uses. That public access isn't permitted if the security group
    /// assigned to the DB cluster doesn't permit it. When the DB instance isn't publicly
    /// accessible, it is an internal DB instance with a DNS name that resolves to a private IP
    /// address. For more information, see CreateDBInstance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,

    /// The storage type associated with the DB instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,

    /// Indicates whether the DB instance is encrypted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,

    /// If StorageEncrypted is enabled, the Amazon Web Services KMS key identifier for the
    /// encrypted DB instance. The Amazon Web Services KMS key identifier is the key ARN, key
    /// ID, alias ARN, or alias name for the KMS key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    /// The Amazon Web Services Region-unique, immutable identifier for the DB instance. This
    /// identifier is found in Amazon Web Services CloudTrail log entries whenever the Amazon
    /// Web Services KMS key for the DB instance is accessed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,

    /// The Amazon Resource Name (ARN) for the DB instance.
    #[serde(rename = "DBInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_arn: Option<String>,

    /// The Provisioned IOPS (I/O operations per second) value for the DB instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,

    /// The storage throughput for the DB instance. This setting applies only to the gp3 storage
    /// type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
}

impl DBInstance {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance_identifier: Some("test-db_instance_identifier".into()),
            db_instance_class: Some("test-db_instance_class".into()),
            engine: Some("test-engine".into()),
            db_instance_status: Some("test-db_instance_status".into()),
            master_username: Some("test-master_username".into()),
            db_name: Some("test-db_name".into()),
            endpoint: Some(Endpoint::fixture()),
            allocated_storage: Some(100),
            instance_create_time: Some("test-instance_create_time".into()),
            preferred_backup_window: Some("test-preferred_backup_window".into()),
            backup_retention_period: Some(100),
            availability_zone: Some("test-availability_zone".into()),
            preferred_maintenance_window: Some("test-preferred_maintenance_window".into()),
            multi_az: Some(false),
            engine_version: Some("test-engine_version".into()),
            auto_minor_version_upgrade: Some(false),
            publicly_accessible: Some(false),
            storage_type: Some("test-storage_type".into()),
            storage_encrypted: Some(false),
            kms_key_id: Some("test-kms_key_id".into()),
            dbi_resource_id: Some("test-dbi_resource_id".into()),
            db_instance_arn: Some("test-db_instance_arn".into()),
            iops: Some(100),
            storage_throughput: Some(100),
        }
    }
}

/// This data type represents the information you need to connect to an Amazon RDS DB instance.
/// This data type is used as a response element in the following actions: CreateDBInstance
/// DescribeDBInstances DeleteDBInstance For the data structure that represents Amazon Aurora DB
/// cluster endpoints, see DBClusterEndpoint.
///
/// **AWS API**: `rds.v1.Endpoint`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//Endpoint>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Endpoint {
    /// Specifies the DNS address of the DB instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Specifies the port that the database engine is listening on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
}

impl Endpoint {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            address: Some("test-address".into()),
            port: Some(100),
            hosted_zone_id: Some("test-hosted_zone_id".into()),
        }
    }
}

/// A filter name and value pair that is used to return a more specific list of results from a
/// describe operation. Filters can be used to match a set of resources by specific criteria,
/// such as IDs. The filters supported by a describe operation are documented with the describe
/// operation. Currently, wildcards are not supported in filters. The following actions can be
/// filtered: DescribeDBClusterBacktracks DescribeDBClusterEndpoints DescribeDBClusters
/// DescribeDBInstances DescribeDBRecommendations DescribeDBShardGroups
/// DescribePendingMaintenanceActions
///
/// **AWS API**: `rds.v1.Filter`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//Filter>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Filter {
    /// The name of the filter. Filter names are case-sensitive.
    pub name: String,

    /// One or more filter values. Filter values are case-sensitive.
    #[serde(default)]
    pub values: Vec<String>,
}

impl Filter {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-name".into(),
            values: vec![],
        }
    }
}

///
/// **AWS API**: `rds.v1.DescribeDBSnapshotsMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DescribeDBSnapshotsMessage>
///
/// ## Coverage
/// 8 of 9 fields included.
/// Omitted fields:
/// - `DbiResourceId` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDBSnapshotsRequest {
    /// The ID of the DB instance to retrieve the list of DB snapshots for. This parameter isn't
    /// case-sensitive. Constraints: If supplied, must match the identifier of an existing
    /// DBInstance.
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,

    /// A specific DB snapshot identifier to describe. This value is stored as a lowercase
    /// string. Constraints: If supplied, must match the identifier of an existing DBSnapshot.
    /// If this identifier is for an automated snapshot, the SnapshotType parameter must also be
    /// specified.
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot_identifier: Option<String>,

    /// The type of snapshots to be returned. You can specify one of the following values:
    /// automated
    /// - Return all DB snapshots that have been automatically taken by Amazon RDS for my Amazon
    ///   Web Services account. manual
    /// - Return all DB snapshots that have been taken by my Amazon Web Services account. shared
    /// - Return all manual DB snapshots that have been shared to my Amazon Web Services
    ///   account. public
    /// - Return all DB snapshots that have been marked as public. awsbackup
    /// - Return the DB snapshots managed by the Amazon Web Services Backup service. For
    ///   information about Amazon Web Services Backup, see the Amazon Web Services Backup
    ///   Developer Guide. The awsbackup type does not apply to Aurora. If you don't specify a
    ///   SnapshotType value, then both automated and manual snapshots are returned. Shared and
    ///   public DB snapshots are not included in the returned results by default. You can
    ///   include shared snapshots with these results by enabling the IncludeShared parameter.
    ///   You can include public snapshots with these results by enabling the IncludePublic
    ///   parameter. The IncludeShared and IncludePublic parameters don't apply for SnapshotType
    ///   values of manual or automated. The IncludePublic parameter doesn't apply when
    ///   SnapshotType is set to shared. The IncludeShared parameter doesn't apply when
    ///   SnapshotType is set to public.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,

    /// A filter that specifies one or more DB snapshots to describe. Supported filters: db-
    /// instance-id
    /// - Accepts DB instance identifiers and DB instance Amazon Resource Names (ARNs). db-
    ///   snapshot-id
    /// - Accepts DB snapshot identifiers. dbi-resource-id
    /// - Accepts identifiers of source DB instances. snapshot-type
    /// - Accepts types of DB snapshots. engine
    /// - Accepts names of database engines.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<Filter>,

    /// The maximum number of records to include in the response. If more records exist than the
    /// specified MaxRecords value, a pagination token called a marker is included in the
    /// response so that you can retrieve the remaining results. Default: 100 Constraints:
    /// Minimum 20, maximum 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,

    /// An optional pagination token provided by a previous DescribeDBSnapshots request. If this
    /// parameter is specified, the response includes only records beyond the marker, up to the
    /// value specified by MaxRecords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Specifies whether to include shared manual DB cluster snapshots from other Amazon Web
    /// Services accounts that this Amazon Web Services account has been given permission to
    /// copy or restore. By default, these snapshots are not included. You can give an Amazon
    /// Web Services account permission to restore a manual DB snapshot from another Amazon Web
    /// Services account by using the ModifyDBSnapshotAttribute API action. This setting doesn't
    /// apply to RDS Custom.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,

    /// Specifies whether to include manual DB cluster snapshots that are public and can be
    /// copied or restored by any Amazon Web Services account. By default, the public snapshots
    /// are not included. You can share a manual DB snapshot as public by using the
    /// ModifyDBSnapshotAttribute API. This setting doesn't apply to RDS Custom.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_public: Option<bool>,
}

impl DescribeDBSnapshotsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance_identifier: Some("test-db_instance_identifier".into()),
            db_snapshot_identifier: Some("test-db_snapshot_identifier".into()),
            snapshot_type: Some("test-snapshot_type".into()),
            filters: vec![],
            max_records: Some(100),
            marker: Some("test-marker".into()),
            include_shared: Some(false),
            include_public: Some(false),
        }
    }
}

/// Contains the result of a successful invocation of the DescribeDBSnapshots action.
///
/// **AWS API**: `rds.v1.DBSnapshotMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DBSnapshotMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDBSnapshotsResponse {
    /// A list of DBSnapshot instances.
    #[serde(rename = "DBSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub db_snapshots: Vec<DBSnapshot>,

    /// An optional pagination token provided by a previous request. If this parameter is
    /// specified, the response includes only records beyond the marker, up to the value
    /// specified by MaxRecords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeDBSnapshotsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshots: vec![],
            marker: Some("test-marker".into()),
        }
    }
}

/// Contains the details of an Amazon RDS DB snapshot. This data type is used as a response
/// element in the DescribeDBSnapshots action.
///
/// **AWS API**: `rds.v1.DBSnapshot`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DBSnapshot>
///
/// ## Coverage
/// 20 of 40 fields included.
/// Omitted fields:
/// - `LicenseModel` — not selected in manifest
/// - `OptionGroupName` — not selected in manifest
/// - `SourceRegion` — not selected in manifest
/// - `SourceDBSnapshotIdentifier` — not selected in manifest
/// - `TdeCredentialArn` — not selected in manifest
/// - `BackupRetentionPeriod` — not selected in manifest
/// - `PreferredBackupWindow` — not selected in manifest
/// - `Timezone` — not selected in manifest
/// - `IAMDatabaseAuthenticationEnabled` — not selected in manifest
/// - `ProcessorFeatures` — not selected in manifest
/// - `DbiResourceId` — not selected in manifest
/// - `TagList` — not selected in manifest
/// - `SnapshotTarget` — not selected in manifest
/// - `OriginalSnapshotCreateTime` — not selected in manifest
/// - `SnapshotDatabaseTime` — not selected in manifest
/// - `DBSystemId` — not selected in manifest
/// - `MultiTenant` — not selected in manifest
/// - `DedicatedLogVolume` — not selected in manifest
/// - `AdditionalStorageVolumes` — not selected in manifest
/// - `SnapshotAvailabilityZone` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DBSnapshot {
    /// Specifies the identifier for the DB snapshot.
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot_identifier: Option<String>,

    /// Specifies the DB instance identifier of the DB instance this DB snapshot was created
    /// from.
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,

    /// Specifies when the snapshot was taken in Coordinated Universal Time (UTC). Changes for
    /// the copy when the snapshot is copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_create_time: Option<String>,

    /// Specifies the name of the database engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,

    /// Specifies the allocated storage size in gibibytes (GiB).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,

    /// Specifies the status of this DB snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Specifies the port that the database engine was listening on at the time of the
    /// snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// Specifies the name of the Availability Zone the DB instance was located in at the time
    /// of the DB snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,

    /// Provides the VPC ID associated with the DB snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// Specifies the time in Coordinated Universal Time (UTC) when the DB instance, from which
    /// the snapshot was taken, was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,

    /// Provides the master username for the DB snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,

    /// Specifies the version of the database engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,

    /// Provides the type of the DB snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,

    /// Specifies the Provisioned IOPS (I/O operations per second) value of the DB instance at
    /// the time of the snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,

    /// Specifies the storage throughput for the DB snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,

    /// Specifies the storage type associated with DB snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,

    /// Indicates whether the DB snapshot is encrypted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,

    /// If Encrypted is true, the Amazon Web Services KMS key identifier for the encrypted DB
    /// snapshot. The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN,
    /// or alias name for the KMS key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    /// The Amazon Resource Name (ARN) for the DB snapshot.
    #[serde(rename = "DBSnapshotArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot_arn: Option<String>,

    /// The percentage of the estimated data that has been transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<i32>,
}

impl DBSnapshot {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshot_identifier: Some("test-db_snapshot_identifier".into()),
            db_instance_identifier: Some("test-db_instance_identifier".into()),
            snapshot_create_time: Some("test-snapshot_create_time".into()),
            engine: Some("test-engine".into()),
            allocated_storage: Some(100),
            status: Some("test-status".into()),
            port: Some(100),
            availability_zone: Some("test-availability_zone".into()),
            vpc_id: Some("test-vpc_id".into()),
            instance_create_time: Some("test-instance_create_time".into()),
            master_username: Some("test-master_username".into()),
            engine_version: Some("test-engine_version".into()),
            snapshot_type: Some("test-snapshot_type".into()),
            iops: Some(100),
            storage_throughput: Some(100),
            storage_type: Some("test-storage_type".into()),
            encrypted: Some(false),
            kms_key_id: Some("test-kms_key_id".into()),
            db_snapshot_arn: Some("test-db_snapshot_arn".into()),
            percent_progress: Some(100),
        }
    }
}

///
/// **AWS API**: `rds.v1.DescribeDBSnapshotAttributesMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DescribeDBSnapshotAttributesMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDBSnapshotAttributesRequest {
    /// The identifier for the DB snapshot to describe the attributes for.
    #[serde(rename = "DBSnapshotIdentifier")]
    pub db_snapshot_identifier: String,
}

impl DescribeDBSnapshotAttributesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshot_identifier: "test-db_snapshot_identifier".into(),
        }
    }
}

///
/// **AWS API**: `rds.v1.DescribeDBSnapshotAttributesResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DescribeDBSnapshotAttributesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDBSnapshotAttributesResponse {
    /// The `DBSnapshotAttributesResult` field.
    #[serde(rename = "DBSnapshotAttributesResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot_attributes_result: Option<DBSnapshotAttributesResult>,
}

impl DescribeDBSnapshotAttributesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshot_attributes_result: Some(DBSnapshotAttributesResult::fixture()),
        }
    }
}

/// Contains the results of a successful call to the DescribeDBSnapshotAttributes API action.
/// Manual DB snapshot attributes are used to authorize other Amazon Web Services accounts to
/// copy or restore a manual DB snapshot. For more information, see the
/// ModifyDBSnapshotAttribute API action.
///
/// **AWS API**: `rds.v1.DBSnapshotAttributesResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DBSnapshotAttributesResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DBSnapshotAttributesResult {
    /// The identifier of the manual DB snapshot that the attributes apply to.
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot_identifier: Option<String>,

    /// The list of attributes and values for the manual DB snapshot.
    #[serde(rename = "DBSnapshotAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub db_snapshot_attributes: Vec<DBSnapshotAttribute>,
}

impl DBSnapshotAttributesResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshot_identifier: Some("test-db_snapshot_identifier".into()),
            db_snapshot_attributes: vec![],
        }
    }
}

/// Contains the name and values of a manual DB snapshot attribute Manual DB snapshot attributes
/// are used to authorize other Amazon Web Services accounts to restore a manual DB snapshot.
/// For more information, see the ModifyDBSnapshotAttribute API.
///
/// **AWS API**: `rds.v1.DBSnapshotAttribute`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DBSnapshotAttribute>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DBSnapshotAttribute {
    /// The name of the manual DB snapshot attribute. The attribute named restore refers to the
    /// list of Amazon Web Services accounts that have permission to copy or restore the manual
    /// DB cluster snapshot. For more information, see the ModifyDBSnapshotAttribute API action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,

    /// The value or values for the manual DB snapshot attribute. If the AttributeName field is
    /// set to restore, then this element returns a list of IDs of the Amazon Web Services
    /// accounts that are authorized to copy or restore the manual DB snapshot. If a value of
    /// all is in the list, then the manual DB snapshot is public and available for any Amazon
    /// Web Services account to copy or restore.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attribute_values: Vec<String>,
}

impl DBSnapshotAttribute {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            attribute_name: Some("test-attribute_name".into()),
            attribute_values: vec![],
        }
    }
}

///
/// **AWS API**: `rds.v1.ModifyDBInstanceMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//ModifyDBInstanceMessage>
///
/// ## Coverage
/// 15 of 64 fields included.
/// Omitted fields:
/// - `DBSubnetGroupName` — not selected in manifest
/// - `DBSecurityGroups` — not selected in manifest
/// - `VpcSecurityGroupIds` — not selected in manifest
/// - `DBParameterGroupName` — not selected in manifest
/// - `AllowMajorVersionUpgrade` — not selected in manifest
/// - `LicenseModel` — not selected in manifest
/// - `OptionGroupName` — not selected in manifest
/// - `NewDBInstanceIdentifier` — not selected in manifest
/// - `TdeCredentialArn` — not selected in manifest
/// - `TdeCredentialPassword` — not selected in manifest
/// - `CACertificateIdentifier` — not selected in manifest
/// - `Domain` — not selected in manifest
/// - `DomainFqdn` — not selected in manifest
/// - `DomainOu` — not selected in manifest
/// - `DomainAuthSecretArn` — not selected in manifest
/// - `DomainDnsIps` — not selected in manifest
/// - `DisableDomain` — not selected in manifest
/// - `CopyTagsToSnapshot` — not selected in manifest
/// - `MonitoringInterval` — not selected in manifest
/// - `DBPortNumber` — not selected in manifest
/// - `MonitoringRoleArn` — not selected in manifest
/// - `DomainIAMRoleName` — not selected in manifest
/// - `PromotionTier` — not selected in manifest
/// - `EnableIAMDatabaseAuthentication` — not selected in manifest
/// - `DatabaseInsightsMode` — not selected in manifest
/// - `EnablePerformanceInsights` — not selected in manifest
/// - `PerformanceInsightsKMSKeyId` — not selected in manifest
/// - `PerformanceInsightsRetentionPeriod` — not selected in manifest
/// - `CloudwatchLogsExportConfiguration` — not selected in manifest
/// - `ProcessorFeatures` — not selected in manifest
/// - `UseDefaultProcessorFeatures` — not selected in manifest
/// - `DeletionProtection` — not selected in manifest
/// - `MaxAllocatedStorage` — not selected in manifest
/// - `CertificateRotationRestart` — not selected in manifest
/// - `ReplicaMode` — not selected in manifest
/// - `AutomationMode` — not selected in manifest
/// - `ResumeFullAutomationModeMinutes` — not selected in manifest
/// - `EnableCustomerOwnedIp` — not selected in manifest
/// - `NetworkType` — not selected in manifest
/// - `AwsBackupRecoveryPointArn` — not selected in manifest
/// - `ManageMasterUserPassword` — not selected in manifest
/// - `RotateMasterUserPassword` — not selected in manifest
/// - `MasterUserSecretKmsKeyId` — not selected in manifest
/// - `MultiTenant` — not selected in manifest
/// - `DedicatedLogVolume` — not selected in manifest
/// - `Engine` — not selected in manifest
/// - `AdditionalStorageVolumes` — not selected in manifest
/// - `TagSpecifications` — not selected in manifest
/// - `MasterUserAuthenticationType` — not selected in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModifyDBInstanceRequest {
    /// The identifier of DB instance to modify. This value is stored as a lowercase string.
    /// Constraints: Must match the identifier of an existing DB instance.
    #[serde(rename = "DBInstanceIdentifier")]
    pub db_instance_identifier: String,

    /// The new amount of storage in gibibytes (GiB) to allocate for the DB instance. For RDS
    /// for Db2, MariaDB, RDS for MySQL, RDS for Oracle, and RDS for PostgreSQL, the value
    /// supplied must be at least 10% greater than the current value. Values that are not at
    /// least 10% greater than the existing value are rounded up so that they are 10% greater
    /// than the current value. For the valid values for allocated storage for each engine, see
    /// CreateDBInstance. Constraints: When you increase the allocated storage for a DB instance
    /// that uses Provisioned IOPS (gp3, io1, or io2 storage type), you must also specify the
    /// Iops parameter. You can use the current value for Iops.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,

    /// The new compute and memory capacity of the DB instance, for example db.m4.large. Not all
    /// DB instance classes are available in all Amazon Web Services Regions, or for all
    /// database engines. For the full list of DB instance classes, and availability for your
    /// engine, see DB Instance Class in the Amazon RDS User Guide or Aurora DB instance classes
    /// in the Amazon Aurora User Guide. For RDS Custom, see DB instance class support for RDS
    /// Custom for Oracle and DB instance class support for RDS Custom for SQL Server. If you
    /// modify the DB instance class, an outage occurs during the change. The change is applied
    /// during the next maintenance window, unless you specify ApplyImmediately in your request.
    /// Default: Uses existing setting Constraints: If you are modifying the DB instance class
    /// and upgrading the engine version at the same time, the currently running engine version
    /// must be supported on the specified DB instance class. Otherwise, the operation returns
    /// an error. In this case, first run the operation to upgrade the engine version, and then
    /// run it again to modify the DB instance class.
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,

    /// Specifies whether the modifications in this request and any pending modifications are
    /// asynchronously applied as soon as possible, regardless of the PreferredMaintenanceWindow
    /// setting for the DB instance. By default, this parameter is disabled. If this parameter
    /// is disabled, changes to the DB instance are applied during the next maintenance window.
    /// Some parameter changes can cause an outage and are applied on the next call to
    /// RebootDBInstance, or the next failure reboot. Review the table of parameters in
    /// Modifying a DB Instance in the Amazon RDS User Guide to see the impact of enabling or
    /// disabling ApplyImmediately for each modified parameter and to determine when the changes
    /// are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,

    /// The new password for the master user. Changing this parameter doesn't result in an
    /// outage and the change is asynchronously applied as soon as possible. Between the time of
    /// the request and the completion of the request, the MasterUserPassword element exists in
    /// the PendingModifiedValues element of the operation response. Amazon RDS API operations
    /// never return the password, so this operation provides a way to regain access to a
    /// primary instance user if the password is lost. This includes restoring privileges that
    /// might have been accidentally revoked. This setting doesn't apply to the following DB
    /// instances: Amazon Aurora The password for the master user is managed by the DB cluster.
    /// For more information, see ModifyDBCluster. RDS Custom RDS for Oracle CDBs in the multi-
    /// tenant configuration Specify the master password in ModifyTenantDatabase instead.
    /// Default: Uses existing setting Constraints: Can't be specified if
    /// ManageMasterUserPassword is turned on. Can include any printable ASCII character except
    /// "/", """, or "@". For RDS for Oracle, can't include the "&amp;" (ampersand) or the "'"
    /// (single quotes) character. Length Constraints: RDS for Db2
    /// - Must contain from 8 to 255 characters. RDS for MariaDB
    /// - Must contain from 8 to 41 characters. RDS for Microsoft SQL Server
    /// - Must contain from 8 to 128 characters. RDS for MySQL
    /// - Must contain from 8 to 41 characters. RDS for Oracle
    /// - Must contain from 8 to 30 characters. RDS for PostgreSQL
    /// - Must contain from 8 to 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,

    /// The number of days to retain automated backups. Setting this parameter to a positive
    /// number enables backups. Setting this parameter to 0 disables automated backups. Enabling
    /// and disabling backups can result in a brief I/O suspension that lasts from a few seconds
    /// to a few minutes, depending on the size and class of your DB instance. These changes are
    /// applied during the next maintenance window unless the ApplyImmediately parameter is
    /// enabled for this request. If you change the parameter from one non-zero value to another
    /// non-zero value, the change is asynchronously applied as soon as possible. This setting
    /// doesn't apply to Amazon Aurora DB instances. The retention period for automated backups
    /// is managed by the DB cluster. For more information, see ModifyDBCluster. Default: Uses
    /// existing setting Constraints: Must be a value from 0 to 35. Can't be set to 0 if the DB
    /// instance is a source to read replicas. Can't be set to 0 for an RDS Custom for Oracle DB
    /// instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,

    /// The daily time range during which automated backups are created if automated backups are
    /// enabled, as determined by the BackupRetentionPeriod parameter. Changing this parameter
    /// doesn't result in an outage and the change is asynchronously applied as soon as
    /// possible. The default is a 30-minute window selected at random from an 8-hour block of
    /// time for each Amazon Web Services Region. For more information, see Backup window in the
    /// Amazon RDS User Guide. This setting doesn't apply to Amazon Aurora DB instances. The
    /// daily time range for creating automated backups is managed by the DB cluster. For more
    /// information, see ModifyDBCluster. Constraints: Must be in the format hh24:mi-hh24:mi.
    /// Must be in Universal Coordinated Time (UTC). Must not conflict with the preferred
    /// maintenance window. Must be at least 30 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,

    /// The weekly time range during which system maintenance can occur, which might result in
    /// an outage. Changing this parameter doesn't result in an outage, except in the following
    /// situation, and the change is asynchronously applied as soon as possible. If there are
    /// pending actions that cause a reboot, and the maintenance window is changed to include
    /// the current time, then changing this parameter causes a reboot of the DB instance. If
    /// you change this window to the current time, there must be at least 30 minutes between
    /// the current time and end of the window to ensure pending changes are applied. For more
    /// information, see Amazon RDS Maintenance Window in the Amazon RDS User Guide. Default:
    /// Uses existing setting Constraints: Must be in the format ddd:hh24:mi-ddd:hh24:mi. The
    /// day values must be mon | tue | wed | thu | fri | sat | sun. Must be in Universal
    /// Coordinated Time (UTC). Must not conflict with the preferred backup window. Must be at
    /// least 30 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,

    /// Specifies whether the DB instance is a Multi-AZ deployment. Changing this parameter
    /// doesn't result in an outage. The change is applied during the next maintenance window
    /// unless the ApplyImmediately parameter is enabled for this request. This setting doesn't
    /// apply to RDS Custom DB instances.
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,

    /// The version number of the database engine to upgrade to. Changing this parameter results
    /// in an outage and the change is applied during the next maintenance window unless the
    /// ApplyImmediately parameter is enabled for this request. For major version upgrades, if a
    /// nondefault DB parameter group is currently in use, a new DB parameter group in the DB
    /// parameter group family for the new engine version must be specified. The new DB
    /// parameter group can be the default for that DB parameter group family. If you specify
    /// only a major version, Amazon RDS updates the DB instance to the default minor version if
    /// the current minor version is lower. For information about valid engine versions, see
    /// CreateDBInstance, or call DescribeDBEngineVersions. If the instance that you're
    /// modifying is acting as a read replica, the engine version that you specify must be the
    /// same or higher than the version that the source DB instance or cluster is running. In
    /// RDS Custom for Oracle, this parameter is supported for read replicas only if they are in
    /// the PATCH_DB_FAILURE lifecycle. Constraints: If you are upgrading the engine version and
    /// modifying the DB instance class at the same time, the currently running engine version
    /// must be supported on the specified DB instance class. Otherwise, the operation returns
    /// an error. In this case, first run the operation to upgrade the engine version, and then
    /// run it again to modify the DB instance class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,

    /// Specifies whether minor version upgrades are applied automatically to the DB instance
    /// during the maintenance window. An outage occurs when all the following conditions are
    /// met: The automatic upgrade is enabled for the maintenance window. A newer minor version
    /// is available. RDS has enabled automatic patching for the engine version. If any of the
    /// preceding conditions isn't met, Amazon RDS applies the change as soon as possible and
    /// doesn't cause an outage. For an RDS Custom DB instance, don't enable this setting.
    /// Otherwise, the operation returns an error. For more information about automatic minor
    /// version upgrades, see Automatically upgrading the minor engine version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,

    /// The new Provisioned IOPS (I/O operations per second) value for the RDS instance.
    /// Changing this setting doesn't result in an outage and the change is applied during the
    /// next maintenance window unless the ApplyImmediately parameter is enabled for this
    /// request. If you are migrating from Provisioned IOPS to standard storage, set this value
    /// to 0. The DB instance will require a reboot for the change in storage type to take
    /// effect. If you choose to migrate your DB instance from using standard storage to
    /// Provisioned IOPS (io1), or from Provisioned IOPS to standard storage, the process can
    /// take time. The duration of the migration depends on several factors such as database
    /// load, storage size, storage type (standard or Provisioned IOPS), amount of IOPS
    /// provisioned (if any), and the number of prior scale storage operations. Typical
    /// migration times are under 24 hours, but the process can take up to several days in some
    /// cases. During the migration, the DB instance is available for use, but might experience
    /// performance degradation. While the migration takes place, nightly backups for the
    /// instance are suspended. No other Amazon RDS operations can take place for the instance,
    /// including modifying the instance, rebooting the instance, deleting the instance,
    /// creating a read replica for the instance, and creating a DB snapshot of the instance.
    /// Constraints: For RDS for MariaDB, RDS for MySQL, RDS for Oracle, and RDS for PostgreSQL
    /// - The value supplied must be at least 10% greater than the current value. Values that
    ///   are not at least 10% greater than the existing value are rounded up so that they are
    ///   10% greater than the current value. When you increase the Provisioned IOPS, you must
    ///   also specify the AllocatedStorage parameter. You can use the current value for
    ///   AllocatedStorage. Default: Uses existing setting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,

    /// The storage type to associate with the DB instance. If you specify io1, io2, or gp3 you
    /// must also include a value for the Iops parameter. If you choose to migrate your DB
    /// instance from using standard storage to gp2 (General Purpose SSD), gp3, or Provisioned
    /// IOPS (io1), or from these storage types to standard storage, the process can take time.
    /// The duration of the migration depends on several factors such as database load, storage
    /// size, storage type (standard or Provisioned IOPS), amount of IOPS provisioned (if any),
    /// and the number of prior scale storage operations. Typical migration times are under 24
    /// hours, but the process can take up to several days in some cases. During the migration,
    /// the DB instance is available for use, but might experience performance degradation.
    /// While the migration takes place, nightly backups for the instance are suspended. No
    /// other Amazon RDS operations can take place for the instance, including modifying the
    /// instance, rebooting the instance, deleting the instance, creating a read replica for the
    /// instance, and creating a DB snapshot of the instance. Valid Values: gp2 | gp3 | io1 |
    /// io2 | standard Default: io1, if the Iops parameter is specified. Otherwise, gp2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,

    /// Specifies whether the DB instance is publicly accessible. When the DB instance is
    /// publicly accessible and you connect from outside of the DB instance's virtual private
    /// cloud (VPC), its Domain Name System (DNS) endpoint resolves to the public IP address.
    /// When you connect from within the same VPC as the DB instance, the endpoint resolves to
    /// the private IP address. Access to the DB instance is ultimately controlled by the
    /// security group it uses. That public access isn't permitted if the security group
    /// assigned to the DB instance doesn't permit it. When the DB instance isn't publicly
    /// accessible, it is an internal DB instance with a DNS name that resolves to a private IP
    /// address. PubliclyAccessible only applies to DB instances in a VPC. The DB instance must
    /// be part of a public subnet and PubliclyAccessible must be enabled for it to be publicly
    /// accessible. Changes to the PubliclyAccessible parameter are applied immediately
    /// regardless of the value of the ApplyImmediately parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,

    /// The storage throughput value for the DB instance. This setting applies only to the gp3
    /// storage type. This setting doesn't apply to Amazon Aurora or RDS Custom DB instances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
}

impl ModifyDBInstanceRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance_identifier: "test-db_instance_identifier".into(),
            allocated_storage: Some(100),
            db_instance_class: Some("test-db_instance_class".into()),
            apply_immediately: Some(false),
            master_user_password: Some("test-master_user_password".into()),
            backup_retention_period: Some(100),
            preferred_backup_window: Some("test-preferred_backup_window".into()),
            preferred_maintenance_window: Some("test-preferred_maintenance_window".into()),
            multi_az: Some(false),
            engine_version: Some("test-engine_version".into()),
            auto_minor_version_upgrade: Some(false),
            iops: Some(100),
            storage_type: Some("test-storage_type".into()),
            publicly_accessible: Some(false),
            storage_throughput: Some(100),
        }
    }
}

///
/// **AWS API**: `rds.v1.ModifyDBInstanceResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//ModifyDBInstanceResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModifyDBInstanceResponse {
    /// The `DBInstance` field.
    #[serde(rename = "DBInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance: Option<DBInstance>,
}

impl ModifyDBInstanceResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance: Some(DBInstance::fixture()),
        }
    }
}

///
/// **AWS API**: `rds.v1.StopDBInstanceMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//StopDBInstanceMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopDBInstanceRequest {
    /// The user-supplied instance identifier.
    #[serde(rename = "DBInstanceIdentifier")]
    pub db_instance_identifier: String,

    /// The user-supplied instance identifier of the DB Snapshot created immediately before the
    /// DB instance is stopped.
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot_identifier: Option<String>,
}

impl StopDBInstanceRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance_identifier: "test-db_instance_identifier".into(),
            db_snapshot_identifier: Some("test-db_snapshot_identifier".into()),
        }
    }
}

///
/// **AWS API**: `rds.v1.StopDBInstanceResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//StopDBInstanceResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopDBInstanceResponse {
    /// The `DBInstance` field.
    #[serde(rename = "DBInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance: Option<DBInstance>,
}

impl StopDBInstanceResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance: Some(DBInstance::fixture()),
        }
    }
}

///
/// **AWS API**: `rds.v1.StartDBInstanceMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//StartDBInstanceMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartDBInstanceRequest {
    /// The user-supplied instance identifier.
    #[serde(rename = "DBInstanceIdentifier")]
    pub db_instance_identifier: String,
}

impl StartDBInstanceRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance_identifier: "test-db_instance_identifier".into(),
        }
    }
}

///
/// **AWS API**: `rds.v1.StartDBInstanceResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//StartDBInstanceResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartDBInstanceResponse {
    /// The `DBInstance` field.
    #[serde(rename = "DBInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance: Option<DBInstance>,
}

impl StartDBInstanceResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance: Some(DBInstance::fixture()),
        }
    }
}

///
/// **AWS API**: `rds.v1.DeleteDBInstanceMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DeleteDBInstanceMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDBInstanceRequest {
    /// The DB instance identifier for the DB instance to be deleted. This parameter isn't case-
    /// sensitive. Constraints: Must match the name of an existing DB instance.
    #[serde(rename = "DBInstanceIdentifier")]
    pub db_instance_identifier: String,

    /// Specifies whether to skip the creation of a final DB snapshot before deleting the
    /// instance. If you enable this parameter, RDS doesn't create a DB snapshot. If you don't
    /// enable this parameter, RDS creates a DB snapshot before the DB instance is deleted. By
    /// default, skip isn't enabled, and the DB snapshot is created. If you don't enable this
    /// parameter, you must specify the FinalDBSnapshotIdentifier parameter. When a DB instance
    /// is in a failure state and has a status of failed, incompatible-restore, or incompatible-
    /// network, RDS can delete the instance only if you enable this parameter. If you delete a
    /// read replica or an RDS Custom instance, you must enable this setting. This setting is
    /// required for RDS Custom.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_snapshot: Option<bool>,

    /// The DBSnapshotIdentifier of the new DBSnapshot created when the SkipFinalSnapshot
    /// parameter is disabled. If you enable this parameter and also enable SkipFinalShapshot,
    /// the command results in an error. This setting doesn't apply to RDS Custom. Constraints:
    /// Must be 1 to 255 letters or numbers. First character must be a letter. Can't end with a
    /// hyphen or contain two consecutive hyphens. Can't be specified when deleting a read
    /// replica.
    #[serde(rename = "FinalDBSnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_db_snapshot_identifier: Option<String>,

    /// Specifies whether to remove automated backups immediately after the DB instance is
    /// deleted. This parameter isn't case-sensitive. The default is to remove automated backups
    /// immediately after the DB instance is deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_automated_backups: Option<bool>,
}

impl DeleteDBInstanceRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance_identifier: "test-db_instance_identifier".into(),
            skip_final_snapshot: Some(false),
            final_db_snapshot_identifier: Some("test-final_db_snapshot_identifier".into()),
            delete_automated_backups: Some(false),
        }
    }
}

///
/// **AWS API**: `rds.v1.DeleteDBInstanceResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DeleteDBInstanceResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDBInstanceResponse {
    /// The `DBInstance` field.
    #[serde(rename = "DBInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance: Option<DBInstance>,
}

impl DeleteDBInstanceResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_instance: Some(DBInstance::fixture()),
        }
    }
}

///
/// **AWS API**: `rds.v1.CreateDBSnapshotMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//CreateDBSnapshotMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDBSnapshotRequest {
    /// The identifier for the DB snapshot. Constraints: Can't be null, empty, or blank Must
    /// contain from 1 to 255 letters, numbers, or hyphens First character must be a letter
    /// Can't end with a hyphen or contain two consecutive hyphens Example: my-snapshot-id
    #[serde(rename = "DBSnapshotIdentifier")]
    pub db_snapshot_identifier: String,

    /// The identifier of the DB instance that you want to create the snapshot of. Constraints:
    /// Must match the identifier of an existing DBInstance.
    #[serde(rename = "DBInstanceIdentifier")]
    pub db_instance_identifier: String,

    /// The `Tags` field.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

impl CreateDBSnapshotRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshot_identifier: "test-db_snapshot_identifier".into(),
            db_instance_identifier: "test-db_instance_identifier".into(),
            tags: vec![],
        }
    }
}

///
/// **AWS API**: `rds.v1.CreateDBSnapshotResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//CreateDBSnapshotResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDBSnapshotResponse {
    /// The `DBSnapshot` field.
    #[serde(rename = "DBSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot: Option<DBSnapshot>,
}

impl CreateDBSnapshotResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshot: Some(DBSnapshot::fixture()),
        }
    }
}

///
/// **AWS API**: `rds.v1.DeleteDBSnapshotMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DeleteDBSnapshotMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDBSnapshotRequest {
    /// The DB snapshot identifier. Constraints: Must be the name of an existing DB snapshot in
    /// the available state.
    #[serde(rename = "DBSnapshotIdentifier")]
    pub db_snapshot_identifier: String,
}

impl DeleteDBSnapshotRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshot_identifier: "test-db_snapshot_identifier".into(),
        }
    }
}

///
/// **AWS API**: `rds.v1.DeleteDBSnapshotResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//DeleteDBSnapshotResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDBSnapshotResponse {
    /// The `DBSnapshot` field.
    #[serde(rename = "DBSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot: Option<DBSnapshot>,
}

impl DeleteDBSnapshotResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshot: Some(DBSnapshot::fixture()),
        }
    }
}

///
/// **AWS API**: `rds.v1.ModifyDBSnapshotAttributeMessage`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//ModifyDBSnapshotAttributeMessage>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModifyDBSnapshotAttributeRequest {
    /// The identifier for the DB snapshot to modify the attributes for.
    #[serde(rename = "DBSnapshotIdentifier")]
    pub db_snapshot_identifier: String,

    /// The name of the DB snapshot attribute to modify. To manage authorization for other
    /// Amazon Web Services accounts to copy or restore a manual DB snapshot, set this value to
    /// restore. To view the list of attributes available to modify, use the
    /// DescribeDBSnapshotAttributes API operation.
    pub attribute_name: String,

    /// A list of DB snapshot attributes to add to the attribute specified by AttributeName. To
    /// authorize other Amazon Web Services accounts to copy or restore a manual snapshot, set
    /// this list to include one or more Amazon Web Services account IDs, or all to make the
    /// manual DB snapshot restorable by any Amazon Web Services account. Do not add the all
    /// value for any manual DB snapshots that contain private information that you don't want
    /// available to all Amazon Web Services accounts.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values_to_add: Vec<String>,

    /// A list of DB snapshot attributes to remove from the attribute specified by
    /// AttributeName. To remove authorization for other Amazon Web Services accounts to copy or
    /// restore a manual snapshot, set this list to include one or more Amazon Web Services
    /// account identifiers, or all to remove authorization for any Amazon Web Services account
    /// to copy or restore the DB snapshot. If you specify all, an Amazon Web Services account
    /// whose account ID is explicitly added to the restore attribute can still copy or restore
    /// the manual DB snapshot.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values_to_remove: Vec<String>,
}

impl ModifyDBSnapshotAttributeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshot_identifier: "test-db_snapshot_identifier".into(),
            attribute_name: "test-attribute_name".into(),
            values_to_add: vec![],
            values_to_remove: vec![],
        }
    }
}

///
/// **AWS API**: `rds.v1.ModifyDBSnapshotAttributeResult`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//ModifyDBSnapshotAttributeResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModifyDBSnapshotAttributeResponse {
    /// The `DBSnapshotAttributesResult` field.
    #[serde(rename = "DBSnapshotAttributesResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot_attributes_result: Option<DBSnapshotAttributesResult>,
}

impl ModifyDBSnapshotAttributeResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            db_snapshot_attributes_result: Some(DBSnapshotAttributesResult::fixture()),
        }
    }
}

/// Metadata assigned to an Amazon RDS resource consisting of a key-value pair. For more
/// information, see Tagging Amazon RDS resources in the Amazon RDS User Guide or Tagging Amazon
/// Aurora and Amazon RDS resources in the Amazon Aurora User Guide.
///
/// **AWS API**: `rds.v1.Tag`
/// **Reference**: <https://docs.aws.amazon.com/AmazonRDS/latest/APIReference//Tag>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    /// A key is the required name of the tag. The string value can be from 1 to 128 Unicode
    /// characters in length and can't be prefixed with aws: or rds:. The string can only
    /// contain only the set of Unicode letters, digits, white-space, '_', '.', ':', '/', '=',
    /// '+', '-', '@' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-@]*)$").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// A value is the optional value of the tag. The string value can be from 1 to 256 Unicode
    /// characters in length and can't be prefixed with aws: or rds:. The string can only
    /// contain only the set of Unicode letters, digits, white-space, '_', '.', ':', '/', '=',
    /// '+', '-', '@' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-@]*)$").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Tag {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key: Some("test-key".into()),
            value: Some("test-value".into()),
        }
    }
}
