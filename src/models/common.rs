//! Common model types shared across the API

use serde::{Deserialize, Serialize};

/// Pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Meta {
    /// Total number of objects available
    pub total: Option<i32>,
    /// Pagination links
    pub links: Option<PaginationLinks>,
}

/// Cursor-based pagination links
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationLinks {
    /// Cursor for the next page
    pub next: Option<String>,
    /// Cursor for the previous page  
    pub prev: Option<String>,
}

/// Generic list response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse<T> {
    #[serde(flatten)]
    pub data: T,
    #[serde(default)]
    pub meta: Meta,
}

/// Region information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    /// Unique ID for the region (e.g., "ewr", "lax")
    pub id: String,
    /// City name
    pub city: Option<String>,
    /// Two-letter country code
    pub country: Option<String>,
    /// Continent name
    pub continent: Option<String>,
    /// Available features in this region
    #[serde(default)]
    pub options: Vec<String>,
}

/// Operating System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Os {
    /// Operating System ID
    pub id: i32,
    /// OS name/description
    pub name: Option<String>,
    /// Architecture (x64, i386, etc.)
    pub arch: Option<String>,
    /// OS family (centos, ubuntu, windows, etc.)
    pub family: Option<String>,
}

/// Compute plan information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    /// Unique plan ID (e.g., "vc2-1c-1gb")
    pub id: String,
    /// Plan name
    pub name: Option<String>,
    /// Number of vCPUs
    pub vcpu_count: Option<i32>,
    /// RAM in MB
    pub ram: Option<i32>,
    /// Disk size in GB
    pub disk: Option<i32>,
    /// Number of disks
    pub disk_count: Option<i32>,
    /// Monthly bandwidth in GB
    pub bandwidth: Option<i32>,
    /// Monthly cost in USD
    pub monthly_cost: Option<f64>,
    /// Hourly cost in USD
    pub hourly_cost: Option<f64>,
    /// Plan type (vc2, vhf, vdc)
    #[serde(rename = "type")]
    pub plan_type: Option<String>,
    /// Regions where this plan is available
    #[serde(default)]
    pub locations: Vec<String>,
}

/// Response wrapper for regions list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionsResponse {
    pub regions: Vec<Region>,
}

/// Response wrapper for OS list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsResponse {
    pub os: Vec<Os>,
}

/// Response wrapper for plans list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlansResponse {
    pub plans: Vec<Plan>,
}
