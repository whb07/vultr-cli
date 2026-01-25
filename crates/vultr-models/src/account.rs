//! Account model types

use serde::{Deserialize, Serialize};

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account name
    pub name: Option<String>,
    /// Account email
    pub email: Option<String>,
    /// Account balance
    pub balance: Option<f64>,
    /// Pending charges
    pub pending_charges: Option<f64>,
    /// Last payment date
    pub last_payment_date: Option<String>,
    /// Last payment amount
    pub last_payment_amount: Option<f64>,
    /// Account ACLs (access control list)
    #[serde(default)]
    pub acls: Vec<String>,
}

/// Response wrapper for account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountResponse {
    pub account: Account,
}

/// BGP information for the account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BgpInfo {
    /// BGP enabled status
    pub enabled: Option<bool>,
    /// BGP ASN
    pub asn: Option<i64>,
    /// IPv4 prefixes
    #[serde(default)]
    pub allowed_prefix_ipv4: Vec<BgpPrefix>,
    /// IPv6 prefixes
    #[serde(default)]
    pub allowed_prefix_ipv6: Vec<BgpPrefix>,
}

/// BGP prefix information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BgpPrefix {
    /// IP prefix
    pub prefix: Option<String>,
    /// Description
    pub description: Option<String>,
}

/// Response wrapper for BGP info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BgpResponse {
    pub bgp_info: BgpInfo,
}

/// Account bandwidth information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBandwidth {
    /// Current month bandwidth usage
    pub current_month_to_date: Option<BandwidthUsage>,
    /// Previous month bandwidth usage
    pub previous_month: Option<BandwidthUsage>,
}

/// Bandwidth usage details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthUsage {
    /// Timestamp
    pub timestamp: Option<String>,
    /// Incoming bytes
    pub incoming_bytes: Option<i64>,
    /// Outgoing bytes
    pub outgoing_bytes: Option<i64>,
    /// Total GB used
    pub gb_total: Option<f64>,
    /// Free tier GB
    pub gb_free: Option<f64>,
    /// Overage GB
    pub gb_overage: Option<f64>,
    /// Overage cost
    pub overage_cost: Option<f64>,
}

/// Response wrapper for account bandwidth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBandwidthResponse {
    pub bandwidth: AccountBandwidth,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_deserialize() {
        let json = r#"{"name":"Test Account","email":"test@example.com","balance":100.50,"pending_charges":10.25,"acls":["manage_users","billing"]}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.name.unwrap(), "Test Account");
        assert_eq!(account.email.unwrap(), "test@example.com");
        assert_eq!(account.balance.unwrap(), 100.50);
        assert_eq!(account.acls.len(), 2);
    }

    #[test]
    fn test_bgp_info_deserialize() {
        let json =
            r#"{"enabled":true,"asn":65000,"allowed_prefix_ipv4":[],"allowed_prefix_ipv6":[]}"#;
        let bgp: BgpInfo = serde_json::from_str(json).unwrap();
        assert!(bgp.enabled.unwrap());
        assert_eq!(bgp.asn.unwrap(), 65000);
    }

    #[test]
    fn test_account_bandwidth_deserialize() {
        let json =
            r#"{"current_month_to_date":{"gb_total":100.5},"previous_month":{"gb_total":200.0}}"#;
        let bw: AccountBandwidth = serde_json::from_str(json).unwrap();
        assert_eq!(bw.current_month_to_date.unwrap().gb_total.unwrap(), 100.5);
    }
}
