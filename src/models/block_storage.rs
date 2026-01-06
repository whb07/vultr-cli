//! Block Storage model types

use serde::{Deserialize, Serialize};

/// Block storage status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockStorageStatus {
    Active,
    Pending,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for BlockStorageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockStorageStatus::Active => write!(f, "active"),
            BlockStorageStatus::Pending => write!(f, "pending"),
            BlockStorageStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// Block storage type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    HighPerf,
    StorageOpt,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockType::HighPerf => write!(f, "high_perf"),
            BlockType::StorageOpt => write!(f, "storage_opt"),
            BlockType::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for BlockType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high_perf" | "highperf" => Ok(BlockType::HighPerf),
            "storage_opt" | "storageopt" => Ok(BlockType::StorageOpt),
            _ => Err(format!("Unknown block type: {}", s)),
        }
    }
}

/// Block Storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStorage {
    /// Unique ID for the block storage
    pub id: String,
    /// Monthly cost in USD
    pub cost: Option<i32>,
    /// Current status
    pub status: Option<BlockStorageStatus>,
    /// Size in GB
    pub size_gb: Option<i32>,
    /// Region ID
    pub region: Option<String>,
    /// ID of the attached instance (if any)
    pub attached_to_instance: Option<String>,
    /// Date created
    pub date_created: Option<String>,
    /// User-supplied label
    pub label: Option<String>,
    /// Mount ID (for /dev/disk/by-id)
    pub mount_id: Option<String>,
    /// Block type
    pub block_type: Option<String>,
}

impl BlockStorage {
    /// Check if the block storage is attached to an instance
    pub fn is_attached(&self) -> bool {
        self.attached_to_instance.is_some()
            && !self.attached_to_instance.as_ref().unwrap().is_empty()
    }
}

/// Request to create block storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockStorageRequest {
    /// Region ID
    pub region: String,
    /// Size in GB (10-40000 depending on type)
    pub size_gb: i32,
    /// Optional label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Block type (high_perf or storage_opt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
}

/// Request to update block storage
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateBlockStorageRequest {
    /// New label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// New size in GB (can only increase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<i32>,
}

/// Request to attach block storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachBlockStorageRequest {
    /// Instance ID to attach to
    pub instance_id: String,
    /// Whether to live attach (without reboot)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
}

/// Request to detach block storage
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DetachBlockStorageRequest {
    /// Whether to live detach (without reboot)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
}

/// Response wrapper for block storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStorageResponse {
    pub block: BlockStorage,
}

/// Response wrapper for block storage list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStoragesResponse {
    pub blocks: Vec<BlockStorage>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_storage_status_display() {
        assert_eq!(format!("{}", BlockStorageStatus::Active), "active");
        assert_eq!(format!("{}", BlockStorageStatus::Pending), "pending");
        assert_eq!(format!("{}", BlockStorageStatus::Unknown), "unknown");
    }

    #[test]
    fn test_block_type_display() {
        assert_eq!(format!("{}", BlockType::HighPerf), "high_perf");
        assert_eq!(format!("{}", BlockType::StorageOpt), "storage_opt");
        assert_eq!(format!("{}", BlockType::Unknown), "unknown");
    }

    #[test]
    fn test_block_type_from_str_high_perf() {
        assert_eq!(
            "high_perf".parse::<BlockType>().unwrap(),
            BlockType::HighPerf
        );
        assert_eq!(
            "highperf".parse::<BlockType>().unwrap(),
            BlockType::HighPerf
        );
    }

    #[test]
    fn test_block_type_from_str_storage_opt() {
        assert_eq!(
            "storage_opt".parse::<BlockType>().unwrap(),
            BlockType::StorageOpt
        );
        assert_eq!(
            "storageopt".parse::<BlockType>().unwrap(),
            BlockType::StorageOpt
        );
    }

    #[test]
    fn test_block_type_from_str_invalid() {
        let result = "invalid".parse::<BlockType>();
        assert!(result.is_err());
    }

    #[test]
    fn test_block_storage_is_attached_true() {
        let block = BlockStorage {
            id: "block-123".to_string(),
            attached_to_instance: Some("inst-456".to_string()),
            cost: None,
            status: None,
            size_gb: None,
            region: None,
            date_created: None,
            label: None,
            mount_id: None,
            block_type: None,
        };
        assert!(block.is_attached());
    }

    #[test]
    fn test_block_storage_is_attached_false_none() {
        let block = BlockStorage {
            id: "block-123".to_string(),
            attached_to_instance: None,
            cost: None,
            status: None,
            size_gb: None,
            region: None,
            date_created: None,
            label: None,
            mount_id: None,
            block_type: None,
        };
        assert!(!block.is_attached());
    }

    #[test]
    fn test_block_storage_is_attached_false_empty() {
        let block = BlockStorage {
            id: "block-123".to_string(),
            attached_to_instance: Some("".to_string()),
            cost: None,
            status: None,
            size_gb: None,
            region: None,
            date_created: None,
            label: None,
            mount_id: None,
            block_type: None,
        };
        assert!(!block.is_attached());
    }

    #[test]
    fn test_block_storage_deserialize() {
        let json = r#"{"id":"block-abc","cost":10,"status":"active","size_gb":100,"region":"ewr","label":"My Storage"}"#;
        let block: BlockStorage = serde_json::from_str(json).unwrap();
        assert_eq!(block.id, "block-abc");
        assert_eq!(block.cost.unwrap(), 10);
        assert_eq!(block.status.unwrap(), BlockStorageStatus::Active);
        assert_eq!(block.size_gb.unwrap(), 100);
    }

    #[test]
    fn test_create_block_storage_request_serialize() {
        let req = CreateBlockStorageRequest {
            region: "ewr".to_string(),
            size_gb: 100,
            label: Some("Test Block".to_string()),
            block_type: Some("high_perf".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("ewr"));
        assert!(json.contains("100"));
        assert!(json.contains("Test Block"));
    }

    #[test]
    fn test_attach_block_storage_request_serialize() {
        let req = AttachBlockStorageRequest {
            instance_id: "inst-123".to_string(),
            live: Some(true),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("inst-123"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_detach_block_storage_request_default() {
        let req = DetachBlockStorageRequest::default();
        assert!(req.live.is_none());
    }

    #[test]
    fn test_block_storage_status_unknown_variant() {
        let json = r#""creating""#;
        let status: BlockStorageStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, BlockStorageStatus::Unknown);
    }
}
