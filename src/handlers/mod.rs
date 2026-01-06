//! Command handlers for the Vultr CLI
//!
//! Each submodule handles a specific resource type.

mod auth;
mod block_storage;
mod firewall;
mod instance;
mod reference;
mod snapshot;
mod ssh_key;
mod startup_script;
mod vpc;

pub use auth::handle_auth;
pub use block_storage::handle_block_storage;
pub use firewall::handle_firewall;
pub use instance::handle_instance;
pub use reference::{handle_os, handle_plans, handle_regions};
pub use snapshot::handle_snapshot;
pub use ssh_key::handle_ssh_key;
pub use startup_script::handle_startup_script;
pub use vpc::handle_vpc;

// Re-export common utilities used by handlers
pub(crate) use common::*;

mod common {
    use crate::error::{VultrError, VultrResult};
    use dialoguer::Confirm;

    /// Read file contents if input starts with '@', otherwise return input as-is
    pub fn read_file_or_string(input: &str) -> VultrResult<String> {
        if let Some(path) = input.strip_prefix('@') {
            std::fs::read_to_string(path)
                .map_err(|e| VultrError::InvalidInput(format!("Failed to read '{}': {}", path, e)))
        } else {
            Ok(input.to_string())
        }
    }

    /// Read file as bytes if input starts with '@', otherwise return input bytes
    pub fn read_file_or_bytes(input: &str) -> VultrResult<Vec<u8>> {
        if let Some(path) = input.strip_prefix('@') {
            std::fs::read(path)
                .map_err(|e| VultrError::InvalidInput(format!("Failed to read '{}': {}", path, e)))
        } else {
            Ok(input.as_bytes().to_vec())
        }
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
