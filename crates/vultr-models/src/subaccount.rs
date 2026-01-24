//! Subaccount model types

use serde::{Deserialize, Serialize};

/// Subaccount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subaccount {
    /// Subaccount UUID
    pub id: Option<String>,
    /// Email address
    pub email: Option<String>,
    /// Subaccount name
    pub subaccount_name: Option<String>,
    /// Subaccount ID
    pub subaccount_id: Option<String>,
    /// Activated status
    pub activated: Option<bool>,
    /// Balance
    pub balance: Option<f64>,
    /// Pending charges
    pub pending_charges: Option<f64>,
}

/// Response wrapper for subaccounts list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountsResponse {
    pub subaccounts: Vec<Subaccount>,
    pub meta: Option<crate::Meta>,
}

/// Response wrapper for subaccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountResponse {
    pub subaccount: Subaccount,
}

/// Request to create a subaccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubaccountRequest {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subaccount_deserialize() {
        let json = r#"{
            "id": "sub-1",
            "email": "test@example.com",
            "subaccount_name": "Acme",
            "subaccount_id": "123",
            "activated": false,
            "balance": 0,
            "pending_charges": 0
        }"#;
        let sub: Subaccount = serde_json::from_str(json).unwrap();
        assert_eq!(sub.email.as_deref(), Some("test@example.com"));
        assert_eq!(sub.activated, Some(false));
    }
}
