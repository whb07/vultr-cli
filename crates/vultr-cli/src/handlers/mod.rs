//! Command handlers for the Vultr CLI
//!
//! Each submodule handles a specific resource type.

mod account;
mod application;
mod auth;
mod backup;
mod bare_metal;
mod billing;
mod block_storage;
mod cdn;
mod config;
mod database;
mod dns;
mod firewall;
mod inference;
mod instance;
mod iso;
mod kubernetes;
mod load_balancer;
mod log;
mod network;
mod object_storage;
mod reference;
mod registry;
mod reserved_ip;
mod snapshot;
mod ssh_key;
mod startup_script;
mod storage_gateway;
mod subaccount;
mod user;
mod vfs;
mod vpc;
mod vpc2;

pub use account::handle_account;
pub use application::handle_applications;
pub use auth::handle_auth;
pub use backup::handle_backup;
pub use bare_metal::handle_bare_metal;
pub use billing::handle_billing;
pub use block_storage::handle_block_storage;
pub use cdn::handle_cdn;
pub use config::handle_config;
pub use database::handle_database;
pub use dns::handle_dns;
pub use firewall::handle_firewall;
pub use inference::handle_inference;
pub use instance::handle_instance;
pub use iso::handle_iso;
pub use kubernetes::handle_kubernetes;
pub use load_balancer::handle_load_balancer;
pub use log::handle_logs;
pub use network::handle_private_network;
pub use object_storage::handle_object_storage;
pub use reference::{handle_os, handle_plans, handle_regions};
pub use registry::handle_registry;
pub use reserved_ip::handle_reserved_ip;
pub use snapshot::handle_snapshot;
pub use ssh_key::handle_ssh_key;
pub use startup_script::handle_startup_script;
pub use storage_gateway::handle_storage_gateway;
pub use subaccount::handle_subaccount;
pub use user::handle_user;
pub use vfs::handle_vfs;
pub use vpc::handle_vpc;
pub use vpc2::handle_vpc2;

// Re-export common utilities used by handlers
pub(crate) use common::*;

mod common {
    use dialoguer::Confirm;
    use std::io::Read;
    use std::path::PathBuf;
    use vultr_config::{VultrError, VultrResult};

    /// Read file contents if input starts with '@', otherwise return input as-is
    pub fn read_file_or_string(input: &str) -> VultrResult<String> {
        if let Some(path) = input.strip_prefix('@') {
            if path == "-" {
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf).map_err(|e| {
                    VultrError::InvalidInput(format!("Failed to read stdin: {}", e))
                })?;
                return Ok(buf);
            }
            let expanded = expand_tilde(path);
            std::fs::read_to_string(&expanded)
                .map_err(|e| VultrError::InvalidInput(format!("Failed to read '{}': {}", path, e)))
        } else {
            Ok(input.to_string())
        }
    }

    /// Read file as bytes if input starts with '@', otherwise return input bytes
    pub fn read_file_or_bytes(input: &str) -> VultrResult<Vec<u8>> {
        if let Some(path) = input.strip_prefix('@') {
            if path == "-" {
                let mut buf = Vec::new();
                std::io::stdin().read_to_end(&mut buf).map_err(|e| {
                    VultrError::InvalidInput(format!("Failed to read stdin: {}", e))
                })?;
                return Ok(buf);
            }
            let expanded = expand_tilde(path);
            std::fs::read(&expanded)
                .map_err(|e| VultrError::InvalidInput(format!("Failed to read '{}': {}", path, e)))
        } else {
            Ok(input.as_bytes().to_vec())
        }
    }

    fn expand_tilde(path: &str) -> PathBuf {
        let (rest, needs_expand) = if path == "~" {
            ("", true)
        } else if let Some(stripped) = path.strip_prefix("~/") {
            (stripped, true)
        } else if let Some(stripped) = path.strip_prefix("~\\") {
            (stripped, true)
        } else {
            (path, false)
        };

        if !needs_expand {
            return PathBuf::from(path);
        }

        if let Some(home) = home_dir() {
            if rest.is_empty() {
                home
            } else {
                home.join(rest)
            }
        } else {
            PathBuf::from(path)
        }
    }

    fn home_dir() -> Option<PathBuf> {
        if let Some(home) = std::env::var_os("HOME") {
            return Some(PathBuf::from(home));
        }
        if cfg!(windows) {
            if let Some(home) = std::env::var_os("USERPROFILE") {
                return Some(PathBuf::from(home));
            }
            if let (Some(drive), Some(path)) =
                (std::env::var_os("HOMEDRIVE"), std::env::var_os("HOMEPATH"))
            {
                let mut buf = PathBuf::from(drive);
                buf.push(path);
                return Some(buf);
            }
        }
        None
    }

    /// Prompt user to confirm a delete operation
    pub fn confirm_delete(resource_type: &str, id: &str) -> VultrResult<bool> {
        Ok(Confirm::new()
            .with_prompt(format!("Delete {} {}?", resource_type, id))
            .default(false)
            .interact()
            .unwrap_or(false))
    }
}
