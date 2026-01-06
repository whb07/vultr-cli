//! Error types for the Vultr CLI

use thiserror::Error;

/// Main error type for the CLI
#[derive(Error, Debug)]
pub enum VultrError {
    #[error("API error: {status} - {message}")]
    ApiError {
        status: u16,
        message: String,
    },

    #[error("Authentication required. Run 'vultr-cli auth login' or set VULTR_API_KEY")]
    AuthenticationRequired,

    #[error("Invalid API key format")]
    InvalidApiKey,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Resource not found: {resource_type} with id '{id}'")]
    NotFound {
        resource_type: String,
        id: String,
    },

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
