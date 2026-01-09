//! Error types for the Vultr CLI

use thiserror::Error;

/// Main error type for the CLI
#[derive(Error, Debug)]
pub enum VultrError {
    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("Authentication required. Run 'vultr-cli auth login' or set VULTR_API_KEY")]
    AuthenticationRequired,

    #[error("Invalid or expired API key. Please check your API key and try again.")]
    InvalidApiKey,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Resource not found: {resource_type} with id '{id}'")]
    NotFound { resource_type: String, id: String },

    #[error("Rate limit exceeded. Please wait and try again.")]
    RateLimited,

    #[error("Operation timed out after {seconds} seconds")]
    Timeout { seconds: u64 },

    #[error("Operation cancelled by user")]
    Cancelled,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Input error: {0}")]
    DialoguerError(String),
}

impl From<dialoguer::Error> for VultrError {
    fn from(e: dialoguer::Error) -> Self {
        VultrError::DialoguerError(e.to_string())
    }
}

/// Result type alias for Vultr operations
pub type VultrResult<T> = Result<T, VultrError>;

/// API error response from Vultr
#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
    #[serde(default)]
    pub status: Option<u16>,
}

impl VultrError {
    /// Create an API error from status code and message
    pub fn api_error(status: u16, message: impl Into<String>) -> Self {
        VultrError::ApiError {
            status,
            message: message.into(),
        }
    }

    /// Create a not found error
    pub fn not_found(resource_type: impl Into<String>, id: impl Into<String>) -> Self {
        VultrError::NotFound {
            resource_type: resource_type.into(),
            id: id.into(),
        }
    }

    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            VultrError::AuthenticationRequired => 2,
            VultrError::InvalidApiKey => 2,
            VultrError::NotFound { .. } => 3,
            VultrError::RateLimited => 4,
            VultrError::Timeout { .. } => 5,
            VultrError::Cancelled => 6,
            VultrError::InvalidInput(_) => 7,
            VultrError::ApiError { status, .. } => {
                if *status >= 500 {
                    10
                } else if *status >= 400 {
                    11
                } else {
                    1
                }
            }
            _ => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_creation() {
        let err = VultrError::api_error(404, "Not found");
        match err {
            VultrError::ApiError { status, message } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Not found");
            }
            _ => panic!("Expected ApiError variant"),
        }
    }

    #[test]
    fn test_not_found_error_creation() {
        let err = VultrError::not_found("instance", "abc-123");
        match err {
            VultrError::NotFound { resource_type, id } => {
                assert_eq!(resource_type, "instance");
                assert_eq!(id, "abc-123");
            }
            _ => panic!("Expected NotFound variant"),
        }
    }

    #[test]
    fn test_exit_code_authentication_required() {
        assert_eq!(VultrError::AuthenticationRequired.exit_code(), 2);
    }

    #[test]
    fn test_exit_code_invalid_api_key() {
        assert_eq!(VultrError::InvalidApiKey.exit_code(), 2);
    }

    #[test]
    fn test_exit_code_not_found() {
        let err = VultrError::not_found("instance", "123");
        assert_eq!(err.exit_code(), 3);
    }

    #[test]
    fn test_exit_code_rate_limited() {
        assert_eq!(VultrError::RateLimited.exit_code(), 4);
    }

    #[test]
    fn test_exit_code_timeout() {
        let err = VultrError::Timeout { seconds: 60 };
        assert_eq!(err.exit_code(), 5);
    }

    #[test]
    fn test_exit_code_cancelled() {
        assert_eq!(VultrError::Cancelled.exit_code(), 6);
    }

    #[test]
    fn test_exit_code_invalid_input() {
        let err = VultrError::InvalidInput("bad input".to_string());
        assert_eq!(err.exit_code(), 7);
    }

    #[test]
    fn test_exit_code_server_error() {
        let err = VultrError::api_error(500, "Internal Server Error");
        assert_eq!(err.exit_code(), 10);
    }

    #[test]
    fn test_exit_code_client_error() {
        let err = VultrError::api_error(400, "Bad Request");
        assert_eq!(err.exit_code(), 11);
    }

    #[test]
    fn test_exit_code_other_status() {
        let err = VultrError::api_error(200, "OK");
        assert_eq!(err.exit_code(), 1);
    }

    #[test]
    fn test_error_display_api_error() {
        let err = VultrError::api_error(404, "Resource not found");
        assert_eq!(format!("{}", err), "API error: 404 - Resource not found");
    }

    #[test]
    fn test_error_display_not_found() {
        let err = VultrError::not_found("snapshot", "snap-123");
        assert_eq!(
            format!("{}", err),
            "Resource not found: snapshot with id 'snap-123'"
        );
    }

    #[test]
    fn test_error_display_timeout() {
        let err = VultrError::Timeout { seconds: 300 };
        assert_eq!(format!("{}", err), "Operation timed out after 300 seconds");
    }

    #[test]
    fn test_dialoguer_error_conversion() {
        let io_err = std::io::Error::other("test error");
        let dialoguer_err = dialoguer::Error::IO(io_err);
        let vultr_err: VultrError = dialoguer_err.into();
        match vultr_err {
            VultrError::DialoguerError(msg) => {
                assert!(msg.contains("test error"));
            }
            _ => panic!("Expected DialoguerError variant"),
        }
    }
}
