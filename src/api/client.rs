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
    pub fn new(
        api_key: String,
        http_timeout_seconds: u64,
        max_retries: u32,
        backoff_initial_ms: u64,
        backoff_max_ms: u64,
    ) -> VultrResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(http_timeout_seconds.max(1)))
            .user_agent(format!("vultr-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Self {
            client,
            api_key,
            max_retries,
            backoff_initial_ms,
            backoff_max_ms,
        })
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
        self.request(Method::POST, path, Some(serde_json::to_value(body)?))
            .await
    }

    /// Make a PATCH request
    async fn patch<T: DeserializeOwned>(&self, path: &str, body: impl Serialize) -> VultrResult<T> {
        self.request(Method::PATCH, path, Some(serde_json::to_value(body)?))
            .await
    }

    /// Make a PUT request
    async fn put<T: DeserializeOwned>(&self, path: &str, body: impl Serialize) -> VultrResult<T> {
        self.request(Method::PUT, path, Some(serde_json::to_value(body)?))
            .await
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
        if response.status().is_success() {
            Ok(())
        } else {
            self.handle_error(response).await
        }
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
        if response.status().is_success() {
            Ok(())
        } else {
            self.handle_error(response).await
        }
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
        if response.status().is_success() {
            Ok(())
        } else {
            self.handle_error(response).await
        }
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
    pub async fn list_instances(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<Instance>, Meta)> {
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
    pub async fn update_instance(
        &self,
        instance_id: &str,
        request: UpdateInstanceRequest,
    ) -> VultrResult<Instance> {
        let response: InstanceResponse = self
            .patch(&format!("/instances/{}", instance_id), request)
            .await?;
        Ok(response.instance)
    }

    /// Delete an instance
    pub async fn delete_instance(&self, instance_id: &str) -> VultrResult<()> {
        self.delete(&format!("/instances/{}", instance_id)).await
    }

    /// Start an instance
    pub async fn start_instance(&self, instance_id: &str) -> VultrResult<()> {
        self.post_no_content(
            &format!("/instances/{}/start", instance_id),
            serde_json::json!({}),
        )
        .await
    }

    /// Stop/halt an instance
    pub async fn halt_instance(&self, instance_id: &str) -> VultrResult<()> {
        self.post_no_content(
            &format!("/instances/{}/halt", instance_id),
            serde_json::json!({}),
        )
        .await
    }

    /// Reboot an instance
    pub async fn reboot_instance(&self, instance_id: &str) -> VultrResult<()> {
        self.post_no_content(
            &format!("/instances/{}/reboot", instance_id),
            serde_json::json!({}),
        )
        .await
    }

    /// Reinstall an instance
    pub async fn reinstall_instance(
        &self,
        instance_id: &str,
        request: ReinstallInstanceRequest,
    ) -> VultrResult<Instance> {
        let response: InstanceResponse = self
            .post(&format!("/instances/{}/reinstall", instance_id), request)
            .await?;
        Ok(response.instance)
    }

    /// Get instance bandwidth
    pub async fn get_instance_bandwidth(
        &self,
        instance_id: &str,
    ) -> VultrResult<std::collections::HashMap<String, BandwidthData>> {
        let response: BandwidthResponse = self
            .get(&format!("/instances/{}/bandwidth", instance_id))
            .await?;
        Ok(response.bandwidth)
    }

    /// Get instance neighbors (instances on the same host)
    pub async fn get_instance_neighbors(&self, instance_id: &str) -> VultrResult<Vec<String>> {
        let response: NeighborsResponse = self
            .get(&format!("/instances/{}/neighbors", instance_id))
            .await?;
        Ok(response.neighbors)
    }

    /// List instance IPv4 addresses
    pub async fn list_instance_ipv4(&self, instance_id: &str) -> VultrResult<Vec<Ipv4Info>> {
        let response: Ipv4Response = self
            .get(&format!("/instances/{}/ipv4", instance_id))
            .await?;
        Ok(response.ipv4s)
    }

    /// Create an additional IPv4 address for an instance
    pub async fn create_instance_ipv4(
        &self,
        instance_id: &str,
        request: CreateIpv4Request,
    ) -> VultrResult<Ipv4Info> {
        #[derive(serde::Deserialize)]
        struct Resp {
            ipv4: Ipv4Info,
        }
        let response: Resp = self
            .post(&format!("/instances/{}/ipv4", instance_id), request)
            .await?;
        Ok(response.ipv4)
    }

    /// Delete an IPv4 address from an instance
    pub async fn delete_instance_ipv4(&self, instance_id: &str, ipv4: &str) -> VultrResult<()> {
        self.delete(&format!("/instances/{}/ipv4/{}", instance_id, ipv4))
            .await
    }

    /// Set reverse DNS for IPv4
    pub async fn set_instance_reverse_ipv4(
        &self,
        instance_id: &str,
        request: SetReverseIpv4Request,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/instances/{}/ipv4/reverse", instance_id), request)
            .await
    }

    /// Set default reverse DNS for IPv4
    pub async fn set_instance_default_reverse_ipv4(
        &self,
        instance_id: &str,
        request: SetDefaultReverseIpv4Request,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/instances/{}/ipv4/reverse/default", instance_id),
            request,
        )
        .await
    }

    /// List instance IPv6 addresses
    pub async fn list_instance_ipv6(&self, instance_id: &str) -> VultrResult<Vec<Ipv6Info>> {
        let response: Ipv6Response = self
            .get(&format!("/instances/{}/ipv6", instance_id))
            .await?;
        Ok(response.ipv6s)
    }

    /// List instance IPv6 reverse DNS entries
    pub async fn list_instance_reverse_ipv6(
        &self,
        instance_id: &str,
    ) -> VultrResult<Vec<ReverseIpv6>> {
        let response: ReverseIpv6Response = self
            .get(&format!("/instances/{}/ipv6/reverse", instance_id))
            .await?;
        Ok(response.reverse_ipv6s)
    }

    /// Set reverse DNS for IPv6
    pub async fn set_instance_reverse_ipv6(
        &self,
        instance_id: &str,
        request: SetReverseIpv6Request,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/instances/{}/ipv6/reverse", instance_id), request)
            .await
    }

    /// Delete reverse DNS for IPv6
    pub async fn delete_instance_reverse_ipv6(
        &self,
        instance_id: &str,
        ipv6: &str,
    ) -> VultrResult<()> {
        self.delete(&format!("/instances/{}/ipv6/reverse/{}", instance_id, ipv6))
            .await
    }

    /// Get instance ISO status
    pub async fn get_instance_iso_status(&self, instance_id: &str) -> VultrResult<IsoStatus> {
        let response: IsoStatusResponse =
            self.get(&format!("/instances/{}/iso", instance_id)).await?;
        Ok(response.iso_status)
    }

    /// Attach ISO to an instance
    pub async fn attach_instance_iso(
        &self,
        instance_id: &str,
        request: AttachIsoRequest,
    ) -> VultrResult<IsoStatus> {
        let response: IsoStatusResponse = self
            .post(&format!("/instances/{}/iso/attach", instance_id), request)
            .await?;
        Ok(response.iso_status)
    }

    /// Detach ISO from an instance
    pub async fn detach_instance_iso(&self, instance_id: &str) -> VultrResult<IsoStatus> {
        let response: IsoStatusResponse = self
            .post(
                &format!("/instances/{}/iso/detach", instance_id),
                serde_json::json!({}),
            )
            .await?;
        Ok(response.iso_status)
    }

    /// Get instance backup schedule
    pub async fn get_instance_backup_schedule(
        &self,
        instance_id: &str,
    ) -> VultrResult<BackupSchedule> {
        let response: BackupScheduleResponse = self
            .get(&format!("/instances/{}/backup-schedule", instance_id))
            .await?;
        Ok(response.backup_schedule)
    }

    /// Set instance backup schedule
    pub async fn set_instance_backup_schedule(
        &self,
        instance_id: &str,
        request: SetBackupScheduleRequest,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/instances/{}/backup-schedule", instance_id),
            request,
        )
        .await
    }

    /// Get available upgrades for an instance
    pub async fn get_instance_upgrades(&self, instance_id: &str) -> VultrResult<AvailableUpgrades> {
        let response: AvailableUpgradesResponse = self
            .get(&format!("/instances/{}/upgrades", instance_id))
            .await?;
        Ok(response.upgrades)
    }

    /// Get instance user data
    pub async fn get_instance_user_data(&self, instance_id: &str) -> VultrResult<UserData> {
        let response: UserDataResponse = self
            .get(&format!("/instances/{}/user-data", instance_id))
            .await?;
        Ok(response.user_data)
    }

    /// Restore an instance from backup or snapshot
    pub async fn restore_instance(
        &self,
        instance_id: &str,
        request: RestoreInstanceRequest,
    ) -> VultrResult<RestoreStatus> {
        let response: RestoreStatusResponse = self
            .post(&format!("/instances/{}/restore", instance_id), request)
            .await?;
        Ok(response.status)
    }

    /// List VPCs attached to an instance
    pub async fn list_instance_vpcs(&self, instance_id: &str) -> VultrResult<Vec<InstanceVpc>> {
        let response: InstanceVpcsResponse = self
            .get(&format!("/instances/{}/vpcs", instance_id))
            .await?;
        Ok(response.vpcs)
    }

    /// Attach a VPC to an instance
    pub async fn attach_instance_vpc(
        &self,
        instance_id: &str,
        request: AttachVpcRequest,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/instances/{}/vpcs/attach", instance_id), request)
            .await
    }

    /// Detach a VPC from an instance
    pub async fn detach_instance_vpc(
        &self,
        instance_id: &str,
        request: DetachVpcRequest,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/instances/{}/vpcs/detach", instance_id), request)
            .await
    }

    /// List VPC2s attached to an instance
    pub async fn list_instance_vpc2s(&self, instance_id: &str) -> VultrResult<Vec<InstanceVpc2>> {
        let response: InstanceVpc2sResponse = self
            .get(&format!("/instances/{}/vpc2", instance_id))
            .await?;
        Ok(response.vpcs)
    }

    /// Attach a VPC2 to an instance
    pub async fn attach_instance_vpc2(
        &self,
        instance_id: &str,
        request: AttachVpc2Request,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/instances/{}/vpc2/attach", instance_id), request)
            .await
    }

    /// Detach a VPC2 from an instance
    pub async fn detach_instance_vpc2(
        &self,
        instance_id: &str,
        request: DetachVpc2Request,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/instances/{}/vpc2/detach", instance_id), request)
            .await
    }

    /// Bulk halt instances
    pub async fn bulk_halt_instances(&self, request: BulkInstancesRequest) -> VultrResult<()> {
        self.post_no_content("/instances/halt", request).await
    }

    /// Bulk start instances
    pub async fn bulk_start_instances(&self, request: BulkInstancesRequest) -> VultrResult<()> {
        self.post_no_content("/instances/start", request).await
    }

    /// Bulk reboot instances
    pub async fn bulk_reboot_instances(&self, request: BulkInstancesRequest) -> VultrResult<()> {
        self.post_no_content("/instances/reboot", request).await
    }

    // =====================
    // SSH Key Operations
    // =====================

    /// List all SSH keys
    pub async fn list_ssh_keys(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<SshKey>, Meta)> {
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
    pub async fn update_ssh_key(
        &self,
        ssh_key_id: &str,
        request: UpdateSshKeyRequest,
    ) -> VultrResult<()> {
        self.patch::<serde_json::Value>(&format!("/ssh-keys/{}", ssh_key_id), request)
            .await?;
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
    pub async fn list_startup_scripts(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<StartupScript>, Meta)> {
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
        let response: StartupScriptResponse =
            self.get(&format!("/startup-scripts/{}", script_id)).await?;
        Ok(response.startup_script)
    }

    /// Create a new startup script
    pub async fn create_startup_script(
        &self,
        request: CreateStartupScriptRequest,
    ) -> VultrResult<StartupScript> {
        let response: StartupScriptResponse = self.post("/startup-scripts", request).await?;
        Ok(response.startup_script)
    }

    /// Update a startup script
    pub async fn update_startup_script(
        &self,
        script_id: &str,
        request: UpdateStartupScriptRequest,
    ) -> VultrResult<()> {
        self.patch::<serde_json::Value>(&format!("/startup-scripts/{}", script_id), request)
            .await?;
        Ok(())
    }

    /// Delete a startup script
    pub async fn delete_startup_script(&self, script_id: &str) -> VultrResult<()> {
        self.delete(&format!("/startup-scripts/{}", script_id))
            .await
    }

    // =====================
    // Snapshot Operations
    // =====================

    /// List all snapshots
    pub async fn list_snapshots(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<Snapshot>, Meta)> {
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
    pub async fn create_snapshot_from_url(
        &self,
        request: CreateSnapshotFromUrlRequest,
    ) -> VultrResult<Snapshot> {
        let response: SnapshotResponse = self.post("/snapshots/create-from-url", request).await?;
        Ok(response.snapshot)
    }

    /// Update a snapshot
    pub async fn update_snapshot(
        &self,
        snapshot_id: &str,
        request: UpdateSnapshotRequest,
    ) -> VultrResult<()> {
        self.put::<serde_json::Value>(&format!("/snapshots/{}", snapshot_id), request)
            .await?;
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
    pub async fn list_block_storage(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<BlockStorage>, Meta)> {
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
    pub async fn create_block_storage(
        &self,
        request: CreateBlockStorageRequest,
    ) -> VultrResult<BlockStorage> {
        let response: BlockStorageResponse = self.post("/blocks", request).await?;
        Ok(response.block)
    }

    /// Update a block storage volume
    pub async fn update_block_storage(
        &self,
        block_id: &str,
        request: UpdateBlockStorageRequest,
    ) -> VultrResult<()> {
        self.patch::<serde_json::Value>(&format!("/blocks/{}", block_id), request)
            .await?;
        Ok(())
    }

    /// Delete a block storage volume
    pub async fn delete_block_storage(&self, block_id: &str) -> VultrResult<()> {
        self.delete(&format!("/blocks/{}", block_id)).await
    }

    /// Attach block storage to an instance
    pub async fn attach_block_storage(
        &self,
        block_id: &str,
        request: AttachBlockStorageRequest,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/blocks/{}/attach", block_id), request)
            .await
    }

    /// Detach block storage from an instance
    pub async fn detach_block_storage(
        &self,
        block_id: &str,
        request: DetachBlockStorageRequest,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/blocks/{}/detach", block_id), request)
            .await
    }

    // =====================
    // Firewall Operations
    // =====================

    /// List all firewall groups
    pub async fn list_firewall_groups(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<FirewallGroup>, Meta)> {
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
    pub async fn create_firewall_group(
        &self,
        request: CreateFirewallGroupRequest,
    ) -> VultrResult<FirewallGroup> {
        let response: FirewallGroupResponse = self.post("/firewalls", request).await?;
        Ok(response.firewall_group)
    }

    /// Update a firewall group
    pub async fn update_firewall_group(
        &self,
        group_id: &str,
        request: UpdateFirewallGroupRequest,
    ) -> VultrResult<()> {
        self.put::<serde_json::Value>(&format!("/firewalls/{}", group_id), request)
            .await?;
        Ok(())
    }

    /// Delete a firewall group
    pub async fn delete_firewall_group(&self, group_id: &str) -> VultrResult<()> {
        self.delete(&format!("/firewalls/{}", group_id)).await
    }

    /// List rules in a firewall group
    pub async fn list_firewall_rules(
        &self,
        group_id: &str,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<FirewallRule>, Meta)> {
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
    pub async fn get_firewall_rule(
        &self,
        group_id: &str,
        rule_id: i32,
    ) -> VultrResult<FirewallRule> {
        let response: FirewallRuleResponse = self
            .get(&format!("/firewalls/{}/rules/{}", group_id, rule_id))
            .await?;
        Ok(response.firewall_rule)
    }

    /// Create a new firewall rule
    pub async fn create_firewall_rule(
        &self,
        group_id: &str,
        request: CreateFirewallRuleRequest,
    ) -> VultrResult<FirewallRule> {
        let response: FirewallRuleResponse = self
            .post(&format!("/firewalls/{}/rules", group_id), request)
            .await?;
        Ok(response.firewall_rule)
    }

    /// Delete a firewall rule
    pub async fn delete_firewall_rule(&self, group_id: &str, rule_id: i32) -> VultrResult<()> {
        self.delete(&format!("/firewalls/{}/rules/{}", group_id, rule_id))
            .await
    }

    // =====================
    // VPC Operations
    // =====================

    /// List all VPCs
    pub async fn list_vpcs(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<Vpc>, Meta)> {
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
        self.put::<serde_json::Value>(&format!("/vpcs/{}", vpc_id), request)
            .await?;
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

    // =====================
    // Kubernetes Operations
    // =====================

    /// List all Kubernetes clusters
    pub async fn list_kubernetes_clusters(&self) -> VultrResult<Vec<KubernetesCluster>> {
        let response: ClustersResponse = self.get("/kubernetes/clusters").await?;
        Ok(response.vke_clusters)
    }

    /// Get a single Kubernetes cluster
    pub async fn get_kubernetes_cluster(&self, vke_id: &str) -> VultrResult<KubernetesCluster> {
        let response: ClusterResponse = self
            .get(&format!("/kubernetes/clusters/{}", vke_id))
            .await?;
        Ok(response.vke_cluster)
    }

    /// Create a new Kubernetes cluster
    pub async fn create_kubernetes_cluster(
        &self,
        request: CreateClusterRequest,
    ) -> VultrResult<KubernetesCluster> {
        let response: ClusterResponse = self.post("/kubernetes/clusters", request).await?;
        Ok(response.vke_cluster)
    }

    /// Update a Kubernetes cluster
    pub async fn update_kubernetes_cluster(
        &self,
        vke_id: &str,
        request: UpdateClusterRequest,
    ) -> VultrResult<KubernetesCluster> {
        let response: ClusterResponse = self
            .put(&format!("/kubernetes/clusters/{}", vke_id), request)
            .await?;
        Ok(response.vke_cluster)
    }

    /// Delete a Kubernetes cluster
    pub async fn delete_kubernetes_cluster(&self, vke_id: &str) -> VultrResult<()> {
        self.delete(&format!("/kubernetes/clusters/{}", vke_id))
            .await
    }

    /// Delete a Kubernetes cluster with all linked resources
    pub async fn delete_kubernetes_cluster_with_resources(&self, vke_id: &str) -> VultrResult<()> {
        self.delete(&format!(
            "/kubernetes/clusters/{}/delete-with-linked-resources",
            vke_id
        ))
        .await
    }

    /// Get kubeconfig for a cluster
    pub async fn get_kubernetes_config(&self, vke_id: &str) -> VultrResult<String> {
        let response: KubeconfigResponse = self
            .get(&format!("/kubernetes/clusters/{}/config", vke_id))
            .await?;
        Ok(response.kube_config)
    }

    /// Get available Kubernetes versions
    pub async fn get_kubernetes_versions(&self) -> VultrResult<Vec<String>> {
        let response: VersionsResponse = self.get("/kubernetes/versions").await?;
        Ok(response.versions)
    }

    /// Get available upgrades for a cluster
    pub async fn get_kubernetes_available_upgrades(
        &self,
        vke_id: &str,
    ) -> VultrResult<Vec<String>> {
        let response: KubernetesUpgradesResponse = self
            .get(&format!(
                "/kubernetes/clusters/{}/available-upgrades",
                vke_id
            ))
            .await?;
        Ok(response.available_upgrades)
    }

    /// Upgrade a Kubernetes cluster
    pub async fn upgrade_kubernetes_cluster(
        &self,
        vke_id: &str,
        request: UpgradeClusterRequest,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/kubernetes/clusters/{}/upgrades", vke_id),
            request,
        )
        .await
    }

    /// Get resources deployed by a cluster
    pub async fn get_kubernetes_resources(&self, vke_id: &str) -> VultrResult<ClusterResources> {
        self.get(&format!("/kubernetes/clusters/{}/resources", vke_id))
            .await
    }

    // Node Pool Operations

    /// List node pools in a cluster
    pub async fn list_node_pools(&self, vke_id: &str) -> VultrResult<Vec<NodePool>> {
        let response: NodePoolsResponse = self
            .get(&format!("/kubernetes/clusters/{}/node-pools", vke_id))
            .await?;
        Ok(response.node_pools)
    }

    /// Get a single node pool
    pub async fn get_node_pool(&self, vke_id: &str, nodepool_id: &str) -> VultrResult<NodePool> {
        let response: NodePoolResponse = self
            .get(&format!(
                "/kubernetes/clusters/{}/node-pools/{}",
                vke_id, nodepool_id
            ))
            .await?;
        Ok(response.node_pool)
    }

    /// Create a node pool
    pub async fn create_node_pool(
        &self,
        vke_id: &str,
        request: CreateNodePoolRequest,
    ) -> VultrResult<NodePool> {
        let response: NodePoolResponse = self
            .post(
                &format!("/kubernetes/clusters/{}/node-pools", vke_id),
                request,
            )
            .await?;
        Ok(response.node_pool)
    }

    /// Update a node pool
    pub async fn update_node_pool(
        &self,
        vke_id: &str,
        nodepool_id: &str,
        request: UpdateNodePoolRequest,
    ) -> VultrResult<NodePool> {
        let response: NodePoolResponse = self
            .patch(
                &format!("/kubernetes/clusters/{}/node-pools/{}", vke_id, nodepool_id),
                request,
            )
            .await?;
        Ok(response.node_pool)
    }

    /// Delete a node pool
    pub async fn delete_node_pool(&self, vke_id: &str, nodepool_id: &str) -> VultrResult<()> {
        self.delete(&format!(
            "/kubernetes/clusters/{}/node-pools/{}",
            vke_id, nodepool_id
        ))
        .await
    }

    // Node Operations

    /// List nodes in a node pool
    pub async fn list_nodes(&self, vke_id: &str, nodepool_id: &str) -> VultrResult<Vec<KubeNode>> {
        let response: NodesResponse = self
            .get(&format!(
                "/kubernetes/clusters/{}/node-pools/{}/nodes",
                vke_id, nodepool_id
            ))
            .await?;
        Ok(response.nodes)
    }

    /// Get a single node
    pub async fn get_node(
        &self,
        vke_id: &str,
        nodepool_id: &str,
        node_id: &str,
    ) -> VultrResult<KubeNode> {
        let response: NodeResponse = self
            .get(&format!(
                "/kubernetes/clusters/{}/node-pools/{}/nodes/{}",
                vke_id, nodepool_id, node_id
            ))
            .await?;
        Ok(response.node)
    }

    /// Delete a node
    pub async fn delete_node(
        &self,
        vke_id: &str,
        nodepool_id: &str,
        node_id: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/kubernetes/clusters/{}/node-pools/{}/nodes/{}",
            vke_id, nodepool_id, node_id
        ))
        .await
    }

    /// Recycle a node
    pub async fn recycle_node(
        &self,
        vke_id: &str,
        nodepool_id: &str,
        node_id: &str,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!(
                "/kubernetes/clusters/{}/node-pools/{}/nodes/{}/recycle",
                vke_id, nodepool_id, node_id
            ),
            serde_json::json!({}),
        )
        .await
    }

    // =========================================================================
    // Managed Database Operations
    // =========================================================================

    /// List all managed databases
    pub async fn list_databases(&self) -> VultrResult<Vec<Database>> {
        let response: DatabasesResponse = self.get("/databases").await?;
        Ok(response.databases)
    }

    /// Get a single managed database
    pub async fn get_database(&self, database_id: &str) -> VultrResult<Database> {
        let response: DatabaseResponse = self.get(&format!("/databases/{}", database_id)).await?;
        Ok(response.database)
    }

    /// Create a managed database
    pub async fn create_database(&self, request: CreateDatabaseRequest) -> VultrResult<Database> {
        let response: DatabaseResponse = self.post("/databases", request).await?;
        Ok(response.database)
    }

    /// Update a managed database
    pub async fn update_database(
        &self,
        database_id: &str,
        request: UpdateDatabaseRequest,
    ) -> VultrResult<Database> {
        let response: DatabaseResponse = self
            .put(&format!("/databases/{}", database_id), request)
            .await?;
        Ok(response.database)
    }

    /// Delete a managed database
    pub async fn delete_database(&self, database_id: &str) -> VultrResult<()> {
        self.delete(&format!("/databases/{}", database_id)).await
    }

    /// List database plans
    pub async fn list_database_plans(
        &self,
        engine: Option<&str>,
        nodes: Option<i32>,
        region: Option<&str>,
    ) -> VultrResult<Vec<DatabasePlan>> {
        let mut params = Vec::new();
        if let Some(e) = engine {
            params.push(format!("engine={}", e));
        }
        if let Some(n) = nodes {
            params.push(format!("nodes={}", n));
        }
        if let Some(r) = region {
            params.push(format!("region={}", r));
        }
        let path = if params.is_empty() {
            "/databases/plans".to_string()
        } else {
            format!("/databases/plans?{}", params.join("&"))
        };
        let response: DatabasePlansResponse = self.get(&path).await?;
        Ok(response.plans)
    }

    /// Get database usage
    pub async fn get_database_usage(&self, database_id: &str) -> VultrResult<DatabaseUsage> {
        let response: DatabaseUsageResponse = self
            .get(&format!("/databases/{}/usage", database_id))
            .await?;
        Ok(response.usage)
    }

    /// Get database alerts
    pub async fn get_database_alerts(&self, database_id: &str) -> VultrResult<Vec<DatabaseAlert>> {
        let response: DatabaseAlertsResponse = self
            .get(&format!("/databases/{}/alerts", database_id))
            .await?;
        Ok(response.alerts)
    }

    /// Get database backups
    pub async fn get_database_backups(
        &self,
        database_id: &str,
    ) -> VultrResult<DatabaseBackupsResponse> {
        self.get(&format!("/databases/{}/backups", database_id))
            .await
    }

    /// Restore database from backup
    pub async fn restore_database(
        &self,
        database_id: &str,
        request: RestoreDatabaseRequest,
    ) -> VultrResult<Database> {
        let response: DatabaseResponse = self
            .post(&format!("/databases/{}/restore", database_id), request)
            .await?;
        Ok(response.database)
    }

    /// Fork a database
    pub async fn fork_database(
        &self,
        database_id: &str,
        request: ForkDatabaseRequest,
    ) -> VultrResult<Database> {
        let response: DatabaseResponse = self
            .post(&format!("/databases/{}/fork", database_id), request)
            .await?;
        Ok(response.database)
    }

    /// Create a read replica
    pub async fn create_read_replica(
        &self,
        database_id: &str,
        request: CreateReadReplicaRequest,
    ) -> VultrResult<Database> {
        let response: DatabaseResponse = self
            .post(&format!("/databases/{}/read-replica", database_id), request)
            .await?;
        Ok(response.database)
    }

    /// Promote a read replica to standalone
    pub async fn promote_read_replica(&self, database_id: &str) -> VultrResult<()> {
        self.post_no_content(
            &format!("/databases/{}/promote-read-replica", database_id),
            serde_json::json!({}),
        )
        .await
    }

    /// Get maintenance schedule
    pub async fn get_database_maintenance(
        &self,
        database_id: &str,
    ) -> VultrResult<MaintenanceSchedule> {
        let response: MaintenanceResponse = self
            .get(&format!("/databases/{}/maintenance", database_id))
            .await?;
        Ok(response.maintenance)
    }

    /// Update maintenance schedule
    pub async fn update_database_maintenance(
        &self,
        database_id: &str,
        request: UpdateMaintenanceRequest,
    ) -> VultrResult<MaintenanceSchedule> {
        let response: MaintenanceResponse = self
            .post(&format!("/databases/{}/maintenance", database_id), request)
            .await?;
        Ok(response.maintenance)
    }

    /// Start database migration
    pub async fn start_database_migration(
        &self,
        database_id: &str,
        request: StartMigrationRequest,
    ) -> VultrResult<DatabaseMigration> {
        let response: DatabaseMigrationResponse = self
            .post(&format!("/databases/{}/migration", database_id), request)
            .await?;
        Ok(response.migration)
    }

    /// Get database migration status
    pub async fn get_database_migration(
        &self,
        database_id: &str,
    ) -> VultrResult<DatabaseMigration> {
        let response: DatabaseMigrationResponse = self
            .get(&format!("/databases/{}/migration", database_id))
            .await?;
        Ok(response.migration)
    }

    /// Detach database migration
    pub async fn detach_database_migration(&self, database_id: &str) -> VultrResult<()> {
        self.delete(&format!("/databases/{}/migration", database_id))
            .await
    }

    /// Get available version upgrades
    pub async fn get_database_version_upgrades(
        &self,
        database_id: &str,
    ) -> VultrResult<Vec<String>> {
        let response: DatabaseVersionsResponse = self
            .get(&format!("/databases/{}/version-upgrade", database_id))
            .await?;
        Ok(response.available_versions)
    }

    /// Upgrade database version
    pub async fn upgrade_database_version(
        &self,
        database_id: &str,
        request: UpgradeDatabaseVersionRequest,
    ) -> VultrResult<Database> {
        let response: DatabaseResponse = self
            .post(
                &format!("/databases/{}/version-upgrade", database_id),
                request,
            )
            .await?;
        Ok(response.database)
    }

    // Database Users

    /// List database users
    pub async fn list_database_users(&self, database_id: &str) -> VultrResult<Vec<DatabaseUser>> {
        let response: DatabaseUsersResponse = self
            .get(&format!("/databases/{}/users", database_id))
            .await?;
        Ok(response.users)
    }

    /// Get a database user
    pub async fn get_database_user(
        &self,
        database_id: &str,
        username: &str,
    ) -> VultrResult<DatabaseUser> {
        let response: DatabaseUserResponse = self
            .get(&format!("/databases/{}/users/{}", database_id, username))
            .await?;
        Ok(response.user)
    }

    /// Create a database user
    pub async fn create_database_user(
        &self,
        database_id: &str,
        request: CreateDatabaseUserRequest,
    ) -> VultrResult<DatabaseUser> {
        let response: DatabaseUserResponse = self
            .post(&format!("/databases/{}/users", database_id), request)
            .await?;
        Ok(response.user)
    }

    /// Update a database user
    pub async fn update_database_user(
        &self,
        database_id: &str,
        username: &str,
        request: UpdateDatabaseUserRequest,
    ) -> VultrResult<DatabaseUser> {
        let response: DatabaseUserResponse = self
            .put(
                &format!("/databases/{}/users/{}", database_id, username),
                request,
            )
            .await?;
        Ok(response.user)
    }

    /// Delete a database user
    pub async fn delete_database_user(&self, database_id: &str, username: &str) -> VultrResult<()> {
        self.delete(&format!("/databases/{}/users/{}", database_id, username))
            .await
    }

    /// Update user access control (Valkey)
    pub async fn update_user_access_control(
        &self,
        database_id: &str,
        username: &str,
        request: UpdateUserAccessControlRequest,
    ) -> VultrResult<DatabaseUser> {
        let response: DatabaseUserResponse = self
            .put(
                &format!(
                    "/databases/{}/users/{}/access-control",
                    database_id, username
                ),
                request,
            )
            .await?;
        Ok(response.user)
    }

    // Logical Databases

    /// List logical databases
    pub async fn list_logical_databases(
        &self,
        database_id: &str,
    ) -> VultrResult<Vec<LogicalDatabase>> {
        let response: LogicalDatabasesResponse =
            self.get(&format!("/databases/{}/dbs", database_id)).await?;
        Ok(response.dbs)
    }

    /// Get a logical database
    pub async fn get_logical_database(
        &self,
        database_id: &str,
        db_name: &str,
    ) -> VultrResult<LogicalDatabase> {
        let response: LogicalDatabaseResponse = self
            .get(&format!("/databases/{}/dbs/{}", database_id, db_name))
            .await?;
        Ok(response.db)
    }

    /// Create a logical database
    pub async fn create_logical_database(
        &self,
        database_id: &str,
        request: CreateLogicalDatabaseRequest,
    ) -> VultrResult<LogicalDatabase> {
        let response: LogicalDatabaseResponse = self
            .post(&format!("/databases/{}/dbs", database_id), request)
            .await?;
        Ok(response.db)
    }

    /// Delete a logical database
    pub async fn delete_logical_database(
        &self,
        database_id: &str,
        db_name: &str,
    ) -> VultrResult<()> {
        self.delete(&format!("/databases/{}/dbs/{}", database_id, db_name))
            .await
    }

    // Connection Pools (PostgreSQL)

    /// List connection pools
    pub async fn list_connection_pools(
        &self,
        database_id: &str,
    ) -> VultrResult<Vec<ConnectionPool>> {
        let response: ConnectionPoolsResponse = self
            .get(&format!("/databases/{}/connection-pools", database_id))
            .await?;
        Ok(response.connection_pools)
    }

    /// Get a connection pool
    pub async fn get_connection_pool(
        &self,
        database_id: &str,
        pool_name: &str,
    ) -> VultrResult<ConnectionPool> {
        let response: ConnectionPoolResponse = self
            .get(&format!(
                "/databases/{}/connection-pools/{}",
                database_id, pool_name
            ))
            .await?;
        Ok(response.connection_pool)
    }

    /// Create a connection pool
    pub async fn create_connection_pool(
        &self,
        database_id: &str,
        request: CreateConnectionPoolRequest,
    ) -> VultrResult<ConnectionPool> {
        let response: ConnectionPoolResponse = self
            .post(
                &format!("/databases/{}/connection-pools", database_id),
                request,
            )
            .await?;
        Ok(response.connection_pool)
    }

    /// Update a connection pool
    pub async fn update_connection_pool(
        &self,
        database_id: &str,
        pool_name: &str,
        request: UpdateConnectionPoolRequest,
    ) -> VultrResult<ConnectionPool> {
        let response: ConnectionPoolResponse = self
            .put(
                &format!("/databases/{}/connection-pools/{}", database_id, pool_name),
                request,
            )
            .await?;
        Ok(response.connection_pool)
    }

    /// Delete a connection pool
    pub async fn delete_connection_pool(
        &self,
        database_id: &str,
        pool_name: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/databases/{}/connection-pools/{}",
            database_id, pool_name
        ))
        .await
    }

    // Kafka Topics

    /// List Kafka topics
    pub async fn list_kafka_topics(&self, database_id: &str) -> VultrResult<Vec<KafkaTopic>> {
        let response: KafkaTopicsResponse = self
            .get(&format!("/databases/{}/topics", database_id))
            .await?;
        Ok(response.topics)
    }

    /// Get a Kafka topic
    pub async fn get_kafka_topic(
        &self,
        database_id: &str,
        topic_name: &str,
    ) -> VultrResult<KafkaTopic> {
        let response: KafkaTopicResponse = self
            .get(&format!("/databases/{}/topics/{}", database_id, topic_name))
            .await?;
        Ok(response.topic)
    }

    /// Create a Kafka topic
    pub async fn create_kafka_topic(
        &self,
        database_id: &str,
        request: CreateKafkaTopicRequest,
    ) -> VultrResult<KafkaTopic> {
        let response: KafkaTopicResponse = self
            .post(&format!("/databases/{}/topics", database_id), request)
            .await?;
        Ok(response.topic)
    }

    /// Update a Kafka topic
    pub async fn update_kafka_topic(
        &self,
        database_id: &str,
        topic_name: &str,
        request: UpdateKafkaTopicRequest,
    ) -> VultrResult<KafkaTopic> {
        let response: KafkaTopicResponse = self
            .put(
                &format!("/databases/{}/topics/{}", database_id, topic_name),
                request,
            )
            .await?;
        Ok(response.topic)
    }

    /// Delete a Kafka topic
    pub async fn delete_kafka_topic(&self, database_id: &str, topic_name: &str) -> VultrResult<()> {
        self.delete(&format!("/databases/{}/topics/{}", database_id, topic_name))
            .await
    }

    // Kafka Connectors

    /// List available connectors
    pub async fn list_available_connectors(
        &self,
        database_id: &str,
    ) -> VultrResult<Vec<AvailableConnector>> {
        let response: AvailableConnectorsResponse = self
            .get(&format!("/databases/{}/available-connectors", database_id))
            .await?;
        Ok(response.available_connectors)
    }

    /// List Kafka connectors
    pub async fn list_kafka_connectors(
        &self,
        database_id: &str,
    ) -> VultrResult<Vec<KafkaConnector>> {
        let response: KafkaConnectorsResponse = self
            .get(&format!("/databases/{}/connectors", database_id))
            .await?;
        Ok(response.connectors)
    }

    /// Get a Kafka connector
    pub async fn get_kafka_connector(
        &self,
        database_id: &str,
        connector_name: &str,
    ) -> VultrResult<KafkaConnector> {
        let response: KafkaConnectorResponse = self
            .get(&format!(
                "/databases/{}/connectors/{}",
                database_id, connector_name
            ))
            .await?;
        Ok(response.connector)
    }

    /// Create a Kafka connector
    pub async fn create_kafka_connector(
        &self,
        database_id: &str,
        request: CreateKafkaConnectorRequest,
    ) -> VultrResult<KafkaConnector> {
        let response: KafkaConnectorResponse = self
            .post(&format!("/databases/{}/connectors", database_id), request)
            .await?;
        Ok(response.connector)
    }

    /// Delete a Kafka connector
    pub async fn delete_kafka_connector(
        &self,
        database_id: &str,
        connector_name: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/databases/{}/connectors/{}",
            database_id, connector_name
        ))
        .await
    }

    /// Get connector status
    pub async fn get_connector_status(
        &self,
        database_id: &str,
        connector_name: &str,
    ) -> VultrResult<ConnectorStatus> {
        let response: ConnectorStatusResponse = self
            .get(&format!(
                "/databases/{}/connectors/{}/status",
                database_id, connector_name
            ))
            .await?;
        Ok(response.status)
    }

    /// Pause a connector
    pub async fn pause_kafka_connector(
        &self,
        database_id: &str,
        connector_name: &str,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!(
                "/databases/{}/connectors/{}/pause",
                database_id, connector_name
            ),
            serde_json::json!({}),
        )
        .await
    }

    /// Resume a connector
    pub async fn resume_kafka_connector(
        &self,
        database_id: &str,
        connector_name: &str,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!(
                "/databases/{}/connectors/{}/resume",
                database_id, connector_name
            ),
            serde_json::json!({}),
        )
        .await
    }

    /// Restart a connector
    pub async fn restart_kafka_connector(
        &self,
        database_id: &str,
        connector_name: &str,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!(
                "/databases/{}/connectors/{}/restart",
                database_id, connector_name
            ),
            serde_json::json!({}),
        )
        .await
    }

    /// Restart a connector task
    pub async fn restart_connector_task(
        &self,
        database_id: &str,
        connector_name: &str,
        task_id: &str,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!(
                "/databases/{}/connectors/{}/tasks/{}/restart",
                database_id, connector_name, task_id
            ),
            serde_json::json!({}),
        )
        .await
    }

    // Kafka Quotas

    /// List database quotas
    pub async fn list_database_quotas(&self, database_id: &str) -> VultrResult<Vec<DatabaseQuota>> {
        let response: DatabaseQuotasResponse = self
            .get(&format!("/databases/{}/quotas", database_id))
            .await?;
        Ok(response.quotas)
    }

    /// Create a database quota
    pub async fn create_database_quota(
        &self,
        database_id: &str,
        username: &str,
        request: CreateDatabaseQuotaRequest,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!(
                "/databases/{}/quotas/{}/{}",
                database_id, request.client_id, username
            ),
            request,
        )
        .await
    }

    /// Delete a database quota
    pub async fn delete_database_quota(
        &self,
        database_id: &str,
        client_id: &str,
        username: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/databases/{}/quotas/{}/{}",
            database_id, client_id, username
        ))
        .await
    }

    // Advanced Options (Kafka)

    /// Get advanced options
    pub async fn get_database_advanced_options(
        &self,
        database_id: &str,
    ) -> VultrResult<serde_json::Value> {
        self.get(&format!("/databases/{}/advanced-options", database_id))
            .await
    }

    /// Update advanced options
    pub async fn update_database_advanced_options(
        &self,
        database_id: &str,
        options: serde_json::Value,
    ) -> VultrResult<serde_json::Value> {
        self.put(
            &format!("/databases/{}/advanced-options", database_id),
            options,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_base_url() {
        assert_eq!(API_BASE_URL, "https://api.vultr.com/v2");
    }

    #[test]
    fn test_vultr_client_new() {
        let client = VultrClient::new("test-api-key".to_string(), 30, 3, 100, 10000);
        assert!(client.is_ok());
    }

    #[test]
    fn test_vultr_client_new_with_min_timeout() {
        let client = VultrClient::new(
            "test-api-key".to_string(),
            0, // Will be clamped to 1
            3,
            100,
            10000,
        );
        assert!(client.is_ok());
    }

    #[test]
    fn test_vultr_client_clone() {
        let client = VultrClient::new("test-key".to_string(), 30, 3, 100, 10000).unwrap();
        let cloned = client.clone();
        assert_eq!(cloned.max_retries, 3);
    }

    #[test]
    fn test_vultr_client_debug() {
        let client = VultrClient::new("test-key".to_string(), 30, 3, 100, 10000).unwrap();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("VultrClient"));
    }

    #[test]
    fn test_should_retry_status_429() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 10000).unwrap();
        assert!(client.should_retry_status(StatusCode::TOO_MANY_REQUESTS));
    }

    #[test]
    fn test_should_retry_status_408() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 10000).unwrap();
        assert!(client.should_retry_status(StatusCode::REQUEST_TIMEOUT));
    }

    #[test]
    fn test_should_retry_status_500() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 10000).unwrap();
        assert!(client.should_retry_status(StatusCode::INTERNAL_SERVER_ERROR));
    }

    #[test]
    fn test_should_retry_status_502() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 10000).unwrap();
        assert!(client.should_retry_status(StatusCode::BAD_GATEWAY));
    }

    #[test]
    fn test_should_not_retry_status_200() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 10000).unwrap();
        assert!(!client.should_retry_status(StatusCode::OK));
    }

    #[test]
    fn test_should_not_retry_status_400() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 10000).unwrap();
        assert!(!client.should_retry_status(StatusCode::BAD_REQUEST));
    }

    #[test]
    fn test_should_not_retry_status_401() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 10000).unwrap();
        assert!(!client.should_retry_status(StatusCode::UNAUTHORIZED));
    }

    #[test]
    fn test_should_not_retry_status_404() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 10000).unwrap();
        assert!(!client.should_retry_status(StatusCode::NOT_FOUND));
    }

    #[test]
    fn test_compute_backoff_attempt_0() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 10000).unwrap();
        let delay = client.compute_backoff(0);
        assert!(delay.as_millis() <= 100);
    }

    #[test]
    fn test_compute_backoff_attempt_1() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 10000).unwrap();
        let delay = client.compute_backoff(1);
        assert!(delay.as_millis() <= 200);
    }

    #[test]
    fn test_compute_backoff_capped_at_max() {
        let client = VultrClient::new("key".to_string(), 30, 3, 100, 5000).unwrap();
        let delay = client.compute_backoff(10);
        assert!(delay.as_millis() <= 5000);
    }
}
