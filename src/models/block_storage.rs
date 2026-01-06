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
        self.attached_to_instance.is_some() && !self.attached_to_instance.as_ref().unwrap().is_empty()
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
