//! Application model types

use serde::{Deserialize, Serialize};

/// Application information
#[derive(Serialize, Deserialize)]
pub struct Application {
    /// A unique ID for the application
    pub id: i32,
    /// The application name
    pub name: String,
    /// The short name of the application
    pub short_name: String,
    /// The deploy name (includes OS info)
    pub deploy_name: String,
    /// The application type (one-click, marketplace)
    #[serde(rename = "type")]
    pub app_type: String,
    /// The application vendor
    pub vendor: String,
    /// The image ID for marketplace apps
    pub image_id: String,
}

/// Marketplace app variable information
#[derive(Serialize, Deserialize)]
pub struct AppVariable {
    /// Variable name
    pub name: Option<String>,
    /// Variable description
    pub description: Option<String>,
    /// Required flag
    pub required: Option<bool>,
}

/// Response wrapper for app variables
#[derive(Serialize, Deserialize)]
pub struct AppVariablesResponse {
    pub variables: Vec<AppVariable>,
}

/// Response wrapper for applications list
#[derive(Serialize, Deserialize)]
pub struct ApplicationsResponse {
    pub applications: Vec<Application>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_deserialize() {
        let json = r#"{
            "id": 1,
            "name": "LEMP",
            "short_name": "lemp",
            "deploy_name": "LEMP on CentOS 6 x64",
            "type": "one-click",
            "vendor": "vultr",
            "image_id": ""
        }"#;
        let app: Application = serde_json::from_str(json).unwrap();
        assert_eq!(app.id, 1);
        assert_eq!(app.name, "LEMP");
        assert_eq!(app.short_name, "lemp");
        assert_eq!(app.deploy_name, "LEMP on CentOS 6 x64");
        assert_eq!(app.app_type, "one-click");
        assert_eq!(app.vendor, "vultr");
        assert_eq!(app.image_id, "");
    }

    #[test]
    fn test_application_deserialize_marketplace() {
        let json = r#"{
            "id": 1028,
            "name": "OpenLiteSpeed WordPress",
            "short_name": "openlitespeedwordpress",
            "deploy_name": "OpenLiteSpeed WordPress on Ubuntu 20.04 x64",
            "type": "marketplace",
            "vendor": "LiteSpeed_Technologies",
            "image_id": "openlitespeed-wordpress"
        }"#;
        let app: Application = serde_json::from_str(json).unwrap();
        assert_eq!(app.id, 1028);
        assert_eq!(app.name, "OpenLiteSpeed WordPress");
        assert_eq!(app.app_type, "marketplace");
        assert_eq!(app.vendor, "LiteSpeed_Technologies");
        assert_eq!(app.image_id, "openlitespeed-wordpress");
    }

    #[test]
    fn test_applications_response_deserialize() {
        let json = r#"{
            "applications": [
                {
                    "id": 1,
                    "name": "LEMP",
                    "short_name": "lemp",
                    "deploy_name": "LEMP on CentOS 6 x64",
                    "type": "one-click",
                    "vendor": "vultr",
                    "image_id": ""
                }
            ]
        }"#;
        let response: ApplicationsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.applications.len(), 1);
        assert_eq!(response.applications[0].name, "LEMP");
    }

    #[test]
    fn test_application_serialize() {
        let app = Application {
            id: 1,
            name: "LEMP".to_string(),
            short_name: "lemp".to_string(),
            deploy_name: "LEMP on CentOS 6 x64".to_string(),
            app_type: "one-click".to_string(),
            vendor: "vultr".to_string(),
            image_id: "".to_string(),
        };
        let json = serde_json::to_string(&app).unwrap();
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"name\":\"LEMP\""));
        assert!(json.contains("\"type\":\"one-click\""));
    }

    #[test]
    fn test_app_variable_deserialize() {
        let json = r#"{
            "name": "password",
            "description": "Admin password",
            "required": true
        }"#;
        let var: AppVariable = serde_json::from_str(json).unwrap();
        assert_eq!(var.name.as_deref(), Some("password"));
        assert_eq!(var.required, Some(true));
    }

    #[test]
    fn test_app_variables_response_deserialize() {
        let json = r#"{
            "variables": [
                {"name": "username", "required": false}
            ]
        }"#;
        let response: AppVariablesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.variables.len(), 1);
    }
}
