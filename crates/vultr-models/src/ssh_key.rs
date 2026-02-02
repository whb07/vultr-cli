//! SSH Key model types

use serde::{Deserialize, Serialize};

/// SSH Key
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct CreateSshKeyRequest {
    /// Name for the SSH key
    pub name: String,
    /// The SSH public key content
    pub ssh_key: String,
}

/// Request to update an SSH key
#[derive(Serialize, Deserialize)]
pub struct UpdateSshKeyRequest {
    /// New name for the SSH key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New SSH key content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<String>,
}

/// Response wrapper for SSH key operations
#[derive(Serialize, Deserialize)]
pub struct SshKeyResponse {
    pub ssh_key: SshKey,
}

/// Response wrapper for SSH key list
#[derive(Serialize, Deserialize)]
pub struct SshKeysResponse {
    pub ssh_keys: Vec<SshKey>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_key_deserialize() {
        let json = r#"{"id":"key-123","date_created":"2024-01-01","name":"My SSH Key","ssh_key":"ssh-rsa AAAA..."}"#;
        let key: SshKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.id, "key-123");
        assert_eq!(key.name.unwrap(), "My SSH Key");
        assert_eq!(key.ssh_key.unwrap(), "ssh-rsa AAAA...");
    }

    #[test]
    fn test_ssh_key_deserialize_minimal() {
        let json = r#"{"id":"key-456"}"#;
        let key: SshKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.id, "key-456");
        assert!(key.name.is_none());
        assert!(key.ssh_key.is_none());
    }

    #[test]
    fn test_create_ssh_key_request_serialize() {
        let req = CreateSshKeyRequest {
            name: "Test Key".to_string(),
            ssh_key: "ssh-rsa AAAAB3...".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Test Key"));
        assert!(json.contains("ssh-rsa"));
    }

    #[test]
    fn test_update_ssh_key_request_default() {
        let req = UpdateSshKeyRequest::default();
        assert!(req.name.is_none());
        assert!(req.ssh_key.is_none());
    }

    #[test]
    fn test_update_ssh_key_request_partial() {
        let req = UpdateSshKeyRequest {
            name: Some("New Name".to_string()),
            ssh_key: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("New Name"));
        assert!(!json.contains("ssh_key"));
    }

    #[test]
    fn test_ssh_key_response_deserialize() {
        let json = r#"{"ssh_key":{"id":"key-789","name":"Primary Key"}}"#;
        let response: SshKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.ssh_key.id, "key-789");
    }

    #[test]
    fn test_ssh_keys_response_deserialize() {
        let json = r#"{"ssh_keys":[{"id":"key-1"},{"id":"key-2"}]}"#;
        let response: SshKeysResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.ssh_keys.len(), 2);
    }

    #[test]
    fn test_ssh_keys_response_empty() {
        let json = r#"{"ssh_keys":[]}"#;
        let response: SshKeysResponse = serde_json::from_str(json).unwrap();
        assert!(response.ssh_keys.is_empty());
    }
}
