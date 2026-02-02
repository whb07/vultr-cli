//! Log model types

use serde::{Deserialize, Serialize};

/// Log metadata
#[derive(Serialize, Deserialize)]
pub struct LogMetadata {
    /// UUID for the user that triggered the event
    pub user_id: Option<String>,
    /// Source IP address
    pub ip_address: Option<String>,
    /// Username (for login logs)
    pub username: Option<String>,
    /// HTTP status code (for API logs)
    pub http_status_code: Option<i32>,
    /// HTTP method
    pub method: Option<String>,
    /// Request path
    pub request_path: Option<String>,
    /// Request body
    pub request_body: Option<String>,
    /// Query parameters
    pub query_parameters: Option<String>,
}

/// Log line information
#[derive(Serialize, Deserialize)]
pub struct Log {
    /// Resource UUID
    pub resource_id: Option<String>,
    /// Resource type
    pub resource_type: Option<String>,
    /// Log level
    pub log_level: Option<String>,
    /// Log message
    pub message: Option<String>,
    /// UTC timestamp
    pub timestamp: Option<String>,
    /// Metadata
    pub metadata: Option<LogMetadata>,
}

/// Log list metadata
#[derive(Serialize, Deserialize)]
pub struct LogMeta {
    /// URL for next page
    pub next_page_url: String,
    /// Continue time (UTC)
    pub continue_time: String,
    /// Returned count
    pub returned_count: i32,
    /// Unreturned count
    pub unreturned_count: i32,
    /// Total count
    pub total_count: i32,
}

/// Response wrapper for logs
#[derive(Serialize, Deserialize)]
pub struct LogsResponse {
    pub logs: Vec<Log>,
    pub meta: LogMeta,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_deserialize() {
        let json = r#"{
            "resource_id": "abc",
            "resource_type": "instance",
            "log_level": "info",
            "message": "created",
            "timestamp": "2024-01-01T00:00:00Z",
            "metadata": {
                "user_id": "u-1",
                "ip_address": "192.0.2.1",
                "method": "POST"
            }
        }"#;
        let entry: Log = serde_json::from_str(json).unwrap();
        assert_eq!(entry.resource_type.as_deref(), Some("instance"));
        assert_eq!(entry.metadata.unwrap().method.as_deref(), Some("POST"));
    }
}
