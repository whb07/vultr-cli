//! Private network model types

use serde::{Deserialize, Serialize};

/// Private network (deprecated)
#[derive(Serialize, Deserialize)]
pub struct Network {
    /// Network ID
    pub id: String,
    /// Region ID
    pub region: Option<String>,
    /// Date created
    pub date_created: Option<String>,
    /// Description
    pub description: Option<String>,
    /// IPv4 subnet
    pub v4_subnet: Option<String>,
    /// IPv4 subnet mask
    pub v4_subnet_mask: Option<i32>,
}

impl Network {
    /// CIDR notation for this network
    pub fn cidr(&self) -> Option<String> {
        match (&self.v4_subnet, self.v4_subnet_mask) {
            (Some(subnet), Some(mask)) => Some(format!("{}/{}", subnet, mask)),
            _ => None,
        }
    }
}

/// Response wrapper for networks list
#[derive(Serialize, Deserialize)]
pub struct NetworksResponse {
    pub networks: Vec<Network>,
    #[serde(default)]
    pub meta: crate::Meta,
}

/// Response wrapper for network
#[derive(Serialize, Deserialize)]
pub struct NetworkResponse {
    pub network: Network,
}

/// Request to create a private network
#[derive(Serialize, Deserialize)]
pub struct CreateNetworkRequest {
    pub region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_subnet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_subnet_mask: Option<i32>,
}

/// Request to update a private network
#[derive(Serialize, Deserialize)]
pub struct UpdateNetworkRequest {
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_cidr() {
        let net = Network {
            id: "net-1".to_string(),
            region: Some("ewr".to_string()),
            date_created: None,
            description: None,
            v4_subnet: Some("10.0.0.0".to_string()),
            v4_subnet_mask: Some(24),
        };
        assert_eq!(net.cidr().as_deref(), Some("10.0.0.0/24"));
    }
}
