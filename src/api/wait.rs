//! Wait/polling functionality for async operations

use crate::api::VultrClient;
use crate::error::{VultrError, VultrResult};
use crate::models::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Default wait timeout in seconds
pub const DEFAULT_TIMEOUT: u64 = 600;
/// Default poll interval in seconds
pub const DEFAULT_POLL_INTERVAL: u64 = 5;

/// Wait options for async operations
#[derive(Debug, Clone)]
pub struct WaitOptions {
    /// Maximum time to wait (seconds)
    pub timeout: u64,
    /// Time between status checks (seconds)
    pub poll_interval: u64,
    /// Whether to show progress
    pub show_progress: bool,
}

impl Default for WaitOptions {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            poll_interval: DEFAULT_POLL_INTERVAL,
            show_progress: true,
        }
    }
}

/// Wait for an instance to be ready
pub async fn wait_for_instance_ready(
    client: &VultrClient,
    instance_id: &str,
    options: &WaitOptions,
) -> VultrResult<Instance> {
    let pb = if options.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message(format!(
            "Waiting for instance {} to be ready...",
            instance_id
        ));
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let start = std::time::Instant::now();

    loop {
        let instance = client.get_instance(instance_id).await?;

        if let Some(ref pb) = pb {
            pb.set_message(format!(
                "Instance {} - status: {:?}, power: {:?}, server: {:?}",
                instance_id, instance.status, instance.power_status, instance.server_status
            ));
        }

        if instance.is_ready() {
            if let Some(pb) = pb {
                pb.finish_with_message(format!("Instance {} is ready!", instance_id));
            }
            return Ok(instance);
        }

        if start.elapsed().as_secs() > options.timeout {
            if let Some(pb) = pb {
                pb.finish_with_message(format!("Timeout waiting for instance {}", instance_id));
            }
            return Err(VultrError::Timeout {
                seconds: options.timeout,
            });
        }

        tokio::time::sleep(Duration::from_secs(options.poll_interval)).await;
    }
}

/// Wait for an instance to be stopped
pub async fn wait_for_instance_stopped(
    client: &VultrClient,
    instance_id: &str,
    options: &WaitOptions,
) -> VultrResult<Instance> {
    let pb = if options.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.yellow} {msg}")
                .unwrap(),
        );
        pb.set_message(format!("Waiting for instance {} to stop...", instance_id));
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let start = std::time::Instant::now();

    loop {
        let instance = client.get_instance(instance_id).await?;

        if let Some(ref pb) = pb {
            pb.set_message(format!(
                "Instance {} - power: {:?}",
                instance_id, instance.power_status
            ));
        }

        if instance.power_status == Some(PowerStatus::Stopped) {
            if let Some(pb) = pb {
                pb.finish_with_message(format!("Instance {} is stopped", instance_id));
            }
            return Ok(instance);
        }

        if start.elapsed().as_secs() > options.timeout {
            if let Some(pb) = pb {
                pb.finish_with_message(format!("Timeout waiting for instance {}", instance_id));
            }
            return Err(VultrError::Timeout {
                seconds: options.timeout,
            });
        }

        tokio::time::sleep(Duration::from_secs(options.poll_interval)).await;
    }
}

/// Wait for a snapshot to be complete
pub async fn wait_for_snapshot_complete(
    client: &VultrClient,
    snapshot_id: &str,
    options: &WaitOptions,
) -> VultrResult<Snapshot> {
    let pb = if options.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg}")
                .unwrap(),
        );
        pb.set_message(format!(
            "Waiting for snapshot {} to complete...",
            snapshot_id
        ));
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let start = std::time::Instant::now();

    loop {
        let snapshot = client.get_snapshot(snapshot_id).await?;

        if let Some(ref pb) = pb {
            pb.set_message(format!(
                "Snapshot {} - status: {:?}",
                snapshot_id, snapshot.status
            ));
        }

        if snapshot.is_ready() {
            if let Some(pb) = pb {
                pb.finish_with_message(format!("Snapshot {} is complete!", snapshot_id));
            }
            return Ok(snapshot);
        }

        if snapshot.status == Some(SnapshotStatus::Deleted) {
            if let Some(pb) = pb {
                pb.finish_with_message(format!("Snapshot {} was deleted", snapshot_id));
            }
            return Err(VultrError::not_found("snapshot", snapshot_id));
        }

        if start.elapsed().as_secs() > options.timeout {
            if let Some(pb) = pb {
                pb.finish_with_message(format!("Timeout waiting for snapshot {}", snapshot_id));
            }
            return Err(VultrError::Timeout {
                seconds: options.timeout,
            });
        }

        tokio::time::sleep(Duration::from_secs(options.poll_interval)).await;
    }
}

/// Wait for block storage to be active
pub async fn wait_for_block_storage_active(
    client: &VultrClient,
    block_id: &str,
    options: &WaitOptions,
) -> VultrResult<BlockStorage> {
    let pb = if options.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(format!(
            "Waiting for block storage {} to be active...",
            block_id
        ));
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let start = std::time::Instant::now();

    loop {
        let block = client.get_block_storage(block_id).await?;

        if let Some(ref pb) = pb {
            pb.set_message(format!(
                "Block storage {} - status: {:?}",
                block_id, block.status
            ));
        }

        if block.status == Some(BlockStorageStatus::Active) {
            if let Some(pb) = pb {
                pb.finish_with_message(format!("Block storage {} is active!", block_id));
            }
            return Ok(block);
        }

        if start.elapsed().as_secs() > options.timeout {
            if let Some(pb) = pb {
                pb.finish_with_message(format!("Timeout waiting for block storage {}", block_id));
            }
            return Err(VultrError::Timeout {
                seconds: options.timeout,
            });
        }

        tokio::time::sleep(Duration::from_secs(options.poll_interval)).await;
    }
}

/// Verify a resource was deleted by confirming it no longer exists
pub async fn verify_instance_deleted(
    client: &VultrClient,
    instance_id: &str,
    options: &WaitOptions,
) -> VultrResult<()> {
    let pb = if options.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.red} {msg}")
                .unwrap(),
        );
        pb.set_message(format!("Verifying instance {} was deleted...", instance_id));
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let start = std::time::Instant::now();

    loop {
        match client.get_instance(instance_id).await {
            Ok(_) => {
                // Instance still exists, keep waiting
                if let Some(ref pb) = pb {
                    pb.set_message(format!("Instance {} still exists, waiting...", instance_id));
                }
            }
            Err(VultrError::ApiError { status: 404, .. }) | Err(VultrError::NotFound { .. }) => {
                if let Some(pb) = pb {
                    pb.finish_with_message(format!("✓ Instance {} has been deleted", instance_id));
                }
                return Ok(());
            }
            Err(e) => {
                if let Some(pb) = pb {
                    pb.finish_with_message(format!("Error checking instance: {}", e));
                }
                return Err(e);
            }
        }

        if start.elapsed().as_secs() > options.timeout {
            if let Some(pb) = pb {
                pb.finish_with_message(format!(
                    "Timeout verifying deletion of instance {}",
                    instance_id
                ));
            }
            return Err(VultrError::Timeout {
                seconds: options.timeout,
            });
        }

        tokio::time::sleep(Duration::from_secs(options.poll_interval)).await;
    }
}

/// Verify a snapshot was deleted
pub async fn verify_snapshot_deleted(
    client: &VultrClient,
    snapshot_id: &str,
    options: &WaitOptions,
) -> VultrResult<()> {
    let pb = if options.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.red} {msg}")
                .unwrap(),
        );
        pb.set_message(format!("Verifying snapshot {} was deleted...", snapshot_id));
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let start = std::time::Instant::now();

    loop {
        match client.get_snapshot(snapshot_id).await {
            Ok(snapshot) => {
                if snapshot.status == Some(SnapshotStatus::Deleted) {
                    if let Some(pb) = pb {
                        pb.finish_with_message(format!(
                            "✓ Snapshot {} has been deleted",
                            snapshot_id
                        ));
                    }
                    return Ok(());
                }
                if let Some(ref pb) = pb {
                    pb.set_message(format!("Snapshot {} still exists, waiting...", snapshot_id));
                }
            }
            Err(VultrError::ApiError { status: 404, .. }) | Err(VultrError::NotFound { .. }) => {
                if let Some(pb) = pb {
                    pb.finish_with_message(format!("✓ Snapshot {} has been deleted", snapshot_id));
                }
                return Ok(());
            }
            Err(e) => {
                if let Some(pb) = pb {
                    pb.finish_with_message(format!("Error checking snapshot: {}", e));
                }
                return Err(e);
            }
        }

        if start.elapsed().as_secs() > options.timeout {
            if let Some(pb) = pb {
                pb.finish_with_message(format!(
                    "Timeout verifying deletion of snapshot {}",
                    snapshot_id
                ));
            }
            return Err(VultrError::Timeout {
                seconds: options.timeout,
            });
        }

        tokio::time::sleep(Duration::from_secs(options.poll_interval)).await;
    }
}

/// Verify block storage was deleted
pub async fn verify_block_storage_deleted(
    client: &VultrClient,
    block_id: &str,
    options: &WaitOptions,
) -> VultrResult<()> {
    let pb = if options.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.red} {msg}")
                .unwrap(),
        );
        pb.set_message(format!(
            "Verifying block storage {} was deleted...",
            block_id
        ));
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let start = std::time::Instant::now();

    loop {
        match client.get_block_storage(block_id).await {
            Ok(_) => {
                if let Some(ref pb) = pb {
                    pb.set_message(format!(
                        "Block storage {} still exists, waiting...",
                        block_id
                    ));
                }
            }
            Err(VultrError::ApiError { status: 404, .. }) | Err(VultrError::NotFound { .. }) => {
                if let Some(pb) = pb {
                    pb.finish_with_message(format!(
                        "✓ Block storage {} has been deleted",
                        block_id
                    ));
                }
                return Ok(());
            }
            Err(e) => {
                if let Some(pb) = pb {
                    pb.finish_with_message(format!("Error checking block storage: {}", e));
                }
                return Err(e);
            }
        }

        if start.elapsed().as_secs() > options.timeout {
            if let Some(pb) = pb {
                pb.finish_with_message(format!(
                    "Timeout verifying deletion of block storage {}",
                    block_id
                ));
            }
            return Err(VultrError::Timeout {
                seconds: options.timeout,
            });
        }

        tokio::time::sleep(Duration::from_secs(options.poll_interval)).await;
    }
}

/// Verify a bare metal server was deleted
pub async fn verify_bare_metal_deleted(
    client: &VultrClient,
    bare_metal_id: &str,
    options: &WaitOptions,
) -> VultrResult<()> {
    verify_deleted_generic(
        options,
        format!(
            "Verifying bare metal server {} was deleted...",
            bare_metal_id
        ),
        format!("✓ Bare metal server {} has been deleted", bare_metal_id),
        || async { client.get_bare_metal(bare_metal_id).await.map(|_| ()) },
    )
    .await
}

/// Verify an SSH key was deleted
pub async fn verify_ssh_key_deleted(
    client: &VultrClient,
    key_id: &str,
    options: &WaitOptions,
) -> VultrResult<()> {
    verify_deleted_generic(
        options,
        format!("Verifying SSH key {} was deleted...", key_id),
        format!("✓ SSH key {} has been deleted", key_id),
        || async { client.get_ssh_key(key_id).await.map(|_| ()) },
    )
    .await
}

/// Verify a startup script was deleted
pub async fn verify_startup_script_deleted(
    client: &VultrClient,
    script_id: &str,
    options: &WaitOptions,
) -> VultrResult<()> {
    verify_deleted_generic(
        options,
        format!("Verifying startup script {} was deleted...", script_id),
        format!("✓ Startup script {} has been deleted", script_id),
        || async { client.get_startup_script(script_id).await.map(|_| ()) },
    )
    .await
}

/// Verify a firewall group was deleted
pub async fn verify_firewall_group_deleted(
    client: &VultrClient,
    group_id: &str,
    options: &WaitOptions,
) -> VultrResult<()> {
    verify_deleted_generic(
        options,
        format!("Verifying firewall group {} was deleted...", group_id),
        format!("✓ Firewall group {} has been deleted", group_id),
        || async { client.get_firewall_group(group_id).await.map(|_| ()) },
    )
    .await
}

/// Verify a firewall rule was deleted
pub async fn verify_firewall_rule_deleted(
    client: &VultrClient,
    group_id: &str,
    rule_id: i32,
    options: &WaitOptions,
) -> VultrResult<()> {
    verify_deleted_generic(
        options,
        format!(
            "Verifying firewall rule {} in group {} was deleted...",
            rule_id, group_id
        ),
        format!(
            "✓ Firewall rule {} has been deleted from group {}",
            rule_id, group_id
        ),
        || async {
            client
                .get_firewall_rule(group_id, rule_id)
                .await
                .map(|_| ())
        },
    )
    .await
}

/// Generic helper for verifying deletion by polling GET until it 404s.
async fn verify_deleted_generic<F, Fut>(
    options: &WaitOptions,
    start_msg: String,
    ok_msg: String,
    getter: F,
) -> VultrResult<()>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = VultrResult<()>>,
{
    let pb = if options.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.red} {msg}")
                .unwrap(),
        );
        pb.set_message(start_msg);
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let start = std::time::Instant::now();
    loop {
        match getter().await {
            Ok(_) => {
                if let Some(ref pb) = pb {
                    pb.set_message("Resource still exists, waiting...".to_string());
                }
            }
            Err(VultrError::ApiError { status: 404, .. }) | Err(VultrError::NotFound { .. }) => {
                if let Some(pb) = pb {
                    pb.finish_with_message(ok_msg);
                }
                return Ok(());
            }
            Err(e) => {
                if let Some(pb) = pb {
                    pb.finish_with_message(format!("Error checking resource: {}", e));
                }
                return Err(e);
            }
        }

        if start.elapsed().as_secs() > options.timeout {
            if let Some(pb) = pb {
                pb.finish_with_message("Timeout verifying deletion".to_string());
            }
            return Err(VultrError::Timeout {
                seconds: options.timeout,
            });
        }

        tokio::time::sleep(Duration::from_secs(options.poll_interval)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wait_options_default() {
        let opts = WaitOptions::default();
        assert_eq!(opts.timeout, DEFAULT_TIMEOUT);
        assert_eq!(opts.poll_interval, DEFAULT_POLL_INTERVAL);
        assert!(opts.show_progress);
    }

    #[test]
    fn test_wait_options_custom() {
        let opts = WaitOptions {
            timeout: 300,
            poll_interval: 10,
            show_progress: false,
        };
        assert_eq!(opts.timeout, 300);
        assert_eq!(opts.poll_interval, 10);
        assert!(!opts.show_progress);
    }

    #[test]
    fn test_default_timeout_value() {
        assert_eq!(DEFAULT_TIMEOUT, 600);
    }

    #[test]
    fn test_default_poll_interval_value() {
        assert_eq!(DEFAULT_POLL_INTERVAL, 5);
    }

    #[test]
    fn test_wait_options_clone() {
        let opts = WaitOptions {
            timeout: 120,
            poll_interval: 3,
            show_progress: true,
        };
        let cloned = opts.clone();
        assert_eq!(cloned.timeout, 120);
        assert_eq!(cloned.poll_interval, 3);
    }

    #[test]
    fn test_wait_options_debug() {
        let opts = WaitOptions::default();
        let debug_str = format!("{:?}", opts);
        assert!(debug_str.contains("timeout"));
        assert!(debug_str.contains("poll_interval"));
    }
}
