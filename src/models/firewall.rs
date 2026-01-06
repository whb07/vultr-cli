//! Firewall model types

use serde::{Deserialize, Serialize};

/// IP type for firewall rules
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IpType {
    V4,
    V6,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for IpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpType::V4 => write!(f, "v4"),
            IpType::V6 => write!(f, "v6"),
            IpType::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for IpType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "v4" | "ipv4" => Ok(IpType::V4),
            "v6" | "ipv6" => Ok(IpType::V6),
            _ => Err(format!("Unknown IP type: {}", s)),
        }
    }
}

/// Protocol for firewall rules
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Gre,
    Esp,
    Ah,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Tcp => write!(f, "TCP"),
            Protocol::Udp => write!(f, "UDP"),
            Protocol::Icmp => write!(f, "ICMP"),
            Protocol::Gre => write!(f, "GRE"),
            Protocol::Esp => write!(f, "ESP"),
            Protocol::Ah => write!(f, "AH"),
            Protocol::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TCP" => Ok(Protocol::Tcp),
            "UDP" => Ok(Protocol::Udp),
            "ICMP" => Ok(Protocol::Icmp),
            "GRE" => Ok(Protocol::Gre),
            "ESP" => Ok(Protocol::Esp),
            "AH" => Ok(Protocol::Ah),
            _ => Err(format!("Unknown protocol: {}", s)),
        }
    }
}

/// Firewall Group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallGroup {
    /// Unique ID for the firewall group
    pub id: String,
    /// User-supplied description
    pub description: Option<String>,
    /// Date created
    pub date_created: Option<String>,
    /// Date last modified
    pub date_modified: Option<String>,
    /// Number of instances using this group
    pub instance_count: Option<i32>,
    /// Number of rules in this group
    pub rule_count: Option<i32>,
    /// Maximum number of rules allowed
    pub max_rule_count: Option<i32>,
}

/// Firewall Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    /// Unique ID for the rule
    pub id: i32,
    /// IP type (v4 or v6)
    pub ip_type: Option<IpType>,
    /// Action (currently only "accept")
    pub action: Option<String>,
    /// Protocol
    pub protocol: Option<Protocol>,
    /// Port or port range
    pub port: Option<String>,
    /// Subnet IP address
    pub subnet: Option<String>,
    /// Subnet size (CIDR bits)
    pub subnet_size: Option<i32>,
    /// Source (empty string, "cloudflare", or load balancer ID)
    pub source: Option<String>,
    /// User-supplied notes
    pub notes: Option<String>,
}

impl FirewallRule {
    /// Get the CIDR notation for this rule
    pub fn cidr(&self) -> Option<String> {
        match (&self.subnet, self.subnet_size) {
            (Some(subnet), Some(size)) => Some(format!("{}/{}", subnet, size)),
            _ => None,
        }
    }
}

/// Request to create a firewall group
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFirewallGroupRequest {
    /// Description for the firewall group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to update a firewall group
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFirewallGroupRequest {
    /// New description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to create a firewall rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFirewallRuleRequest {
    /// IP type (v4 or v6)
    pub ip_type: String,
    /// Protocol (TCP, UDP, ICMP, GRE, ESP, AH)
    pub protocol: String,
    /// Subnet IP address
    pub subnet: String,
    /// Subnet size (CIDR bits)
    pub subnet_size: i32,
    /// Port or port range (for TCP/UDP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// Source (empty, "cloudflare", or load balancer ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Response wrapper for firewall group operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallGroupResponse {
    pub firewall_group: FirewallGroup,
}

/// Response wrapper for firewall group list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallGroupsResponse {
    pub firewall_groups: Vec<FirewallGroup>,
}

/// Response wrapper for firewall rule operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRuleResponse {
    pub firewall_rule: FirewallRule,
}

/// Response wrapper for firewall rule list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRulesResponse {
    pub firewall_rules: Vec<FirewallRule>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_type_display() {
        assert_eq!(format!("{}", IpType::V4), "v4");
        assert_eq!(format!("{}", IpType::V6), "v6");
        assert_eq!(format!("{}", IpType::Unknown), "unknown");
    }

    #[test]
    fn test_ip_type_from_str() {
        assert_eq!("v4".parse::<IpType>().unwrap(), IpType::V4);
        assert_eq!("ipv4".parse::<IpType>().unwrap(), IpType::V4);
        assert_eq!("v6".parse::<IpType>().unwrap(), IpType::V6);
        assert_eq!("ipv6".parse::<IpType>().unwrap(), IpType::V6);
    }

    #[test]
    fn test_ip_type_from_str_invalid() {
        let result = "invalid".parse::<IpType>();
        assert!(result.is_err());
    }

    #[test]
    fn test_protocol_display() {
        assert_eq!(format!("{}", Protocol::Tcp), "TCP");
        assert_eq!(format!("{}", Protocol::Udp), "UDP");
        assert_eq!(format!("{}", Protocol::Icmp), "ICMP");
        assert_eq!(format!("{}", Protocol::Gre), "GRE");
        assert_eq!(format!("{}", Protocol::Esp), "ESP");
        assert_eq!(format!("{}", Protocol::Ah), "AH");
        assert_eq!(format!("{}", Protocol::Unknown), "unknown");
    }

    #[test]
    fn test_protocol_from_str() {
        assert_eq!("TCP".parse::<Protocol>().unwrap(), Protocol::Tcp);
        assert_eq!("tcp".parse::<Protocol>().unwrap(), Protocol::Tcp);
        assert_eq!("UDP".parse::<Protocol>().unwrap(), Protocol::Udp);
        assert_eq!("ICMP".parse::<Protocol>().unwrap(), Protocol::Icmp);
    }

    #[test]
    fn test_protocol_from_str_invalid() {
        let result = "INVALID".parse::<Protocol>();
        assert!(result.is_err());
    }

    #[test]
    fn test_firewall_rule_cidr() {
        let rule = FirewallRule {
            id: 1,
            ip_type: Some(IpType::V4),
            action: Some("accept".to_string()),
            protocol: Some(Protocol::Tcp),
            port: Some("22".to_string()),
            subnet: Some("10.0.0.0".to_string()),
            subnet_size: Some(8),
            source: None,
            notes: None,
        };
        assert_eq!(rule.cidr().unwrap(), "10.0.0.0/8");
    }

    #[test]
    fn test_firewall_rule_cidr_none() {
        let rule = FirewallRule {
            id: 1,
            ip_type: None,
            action: None,
            protocol: None,
            port: None,
            subnet: None,
            subnet_size: None,
            source: None,
            notes: None,
        };
        assert!(rule.cidr().is_none());
    }

    #[test]
    fn test_firewall_rule_cidr_partial() {
        let rule = FirewallRule {
            id: 1,
            ip_type: None,
            action: None,
            protocol: None,
            port: None,
            subnet: Some("192.168.0.0".to_string()),
            subnet_size: None,
            source: None,
            notes: None,
        };
        assert!(rule.cidr().is_none());
    }

    #[test]
    fn test_firewall_group_deserialize() {
        let json =
            r#"{"id":"fw-123","description":"Web servers","rule_count":5,"instance_count":3}"#;
        let group: FirewallGroup = serde_json::from_str(json).unwrap();
        assert_eq!(group.id, "fw-123");
        assert_eq!(group.description.unwrap(), "Web servers");
        assert_eq!(group.rule_count.unwrap(), 5);
    }

    #[test]
    fn test_firewall_rule_deserialize() {
        let json = r#"{"id":1,"ip_type":"v4","action":"accept","protocol":"TCP","port":"443","subnet":"0.0.0.0","subnet_size":0}"#;
        let rule: FirewallRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.id, 1);
        assert_eq!(rule.ip_type.unwrap(), IpType::V4);
        assert_eq!(rule.protocol.unwrap(), Protocol::Tcp);
    }

    #[test]
    fn test_create_firewall_rule_request_serialize() {
        let req = CreateFirewallRuleRequest {
            ip_type: "v4".to_string(),
            protocol: "TCP".to_string(),
            subnet: "0.0.0.0".to_string(),
            subnet_size: 0,
            port: Some("80".to_string()),
            source: None,
            notes: Some("HTTP".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("TCP"));
        assert!(json.contains("HTTP"));
    }

    #[test]
    fn test_ip_type_unknown_variant() {
        let json = r#""ipv8""#;
        let ip_type: IpType = serde_json::from_str(json).unwrap();
        assert_eq!(ip_type, IpType::Unknown);
    }

    #[test]
    fn test_protocol_unknown_variant() {
        let json = r#""QUIC""#;
        let protocol: Protocol = serde_json::from_str(json).unwrap();
        assert_eq!(protocol, Protocol::Unknown);
    }
}
