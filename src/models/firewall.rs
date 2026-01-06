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
