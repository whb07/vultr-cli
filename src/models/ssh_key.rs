//! SSH Key model types

use serde::{Deserialize, Serialize};

/// SSH Key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKey {
    /// Unique ID for the SSH key
    pub id: String,
    /// Date the key was created
    pub date_created: Option<String>,
    /// User-supplied name
    pub name: Option<String>,
    /// The SSH public key
    pub ssh_key: Option<String>,
}

/// Request to create an SSH key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSshKeyRequest {
    /// Name for the SSH key
    pub name: String,
    /// The SSH public key content
    pub ssh_key: String,
}

/// Request to update an SSH key
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSshKeyRequest {
    /// New name for the SSH key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New SSH key content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<String>,
}

/// Response wrapper for SSH key operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeyResponse {
    pub ssh_key: SshKey,
}

/// Response wrapper for SSH key list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeysResponse {
    pub ssh_keys: Vec<SshKey>,
}
