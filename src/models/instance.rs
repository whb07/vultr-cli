//! Instance (VM) model types

use serde::{Deserialize, Serialize};

/// Instance status values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstanceStatus {
    Active,
    Pending,
    Suspended,
    Resizing,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for InstanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceStatus::Active => write!(f, "active"),
            InstanceStatus::Pending => write!(f, "pending"),
            InstanceStatus::Suspended => write!(f, "suspended"),
            InstanceStatus::Resizing => write!(f, "resizing"),
            InstanceStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// Instance power status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PowerStatus {
    Running,
    Stopped,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for PowerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerStatus::Running => write!(f, "running"),
            PowerStatus::Stopped => write!(f, "stopped"),
            PowerStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// Instance server status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerStatus {
    None,
    Locked,
    Installingbooting,
    Ok,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerStatus::None => write!(f, "none"),
            ServerStatus::Locked => write!(f, "locked"),
            ServerStatus::Installingbooting => write!(f, "installing/booting"),
            ServerStatus::Ok => write!(f, "ok"),
            ServerStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// IPv6 network information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv6Network {
    /// IPv6 subnet
    pub network: Option<String>,
    /// Main IPv6 address
    pub main_ip: Option<String>,
    /// Network size in bits
    pub network_size: Option<i32>,
}

/// VPS Instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    /// Unique instance ID
    pub id: String,
    /// Operating system name
    pub os: Option<String>,
    /// RAM in MB
    pub ram: Option<i32>,
    /// Disk size in GB
    pub disk: Option<i32>,
    /// Main IPv4 address
    pub main_ip: Option<String>,
    /// Number of vCPUs
    pub vcpu_count: Option<i32>,
    /// Region ID
    pub region: Option<String>,
    /// Default password (only available briefly after deployment)
    pub default_password: Option<String>,
    /// Date created
    pub date_created: Option<String>,
    /// Current status
    pub status: Option<InstanceStatus>,
    /// Power status
    pub power_status: Option<PowerStatus>,
    /// Server health status
    pub server_status: Option<ServerStatus>,
    /// Monthly bandwidth quota in GB
    pub allowed_bandwidth: Option<i32>,
    /// IPv4 netmask
    pub netmask_v4: Option<String>,
    /// IPv4 gateway
    pub gateway_v4: Option<String>,
    /// IPv6 networks
    #[serde(default)]
    pub v6_networks: Vec<Ipv6Network>,
    /// Hostname
    pub hostname: Option<String>,
    /// User-supplied label
    pub label: Option<String>,
    /// Internal IP (when VPC attached)
    pub internal_ip: Option<String>,
    /// NoVNC console URL
    pub kvm: Option<String>,
    /// OS ID
    pub os_id: Option<i32>,
    /// Application ID
    pub app_id: Option<i32>,
    /// Application image ID
    pub image_id: Option<String>,
    /// Firewall group ID
    pub firewall_group_id: Option<String>,
    /// Enabled features
    #[serde(default)]
    pub features: Vec<String>,
    /// Plan ID
    pub plan: Option<String>,
    /// Tags
    #[serde(default)]
    pub tags: Vec<String>,
    /// User scheme (root or limited)
    pub user_scheme: Option<String>,
}

impl Instance {
    /// Check if the instance is ready (active and running)
    pub fn is_ready(&self) -> bool {
        self.status == Some(InstanceStatus::Active) 
            && self.power_status == Some(PowerStatus::Running)
            && self.server_status == Some(ServerStatus::Ok)
    }
}

/// Request to create a new instance
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateInstanceRequest {
    /// Region ID (required)
    pub region: String,
    /// Plan ID (required)
    pub plan: String,
    /// OS ID (one of os_id, iso_id, snapshot_id, or app_id required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_id: Option<i32>,
    /// ISO ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_id: Option<String>,
    /// Snapshot ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// Application ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i32>,
    /// Application image ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// Startup script ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_id: Option<String>,
    /// Enable IPv6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    /// Disable public IPv4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_public_ipv4: Option<bool>,
    /// VPC IDs to attach
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_vpc: Option<Vec<String>>,
    /// Instance label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// SSH key IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sshkey_id: Option<Vec<String>>,
    /// Enable automatic backups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<String>,
    /// User data (base64 encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// Enable DDoS protection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddos_protection: Option<bool>,
    /// Send activation email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_email: Option<bool>,
    /// Hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Firewall group ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_group_id: Option<String>,
    /// Reserved IPv4 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_ipv4: Option<String>,
    /// Enable VPC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_vpc: Option<bool>,
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// User scheme (root or limited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_scheme: Option<String>,
}

/// Request to update an instance
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateInstanceRequest {
    /// Application ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i32>,
    /// Application image ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// Enable IPv6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    /// Instance label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Backups setting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<String>,
    /// User data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// DDoS protection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddos_protection: Option<bool>,
    /// Firewall group ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_group_id: Option<String>,
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Plan ID (for resizing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// OS ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_id: Option<i32>,
}

/// Request to reinstall an instance
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReinstallInstanceRequest {
    /// Hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

/// Response wrapper for instance operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceResponse {
    pub instance: Instance,
}

/// Response wrapper for instance list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancesResponse {
    pub instances: Vec<Instance>,
}

/// Instance bandwidth data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthData {
    /// Incoming bytes
    pub incoming_bytes: Option<i64>,
    /// Outgoing bytes
    pub outgoing_bytes: Option<i64>,
}

/// Instance bandwidth response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthResponse {
    pub bandwidth: std::collections::HashMap<String, BandwidthData>,
}

/// Instance IPv4 information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv4Info {
    pub ip: String,
    pub netmask: Option<String>,
    pub gateway: Option<String>,
    #[serde(rename = "type")]
    pub ip_type: Option<String>,
    pub reverse: Option<String>,
}

/// Response for instance IPv4 list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv4Response {
    pub ipv4s: Vec<Ipv4Info>,
}
