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

    /// Make a PUT request that returns no content
    async fn put_no_content(&self, path: &str, body: impl Serialize) -> VultrResult<()> {
        let url = format!("{}{}", API_BASE_URL, path);
        let body_value = serde_json::to_value(body)?;

        let make_request = || {
            self.client
                .put(&url)
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

    /// Make a PATCH request that returns no content
    async fn patch_no_content(&self, path: &str, body: impl Serialize) -> VultrResult<()> {
        let url = format!("{}{}", API_BASE_URL, path);
        let body_value = serde_json::to_value(body)?;

        let make_request = || {
            self.client
                .patch(&url)
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
            StatusCode::UNAUTHORIZED => Err(VultrError::InvalidApiKey),
            StatusCode::TOO_MANY_REQUESTS => Err(VultrError::RateLimited),
            StatusCode::NOT_FOUND => {
                let body = response.text().await.unwrap_or_default();
                if let Ok(error) = serde_json::from_str::<ApiErrorResponse>(&body) {
                    let error_status = error.status.unwrap_or(status.as_u16());
                    Err(VultrError::api_error(error_status, error.error))
                } else {
                    Err(VultrError::api_error(status.as_u16(), "Resource not found"))
                }
            }
            _ => {
                let body = response.text().await.unwrap_or_default();
                if let Ok(error) = serde_json::from_str::<ApiErrorResponse>(&body) {
                    let error_status = error.status.unwrap_or(status.as_u16());
                    Err(VultrError::api_error(error_status, error.error))
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
    // ISO Operations
    // =====================

    /// List all ISOs
    pub async fn list_isos(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<Iso>, Meta)> {
        let mut path = "/iso".to_string();
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

        let response: ListResponse<IsosResponse> = self.get(&path).await?;
        Ok((response.data.isos, response.meta))
    }

    /// Get a single ISO
    pub async fn get_iso(&self, iso_id: &str) -> VultrResult<Iso> {
        let response: IsoResponse = self.get(&format!("/iso/{}", iso_id)).await?;
        Ok(response.iso)
    }

    /// Create a new ISO from URL
    pub async fn create_iso(&self, request: CreateIsoRequest) -> VultrResult<Iso> {
        let response: IsoResponse = self.post("/iso", request).await?;
        Ok(response.iso)
    }

    /// Delete an ISO
    pub async fn delete_iso(&self, iso_id: &str) -> VultrResult<()> {
        self.delete(&format!("/iso/{}", iso_id)).await
    }

    /// List all public ISOs
    pub async fn list_public_isos(&self) -> VultrResult<Vec<PublicIso>> {
        let response: PublicIsosResponse = self.get("/iso-public").await?;
        Ok(response.public_isos)
    }

    // =====================
    // Backup Operations
    // =====================

    /// List all backups
    pub async fn list_backups(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<Backup>, Meta)> {
        let mut path = "/backups".to_string();
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

        let response: ListResponse<BackupsResponse> = self.get(&path).await?;
        Ok((response.data.backups, response.meta))
    }

    /// Get a single backup
    pub async fn get_backup(&self, backup_id: &str) -> VultrResult<Backup> {
        let response: BackupResponse = self.get(&format!("/backups/{}", backup_id)).await?;
        Ok(response.backup)
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
    // Object Storage Operations
    // =====================

    /// List all object storages
    pub async fn list_object_storages(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<ObjectStorage>, Meta)> {
        let mut path = "/object-storage".to_string();
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

        let response: ListResponse<ObjectStoragesResponse> = self.get(&path).await?;
        Ok((response.data.object_storages, response.meta))
    }

    /// Get a single object storage
    pub async fn get_object_storage(&self, object_storage_id: &str) -> VultrResult<ObjectStorage> {
        let response: ObjectStorageResponse = self
            .get(&format!("/object-storage/{}", object_storage_id))
            .await?;
        Ok(response.object_storage)
    }

    /// Create a new object storage
    pub async fn create_object_storage(
        &self,
        request: CreateObjectStorageRequest,
    ) -> VultrResult<ObjectStorage> {
        let response: ObjectStorageResponse = self.post("/object-storage", request).await?;
        Ok(response.object_storage)
    }

    /// Update an object storage
    pub async fn update_object_storage(
        &self,
        object_storage_id: &str,
        request: UpdateObjectStorageRequest,
    ) -> VultrResult<()> {
        self.put::<serde_json::Value>(&format!("/object-storage/{}", object_storage_id), request)
            .await?;
        Ok(())
    }

    /// Delete an object storage
    pub async fn delete_object_storage(&self, object_storage_id: &str) -> VultrResult<()> {
        self.delete(&format!("/object-storage/{}", object_storage_id))
            .await
    }

    /// Regenerate object storage keys
    pub async fn regenerate_object_storage_keys(
        &self,
        object_storage_id: &str,
    ) -> VultrResult<S3Credentials> {
        let response: RegenerateKeysResponse = self
            .post(
                &format!("/object-storage/{}/regenerate-keys", object_storage_id),
                serde_json::json!({}),
            )
            .await?;
        Ok(response.s3_credentials)
    }

    /// List all object storage clusters
    pub async fn list_object_storage_clusters(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<ObjectStorageCluster>, Meta)> {
        let mut path = "/object-storage/clusters".to_string();
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

        let response: ListResponse<ObjectStorageClustersResponse> = self.get(&path).await?;
        Ok((response.data.clusters, response.meta))
    }

    /// List all object storage tiers
    pub async fn list_object_storage_tiers(&self) -> VultrResult<Vec<ObjectStorageTier>> {
        let response: TiersResponse = self.get("/object-storage/tiers").await?;
        Ok(response.tiers)
    }

    /// List all tiers for a specific cluster
    pub async fn list_cluster_tiers(&self, cluster_id: i32) -> VultrResult<Vec<ClusterTier>> {
        let response: ClusterTiersResponse = self
            .get(&format!("/object-storage/clusters/{}/tiers", cluster_id))
            .await?;
        Ok(response.tiers)
    }

    // =====================
    // Firewall Operations
    // =====================
    // Reserved IP Operations
    // =====================

    /// List all Reserved IPs
    pub async fn list_reserved_ips(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<ReservedIp>, Meta)> {
        let mut path = "/reserved-ips".to_string();
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

        let response: ListResponse<ReservedIpsResponse> = self.get(&path).await?;
        Ok((response.data.reserved_ips, response.meta))
    }

    /// Get a single Reserved IP
    pub async fn get_reserved_ip(&self, reserved_ip_id: &str) -> VultrResult<ReservedIp> {
        let response: ReservedIpResponse = self
            .get(&format!("/reserved-ips/{}", reserved_ip_id))
            .await?;
        Ok(response.reserved_ip)
    }

    /// Create a new Reserved IP
    pub async fn create_reserved_ip(
        &self,
        request: CreateReservedIpRequest,
    ) -> VultrResult<ReservedIp> {
        let response: ReservedIpResponse = self.post("/reserved-ips", request).await?;
        Ok(response.reserved_ip)
    }

    /// Update a Reserved IP
    pub async fn update_reserved_ip(
        &self,
        reserved_ip_id: &str,
        request: UpdateReservedIpRequest,
    ) -> VultrResult<ReservedIp> {
        let response: ReservedIpResponse = self
            .patch(&format!("/reserved-ips/{}", reserved_ip_id), request)
            .await?;
        Ok(response.reserved_ip)
    }

    /// Delete a Reserved IP
    pub async fn delete_reserved_ip(&self, reserved_ip_id: &str) -> VultrResult<()> {
        self.delete(&format!("/reserved-ips/{}", reserved_ip_id))
            .await
    }

    /// Attach a Reserved IP to an instance
    pub async fn attach_reserved_ip(
        &self,
        reserved_ip_id: &str,
        request: AttachReservedIpRequest,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/reserved-ips/{}/attach", reserved_ip_id), request)
            .await
    }

    /// Detach a Reserved IP from an instance
    pub async fn detach_reserved_ip(&self, reserved_ip_id: &str) -> VultrResult<()> {
        self.post_no_content(
            &format!("/reserved-ips/{}/detach", reserved_ip_id),
            serde_json::json!({}),
        )
        .await
    }

    /// Convert an instance IP to a Reserved IP
    pub async fn convert_to_reserved_ip(
        &self,
        request: ConvertReservedIpRequest,
    ) -> VultrResult<ReservedIp> {
        let response: ReservedIpResponse = self.post("/reserved-ips/convert", request).await?;
        Ok(response.reserved_ip)
    }

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

    /// List VPC attachments
    pub async fn list_vpc_attachments(
        &self,
        vpc_id: &str,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<VpcAttachment>, Option<Meta>)> {
        let mut path = format!("/vpcs/{}/attachments", vpc_id);
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
        let response: VpcAttachmentsResponse = self.get(&path).await?;
        Ok((response.attachments, response.meta))
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
    // VPC 2.0 Operations
    // =====================

    /// List all VPC 2.0 networks
    pub async fn list_vpc2s(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<Vpc2>, Meta)> {
        let mut path = "/vpc2".to_string();
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

        let response: ListResponse<Vpcs2Response> = self.get(&path).await?;
        Ok((response.data.vpcs, response.meta))
    }

    /// Get a single VPC 2.0 network
    pub async fn get_vpc2(&self, vpc_id: &str) -> VultrResult<Vpc2> {
        let response: Vpc2Response = self.get(&format!("/vpc2/{}", vpc_id)).await?;
        Ok(response.vpc)
    }

    /// Create a new VPC 2.0 network
    pub async fn create_vpc2(&self, request: CreateVpc2Request) -> VultrResult<Vpc2> {
        let response: Vpc2Response = self.post("/vpc2", request).await?;
        Ok(response.vpc)
    }

    /// Update a VPC 2.0 network
    pub async fn update_vpc2(&self, vpc_id: &str, request: UpdateVpcRequest) -> VultrResult<()> {
        self.put::<serde_json::Value>(&format!("/vpc2/{}", vpc_id), request)
            .await?;
        Ok(())
    }

    /// Delete a VPC 2.0 network
    pub async fn delete_vpc2(&self, vpc_id: &str) -> VultrResult<()> {
        self.delete(&format!("/vpc2/{}", vpc_id)).await
    }

    /// List nodes attached to a VPC 2.0 network
    pub async fn list_vpc2_nodes(
        &self,
        vpc_id: &str,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<Vpc2Node>, Meta)> {
        let mut path = format!("/vpc2/{}/nodes", vpc_id);
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

        let response: ListResponse<Vpc2NodesResponse> = self.get(&path).await?;
        Ok((response.data.nodes, response.meta))
    }

    /// Attach nodes to a VPC 2.0 network
    pub async fn attach_vpc2_nodes(
        &self,
        vpc_id: &str,
        request: AttachVpc2NodesRequest,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/vpc2/{}/nodes/attach", vpc_id), request)
            .await
    }

    /// Detach nodes from a VPC 2.0 network
    pub async fn detach_vpc2_nodes(
        &self,
        vpc_id: &str,
        request: DetachVpc2NodesRequest,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/vpc2/{}/nodes/detach", vpc_id), request)
            .await
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

    /// List all bare metal plans
    pub async fn list_bare_metal_plans(&self) -> VultrResult<Vec<BareMetalPlan>> {
        let response: ListResponse<BareMetalPlansResponse> = self.get("/plans-metal").await?;
        Ok(response.data.plans)
    }

    /// List all applications
    pub async fn list_applications(&self) -> VultrResult<Vec<Application>> {
        let response: ApplicationsResponse = self.get("/applications").await?;
        Ok(response.applications)
    }

    /// List marketplace app variables for an image ID
    pub async fn list_app_variables(&self, image_id: &str) -> VultrResult<Vec<AppVariable>> {
        let response: AppVariablesResponse = self
            .get(&format!("/marketplace/apps/{}/variables", image_id))
            .await?;
        Ok(response.variables)
    }

    // =====================
    // Serverless Inference
    // =====================

    /// List inference subscriptions
    pub async fn list_inference(&self) -> VultrResult<Vec<InferenceSubscription>> {
        let response: InferenceListResponse = self.get("/inference").await?;
        Ok(response.subscriptions)
    }

    /// Get inference subscription
    pub async fn get_inference(&self, inference_id: &str) -> VultrResult<InferenceSubscription> {
        let response: InferenceResponse = self
            .get(&format!("/inference/{}", inference_id))
            .await?;
        Ok(response.subscription)
    }

    /// Create inference subscription
    pub async fn create_inference(
        &self,
        request: CreateInferenceRequest,
    ) -> VultrResult<InferenceSubscription> {
        let response: InferenceResponse = self.post("/inference", request).await?;
        Ok(response.subscription)
    }

    /// Update inference subscription
    pub async fn update_inference(
        &self,
        inference_id: &str,
        request: UpdateInferenceRequest,
    ) -> VultrResult<InferenceSubscription> {
        let response: InferenceResponse = self
            .patch(&format!("/inference/{}", inference_id), request)
            .await?;
        Ok(response.subscription)
    }

    /// Delete inference subscription
    pub async fn delete_inference(&self, inference_id: &str) -> VultrResult<()> {
        self.delete(&format!("/inference/{}", inference_id)).await
    }

    /// Get inference usage
    pub async fn get_inference_usage(&self, inference_id: &str) -> VultrResult<InferenceUsage> {
        let response: InferenceUsageResponse = self
            .get(&format!("/inference/{}/usage", inference_id))
            .await?;
        Ok(response.usage)
    }

    // =====================
    // Logs
    // =====================

    /// List logs with optional filters
    pub async fn list_logs(
        &self,
        start_time: Option<&str>,
        end_time: Option<&str>,
        log_level: Option<&str>,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        continue_time: Option<&str>,
    ) -> VultrResult<LogsResponse> {
        let mut serializer = url::form_urlencoded::Serializer::new(String::new());
        if let Some(value) = start_time {
            serializer.append_pair("start_time", value);
        }
        if let Some(value) = end_time {
            serializer.append_pair("end_time", value);
        }
        if let Some(value) = log_level {
            serializer.append_pair("log_level", value);
        }
        if let Some(value) = resource_type {
            serializer.append_pair("resource_type", value);
        }
        if let Some(value) = resource_id {
            serializer.append_pair("resource_id", value);
        }
        if let Some(value) = continue_time {
            serializer.append_pair("continue_time", value);
        }
        let query = serializer.finish();
        let path = if query.is_empty() {
            "/logs".to_string()
        } else {
            format!("/logs?{}", query)
        };
        self.get(&path).await
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

    // Node Pool Label Operations

    /// List labels for a node pool
    pub async fn list_node_pool_labels(
        &self,
        vke_id: &str,
        nodepool_id: &str,
    ) -> VultrResult<Vec<NodePoolLabel>> {
        let response: NodePoolLabelsResponse = self
            .get(&format!(
                "/kubernetes/clusters/{}/node-pools/{}/labels",
                vke_id, nodepool_id
            ))
            .await?;
        Ok(response.labels)
    }

    /// Create a label for a node pool
    pub async fn create_node_pool_label(
        &self,
        vke_id: &str,
        nodepool_id: &str,
        request: CreateNodePoolLabelRequest,
    ) -> VultrResult<NodePoolLabel> {
        let response: serde_json::Value = self
            .post(
                &format!(
                    "/kubernetes/clusters/{}/node-pools/{}/labels",
                    vke_id, nodepool_id
                ),
                &request,
            )
            .await?;
        serde_json::from_value(response["label"].clone()).map_err(VultrError::JsonError)
    }

    /// Delete a label from a node pool
    pub async fn delete_node_pool_label(
        &self,
        vke_id: &str,
        nodepool_id: &str,
        label_id: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/kubernetes/clusters/{}/node-pools/{}/labels/{}",
            vke_id, nodepool_id, label_id
        ))
        .await
    }

    // Node Pool Taint Operations

    /// List taints for a node pool
    pub async fn list_node_pool_taints(
        &self,
        vke_id: &str,
        nodepool_id: &str,
    ) -> VultrResult<Vec<NodePoolTaint>> {
        let response: NodePoolTaintsResponse = self
            .get(&format!(
                "/kubernetes/clusters/{}/node-pools/{}/taints",
                vke_id, nodepool_id
            ))
            .await?;
        Ok(response.taints)
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
        let _ = response.meta.as_ref();
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
        let _ = response.meta.as_ref();
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
        let _ = response.meta.as_ref();
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

    /// Update Kafka user permissions
    pub async fn update_kafka_permissions(
        &self,
        database_id: &str,
        username: &str,
        request: KafkaPermissions,
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
        let _ = response.meta.as_ref();
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
    ) -> VultrResult<ConnectionPoolsResponse> {
        let response: ConnectionPoolsResponse = self
            .get(&format!("/databases/{}/connection-pools", database_id))
            .await?;
        let _ = response.meta.as_ref();
        Ok(response)
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
        let _ = response.meta.as_ref();
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
        let _ = response.meta.as_ref();
        Ok(response.available_connectors)
    }

    /// Get connector configuration schema
    pub async fn get_connector_configuration_schema(
        &self,
        database_id: &str,
        connector_class: &str,
    ) -> VultrResult<Vec<DatabaseConnectorConfigurationSchema>> {
        let response: DatabaseConnectorConfigurationSchemaResponse = self
            .get(&format!(
                "/databases/{}/available-connectors/{}/configuration",
                database_id, connector_class
            ))
            .await?;
        Ok(response.configuration_schema)
    }

    /// List Kafka connectors
    pub async fn list_kafka_connectors(
        &self,
        database_id: &str,
    ) -> VultrResult<Vec<KafkaConnector>> {
        let response: KafkaConnectorsResponse = self
            .get(&format!("/databases/{}/connectors", database_id))
            .await?;
        let _ = response.meta.as_ref();
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
        let _ = response.meta.as_ref();
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

    fn touch_advanced_options_types() {
        let _ = PgAdvancedOptions::default();
        let _ = MysqlAdvancedOptions::default();
        let _ = KafkaAdvancedOptions::default();
        let _ = KafkaRestAdvancedOptions::default();
        let _ = SchemaRegistryAdvancedOptions::default();
        let _ = KafkaConnectAdvancedOptions::default();
    }

    // Advanced Options (Kafka)

    /// Get advanced options
    pub async fn get_database_advanced_options(
        &self,
        database_id: &str,
    ) -> VultrResult<DatabaseAdvancedOptionsResponse> {
        Self::touch_advanced_options_types();
        self.get(&format!("/databases/{}/advanced-options", database_id))
            .await
    }

    /// Update advanced options
    pub async fn update_database_advanced_options(
        &self,
        database_id: &str,
        options: serde_json::Value,
    ) -> VultrResult<DatabaseAdvancedOptionsResponse> {
        self.put(
            &format!("/databases/{}/advanced-options", database_id),
            options,
        )
        .await
    }

    /// Get Kafka REST advanced options
    pub async fn get_kafka_rest_advanced_options(
        &self,
        database_id: &str,
    ) -> VultrResult<DatabaseAdvancedOptionsResponse> {
        self.get(&format!(
            "/databases/{}/advanced-options/kafka-rest",
            database_id
        ))
        .await
    }

    /// Update Kafka REST advanced options
    pub async fn update_kafka_rest_advanced_options(
        &self,
        database_id: &str,
        options: serde_json::Value,
    ) -> VultrResult<DatabaseAdvancedOptionsResponse> {
        self.put(
            &format!("/databases/{}/advanced-options/kafka-rest", database_id),
            options,
        )
        .await
    }

    /// Get schema registry advanced options
    pub async fn get_schema_registry_advanced_options(
        &self,
        database_id: &str,
    ) -> VultrResult<DatabaseAdvancedOptionsResponse> {
        self.get(&format!(
            "/databases/{}/advanced-options/schema-registry",
            database_id
        ))
        .await
    }

    /// Update schema registry advanced options
    pub async fn update_schema_registry_advanced_options(
        &self,
        database_id: &str,
        options: serde_json::Value,
    ) -> VultrResult<DatabaseAdvancedOptionsResponse> {
        self.put(
            &format!(
                "/databases/{}/advanced-options/schema-registry",
                database_id
            ),
            options,
        )
        .await
    }

    /// Get Kafka Connect advanced options
    pub async fn get_kafka_connect_advanced_options(
        &self,
        database_id: &str,
    ) -> VultrResult<DatabaseAdvancedOptionsResponse> {
        self.get(&format!(
            "/databases/{}/advanced-options/kafka-connect",
            database_id
        ))
        .await
    }

    /// Update Kafka Connect advanced options
    pub async fn update_kafka_connect_advanced_options(
        &self,
        database_id: &str,
        options: serde_json::Value,
    ) -> VultrResult<DatabaseAdvancedOptionsResponse> {
        self.put(
            &format!(
                "/databases/{}/advanced-options/kafka-connect",
                database_id
            ),
            options,
        )
        .await
    }

    // =====================
    // DNS Domain Operations
    // =====================

    /// List all DNS domains
    pub async fn list_dns_domains(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<DnsDomain>, Meta)> {
        let mut path = "/domains".to_string();
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
        let response: DomainsResponseWithMeta = self.get(&path).await?;
        Ok((response.domains, response.meta))
    }

    /// Get a DNS domain
    pub async fn get_dns_domain(&self, dns_domain: &str) -> VultrResult<DnsDomain> {
        let response: DomainResponse = self.get(&format!("/domains/{}", dns_domain)).await?;
        Ok(response.domain)
    }

    /// Create a DNS domain
    pub async fn create_dns_domain(&self, request: CreateDomainRequest) -> VultrResult<DnsDomain> {
        let response: DomainResponse = self.post("/domains", request).await?;
        Ok(response.domain)
    }

    /// Update a DNS domain
    pub async fn update_dns_domain(
        &self,
        dns_domain: &str,
        request: UpdateDomainRequest,
    ) -> VultrResult<()> {
        self.put_no_content(&format!("/domains/{}", dns_domain), request)
            .await
    }

    /// Delete a DNS domain
    pub async fn delete_dns_domain(&self, dns_domain: &str) -> VultrResult<()> {
        self.delete(&format!("/domains/{}", dns_domain)).await
    }

    // =====================
    // DNS SOA Operations
    // =====================

    /// Get SOA information for a DNS domain
    pub async fn get_dns_soa(&self, dns_domain: &str) -> VultrResult<DnsSoa> {
        let response: SoaResponse = self.get(&format!("/domains/{}/soa", dns_domain)).await?;
        Ok(response.dns_soa)
    }

    /// Update SOA information for a DNS domain
    pub async fn update_dns_soa(
        &self,
        dns_domain: &str,
        request: UpdateSoaRequest,
    ) -> VultrResult<()> {
        self.patch_no_content(&format!("/domains/{}/soa", dns_domain), request)
            .await
    }

    // =====================
    // DNS DNSSEC Operations
    // =====================

    /// Get DNSSEC information for a DNS domain
    pub async fn get_dns_dnssec(&self, dns_domain: &str) -> VultrResult<Vec<String>> {
        let response: DnsSec = self.get(&format!("/domains/{}/dnssec", dns_domain)).await?;
        Ok(response.dns_sec)
    }

    // =====================
    // DNS Record Operations
    // =====================

    /// List DNS records for a domain
    pub async fn list_dns_records(
        &self,
        dns_domain: &str,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<DnsRecord>, Meta)> {
        let mut path = format!("/domains/{}/records", dns_domain);
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
        let response: RecordsResponseWithMeta = self.get(&path).await?;
        Ok((response.records, response.meta))
    }

    /// Get a DNS record
    pub async fn get_dns_record(
        &self,
        dns_domain: &str,
        record_id: &str,
    ) -> VultrResult<DnsRecord> {
        let response: RecordResponse = self
            .get(&format!("/domains/{}/records/{}", dns_domain, record_id))
            .await?;
        Ok(response.record)
    }

    /// Create a DNS record
    pub async fn create_dns_record(
        &self,
        dns_domain: &str,
        request: CreateRecordRequest,
    ) -> VultrResult<DnsRecord> {
        let response: RecordResponse = self
            .post(&format!("/domains/{}/records", dns_domain), request)
            .await?;
        Ok(response.record)
    }

    /// Update a DNS record
    pub async fn update_dns_record(
        &self,
        dns_domain: &str,
        record_id: &str,
        request: UpdateRecordRequest,
    ) -> VultrResult<()> {
        self.patch_no_content(
            &format!("/domains/{}/records/{}", dns_domain, record_id),
            request,
        )
        .await
    }

    /// Delete a DNS record
    pub async fn delete_dns_record(&self, dns_domain: &str, record_id: &str) -> VultrResult<()> {
        self.delete(&format!("/domains/{}/records/{}", dns_domain, record_id))
            .await
    }

    // =====================
    // Load Balancer Operations
    // =====================

    /// List all load balancers
    pub async fn list_load_balancers(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<LoadBalancer>, Meta)> {
        let mut path = "/load-balancers".to_string();
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

        let response: ListResponse<LoadBalancersResponse> = self.get(&path).await?;
        Ok((response.data.load_balancers, response.meta))
    }

    /// Get a single load balancer
    pub async fn get_load_balancer(&self, lb_id: &str) -> VultrResult<LoadBalancer> {
        let response: LoadBalancerResponse =
            self.get(&format!("/load-balancers/{}", lb_id)).await?;
        Ok(response.load_balancer)
    }

    /// Create a new load balancer
    pub async fn create_load_balancer(
        &self,
        request: CreateLoadBalancerRequest,
    ) -> VultrResult<LoadBalancer> {
        let response: LoadBalancerResponse = self.post("/load-balancers", request).await?;
        Ok(response.load_balancer)
    }

    /// Update a load balancer
    pub async fn update_load_balancer(
        &self,
        lb_id: &str,
        request: UpdateLoadBalancerRequest,
    ) -> VultrResult<LoadBalancer> {
        let response: LoadBalancerResponse = self
            .patch(&format!("/load-balancers/{}", lb_id), request)
            .await?;
        Ok(response.load_balancer)
    }

    /// Delete a load balancer
    pub async fn delete_load_balancer(&self, lb_id: &str) -> VultrResult<()> {
        self.delete(&format!("/load-balancers/{}", lb_id)).await
    }

    // =====================
    // Load Balancer SSL Operations
    // =====================

    /// Add SSL certificate to a load balancer
    pub async fn create_load_balancer_ssl(&self, lb_id: &str, ssl: SSLConfig) -> VultrResult<()> {
        self.post_no_content(&format!("/load-balancers/{}/ssl", lb_id), ssl)
            .await
    }

    /// Delete SSL certificate from a load balancer
    pub async fn delete_load_balancer_ssl(&self, lb_id: &str) -> VultrResult<()> {
        self.delete(&format!("/load-balancers/{}/ssl", lb_id)).await
    }

    /// Disable auto SSL on a load balancer
    pub async fn disable_load_balancer_auto_ssl(&self, lb_id: &str) -> VultrResult<()> {
        self.delete(&format!("/load-balancers/{}/auto_ssl", lb_id))
            .await
    }

    // =====================
    // Load Balancer Forwarding Rules
    // =====================

    /// List forwarding rules for a load balancer
    pub async fn list_load_balancer_forwarding_rules(
        &self,
        lb_id: &str,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<ForwardingRule>, Meta)> {
        let mut path = format!("/load-balancers/{}/forwarding-rules", lb_id);
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

        let response: ListResponse<ForwardingRulesResponse> = self.get(&path).await?;
        Ok((response.data.forwarding_rules, response.meta))
    }

    /// Get a forwarding rule for a load balancer
    pub async fn get_load_balancer_forwarding_rule(
        &self,
        lb_id: &str,
        rule_id: &str,
    ) -> VultrResult<ForwardingRule> {
        let response: ForwardingRuleResponse = self
            .get(&format!(
                "/load-balancers/{}/forwarding-rules/{}",
                lb_id, rule_id
            ))
            .await?;
        Ok(response.forwarding_rule)
    }

    /// Create a forwarding rule for a load balancer
    pub async fn create_load_balancer_forwarding_rule(
        &self,
        lb_id: &str,
        request: CreateForwardingRuleRequest,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/load-balancers/{}/forwarding-rules", lb_id),
            request,
        )
        .await
    }

    /// Delete a forwarding rule from a load balancer
    pub async fn delete_load_balancer_forwarding_rule(
        &self,
        lb_id: &str,
        rule_id: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/load-balancers/{}/forwarding-rules/{}",
            lb_id, rule_id
        ))
        .await
    }

    // =====================
    // Load Balancer Firewall Rules
    // =====================

    /// List firewall rules for a load balancer
    pub async fn list_load_balancer_firewall_rules(
        &self,
        lb_id: &str,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<LBFirewallRule>, Meta)> {
        let mut path = format!("/load-balancers/{}/firewall-rules", lb_id);
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

        let response: ListResponse<LBFirewallRulesResponse> = self.get(&path).await?;
        Ok((response.data.firewall_rules, response.meta))
    }

    /// Get a firewall rule for a load balancer
    pub async fn get_load_balancer_firewall_rule(
        &self,
        lb_id: &str,
        rule_id: &str,
    ) -> VultrResult<LBFirewallRule> {
        let response: LBFirewallRuleResponse = self
            .get(&format!(
                "/load-balancers/{}/firewall-rules/{}",
                lb_id, rule_id
            ))
            .await?;
        Ok(response.firewall_rule)
    }

    /// Create a firewall rule for a load balancer
    pub async fn create_load_balancer_firewall_rule(
        &self,
        lb_id: &str,
        request: CreateLBFirewallRuleRequest,
    ) -> VultrResult<LBFirewallRule> {
        let response: LBFirewallRuleResponse = self
            .post(
                &format!("/load-balancers/{}/firewall-rules", lb_id),
                request,
            )
            .await?;
        Ok(response.firewall_rule)
    }

    /// Delete a firewall rule from a load balancer
    pub async fn delete_load_balancer_firewall_rule(
        &self,
        lb_id: &str,
        rule_id: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/load-balancers/{}/firewall-rules/{}",
            lb_id, rule_id
        ))
        .await
    }

    // =====================
    // Load Balancer Reverse DNS
    // =====================

    /// Get reverse DNS for a load balancer
    pub async fn get_load_balancer_reverse_dns(&self, lb_id: &str) -> VultrResult<ReverseDNS> {
        self.get(&format!("/load-balancers/{}/reverse-dns", lb_id))
            .await
    }

    /// Update IPv4 reverse DNS for a load balancer
    pub async fn update_load_balancer_reverse_dns_ipv4(
        &self,
        lb_id: &str,
        request: UpdateReverseDNSv4Request,
    ) -> VultrResult<()> {
        self.put_no_content(&format!("/load-balancers/{}/reverse-dns", lb_id), request)
            .await
    }

    /// Create IPv6 reverse DNS for a load balancer
    pub async fn create_load_balancer_reverse_dns_ipv6(
        &self,
        lb_id: &str,
        request: CreateReverseDNSv6Request,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/load-balancers/{}/reverse-dns", lb_id), request)
            .await
    }

    // =====================
    // CDN Pull Zone Operations
    // =====================

    /// List all CDN Pull Zones
    pub async fn list_cdn_pull_zones(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<CdnPullZone>, Meta)> {
        let mut path = "/cdns/pull-zones".to_string();
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
        let response: PullZonesResponse = self.get(&path).await?;
        let meta = Meta {
            total: Some(response.pull_zones.len() as i32),
            links: None,
        };
        Ok((response.pull_zones, meta))
    }

    /// Get a CDN Pull Zone
    pub async fn get_cdn_pull_zone(&self, pullzone_id: &str) -> VultrResult<CdnPullZone> {
        let response: PullZoneResponse = self
            .get(&format!("/cdns/pull-zones/{}", pullzone_id))
            .await?;
        Ok(response.pull_zone)
    }

    /// Create a CDN Pull Zone
    pub async fn create_cdn_pull_zone(
        &self,
        request: CreatePullZoneRequest,
    ) -> VultrResult<CdnPullZone> {
        let response: PullZoneResponse = self.post("/cdns/pull-zones", request).await?;
        Ok(response.pull_zone)
    }

    /// Update a CDN Pull Zone
    pub async fn update_cdn_pull_zone(
        &self,
        pullzone_id: &str,
        request: UpdatePullZoneRequest,
    ) -> VultrResult<CdnPullZone> {
        let response: PullZoneResponse = self
            .put(&format!("/cdns/pull-zones/{}", pullzone_id), request)
            .await?;
        Ok(response.pull_zone)
    }

    /// Delete a CDN Pull Zone
    pub async fn delete_cdn_pull_zone(&self, pullzone_id: &str) -> VultrResult<()> {
        self.delete(&format!("/cdns/pull-zones/{}", pullzone_id))
            .await
    }

    /// Purge a CDN Pull Zone cache
    pub async fn purge_cdn_pull_zone(&self, pullzone_id: &str) -> VultrResult<()> {
        self.post_no_content(
            &format!("/cdns/pull-zones/{}/purge", pullzone_id),
            serde_json::json!({}),
        )
        .await
    }

    // =====================
    // CDN Push Zone Operations
    // =====================

    /// List all CDN Push Zones
    pub async fn list_cdn_push_zones(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<CdnPushZone>, Meta)> {
        let mut path = "/cdns/push-zones".to_string();
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
        let response: PushZonesResponse = self.get(&path).await?;
        let meta = Meta {
            total: Some(response.push_zones.len() as i32),
            links: None,
        };
        Ok((response.push_zones, meta))
    }

    /// Get a CDN Push Zone
    pub async fn get_cdn_push_zone(&self, pushzone_id: &str) -> VultrResult<CdnPushZone> {
        let response: PushZoneResponse = self
            .get(&format!("/cdns/push-zones/{}", pushzone_id))
            .await?;
        Ok(response.push_zone)
    }

    /// Create a CDN Push Zone
    pub async fn create_cdn_push_zone(
        &self,
        request: CreatePushZoneRequest,
    ) -> VultrResult<CdnPushZone> {
        let response: PushZoneResponse = self.post("/cdns/push-zones", request).await?;
        Ok(response.push_zone)
    }

    /// Update a CDN Push Zone
    pub async fn update_cdn_push_zone(
        &self,
        pushzone_id: &str,
        request: UpdatePushZoneRequest,
    ) -> VultrResult<CdnPushZone> {
        let response: PushZoneResponse = self
            .put(&format!("/cdns/push-zones/{}", pushzone_id), request)
            .await?;
        Ok(response.push_zone)
    }

    /// Delete a CDN Push Zone
    pub async fn delete_cdn_push_zone(&self, pushzone_id: &str) -> VultrResult<()> {
        self.delete(&format!("/cdns/push-zones/{}", pushzone_id))
            .await
    }

    // =====================
    // CDN Push Zone File Operations
    // =====================

    /// List files in a CDN Push Zone
    pub async fn list_cdn_push_zone_files(
        &self,
        pushzone_id: &str,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<CdnPushZoneFileMeta>, Meta)> {
        let mut path = format!("/cdns/push-zones/{}/files", pushzone_id);
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
        let response: PushZoneFilesResponse = self.get(&path).await?;
        let meta = Meta {
            total: Some(response.files.len() as i32),
            links: None,
        };
        Ok((response.files, meta))
    }

    /// Create a file upload endpoint for a CDN Push Zone
    pub async fn create_cdn_push_zone_file_endpoint(
        &self,
        pushzone_id: &str,
        request: CreateFileEndpointRequest,
    ) -> VultrResult<CdnUploadEndpoint> {
        let response: UploadEndpointResponse = self
            .post(&format!("/cdns/push-zones/{}/files", pushzone_id), request)
            .await?;
        Ok(response.upload_endpoint)
    }

    /// Get a file from a CDN Push Zone
    pub async fn get_cdn_push_zone_file(
        &self,
        pushzone_id: &str,
        file_name: &str,
    ) -> VultrResult<CdnPushZoneFile> {
        let response: PushZoneFileResponse = self
            .get(&format!(
                "/cdns/push-zones/{}/files/{}",
                pushzone_id, file_name
            ))
            .await?;
        Ok(response.file)
    }

    /// Delete a file from a CDN Push Zone
    pub async fn delete_cdn_push_zone_file(
        &self,
        pushzone_id: &str,
        file_name: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/cdns/push-zones/{}/files/{}",
            pushzone_id, file_name
        ))
        .await
    }

    // =====================
    // Container Registry Operations
    // =====================

    /// List all container registries
    pub async fn list_registries(&self) -> VultrResult<Vec<Registry>> {
        let response: RegistriesResponse = self.get("/registries").await?;
        Ok(response.registries)
    }

    /// Get a single container registry
    pub async fn get_registry(&self, registry_id: &str) -> VultrResult<Registry> {
        let response: RegistryResponse = self.get(&format!("/registry/{}", registry_id)).await?;
        Ok(response.registry)
    }

    /// Create a new container registry
    pub async fn create_registry(&self, request: CreateRegistryRequest) -> VultrResult<Registry> {
        let response: RegistryResponse = self.post("/registry", request).await?;
        Ok(response.registry)
    }

    /// Update a container registry
    pub async fn update_registry(
        &self,
        registry_id: &str,
        request: UpdateRegistryRequest,
    ) -> VultrResult<Registry> {
        let response: RegistryResponse = self
            .put(&format!("/registry/{}", registry_id), request)
            .await?;
        Ok(response.registry)
    }

    /// Delete a container registry
    pub async fn delete_registry(&self, registry_id: &str) -> VultrResult<()> {
        self.delete(&format!("/registry/{}", registry_id)).await
    }

    /// List repositories in a registry
    pub async fn list_registry_repositories(
        &self,
        registry_id: &str,
    ) -> VultrResult<Vec<RegistryRepository>> {
        let response: RepositoriesResponse = self
            .get(&format!("/registry/{}/repositories", registry_id))
            .await?;
        Ok(response.repositories)
    }

    /// Get a repository in a registry
    pub async fn get_registry_repository(
        &self,
        registry_id: &str,
        repository_image: &str,
    ) -> VultrResult<RegistryRepository> {
        let response: RepositoryResponse = self
            .get(&format!(
                "/registry/{}/repository/{}",
                registry_id, repository_image
            ))
            .await?;
        Ok(response.repository)
    }

    /// Delete a repository from a registry
    pub async fn delete_registry_repository(
        &self,
        registry_id: &str,
        repository_image: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/registry/{}/repository/{}",
            registry_id, repository_image
        ))
        .await
    }

    /// List artifacts in a repository
    pub async fn list_registry_artifacts(
        &self,
        registry_id: &str,
        repository_image: &str,
    ) -> VultrResult<Vec<RegistryArtifact>> {
        let response: ArtifactsResponse = self
            .get(&format!(
                "/registry/{}/repository/{}/artifacts",
                registry_id, repository_image
            ))
            .await?;
        Ok(response.artifacts)
    }

    /// Delete an artifact from a repository
    pub async fn delete_registry_artifact(
        &self,
        registry_id: &str,
        repository_image: &str,
        artifact_digest: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/registry/{}/repository/{}/artifact/{}",
            registry_id, repository_image, artifact_digest
        ))
        .await
    }

    /// Get Docker credentials for a registry
    pub async fn get_registry_docker_credentials(
        &self,
        registry_id: &str,
        expiry_seconds: Option<i64>,
        read_write: Option<bool>,
    ) -> VultrResult<RegistryDockerCredentials> {
        let mut path = format!("/registry/{}/docker-credentials", registry_id);
        let mut params = vec![];
        if let Some(e) = expiry_seconds {
            params.push(format!("expiry_seconds={}", e));
        }
        if let Some(rw) = read_write {
            params.push(format!("read_write={}", rw));
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        let response: DockerCredentialsResponse = self.get(&path).await?;
        Ok(response.credentials)
    }

    /// Get Kubernetes Docker credentials for a registry
    pub async fn get_registry_kubernetes_credentials(
        &self,
        registry_id: &str,
        expiry_seconds: Option<i64>,
        read_write: Option<bool>,
        base64_encode: Option<bool>,
    ) -> VultrResult<RegistryKubernetesCredentials> {
        let mut path = format!("/registry/{}/docker-credentials/kubernetes", registry_id);
        let mut params = vec![];
        if let Some(e) = expiry_seconds {
            params.push(format!("expiry_seconds={}", e));
        }
        if let Some(rw) = read_write {
            params.push(format!("read_write={}", rw));
        }
        if let Some(b64) = base64_encode {
            params.push(format!("base64_encode={}", b64));
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        let response: KubernetesCredentialsResponse = self.get(&path).await?;
        Ok(response.credentials)
    }

    /// List robot accounts in a registry
    pub async fn list_registry_robots(&self, registry_id: &str) -> VultrResult<Vec<RegistryRobot>> {
        let response: RobotsResponse = self
            .get(&format!("/registry/{}/robots", registry_id))
            .await?;
        Ok(response.robots)
    }

    /// Create a robot account
    pub async fn create_registry_robot(
        &self,
        registry_id: &str,
        request: CreateRobotRequest,
    ) -> VultrResult<RegistryRobot> {
        let response: RobotResponse = self
            .post(&format!("/registry/{}/robots", registry_id), request)
            .await?;
        Ok(response.robot)
    }

    /// Get a robot account
    pub async fn get_registry_robot(
        &self,
        registry_id: &str,
        robot_name: &str,
    ) -> VultrResult<RegistryRobot> {
        let response: RobotResponse = self
            .get(&format!("/registry/{}/robot/{}", registry_id, robot_name))
            .await?;
        Ok(response.robot)
    }

    /// Update a robot account
    pub async fn update_registry_robot(
        &self,
        registry_id: &str,
        robot_name: &str,
        request: UpdateRobotRequest,
    ) -> VultrResult<RegistryRobot> {
        let response: RobotResponse = self
            .put(
                &format!("/registry/{}/robot/{}", registry_id, robot_name),
                request,
            )
            .await?;
        Ok(response.robot)
    }

    /// Delete a robot account
    pub async fn delete_registry_robot(
        &self,
        registry_id: &str,
        robot_name: &str,
    ) -> VultrResult<()> {
        self.delete(&format!("/registry/{}/robot/{}", registry_id, robot_name))
            .await
    }

    /// List replications for a registry
    pub async fn list_registry_replications(
        &self,
        registry_id: &str,
    ) -> VultrResult<Vec<RegistryReplication>> {
        let response: ReplicationsResponse = self
            .get(&format!("/registry/{}/replications", registry_id))
            .await?;
        Ok(response.replications)
    }

    /// Create a replication for a registry
    pub async fn create_registry_replication(
        &self,
        registry_id: &str,
        request: CreateReplicationRequest,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/registry/{}/replication", registry_id), request)
            .await
    }

    /// Delete a replication from a registry
    pub async fn delete_registry_replication(
        &self,
        registry_id: &str,
        region: &str,
    ) -> VultrResult<()> {
        self.delete(&format!("/registry/{}/replication/{}", registry_id, region))
            .await
    }

    /// Get retention schedule for a registry
    pub async fn get_registry_retention_schedule(
        &self,
        registry_id: &str,
    ) -> VultrResult<Option<RegistryRetentionSchedule>> {
        let response: RetentionScheduleResponse = self
            .get(&format!("/registry/{}/retention/schedule", registry_id))
            .await?;
        Ok(response.schedule)
    }

    /// Update retention schedule for a registry
    pub async fn update_registry_retention_schedule(
        &self,
        registry_id: &str,
        request: UpdateRetentionScheduleRequest,
    ) -> VultrResult<()> {
        self.put::<serde_json::Value>(
            &format!("/registry/{}/retention/schedule", registry_id),
            request,
        )
        .await?;
        Ok(())
    }

    /// List retention rules for a registry
    pub async fn list_registry_retention_rules(
        &self,
        registry_id: &str,
    ) -> VultrResult<Vec<RegistryRetentionRule>> {
        let response: RetentionRulesResponse = self
            .get(&format!("/registry/{}/retention/rules", registry_id))
            .await?;
        Ok(response.rules)
    }

    /// Create a retention rule for a registry
    pub async fn create_registry_retention_rule(
        &self,
        registry_id: &str,
        request: CreateRetentionRuleRequest,
    ) -> VultrResult<RegistryRetentionRule> {
        let response: RetentionRuleResponse = self
            .post(
                &format!("/registry/{}/retention/rules", registry_id),
                request,
            )
            .await?;
        Ok(response.rule)
    }

    /// Update a retention rule
    pub async fn update_registry_retention_rule(
        &self,
        registry_id: &str,
        rule_id: i64,
        request: UpdateRetentionRuleRequest,
    ) -> VultrResult<RegistryRetentionRule> {
        let response: RetentionRuleResponse = self
            .put(
                &format!("/registry/{}/retention/rules/{}", registry_id, rule_id),
                request,
            )
            .await?;
        Ok(response.rule)
    }

    /// Delete a retention rule
    pub async fn delete_registry_retention_rule(
        &self,
        registry_id: &str,
        rule_id: i64,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/registry/{}/retention/rules/{}",
            registry_id, rule_id
        ))
        .await
    }

    /// List retention executions for a registry
    pub async fn list_registry_retention_executions(
        &self,
        registry_id: &str,
    ) -> VultrResult<Vec<RegistryRetentionExecution>> {
        let response: RetentionExecutionsResponse = self
            .get(&format!("/registry/{}/retention/executions", registry_id))
            .await?;
        Ok(response.executions)
    }

    /// Update user password for a registry
    pub async fn update_registry_user_password(
        &self,
        registry_id: &str,
        request: UpdateUserPasswordRequest,
    ) -> VultrResult<()> {
        self.put::<serde_json::Value>(&format!("/registry/{}/user/password", registry_id), request)
            .await?;
        Ok(())
    }

    /// List available registry regions
    pub async fn list_registry_regions(&self) -> VultrResult<Vec<RegistryRegion>> {
        let response: RegistryRegionsResponse = self.get("/registry/region/list").await?;
        Ok(response.regions)
    }

    /// List available registry plans
    pub async fn list_registry_plans(&self) -> VultrResult<Vec<RegistryPlan>> {
        let response: RegistryPlansResponse = self.get("/registry/plan/list").await?;
        // Convert the HashMap to Vec with the key as plan id
        Ok(response
            .plans
            .into_iter()
            .map(|(id, mut plan)| {
                plan.id = id;
                plan
            })
            .collect())
    }

    // =====================
    // Bare Metal Operations
    // =====================

    /// List all bare metal servers
    pub async fn list_bare_metals(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<BareMetal>, Meta)> {
        let mut path = "/bare-metals".to_string();
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

        let response: ListResponse<BareMetalsResponse> = self.get(&path).await?;
        Ok((response.data.bare_metals, response.meta))
    }

    /// Get a single bare metal server
    pub async fn get_bare_metal(&self, baremetal_id: &str) -> VultrResult<BareMetal> {
        let response: BareMetalResponse =
            self.get(&format!("/bare-metals/{}", baremetal_id)).await?;
        Ok(response.bare_metal)
    }

    /// Create a new bare metal server
    pub async fn create_bare_metal(
        &self,
        request: CreateBareMetalRequest,
    ) -> VultrResult<BareMetal> {
        let response: BareMetalResponse = self.post("/bare-metals", request).await?;
        Ok(response.bare_metal)
    }

    /// Update a bare metal server
    pub async fn update_bare_metal(
        &self,
        baremetal_id: &str,
        request: UpdateBareMetalRequest,
    ) -> VultrResult<BareMetal> {
        let response: BareMetalResponse = self
            .patch(&format!("/bare-metals/{}", baremetal_id), request)
            .await?;
        Ok(response.bare_metal)
    }

    /// Delete a bare metal server
    pub async fn delete_bare_metal(&self, baremetal_id: &str) -> VultrResult<()> {
        self.delete(&format!("/bare-metals/{}", baremetal_id)).await
    }

    /// Start a bare metal server
    pub async fn start_bare_metal(&self, baremetal_id: &str) -> VultrResult<()> {
        self.post_no_content(
            &format!("/bare-metals/{}/start", baremetal_id),
            serde_json::json!({}),
        )
        .await
    }

    /// Stop/halt a bare metal server
    pub async fn halt_bare_metal(&self, baremetal_id: &str) -> VultrResult<()> {
        self.post_no_content(
            &format!("/bare-metals/{}/halt", baremetal_id),
            serde_json::json!({}),
        )
        .await
    }

    /// Reboot a bare metal server
    pub async fn reboot_bare_metal(&self, baremetal_id: &str) -> VultrResult<()> {
        self.post_no_content(
            &format!("/bare-metals/{}/reboot", baremetal_id),
            serde_json::json!({}),
        )
        .await
    }

    /// Reinstall a bare metal server
    pub async fn reinstall_bare_metal(
        &self,
        baremetal_id: &str,
        request: ReinstallBareMetalRequest,
    ) -> VultrResult<BareMetal> {
        let response: BareMetalResponse = self
            .post(&format!("/bare-metals/{}/reinstall", baremetal_id), request)
            .await?;
        Ok(response.bare_metal)
    }

    /// Get bare metal bandwidth
    pub async fn get_bare_metal_bandwidth(
        &self,
        baremetal_id: &str,
    ) -> VultrResult<std::collections::HashMap<String, BandwidthData>> {
        let response: BareMetalBandwidthResponse = self
            .get(&format!("/bare-metals/{}/bandwidth", baremetal_id))
            .await?;
        Ok(response.bandwidth)
    }

    /// List bare metal IPv4 addresses
    pub async fn list_bare_metal_ipv4(
        &self,
        baremetal_id: &str,
    ) -> VultrResult<Vec<BareMetalIpv4>> {
        let response: BareMetalIpv4Response = self
            .get(&format!("/bare-metals/{}/ipv4", baremetal_id))
            .await?;
        Ok(response.ipv4s)
    }

    /// List bare metal IPv6 addresses
    pub async fn list_bare_metal_ipv6(
        &self,
        baremetal_id: &str,
    ) -> VultrResult<Vec<BareMetalIpv6>> {
        let response: BareMetalIpv6Response = self
            .get(&format!("/bare-metals/{}/ipv6", baremetal_id))
            .await?;
        Ok(response.ipv6s)
    }

    /// Set reverse DNS for bare metal IPv4
    pub async fn set_bare_metal_reverse_ipv4(
        &self,
        baremetal_id: &str,
        request: SetBareMetalReverseIpv4Request,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/bare-metals/{}/ipv4/reverse", baremetal_id),
            request,
        )
        .await
    }

    /// Set default reverse DNS for bare metal IPv4
    pub async fn set_bare_metal_default_reverse_ipv4(
        &self,
        baremetal_id: &str,
        request: SetBareMetalDefaultReverseIpv4Request,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/bare-metals/{}/ipv4/reverse/default", baremetal_id),
            request,
        )
        .await
    }

    /// Set reverse DNS for bare metal IPv6
    pub async fn set_bare_metal_reverse_ipv6(
        &self,
        baremetal_id: &str,
        request: SetBareMetalReverseIpv6Request,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/bare-metals/{}/ipv6/reverse", baremetal_id),
            request,
        )
        .await
    }

    /// Delete reverse DNS for bare metal IPv6
    pub async fn delete_bare_metal_reverse_ipv6(
        &self,
        baremetal_id: &str,
        ipv6: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/bare-metals/{}/ipv6/reverse/{}",
            baremetal_id, ipv6
        ))
        .await
    }

    /// Get bare metal user data
    pub async fn get_bare_metal_user_data(
        &self,
        baremetal_id: &str,
    ) -> VultrResult<BareMetalUserData> {
        let response: BareMetalUserDataResponse = self
            .get(&format!("/bare-metals/{}/user-data", baremetal_id))
            .await?;
        Ok(response.user_data)
    }

    /// Get available upgrades for a bare metal server
    pub async fn get_bare_metal_upgrades(
        &self,
        baremetal_id: &str,
    ) -> VultrResult<BareMetalUpgrades> {
        let response: BareMetalUpgradesResponse = self
            .get(&format!("/bare-metals/{}/upgrades", baremetal_id))
            .await?;
        Ok(response.upgrades)
    }

    /// Get VNC URL for a bare metal server
    pub async fn get_bare_metal_vnc(&self, baremetal_id: &str) -> VultrResult<BareMetalVnc> {
        let response: BareMetalVncResponse = self
            .get(&format!("/bare-metals/{}/vnc", baremetal_id))
            .await?;
        Ok(response.vnc)
    }

    /// List VPCs attached to a bare metal server
    pub async fn list_bare_metal_vpcs(&self, baremetal_id: &str) -> VultrResult<Vec<BareMetalVpc>> {
        let response: BareMetalVpcsResponse = self
            .get(&format!("/bare-metals/{}/vpcs", baremetal_id))
            .await?;
        Ok(response.vpcs)
    }

    /// Attach a VPC to a bare metal server
    pub async fn attach_bare_metal_vpc(
        &self,
        baremetal_id: &str,
        request: AttachBareMetalVpcRequest,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/bare-metals/{}/vpcs/attach", baremetal_id),
            request,
        )
        .await
    }

    /// Detach a VPC from a bare metal server
    pub async fn detach_bare_metal_vpc(
        &self,
        baremetal_id: &str,
        request: DetachBareMetalVpcRequest,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/bare-metals/{}/vpcs/detach", baremetal_id),
            request,
        )
        .await
    }

    /// List VPC2s attached to a bare metal server
    pub async fn list_bare_metal_vpc2s(
        &self,
        baremetal_id: &str,
    ) -> VultrResult<Vec<BareMetalVpc2>> {
        let response: BareMetalVpc2sResponse = self
            .get(&format!("/bare-metals/{}/vpc2", baremetal_id))
            .await?;
        Ok(response.vpcs)
    }

    /// Attach a VPC2 to a bare metal server
    pub async fn attach_bare_metal_vpc2(
        &self,
        baremetal_id: &str,
        request: AttachBareMetalVpc2Request,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/bare-metals/{}/vpc2/attach", baremetal_id),
            request,
        )
        .await
    }

    /// Detach a VPC2 from a bare metal server
    pub async fn detach_bare_metal_vpc2(
        &self,
        baremetal_id: &str,
        request: DetachBareMetalVpc2Request,
    ) -> VultrResult<()> {
        self.post_no_content(
            &format!("/bare-metals/{}/vpc2/detach", baremetal_id),
            request,
        )
        .await
    }

    /// Bulk halt bare metal servers
    pub async fn bulk_halt_bare_metals(&self, request: BulkBareMetalRequest) -> VultrResult<()> {
        self.post_no_content("/bare-metals/halt", request).await
    }

    /// Bulk start bare metal servers
    pub async fn bulk_start_bare_metals(&self, request: BulkBareMetalRequest) -> VultrResult<()> {
        self.post_no_content("/bare-metals/start", request).await
    }

    /// Bulk reboot bare metal servers
    pub async fn bulk_reboot_bare_metals(&self, request: BulkBareMetalRequest) -> VultrResult<()> {
        self.post_no_content("/bare-metals/reboot", request).await
    }

    // =====================
    // Account Operations
    // =====================

    /// Get account information
    pub async fn get_account(&self) -> VultrResult<Account> {
        let response: AccountResponse = self.get("/account").await?;
        Ok(response.account)
    }

    /// Get BGP information
    pub async fn get_account_bgp(&self) -> VultrResult<BgpInfo> {
        let response: BgpResponse = self.get("/account/bgp").await?;
        Ok(response.bgp_info)
    }

    /// Get account bandwidth usage
    pub async fn get_account_bandwidth(&self) -> VultrResult<AccountBandwidth> {
        let response: AccountBandwidthResponse = self.get("/account/bandwidth").await?;
        Ok(response.bandwidth)
    }

    // =====================
    // Billing Operations
    // =====================

    /// List billing history
    pub async fn list_billing_history(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<BillingHistory>, Option<Meta>)> {
        let mut path = "/billing/history".to_string();
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

        let response: BillingHistoryResponse = self.get(&path).await?;
        Ok((response.billing_history, response.meta))
    }

    /// List invoices
    pub async fn list_invoices(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<Invoice>, Option<Meta>)> {
        let mut path = "/billing/invoices".to_string();
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

        let response: InvoicesResponse = self.get(&path).await?;
        Ok((response.billing_invoices, response.meta))
    }

    /// Get a single invoice
    pub async fn get_invoice(&self, invoice_id: i64) -> VultrResult<Invoice> {
        let response: InvoiceResponse = self
            .get(&format!("/billing/invoices/{}", invoice_id))
            .await?;
        Ok(response.billing_invoice)
    }

    /// List invoice items
    pub async fn list_invoice_items(
        &self,
        invoice_id: i64,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<InvoiceItem>, Option<Meta>)> {
        let mut path = format!("/billing/invoices/{}/items", invoice_id);
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

        let response: InvoiceItemsResponse = self.get(&path).await?;
        Ok((response.invoice_items, response.meta))
    }

    /// List pending charges
    pub async fn list_pending_charges(&self) -> VultrResult<Vec<PendingCharge>> {
        let response: PendingChargesResponse = self.get("/billing/pending-charges").await?;
        Ok(response.pending_charges)
    }

    /// Get pending charges as CSV
    pub async fn get_pending_charges_csv(&self) -> VultrResult<String> {
        let url = format!("{}/billing/pending-charges/csv", API_BASE_URL);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            self.handle_error(response).await
        }
    }

    // =====================
    // Subaccount Operations
    // =====================

    /// List subaccounts
    pub async fn list_subaccounts(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<Subaccount>, Option<Meta>)> {
        let mut path = "/subaccounts".to_string();
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
        let response: SubaccountsResponse = self.get(&path).await?;
        Ok((response.subaccounts, response.meta))
    }

    /// Create a subaccount
    pub async fn create_subaccount(
        &self,
        request: CreateSubaccountRequest,
    ) -> VultrResult<Subaccount> {
        let response: SubaccountResponse = self.post("/subaccounts", request).await?;
        Ok(response.subaccount)
    }

    // =====================
    // User Operations
    // =====================

    /// List all users
    pub async fn list_users(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<User>, Option<Meta>)> {
        let mut path = "/users".to_string();
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

        let response: UsersResponse = self.get(&path).await?;
        Ok((response.users, response.meta))
    }

    /// Get a single user
    pub async fn get_user(&self, user_id: &str) -> VultrResult<User> {
        let response: UserResponse = self.get(&format!("/users/{}", user_id)).await?;
        Ok(response.user)
    }

    /// Create a new user
    pub async fn create_user(&self, request: CreateUserRequest) -> VultrResult<User> {
        let response: UserResponse = self.post("/users", request).await?;
        Ok(response.user)
    }

    /// Update a user
    pub async fn update_user(&self, user_id: &str, request: UpdateUserRequest) -> VultrResult<()> {
        self.patch_no_content(&format!("/users/{}", user_id), request)
            .await
    }

    /// Delete a user
    pub async fn delete_user(&self, user_id: &str) -> VultrResult<()> {
        self.delete(&format!("/users/{}", user_id)).await
    }

    /// List API keys for a user
    pub async fn list_user_api_keys(
        &self,
        user_id: &str,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<ApiKey>, Option<Meta>)> {
        let mut path = format!("/users/{}/apikeys", user_id);
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

        let response: ApiKeysResponse = self.get(&path).await?;
        Ok((response.api_keys, response.meta))
    }

    /// Create an API key for a user
    pub async fn create_user_api_key(
        &self,
        user_id: &str,
        request: CreateApiKeyRequest,
    ) -> VultrResult<ApiKey> {
        let response: ApiKeyResponse = self
            .post(&format!("/users/{}/apikeys", user_id), request)
            .await?;
        Ok(response.api_key)
    }

    /// Delete an API key for a user
    pub async fn delete_user_api_key(&self, user_id: &str, api_key_id: &str) -> VultrResult<()> {
        self.delete(&format!("/users/{}/apikeys/{}", user_id, api_key_id))
            .await
    }

    /// List IP whitelist for a user
    pub async fn list_user_ip_whitelist(
        &self,
        user_id: &str,
    ) -> VultrResult<Vec<IpWhitelistEntry>> {
        let response: IpWhitelistResponse = self
            .get(&format!("/users/{}/ip-whitelist", user_id))
            .await?;
        Ok(response.ip_whitelist)
    }

    /// Get a specific IP whitelist entry for a user
    pub async fn get_user_ip_whitelist_entry(
        &self,
        user_id: &str,
        subnet: &str,
        subnet_size: i32,
    ) -> VultrResult<IpWhitelistEntry> {
        let response: IpWhitelistEntryResponse = self
            .get(&format!(
                "/users/{}/ip-whitelist/entry?subnet={}&subnet_size={}",
                user_id, subnet, subnet_size
            ))
            .await?;
        Ok(response.ip_whitelist_entry)
    }

    /// Add an IP to user whitelist
    pub async fn add_user_ip_whitelist(
        &self,
        user_id: &str,
        request: AddIpWhitelistRequest,
    ) -> VultrResult<()> {
        self.post_no_content(&format!("/users/{}/ip-whitelist", user_id), request)
            .await
    }

    /// Delete an IP from user whitelist
    pub async fn delete_user_ip_whitelist(
        &self,
        user_id: &str,
        request: DeleteIpWhitelistRequest,
    ) -> VultrResult<()> {
        self.delete_with_body(&format!("/users/{}/ip-whitelist/entry", user_id), request)
            .await
    }

    // =====================
    // Private Network Operations (deprecated)
    // =====================

    /// List private networks
    pub async fn list_networks(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<Network>, Option<Meta>)> {
        let mut path = "/private-networks".to_string();
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
        let response: NetworksResponse = self.get(&path).await?;
        Ok((response.networks, Some(response.meta)))
    }

    /// Get a private network
    pub async fn get_network(&self, network_id: &str) -> VultrResult<Network> {
        let response: NetworkResponse = self
            .get(&format!("/private-networks/{}", network_id))
            .await?;
        Ok(response.network)
    }

    /// Create a private network
    pub async fn create_network(&self, request: CreateNetworkRequest) -> VultrResult<Network> {
        let response: NetworkResponse = self.post("/private-networks", request).await?;
        Ok(response.network)
    }

    /// Update a private network
    pub async fn update_network(
        &self,
        network_id: &str,
        request: UpdateNetworkRequest,
    ) -> VultrResult<()> {
        self.put::<serde_json::Value>(&format!("/private-networks/{}", network_id), request)
            .await?;
        Ok(())
    }

    /// Delete a private network
    pub async fn delete_network(&self, network_id: &str) -> VultrResult<()> {
        self.delete(&format!("/private-networks/{}", network_id))
            .await
    }

    // =====================
    // Storage Gateway Operations
    // =====================

    /// List storage gateways
    pub async fn list_storage_gateways(
        &self,
        per_page: Option<u32>,
        cursor: Option<&str>,
    ) -> VultrResult<(Vec<StorageGateway>, Meta)> {
        let mut path = "/storage-gateways".to_string();
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
        let response: StorageGatewaysResponse = self.get(&path).await?;
        Ok((response.storage_gateway, response.meta))
    }

    /// Get a storage gateway
    pub async fn get_storage_gateway(&self, gateway_id: &str) -> VultrResult<StorageGateway> {
        let response: StorageGatewayResponse = self
            .get(&format!("/storage-gateways/{}", gateway_id))
            .await?;
        Ok(response.storage_gateway)
    }

    /// Create a storage gateway
    pub async fn create_storage_gateway(
        &self,
        request: CreateStorageGatewayRequest,
    ) -> VultrResult<StorageGateway> {
        let response: StorageGatewayResponse = self.post("/storage-gateways", request).await?;
        Ok(response.storage_gateway)
    }

    /// Update a storage gateway
    pub async fn update_storage_gateway(
        &self,
        gateway_id: &str,
        request: UpdateStorageGatewayRequest,
    ) -> VultrResult<()> {
        self.put_no_content(&format!("/storage-gateways/{}", gateway_id), request)
            .await
    }

    /// Delete a storage gateway
    pub async fn delete_storage_gateway(&self, gateway_id: &str) -> VultrResult<()> {
        self.delete(&format!("/storage-gateways/{}", gateway_id))
            .await
    }

    /// Add export to storage gateway
    pub async fn add_storage_gateway_export(
        &self,
        gateway_id: &str,
        exports: Vec<StorageGatewayExport>,
    ) -> VultrResult<StorageGatewayExport> {
        let response: StorageGatewayExportResponse = self
            .post(
                &format!("/storage-gateways/{}/exports", gateway_id),
                exports,
            )
            .await?;
        Ok(response.export)
    }

    /// Delete storage gateway export
    pub async fn delete_storage_gateway_export(
        &self,
        gateway_id: &str,
        export_id: &str,
    ) -> VultrResult<()> {
        self.delete(&format!(
            "/storage-gateways/{}/exports/{}",
            gateway_id, export_id
        ))
        .await
    }

    // =====================
    // Vultr File System (VFS) Operations
    // =====================

    /// List VFS regions
    pub async fn list_vfs_regions(&self) -> VultrResult<Vec<VfsRegion>> {
        let response: VfsRegionsResponse = self.get("/vfs/regions").await?;
        Ok(response.regions)
    }

    /// List VFS subscriptions
    pub async fn list_vfs(&self) -> VultrResult<Vec<Vfs>> {
        let response: VfsListResponse = self.get("/vfs").await?;
        Ok(response.vfs)
    }

    /// Get a VFS subscription
    pub async fn get_vfs(&self, vfs_id: &str) -> VultrResult<Vfs> {
        self.get(&format!("/vfs/{}", vfs_id)).await
    }

    /// Create a VFS subscription
    pub async fn create_vfs(&self, request: CreateVfsRequest) -> VultrResult<Vfs> {
        self.post("/vfs", request).await
    }

    /// Update a VFS subscription
    pub async fn update_vfs(&self, vfs_id: &str, request: UpdateVfsRequest) -> VultrResult<Vfs> {
        self.put(&format!("/vfs/{}", vfs_id), request).await
    }

    /// Delete a VFS subscription
    pub async fn delete_vfs(&self, vfs_id: &str) -> VultrResult<()> {
        self.delete(&format!("/vfs/{}", vfs_id)).await
    }

    /// List VFS attachments
    pub async fn list_vfs_attachments(&self, vfs_id: &str) -> VultrResult<Vec<VfsAttachment>> {
        let response: VfsAttachmentsResponse = self
            .get(&format!("/vfs/{}/attachments", vfs_id))
            .await?;
        Ok(response.attachments)
    }

    /// Get a VFS attachment
    pub async fn get_vfs_attachment(
        &self,
        vfs_id: &str,
        vps_id: &str,
    ) -> VultrResult<VfsAttachment> {
        self.get(&format!("/vfs/{}/attachments/{}", vfs_id, vps_id))
            .await
    }

    /// Attach a VFS to a VPS
    pub async fn create_vfs_attachment(
        &self,
        vfs_id: &str,
        vps_id: &str,
    ) -> VultrResult<VfsAttachment> {
        self.put(&format!("/vfs/{}/attachments/{}", vfs_id, vps_id), serde_json::json!({}))
            .await
    }

    /// Delete a VFS attachment
    pub async fn delete_vfs_attachment(&self, vfs_id: &str, vps_id: &str) -> VultrResult<()> {
        self.delete(&format!("/vfs/{}/attachments/{}", vfs_id, vps_id))
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
