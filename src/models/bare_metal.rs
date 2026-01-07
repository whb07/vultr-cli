//! Bare Metal server models

use serde::{Deserialize, Serialize};

/// Bare Metal server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareMetal {
    /// A unique ID for the Bare Metal instance
    pub id: String,
    /// The Operating System name
    #[serde(default)]
    pub os: Option<String>,
    /// Text description of the instances' RAM
    #[serde(default)]
    pub ram: Option<String>,
    /// Text description of the instances' disk configuration
    #[serde(default)]
    pub disk: Option<String>,
    /// The main IPv4 address
    #[serde(default)]
    pub main_ip: Option<String>,
    /// Number of CPUs
    #[serde(default)]
    pub cpu_count: Option<i32>,
    /// The Region id where the instance is located
    #[serde(default)]
    pub region: Option<String>,
    /// The default password assigned at deployment
    #[serde(default)]
    pub default_password: Option<String>,
    /// The date this instance was created
    #[serde(default)]
    pub date_created: Option<String>,
    /// The current status (active, pending, suspended)
    #[serde(default)]
    pub status: Option<String>,
    /// The IPv4 netmask in dot-decimal notation
    #[serde(default)]
    pub netmask_v4: Option<String>,
    /// The IPv4 gateway address
    #[serde(default)]
    pub gateway_v4: Option<String>,
    /// The Bare Metal Plan id
    #[serde(default)]
    pub plan: Option<String>,
    /// The user-supplied label for this instance
    #[serde(default)]
    pub label: Option<String>,
    /// The Operating System id
    #[serde(default)]
    pub os_id: Option<i32>,
    /// The Application id
    #[serde(default)]
    pub app_id: Option<i32>,
    /// The Application image_id
    #[serde(default)]
    pub image_id: Option<String>,
    /// The Snapshot id
    #[serde(default)]
    pub snapshot_id: Option<String>,
    /// The IPv6 network size in bits
    #[serde(default)]
    pub v6_network: Option<String>,
    /// The main IPv6 network address
    #[serde(default)]
    pub v6_main_ip: Option<String>,
    /// The IPv6 subnet
    #[serde(default)]
    pub v6_network_size: Option<i32>,
    /// The MAC address for a Bare Metal server
    #[serde(default)]
    pub mac_address: Option<i64>,
    /// Tags to apply to the instance
    #[serde(default)]
    pub tags: Vec<String>,
    /// The user scheme (root, limited)
    #[serde(default)]
    pub user_scheme: Option<String>,
    /// The internal IP used by this instance, if set
    #[serde(default)]
    pub internal_ip: Option<String>,
    /// List of VPC Networks to which the instance is attached
    #[serde(default)]
    pub vpcs: Vec<BareMetalVpc>,
}

/// Bare Metal VPC attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareMetalVpc {
    /// VPC ID
    pub id: String,
    /// MAC address
    #[serde(default)]
    pub mac_address: Option<String>,
    /// IP address
    #[serde(default)]
    pub ip_address: Option<String>,
}

/// Bare Metal IPv4 information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareMetalIpv4 {
    /// The IPv4 address
    pub ip: String,
    /// The IPv4 netmask in dot-decimal notation
    #[serde(default)]
    pub netmask: Option<String>,
    /// The gateway IP address
    #[serde(default)]
    pub gateway: Option<String>,
    /// The type of IP address (main_ip)
    #[serde(default, rename = "type")]
    pub ip_type: Option<String>,
    /// The reverse DNS information for this IP address
    #[serde(default)]
    pub reverse: Option<String>,
    /// The MAC address associated with this IP address
    #[serde(default)]
    pub mac_address: Option<String>,
}

/// Bare Metal IPv6 information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareMetalIpv6 {
    /// A unique ID for the IPv6 address
    pub ip: String,
    /// The IPv6 subnet
    #[serde(default)]
    pub network: Option<String>,
    /// The IPv6 network size in bits
    #[serde(default)]
    pub network_size: Option<i32>,
    /// The type of IP address (main_ip)
    #[serde(default, rename = "type")]
    pub ip_type: Option<String>,
}

/// Available upgrades for Bare Metal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareMetalUpgrades {
    /// Available operating systems
    #[serde(default)]
    pub os: Vec<BareMetalUpgradeOs>,
    /// Available applications
    #[serde(default)]
    pub applications: Vec<BareMetalUpgradeApp>,
}

/// OS upgrade option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareMetalUpgradeOs {
    /// OS ID
    pub id: i32,
    /// OS name
    pub name: String,
    /// OS architecture
    #[serde(default)]
    pub arch: Option<String>,
    /// OS family
    #[serde(default)]
    pub family: Option<String>,
}

/// Application upgrade option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareMetalUpgradeApp {
    /// Application ID
    pub id: i32,
    /// Application name
    pub name: String,
    /// Short name
    #[serde(default)]
    pub short_name: Option<String>,
    /// Deploy name
    #[serde(default)]
    pub deploy_name: Option<String>,
}

/// VNC URL information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareMetalVnc {
    /// VNC URL
    pub url: String,
}

/// User data response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareMetalUserData {
    /// Base64 encoded user data
    pub data: String,
}

// ==================
// Request Types
// ==================

/// Request to create a Bare Metal server
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBareMetalRequest {
    /// Region ID
    pub region: String,
    /// Plan ID
    pub plan: String,
    /// Operating System ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_id: Option<i32>,
    /// Snapshot ID to deploy from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// Application ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i32>,
    /// Application image ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// SSH key IDs to install
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sshkey_id: Option<Vec<String>>,
    /// Startup script ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_id: Option<String>,
    /// User-supplied label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Enable IPv6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    /// VPC ID to attach
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_vpc: Option<Vec<String>>,
    /// VPC2 ID to attach
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_vpc2: Option<Vec<String>>,
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// User data (cloud-init)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// Reserved IPv4 address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_ipv4: Option<String>,
    /// Persistent PXE boot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_pxe: Option<bool>,
    /// Activation email (true to send)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_email: Option<bool>,
    /// Hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Mdisk mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdisk_mode: Option<String>,
    /// User scheme (root or limited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_scheme: Option<String>,
}

/// Request to update a Bare Metal server
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateBareMetalRequest {
    /// User-supplied label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Enable IPv6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    /// User data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// VPCs to attach
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_vpc: Option<Vec<String>>,
    /// VPCs to detach
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_vpc: Option<Vec<String>>,
    /// VPC2s to attach
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_vpc2: Option<Vec<String>>,
    /// VPC2s to detach
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_vpc2: Option<Vec<String>>,
}

/// Request to reinstall a Bare Metal server
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReinstallBareMetalRequest {
    /// Hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

/// Request to set reverse DNS for IPv4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBareMetalReverseIpv4Request {
    /// IPv4 address
    pub ip: String,
    /// Reverse DNS hostname
    pub reverse: String,
}

/// Request to set default reverse DNS for IPv4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBareMetalDefaultReverseIpv4Request {
    /// IPv4 address
    pub ip: String,
}

/// Request to set reverse DNS for IPv6
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBareMetalReverseIpv6Request {
    /// IPv6 address
    pub ip: String,
    /// Reverse DNS hostname
    pub reverse: String,
}

/// Request for bulk operations on Bare Metal servers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkBareMetalRequest {
    /// Bare Metal IDs
    pub baremetal_ids: Vec<String>,
}

/// Request to attach VPC to Bare Metal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachBareMetalVpcRequest {
    /// VPC ID
    pub vpc_id: String,
}

/// Request to detach VPC from Bare Metal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachBareMetalVpcRequest {
    /// VPC ID
    pub vpc_id: String,
}

/// Request to attach VPC2 to Bare Metal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachBareMetalVpc2Request {
    /// VPC2 ID
    pub vpc_id: String,
    /// IP address to assign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

/// Request to detach VPC2 from Bare Metal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachBareMetalVpc2Request {
    /// VPC2 ID
    pub vpc_id: String,
}

// ==================
// Response Types
// ==================

/// Response wrapper for a single Bare Metal server
#[derive(Debug, Clone, Deserialize)]
pub struct BareMetalResponse {
    /// The Bare Metal server
    pub bare_metal: BareMetal,
}

/// Response wrapper for list of Bare Metal servers
#[derive(Debug, Clone, Deserialize)]
pub struct BareMetalsResponse {
    /// List of Bare Metal servers
    pub bare_metals: Vec<BareMetal>,
}

/// Response wrapper for Bare Metal IPv4 addresses
#[derive(Debug, Clone, Deserialize)]
pub struct BareMetalIpv4Response {
    /// List of IPv4 addresses
    pub ipv4s: Vec<BareMetalIpv4>,
}

/// Response wrapper for Bare Metal IPv6 addresses
#[derive(Debug, Clone, Deserialize)]
pub struct BareMetalIpv6Response {
    /// List of IPv6 addresses
    pub ipv6s: Vec<BareMetalIpv6>,
}

/// Response wrapper for Bare Metal upgrades
#[derive(Debug, Clone, Deserialize)]
pub struct BareMetalUpgradesResponse {
    /// Available upgrades
    pub upgrades: BareMetalUpgrades,
}

/// Response wrapper for Bare Metal VNC
#[derive(Debug, Clone, Deserialize)]
pub struct BareMetalVncResponse {
    /// VNC information
    pub vnc: BareMetalVnc,
}

/// Response wrapper for Bare Metal user data
#[derive(Debug, Clone, Deserialize)]
pub struct BareMetalUserDataResponse {
    /// User data
    pub user_data: BareMetalUserData,
}

/// Response wrapper for Bare Metal VPCs
#[derive(Debug, Clone, Deserialize)]
pub struct BareMetalVpcsResponse {
    /// List of VPCs
    pub vpcs: Vec<BareMetalVpc>,
}

/// Bare Metal VPC2 attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareMetalVpc2 {
    /// VPC2 ID
    pub id: String,
    /// MAC address
    #[serde(default)]
    pub mac_address: Option<String>,
    /// IP address
    #[serde(default)]
    pub ip_address: Option<String>,
}

/// Response wrapper for Bare Metal VPC2s
#[derive(Debug, Clone, Deserialize)]
pub struct BareMetalVpc2sResponse {
    /// List of VPC2s
    pub vpcs: Vec<BareMetalVpc2>,
}

/// Response wrapper for Bare Metal bandwidth
#[derive(Debug, Clone, Deserialize)]
pub struct BareMetalBandwidthResponse {
    /// Bandwidth data by date
    pub bandwidth: std::collections::HashMap<String, crate::models::BandwidthData>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bare_metal_deserialize() {
        let json = r#"{
            "id": "cb676a46-66fd-4dfb-b839-443f2e6c0b60",
            "os": "CentOS 8 x64",
            "ram": "32768 MB",
            "disk": "2x 240GB SSD",
            "main_ip": "192.0.2.123",
            "cpu_count": 4,
            "region": "ewr",
            "status": "active",
            "plan": "vbm-4c-32gb",
            "label": "Test Server",
            "tags": ["web", "production"]
        }"#;

        let bm: BareMetal = serde_json::from_str(json).unwrap();
        assert_eq!(bm.id, "cb676a46-66fd-4dfb-b839-443f2e6c0b60");
        assert_eq!(bm.os.as_deref(), Some("CentOS 8 x64"));
        assert_eq!(bm.cpu_count, Some(4));
        assert_eq!(bm.region.as_deref(), Some("ewr"));
        assert_eq!(bm.status.as_deref(), Some("active"));
        assert_eq!(bm.tags.len(), 2);
    }

    #[test]
    fn test_bare_metal_ipv4_deserialize() {
        let json = r#"{
            "ip": "192.0.2.123",
            "netmask": "255.255.254.0",
            "gateway": "192.0.2.1",
            "type": "main_ip",
            "reverse": "server.example.com"
        }"#;

        let ip: BareMetalIpv4 = serde_json::from_str(json).unwrap();
        assert_eq!(ip.ip, "192.0.2.123");
        assert_eq!(ip.netmask.as_deref(), Some("255.255.254.0"));
        assert_eq!(ip.ip_type.as_deref(), Some("main_ip"));
    }

    #[test]
    fn test_bare_metal_ipv6_deserialize() {
        let json = r#"{
            "ip": "2001:0db8:1000::100",
            "network": "2001:0db8:1000::",
            "network_size": 64,
            "type": "main_ip"
        }"#;

        let ip: BareMetalIpv6 = serde_json::from_str(json).unwrap();
        assert_eq!(ip.ip, "2001:0db8:1000::100");
        assert_eq!(ip.network.as_deref(), Some("2001:0db8:1000::"));
        assert_eq!(ip.network_size, Some(64));
    }

    #[test]
    fn test_create_bare_metal_request_serialize() {
        let req = CreateBareMetalRequest {
            region: "ewr".to_string(),
            plan: "vbm-4c-32gb".to_string(),
            os_id: Some(215),
            label: Some("Test Server".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"region\":\"ewr\""));
        assert!(json.contains("\"plan\":\"vbm-4c-32gb\""));
        assert!(json.contains("\"os_id\":215"));
        // Optional fields not set should not be in output
        assert!(!json.contains("snapshot_id"));
    }

    #[test]
    fn test_update_bare_metal_request_serialize() {
        let req = UpdateBareMetalRequest {
            label: Some("Updated Label".to_string()),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            ..Default::default()
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"label\":\"Updated Label\""));
        assert!(json.contains("\"tags\":[\"tag1\",\"tag2\"]"));
    }

    #[test]
    fn test_bare_metal_vnc_deserialize() {
        let json = r#"{"url": "https://example.com/vnc"}"#;
        let vnc: BareMetalVnc = serde_json::from_str(json).unwrap();
        assert_eq!(vnc.url, "https://example.com/vnc");
    }

    #[test]
    fn test_bare_metal_upgrades_deserialize() {
        let json = r#"{
            "os": [
                {"id": 215, "name": "CentOS 8 x64", "arch": "x64", "family": "centos"}
            ],
            "applications": [
                {"id": 1, "name": "WordPress", "short_name": "wordpress"}
            ]
        }"#;

        let upgrades: BareMetalUpgrades = serde_json::from_str(json).unwrap();
        assert_eq!(upgrades.os.len(), 1);
        assert_eq!(upgrades.os[0].id, 215);
        assert_eq!(upgrades.applications.len(), 1);
        assert_eq!(upgrades.applications[0].name, "WordPress");
    }

    #[test]
    fn test_bulk_bare_metal_request_serialize() {
        let req = BulkBareMetalRequest {
            baremetal_ids: vec!["id1".to_string(), "id2".to_string()],
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"baremetal_ids\":[\"id1\",\"id2\"]"));
    }
}
