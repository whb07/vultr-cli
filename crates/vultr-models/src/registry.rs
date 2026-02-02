//! Container Registry model types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Container Registry
#[derive(Serialize, Deserialize)]
pub struct Registry {
    /// Unique identifier for the registry
    pub id: String,
    /// Globally unique name for the registry
    pub name: Option<String>,
    /// Base URN of the registry
    pub urn: Option<String>,
    /// Storage information
    pub storage: Option<RegistryStorageInfo>,
    /// Date created
    pub date_created: Option<String>,
    /// Whether the registry is public
    #[serde(default)]
    pub public: bool,
    /// Root user information
    pub root_user: Option<RegistryUser>,
    /// Registry metadata
    pub metadata: Option<RegistryMetadata>,
}

/// Storage information for registry
#[derive(Serialize, Deserialize)]
pub struct RegistryStorageInfo {
    /// Used storage
    pub used: Option<RegistryStorage>,
    /// Allowed storage
    pub allowed: Option<RegistryStorage>,
}

/// Storage details
#[derive(Serialize, Deserialize)]
pub struct RegistryStorage {
    /// Storage in bytes
    pub bytes: Option<f64>,
    /// Storage in megabytes
    pub mb: Option<f64>,
    /// Storage in gigabytes
    pub gb: Option<f64>,
    /// Storage in terabytes
    pub tb: Option<f64>,
    /// Last updated timestamp
    pub updated_at: Option<String>,
}

/// Registry metadata
#[derive(Serialize, Deserialize)]
pub struct RegistryMetadata {
    /// Region information
    pub region: Option<RegistryRegion>,
    /// Subscription information
    pub subscription: Option<RegistrySubscription>,
}

/// Subscription information
#[derive(Serialize, Deserialize)]
pub struct RegistrySubscription {
    /// Billing information
    pub billing: Option<RegistryBilling>,
}

/// Billing information
#[derive(Serialize, Deserialize)]
pub struct RegistryBilling {
    /// Monthly price
    pub monthly_price: Option<f64>,
    /// Pending charges
    pub pending_charges: Option<f64>,
}

/// Registry user
#[derive(Serialize, Deserialize)]
pub struct RegistryUser {
    /// Numeric ID
    pub id: Option<i64>,
    /// Username
    pub username: Option<String>,
    /// Password
    pub password: Option<String>,
    /// Whether this is a root user
    #[serde(default)]
    pub root: bool,
    /// Date added
    pub added_at: Option<String>,
    /// Date updated
    pub updated_at: Option<String>,
}

/// Registry region
#[derive(Serialize, Deserialize)]
pub struct RegistryRegion {
    /// Numeric ID
    pub id: Option<i64>,
    /// Region name
    pub name: Option<String>,
    /// Base URN
    pub urn: Option<String>,
    /// Base URL
    pub base_url: Option<String>,
    /// Whether publicly available
    #[serde(default)]
    pub public: bool,
    /// Date added
    pub added_at: Option<String>,
    /// Date updated
    pub updated_at: Option<String>,
    /// Data center info
    pub data_center: Option<serde_json::Value>,
}

/// Registry plan
#[derive(Serialize, Deserialize)]
pub struct RegistryPlan {
    /// Plan ID (the key in the plans map)
    #[serde(skip)]
    pub id: String,
    /// Display name
    pub vanity_name: Option<String>,
    /// Max storage in MB
    pub max_storage_mb: Option<i64>,
    /// Monthly price
    pub monthly_price: Option<i64>,
}

/// Repository in a registry
#[derive(Serialize, Deserialize)]
pub struct RegistryRepository {
    /// Full name (registry/image)
    pub name: Option<String>,
    /// Image name
    pub image: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Date added
    pub added_at: Option<String>,
    /// Date updated
    pub updated_at: Option<String>,
    /// Pull count
    pub pull_count: Option<i64>,
    /// Artifact count
    pub artifact_count: Option<i64>,
}

/// Artifact in a repository
#[derive(Serialize, Deserialize)]
pub struct RegistryArtifact {
    /// Artifact type
    pub artifact_type: Option<String>,
    /// SHA256 digest
    pub digest: Option<String>,
    /// Manifest media type
    pub manifest_media_type: Option<String>,
    /// Media type
    pub media_type: Option<String>,
    /// Pull time
    pub pull_time: Option<String>,
    /// Push time
    pub push_time: Option<String>,
    /// Repository name
    pub repository_name: Option<String>,
    /// Size in bytes
    pub size: Option<i64>,
    /// Type (IMAGE, HELM, CHART, etc.)
    #[serde(rename = "type")]
    pub artifact_kind: Option<String>,
    /// Tags
    #[serde(default)]
    pub tags: Vec<serde_json::Value>,
}

/// Robot account
#[derive(Serialize, Deserialize)]
pub struct RegistryRobot {
    /// Robot name
    pub name: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Secret
    pub secret: Option<String>,
    /// Whether disabled
    #[serde(default)]
    pub disable: bool,
    /// Duration in seconds (-1 for never)
    pub duration: Option<i64>,
    /// Creation time
    pub creation_time: Option<String>,
    /// Permissions
    pub permissions: Option<Vec<RobotPermission>>,
}

/// Robot permission
#[derive(Serialize, Deserialize)]
pub struct RobotPermission {
    /// Kind of permission
    pub kind: Option<String>,
    /// Namespace
    pub namespace: Option<String>,
    /// Access rules
    pub access: Option<Vec<RobotAccess>>,
}

/// Robot access rule
#[derive(Serialize, Deserialize)]
pub struct RobotAccess {
    /// Action (pull, push, read, delete)
    pub action: Option<String>,
    /// Resource (repository, artifact)
    pub resource: Option<String>,
    /// Effect (deny removes access)
    pub effect: Option<String>,
}

/// Replication policy
#[derive(Serialize, Deserialize)]
pub struct RegistryReplication {
    /// Region name
    pub region: Option<String>,
    /// Namespace (registry name)
    pub namespace: Option<String>,
    /// Base URN
    pub urn: Option<String>,
}

/// Retention rule
#[derive(Serialize, Deserialize)]
pub struct RegistryRetentionRule {
    /// Rule ID
    pub id: Option<i64>,
    /// Whether disabled
    #[serde(default)]
    pub disabled: bool,
    /// Action
    pub action: Option<String>,
    /// Parameters
    pub params: Option<HashMap<String, i64>>,
    /// Scope selectors
    pub scope_selectors: Option<RetentionScopeSelectors>,
    /// Tag selectors
    #[serde(default)]
    pub tag_selectors: Vec<RetentionTagSelector>,
    /// Template
    pub template: Option<String>,
}

/// Retention scope selectors
#[derive(Serialize, Deserialize)]
pub struct RetentionScopeSelectors {
    /// Repository selectors
    #[serde(default)]
    pub repository: Vec<RetentionRepositorySelector>,
}

/// Retention repository selector
#[derive(Serialize, Deserialize)]
pub struct RetentionRepositorySelector {
    /// Decoration (repoMatches, repoExcludes)
    pub decoration: Option<String>,
    /// Kind
    pub kind: Option<String>,
    /// Pattern
    pub pattern: Option<String>,
}

/// Retention tag selector
#[derive(Serialize, Deserialize)]
pub struct RetentionTagSelector {
    /// Decoration (matches, excludes)
    pub decoration: Option<String>,
    /// Extras
    pub extras: Option<serde_json::Value>,
    /// Kind
    pub kind: Option<String>,
    /// Pattern
    pub pattern: Option<String>,
}

/// Retention schedule
#[derive(Serialize, Deserialize)]
pub struct RegistryRetentionSchedule {
    /// Schedule type
    #[serde(rename = "type")]
    pub schedule_type: Option<String>,
    /// Cron expression
    pub cron: Option<String>,
}

/// Retention execution
#[derive(Serialize, Deserialize)]
pub struct RegistryRetentionExecution {
    /// Execution ID
    pub id: Option<i64>,
    /// Status
    pub status: Option<String>,
    /// Trigger type
    pub trigger: Option<String>,
    /// Start time
    pub start_time: Option<String>,
    /// End time
    pub end_time: Option<String>,
}

/// Docker credentials
#[derive(Serialize, Deserialize)]
pub struct RegistryDockerCredentials {
    /// Auth entries
    pub auths: Option<HashMap<String, DockerAuth>>,
}

/// Docker auth entry
#[derive(Serialize, Deserialize)]
pub struct DockerAuth {
    /// Base64 encoded credentials
    pub auth: Option<String>,
}

/// Kubernetes docker credentials
#[derive(Serialize, Deserialize)]
pub struct RegistryKubernetesCredentials {
    /// API version
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind
    pub kind: Option<String>,
    /// Metadata
    pub metadata: Option<K8sCredentialsMetadata>,
    /// Data
    pub data: Option<K8sCredentialsData>,
    /// Type
    #[serde(rename = "type")]
    pub cred_type: Option<String>,
}

/// K8s credentials metadata
#[derive(Serialize, Deserialize)]
pub struct K8sCredentialsMetadata {
    /// Name
    pub name: Option<String>,
}

/// K8s credentials data
#[derive(Serialize, Deserialize)]
pub struct K8sCredentialsData {
    /// Docker config JSON (base64 encoded)
    #[serde(rename = ".dockerconfigjson")]
    pub dockerconfigjson: Option<String>,
}

// =====================
// Response Types
// =====================

/// Response for list of registries
#[derive(Deserialize)]
pub struct RegistriesResponse {
    pub registries: Vec<Registry>,
}

/// Response for single registry
#[derive(Deserialize)]
pub struct RegistryResponse {
    pub registry: Registry,
}

/// Response for repositories
#[derive(Deserialize)]
pub struct RepositoriesResponse {
    pub repositories: Vec<RegistryRepository>,
}

/// Response for single repository
#[derive(Deserialize)]
pub struct RepositoryResponse {
    pub repository: RegistryRepository,
}

/// Response for artifacts
#[derive(Deserialize)]
pub struct ArtifactsResponse {
    pub artifacts: Vec<RegistryArtifact>,
}

/// Response for robots
#[derive(Deserialize)]
pub struct RobotsResponse {
    pub robots: Vec<RegistryRobot>,
}

/// Response for single robot
#[derive(Deserialize)]
pub struct RobotResponse {
    pub robot: RegistryRobot,
}

/// Response for replications
#[derive(Deserialize)]
pub struct ReplicationsResponse {
    pub replications: Vec<RegistryReplication>,
}

/// Response for retention rules
#[derive(Deserialize)]
pub struct RetentionRulesResponse {
    #[serde(default)]
    pub rules: Vec<RegistryRetentionRule>,
}

/// Response for retention executions
#[derive(Deserialize)]
pub struct RetentionExecutionsResponse {
    #[serde(default)]
    pub executions: Vec<RegistryRetentionExecution>,
}

/// Response for retention schedule
#[derive(Deserialize)]
pub struct RetentionScheduleResponse {
    pub schedule: Option<RegistryRetentionSchedule>,
}

/// Response for registry regions
#[derive(Deserialize)]
pub struct RegistryRegionsResponse {
    pub regions: Vec<RegistryRegion>,
}

/// Response for registry plans
#[derive(Deserialize)]
pub struct RegistryPlansResponse {
    pub plans: HashMap<String, RegistryPlan>,
}

/// Response for docker credentials
#[derive(Deserialize)]
pub struct DockerCredentialsResponse {
    #[serde(flatten)]
    pub credentials: RegistryDockerCredentials,
}

/// Response for kubernetes credentials
#[derive(Deserialize)]
pub struct KubernetesCredentialsResponse {
    #[serde(flatten)]
    pub credentials: RegistryKubernetesCredentials,
}

// =====================
// Request Types
// =====================

/// Request to create a registry
#[derive(Serialize)]
pub struct CreateRegistryRequest {
    /// Registry name (required)
    pub name: String,
    /// Whether public
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// Region name
    pub region: String,
    /// Plan ID
    pub plan: String,
}

/// Request to update a registry
#[derive(Serialize)]
pub struct UpdateRegistryRequest {
    /// Whether public
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// Plan ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
}

/// Request to create a robot
#[derive(Serialize)]
pub struct CreateRobotRequest {
    /// Robot name (required)
    pub name: String,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Duration in seconds (-1 for never expires)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<CreateRobotPermission>>,
}

/// Permission for robot creation
#[derive(Serialize)]
pub struct CreateRobotPermission {
    /// Permission kind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Namespace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Access rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<CreateRobotAccess>>,
}

/// Access rule for robot creation
#[derive(Serialize)]
pub struct CreateRobotAccess {
    /// Action
    pub action: String,
    /// Resource
    pub resource: String,
}

/// Request to update a robot
#[derive(Serialize)]
pub struct UpdateRobotRequest {
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Whether disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    /// Permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<CreateRobotPermission>>,
}

/// Request to create a replication
#[derive(Serialize)]
pub struct CreateReplicationRequest {
    /// Region to replicate to
    pub region: String,
}

/// Request to create a retention rule
#[derive(Serialize)]
pub struct CreateRetentionRuleRequest {
    /// Action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Template
    pub template: String,
    /// Parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, i64>>,
    /// Scope selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_selectors: Option<CreateRetentionScopeSelectors>,
    /// Tag selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_selectors: Option<Vec<CreateRetentionTagSelector>>,
}

/// Scope selectors for retention rule creation
#[derive(Serialize)]
pub struct CreateRetentionScopeSelectors {
    /// Repository selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Vec<CreateRetentionRepositorySelector>>,
}

/// Repository selector for retention rule creation
#[derive(Serialize)]
pub struct CreateRetentionRepositorySelector {
    /// Decoration
    pub decoration: String,
    /// Kind
    pub kind: String,
    /// Pattern
    pub pattern: String,
}

/// Tag selector for retention rule creation
#[derive(Serialize)]
pub struct CreateRetentionTagSelector {
    /// Decoration
    pub decoration: String,
    /// Kind
    pub kind: String,
    /// Pattern
    pub pattern: String,
}

/// Request to update a retention rule
#[derive(Serialize)]
pub struct UpdateRetentionRuleRequest {
    /// Action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// Parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, i64>>,
    /// Whether disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Scope selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_selectors: Option<CreateRetentionScopeSelectors>,
    /// Tag selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_selectors: Option<Vec<CreateRetentionTagSelector>>,
}

/// Request to update retention schedule
#[derive(Serialize)]
pub struct UpdateRetentionScheduleRequest {
    /// Schedule type (Hourly, Daily, Weekly, Custom)
    #[serde(rename = "type")]
    pub schedule_type: String,
    /// Cron expression (required for Custom)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
}

/// Request to update user password
#[derive(Serialize)]
pub struct UpdateUserPasswordRequest {
    /// New password
    pub password: String,
}

/// Response for updated retention rule
#[derive(Deserialize)]
pub struct RetentionRuleResponse {
    pub rule: RegistryRetentionRule,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_deserialize() {
        let json = r#"{
            "id": "abc123",
            "name": "my-registry",
            "urn": "sjc.vultrcr.com/my-registry",
            "public": false,
            "date_created": "2024-01-01T00:00:00Z"
        }"#;
        let registry: Registry = serde_json::from_str(json).unwrap();
        assert_eq!(registry.id, "abc123");
        assert_eq!(registry.name, Some("my-registry".to_string()));
        assert!(!registry.public);
    }

    #[test]
    fn test_repository_deserialize() {
        let json = r#"{
            "name": "my-registry/my-image",
            "image": "my-image",
            "pull_count": 100,
            "artifact_count": 5
        }"#;
        let repo: RegistryRepository = serde_json::from_str(json).unwrap();
        assert_eq!(repo.name, Some("my-registry/my-image".to_string()));
        assert_eq!(repo.pull_count, Some(100));
    }

    #[test]
    fn test_artifact_deserialize() {
        let json = r#"{
            "digest": "sha256:abc123",
            "type": "IMAGE",
            "size": 1024000,
            "tags": ["latest", "v1.0"]
        }"#;
        let artifact: RegistryArtifact = serde_json::from_str(json).unwrap();
        assert_eq!(artifact.digest, Some("sha256:abc123".to_string()));
        assert_eq!(artifact.artifact_kind, Some("IMAGE".to_string()));
    }

    #[test]
    fn test_robot_deserialize() {
        let json = r#"{
            "name": "robot$myrobot",
            "description": "CI/CD robot",
            "disable": false,
            "duration": -1
        }"#;
        let robot: RegistryRobot = serde_json::from_str(json).unwrap();
        assert_eq!(robot.name, Some("robot$myrobot".to_string()));
        assert!(!robot.disable);
    }

    #[test]
    fn test_create_registry_request_serialize() {
        let request = CreateRegistryRequest {
            name: "my-registry".to_string(),
            public: Some(false),
            region: "sjc".to_string(),
            plan: "start_up".to_string(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"name\":\"my-registry\""));
        assert!(json.contains("\"region\":\"sjc\""));
    }

    #[test]
    fn test_retention_rule_deserialize() {
        let json = r#"{
            "id": 1,
            "disabled": false,
            "action": "retain",
            "template": "latestPushedK",
            "tag_selectors": []
        }"#;
        let rule: RegistryRetentionRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.id, Some(1));
        assert!(!rule.disabled);
    }

    #[test]
    fn test_replication_deserialize() {
        let json = r#"{
            "region": "lax",
            "namespace": "my-registry",
            "urn": "lax.vultrcr.com/my-registry"
        }"#;
        let repl: RegistryReplication = serde_json::from_str(json).unwrap();
        assert_eq!(repl.region, Some("lax".to_string()));
    }
}
