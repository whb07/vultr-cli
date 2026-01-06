//! Configuration management for the Vultr CLI

use crate::error::{VultrError, VultrResult};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application name for config directories
const APP_NAME: &str = "vultr-cli";

/// CLI Configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Default profile to use
    #[serde(default)]
    pub default_profile: String,
    /// Named profiles
    #[serde(default)]
    pub profiles: std::collections::HashMap<String, Profile>,
    /// Global settings
    #[serde(default)]
    pub settings: Settings,
}

/// Profile configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Profile {
    /// API key (stored in keyring, not in config file)
    #[serde(skip)]
    pub api_key: Option<String>,
    /// Default region for this profile
    pub default_region: Option<String>,
    /// Default output format
    pub output_format: Option<OutputFormat>,
}

/// Global settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Default output format
    #[serde(default)]
    pub output_format: OutputFormat,
    /// Whether to confirm destructive operations
    #[serde(default = "default_true")]
    pub confirm_destructive: bool,
    /// Default timeout for wait operations (seconds)
    #[serde(default = "default_timeout")]
    pub wait_timeout: u64,
    /// Polling interval for wait operations (seconds)
    #[serde(default = "default_poll_interval")]
    pub poll_interval: u64,

    /// HTTP settings (timeouts, retries)
    #[serde(default)]
    pub http: HttpSettings,
}

/// HTTP behavior settings.
///
/// These defaults are intentionally conservative: fast retry for transient errors,
/// and bounded exponential backoff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpSettings {
    /// Request timeout in seconds
    #[serde(default = "default_http_timeout")]
    pub timeout: u64,
    /// Maximum retries for transient failures (429/5xx/timeouts)
    #[serde(default = "default_http_max_retries")]
    pub max_retries: u32,
    /// Initial backoff in milliseconds
    #[serde(default = "default_http_backoff_initial_ms")]
    pub backoff_initial_ms: u64,
    /// Maximum backoff in milliseconds
    #[serde(default = "default_http_backoff_max_ms")]
    pub backoff_max_ms: u64,
}

fn default_http_timeout() -> u64 {
    30
}

fn default_http_max_retries() -> u32 {
    6
}

fn default_http_backoff_initial_ms() -> u64 {
    250
}

fn default_http_backoff_max_ms() -> u64 {
    10_000
}

impl Default for HttpSettings {
    fn default() -> Self {
        Self {
            timeout: default_http_timeout(),
            max_retries: default_http_max_retries(),
            backoff_initial_ms: default_http_backoff_initial_ms(),
            backoff_max_ms: default_http_backoff_max_ms(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_timeout() -> u64 {
    600 // 10 minutes
}

fn default_poll_interval() -> u64 {
    5
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Table,
            confirm_destructive: true,
            wait_timeout: 600,
            poll_interval: 5,
            http: HttpSettings::default(),
        }
    }
}

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "table" => Ok(OutputFormat::Table),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!("Unknown output format: {}. Valid options: table, json", s)),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Table => write!(f, "table"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

impl Config {
    /// Get the config directory path
    pub fn config_dir() -> VultrResult<PathBuf> {
        ProjectDirs::from("com", "vultr", APP_NAME)
            .map(|dirs| dirs.config_dir().to_path_buf())
            .ok_or_else(|| VultrError::ConfigError("Could not determine config directory".into()))
    }

    /// Get the config file path
    pub fn config_path() -> VultrResult<PathBuf> {
        Ok(Self::config_dir()?.join("config.json"))
    }

    /// Load configuration from disk
    pub fn load() -> VultrResult<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Save configuration to disk
    pub fn save(&self) -> VultrResult<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// Get the current profile
    pub fn current_profile(&self) -> Option<&Profile> {
        let profile_name = if self.default_profile.is_empty() {
            "default"
        } else {
            &self.default_profile
        };
        self.profiles.get(profile_name)
    }

    /// Get a mutable reference to the current profile, creating it if needed
    pub fn current_profile_mut(&mut self) -> &mut Profile {
        let profile_name = if self.default_profile.is_empty() {
            "default".to_string()
        } else {
            self.default_profile.clone()
        };
        self.profiles.entry(profile_name).or_default()
    }
}

/// Secure storage for API keys / tokens.
/// 
/// Default: OS keyring (macOS Keychain, Windows Credential Manager, Linux Secret Service).
/// Optional fallback: a local credentials.json file if `VULTR_CLI_INSECURE_FILE_SECRETS=1`
/// is set (useful for some CI environments).
pub struct SecureStorage;

impl SecureStorage {
    const SERVICE: &'static str = "vultr-cli";

    fn file_fallback_enabled() -> bool {
        std::env::var("VULTR_CLI_INSECURE_FILE_SECRETS")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    /// Get the credentials file path (fallback only)
    fn credentials_path() -> VultrResult<PathBuf> {
        Ok(Config::config_dir()?.join("credentials.json"))
    }

    /// Load credentials from file (fallback only)
    fn load_credentials_file() -> VultrResult<std::collections::HashMap<String, String>> {
        let path = Self::credentials_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let creds: std::collections::HashMap<String, String> = serde_json::from_str(&content)?;
            Ok(creds)
        } else {
            Ok(std::collections::HashMap::new())
        }
    }

    /// Save credentials to file with restricted permissions (fallback only)
    fn save_credentials_file(creds: &std::collections::HashMap<String, String>) -> VultrResult<()> {
        let path = Self::credentials_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(creds)?;
        std::fs::write(&path, &content)?;

        // Set restrictive permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&path)?.permissions();
            perms.set_mode(0o600); // Read/write for owner only
            std::fs::set_permissions(&path, perms)?;
        }

        Ok(())
    }

    /// Store a token (API key / PAT) securely for a profile
    pub fn store_token(profile: &str, token: &str) -> VultrResult<()> {
        // Prefer OS keyring
        match keyring::Entry::new(Self::SERVICE, profile) {
            Ok(entry) => {
                if let Err(e) = entry.set_password(token) {
                    if Self::file_fallback_enabled() {
                        let mut creds = Self::load_credentials_file()?;
                        creds.insert(profile.to_string(), token.to_string());
                        return Self::save_credentials_file(&creds);
                    }
                    return Err(VultrError::ConfigError(format!("Failed to store token in OS keyring: {e}")));
                }
                Ok(())
            }
            Err(e) => {
                if Self::file_fallback_enabled() {
                    let mut creds = Self::load_credentials_file()?;
                    creds.insert(profile.to_string(), token.to_string());
                    return Self::save_credentials_file(&creds);
                }
                Err(VultrError::ConfigError(format!("Failed to initialize OS keyring entry: {e}")))
            }
        }
    }

    /// Retrieve token for a profile
    pub fn get_token(profile: &str) -> VultrResult<Option<String>> {
        // Prefer OS keyring
        match keyring::Entry::new(Self::SERVICE, profile) {
            Ok(entry) => match entry.get_password() {
                Ok(v) => Ok(Some(v)),
                Err(e) => {
                    // If not found in keyring, optionally fall back
                    if Self::file_fallback_enabled() {
                        let creds = Self::load_credentials_file()?;
                        return Ok(creds.get(profile).cloned());
                    }
                    // Treat "not found" as None; other errors bubble up
                    // keyring error types vary, so use string matching conservatively
                    let msg = e.to_string().to_lowercase();
                    if msg.contains("not found") || msg.contains("no entry") || msg.contains("item not found") {
                        Ok(None)
                    } else {
                        Err(VultrError::ConfigError(format!("Failed to read token from OS keyring: {e}")))
                    }
                }
            },
            Err(e) => {
                if Self::file_fallback_enabled() {
                    let creds = Self::load_credentials_file()?;
                    return Ok(creds.get(profile).cloned());
                }
                Err(VultrError::ConfigError(format!("Failed to initialize OS keyring entry: {e}")))
            }
        }
    }

    /// Delete token for a profile
    pub fn delete_token(profile: &str) -> VultrResult<()> {
        match keyring::Entry::new(Self::SERVICE, profile) {
            Ok(entry) => {
                let _ = entry.delete_password(); // ignore "not found"
            }
            Err(e) => {
                if !Self::file_fallback_enabled() {
                    return Err(VultrError::ConfigError(format!("Failed to initialize OS keyring entry: {e}")));
                }
            }
        }

        if Self::file_fallback_enabled() {
            let mut creds = Self::load_credentials_file()?;
            creds.remove(profile);
            Self::save_credentials_file(&creds)?;
        }

        Ok(())
    }
}

/// Resolve the API token from various sources
/// Priority: 1. Command line arg, 2. Environment variable, 3. OS keyring (or fallback file)
pub fn resolve_api_key(cli_key: Option<&str>, profile: &str) -> VultrResult<Option<String>> {
    if let Some(k) = cli_key {
        return Ok(Some(k.to_string()));
    }
    if let Ok(k) = std::env::var("VULTR_API_KEY") {
        if !k.trim().is_empty() {
            return Ok(Some(k));
        }
    }
    SecureStorage::get_token(profile)
}

