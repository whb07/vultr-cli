//! Vultr File System model types

use serde::{Deserialize, Serialize};

/// VFS region pricing details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsRegionPrice {
    pub nvme: Option<f64>,
    pub hdd: Option<f64>,
}

/// VFS region minimum size details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsRegionMinSize {
    pub nvme: Option<i32>,
    pub hdd: Option<i32>,
}

/// VFS region information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsRegion {
    pub id: Option<String>,
    pub country: Option<String>,
    pub continent: Option<String>,
    pub description: Option<String>,
    pub price_per_gb: Option<VfsRegionPrice>,
    pub min_size_gb: Option<VfsRegionMinSize>,
}

/// VFS storage size
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsStorageSize {
    pub bytes: Option<i64>,
    pub gb: Option<i32>,
}

/// VFS billing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsBilling {
    pub charges: Option<f64>,
    pub monthly: Option<f64>,
}

/// VFS subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vfs {
    pub id: Option<String>,
    pub region: Option<String>,
    pub date_created: Option<String>,
    pub status: Option<String>,
    pub label: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub disk_type: Option<String>,
    pub storage_size: Option<VfsStorageSize>,
    pub storage_used: Option<VfsStorageSize>,
    pub billing: Option<VfsBilling>,
}

/// VFS attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsAttachment {
    pub state: Option<String>,
    pub vfs_id: Option<String>,
    pub target_id: Option<String>,
    pub mount_tag: Option<i32>,
}

/// Response wrapper for VFS list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsListResponse {
    pub vfs: Vec<Vfs>,
}

/// Response wrapper for VFS regions list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsRegionsResponse {
    pub regions: Vec<VfsRegion>,
}

/// Response wrapper for VFS attachments list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsAttachmentsResponse {
    pub attachments: Vec<VfsAttachment>,
}

/// Request to create a VFS subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVfsRequest {
    pub region: String,
    pub label: String,
    pub storage_size: VfsStorageSizeRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Request to update a VFS subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVfsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<VfsStorageSizeRequest>,
}

/// Storage size request (GB)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsStorageSizeRequest {
    pub gb: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfs_deserialize() {
        let json = r#"{
            "id": "vfs-1",
            "region": "ewr",
            "status": "active",
            "label": "storage",
            "disk_type": "nvme",
            "storage_size": {"gb": 100},
            "storage_used": {"gb": 20}
        }"#;
        let vfs: Vfs = serde_json::from_str(json).unwrap();
        assert_eq!(vfs.id.as_deref(), Some("vfs-1"));
        assert_eq!(vfs.storage_size.unwrap().gb, Some(100));
    }
}
