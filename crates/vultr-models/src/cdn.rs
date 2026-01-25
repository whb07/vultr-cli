//! CDN model types

use serde::{Deserialize, Serialize};

/// CDN Zone status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CdnZoneStatus {
    Active,
    Pending,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for CdnZoneStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CdnZoneStatus::Active => write!(f, "active"),
            CdnZoneStatus::Pending => write!(f, "pending"),
            CdnZoneStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// CDN origin scheme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OriginScheme {
    Http,
    Https,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for OriginScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OriginScheme::Http => write!(f, "http"),
            OriginScheme::Https => write!(f, "https"),
            OriginScheme::Unknown => write!(f, "unknown"),
        }
    }
}

/// CDN Pull Zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnPullZone {
    /// Unique ID for the CDN Pull Zone
    pub id: String,
    /// Date the CDN Pull Zone was created
    pub date_created: Option<String>,
    /// Current status (active, pending)
    pub status: Option<CdnZoneStatus>,
    /// User-supplied label
    pub label: Option<String>,
    /// URI scheme of the origin domain (http, https)
    pub origin_scheme: Option<OriginScheme>,
    /// Domain name from which content is pulled
    pub origin_domain: Option<String>,
    /// Vultr CDN endpoint URL
    pub cdn_url: Option<String>,
    /// Custom vanity domain
    pub vanity_domain: Option<String>,
    /// Cache size in bytes
    pub cache_size: Option<i64>,
    /// Number of requests
    pub requests: Option<i64>,
    /// Inbound bytes
    pub in_bytes: Option<i64>,
    /// Outbound bytes
    pub out_bytes: Option<i64>,
    /// Rate limit in packets per second
    pub packets_per_sec: Option<i64>,
    /// Last purge date
    pub last_purge: Option<String>,
    /// Cross-origin resource sharing enabled
    pub cors: Option<bool>,
    /// Gzip compression enabled
    pub gzip: Option<bool>,
    /// Block AI bots
    pub block_ai: Option<bool>,
    /// Block bad bots
    pub block_bad_bots: Option<bool>,
    /// List of region IDs
    #[serde(default)]
    pub regions: Vec<String>,
}

impl CdnPullZone {
    /// Check if the pull zone is active
    pub fn is_active(&self) -> bool {
        self.status == Some(CdnZoneStatus::Active)
    }
}

/// CDN Push Zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnPushZone {
    /// Unique ID for the CDN Push Zone
    pub id: String,
    /// Date the CDN Push Zone was created
    pub date_created: Option<String>,
    /// Current status (active, pending)
    pub status: Option<CdnZoneStatus>,
    /// User-supplied label
    pub label: Option<String>,
    /// Vultr CDN endpoint URL
    pub cdn_url: Option<String>,
    /// Custom vanity domain
    pub vanity_domain: Option<String>,
    /// Cache size in bytes
    pub cache_size: Option<i64>,
    /// Number of requests
    pub requests: Option<i64>,
    /// Inbound bytes
    pub in_bytes: Option<i64>,
    /// Outbound bytes
    pub out_bytes: Option<i64>,
    /// Rate limit in packets per second
    pub packets_per_sec: Option<i64>,
    /// Cross-origin resource sharing enabled
    pub cors: Option<bool>,
    /// Gzip compression enabled
    pub gzip: Option<bool>,
    /// Block AI bots
    pub block_ai: Option<bool>,
    /// Block bad bots
    pub block_bad_bots: Option<bool>,
    /// List of region IDs
    #[serde(default)]
    pub regions: Vec<String>,
}

impl CdnPushZone {
    /// Check if the push zone is active
    pub fn is_active(&self) -> bool {
        self.status == Some(CdnZoneStatus::Active)
    }
}

/// CDN Push Zone File
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnPushZoneFile {
    /// File name
    pub name: Option<String>,
    /// MIME type
    pub mime: Option<String>,
    /// File size
    pub size: Option<String>,
    /// Base64 encoded file content
    pub content: Option<String>,
    /// Last modified date
    pub last_modified: Option<String>,
}

/// CDN Push Zone File Meta (for list operations)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnPushZoneFileMeta {
    /// File name
    pub name: Option<String>,
    /// File size
    pub size: Option<String>,
    /// Last modified date
    pub last_modified: Option<String>,
}

/// CDN Upload Endpoint inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnUploadEndpointInputs {
    /// Access control list
    pub acl: Option<String>,
    /// Upload key
    pub key: Option<String>,
    /// AWS credential
    #[serde(rename = "X-Amz-Credential")]
    pub x_amz_credential: Option<String>,
    /// AWS algorithm
    #[serde(rename = "X-Amz-Algorithm")]
    pub x_amz_algorithm: Option<String>,
    /// Encrypted policy
    #[serde(rename = "Policy")]
    pub policy: Option<String>,
    /// Request signature
    #[serde(rename = "X-Amz-Signature")]
    pub x_amz_signature: Option<String>,
    /// AWS date
    #[serde(rename = "X-Amz-Date")]
    pub x_amz_date: Option<String>,
}

/// CDN Upload Endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnUploadEndpoint {
    /// Upload URL
    #[serde(rename = "URL")]
    pub url: Option<String>,
    /// Upload inputs/credentials
    pub inputs: Option<CdnUploadEndpointInputs>,
}

// =====================
// Request types
// =====================

/// Request to create a CDN Pull Zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePullZoneRequest {
    /// Label for the pull zone
    pub label: String,
    /// Origin domain to pull content from
    pub origin_domain: String,
    /// Origin scheme (http or https)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_scheme: Option<String>,
    /// Custom vanity domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanity_domain: Option<String>,
    /// Enable SSL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cert: Option<String>,
    /// SSL private key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cert_key: Option<String>,
    /// Enable CORS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<bool>,
    /// Enable gzip compression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<bool>,
    /// Block AI bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ai: Option<bool>,
    /// Block bad bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_bad_bots: Option<bool>,
    /// List of region IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

/// Request to update a CDN Pull Zone
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePullZoneRequest {
    /// New label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Origin domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_domain: Option<String>,
    /// Origin scheme
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_scheme: Option<String>,
    /// Custom vanity domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanity_domain: Option<String>,
    /// SSL certificate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cert: Option<String>,
    /// SSL private key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cert_key: Option<String>,
    /// Enable CORS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<bool>,
    /// Enable gzip compression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<bool>,
    /// Block AI bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ai: Option<bool>,
    /// Block bad bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_bad_bots: Option<bool>,
    /// List of region IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

/// Request to create a CDN Push Zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePushZoneRequest {
    /// Label for the push zone
    pub label: String,
    /// Custom vanity domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanity_domain: Option<String>,
    /// SSL certificate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cert: Option<String>,
    /// SSL private key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cert_key: Option<String>,
    /// Enable CORS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<bool>,
    /// Enable gzip compression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<bool>,
    /// Block AI bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ai: Option<bool>,
    /// Block bad bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_bad_bots: Option<bool>,
    /// List of region IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

/// Request to update a CDN Push Zone
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePushZoneRequest {
    /// New label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Custom vanity domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanity_domain: Option<String>,
    /// SSL certificate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cert: Option<String>,
    /// SSL private key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cert_key: Option<String>,
    /// Enable CORS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<bool>,
    /// Enable gzip compression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gzip: Option<bool>,
    /// Block AI bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ai: Option<bool>,
    /// Block bad bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_bad_bots: Option<bool>,
    /// List of region IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

/// Request to create a file upload endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileEndpointRequest {
    /// File name
    pub name: String,
    /// File size in bytes
    pub size: i64,
}

// =====================
// Response types
// =====================

/// Response wrapper for pull zone operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullZoneResponse {
    pub pull_zone: CdnPullZone,
}

/// Response wrapper for pull zone list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullZonesResponse {
    pub pull_zones: Vec<CdnPullZone>,
}

/// Response wrapper for push zone operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushZoneResponse {
    pub push_zone: CdnPushZone,
}

/// Response wrapper for push zone list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushZonesResponse {
    pub push_zones: Vec<CdnPushZone>,
}

/// Response wrapper for push zone file operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushZoneFileResponse {
    pub file: CdnPushZoneFile,
}

/// Response wrapper for push zone file list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushZoneFilesResponse {
    pub files: Vec<CdnPushZoneFileMeta>,
}

/// Response wrapper for upload endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadEndpointResponse {
    pub upload_endpoint: CdnUploadEndpoint,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdn_zone_status_display() {
        assert_eq!(format!("{}", CdnZoneStatus::Active), "active");
        assert_eq!(format!("{}", CdnZoneStatus::Pending), "pending");
        assert_eq!(format!("{}", CdnZoneStatus::Unknown), "unknown");
    }

    #[test]
    fn test_origin_scheme_display() {
        assert_eq!(format!("{}", OriginScheme::Http), "http");
        assert_eq!(format!("{}", OriginScheme::Https), "https");
        assert_eq!(format!("{}", OriginScheme::Unknown), "unknown");
    }

    #[test]
    fn test_pull_zone_is_active() {
        let zone = CdnPullZone {
            id: "zone-123".to_string(),
            status: Some(CdnZoneStatus::Active),
            date_created: None,
            label: None,
            origin_scheme: None,
            origin_domain: None,
            cdn_url: None,
            vanity_domain: None,
            cache_size: None,
            requests: None,
            in_bytes: None,
            out_bytes: None,
            packets_per_sec: None,
            last_purge: None,
            cors: None,
            gzip: None,
            block_ai: None,
            block_bad_bots: None,
            regions: vec![],
        };
        assert!(zone.is_active());
    }

    #[test]
    fn test_pull_zone_is_not_active() {
        let zone = CdnPullZone {
            id: "zone-123".to_string(),
            status: Some(CdnZoneStatus::Pending),
            date_created: None,
            label: None,
            origin_scheme: None,
            origin_domain: None,
            cdn_url: None,
            vanity_domain: None,
            cache_size: None,
            requests: None,
            in_bytes: None,
            out_bytes: None,
            packets_per_sec: None,
            last_purge: None,
            cors: None,
            gzip: None,
            block_ai: None,
            block_bad_bots: None,
            regions: vec![],
        };
        assert!(!zone.is_active());
    }

    #[test]
    fn test_push_zone_is_active() {
        let zone = CdnPushZone {
            id: "zone-456".to_string(),
            status: Some(CdnZoneStatus::Active),
            date_created: None,
            label: None,
            cdn_url: None,
            vanity_domain: None,
            cache_size: None,
            requests: None,
            in_bytes: None,
            out_bytes: None,
            packets_per_sec: None,
            cors: None,
            gzip: None,
            block_ai: None,
            block_bad_bots: None,
            regions: vec![],
        };
        assert!(zone.is_active());
    }

    #[test]
    fn test_pull_zone_deserialize() {
        let json = r#"{
            "id": "zone-abc",
            "date_created": "2024-01-01",
            "status": "active",
            "label": "My CDN",
            "origin_scheme": "https",
            "origin_domain": "example.com",
            "cdn_url": "https://cdn-xyz.vultrcdn.com",
            "cors": true,
            "gzip": true,
            "regions": ["ewr", "lax"]
        }"#;
        let zone: CdnPullZone = serde_json::from_str(json).unwrap();
        assert_eq!(zone.id, "zone-abc");
        assert_eq!(zone.label.unwrap(), "My CDN");
        assert_eq!(zone.origin_scheme.unwrap(), OriginScheme::Https);
        assert!(zone.cors.unwrap());
        assert_eq!(zone.regions.len(), 2);
    }

    #[test]
    fn test_push_zone_deserialize() {
        let json = r#"{
            "id": "zone-def",
            "date_created": "2024-02-01",
            "status": "pending",
            "label": "Push CDN",
            "cdn_url": "https://cdn-push.vultrcdn.com",
            "gzip": false
        }"#;
        let zone: CdnPushZone = serde_json::from_str(json).unwrap();
        assert_eq!(zone.id, "zone-def");
        assert_eq!(zone.status.unwrap(), CdnZoneStatus::Pending);
        assert!(!zone.gzip.unwrap());
    }

    #[test]
    fn test_push_zone_file_deserialize() {
        let json = r#"{
            "name": "image.jpg",
            "mime": "image/jpeg",
            "size": "1024",
            "last_modified": "2024-04-18T13:07:15+00:00"
        }"#;
        let file: CdnPushZoneFile = serde_json::from_str(json).unwrap();
        assert_eq!(file.name.unwrap(), "image.jpg");
        assert_eq!(file.mime.unwrap(), "image/jpeg");
    }

    #[test]
    fn test_create_pull_zone_request_serialize() {
        let req = CreatePullZoneRequest {
            label: "my-cdn".to_string(),
            origin_domain: "example.com".to_string(),
            origin_scheme: Some("https".to_string()),
            vanity_domain: None,
            ssl_cert: None,
            ssl_cert_key: None,
            cors: Some(true),
            gzip: Some(true),
            block_ai: None,
            block_bad_bots: None,
            regions: Some(vec!["ewr".to_string()]),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("my-cdn"));
        assert!(json.contains("example.com"));
        assert!(json.contains("\"cors\":true"));
    }

    #[test]
    fn test_create_push_zone_request_serialize() {
        let req = CreatePushZoneRequest {
            label: "push-cdn".to_string(),
            vanity_domain: Some("cdn.example.com".to_string()),
            ssl_cert: None,
            ssl_cert_key: None,
            cors: Some(false),
            gzip: Some(true),
            block_ai: Some(true),
            block_bad_bots: Some(true),
            regions: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("push-cdn"));
        assert!(json.contains("cdn.example.com"));
        assert!(json.contains("\"block_ai\":true"));
    }

    #[test]
    fn test_upload_endpoint_deserialize() {
        let json = r#"{
            "URL": "https://cdn.s3-ewr-000.vultr.dev/v-cdn-agent-assets",
            "inputs": {
                "acl": "public-read",
                "key": "cdn-test.vultrcdn.com/file.jpg",
                "X-Amz-Credential": "creds",
                "X-Amz-Algorithm": "AWS4-HMAC-SHA256",
                "Policy": "base64policy",
                "X-Amz-Signature": "signature"
            }
        }"#;
        let endpoint: CdnUploadEndpoint = serde_json::from_str(json).unwrap();
        assert!(endpoint.url.is_some());
        assert!(endpoint.inputs.is_some());
        let inputs = endpoint.inputs.unwrap();
        assert_eq!(inputs.acl.unwrap(), "public-read");
    }

    #[test]
    fn test_cdn_zone_status_unknown_variant() {
        let json = r#""processing""#;
        let status: CdnZoneStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, CdnZoneStatus::Unknown);
    }
}
