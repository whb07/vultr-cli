//! Startup Script model types

use serde::{Deserialize, Serialize};

/// Startup script type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScriptType {
    Boot,
    Pxe,
    #[serde(other)]
    Unknown,
}

impl Default for ScriptType {
    fn default() -> Self {
        ScriptType::Boot
    }
}

impl std::fmt::Display for ScriptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptType::Boot => write!(f, "boot"),
            ScriptType::Pxe => write!(f, "pxe"),
            ScriptType::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for ScriptType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "boot" => Ok(ScriptType::Boot),
            "pxe" => Ok(ScriptType::Pxe),
            _ => Err(format!("Unknown script type: {}", s)),
        }
    }
}

/// Startup Script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupScript {
    /// Unique ID for the startup script
    pub id: String,
    /// Date the script was created
    pub date_created: Option<String>,
    /// Date the script was last modified
    pub date_modified: Option<String>,
    /// User-supplied name
    pub name: Option<String>,
    /// Base64-encoded script content
    pub script: Option<String>,
    /// Script type (boot or pxe)
    #[serde(rename = "type")]
    pub script_type: Option<ScriptType>,
}

impl StartupScript {
    /// Decode the script content from base64
    pub fn decode_script(&self) -> Option<String> {
        use base64::Engine;
        self.script.as_ref().and_then(|s| {
            base64::engine::general_purpose::STANDARD
                .decode(s)
                .ok()
                .and_then(|bytes| String::from_utf8(bytes).ok())
        })
    }
}

/// Request to create a startup script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStartupScriptRequest {
    /// Name for the script
    pub name: String,
    /// Base64-encoded script content
    pub script: String,
    /// Script type (boot or pxe)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub script_type: Option<String>,
}

impl CreateStartupScriptRequest {
    /// Create a new startup script request with raw (non-base64) content
    pub fn new(name: String, content: &str, script_type: Option<ScriptType>) -> Self {
        use base64::Engine;
        let encoded = base64::engine::general_purpose::STANDARD.encode(content);
        Self {
            name,
            script: encoded,
            script_type: script_type.map(|t| t.to_string()),
        }
    }
}

/// Request to update a startup script
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateStartupScriptRequest {
    /// New name for the script
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New base64-encoded script content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    /// New script type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub script_type: Option<String>,
}

impl UpdateStartupScriptRequest {
    /// Set the script content from raw (non-base64) content
    pub fn with_raw_script(mut self, content: &str) -> Self {
        use base64::Engine;
        self.script = Some(base64::engine::general_purpose::STANDARD.encode(content));
        self
    }
}

/// Response wrapper for startup script operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupScriptResponse {
    pub startup_script: StartupScript,
}

/// Response wrapper for startup script list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupScriptsResponse {
    pub startup_scripts: Vec<StartupScript>,
}
