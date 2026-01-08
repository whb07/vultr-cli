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
    /// Internet connectivity
    pub internet: Option<VpcInternet>,
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

/// VPC internet connectivity details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcInternet {
    /// Whether internet connectivity is enabled
    pub connectivity: Option<bool>,
    /// Connectivity types
    #[serde(default)]
    pub types: Vec<String>,
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

/// VPC attachment IP information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcAttachmentIp {
    /// IPv4 address
    pub v4: Option<String>,
}

/// VPC attachment linked subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcAttachmentLinkedSubscription {
    /// Subscription ID
    pub id: Option<String>,
    /// Subscription type
    #[serde(rename = "type")]
    pub subscription_type: Option<String>,
}

/// VPC attachment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcAttachment {
    /// Attachment ID
    pub id: Option<String>,
    /// Attachment type (vps/baremetal)
    #[serde(rename = "type")]
    pub attachment_type: Option<String>,
    /// MAC address
    pub mac_address: Option<String>,
    /// Date added
    pub date_added: Option<String>,
    /// Attachment IP
    pub ip: Option<VpcAttachmentIp>,
    /// Linked subscription
    pub linked_subscription: Option<VpcAttachmentLinkedSubscription>,
}

/// Response wrapper for VPC attachments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcAttachmentsResponse {
    pub attachments: Vec<VpcAttachment>,
    pub meta: Option<crate::models::Meta>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vpc_cidr() {
        let vpc = Vpc {
            id: "vpc-123".to_string(),
            region: Some("ewr".to_string()),
            date_created: None,
            description: Some("Test VPC".to_string()),
            v4_subnet: Some("10.0.0.0".to_string()),
            v4_subnet_mask: Some(16),
            internet: None,
        };
        assert_eq!(vpc.cidr().unwrap(), "10.0.0.0/16");
    }

    #[test]
    fn test_vpc_cidr_none() {
        let vpc = Vpc {
            id: "vpc-123".to_string(),
            region: None,
            date_created: None,
            description: None,
            v4_subnet: None,
            v4_subnet_mask: None,
            internet: None,
        };
        assert!(vpc.cidr().is_none());
    }

    #[test]
    fn test_vpc_cidr_partial() {
        let vpc = Vpc {
            id: "vpc-123".to_string(),
            region: None,
            date_created: None,
            description: None,
            v4_subnet: Some("10.0.0.0".to_string()),
            v4_subnet_mask: None,
            internet: None,
        };
        assert!(vpc.cidr().is_none());
    }

    #[test]
    fn test_vpc2_cidr() {
        let vpc2 = Vpc2 {
            id: "vpc2-123".to_string(),
            region: Some("lax".to_string()),
            date_created: None,
            description: Some("Test VPC 2.0".to_string()),
            ip_block: Some("172.16.0.0".to_string()),
            prefix_length: Some(24),
        };
        assert_eq!(vpc2.cidr().unwrap(), "172.16.0.0/24");
    }

    #[test]
    fn test_vpc2_cidr_none() {
        let vpc2 = Vpc2 {
            id: "vpc2-123".to_string(),
            region: None,
            date_created: None,
            description: None,
            ip_block: None,
            prefix_length: None,
        };
        assert!(vpc2.cidr().is_none());
    }

    #[test]
    fn test_vpc_deserialize() {
        let json = r#"{"id":"vpc-abc","region":"ewr","description":"Production VPC","v4_subnet":"10.1.0.0","v4_subnet_mask":24}"#;
        let vpc: Vpc = serde_json::from_str(json).unwrap();
        assert_eq!(vpc.id, "vpc-abc");
        assert_eq!(vpc.region.unwrap(), "ewr");
        assert_eq!(vpc.v4_subnet.unwrap(), "10.1.0.0");
        assert_eq!(vpc.v4_subnet_mask.unwrap(), 24);
    }

    #[test]
    fn test_vpc2_deserialize() {
        let json =
            r#"{"id":"vpc2-xyz","region":"lax","ip_block":"192.168.0.0","prefix_length":16}"#;
        let vpc2: Vpc2 = serde_json::from_str(json).unwrap();
        assert_eq!(vpc2.id, "vpc2-xyz");
        assert_eq!(vpc2.ip_block.unwrap(), "192.168.0.0");
    }

    #[test]
    fn test_create_vpc_request_serialize() {
        let req = CreateVpcRequest {
            region: "ewr".to_string(),
            description: Some("New VPC".to_string()),
            v4_subnet: Some("10.0.0.0".to_string()),
            v4_subnet_mask: Some(16),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("ewr"));
        assert!(json.contains("10.0.0.0"));
    }

    #[test]
    fn test_create_vpc_request_minimal() {
        let req = CreateVpcRequest {
            region: "lax".to_string(),
            description: None,
            v4_subnet: None,
            v4_subnet_mask: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("lax"));
        assert!(!json.contains("v4_subnet"));
    }

    #[test]
    fn test_update_vpc_request_default() {
        let req = UpdateVpcRequest::default();
        assert!(req.description.is_none());
    }

    #[test]
    fn test_vpc2_node_deserialize() {
        let json = r#"{"id":"inst-123","ip_address":"10.0.0.5","description":"Web Server","type":"instance"}"#;
        let node: Vpc2Node = serde_json::from_str(json).unwrap();
        assert_eq!(node.id.unwrap(), "inst-123");
        assert_eq!(node.ip_address.unwrap(), "10.0.0.5");
        assert_eq!(node.node_type.unwrap(), "instance");
    }

    #[test]
    fn test_vpc2_node_attachment_serialize() {
        let attachment = Vpc2NodeAttachment {
            id: "inst-456".to_string(),
            ip_address: Some("10.0.0.10".to_string()),
        };
        let json = serde_json::to_string(&attachment).unwrap();
        assert!(json.contains("inst-456"));
        assert!(json.contains("10.0.0.10"));
    }

    #[test]
    fn test_attach_vpc2_nodes_request() {
        let req = AttachVpc2NodesRequest {
            nodes: vec![
                Vpc2NodeAttachment {
                    id: "inst-1".to_string(),
                    ip_address: None,
                },
                Vpc2NodeAttachment {
                    id: "inst-2".to_string(),
                    ip_address: Some("10.0.0.2".to_string()),
                },
            ],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("inst-1"));
        assert!(json.contains("inst-2"));
    }

    #[test]
    fn test_detach_vpc2_nodes_request() {
        let req = DetachVpc2NodesRequest {
            nodes: vec!["inst-1".to_string(), "inst-2".to_string()],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("inst-1"));
    }
}
