//! Snapshot model types

use serde::{Deserialize, Serialize};

/// Snapshot status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnapshotRequest {
    /// Instance ID to snapshot
    pub instance_id: String,
    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to create a snapshot from URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnapshotFromUrlRequest {
    /// URL of the raw image
    pub url: String,
    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to update a snapshot
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSnapshotRequest {
    /// New description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response wrapper for snapshot operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotResponse {
    pub snapshot: Snapshot,
}

/// Response wrapper for snapshot list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotsResponse {
    pub snapshots: Vec<Snapshot>,
}
