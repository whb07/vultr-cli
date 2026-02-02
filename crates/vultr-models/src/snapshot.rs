//! Snapshot model types

use serde::{Deserialize, Serialize};

/// Snapshot status
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotStatus {
    Pending,
    Complete,
    Deleted,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for SnapshotStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnapshotStatus::Pending => write!(f, "pending"),
            SnapshotStatus::Complete => write!(f, "complete"),
            SnapshotStatus::Deleted => write!(f, "deleted"),
            SnapshotStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// Snapshot
#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    /// Unique ID for the snapshot
    pub id: String,
    /// Date the snapshot was created
    pub date_created: Option<String>,
    /// User-supplied description
    pub description: Option<String>,
    /// Snapshot size in bytes
    pub size: Option<i64>,
    /// Current status
    pub status: Option<SnapshotStatus>,
    /// Operating system ID
    pub os_id: Option<i32>,
    /// Application ID
    pub app_id: Option<i32>,
}

impl Snapshot {
    /// Check if the snapshot is complete and ready for use
    pub fn is_ready(&self) -> bool {
        self.status == Some(SnapshotStatus::Complete)
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

/// Request to create a snapshot
#[derive(Serialize, Deserialize)]
pub struct CreateSnapshotRequest {
    /// Instance ID to snapshot
    pub instance_id: String,
    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to create a snapshot from URL
#[derive(Serialize, Deserialize)]
pub struct CreateSnapshotFromUrlRequest {
    /// URL of the raw image
    pub url: String,
    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to update a snapshot
#[derive(Serialize, Deserialize)]
pub struct UpdateSnapshotRequest {
    /// New description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response wrapper for snapshot operations
#[derive(Serialize, Deserialize)]
pub struct SnapshotResponse {
    pub snapshot: Snapshot,
}

/// Response wrapper for snapshot list
#[derive(Serialize, Deserialize)]
pub struct SnapshotsResponse {
    pub snapshots: Vec<Snapshot>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_status_display() {
        assert_eq!(format!("{}", SnapshotStatus::Pending), "pending");
        assert_eq!(format!("{}", SnapshotStatus::Complete), "complete");
        assert_eq!(format!("{}", SnapshotStatus::Deleted), "deleted");
        assert_eq!(format!("{}", SnapshotStatus::Unknown), "unknown");
    }

    #[test]
    fn test_snapshot_is_ready_complete() {
        let snapshot = Snapshot {
            id: "snap-123".to_string(),
            status: Some(SnapshotStatus::Complete),
            date_created: None,
            description: None,
            size: None,
            os_id: None,
            app_id: None,
        };
        assert!(snapshot.is_ready());
    }

    #[test]
    fn test_snapshot_is_ready_pending() {
        let snapshot = Snapshot {
            id: "snap-123".to_string(),
            status: Some(SnapshotStatus::Pending),
            date_created: None,
            description: None,
            size: None,
            os_id: None,
            app_id: None,
        };
        assert!(!snapshot.is_ready());
    }

    #[test]
    fn test_snapshot_size_human_gb() {
        let snapshot = Snapshot {
            id: "snap-123".to_string(),
            status: None,
            date_created: None,
            description: None,
            size: Some(5_368_709_120), // 5 GB
            os_id: None,
            app_id: None,
        };
        assert_eq!(snapshot.size_human(), "5.00 GB");
    }

    #[test]
    fn test_snapshot_size_human_mb() {
        let snapshot = Snapshot {
            id: "snap-123".to_string(),
            status: None,
            date_created: None,
            description: None,
            size: Some(104_857_600), // 100 MB
            os_id: None,
            app_id: None,
        };
        assert_eq!(snapshot.size_human(), "100.00 MB");
    }

    #[test]
    fn test_snapshot_size_human_kb() {
        let snapshot = Snapshot {
            id: "snap-123".to_string(),
            status: None,
            date_created: None,
            description: None,
            size: Some(51200), // 50 KB
            os_id: None,
            app_id: None,
        };
        assert_eq!(snapshot.size_human(), "50.00 KB");
    }

    #[test]
    fn test_snapshot_size_human_bytes() {
        let snapshot = Snapshot {
            id: "snap-123".to_string(),
            status: None,
            date_created: None,
            description: None,
            size: Some(512),
            os_id: None,
            app_id: None,
        };
        assert_eq!(snapshot.size_human(), "512 bytes");
    }

    #[test]
    fn test_snapshot_size_human_unknown() {
        let snapshot = Snapshot {
            id: "snap-123".to_string(),
            status: None,
            date_created: None,
            description: None,
            size: None,
            os_id: None,
            app_id: None,
        };
        assert_eq!(snapshot.size_human(), "Unknown");
    }

    #[test]
    fn test_snapshot_deserialize() {
        let json = r#"{"id":"snap-abc","date_created":"2024-01-01","description":"Test snapshot","size":1073741824,"status":"complete","os_id":215}"#;
        let snapshot: Snapshot = serde_json::from_str(json).unwrap();
        assert_eq!(snapshot.id, "snap-abc");
        assert_eq!(snapshot.description.unwrap(), "Test snapshot");
        assert_eq!(snapshot.status.unwrap(), SnapshotStatus::Complete);
    }

    #[test]
    fn test_snapshot_status_unknown_variant() {
        let json = r#""processing""#;
        let status: SnapshotStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, SnapshotStatus::Unknown);
    }

    #[test]
    fn test_create_snapshot_request_serialize() {
        let req = CreateSnapshotRequest {
            instance_id: "inst-123".to_string(),
            description: Some("My snapshot".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("inst-123"));
        assert!(json.contains("My snapshot"));
    }
}
