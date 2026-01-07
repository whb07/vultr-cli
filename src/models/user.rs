//! User model types

use serde::{Deserialize, Serialize};

use crate::models::Meta;

/// User account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: String,
    /// User name
    pub name: Option<String>,
    /// User email
    pub email: Option<String>,
    /// Whether API is enabled
    pub api_enabled: Option<bool>,
    /// Access control list
    #[serde(default)]
    pub acls: Vec<String>,
}

/// Response wrapper for user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

/// Response wrapper for user list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<User>,
    #[serde(default)]
    pub meta: Option<Meta>,
}

/// Request to create a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// User email
    pub email: String,
    /// User name
    pub name: String,
    /// User password
    pub password: String,
    /// Whether API is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_enabled: Option<bool>,
    /// Access control list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acls: Option<Vec<String>>,
}

/// Request to update a user
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateUserRequest {
    /// User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Whether API is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_enabled: Option<bool>,
    /// Access control list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acls: Option<Vec<String>>,
}

/// API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// API key ID
    pub id: Option<String>,
    /// API key name
    pub name: Option<String>,
    /// The API key value (only shown on creation)
    pub api_key: Option<String>,
    /// Date created
    pub date_created: Option<String>,
}

/// Response wrapper for API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    pub api_key: ApiKey,
}

/// Response wrapper for API key list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeysResponse {
    pub api_keys: Vec<ApiKey>,
    #[serde(default)]
    pub meta: Option<Meta>,
}

/// Request to create an API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    /// API key name
    pub name: String,
}

/// IP whitelist entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpWhitelistEntry {
    /// Subnet (IP address)
    pub subnet: Option<String>,
    /// Subnet size (CIDR mask)
    pub subnet_size: Option<i32>,
    /// Date added
    pub date_added: Option<String>,
    /// IP type (v4 or v6)
    pub ip_type: Option<String>,
}

/// Response wrapper for IP whitelist entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpWhitelistEntryResponse {
    pub ip_whitelist_entry: IpWhitelistEntry,
}

/// Response wrapper for IP whitelist list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpWhitelistResponse {
    pub ip_whitelist: Vec<IpWhitelistEntry>,
    #[serde(default)]
    pub meta: Option<Meta>,
}

/// Request to add an IP to whitelist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddIpWhitelistRequest {
    /// Subnet (IP address)
    pub subnet: String,
    /// Subnet size (CIDR mask)
    pub subnet_size: i32,
}

/// Request to delete an IP from whitelist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteIpWhitelistRequest {
    /// Subnet (IP address)
    pub subnet: String,
    /// Subnet size (CIDR mask)
    pub subnet_size: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_deserialize() {
        let json = r#"{"id":"user-123","name":"John Doe","email":"john@example.com","api_enabled":true,"acls":["manage_users","billing"]}"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, "user-123");
        assert_eq!(user.name.unwrap(), "John Doe");
        assert!(user.api_enabled.unwrap());
        assert_eq!(user.acls.len(), 2);
    }

    #[test]
    fn test_create_user_request_serialize() {
        let req = CreateUserRequest {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            password: "secret123".to_string(),
            api_enabled: Some(true),
            acls: Some(vec!["billing".to_string()]),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test@example.com"));
        assert!(json.contains("secret123"));
    }

    #[test]
    fn test_api_key_deserialize() {
        let json = r#"{"id":"key-123","name":"My API Key","api_key":"ABC123","date_created":"2024-01-01"}"#;
        let key: ApiKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.id.unwrap(), "key-123");
        assert_eq!(key.api_key.unwrap(), "ABC123");
    }

    #[test]
    fn test_ip_whitelist_entry_deserialize() {
        let json =
            r#"{"subnet":"8.8.8.0","subnet_size":24,"date_added":"2024-01-01","ip_type":"v4"}"#;
        let entry: IpWhitelistEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.subnet.unwrap(), "8.8.8.0");
        assert_eq!(entry.subnet_size.unwrap(), 24);
    }
}
