//! Backup model types

use serde::{Deserialize, Serialize};

/// Backup status
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BackupStatus {
    Pending,
    Complete,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for BackupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupStatus::Pending => write!(f, "pending"),
            BackupStatus::Complete => write!(f, "complete"),
            BackupStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// Backup
#[derive(Serialize, Deserialize)]
pub struct Backup {
    /// Unique ID for the backup
    pub id: String,
    /// Date the backup was created
    pub date_created: Option<String>,
    /// User-supplied description
    pub description: Option<String>,
    /// Backup size in bytes
    pub size: Option<i64>,
    /// Current status
    pub status: Option<BackupStatus>,
    /// Operating system ID
    pub os_id: Option<i32>,
    /// Application ID
    pub app_id: Option<i32>,
}

impl Backup {
    /// Check if the backup is complete and ready for use
    pub fn is_ready(&self) -> bool {
        self.status == Some(BackupStatus::Complete)
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

/// Response wrapper for backup operations
#[derive(Serialize, Deserialize)]
pub struct BackupResponse {
    pub backup: Backup,
}

/// Response wrapper for backup list
#[derive(Serialize, Deserialize)]
pub struct BackupsResponse {
    pub backups: Vec<Backup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_status_display() {
        assert_eq!(format!("{}", BackupStatus::Pending), "pending");
        assert_eq!(format!("{}", BackupStatus::Complete), "complete");
        assert_eq!(format!("{}", BackupStatus::Unknown), "unknown");
    }

    #[test]
    fn test_backup_is_ready_complete() {
        let backup = Backup {
            id: "backup-123".to_string(),
            status: Some(BackupStatus::Complete),
            date_created: None,
            description: None,
            size: None,
            os_id: None,
            app_id: None,
        };
        assert!(backup.is_ready());
    }

    #[test]
    fn test_backup_is_ready_pending() {
        let backup = Backup {
            id: "backup-123".to_string(),
            status: Some(BackupStatus::Pending),
            date_created: None,
            description: None,
            size: None,
            os_id: None,
            app_id: None,
        };
        assert!(!backup.is_ready());
    }

    #[test]
    fn test_backup_size_human_gb() {
        let backup = Backup {
            id: "backup-123".to_string(),
            status: None,
            date_created: None,
            description: None,
            size: Some(5_368_709_120), // 5 GB
            os_id: None,
            app_id: None,
        };
        assert_eq!(backup.size_human(), "5.00 GB");
    }

    #[test]
    fn test_backup_size_human_mb() {
        let backup = Backup {
            id: "backup-123".to_string(),
            status: None,
            date_created: None,
            description: None,
            size: Some(104_857_600), // 100 MB
            os_id: None,
            app_id: None,
        };
        assert_eq!(backup.size_human(), "100.00 MB");
    }

    #[test]
    fn test_backup_size_human_kb() {
        let backup = Backup {
            id: "backup-123".to_string(),
            status: None,
            date_created: None,
            description: None,
            size: Some(51200), // 50 KB
            os_id: None,
            app_id: None,
        };
        assert_eq!(backup.size_human(), "50.00 KB");
    }

    #[test]
    fn test_backup_size_human_bytes() {
        let backup = Backup {
            id: "backup-123".to_string(),
            status: None,
            date_created: None,
            description: None,
            size: Some(512),
            os_id: None,
            app_id: None,
        };
        assert_eq!(backup.size_human(), "512 bytes");
    }

    #[test]
    fn test_backup_size_human_unknown() {
        let backup = Backup {
            id: "backup-123".to_string(),
            status: None,
            date_created: None,
            description: None,
            size: None,
            os_id: None,
            app_id: None,
        };
        assert_eq!(backup.size_human(), "Unknown");
    }

    #[test]
    fn test_backup_deserialize() {
        let json = r#"{"id":"backup-abc","date_created":"2024-01-01","description":"Test backup","size":1073741824,"status":"complete","os_id":215}"#;
        let backup: Backup = serde_json::from_str(json).unwrap();
        assert_eq!(backup.id, "backup-abc");
        assert_eq!(backup.description.unwrap(), "Test backup");
        assert_eq!(backup.status.unwrap(), BackupStatus::Complete);
    }

    #[test]
    fn test_backup_status_unknown_variant() {
        let json = r#""processing""#;
        let status: BackupStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, BackupStatus::Unknown);
    }

    #[test]
    fn test_backup_response_deserialize() {
        let json = r#"{"backup":{"id":"backup-123","date_created":"2024-01-01","description":"Daily backup","size":10000000,"status":"complete"}}"#;
        let response: BackupResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.backup.id, "backup-123");
    }

    #[test]
    fn test_backups_response_deserialize() {
        let json = r#"{"backups":[{"id":"backup-1","status":"complete"},{"id":"backup-2","status":"pending"}]}"#;
        let response: BackupsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.backups.len(), 2);
        assert_eq!(response.backups[0].id, "backup-1");
        assert_eq!(response.backups[1].id, "backup-2");
    }
}
