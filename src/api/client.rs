//! HTTP client for the Vultr API

use crate::error::{ApiErrorResponse, VultrError, VultrResult};
use crate::models::*;
use reqwest::{header::RETRY_AFTER, Client, Method, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

use rand::Rng;

/// Base URL for the Vultr API
const API_BASE_URL: &str = "https://api.vultr.com/v2";

/// HTTP client for making Vultr API requests
#[derive(Debug, Clone)]
pub struct VultrClient {
    client: Client,
    api_key: String,
    max_retries: u32,
    backoff_initial_ms: u64,
    backoff_max_ms: u64,
}

impl VultrClient {
    /// Create a new Vultr API client
    pub fn new(api_key: String, http_timeout_seconds: u64, max_retries: u32, backoff_initial_ms: u64, backoff_max_ms: u64) -> VultrResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(http_timeout_seconds.max(1)))
            .user_agent(format!("vultr-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Self { client, api_key, max_retries, backoff_initial_ms, backoff_max_ms })
    }

    /// Make an authenticated request to the Vultr API
    async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> VultrResult<T> {
        let url = format!("{}{}", API_BASE_URL, path);

        let make_request = || {
            let mut req = self
                .client
                .request(method.clone(), &url)
                .header("Authorization", format!("Bearer {}", self.api_key));
            if let Some(ref b) = body {
                req = req.json(b);
            }
            req
        };

        let response = self.send_with_retry(make_request).await?;
        self.handle_response(response).await
    }

    /// Make a GET request
    async fn get<T: DeserializeOwned>(&self, path: &str) -> VultrResult<T> {
        self.request::<T>(Method::GET, path, None).await
    }

    /// Make a POST request
    async fn post<T: DeserializeOwned>(&self, path: &str, body: impl Serialize) -> VultrResult<T> {
        self.request(Method::POST, path, Some(serde_json::to_value(body)?)).await
    }

    /// Make a PATCH request
    async fn patch<T: DeserializeOwned>(&self, path: &str, body: impl Serialize) -> VultrResult<T> {
        self.request(Method::PATCH, path, Some(serde_json::to_value(body)?)).await
    }

    /// Make a PUT request
    async fn put<T: DeserializeOwned>(&self, path: &str, body: impl Serialize) -> VultrResult<T> {
        self.request(Method::PUT, path, Some(serde_json::to_value(body)?)).await
    }

    /// Make a DELETE request
    async fn delete(&self, path: &str) -> VultrResult<()> {
        let url = format!("{}{}", API_BASE_URL, path);

        let make_request = || {
            self.client
                .delete(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
        };

        let response = self.send_with_retry(make_request).await?;
        if response.status().is_success() { Ok(()) } else { self.handle_error(response).await }
    }

    /// Make a DELETE request with a body
    async fn delete_with_body(&self, path: &str, body: impl Serialize) -> VultrResult<()> {
        let url = format!("{}{}", API_BASE_URL, path);
        let body_value = serde_json::to_value(body)?;

        let make_request = || {
            self.client
                .delete(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&body_value)
        };

        let response = self.send_with_retry(make_request).await?;
        if response.status().is_success() { Ok(()) } else { self.handle_error(response).await }
    }

    /// Make a POST request that returns no content
    async fn post_no_content(&self, path: &str, body: impl Serialize) -> VultrResult<()> {
        let url = format!("{}{}", API_BASE_URL, path);
        let body_value = serde_json::to_value(body)?;

        let make_request = || {
            self.client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&body_value)
        };

        let response = self.send_with_retry(make_request).await?;
        if response.status().is_success() { Ok(()) } else { self.handle_error(response).await }
    }

    /// Send a request with retries for transient failures.
    ///
    /// Retries on:
    /// - 429 Too Many Requests (honors Retry-After)
    /// - 408 Request Timeout
    /// - 5xx server errors
    /// - network timeouts / connect errors
    async fn send_with_retry<F>(&self, make_request: F) -> VultrResult<Response>
    where
        F: Fn() -> reqwest::RequestBuilder,
    {
        let mut attempt: u32 = 0;
        loop {
            let resp = make_request().send().await;

            match resp {
                Ok(response) => {
                    let status = response.status();
                    if self.should_retry_status(status) && attempt < self.max_retries {
                        let sleep_for = self.compute_retry_delay(&response, attempt);
                        tracing::warn!(
                            "Transient HTTP {}. Retrying in {:?} (attempt {}/{})",
                            status,
                            sleep_for,
                            attempt + 1,
                            self.max_retries
                        );
                        tokio::time::sleep(sleep_for).await;
                        attempt += 1;
                        continue;
                    }
                    return Ok(response);
                }
                Err(e) => {
                    if self.should_retry_error(&e) && attempt < self.max_retries {
                        let sleep_for = self.compute_backoff(attempt);
                        tracing::warn!(
                            "Transient network error: {}. Retrying in {:?} (attempt {}/{})",
                            e,
                            sleep_for,
                            attempt + 1,
                            self.max_retries
                        );
                        tokio::time::sleep(sleep_for).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(crate::error::VultrError::NetworkError(e));
                }
            }
        }
    }

    fn should_retry_status(&self, status: StatusCode) -> bool {
        status == StatusCode::TOO_MANY_REQUESTS
            || status == StatusCode::REQUEST_TIMEOUT
            || status.is_server_error()
    }

    fn should_retry_error(&self, e: &reqwest::Error) -> bool {
        e.is_timeout() || e.is_connect() || e.is_request()
    }

    fn compute_retry_delay(&self, response: &Response, attempt: u32) -> Duration {
        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            if let Some(delay) = self.retry_after_delay(response) {
                return delay;
            }
        }
        self.compute_backoff(attempt)
    }

    fn retry_after_delay(&self, response: &Response) -> Option<Duration> {
        let h = response.headers().get(RETRY_AFTER)?.to_str().ok()?;
        // Retry-After can be a delta-seconds or an HTTP-date.
        let cap_secs = (self.backoff_max_ms.max(1000) / 1000).max(1);
        if let Ok(secs) = h.parse::<u64>() {
            return Some(Duration::from_secs(secs.min(cap_secs)));
        }
        if let Ok(when) = httpdate::parse_http_date(h) {
            let now = std::time::SystemTime::now();
            if let Ok(delta) = when.duration_since(now) {
                return Some(Duration::from_secs(delta.as_secs().min(cap_secs)));
            }
        }
        None
    }

    fn compute_backoff(&self, attempt: u32) -> Duration {
        let base = self.backoff_initial_ms.max(1);
        let exp = 2u64.saturating_pow(attempt.min(16));
        let mut ms = base.saturating_mul(exp);
        ms = ms.min(self.backoff_max_ms.max(base));
        // Full jitter: random in [0, ms]
        let jittered = rand::thread_rng().gen_range(0..=ms);
        Duration::from_millis(jittered)
    }

    /// Handle API response
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> VultrResult<T> {
        let status = response.status();

        if status.is_success() {
            let body = response.text().await?;
            if body.is_empty() {
                // Handle empty responses - try to parse as default
                return Err(VultrError::JsonError(serde_json::Error::io(
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Empty response"),
                )));
            }
            serde_json::from_str(&body).map_err(|e| {
                tracing::error!("Failed to parse response: {}\nBody: {}", e, body);
                VultrError::JsonError(e)
            })
        } else {
            self.handle_error(response).await
        }
    }

    /// Handle API error response
    async fn handle_error<T>(&self, response: Response) -> VultrResult<T> {
        let status = response.status();

        match status {
            StatusCode::UNAUTHORIZED => Err(VultrError::AuthenticationRequired),
            StatusCode::TOO_MANY_REQUESTS => Err(VultrError::RateLimited),
            StatusCode::NOT_FOUND => {
                let body = response.text().await.unwrap_or_default();
                if let Ok(error) = serde_json::from_str::<ApiErrorResponse>(&body) {
                    Err(VultrError::api_error(status.as_u16(), error.error))
                } else {
                    Err(VultrError::api_error(status.as_u16(), "Resource not found"))
                }
            }
            _ => {
                let body = response.text().await.unwrap_or_default();
                if let Ok(error) = serde_json::from_str::<ApiErrorResponse>(&body) {
                    Err(VultrError::api_error(status.as_u16(), error.error))
                } else {
                    Err(VultrError::api_error(
                        status.as_u16(),
                        format!("API error: {}", body),
                    ))
                }
            }
        }
    }

    // =====================
    // Instance Operations
    // =====================

    /// List all instances
    pub async fn list_instances(&self, per_page: Option<u32>, cursor: Option<&str>) -> VultrResult<(Vec<Instance>, Meta)> {
        let mut path = "/instances".to_string();
        let mut params = vec![];
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        
        let response: ListResponse<InstancesResponse> = self.get(&path).await?;
        Ok((response.data.instances, response.meta))
    }

    /// Get a single instance
    pub async fn get_instance(&self, instance_id: &str) -> VultrResult<Instance> {
        let response: InstanceResponse = self.get(&format!("/instances/{}", instance_id)).await?;
        Ok(response.instance)
    }

    /// Create a new instance
    pub async fn create_instance(&self, request: CreateInstanceRequest) -> VultrResult<Instance> {
        let response: InstanceResponse = self.post("/instances", request).await?;
        Ok(response.instance)
    }

    /// Update an instance
    pub async fn update_instance(&self, instance_id: &str, request: UpdateInstanceRequest) -> VultrResult<Instance> {
        let response: InstanceResponse = self.patch(&format!("/instances/{}", instance_id), request).await?;
        Ok(response.instance)
    }

    /// Delete an instance
    pub async fn delete_instance(&self, instance_id: &str) -> VultrResult<()> {
        self.delete(&format!("/instances/{}", instance_id)).await
    }

    /// Start an instance
    pub async fn start_instance(&self, instance_id: &str) -> VultrResult<()> {
        self.post_no_content(&format!("/instances/{}/start", instance_id), serde_json::json!({})).await
    }

    /// Stop/halt an instance
    pub async fn halt_instance(&self, instance_id: &str) -> VultrResult<()> {
        self.post_no_content(&format!("/instances/{}/halt", instance_id), serde_json::json!({})).await
    }

    /// Reboot an instance
    pub async fn reboot_instance(&self, instance_id: &str) -> VultrResult<()> {
        self.post_no_content(&format!("/instances/{}/reboot", instance_id), serde_json::json!({})).await
    }

    /// Reinstall an instance
    pub async fn reinstall_instance(&self, instance_id: &str, request: ReinstallInstanceRequest) -> VultrResult<Instance> {
        let response: InstanceResponse = self.post(&format!("/instances/{}/reinstall", instance_id), request).await?;
        Ok(response.instance)
    }

    // =====================
    // SSH Key Operations
    // =====================

    /// List all SSH keys
    pub async fn list_ssh_keys(&self, per_page: Option<u32>, cursor: Option<&str>) -> VultrResult<(Vec<SshKey>, Meta)> {
        let mut path = "/ssh-keys".to_string();
        let mut params = vec![];
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        
        let response: ListResponse<SshKeysResponse> = self.get(&path).await?;
        Ok((response.data.ssh_keys, response.meta))
    }

    /// Get a single SSH key
    pub async fn get_ssh_key(&self, ssh_key_id: &str) -> VultrResult<SshKey> {
        let response: SshKeyResponse = self.get(&format!("/ssh-keys/{}", ssh_key_id)).await?;
        Ok(response.ssh_key)
    }

    /// Create a new SSH key
    pub async fn create_ssh_key(&self, request: CreateSshKeyRequest) -> VultrResult<SshKey> {
        let response: SshKeyResponse = self.post("/ssh-keys", request).await?;
        Ok(response.ssh_key)
    }

    /// Update an SSH key
    pub async fn update_ssh_key(&self, ssh_key_id: &str, request: UpdateSshKeyRequest) -> VultrResult<()> {
        self.patch::<serde_json::Value>(&format!("/ssh-keys/{}", ssh_key_id), request).await?;
        Ok(())
    }

    /// Delete an SSH key
    pub async fn delete_ssh_key(&self, ssh_key_id: &str) -> VultrResult<()> {
        self.delete(&format!("/ssh-keys/{}", ssh_key_id)).await
    }

    // =====================
    // Startup Script Operations
    // =====================

    /// List all startup scripts
    pub async fn list_startup_scripts(&self, per_page: Option<u32>, cursor: Option<&str>) -> VultrResult<(Vec<StartupScript>, Meta)> {
        let mut path = "/startup-scripts".to_string();
        let mut params = vec![];
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        
        let response: ListResponse<StartupScriptsResponse> = self.get(&path).await?;
        Ok((response.data.startup_scripts, response.meta))
    }

    /// Get a single startup script
    pub async fn get_startup_script(&self, script_id: &str) -> VultrResult<StartupScript> {
        let response: StartupScriptResponse = self.get(&format!("/startup-scripts/{}", script_id)).await?;
        Ok(response.startup_script)
    }

    /// Create a new startup script
    pub async fn create_startup_script(&self, request: CreateStartupScriptRequest) -> VultrResult<StartupScript> {
        let response: StartupScriptResponse = self.post("/startup-scripts", request).await?;
        Ok(response.startup_script)
    }

    /// Update a startup script
    pub async fn update_startup_script(&self, script_id: &str, request: UpdateStartupScriptRequest) -> VultrResult<()> {
        self.patch::<serde_json::Value>(&format!("/startup-scripts/{}", script_id), request).await?;
        Ok(())
    }

    /// Delete a startup script
    pub async fn delete_startup_script(&self, script_id: &str) -> VultrResult<()> {
        self.delete(&format!("/startup-scripts/{}", script_id)).await
    }

    // =====================
    // Snapshot Operations
    // =====================

    /// List all snapshots
    pub async fn list_snapshots(&self, per_page: Option<u32>, cursor: Option<&str>) -> VultrResult<(Vec<Snapshot>, Meta)> {
        let mut path = "/snapshots".to_string();
        let mut params = vec![];
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        
        let response: ListResponse<SnapshotsResponse> = self.get(&path).await?;
        Ok((response.data.snapshots, response.meta))
    }

    /// Get a single snapshot
    pub async fn get_snapshot(&self, snapshot_id: &str) -> VultrResult<Snapshot> {
        let response: SnapshotResponse = self.get(&format!("/snapshots/{}", snapshot_id)).await?;
        Ok(response.snapshot)
    }

    /// Create a new snapshot
    pub async fn create_snapshot(&self, request: CreateSnapshotRequest) -> VultrResult<Snapshot> {
        let response: SnapshotResponse = self.post("/snapshots", request).await?;
        Ok(response.snapshot)
    }

    /// Create a snapshot from URL
    pub async fn create_snapshot_from_url(&self, request: CreateSnapshotFromUrlRequest) -> VultrResult<Snapshot> {
        let response: SnapshotResponse = self.post("/snapshots/create-from-url", request).await?;
        Ok(response.snapshot)
    }

    /// Update a snapshot
    pub async fn update_snapshot(&self, snapshot_id: &str, request: UpdateSnapshotRequest) -> VultrResult<()> {
        self.put::<serde_json::Value>(&format!("/snapshots/{}", snapshot_id), request).await?;
        Ok(())
    }

    /// Delete a snapshot
    pub async fn delete_snapshot(&self, snapshot_id: &str) -> VultrResult<()> {
        self.delete(&format!("/snapshots/{}", snapshot_id)).await
    }

    // =====================
    // Block Storage Operations
    // =====================

    /// List all block storage volumes
    pub async fn list_block_storage(&self, per_page: Option<u32>, cursor: Option<&str>) -> VultrResult<(Vec<BlockStorage>, Meta)> {
        let mut path = "/blocks".to_string();
        let mut params = vec![];
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        
        let response: ListResponse<BlockStoragesResponse> = self.get(&path).await?;
        Ok((response.data.blocks, response.meta))
    }

    /// Get a single block storage volume
    pub async fn get_block_storage(&self, block_id: &str) -> VultrResult<BlockStorage> {
        let response: BlockStorageResponse = self.get(&format!("/blocks/{}", block_id)).await?;
        Ok(response.block)
    }

    /// Create a new block storage volume
    pub async fn create_block_storage(&self, request: CreateBlockStorageRequest) -> VultrResult<BlockStorage> {
        let response: BlockStorageResponse = self.post("/blocks", request).await?;
        Ok(response.block)
    }

    /// Update a block storage volume
    pub async fn update_block_storage(&self, block_id: &str, request: UpdateBlockStorageRequest) -> VultrResult<()> {
        self.patch::<serde_json::Value>(&format!("/blocks/{}", block_id), request).await?;
        Ok(())
    }

    /// Delete a block storage volume
    pub async fn delete_block_storage(&self, block_id: &str) -> VultrResult<()> {
        self.delete(&format!("/blocks/{}", block_id)).await
    }

    /// Attach block storage to an instance
    pub async fn attach_block_storage(&self, block_id: &str, request: AttachBlockStorageRequest) -> VultrResult<()> {
        self.post_no_content(&format!("/blocks/{}/attach", block_id), request).await
    }

    /// Detach block storage from an instance
    pub async fn detach_block_storage(&self, block_id: &str, request: DetachBlockStorageRequest) -> VultrResult<()> {
        self.post_no_content(&format!("/blocks/{}/detach", block_id), request).await
    }

    // =====================
    // Firewall Operations
    // =====================

    /// List all firewall groups
    pub async fn list_firewall_groups(&self, per_page: Option<u32>, cursor: Option<&str>) -> VultrResult<(Vec<FirewallGroup>, Meta)> {
        let mut path = "/firewalls".to_string();
        let mut params = vec![];
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        
        let response: ListResponse<FirewallGroupsResponse> = self.get(&path).await?;
        Ok((response.data.firewall_groups, response.meta))
    }

    /// Get a single firewall group
    pub async fn get_firewall_group(&self, group_id: &str) -> VultrResult<FirewallGroup> {
        let response: FirewallGroupResponse = self.get(&format!("/firewalls/{}", group_id)).await?;
        Ok(response.firewall_group)
    }

    /// Create a new firewall group
    pub async fn create_firewall_group(&self, request: CreateFirewallGroupRequest) -> VultrResult<FirewallGroup> {
        let response: FirewallGroupResponse = self.post("/firewalls", request).await?;
        Ok(response.firewall_group)
    }

    /// Update a firewall group
    pub async fn update_firewall_group(&self, group_id: &str, request: UpdateFirewallGroupRequest) -> VultrResult<()> {
        self.put::<serde_json::Value>(&format!("/firewalls/{}", group_id), request).await?;
        Ok(())
    }

    /// Delete a firewall group
    pub async fn delete_firewall_group(&self, group_id: &str) -> VultrResult<()> {
        self.delete(&format!("/firewalls/{}", group_id)).await
    }

    /// List rules in a firewall group
    pub async fn list_firewall_rules(&self, group_id: &str, per_page: Option<u32>, cursor: Option<&str>) -> VultrResult<(Vec<FirewallRule>, Meta)> {
        let mut path = format!("/firewalls/{}/rules", group_id);
        let mut params = vec![];
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        
        let response: ListResponse<FirewallRulesResponse> = self.get(&path).await?;
        Ok((response.data.firewall_rules, response.meta))
    }

    /// Get a single firewall rule
    pub async fn get_firewall_rule(&self, group_id: &str, rule_id: i32) -> VultrResult<FirewallRule> {
        let response: FirewallRuleResponse = self.get(&format!("/firewalls/{}/rules/{}", group_id, rule_id)).await?;
        Ok(response.firewall_rule)
    }

    /// Create a new firewall rule
    pub async fn create_firewall_rule(&self, group_id: &str, request: CreateFirewallRuleRequest) -> VultrResult<FirewallRule> {
        let response: FirewallRuleResponse = self.post(&format!("/firewalls/{}/rules", group_id), request).await?;
        Ok(response.firewall_rule)
    }

    /// Delete a firewall rule
    pub async fn delete_firewall_rule(&self, group_id: &str, rule_id: i32) -> VultrResult<()> {
        self.delete(&format!("/firewalls/{}/rules/{}", group_id, rule_id)).await
    }

    // =====================
    // VPC Operations
    // =====================

    /// List all VPCs
    pub async fn list_vpcs(&self, per_page: Option<u32>, cursor: Option<&str>) -> VultrResult<(Vec<Vpc>, Meta)> {
        let mut path = "/vpcs".to_string();
        let mut params = vec![];
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        
        let response: ListResponse<VpcsResponse> = self.get(&path).await?;
        Ok((response.data.vpcs, response.meta))
    }

    /// Get a single VPC
    pub async fn get_vpc(&self, vpc_id: &str) -> VultrResult<Vpc> {
        let response: VpcResponse = self.get(&format!("/vpcs/{}", vpc_id)).await?;
        Ok(response.vpc)
    }

    /// Create a new VPC
    pub async fn create_vpc(&self, request: CreateVpcRequest) -> VultrResult<Vpc> {
        let response: VpcResponse = self.post("/vpcs", request).await?;
        Ok(response.vpc)
    }

    /// Update a VPC
    pub async fn update_vpc(&self, vpc_id: &str, request: UpdateVpcRequest) -> VultrResult<()> {
        self.put::<serde_json::Value>(&format!("/vpcs/{}", vpc_id), request).await?;
        Ok(())
    }

    /// Delete a VPC
    pub async fn delete_vpc(&self, vpc_id: &str) -> VultrResult<()> {
        self.delete(&format!("/vpcs/{}", vpc_id)).await
    }

    // =====================
    // Reference Data
    // =====================

    /// List all regions
    pub async fn list_regions(&self) -> VultrResult<Vec<Region>> {
        let response: ListResponse<RegionsResponse> = self.get("/regions").await?;
        Ok(response.data.regions)
    }

    /// List all operating systems
    pub async fn list_os(&self) -> VultrResult<Vec<Os>> {
        let response: ListResponse<OsResponse> = self.get("/os").await?;
        Ok(response.data.os)
    }

    /// List all plans
    pub async fn list_plans(&self, plan_type: Option<&str>) -> VultrResult<Vec<Plan>> {
        let path = match plan_type {
            Some(t) => format!("/plans?type={}", t),
            None => "/plans".to_string(),
        };
        let response: ListResponse<PlansResponse> = self.get(&path).await?;
        Ok(response.data.plans)
    }
}
