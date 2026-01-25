//! Storage gateway model types

use serde::{Deserialize, Serialize};

/// Storage gateway network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGatewayNetwork {
    /// Primary network configuration
    pub primary: Option<StorageGatewayNetworkPrimary>,
}

/// Primary network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGatewayNetworkPrimary {
    /// Enable public IPv4
    pub ipv4_public_enabled: Option<bool>,
    /// Enable public IPv6
    pub ipv6_public_enabled: Option<bool>,
    /// Optional VPC attachment
    pub vpc: Option<StorageGatewayVpc>,
}

/// VPC reference for storage gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGatewayVpc {
    /// VPC UUID
    pub vpc_uuid: Option<String>,
}

/// Storage gateway export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGatewayExport {
    /// Export label
    pub label: Option<String>,
    /// VFS UUID
    pub vfs_uuid: Option<String>,
    /// Pseudo root path
    pub pseudo_root_path: Option<String>,
    /// Allowed IPs
    pub allowed_ips: Option<Vec<String>>,
}

/// Storage gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGateway {
    /// Gateway ID
    pub id: Option<String>,
    /// Date created
    pub date_created: Option<String>,
    /// Status
    pub status: Option<String>,
    /// Type
    #[serde(rename = "type")]
    pub gateway_type: Option<String>,
    /// Label
    pub label: Option<String>,
    /// Pending charges
    pub pending_charges: Option<f64>,
    /// Tags
    #[serde(default)]
    pub tags: Vec<String>,
    /// Health
    pub health: Option<String>,
    /// Network configuration
    pub network_config: Option<StorageGatewayNetwork>,
    /// Export configuration
    pub export_config: Option<Vec<StorageGatewayExport>>,
}

/// Response wrapper for storage gateways list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGatewaysResponse {
    pub storage_gateway: Vec<StorageGateway>,
    #[serde(default)]
    pub meta: crate::Meta,
}

/// Response wrapper for storage gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGatewayResponse {
    pub storage_gateway: StorageGateway,
}

/// Response wrapper for add export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGatewayExportResponse {
    #[serde(rename = "vpc")]
    pub export: StorageGatewayExport,
}

/// Request to create a storage gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStorageGatewayRequest {
    pub label: String,
    #[serde(rename = "type")]
    pub gateway_type: String,
    pub region: String,
    pub export_config: Vec<StorageGatewayExport>,
    pub network_config: StorageGatewayNetwork,
}

/// Request to update a storage gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStorageGatewayRequest {
    pub label: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_gateway_deserialize() {
        let json = r#"{
            "id": "sg-1",
            "date_created": "2024-01-01T00:00:00Z",
            "status": "active",
            "type": "nfs4",
            "label": "my-gateway",
            "pending_charges": 1.5,
            "tags": ["prod"],
            "health": "ok",
            "network_config": {
              "primary": {"ipv4_public_enabled": true}
            },
            "export_config": [{"label": "export1"}]
        }"#;
        let sg: StorageGateway = serde_json::from_str(json).unwrap();
        assert_eq!(sg.id.as_deref(), Some("sg-1"));
        assert_eq!(sg.gateway_type.as_deref(), Some("nfs4"));
        assert_eq!(sg.export_config.unwrap().len(), 1);
    }
}
