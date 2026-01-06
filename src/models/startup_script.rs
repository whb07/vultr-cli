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

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;

    #[test]
    fn test_script_type_display() {
        assert_eq!(format!("{}", ScriptType::Boot), "boot");
        assert_eq!(format!("{}", ScriptType::Pxe), "pxe");
        assert_eq!(format!("{}", ScriptType::Unknown), "unknown");
    }

    #[test]
    fn test_script_type_from_str() {
        assert_eq!("boot".parse::<ScriptType>().unwrap(), ScriptType::Boot);
        assert_eq!("pxe".parse::<ScriptType>().unwrap(), ScriptType::Pxe);
    }

    #[test]
    fn test_script_type_from_str_invalid() {
        let result = "invalid".parse::<ScriptType>();
        assert!(result.is_err());
    }

    #[test]
    fn test_script_type_default() {
        assert_eq!(ScriptType::default(), ScriptType::Boot);
    }

    #[test]
    fn test_startup_script_decode_script() {
        let script_content = "#!/bin/bash\necho Hello";
        let encoded = base64::engine::general_purpose::STANDARD.encode(script_content);
        let script = StartupScript {
            id: "script-123".to_string(),
            name: Some("Test Script".to_string()),
            script_type: Some(ScriptType::Boot),
            script: Some(encoded),
            date_created: None,
            date_modified: None,
        };
        assert_eq!(script.decode_script().unwrap(), script_content);
    }

    #[test]
    fn test_startup_script_decode_script_none() {
        let script = StartupScript {
            id: "script-123".to_string(),
            name: None,
            script_type: None,
            script: None,
            date_created: None,
            date_modified: None,
        };
        assert!(script.decode_script().is_none());
    }

    #[test]
    fn test_startup_script_decode_script_invalid_base64() {
        let script = StartupScript {
            id: "script-123".to_string(),
            name: None,
            script_type: None,
            script: Some("not valid base64!!!".to_string()),
            date_created: None,
            date_modified: None,
        };
        assert!(script.decode_script().is_none());
    }

    #[test]
    fn test_startup_script_deserialize() {
        let json = r#"{"id":"script-abc","name":"Setup Script","type":"boot","date_modified":"2024-01-01"}"#;
        let script: StartupScript = serde_json::from_str(json).unwrap();
        assert_eq!(script.id, "script-abc");
        assert_eq!(script.name.unwrap(), "Setup Script");
        assert_eq!(script.script_type.unwrap(), ScriptType::Boot);
    }

    #[test]
    fn test_create_startup_script_request_new() {
        let req = CreateStartupScriptRequest::new(
            "Boot Script".to_string(),
            "#!/bin/bash",
            Some(ScriptType::Boot),
        );
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&req.script)
            .unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), "#!/bin/bash");
    }

    #[test]
    fn test_create_startup_script_request_serialize() {
        let req = CreateStartupScriptRequest {
            name: "Boot Script".to_string(),
            script: "IyEvYmluL2Jhc2g=".to_string(),
            script_type: Some("boot".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Boot Script"));
        assert!(json.contains("IyEvYmluL2Jhc2g="));
    }

    #[test]
    fn test_update_startup_script_request_default() {
        let req = UpdateStartupScriptRequest::default();
        assert!(req.name.is_none());
        assert!(req.script.is_none());
        assert!(req.script_type.is_none());
    }

    #[test]
    fn test_update_startup_script_request_with_raw_script() {
        let req = UpdateStartupScriptRequest::default().with_raw_script("#!/bin/bash");
        assert!(req.script.is_some());
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&req.script.unwrap())
            .unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), "#!/bin/bash");
    }

    #[test]
    fn test_script_type_unknown_variant() {
        let json = r#""cloud-init""#;
        let script_type: ScriptType = serde_json::from_str(json).unwrap();
        assert_eq!(script_type, ScriptType::Unknown);
    }

    #[test]
    fn test_startup_scripts_response_deserialize() {
        let json = r#"{"startup_scripts":[{"id":"s1"},{"id":"s2"}]}"#;
        let response: StartupScriptsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.startup_scripts.len(), 2);
    }
}
