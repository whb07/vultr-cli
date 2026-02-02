//! ISO model types

use serde::{Deserialize, Serialize};

/// ISO resource status (different from instance ISO attachment status)
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IsoResourceStatus {
    Pending,
    Complete,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for IsoResourceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IsoResourceStatus::Pending => write!(f, "pending"),
            IsoResourceStatus::Complete => write!(f, "complete"),
            IsoResourceStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// ISO information
#[derive(Serialize, Deserialize)]
pub struct Iso {
    /// Unique ID for the ISO
    pub id: String,
    /// Date the ISO was created
    pub date_created: Option<String>,
    /// Filename of the ISO
    pub filename: Option<String>,
    /// ISO size in bytes
    pub size: Option<i64>,
    /// MD5 checksum
    pub md5sum: Option<String>,
    /// SHA512 checksum
    pub sha512sum: Option<String>,
    /// Current status
    pub status: Option<IsoResourceStatus>,
}

impl Iso {
    /// Check if the ISO is complete and ready for use
    pub fn is_ready(&self) -> bool {
        self.status == Some(IsoResourceStatus::Complete)
    }

    /// Get the size in human-readable format
    pub fn size_human(&self) -> String {
        match self.size {
            Some(bytes) if bytes >= 1_073_741_824 => {
                format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
            }
            Some(bytes) if bytes >= 1_048_576 => {
                format!("{:.2} MB", bytes as f64 / 1_048_576.0)
            }
            Some(bytes) if bytes >= 1024 => {
                format!("{:.2} KB", bytes as f64 / 1024.0)
            }
            Some(bytes) => format!("{} bytes", bytes),
            None => "Unknown".to_string(),
        }
    }
}

/// Public ISO information
#[derive(Serialize, Deserialize)]
pub struct PublicIso {
    /// Unique ID for the public ISO
    pub id: String,
    /// Name of the ISO
    pub name: Option<String>,
    /// Description
    pub description: Option<String>,
    /// MD5 checksum
    pub md5sum: Option<String>,
}

/// Request to create an ISO
#[derive(Serialize, Deserialize)]
pub struct CreateIsoRequest {
    /// Public URL location of the ISO image to download
    pub url: String,
}

/// Response wrapper for ISO operations
#[derive(Serialize, Deserialize)]
pub struct IsoResponse {
    pub iso: Iso,
}

/// Response wrapper for ISO list
#[derive(Serialize, Deserialize)]
pub struct IsosResponse {
    pub isos: Vec<Iso>,
}

/// Response wrapper for public ISO list
#[derive(Serialize, Deserialize)]
pub struct PublicIsosResponse {
    pub public_isos: Vec<PublicIso>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_status_display() {
        assert_eq!(format!("{}", IsoResourceStatus::Pending), "pending");
        assert_eq!(format!("{}", IsoResourceStatus::Complete), "complete");
        assert_eq!(format!("{}", IsoResourceStatus::Unknown), "unknown");
    }

    #[test]
    fn test_iso_is_ready_complete() {
        let iso = Iso {
            id: "iso-123".to_string(),
            status: Some(IsoResourceStatus::Complete),
            date_created: None,
            filename: None,
            size: None,
            md5sum: None,
            sha512sum: None,
        };
        assert!(iso.is_ready());
    }

    #[test]
    fn test_iso_is_ready_pending() {
        let iso = Iso {
            id: "iso-123".to_string(),
            status: Some(IsoResourceStatus::Pending),
            date_created: None,
            filename: None,
            size: None,
            md5sum: None,
            sha512sum: None,
        };
        assert!(!iso.is_ready());
    }

    #[test]
    fn test_iso_size_human_gb() {
        let iso = Iso {
            id: "iso-123".to_string(),
            status: None,
            date_created: None,
            filename: None,
            size: Some(5_368_709_120), // 5 GB
            md5sum: None,
            sha512sum: None,
        };
        assert_eq!(iso.size_human(), "5.00 GB");
    }

    #[test]
    fn test_iso_size_human_mb() {
        let iso = Iso {
            id: "iso-123".to_string(),
            status: None,
            date_created: None,
            filename: None,
            size: Some(104_857_600), // 100 MB
            md5sum: None,
            sha512sum: None,
        };
        assert_eq!(iso.size_human(), "100.00 MB");
    }

    #[test]
    fn test_iso_size_human_kb() {
        let iso = Iso {
            id: "iso-123".to_string(),
            status: None,
            date_created: None,
            filename: None,
            size: Some(51200), // 50 KB
            md5sum: None,
            sha512sum: None,
        };
        assert_eq!(iso.size_human(), "50.00 KB");
    }

    #[test]
    fn test_iso_size_human_bytes() {
        let iso = Iso {
            id: "iso-123".to_string(),
            status: None,
            date_created: None,
            filename: None,
            size: Some(512),
            md5sum: None,
            sha512sum: None,
        };
        assert_eq!(iso.size_human(), "512 bytes");
    }

    #[test]
    fn test_iso_size_human_unknown() {
        let iso = Iso {
            id: "iso-123".to_string(),
            status: None,
            date_created: None,
            filename: None,
            size: None,
            md5sum: None,
            sha512sum: None,
        };
        assert_eq!(iso.size_human(), "Unknown");
    }

    #[test]
    fn test_iso_deserialize() {
        let json = r#"{"id":"iso-abc","date_created":"2024-01-01","filename":"my-iso.iso","size":120586240,"md5sum":"77ba289bdc966ec996278a5a740d96d8","status":"complete"}"#;
        let iso: Iso = serde_json::from_str(json).unwrap();
        assert_eq!(iso.id, "iso-abc");
        assert_eq!(iso.filename.unwrap(), "my-iso.iso");
        assert_eq!(iso.status.unwrap(), IsoResourceStatus::Complete);
    }

    #[test]
    fn test_iso_status_unknown_variant() {
        let json = r#""processing""#;
        let status: IsoResourceStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, IsoResourceStatus::Unknown);
    }

    #[test]
    fn test_public_iso_deserialize() {
        let json = r#"{"id":"pub-iso-123","name":"CentOS 7","description":"7 x86_64 Minimal","md5sum":"7f4df50f42ee1b52b193e79855a3aa19"}"#;
        let iso: PublicIso = serde_json::from_str(json).unwrap();
        assert_eq!(iso.id, "pub-iso-123");
        assert_eq!(iso.name.unwrap(), "CentOS 7");
        assert_eq!(iso.description.unwrap(), "7 x86_64 Minimal");
    }

    #[test]
    fn test_create_iso_request_serialize() {
        let req = CreateIsoRequest {
            url: "https://example.com/my-iso.iso".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("https://example.com/my-iso.iso"));
    }
}
