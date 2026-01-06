//! VPC model types

use serde::{Deserialize, Serialize};

/// VPC (Virtual Private Cloud)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vpc {
    /// Unique ID for the VPC
    pub id: String,
    /// Region ID
    pub region: Option<String>,
    /// Date created
    pub date_created: Option<String>,
    /// User-supplied description
    pub description: Option<String>,
    /// IPv4 subnet (e.g., "10.99.0.0")
    pub v4_subnet: Option<String>,
    /// IPv4 subnet mask (CIDR bits)
    pub v4_subnet_mask: Option<i32>,
}

impl Vpc {
    /// Get the CIDR notation for this VPC
    pub fn cidr(&self) -> Option<String> {
        match (&self.v4_subnet, self.v4_subnet_mask) {
            (Some(subnet), Some(mask)) => Some(format!("{}/{}", subnet, mask)),
            _ => None,
        }
    }
}

/// VPC 2.0 (newer VPC version)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vpc2 {
    /// Unique ID for the VPC
    pub id: String,
    /// Region ID
    pub region: Option<String>,
    /// Date created
    pub date_created: Option<String>,
    /// User-supplied description
    pub description: Option<String>,
    /// IP block (e.g., "10.99.0.0")
    pub ip_block: Option<String>,
    /// Prefix length (CIDR bits)
    pub prefix_length: Option<i32>,
}

impl Vpc2 {
    /// Get the CIDR notation for this VPC
    pub fn cidr(&self) -> Option<String> {
        match (&self.ip_block, self.prefix_length) {
            (Some(block), Some(prefix)) => Some(format!("{}/{}", block, prefix)),
            _ => None,
        }
    }
}

/// Node attached to a VPC 2.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vpc2Node {
    /// Node ID (instance ID)
    pub id: Option<String>,
    /// IP address in the VPC
    pub ip_address: Option<String>,
    /// MAC address
    pub mac_address: Option<i32>,
    /// Description/label
    pub description: Option<String>,
    /// Node type
    #[serde(rename = "type")]
    pub node_type: Option<String>,
}

/// Request to create a VPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVpcRequest {
    /// Region ID
    pub region: String,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// IPv4 subnet (required if v4_subnet_mask is specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_subnet: Option<String>,
    /// IPv4 subnet mask (required if v4_subnet is specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_subnet_mask: Option<i32>,
}

/// Request to create a VPC 2.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVpc2Request {
    /// Region ID
    pub region: String,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// IP type (currently only "v4")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_type: Option<String>,
    /// IP block (required if prefix_length is specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_block: Option<String>,
    /// Prefix length (required if ip_block is specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,
}

/// Request to update a VPC
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateVpcRequest {
    /// New description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to attach nodes to VPC 2.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachVpc2NodesRequest {
    /// List of nodes to attach
    pub nodes: Vec<Vpc2NodeAttachment>,
}

/// Node attachment for VPC 2.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vpc2NodeAttachment {
    /// Instance ID
    pub id: String,
    /// IP address to assign (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

/// Request to detach nodes from VPC 2.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachVpc2NodesRequest {
    /// List of instance IDs to detach
    pub nodes: Vec<String>,
}

/// Response wrapper for VPC operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcResponse {
    pub vpc: Vpc,
}

/// Response wrapper for VPC list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcsResponse {
    pub vpcs: Vec<Vpc>,
}

/// Response wrapper for VPC 2.0 operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vpc2Response {
    pub vpc: Vpc2,
}

/// Response wrapper for VPC 2.0 list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vpcs2Response {
    pub vpcs: Vec<Vpc2>,
}

/// Response wrapper for VPC 2.0 nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vpc2NodesResponse {
    pub nodes: Vec<Vpc2Node>,
}
