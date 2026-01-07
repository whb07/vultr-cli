//! DNS model types

use serde::{Deserialize, Serialize};

/// DNS Domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsDomain {
    /// Your registered domain name
    pub domain: String,
    /// Date the DNS Domain was created
    pub date_created: Option<String>,
    /// The domain's DNSSEC status (enabled/disabled)
    pub dns_sec: Option<String>,
}

/// DNS SOA Record information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsSoa {
    /// Primary nameserver for this domain
    pub nsprimary: Option<String>,
    /// Domain contact email address
    pub email: Option<String>,
}

/// DNS Record information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    /// A unique ID for the DNS Record
    pub id: String,
    /// The DNS record type (A, AAAA, CNAME, NS, MX, SRV, TXT, CAA, SSHFP)
    #[serde(rename = "type")]
    pub record_type: Option<String>,
    /// The hostname for this DNS record
    pub name: Option<String>,
    /// The DNS data for this record type
    pub data: Option<String>,
    /// DNS priority. Does not apply to all record types
    pub priority: Option<i32>,
    /// Time to Live in seconds
    pub ttl: Option<i32>,
}

/// DNSSEC information (array of DNSKEY and DS records as strings)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsSec {
    /// Array of DNSKEY and DS record strings
    pub dns_sec: Vec<String>,
}

/// Request to create a DNS domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDomainRequest {
    /// Your registered DNS Domain name
    pub domain: String,
    /// The default IP address for your DNS Domain (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Enable or disable DNSSEC (enabled/disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_sec: Option<String>,
}

/// Request to update a DNS domain
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateDomainRequest {
    /// Enable or disable DNSSEC (enabled/disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_sec: Option<String>,
}

/// Request to create a DNS record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecordRequest {
    /// The hostname for this DNS record
    pub name: String,
    /// The DNS record type (A, AAAA, CNAME, NS, MX, SRV, TXT, CAA, SSHFP)
    #[serde(rename = "type")]
    pub record_type: String,
    /// The DNS data for this record type
    pub data: String,
    /// Time to Live in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// DNS priority. Only required for MX and SRV
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// Request to update a DNS record
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateRecordRequest {
    /// The hostname for this DNS record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The DNS data for this record type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Time to Live in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// DNS priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// Request to update SOA information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSoaRequest {
    /// Set the primary nameserver
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsprimary: Option<String>,
    /// Set the contact email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Response wrapper for domain operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainResponse {
    pub domain: DnsDomain,
}

/// Response wrapper for record operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordResponse {
    pub record: DnsRecord,
}

/// Response wrapper for SOA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoaResponse {
    pub dns_soa: DnsSoa,
}

/// Response wrapper for domain list with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainsResponseWithMeta {
    pub domains: Vec<DnsDomain>,
    pub meta: crate::models::Meta,
}

/// Response wrapper for record list with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordsResponseWithMeta {
    pub records: Vec<DnsRecord>,
    pub meta: crate::models::Meta,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_domain_deserialize() {
        let json = r#"{"domain":"example.com","date_created":"2024-01-01","dns_sec":"enabled"}"#;
        let domain: DnsDomain = serde_json::from_str(json).unwrap();
        assert_eq!(domain.domain, "example.com");
        assert_eq!(domain.date_created.unwrap(), "2024-01-01");
        assert_eq!(domain.dns_sec.unwrap(), "enabled");
    }

    #[test]
    fn test_dns_domain_deserialize_minimal() {
        let json = r#"{"domain":"example.com"}"#;
        let domain: DnsDomain = serde_json::from_str(json).unwrap();
        assert_eq!(domain.domain, "example.com");
        assert!(domain.date_created.is_none());
        assert!(domain.dns_sec.is_none());
    }

    #[test]
    fn test_dns_soa_deserialize() {
        let json = r#"{"nsprimary":"ns1.vultr.com","email":"admin@example.com"}"#;
        let soa: DnsSoa = serde_json::from_str(json).unwrap();
        assert_eq!(soa.nsprimary.unwrap(), "ns1.vultr.com");
        assert_eq!(soa.email.unwrap(), "admin@example.com");
    }

    #[test]
    fn test_dns_record_deserialize() {
        let json = r#"{"id":"cb676a46-66fd-4dfb-b839-443f2e6c0b60","type":"A","name":"www","data":"192.0.2.123","priority":0,"ttl":300}"#;
        let record: DnsRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "cb676a46-66fd-4dfb-b839-443f2e6c0b60");
        assert_eq!(record.record_type.unwrap(), "A");
        assert_eq!(record.name.unwrap(), "www");
        assert_eq!(record.data.unwrap(), "192.0.2.123");
        assert_eq!(record.priority.unwrap(), 0);
        assert_eq!(record.ttl.unwrap(), 300);
    }

    #[test]
    fn test_dns_record_deserialize_minimal() {
        let json = r#"{"id":"rec-123"}"#;
        let record: DnsRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "rec-123");
        assert!(record.record_type.is_none());
    }

    #[test]
    fn test_create_domain_request_serialize() {
        let req = CreateDomainRequest {
            domain: "example.com".to_string(),
            ip: Some("192.0.2.123".to_string()),
            dns_sec: Some("enabled".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("example.com"));
        assert!(json.contains("192.0.2.123"));
        assert!(json.contains("enabled"));
    }

    #[test]
    fn test_create_domain_request_minimal() {
        let req = CreateDomainRequest {
            domain: "example.com".to_string(),
            ip: None,
            dns_sec: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("example.com"));
        assert!(!json.contains("ip"));
        assert!(!json.contains("dns_sec"));
    }

    #[test]
    fn test_update_domain_request_default() {
        let req = UpdateDomainRequest::default();
        assert!(req.dns_sec.is_none());
    }

    #[test]
    fn test_create_record_request_serialize() {
        let req = CreateRecordRequest {
            name: "www".to_string(),
            record_type: "A".to_string(),
            data: "192.0.2.123".to_string(),
            ttl: Some(300),
            priority: Some(0),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("www"));
        assert!(json.contains("\"type\":\"A\""));
        assert!(json.contains("192.0.2.123"));
        assert!(json.contains("300"));
    }

    #[test]
    fn test_update_record_request_partial() {
        let req = UpdateRecordRequest {
            name: None,
            data: Some("192.0.2.100".to_string()),
            ttl: Some(600),
            priority: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("name"));
        assert!(json.contains("192.0.2.100"));
        assert!(json.contains("600"));
        assert!(!json.contains("priority"));
    }

    #[test]
    fn test_update_soa_request_serialize() {
        let req = UpdateSoaRequest {
            nsprimary: Some("ns1.vultr.com".to_string()),
            email: Some("admin@example.com".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("ns1.vultr.com"));
        assert!(json.contains("admin@example.com"));
    }

    #[test]
    fn test_domain_response_deserialize() {
        let json = r#"{"domain":{"domain":"example.com","date_created":"2024-01-01","dns_sec":"enabled"}}"#;
        let response: DomainResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.domain.domain, "example.com");
    }

    #[test]
    fn test_domains_response_deserialize() {
        let json = r#"{"domains":[{"domain":"example.com"},{"domain":"test.com"}],"meta":{"total":2,"links":{"next":"","prev":""}}}"#;
        let response: DomainsResponseWithMeta = serde_json::from_str(json).unwrap();
        assert_eq!(response.domains.len(), 2);
        assert_eq!(response.domains[0].domain, "example.com");
        assert_eq!(response.domains[1].domain, "test.com");
    }

    #[test]
    fn test_domains_response_empty() {
        let json = r#"{"domains":[],"meta":{"total":0,"links":{"next":"","prev":""}}}"#;
        let response: DomainsResponseWithMeta = serde_json::from_str(json).unwrap();
        assert!(response.domains.is_empty());
    }

    #[test]
    fn test_record_response_deserialize() {
        let json = r#"{"record":{"id":"rec-123","type":"A","name":"www","data":"192.0.2.1"}}"#;
        let response: RecordResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.record.id, "rec-123");
    }

    #[test]
    fn test_records_response_deserialize() {
        let json = r#"{"records":[{"id":"rec-1"},{"id":"rec-2"}],"meta":{"total":2,"links":{"next":"","prev":""}}}"#;
        let response: RecordsResponseWithMeta = serde_json::from_str(json).unwrap();
        assert_eq!(response.records.len(), 2);
    }

    #[test]
    fn test_soa_response_deserialize() {
        let json = r#"{"dns_soa":{"nsprimary":"ns1.vultr.com","email":"admin@example.com"}}"#;
        let response: SoaResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.dns_soa.nsprimary.unwrap(), "ns1.vultr.com");
    }

    #[test]
    fn test_dnssec_response_deserialize() {
        let json = r#"{"dns_sec":["example.com IN DNSKEY 257 3 13 kRrxANp7YTGq..."]}"#;
        let response: DnsSec = serde_json::from_str(json).unwrap();
        assert_eq!(response.dns_sec.len(), 1);
        assert!(response.dns_sec[0].contains("DNSKEY"));
    }
}
