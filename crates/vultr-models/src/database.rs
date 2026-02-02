//! Managed Database model types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Managed Database
#[derive(Serialize, Deserialize)]
pub struct Database {
    /// Unique ID for the database
    pub id: String,
    /// Date of creation
    pub date_created: Option<String>,
    /// Plan name
    pub plan: Option<String>,
    /// Disk size in GB
    pub plan_disk: Option<i32>,
    /// RAM in MB
    pub plan_ram: Option<i32>,
    /// Number of vCPUs
    pub plan_vcpus: Option<i32>,
    /// Number of replica nodes
    pub plan_replicas: Option<i32>,
    /// Number of brokers (Kafka only)
    pub plan_brokers: Option<i32>,
    /// Region ID
    pub region: Option<String>,
    /// Database engine type (MySQL, PostgreSQL, Valkey, Kafka)
    pub database_engine: Option<String>,
    /// Database engine version
    pub database_engine_version: Option<String>,
    /// VPC ID
    pub vpc_id: Option<String>,
    /// Current status
    pub status: Option<String>,
    /// User-supplied label
    pub label: Option<String>,
    /// User-supplied tag
    pub tag: Option<String>,
    /// Default database name
    pub dbname: Option<String>,
    /// Public hostname (or private if VPC attached)
    pub host: Option<String>,
    /// Public hostname when VPC attached
    pub public_host: Option<String>,
    /// Default user
    pub user: Option<String>,
    /// Default user's password
    pub password: Option<String>,
    /// Access key (Kafka only)
    pub access_key: Option<String>,
    /// Access certificate (Kafka only)
    pub access_cert: Option<String>,
    /// Connection port
    pub port: Option<String>,
    /// SASL port (Kafka only)
    pub sasl_port: Option<String>,
    /// Kafka REST enabled
    #[serde(default)]
    pub enable_kafka_rest: bool,
    /// Kafka REST URI
    pub kafka_rest_uri: Option<String>,
    /// Schema Registry enabled
    #[serde(default)]
    pub enable_schema_registry: bool,
    /// Schema Registry URI
    pub schema_registry_uri: Option<String>,
    /// Kafka Connect enabled
    #[serde(default)]
    pub enable_kafka_connect: bool,
    /// Maintenance day of week
    pub maintenance_dow: Option<String>,
    /// Maintenance time
    pub maintenance_time: Option<String>,
    /// Backup hour
    pub backup_hour: Option<String>,
    /// Backup minute
    pub backup_minute: Option<String>,
    /// Latest backup date
    pub latest_backup: Option<String>,
    /// Trusted IP addresses
    #[serde(default)]
    pub trusted_ips: Vec<String>,
    /// CA certificate
    pub ca_certificate: Option<String>,
    /// MySQL SQL modes
    #[serde(default)]
    pub mysql_sql_modes: Vec<String>,
    /// MySQL require primary key
    #[serde(default)]
    pub mysql_require_primary_key: bool,
    /// MySQL slow query log
    #[serde(default)]
    pub mysql_slow_query_log: bool,
    /// MySQL long query time
    pub mysql_long_query_time: Option<i32>,
    /// PostgreSQL available extensions
    #[serde(default)]
    pub pg_available_extensions: Vec<serde_json::Value>,
    /// Valkey eviction policy
    pub eviction_policy: Option<String>,
    /// Cluster time zone
    pub cluster_time_zone: Option<String>,
    /// Read replicas
    #[serde(default)]
    pub read_replicas: Vec<serde_json::Value>,
}

/// Database user
#[derive(Serialize, Deserialize)]
pub struct DatabaseUser {
    /// Username
    pub username: Option<String>,
    /// Password
    pub password: Option<String>,
    /// Password encryption (MySQL only)
    pub encryption: Option<String>,
    /// Access control (Valkey only)
    pub access_control: Option<UserAccessControl>,
    /// Permission (Kafka only)
    pub permission: Option<String>,
    /// Access key (Kafka only)
    pub access_key: Option<String>,
    /// Access certificate (Kafka only)
    pub access_cert: Option<String>,
}

/// User access control settings (Valkey)
#[derive(Serialize, Deserialize)]
pub struct UserAccessControl {
    /// ACL categories
    #[serde(default)]
    pub acl_categories: Vec<String>,
    /// ACL channels
    #[serde(default)]
    pub acl_channels: Vec<String>,
    /// ACL commands
    #[serde(default)]
    pub acl_commands: Vec<String>,
    /// ACL keys
    #[serde(default)]
    pub acl_keys: Vec<String>,
}

/// Logical database
#[derive(Serialize, Deserialize)]
pub struct LogicalDatabase {
    /// Database name
    pub name: Option<String>,
}

/// Connection pool (PostgreSQL)
#[derive(Serialize, Deserialize)]
pub struct ConnectionPool {
    /// Pool name
    pub name: Option<String>,
    /// Associated database
    pub database: Option<String>,
    /// Associated user
    pub username: Option<String>,
    /// Pool mode (session, transaction, statement)
    pub mode: Option<String>,
    /// Pool size
    pub size: Option<i32>,
}

/// Database connection counts
#[derive(Serialize, Deserialize)]
pub struct DatabaseConnections {
    /// Used connections
    pub used: Option<i32>,
    /// Available connections
    pub available: Option<i32>,
    /// Max connections
    pub max: Option<i32>,
}

/// Kafka topic
#[derive(Serialize, Deserialize)]
pub struct KafkaTopic {
    /// Topic name
    pub name: Option<String>,
    /// Number of partitions
    pub partitions: Option<i32>,
    /// Replication factor
    pub replication: Option<i32>,
    /// Retention hours
    pub retention_hours: Option<i32>,
    /// Retention bytes
    pub retention_bytes: Option<i64>,
}

/// Kafka connector
#[derive(Serialize, Deserialize)]
pub struct KafkaConnector {
    /// Connector name
    pub name: Option<String>,
    /// Connector class
    pub class: Option<String>,
    /// Comma-separated topics
    pub topics: Option<String>,
    /// Connector configuration
    pub config: Option<serde_json::Value>,
}

/// Kafka user permissions
#[derive(Serialize, Deserialize)]
pub struct KafkaPermissions {
    /// Permission level (admin, read, write, readwrite)
    pub permission: Option<String>,
}

/// Connector status
#[derive(Serialize, Deserialize)]
pub struct ConnectorStatus {
    /// Connector status details
    pub connector: Option<ConnectorStatusDetails>,
    /// Task statuses
    #[serde(default)]
    pub tasks: Vec<ConnectorTaskStatus>,
}

/// Connector status details
#[derive(Serialize, Deserialize)]
pub struct ConnectorStatusDetails {
    /// State (running, paused, etc.)
    pub state: Option<String>,
}

/// Connector task status
#[derive(Serialize, Deserialize)]
pub struct ConnectorTaskStatus {
    /// Task ID
    pub id: Option<i32>,
    /// Task state
    pub state: Option<String>,
    /// Trace (error info)
    pub trace: Option<String>,
}

/// Available connector
#[derive(Serialize, Deserialize)]
pub struct AvailableConnector {
    /// Connector class name
    pub class: Option<String>,
    /// Connector type
    #[serde(rename = "type")]
    pub connector_type: Option<String>,
    /// Version
    pub version: Option<String>,
}

/// Connector configuration schema entry
#[derive(Serialize, Deserialize)]
pub struct DatabaseConnectorConfigurationSchema {
    /// Option name
    pub name: Option<String>,
    /// Option type
    #[serde(rename = "type")]
    pub option_type: Option<String>,
    /// Required flag
    pub required: Option<bool>,
    /// Default value
    pub default_value: Option<String>,
    /// Description
    pub description: Option<String>,
}

/// Available advanced option descriptor
#[derive(Serialize, Deserialize)]
pub struct DatabaseAvailableOption {
    /// Option name
    pub name: Option<String>,
    /// Option type
    #[serde(rename = "type")]
    pub option_type: Option<String>,
    /// Allowed enumerals
    pub enumerals: Option<Vec<String>>,
    /// Minimum value
    pub min_value: Option<f64>,
    /// Maximum value
    pub max_value: Option<f64>,
    /// Alternate values
    pub alt_values: Option<Vec<i64>>,
    /// Units
    pub units: Option<String>,
}

/// Alias for OpenAPI schema name `dbaas-available-options`
pub type DbaasAvailableOptions = DatabaseAvailableOption;

/// DBaaS meta wrapper (total only)
#[derive(Serialize, Deserialize)]
pub struct DbaasMeta {
    /// Total objects available
    pub total: Option<i32>,
}

/// Database plan
#[derive(Serialize, Deserialize)]
pub struct DatabasePlan {
    /// Plan ID
    pub id: Option<String>,
    /// Number of nodes
    pub number_of_nodes: Option<i32>,
    /// Plan type
    #[serde(rename = "type")]
    pub plan_type: Option<String>,
    /// vCPU count
    pub vcpu_count: Option<i32>,
    /// RAM in MB
    pub ram: Option<i32>,
    /// Disk in GB
    pub disk: Option<i32>,
    /// Monthly cost
    pub monthly_cost: Option<i32>,
    /// Supported engines
    pub supported_engines: Option<serde_json::Value>,
    /// Max connections per engine
    pub max_connections: Option<serde_json::Value>,
    /// Available locations
    #[serde(default)]
    pub locations: Vec<String>,
}

/// PostgreSQL advanced options
#[derive(Serialize, Deserialize, Default)]
pub struct PgAdvancedOptions {
    #[serde(flatten)]
    pub options: HashMap<String, serde_json::Value>,
}

/// MySQL advanced options
#[derive(Serialize, Deserialize, Default)]
pub struct MysqlAdvancedOptions {
    #[serde(flatten)]
    pub options: HashMap<String, serde_json::Value>,
}

/// Kafka advanced options
#[derive(Serialize, Deserialize, Default)]
pub struct KafkaAdvancedOptions {
    #[serde(flatten)]
    pub options: HashMap<String, serde_json::Value>,
}

/// Kafka REST advanced options
#[derive(Serialize, Deserialize, Default)]
pub struct KafkaRestAdvancedOptions {
    #[serde(flatten)]
    pub options: HashMap<String, serde_json::Value>,
}

/// Schema registry advanced options
#[derive(Serialize, Deserialize, Default)]
pub struct SchemaRegistryAdvancedOptions {
    #[serde(flatten)]
    pub options: HashMap<String, serde_json::Value>,
}

/// Kafka Connect advanced options
#[derive(Serialize, Deserialize, Default)]
pub struct KafkaConnectAdvancedOptions {
    #[serde(flatten)]
    pub options: HashMap<String, serde_json::Value>,
}

/// Advanced options response wrapper
#[derive(Serialize, Deserialize)]
pub struct DatabaseAdvancedOptionsResponse {
    pub configured_options: serde_json::Value,
    #[serde(default)]
    pub available_options: Vec<DbaasAvailableOptions>,
}

/// Database alert
#[derive(Serialize, Deserialize)]
pub struct DatabaseAlert {
    /// Alert timestamp
    pub timestamp: Option<String>,
    /// Alert type
    pub message_type: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Recommendation
    pub recommendation: Option<String>,
    /// Scheduled maintenance time
    pub maintenance_scheduled: Option<String>,
    /// Affected resource type
    pub resource_type: Option<String>,
    /// Affected table count
    pub table_count: Option<i32>,
}

/// Database migration
#[derive(Serialize, Deserialize)]
pub struct DatabaseMigration {
    /// Migration status
    pub status: Option<String>,
    /// Migration method
    pub method: Option<String>,
    /// Error message
    pub error: Option<String>,
    /// Source credentials
    pub credentials: Option<MigrationCredentials>,
}

/// Migration source credentials
#[derive(Serialize, Deserialize)]
pub struct MigrationCredentials {
    /// Source host
    pub host: Option<String>,
    /// Source port
    pub port: Option<i32>,
    /// Source username
    pub username: Option<String>,
    /// Source password
    pub password: Option<String>,
    /// Source database
    pub database: Option<String>,
    /// Ignored databases
    pub ignored_databases: Option<String>,
    /// SSL required
    #[serde(default)]
    pub ssl: bool,
}

/// Database usage
#[derive(Serialize, Deserialize)]
pub struct DatabaseUsage {
    /// Disk usage
    pub disk: Option<UsageMetric>,
    /// Memory usage
    pub memory: Option<UsageMetric>,
    /// CPU usage
    pub cpu: Option<CpuUsage>,
}

/// Usage metric (disk/memory)
#[derive(Serialize, Deserialize)]
pub struct UsageMetric {
    /// Current usage (GB for disk, MB for memory)
    pub current_gb: Option<String>,
    pub current_mb: Option<String>,
    /// Maximum capacity
    pub max_gb: Option<String>,
    pub max_mb: Option<String>,
    /// Percentage used
    pub percentage: Option<String>,
}

/// CPU usage
#[derive(Serialize, Deserialize)]
pub struct CpuUsage {
    /// Percentage used
    pub percentage: Option<String>,
}

/// Database quota (Kafka)
#[derive(Serialize, Deserialize)]
pub struct DatabaseQuota {
    /// Client ID
    pub client_id: Option<String>,
    /// Username
    pub user: Option<String>,
    /// Consumer byte rate
    pub consumer_byte_rate: Option<i64>,
    /// Producer byte rate
    pub producer_byte_rate: Option<i64>,
    /// Request percentage
    pub request_percentage: Option<i32>,
}

/// Database backup
#[derive(Serialize, Deserialize)]
pub struct DatabaseBackup {
    /// Backup date
    pub date: Option<String>,
    /// Backup time
    pub time: Option<String>,
}

/// Maintenance schedule
#[derive(Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    /// Day of week
    pub day: Option<String>,
    /// Hour
    pub hour: Option<i32>,
}

// Response types

/// Response wrapper for list of databases
#[derive(Deserialize)]
pub struct DatabasesResponse {
    pub databases: Vec<Database>,
    pub meta: Option<DbaasMeta>,
}

/// Response wrapper for single database
#[derive(Deserialize)]
pub struct DatabaseResponse {
    pub database: Database,
}

/// Response for database plans
#[derive(Deserialize)]
pub struct DatabasePlansResponse {
    pub plans: Vec<DatabasePlan>,
}

/// Response for database users
#[derive(Deserialize)]
pub struct DatabaseUsersResponse {
    pub users: Vec<DatabaseUser>,
    pub meta: Option<DbaasMeta>,
}

/// Response for single database user
#[derive(Deserialize)]
pub struct DatabaseUserResponse {
    pub user: DatabaseUser,
}

/// Response for logical databases
#[derive(Deserialize)]
pub struct LogicalDatabasesResponse {
    pub dbs: Vec<LogicalDatabase>,
    pub meta: Option<DbaasMeta>,
}

/// Response for single logical database
#[derive(Deserialize)]
pub struct LogicalDatabaseResponse {
    pub db: LogicalDatabase,
}

/// Response for connection pools
#[derive(Serialize, Deserialize)]
pub struct ConnectionPoolsResponse {
    pub connections: Option<DatabaseConnections>,
    pub connection_pools: Vec<ConnectionPool>,
    pub meta: Option<DbaasMeta>,
}

/// Response for single connection pool
#[derive(Deserialize)]
pub struct ConnectionPoolResponse {
    pub connection_pool: ConnectionPool,
}

/// Response for Kafka topics
#[derive(Deserialize)]
pub struct KafkaTopicsResponse {
    pub topics: Vec<KafkaTopic>,
    pub meta: Option<DbaasMeta>,
}

/// Response for single Kafka topic
#[derive(Deserialize)]
pub struct KafkaTopicResponse {
    pub topic: KafkaTopic,
}

/// Response for Kafka connectors
#[derive(Deserialize)]
pub struct KafkaConnectorsResponse {
    pub connectors: Vec<KafkaConnector>,
    pub meta: Option<DbaasMeta>,
}

/// Response for single Kafka connector
#[derive(Deserialize)]
pub struct KafkaConnectorResponse {
    pub connector: KafkaConnector,
}

/// Response for connector status
#[derive(Deserialize)]
pub struct ConnectorStatusResponse {
    pub status: ConnectorStatus,
}

/// Response for available connectors
#[derive(Deserialize)]
pub struct AvailableConnectorsResponse {
    pub available_connectors: Vec<AvailableConnector>,
    pub meta: Option<DbaasMeta>,
}

/// Response for database alerts
#[derive(Deserialize)]
pub struct DatabaseAlertsResponse {
    pub alerts: Vec<DatabaseAlert>,
    pub meta: Option<DbaasMeta>,
}

/// Response for database migration
#[derive(Deserialize)]
pub struct DatabaseMigrationResponse {
    pub migration: DatabaseMigration,
}

/// Response for database usage
#[derive(Deserialize)]
pub struct DatabaseUsageResponse {
    pub usage: DatabaseUsage,
}

/// Response for database quotas
#[derive(Deserialize)]
pub struct DatabaseQuotasResponse {
    pub quotas: Vec<DatabaseQuota>,
    pub meta: Option<DbaasMeta>,
}

/// Response for database backups
#[derive(Serialize, Deserialize)]
pub struct DatabaseBackupsResponse {
    pub latest_backup: Option<DatabaseBackup>,
    pub oldest_backup: Option<DatabaseBackup>,
}

/// Response for maintenance
#[derive(Deserialize)]
pub struct MaintenanceResponse {
    pub maintenance: MaintenanceSchedule,
}

/// Response for available versions
#[derive(Deserialize)]
pub struct DatabaseVersionsResponse {
    pub available_versions: Vec<String>,
}

/// Response for connector configuration schema
#[derive(Deserialize)]
pub struct DatabaseConnectorConfigurationSchemaResponse {
    pub configuration_schema: Vec<DatabaseConnectorConfigurationSchema>,
}

// Request types

/// Request to create a database
#[derive(Serialize, Default)]
pub struct CreateDatabaseRequest {
    /// Database engine (mysql, pg, valkey, kafka)
    pub database_engine: String,
    /// Engine version
    pub database_engine_version: String,
    /// Region ID
    pub region: String,
    /// Plan name
    pub plan: String,
    /// Label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// VPC ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// Maintenance day of week
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_dow: Option<String>,
    /// Maintenance time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_time: Option<String>,
    /// Backup hour
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_hour: Option<i32>,
    /// Backup minute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_minute: Option<i32>,
    /// Trusted IPs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_ips: Option<Vec<String>>,
    /// MySQL SQL modes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql_sql_modes: Option<Vec<String>>,
    /// MySQL require primary key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql_require_primary_key: Option<bool>,
    /// MySQL slow query log
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql_slow_query_log: Option<bool>,
    /// MySQL long query time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql_long_query_time: Option<i32>,
    /// Valkey eviction policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<String>,
}

/// Request to update a database
#[derive(Serialize, Default)]
pub struct UpdateDatabaseRequest {
    /// Plan name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// VPC ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// Maintenance day of week
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_dow: Option<String>,
    /// Maintenance time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_time: Option<String>,
    /// Backup hour
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_hour: Option<i32>,
    /// Backup minute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_minute: Option<i32>,
    /// Trusted IPs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_ips: Option<Vec<String>>,
    /// MySQL SQL modes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql_sql_modes: Option<Vec<String>>,
    /// MySQL require primary key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql_require_primary_key: Option<bool>,
    /// MySQL slow query log
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql_slow_query_log: Option<bool>,
    /// MySQL long query time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql_long_query_time: Option<i32>,
    /// Valkey eviction policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<String>,
    /// Cluster time zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_time_zone: Option<String>,
}

/// Request to create a database user
#[derive(Serialize)]
pub struct CreateDatabaseUserRequest {
    /// Username
    pub username: String,
    /// Password (optional, auto-generated if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Encryption type (MySQL only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<String>,
    /// Permission (Kafka only: admin, read, readwrite, write)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

/// Request to update a database user
#[derive(Serialize)]
pub struct UpdateDatabaseUserRequest {
    /// New password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Request for user access control (Valkey)
#[derive(Serialize)]
pub struct UpdateUserAccessControlRequest {
    /// ACL categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_categories: Option<Vec<String>>,
    /// ACL channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_channels: Option<Vec<String>>,
    /// ACL commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_commands: Option<Vec<String>>,
    /// ACL keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_keys: Option<Vec<String>>,
}

/// Request to create a logical database
#[derive(Serialize)]
pub struct CreateLogicalDatabaseRequest {
    /// Database name
    pub name: String,
}

/// Request to create a connection pool
#[derive(Serialize)]
pub struct CreateConnectionPoolRequest {
    /// Pool name
    pub name: String,
    /// Database name
    pub database: String,
    /// Username
    pub username: String,
    /// Pool mode
    pub mode: String,
    /// Pool size
    pub size: i32,
}

/// Request to update a connection pool
#[derive(Serialize)]
pub struct UpdateConnectionPoolRequest {
    /// Database name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Pool mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Pool size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

/// Request to create a Kafka topic
#[derive(Serialize)]
pub struct CreateKafkaTopicRequest {
    /// Topic name
    pub name: String,
    /// Number of partitions
    pub partitions: i32,
    /// Replication factor
    pub replication: i32,
    /// Retention hours
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_hours: Option<i32>,
    /// Retention bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_bytes: Option<i64>,
}

/// Request to update a Kafka topic
#[derive(Serialize)]
pub struct UpdateKafkaTopicRequest {
    /// Number of partitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<i32>,
    /// Retention hours
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_hours: Option<i32>,
    /// Retention bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_bytes: Option<i64>,
}

/// Request to create a Kafka connector
#[derive(Serialize)]
pub struct CreateKafkaConnectorRequest {
    /// Connector name
    pub name: String,
    /// Connector class
    pub class: String,
    /// Topics (comma-separated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<String>,
    /// Connector configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}

/// Request to start migration
#[derive(Serialize)]
pub struct StartMigrationRequest {
    /// Source host
    pub host: String,
    /// Source port
    pub port: i32,
    /// Source username
    pub username: String,
    /// Source password
    pub password: String,
    /// Source database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// Ignored databases
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_databases: Option<String>,
    /// SSL required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<bool>,
}

/// Request to restore database from backup
#[derive(Serialize)]
pub struct RestoreDatabaseRequest {
    /// Label for restored database
    pub label: String,
    /// Backup date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Backup time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Type of restoration
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<String>,
}

/// Request to fork database
#[derive(Serialize)]
pub struct ForkDatabaseRequest {
    /// Label for forked database
    pub label: String,
    /// Region (optional, defaults to source region)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Plan (optional, defaults to source plan)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// VPC ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// Backup date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Backup time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Type of fork
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub fork_type: Option<String>,
}

/// Request to create read replica
#[derive(Serialize)]
pub struct CreateReadReplicaRequest {
    /// Label for replica
    pub label: String,
    /// Region (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Request to update maintenance schedule
#[derive(Serialize)]
pub struct UpdateMaintenanceRequest {
    /// Day of week
    pub day: String,
    /// Hour
    pub hour: i32,
}

/// Request to create quota
#[derive(Serialize)]
pub struct CreateDatabaseQuotaRequest {
    /// Client ID
    pub client_id: String,
    /// Consumer byte rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_byte_rate: Option<i64>,
    /// Producer byte rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_byte_rate: Option<i64>,
    /// Request percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_percentage: Option<i32>,
}

/// Request to upgrade database version
#[derive(Serialize)]
pub struct UpgradeDatabaseVersionRequest {
    /// Target version
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_deserialize() {
        let json = r#"{
            "id": "db123",
            "label": "my-database",
            "region": "ewr",
            "database_engine": "mysql",
            "database_engine_version": "8",
            "status": "Running",
            "plan": "vultr-dbaas-hobbyist-cc-1-25-1",
            "host": "db123.vultrdb.com",
            "port": "3306",
            "user": "vultradmin",
            "password": "secret123",
            "trusted_ips": ["0.0.0.0/0"]
        }"#;
        let db: Database = serde_json::from_str(json).unwrap();
        assert_eq!(db.id, "db123");
        assert_eq!(db.database_engine, Some("mysql".into()));
        assert_eq!(db.trusted_ips.len(), 1);
    }

    #[test]
    fn test_database_user_deserialize() {
        let json = r#"{
            "username": "testuser",
            "password": "pass123",
            "encryption": "Default (MySQL 8+)"
        }"#;
        let user: DatabaseUser = serde_json::from_str(json).unwrap();
        assert_eq!(user.username, Some("testuser".into()));
        assert_eq!(user.encryption, Some("Default (MySQL 8+)".into()));
    }

    #[test]
    fn test_connection_pool_deserialize() {
        let json = r#"{
            "name": "mypool",
            "database": "defaultdb",
            "username": "vultradmin",
            "mode": "transaction",
            "size": 10
        }"#;
        let pool: ConnectionPool = serde_json::from_str(json).unwrap();
        assert_eq!(pool.name, Some("mypool".into()));
        assert_eq!(pool.mode, Some("transaction".into()));
        assert_eq!(pool.size, Some(10));
    }

    #[test]
    fn test_kafka_topic_deserialize() {
        let json = r#"{
            "name": "my-topic",
            "partitions": 3,
            "replication": 2,
            "retention_hours": 168
        }"#;
        let topic: KafkaTopic = serde_json::from_str(json).unwrap();
        assert_eq!(topic.name, Some("my-topic".into()));
        assert_eq!(topic.partitions, Some(3));
    }

    #[test]
    fn test_kafka_connector_deserialize() {
        let json = r#"{
            "name": "my-connector",
            "class": "io.debezium.connector.mysql.MySqlConnector",
            "topics": "topic1,topic2"
        }"#;
        let connector: KafkaConnector = serde_json::from_str(json).unwrap();
        assert_eq!(connector.name, Some("my-connector".into()));
        assert_eq!(connector.topics, Some("topic1,topic2".into()));
    }

    #[test]
    fn test_database_plan_deserialize() {
        let json = r#"{
            "id": "vultr-dbaas-hobbyist-cc-1-25-1",
            "number_of_nodes": 1,
            "type": "hobbyist",
            "vcpu_count": 1,
            "ram": 1024,
            "disk": 25,
            "monthly_cost": 15,
            "locations": ["ewr", "ord", "dfw"]
        }"#;
        let plan: DatabasePlan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.id, Some("vultr-dbaas-hobbyist-cc-1-25-1".into()));
        assert_eq!(plan.vcpu_count, Some(1));
        assert_eq!(plan.locations.len(), 3);
    }

    #[test]
    fn test_database_usage_deserialize() {
        let json = r#"{
            "disk": {
                "current_gb": "5.2",
                "max_gb": "25",
                "percentage": "20.8"
            },
            "memory": {
                "current_mb": "512",
                "max_mb": "1024",
                "percentage": "50"
            },
            "cpu": {
                "percentage": "15"
            }
        }"#;
        let usage: DatabaseUsage = serde_json::from_str(json).unwrap();
        assert_eq!(usage.cpu.unwrap().percentage, Some("15".into()));
    }

    #[test]
    fn test_create_database_request_serialize() {
        let request = CreateDatabaseRequest {
            database_engine: "mysql".into(),
            database_engine_version: "8".into(),
            region: "ewr".into(),
            plan: "vultr-dbaas-hobbyist-cc-1-25-1".into(),
            label: Some("my-db".into()),
            ..Default::default()
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"database_engine\":\"mysql\""));
        assert!(json.contains("\"label\":\"my-db\""));
    }

    #[test]
    fn test_create_kafka_topic_request_serialize() {
        let request = CreateKafkaTopicRequest {
            name: "test-topic".into(),
            partitions: 3,
            replication: 2,
            retention_hours: Some(168),
            retention_bytes: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"partitions\":3"));
        assert!(json.contains("\"retention_hours\":168"));
    }

    #[test]
    fn test_migration_deserialize() {
        let json = r#"{
            "status": "running",
            "credentials": {
                "host": "source.db.com",
                "port": 3306,
                "username": "admin",
                "ssl": true
            }
        }"#;
        let migration: DatabaseMigration = serde_json::from_str(json).unwrap();
        assert_eq!(migration.status, Some("running".into()));
        assert!(migration.credentials.unwrap().ssl);
    }

    #[test]
    fn test_database_alert_deserialize() {
        let json = r#"{
            "timestamp": "2024-01-15T10:30:00Z",
            "message_type": "RESOURCE USAGE DISK",
            "description": "Disk usage is above 80%",
            "recommendation": "Consider upgrading your plan"
        }"#;
        let alert: DatabaseAlert = serde_json::from_str(json).unwrap();
        assert_eq!(alert.message_type, Some("RESOURCE USAGE DISK".into()));
    }
}
