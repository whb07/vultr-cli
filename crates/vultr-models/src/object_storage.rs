//! Object Storage model types

use serde::{Deserialize, Serialize};

/// Object Storage status
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectStorageStatus {
    Active,
    Pending,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for ObjectStorageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectStorageStatus::Active => write!(f, "active"),
            ObjectStorageStatus::Pending => write!(f, "pending"),
            ObjectStorageStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// Tier information embedded in Object Storage response
#[derive(Serialize, Deserialize)]
pub struct ObjectStorageTierInfo {
    /// Tier ID
    #[serde(rename = "OBJSTORETIERID")]
    pub tier_id: Option<i32>,
    /// Price per additional gigabyte of bandwidth
    pub bw_gb_price: Option<f64>,
    /// Price per additional gigabyte of capacity
    pub disk_gb_price: Option<f64>,
    /// Is this tier the default
    pub is_default: Option<String>,
    /// Monthly price for this tier
    pub price: Option<f64>,
    /// Rate limit on the number of bytes per second
    pub ratelimit_ops_bytes: Option<i64>,
    /// Rate limit on the number of operations per second
    pub ratelimit_ops_secs: Option<i64>,
    /// Sales description
    pub sales_desc: Option<String>,
    /// Sales name
    pub sales_name: Option<String>,
    /// Slug, unique name
    pub slug: Option<String>,
}

/// Object Storage
#[derive(Serialize, Deserialize)]
pub struct ObjectStorage {
    /// Unique ID for the Object Storage
    pub id: String,
    /// Date the Object Store was created
    pub date_created: Option<String>,
    /// The Cluster ID
    pub cluster_id: Option<i32>,
    /// The Region ID for this Object Storage
    pub region: Option<String>,
    /// The user-supplied label for this Object Storage
    pub label: Option<String>,
    /// The status of this Object Storage
    pub status: Option<ObjectStorageStatus>,
    /// The Cluster hostname for this Object Storage
    pub s3_hostname: Option<String>,
    /// The Object Storage access key
    pub s3_access_key: Option<String>,
    /// The Object Storage secret key
    pub s3_secret_key: Option<String>,
    /// Additional tier info for this object storage
    pub tier: Option<ObjectStorageTierInfo>,
}

/// Object Storage Cluster
#[derive(Serialize, Deserialize)]
pub struct ObjectStorageCluster {
    /// A unique ID for the Object Storage cluster
    pub id: i32,
    /// The Region ID where the cluster is located
    pub region: Option<String>,
    /// The cluster host name
    pub hostname: Option<String>,
    /// The Cluster is eligible for Object Storage deployment (yes/no)
    pub deploy: Option<String>,
}

/// Tier location information
#[derive(Serialize, Deserialize)]
pub struct TierLocation {
    /// Cluster hostname
    pub hostname: Option<String>,
    /// Cluster ID
    pub id: Option<i32>,
    /// Cluster name
    pub name: Option<String>,
    /// Cluster region
    pub region: Option<String>,
}

/// Object Storage Tier
#[derive(Serialize, Deserialize)]
pub struct ObjectStorageTier {
    /// Object Storage Tier ID
    pub id: i32,
    /// Price per additional gigabyte of bandwidth
    pub bw_gb_price: Option<f64>,
    /// Price per additional gigabyte of capacity
    pub disk_gb_price: Option<f64>,
    /// Is this tier the default
    pub is_default: Option<String>,
    /// Monthly price for this tier
    pub price: Option<f64>,
    /// Rate limit on the number of bytes per second
    pub ratelimit_ops_bytes: Option<i64>,
    /// Rate limit on the number of operations per second
    pub ratelimit_ops_secs: Option<i64>,
    /// Sales description
    pub sales_desc: Option<String>,
    /// Sales name
    pub sales_name: Option<String>,
    /// Slug, unique name
    pub slug: Option<String>,
    /// Clusters where the tier is available
    #[serde(default)]
    pub locations: Vec<TierLocation>,
}

/// Cluster-specific tier (no locations field)
#[derive(Serialize, Deserialize)]
pub struct ClusterTier {
    /// Object Storage Tier ID
    pub id: i32,
    /// Price per additional gigabyte of bandwidth
    pub bw_gb_price: Option<f64>,
    /// Price per additional gigabyte of capacity
    pub disk_gb_price: Option<f64>,
    /// Is this tier the default
    pub is_default: Option<String>,
    /// Monthly price for this tier
    pub price: Option<f64>,
    /// Rate limit on the number of bytes per second
    pub ratelimit_ops_bytes: Option<i64>,
    /// Rate limit on the number of operations per second
    pub ratelimit_ops_secs: Option<i64>,
    /// Sales description
    pub sales_desc: Option<String>,
    /// Sales name
    pub sales_name: Option<String>,
    /// Slug, unique name
    pub slug: Option<String>,
}

/// S3 Credentials (returned from regenerate-keys)
#[derive(Serialize, Deserialize)]
pub struct S3Credentials {
    /// The Cluster hostname for this Object Storage
    pub s3_hostname: Option<String>,
    /// The new Object Storage access key
    pub s3_access_key: Option<String>,
    /// The new Object Storage secret key
    pub s3_secret_key: Option<String>,
}

/// Request to create Object Storage
#[derive(Serialize, Deserialize)]
pub struct CreateObjectStorageRequest {
    /// The Cluster ID where the Object Storage will be created
    pub cluster_id: i32,
    /// The Tier ID of the tier to set up for
    pub tier_id: i32,
    /// The user-supplied label for this Object Storage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Request to update Object Storage
#[derive(Serialize, Deserialize)]
pub struct UpdateObjectStorageRequest {
    /// The user-supplied label for this Object Storage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Response wrapper for single object storage
#[derive(Serialize, Deserialize)]
pub struct ObjectStorageResponse {
    pub object_storage: ObjectStorage,
}

/// Response wrapper for object storage list
#[derive(Serialize, Deserialize)]
pub struct ObjectStoragesResponse {
    pub object_storages: Vec<ObjectStorage>,
}

/// Response wrapper for object storage clusters list
#[derive(Serialize, Deserialize)]
pub struct ObjectStorageClustersResponse {
    pub clusters: Vec<ObjectStorageCluster>,
}

/// Response wrapper for tiers list
#[derive(Serialize, Deserialize)]
pub struct TiersResponse {
    pub tiers: Vec<ObjectStorageTier>,
}

/// Response wrapper for cluster tiers list
#[derive(Serialize, Deserialize)]
pub struct ClusterTiersResponse {
    pub tiers: Vec<ClusterTier>,
}

/// Response wrapper for regenerate keys
#[derive(Serialize, Deserialize)]
pub struct RegenerateKeysResponse {
    pub s3_credentials: S3Credentials,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_storage_status_display() {
        assert_eq!(format!("{}", ObjectStorageStatus::Active), "active");
        assert_eq!(format!("{}", ObjectStorageStatus::Pending), "pending");
        assert_eq!(format!("{}", ObjectStorageStatus::Unknown), "unknown");
    }

    #[test]
    fn test_object_storage_status_unknown_variant() {
        let json = r#""creating""#;
        let status: ObjectStorageStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, ObjectStorageStatus::Unknown);
    }

    #[test]
    fn test_object_storage_deserialize() {
        let json = r#"{
            "id": "cb676a46-66fd-4dfb-b839-443f2e6c0b60",
            "date_created": "2020-10-10T01:56:20+00:00",
            "cluster_id": 2,
            "region": "ewr",
            "label": "Example Object Storage",
            "status": "active",
            "s3_hostname": "ewr1.vultrobjects.com",
            "s3_access_key": "00example11223344",
            "s3_secret_key": "00example1122334455667788990011"
        }"#;
        let storage: ObjectStorage = serde_json::from_str(json).unwrap();
        assert_eq!(storage.id, "cb676a46-66fd-4dfb-b839-443f2e6c0b60");
        assert_eq!(storage.cluster_id, Some(2));
        assert_eq!(storage.region, Some("ewr".to_string()));
        assert_eq!(storage.status, Some(ObjectStorageStatus::Active));
    }

    #[test]
    fn test_object_storage_cluster_deserialize() {
        let json = r#"{
            "id": 2,
            "region": "ewr",
            "hostname": "ewr1.vultrobjects.com",
            "deploy": "yes"
        }"#;
        let cluster: ObjectStorageCluster = serde_json::from_str(json).unwrap();
        assert_eq!(cluster.id, 2);
        assert_eq!(cluster.region, Some("ewr".to_string()));
        assert_eq!(cluster.hostname, Some("ewr1.vultrobjects.com".to_string()));
        assert_eq!(cluster.deploy, Some("yes".to_string()));
    }

    #[test]
    fn test_object_storage_tier_deserialize() {
        let json = r#"{
            "id": 1,
            "bw_gb_price": 0.01,
            "disk_gb_price": 0.006,
            "is_default": "yes",
            "price": 6,
            "ratelimit_ops_bytes": 314572800,
            "ratelimit_ops_secs": 400,
            "sales_desc": "Vultr's historic object storage solution.",
            "sales_name": "Legacy",
            "slug": "tier_004h_0300m",
            "locations": [
                {
                    "hostname": "ewr1.vultrobjects.com",
                    "id": 2,
                    "name": "New Jersey",
                    "region": "ewr"
                }
            ]
        }"#;
        let tier: ObjectStorageTier = serde_json::from_str(json).unwrap();
        assert_eq!(tier.id, 1);
        assert_eq!(tier.is_default, Some("yes".to_string()));
        assert_eq!(tier.price, Some(6.0));
        assert_eq!(tier.locations.len(), 1);
        assert_eq!(tier.locations[0].region, Some("ewr".to_string()));
    }

    #[test]
    fn test_cluster_tier_deserialize() {
        let json = r#"{
            "id": 1,
            "bw_gb_price": 0.01,
            "disk_gb_price": 0.006,
            "is_default": "yes",
            "price": 6,
            "ratelimit_ops_bytes": 314572800,
            "ratelimit_ops_secs": 400,
            "sales_desc": "Vultr's historic object storage solution.",
            "sales_name": "Legacy",
            "slug": "tier_004h_0300m"
        }"#;
        let tier: ClusterTier = serde_json::from_str(json).unwrap();
        assert_eq!(tier.id, 1);
        assert_eq!(tier.sales_name, Some("Legacy".to_string()));
    }

    #[test]
    fn test_create_object_storage_request_serialize() {
        let req = CreateObjectStorageRequest {
            cluster_id: 2,
            tier_id: 4,
            label: Some("My Object Storage".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("cluster_id"));
        assert!(json.contains("tier_id"));
        assert!(json.contains("My Object Storage"));
    }

    #[test]
    fn test_update_object_storage_request_serialize() {
        let req = UpdateObjectStorageRequest {
            label: Some("Updated Label".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Updated Label"));
    }

    #[test]
    fn test_update_object_storage_request_skip_none() {
        let req = UpdateObjectStorageRequest { label: None };
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_s3_credentials_deserialize() {
        let json = r#"{
            "s3_hostname": "ewr1.vultrobjects.com",
            "s3_access_key": "00example11223344",
            "s3_secret_key": "00example1122334455667788990011"
        }"#;
        let creds: S3Credentials = serde_json::from_str(json).unwrap();
        assert_eq!(creds.s3_hostname, Some("ewr1.vultrobjects.com".to_string()));
        assert_eq!(creds.s3_access_key, Some("00example11223344".to_string()));
    }

    #[test]
    fn test_tier_location_deserialize() {
        let json = r#"{
            "hostname": "ewr1.vultrobjects.com",
            "id": 2,
            "name": "New Jersey",
            "region": "ewr"
        }"#;
        let loc: TierLocation = serde_json::from_str(json).unwrap();
        assert_eq!(loc.id, Some(2));
        assert_eq!(loc.name, Some("New Jersey".to_string()));
    }

    #[test]
    fn test_object_storage_tier_info_deserialize() {
        let json = r#"{
            "OBJSTORETIERID": 5,
            "bw_gb_price": 0.01,
            "disk_gb_price": 0.05,
            "is_default": "no",
            "price": 50,
            "ratelimit_ops_bytes": 1048576000,
            "ratelimit_ops_secs": 4000,
            "sales_desc": "Low-latency storage for datacenter workloads.",
            "sales_name": "Performance",
            "slug": "tier_004k_1000m"
        }"#;
        let tier: ObjectStorageTierInfo = serde_json::from_str(json).unwrap();
        assert_eq!(tier.tier_id, Some(5));
        assert_eq!(tier.sales_name, Some("Performance".to_string()));
    }
}
