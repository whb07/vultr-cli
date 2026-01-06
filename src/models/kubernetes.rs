//! Kubernetes (VKE) model types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Kubernetes cluster (VKE)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesCluster {
    /// Unique identifier for the cluster
    pub id: String,
    /// The Firewall Group ID linked to this cluster
    #[serde(default)]
    pub firewall_group_id: Option<String>,
    /// Label for the cluster
    pub label: Option<String>,
    /// Date of creation
    pub date_created: Option<String>,
    /// IP range for pods
    pub cluster_subnet: Option<String>,
    /// IP range for services
    pub service_subnet: Option<String>,
    /// IP for the control plane
    pub ip: Option<String>,
    /// Domain for the control plane
    pub endpoint: Option<String>,
    /// Kubernetes version
    pub version: Option<String>,
    /// Region ID
    pub region: Option<String>,
    /// Cluster status
    pub status: Option<String>,
    /// Whether HA control planes are enabled
    #[serde(default)]
    pub ha_controlplanes: bool,
    /// OIDC configuration
    pub oidc: Option<OidcConfig>,
    /// Node pools in the cluster
    #[serde(default)]
    pub node_pools: Vec<NodePool>,
    /// Attached VPCs
    #[serde(default)]
    pub vpcs: Vec<AttachedVpc>,
}

/// OIDC configuration for Kubernetes cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcConfig {
    /// OIDC provider URL
    pub issuer_url: Option<String>,
    /// Client ID for the OIDC application
    pub client_id: Option<String>,
    /// Claim for username
    pub username_claim: Option<String>,
    /// Claim for groups
    pub groups_claim: Option<String>,
}

/// VPC attached to a cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachedVpc {
    /// VPC ID
    pub id: Option<String>,
    /// VPC version (1 or 2)
    #[deprecated]
    pub version: Option<i32>,
    /// Subnet
    pub subnet: Option<String>,
}

/// Node pool in a Kubernetes cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePool {
    /// Node pool ID
    pub id: String,
    /// Date of creation
    pub date_created: Option<String>,
    /// Date of last update
    pub date_updated: Option<String>,
    /// Label for the node pool
    pub label: Option<String>,
    /// Tag for the node pool
    pub tag: Option<String>,
    /// Plan ID for the nodes
    pub plan: Option<String>,
    /// Status of the node pool
    pub status: Option<String>,
    /// Number of nodes
    pub node_quantity: Option<i32>,
    /// Minimum nodes (for auto-scaler)
    pub min_nodes: Option<i32>,
    /// Maximum nodes (for auto-scaler)
    pub max_nodes: Option<i32>,
    /// Whether auto-scaler is enabled
    #[serde(default)]
    pub auto_scaler: bool,
    /// Labels for nodes
    pub labels: Option<HashMap<String, String>>,
    /// Taints for nodes
    #[serde(default)]
    pub taints: Vec<NodePoolTaint>,
    /// Nodes in the pool
    #[serde(default)]
    pub nodes: Vec<KubeNode>,
    /// User data (base64 encoded)
    pub user_data: Option<String>,
}

/// Taint for a node pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePoolTaint {
    /// Taint key
    pub key: Option<String>,
    /// Taint value
    pub value: Option<String>,
    /// Taint effect (NoSchedule, PreferNoSchedule, NoExecute)
    pub effect: Option<String>,
}

/// Node in a Kubernetes cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubeNode {
    /// Node ID
    pub id: String,
    /// Node label
    pub label: Option<String>,
    /// Date of creation
    pub date_created: Option<String>,
    /// Node status
    pub status: Option<String>,
}

/// Label for a node pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePoolLabel {
    /// Label ID
    pub id: Option<String>,
    /// Label key
    pub key: Option<String>,
    /// Label value
    pub value: Option<String>,
}

// Response types

/// Response wrapper for list of clusters
#[derive(Debug, Clone, Deserialize)]
pub struct ClustersResponse {
    pub vke_clusters: Vec<KubernetesCluster>,
}

/// Response wrapper for single cluster
#[derive(Debug, Clone, Deserialize)]
pub struct ClusterResponse {
    pub vke_cluster: KubernetesCluster,
}

/// Response wrapper for node pool
#[derive(Debug, Clone, Deserialize)]
pub struct NodePoolResponse {
    pub node_pool: NodePool,
}

/// Response wrapper for list of node pools
#[derive(Debug, Clone, Deserialize)]
pub struct NodePoolsResponse {
    pub node_pools: Vec<NodePool>,
}

/// Response wrapper for nodes
#[derive(Debug, Clone, Deserialize)]
pub struct NodesResponse {
    pub nodes: Vec<KubeNode>,
}

/// Response wrapper for single node
#[derive(Debug, Clone, Deserialize)]
pub struct NodeResponse {
    pub node: KubeNode,
}

/// Response for kubeconfig
#[derive(Debug, Clone, Deserialize)]
pub struct KubeconfigResponse {
    /// Base64-encoded kubeconfig
    pub kube_config: String,
}

/// Response for available Kubernetes versions
#[derive(Debug, Clone, Deserialize)]
pub struct VersionsResponse {
    pub versions: Vec<String>,
}

/// Response for available upgrades
#[derive(Debug, Clone, Deserialize)]
pub struct KubernetesUpgradesResponse {
    pub available_upgrades: Vec<String>,
}

/// Resources deployed by a cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterResources {
    pub resources: ClusterResourcesInner,
}

/// Inner resources structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterResourcesInner {
    #[serde(default)]
    pub block_storage: Vec<ClusterResource>,
    #[serde(default)]
    pub load_balancer: Vec<ClusterResource>,
}

/// A resource deployed by the cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterResource {
    pub id: Option<String>,
    pub label: Option<String>,
    pub date_created: Option<String>,
    pub status: Option<String>,
}

/// Response for node pool labels
#[derive(Debug, Clone, Deserialize)]
pub struct NodePoolLabelsResponse {
    pub labels: Vec<NodePoolLabel>,
}

/// Response for node pool taints
#[derive(Debug, Clone, Deserialize)]
pub struct NodePoolTaintsResponse {
    pub taints: Vec<NodePoolTaint>,
}

// Request types

/// Request to create a Kubernetes cluster
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateClusterRequest {
    /// Region ID (required)
    pub region: String,
    /// Kubernetes version (required)
    pub version: String,
    /// Cluster label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Whether to enable HA control planes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ha_controlplanes: Option<bool>,
    /// Whether to enable the managed firewall
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_firewall: Option<bool>,
    /// OIDC configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc: Option<CreateOidcRequest>,
    /// Initial node pools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_pools: Option<Vec<CreateNodePoolRequest>>,
}

/// OIDC configuration for cluster creation
#[derive(Debug, Clone, Serialize)]
pub struct CreateOidcRequest {
    /// OIDC provider URL (required)
    pub issuer_url: String,
    /// Client ID (required)
    pub client_id: String,
    /// Username claim
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
    /// Groups claim
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_claim: Option<String>,
}

/// Request to create a node pool
#[derive(Debug, Clone, Serialize)]
pub struct CreateNodePoolRequest {
    /// Number of nodes (required)
    pub node_quantity: i32,
    /// Label for the node pool (required)
    pub label: String,
    /// Plan ID (required)
    pub plan: String,
    /// Tag for nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Enable auto-scaler
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaler: Option<bool>,
    /// Minimum nodes for auto-scaler
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_nodes: Option<i32>,
    /// Maximum nodes for auto-scaler
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nodes: Option<i32>,
    /// Labels to apply to nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Taints to apply to nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<NodePoolTaintRequest>>,
    /// User data (base64 encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

/// Taint request for node pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePoolTaintRequest {
    /// Taint key
    pub key: String,
    /// Taint value
    pub value: String,
    /// Taint effect
    pub effect: String,
}

/// Request to update a Kubernetes cluster
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateClusterRequest {
    /// New label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Request to update a node pool
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateNodePoolRequest {
    /// New node count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_quantity: Option<i32>,
    /// New tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Enable/disable auto-scaler
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaler: Option<bool>,
    /// New minimum nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_nodes: Option<i32>,
    /// New maximum nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nodes: Option<i32>,
    /// Labels to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Taints to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<NodePoolTaintRequest>>,
}

/// Request to upgrade a cluster
#[derive(Debug, Clone, Serialize)]
pub struct UpgradeClusterRequest {
    /// Target version
    pub upgrade_version: String,
}

/// Request to create a node pool label
#[derive(Debug, Clone, Serialize)]
pub struct CreateNodePoolLabelRequest {
    /// Label key
    pub key: String,
    /// Label value
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_deserialize() {
        let json = r#"{
            "id": "abc123",
            "label": "my-cluster",
            "region": "ewr",
            "version": "v1.28.0+1",
            "status": "active",
            "ha_controlplanes": true,
            "node_pools": []
        }"#;
        let cluster: KubernetesCluster = serde_json::from_str(json).unwrap();
        assert_eq!(cluster.id, "abc123");
        assert_eq!(cluster.label, Some("my-cluster".into()));
        assert!(cluster.ha_controlplanes);
    }

    #[test]
    fn test_node_pool_deserialize() {
        let json = r#"{
            "id": "pool123",
            "label": "worker-pool",
            "plan": "vc2-1c-2gb",
            "node_quantity": 3,
            "auto_scaler": true,
            "min_nodes": 2,
            "max_nodes": 5,
            "nodes": []
        }"#;
        let pool: NodePool = serde_json::from_str(json).unwrap();
        assert_eq!(pool.id, "pool123");
        assert_eq!(pool.node_quantity, Some(3));
        assert!(pool.auto_scaler);
    }

    #[test]
    fn test_create_cluster_request_serialize() {
        let request = CreateClusterRequest {
            region: "ewr".into(),
            version: "v1.28.0+1".into(),
            label: Some("my-cluster".into()),
            ha_controlplanes: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"region\":\"ewr\""));
        assert!(json.contains("\"ha_controlplanes\":true"));
    }

    #[test]
    fn test_create_node_pool_request_serialize() {
        let request = CreateNodePoolRequest {
            node_quantity: 3,
            label: "workers".into(),
            plan: "vc2-2c-4gb".into(),
            auto_scaler: Some(true),
            min_nodes: Some(2),
            max_nodes: Some(10),
            tag: None,
            labels: None,
            taints: None,
            user_data: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"node_quantity\":3"));
        assert!(json.contains("\"auto_scaler\":true"));
    }

    #[test]
    fn test_kubeconfig_response_deserialize() {
        let json = r#"{"kube_config": "YXBpdmVyc2lvbjogdjE="}"#;
        let response: KubeconfigResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.kube_config, "YXBpdmVyc2lvbjogdjE=");
    }

    #[test]
    fn test_versions_response_deserialize() {
        let json = r#"{"versions": ["v1.28.0+1", "v1.27.0+1"]}"#;
        let response: VersionsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.versions.len(), 2);
    }

    #[test]
    fn test_cluster_resources_deserialize() {
        let json = r#"{
            "resources": {
                "block_storage": [
                    {"id": "bs1", "label": "volume1", "status": "active"}
                ],
                "load_balancer": []
            }
        }"#;
        let resources: ClusterResources = serde_json::from_str(json).unwrap();
        assert_eq!(resources.resources.block_storage.len(), 1);
        assert_eq!(resources.resources.load_balancer.len(), 0);
    }

    #[test]
    fn test_upgrade_cluster_request_serialize() {
        let request = UpgradeClusterRequest {
            upgrade_version: "v1.29.0+1".into(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"upgrade_version\":\"v1.29.0+1\""));
    }

    #[test]
    fn test_node_pool_taint_deserialize() {
        let json = r#"{"key": "dedicated", "value": "gpu", "effect": "NoSchedule"}"#;
        let taint: NodePoolTaint = serde_json::from_str(json).unwrap();
        assert_eq!(taint.key, Some("dedicated".into()));
        assert_eq!(taint.effect, Some("NoSchedule".into()));
    }
}
