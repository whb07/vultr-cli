//! Common model types shared across the API

use serde::{Deserialize, Serialize};

/// Pagination metadata
#[derive(Serialize, Deserialize, Default)]
pub struct Meta {
    /// Total number of objects available
    pub total: Option<i32>,
    /// Pagination links
    pub links: Option<PaginationLinks>,
}

/// Cursor-based pagination links
#[derive(Serialize, Deserialize, Default)]
pub struct PaginationLinks {
    /// Cursor for the next page
    pub next: Option<String>,
    /// Cursor for the previous page  
    pub prev: Option<String>,
}

/// Generic list response wrapper
#[derive(Serialize, Deserialize)]
pub struct ListResponse<T> {
    #[serde(flatten)]
    pub data: T,
    #[serde(default)]
    pub meta: Meta,
}

/// Region information
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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

/// Bare metal plan information
#[derive(Serialize, Deserialize)]
pub struct BareMetalPlan {
    /// Unique plan ID (e.g., "vbm-4c-32gb")
    pub id: String,
    /// Number of physical CPUs
    #[serde(default)]
    pub physical_cpus: Option<i32>,
    /// Number of CPUs
    #[serde(default)]
    pub cpu_count: Option<i32>,
    /// Number of CPU cores
    #[serde(default)]
    pub cpu_cores: Option<i32>,
    /// Number of CPU threads
    #[serde(default)]
    pub cpu_threads: Option<i32>,
    /// CPU manufacturer
    #[serde(default)]
    pub cpu_manufacturer: Option<String>,
    /// CPU model
    #[serde(default)]
    pub cpu_model: Option<String>,
    /// CPU clock speed in MHz
    #[serde(default)]
    pub cpu_mhz: Option<i32>,
    /// RAM in MB
    #[serde(default)]
    pub ram: Option<i32>,
    /// Disk size in GB
    #[serde(default)]
    pub disk: Option<i32>,
    /// Number of disks
    #[serde(default)]
    pub disk_count: Option<i32>,
    /// Monthly bandwidth in GB
    #[serde(default)]
    pub bandwidth: Option<i32>,
    /// Invoice type (hourly or monthly)
    #[serde(default)]
    pub invoice_type: Option<String>,
    /// Whether the plan supports on-demand deployment
    #[serde(default)]
    pub deploy_ondemand: Option<bool>,
    /// Whether the plan supports preemptible deployment
    #[serde(default)]
    pub deploy_preemptible: Option<bool>,
    /// Monthly cost in USD
    #[serde(default)]
    pub monthly_cost: Option<f64>,
    /// Hourly cost in USD
    #[serde(default)]
    pub hourly_cost: Option<f64>,
    /// Monthly cost for preemptible configurations
    #[serde(default)]
    pub monthly_cost_preemptible: Option<f64>,
    /// Hourly cost for preemptible configurations
    #[serde(default)]
    pub hourly_cost_preemptible: Option<f64>,
    /// Plan type (e.g., SSD)
    #[serde(default, rename = "type")]
    pub plan_type: Option<String>,
    /// Regions where this plan is available
    #[serde(default)]
    pub locations: Vec<String>,
}

/// Response wrapper for regions list
#[derive(Serialize, Deserialize)]
pub struct RegionsResponse {
    pub regions: Vec<Region>,
}

/// Response wrapper for OS list
#[derive(Serialize, Deserialize)]
pub struct OsResponse {
    pub os: Vec<Os>,
}

/// Response wrapper for plans list
#[derive(Serialize, Deserialize)]
pub struct PlansResponse {
    pub plans: Vec<Plan>,
}

/// Response wrapper for bare metal plans list
#[derive(Serialize, Deserialize)]
pub struct BareMetalPlansResponse {
    #[serde(rename = "plans_metal")]
    pub plans: Vec<BareMetalPlan>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_default() {
        let meta = Meta::default();
        assert!(meta.total.is_none());
        assert!(meta.links.is_none());
    }

    #[test]
    fn test_pagination_links_default() {
        let links = PaginationLinks::default();
        assert!(links.next.is_none());
        assert!(links.prev.is_none());
    }

    #[test]
    fn test_region_deserialize() {
        let json = r#"{"id":"ewr","city":"New York","country":"US","continent":"North America","options":["ddos_protection","block_storage"]}"#;
        let region: Region = serde_json::from_str(json).unwrap();
        assert_eq!(region.id, "ewr");
        assert_eq!(region.city.unwrap(), "New York");
        assert_eq!(region.country.unwrap(), "US");
        assert_eq!(region.options.len(), 2);
    }

    #[test]
    fn test_region_deserialize_minimal() {
        let json = r#"{"id":"lax"}"#;
        let region: Region = serde_json::from_str(json).unwrap();
        assert_eq!(region.id, "lax");
        assert!(region.city.is_none());
        assert!(region.options.is_empty());
    }

    #[test]
    fn test_os_deserialize() {
        let json = r#"{"id":215,"name":"Ubuntu 22.04 LTS","arch":"x64","family":"ubuntu"}"#;
        let os: Os = serde_json::from_str(json).unwrap();
        assert_eq!(os.id, 215);
        assert_eq!(os.name.unwrap(), "Ubuntu 22.04 LTS");
        assert_eq!(os.arch.unwrap(), "x64");
        assert_eq!(os.family.unwrap(), "ubuntu");
    }

    #[test]
    fn test_plan_deserialize() {
        let json = r#"{"id":"vc2-1c-1gb","name":"Cloud Compute","vcpu_count":1,"ram":1024,"disk":25,"bandwidth":1000,"monthly_cost":5.0,"hourly_cost":0.007,"type":"vc2","locations":["ewr","lax"]}"#;
        let plan: Plan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.id, "vc2-1c-1gb");
        assert_eq!(plan.vcpu_count.unwrap(), 1);
        assert_eq!(plan.ram.unwrap(), 1024);
        assert_eq!(plan.monthly_cost.unwrap(), 5.0);
        assert_eq!(plan.locations.len(), 2);
    }

    #[test]
    fn test_plan_deserialize_minimal() {
        let json = r#"{"id":"vc2-2c-2gb"}"#;
        let plan: Plan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.id, "vc2-2c-2gb");
        assert!(plan.vcpu_count.is_none());
        assert!(plan.locations.is_empty());
    }

    #[test]
    fn test_meta_with_pagination() {
        let json = r#"{"total":100,"links":{"next":"cursor123","prev":"cursor000"}}"#;
        let meta: Meta = serde_json::from_str(json).unwrap();
        assert_eq!(meta.total.unwrap(), 100);
        let links = meta.links.unwrap();
        assert_eq!(links.next.unwrap(), "cursor123");
        assert_eq!(links.prev.unwrap(), "cursor000");
    }

    #[test]
    fn test_list_response_deserialize() {
        let json = r#"{"regions":[{"id":"ewr"}],"meta":{"total":1}}"#;
        let response: ListResponse<RegionsResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.regions.len(), 1);
        assert_eq!(response.meta.total.unwrap(), 1);
    }
}
