//! Load Balancer models

use serde::{Deserialize, Serialize};

/// Load Balancer information
#[derive(Serialize, Deserialize)]
pub struct LoadBalancer {
    /// A unique ID for the Load Balancer
    pub id: String,

    /// Date this Load Balancer was created
    #[serde(default)]
    pub date_created: Option<String>,

    /// The region where the Load Balancer is located
    #[serde(default)]
    pub region: Option<String>,

    /// The user-supplied label for this Load Balancer
    #[serde(default)]
    pub label: Option<String>,

    /// The current status (e.g., "active", "pending")
    #[serde(default)]
    pub status: Option<String>,

    /// The IPv4 address of this Load Balancer
    #[serde(default)]
    pub ipv4: Option<String>,

    /// The IPv6 address of this Load Balancer
    #[serde(default)]
    pub ipv6: Option<String>,

    /// Generic configuration options
    #[serde(default)]
    pub generic_info: Option<GenericInfo>,

    /// Health check configuration
    #[serde(default)]
    pub health_check: Option<HealthCheck>,

    /// Indicates if this Load Balancer has an SSL certificate installed
    #[serde(default)]
    pub has_ssl: Option<bool>,

    /// Indicates if HTTP2 is enabled
    #[serde(default)]
    pub http2: Option<bool>,

    /// Indicates if HTTP3/QUIC is enabled
    #[serde(default)]
    pub http3: Option<bool>,

    /// Number of nodes in the load balancer (1-99, must be odd)
    #[serde(default)]
    pub nodes: Option<i32>,

    /// Array of forwarding rules (API returns as "forwarding_rules")
    #[serde(default, alias = "forward_rules")]
    pub forwarding_rules: Vec<ForwardingRule>,

    /// Array of firewall rules
    #[serde(default)]
    pub firewall_rules: Vec<LBFirewallRule>,

    /// Array of instance IDs attached to this Load Balancer
    #[serde(default)]
    pub instances: Vec<String>,

    /// Node IP addresses
    #[serde(default)]
    pub node_ips: Option<NodeIps>,

    /// Auto SSL configuration
    #[serde(default)]
    pub auto_ssl: Option<AutoSSL>,

    /// If this LB is a child of a global LB, this is the parent ID
    #[serde(default)]
    pub global_parent_id: Option<String>,

    /// Array of region IDs for child Load Balancers
    #[serde(default)]
    pub global_regions: Vec<String>,

    /// Array of child load balancer IDs (if this is a global LB parent)
    #[serde(default)]
    pub global_children_ids: Vec<String>,

    /// Base64 encoded SSL certificate, private key, and chain
    #[serde(default)]
    pub ssl_cert_b64: Option<String>,

    /// Pending charges
    #[serde(default)]
    pub pending_charges: Option<f64>,
}

/// Generic configuration options for a Load Balancer
#[derive(Serialize, Deserialize)]
pub struct GenericInfo {
    /// The balancing algorithm (roundrobin, leastconn)
    #[serde(default)]
    pub balancing_algorithm: Option<String>,

    /// If true, redirect all HTTP to HTTPS
    #[serde(default)]
    pub ssl_redirect: Option<bool>,

    /// Sticky session configuration
    #[serde(default)]
    pub sticky_sessions: Option<StickySessions>,

    /// If true, backend nodes must accept Proxy protocol
    #[serde(default)]
    pub proxy_protocol: Option<bool>,

    /// Connection timeout in seconds
    #[serde(default)]
    pub timeout: Option<i32>,

    /// Deprecated: Use vpc instead
    #[serde(default)]
    pub private_network: Option<String>,

    /// VPC ID
    #[serde(default)]
    pub vpc: Option<String>,
}

/// Sticky session configuration
#[derive(Serialize, Deserialize)]
pub struct StickySessions {
    /// The cookie name for sticky sessions
    #[serde(default)]
    pub cookie_name: Option<String>,
}

/// Health check configuration for a Load Balancer
#[derive(Serialize, Deserialize)]
pub struct HealthCheck {
    /// The protocol for health checks (HTTP, HTTPS, TCP)
    #[serde(default)]
    pub protocol: Option<String>,

    /// The port for health checks
    #[serde(default)]
    pub port: Option<i32>,

    /// HTTP path to check (only for HTTP/HTTPS)
    #[serde(default)]
    pub path: Option<String>,

    /// Interval between health checks (seconds)
    #[serde(default)]
    pub check_interval: Option<i32>,

    /// Timeout before health check fails (seconds)
    #[serde(default)]
    pub response_timeout: Option<i32>,

    /// Number of failures before marking unhealthy
    #[serde(default)]
    pub unhealthy_threshold: Option<i32>,

    /// Number of successes before marking healthy
    #[serde(default)]
    pub healthy_threshold: Option<i32>,
}

/// Forwarding rule for a Load Balancer
#[derive(Serialize, Deserialize)]
pub struct ForwardingRule {
    /// A unique ID for the forwarding rule
    #[serde(default)]
    pub id: Option<String>,

    /// The frontend protocol (HTTP, HTTPS, TCP)
    #[serde(default)]
    pub frontend_protocol: Option<String>,

    /// The frontend port number
    #[serde(default)]
    pub frontend_port: Option<i32>,

    /// The backend protocol (HTTP, HTTPS, TCP)
    #[serde(default)]
    pub backend_protocol: Option<String>,

    /// The backend port number
    #[serde(default)]
    pub backend_port: Option<i32>,
}

/// Firewall rule for a Load Balancer
#[derive(Serialize, Deserialize)]
pub struct LBFirewallRule {
    /// A unique ID for the firewall rule
    #[serde(default)]
    pub id: Option<String>,

    /// Port for this rule
    #[serde(default)]
    pub port: Option<i32>,

    /// Source IP/CIDR or "cloudflare"
    #[serde(default)]
    pub source: Option<String>,

    /// IP type (v4 or v6)
    #[serde(default)]
    pub ip_type: Option<String>,
}

/// Node IP addresses for a Load Balancer
#[derive(Serialize, Deserialize)]
pub struct NodeIps {
    /// IPv4 addresses of the load balancer nodes
    #[serde(default)]
    pub v4: Vec<String>,

    /// IPv6 addresses of the load balancer nodes
    #[serde(default)]
    pub v6: Vec<String>,
}

/// Auto SSL configuration
#[derive(Serialize, Deserialize)]
pub struct AutoSSL {
    /// The domain zone (example.com)
    #[serde(default)]
    pub domain_zone: Option<String>,

    /// Full domain including subdomain (subdomain.example.com)
    #[serde(default)]
    pub domain: Option<String>,

    /// Subdomain to append to the domain zone (for create/update)
    #[serde(default)]
    pub domain_sub: Option<String>,
}

/// SSL certificate configuration for requests
#[derive(Serialize, Deserialize)]
pub struct SSLConfig {
    /// The private key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,

    /// The SSL certificate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    /// The certificate chain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,

    /// The private key base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_b64: Option<String>,

    /// The SSL certificate base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_b64: Option<String>,

    /// The certificate chain base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_b64: Option<String>,
}

/// Global region configuration for multi-region load balancers
#[derive(Serialize, Deserialize)]
pub struct GlobalRegion {
    /// The region ID
    pub region_id: String,

    /// Optional VPC ID for this region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// Reverse DNS information
#[derive(Serialize, Deserialize)]
pub struct ReverseDNS {
    /// IPv4 reverse DNS
    #[serde(default)]
    pub ipv4: Option<String>,

    /// IPv6 reverse DNS entries
    #[serde(default)]
    pub ipv6: Vec<String>,
}

/// IPv6 Reverse DNS entry for create/update
#[derive(Serialize, Deserialize)]
pub struct ReverseIPv6Entry {
    /// The IPv6 IP address
    pub ip: String,

    /// The domain for the reverse DNS
    pub domain: String,
}

// =====================
// Request Types
// =====================

/// Request to create a new Load Balancer
#[derive(Serialize, Deserialize, Default)]
pub struct CreateLoadBalancerRequest {
    /// The region ID to create this Load Balancer in
    pub region: String,

    /// The balancing algorithm (roundrobin, leastconn)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_algorithm: Option<String>,

    /// If true, redirect all HTTP to HTTPS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_redirect: Option<bool>,

    /// If true, enable HTTP2 traffic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2: Option<bool>,

    /// If true, enable HTTP3/QUIC traffic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http3: Option<bool>,

    /// Number of nodes (1-99, must be odd)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<i32>,

    /// If true, backend nodes must accept Proxy protocol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol: Option<bool>,

    /// Connection timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,

    /// Health check configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,

    /// Forwarding rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_rules: Option<Vec<CreateForwardingRuleRequest>>,

    /// Sticky session configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<StickySessions>,

    /// SSL certificate configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<SSLConfig>,

    /// Label for the Load Balancer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Instance IDs to attach
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,

    /// Firewall rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rules: Option<Vec<CreateLBFirewallRuleRequest>>,

    /// VPC ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,

    /// Auto SSL configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ssl: Option<AutoSSL>,

    /// Global regions for multi-region load balancers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_regions: Option<Vec<GlobalRegion>>,
}

/// Request to update a Load Balancer
#[derive(Serialize, Deserialize, Default)]
pub struct UpdateLoadBalancerRequest {
    /// SSL certificate configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<SSLConfig>,

    /// Sticky session configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_session: Option<StickySessions>,

    /// Forwarding rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_rules: Option<Vec<CreateForwardingRuleRequest>>,

    /// Health check configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,

    /// If true, backend nodes must accept Proxy protocol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol: Option<bool>,

    /// Connection timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,

    /// If true, redirect all HTTP to HTTPS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_redirect: Option<bool>,

    /// If true, enable HTTP2 traffic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2: Option<bool>,

    /// If true, enable HTTP3/QUIC traffic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http3: Option<bool>,

    /// Number of nodes (1-99, must be odd)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<i32>,

    /// The balancing algorithm (roundrobin, leastconn)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_algorithm: Option<String>,

    /// Instance IDs to attach (send complete array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,

    /// Label for the Load Balancer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// VPC ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,

    /// Firewall rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rules: Option<Vec<CreateLBFirewallRuleRequest>>,

    /// Auto SSL configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ssl: Option<AutoSSL>,

    /// Global regions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_regions: Option<Vec<String>>,
}

/// Request to create a forwarding rule
#[derive(Serialize, Deserialize)]
pub struct CreateForwardingRuleRequest {
    /// The frontend protocol (HTTP, HTTPS, TCP)
    pub frontend_protocol: String,

    /// The frontend port number
    pub frontend_port: i32,

    /// The backend protocol (HTTP, HTTPS, TCP)
    pub backend_protocol: String,

    /// The backend port number
    pub backend_port: i32,
}

/// Request to create a firewall rule for a Load Balancer
#[derive(Serialize, Deserialize)]
pub struct CreateLBFirewallRuleRequest {
    /// Port for this rule
    pub port: i32,

    /// Source IP/CIDR or "cloudflare"
    pub source: String,

    /// IP type (v4 or v6)
    pub ip_type: String,
}

/// Request to update IPv4 reverse DNS
#[derive(Serialize, Deserialize)]
pub struct UpdateReverseDNSv4Request {
    /// The domain for reverse DNS
    pub v4: String,
}

/// Request to create IPv6 reverse DNS entries
#[derive(Serialize, Deserialize)]
pub struct CreateReverseDNSv6Request {
    /// Array of IPv6 reverse DNS entries
    pub v6: Vec<ReverseIPv6Entry>,
}

// =====================
// Response Types
// =====================

/// Response containing a single Load Balancer
#[derive(Serialize, Deserialize)]
pub struct LoadBalancerResponse {
    pub load_balancer: LoadBalancer,
}

/// Response containing a list of Load Balancers
#[derive(Serialize, Deserialize)]
pub struct LoadBalancersResponse {
    pub load_balancers: Vec<LoadBalancer>,
}

/// Response containing a single forwarding rule
#[derive(Serialize, Deserialize)]
pub struct ForwardingRuleResponse {
    pub forwarding_rule: ForwardingRule,
}

/// Response containing a list of forwarding rules
#[derive(Serialize, Deserialize)]
pub struct ForwardingRulesResponse {
    pub forwarding_rules: Vec<ForwardingRule>,
}

/// Response containing a single firewall rule
#[derive(Serialize, Deserialize)]
pub struct LBFirewallRuleResponse {
    pub firewall_rule: LBFirewallRule,
}

/// Response containing a list of firewall rules
#[derive(Serialize, Deserialize)]
pub struct LBFirewallRulesResponse {
    pub firewall_rules: Vec<LBFirewallRule>,
}

// =====================
// Unit Tests
// =====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_balancer_deserialize() {
        let json = r#"{
            "id": "cb676a46-66fd-4dfb-b839-443f2e6c0b60",
            "date_created": "2020-10-10T01:56:20+00:00",
            "region": "ewr",
            "label": "Example Load Balancer",
            "status": "active",
            "ipv4": "192.0.2.123",
            "ipv6": "2001:0db8:5:4973:ffff:ffff:ffff:ffff",
            "has_ssl": false,
            "http2": false,
            "http3": false,
            "nodes": 1,
            "forwarding_rules": [],
            "firewall_rules": [],
            "instances": []
        }"#;

        let lb: LoadBalancer = serde_json::from_str(json).unwrap();
        assert_eq!(lb.id, "cb676a46-66fd-4dfb-b839-443f2e6c0b60");
        assert_eq!(lb.label, Some("Example Load Balancer".to_string()));
        assert_eq!(lb.status, Some("active".to_string()));
        assert_eq!(lb.region, Some("ewr".to_string()));
    }

    #[test]
    fn test_load_balancer_with_generic_info() {
        let json = r#"{
            "id": "test-lb",
            "generic_info": {
                "balancing_algorithm": "roundrobin",
                "ssl_redirect": false,
                "proxy_protocol": false,
                "timeout": 600,
                "vpc": "vpc-123"
            },
            "forwarding_rules": [],
            "firewall_rules": [],
            "instances": []
        }"#;

        let lb: LoadBalancer = serde_json::from_str(json).unwrap();
        let info = lb.generic_info.unwrap();
        assert_eq!(info.balancing_algorithm, Some("roundrobin".to_string()));
        assert_eq!(info.timeout, Some(600));
        assert_eq!(info.vpc, Some("vpc-123".to_string()));
    }

    #[test]
    fn test_health_check_deserialize() {
        let json = r#"{
            "protocol": "http",
            "port": 80,
            "path": "/health",
            "check_interval": 10,
            "response_timeout": 5,
            "unhealthy_threshold": 3,
            "healthy_threshold": 3
        }"#;

        let hc: HealthCheck = serde_json::from_str(json).unwrap();
        assert_eq!(hc.protocol, Some("http".to_string()));
        assert_eq!(hc.port, Some(80));
        assert_eq!(hc.path, Some("/health".to_string()));
        assert_eq!(hc.check_interval, Some(10));
    }

    #[test]
    fn test_forwarding_rule_deserialize() {
        let json = r#"{
            "id": "73d85156c2c3129d",
            "frontend_protocol": "http",
            "frontend_port": 80,
            "backend_protocol": "http",
            "backend_port": 80
        }"#;

        let rule: ForwardingRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.id, Some("73d85156c2c3129d".to_string()));
        assert_eq!(rule.frontend_protocol, Some("http".to_string()));
        assert_eq!(rule.frontend_port, Some(80));
    }

    #[test]
    fn test_lb_firewall_rule_deserialize() {
        let json = r#"{
            "id": "abcd123b93016eafb",
            "port": 80,
            "source": "24.187.16.196/16",
            "ip_type": "v4"
        }"#;

        let rule: LBFirewallRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.id, Some("abcd123b93016eafb".to_string()));
        assert_eq!(rule.port, Some(80));
        assert_eq!(rule.source, Some("24.187.16.196/16".to_string()));
        assert_eq!(rule.ip_type, Some("v4".to_string()));
    }

    #[test]
    fn test_create_load_balancer_request_serialize() {
        let request = CreateLoadBalancerRequest {
            region: "ewr".to_string(),
            label: Some("Test LB".to_string()),
            balancing_algorithm: Some("roundrobin".to_string()),
            nodes: Some(1),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"region\":\"ewr\""));
        assert!(json.contains("\"label\":\"Test LB\""));
        assert!(json.contains("\"balancing_algorithm\":\"roundrobin\""));
    }

    #[test]
    fn test_create_forwarding_rule_request_serialize() {
        let request = CreateForwardingRuleRequest {
            frontend_protocol: "http".to_string(),
            frontend_port: 80,
            backend_protocol: "http".to_string(),
            backend_port: 8080,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"frontend_protocol\":\"http\""));
        assert!(json.contains("\"frontend_port\":80"));
        assert!(json.contains("\"backend_port\":8080"));
    }

    #[test]
    fn test_load_balancer_response_deserialize() {
        let json = r#"{
            "load_balancer": {
                "id": "test-id",
                "status": "active",
                "forwarding_rules": [],
                "firewall_rules": [],
                "instances": []
            }
        }"#;

        let resp: LoadBalancerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.load_balancer.id, "test-id");
    }

    #[test]
    fn test_load_balancers_response_deserialize() {
        let json = r#"{
            "load_balancers": [
                {
                    "id": "lb-1",
                    "forwarding_rules": [],
                    "firewall_rules": [],
                    "instances": []
                },
                {
                    "id": "lb-2",
                    "forwarding_rules": [],
                    "firewall_rules": [],
                    "instances": []
                }
            ]
        }"#;

        let resp: LoadBalancersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.load_balancers.len(), 2);
        assert_eq!(resp.load_balancers[0].id, "lb-1");
    }
}
