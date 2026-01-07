//! Reserved IP model types

use serde::{Deserialize, Serialize};

/// Reserved IP information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservedIp {
    /// Unique ID for the Reserved IP
    pub id: String,
    /// Region ID where the Reserved IP is located
    pub region: Option<String>,
    /// The type of IP address (v4 or v6)
    pub ip_type: Option<String>,
    /// The IP subnet
    pub subnet: Option<String>,
    /// The IP network size in bits
    pub subnet_size: Option<i32>,
    /// User-supplied label
    pub label: Option<String>,
    /// Instance ID attached to this Reserved IP
    pub instance_id: Option<String>,
}

impl ReservedIp {
    /// Check if the Reserved IP is attached to an instance
    pub fn is_attached(&self) -> bool {
        self.instance_id.is_some() && !self.instance_id.as_ref().unwrap().is_empty()
    }

    /// Get the CIDR notation for the subnet
    pub fn cidr(&self) -> Option<String> {
        match (&self.subnet, self.subnet_size) {
            (Some(subnet), Some(size)) => Some(format!("{}/{}", subnet, size)),
            _ => None,
        }
    }
}

/// Request to create a Reserved IP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReservedIpRequest {
    /// Region ID
    pub region: String,
    /// IP type (v4 or v6)
    pub ip_type: String,
    /// Optional label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Request to update a Reserved IP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReservedIpRequest {
    /// New label
    pub label: String,
}

/// Request to attach a Reserved IP to an instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachReservedIpRequest {
    /// Instance ID to attach to
    pub instance_id: String,
}

/// Request to convert an instance IP to a Reserved IP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertReservedIpRequest {
    /// The IP address to convert
    pub ip_address: String,
    /// Optional label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Response wrapper for a single Reserved IP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservedIpResponse {
    pub reserved_ip: ReservedIp,
}

/// Response wrapper for Reserved IP list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservedIpsResponse {
    pub reserved_ips: Vec<ReservedIp>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reserved_ip_is_attached_true() {
        let rip = ReservedIp {
            id: "rip-123".to_string(),
            region: Some("ewr".to_string()),
            ip_type: Some("v4".to_string()),
            subnet: Some("192.0.2.123".to_string()),
            subnet_size: Some(32),
            label: Some("Test IP".to_string()),
            instance_id: Some("inst-456".to_string()),
        };
        assert!(rip.is_attached());
    }

    #[test]
    fn test_reserved_ip_is_attached_false_none() {
        let rip = ReservedIp {
            id: "rip-123".to_string(),
            region: Some("ewr".to_string()),
            ip_type: Some("v4".to_string()),
            subnet: Some("192.0.2.123".to_string()),
            subnet_size: Some(32),
            label: Some("Test IP".to_string()),
            instance_id: None,
        };
        assert!(!rip.is_attached());
    }

    #[test]
    fn test_reserved_ip_is_attached_false_empty() {
        let rip = ReservedIp {
            id: "rip-123".to_string(),
            region: Some("ewr".to_string()),
            ip_type: Some("v4".to_string()),
            subnet: Some("192.0.2.123".to_string()),
            subnet_size: Some(32),
            label: Some("Test IP".to_string()),
            instance_id: Some("".to_string()),
        };
        assert!(!rip.is_attached());
    }

    #[test]
    fn test_reserved_ip_cidr() {
        let rip = ReservedIp {
            id: "rip-123".to_string(),
            region: Some("ewr".to_string()),
            ip_type: Some("v4".to_string()),
            subnet: Some("192.0.2.123".to_string()),
            subnet_size: Some(32),
            label: None,
            instance_id: None,
        };
        assert_eq!(rip.cidr(), Some("192.0.2.123/32".to_string()));
    }

    #[test]
    fn test_reserved_ip_cidr_ipv6() {
        let rip = ReservedIp {
            id: "rip-456".to_string(),
            region: Some("ewr".to_string()),
            ip_type: Some("v6".to_string()),
            subnet: Some("2001:db8:5:5157::".to_string()),
            subnet_size: Some(64),
            label: None,
            instance_id: None,
        };
        assert_eq!(rip.cidr(), Some("2001:db8:5:5157::/64".to_string()));
    }

    #[test]
    fn test_reserved_ip_cidr_none() {
        let rip = ReservedIp {
            id: "rip-789".to_string(),
            region: None,
            ip_type: None,
            subnet: None,
            subnet_size: None,
            label: None,
            instance_id: None,
        };
        assert_eq!(rip.cidr(), None);
    }

    #[test]
    fn test_reserved_ip_deserialize() {
        let json = r#"{
            "id": "rip-abc",
            "region": "ewr",
            "ip_type": "v4",
            "subnet": "192.0.2.123",
            "subnet_size": 32,
            "label": "My Reserved IP",
            "instance_id": "inst-xyz"
        }"#;
        let rip: ReservedIp = serde_json::from_str(json).unwrap();
        assert_eq!(rip.id, "rip-abc");
        assert_eq!(rip.region.unwrap(), "ewr");
        assert_eq!(rip.ip_type.unwrap(), "v4");
        assert_eq!(rip.subnet.unwrap(), "192.0.2.123");
        assert_eq!(rip.subnet_size.unwrap(), 32);
        assert_eq!(rip.label.unwrap(), "My Reserved IP");
        assert_eq!(rip.instance_id.unwrap(), "inst-xyz");
    }

    #[test]
    fn test_create_reserved_ip_request_serialize() {
        let req = CreateReservedIpRequest {
            region: "ewr".to_string(),
            ip_type: "v4".to_string(),
            label: Some("Test Label".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("ewr"));
        assert!(json.contains("v4"));
        assert!(json.contains("Test Label"));
    }

    #[test]
    fn test_create_reserved_ip_request_serialize_without_label() {
        let req = CreateReservedIpRequest {
            region: "lax".to_string(),
            ip_type: "v6".to_string(),
            label: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("lax"));
        assert!(json.contains("v6"));
        assert!(!json.contains("label"));
    }

    #[test]
    fn test_update_reserved_ip_request_serialize() {
        let req = UpdateReservedIpRequest {
            label: "New Label".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("New Label"));
    }

    #[test]
    fn test_attach_reserved_ip_request_serialize() {
        let req = AttachReservedIpRequest {
            instance_id: "inst-123".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("inst-123"));
    }

    #[test]
    fn test_convert_reserved_ip_request_serialize() {
        let req = ConvertReservedIpRequest {
            ip_address: "192.0.2.123".to_string(),
            label: Some("Converted IP".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("192.0.2.123"));
        assert!(json.contains("Converted IP"));
    }

    #[test]
    fn test_convert_reserved_ip_request_serialize_without_label() {
        let req = ConvertReservedIpRequest {
            ip_address: "10.0.0.1".to_string(),
            label: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("10.0.0.1"));
        assert!(!json.contains("label"));
    }

    #[test]
    fn test_reserved_ip_response_deserialize() {
        let json = r#"{
            "reserved_ip": {
                "id": "rip-resp",
                "region": "ewr",
                "ip_type": "v4",
                "subnet": "192.0.2.100",
                "subnet_size": 32,
                "label": "Response Test",
                "instance_id": null
            }
        }"#;
        let resp: ReservedIpResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.reserved_ip.id, "rip-resp");
        assert_eq!(resp.reserved_ip.label.unwrap(), "Response Test");
    }

    #[test]
    fn test_reserved_ips_response_deserialize() {
        let json = r#"{
            "reserved_ips": [
                {
                    "id": "rip-1",
                    "region": "ewr",
                    "ip_type": "v4",
                    "subnet": "192.0.2.1",
                    "subnet_size": 32,
                    "label": "First",
                    "instance_id": null
                },
                {
                    "id": "rip-2",
                    "region": "lax",
                    "ip_type": "v6",
                    "subnet": "2001:db8::",
                    "subnet_size": 64,
                    "label": "Second",
                    "instance_id": "inst-abc"
                }
            ]
        }"#;
        let resp: ReservedIpsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.reserved_ips.len(), 2);
        assert_eq!(resp.reserved_ips[0].id, "rip-1");
        assert_eq!(resp.reserved_ips[1].id, "rip-2");
    }
}
