//! Serverless inference model types

use serde::{Deserialize, Serialize};

/// Serverless inference subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceSubscription {
    /// A unique ID for the subscription
    pub id: Option<String>,
    /// Date created
    pub date_created: Option<String>,
    /// User-supplied label
    pub label: Option<String>,
    /// API key for inference API
    pub api_key: Option<String>,
}

/// Usage metrics for chat completions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceChatUsage {
    /// Tokens used in current period
    pub current_tokens: Option<String>,
    /// Monthly token allotment
    pub monthly_allotment: Option<String>,
    /// Overage tokens in current period
    pub overage: Option<String>,
}

/// Usage metrics for audio generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceAudioUsage {
    /// Characters used for HD TTS
    pub tts_characters: Option<String>,
    /// Characters used for basic TTS
    pub tts_sm_characters: Option<String>,
}

/// Serverless inference usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceUsage {
    /// Chat usage metrics
    pub chat: Option<InferenceChatUsage>,
    /// Audio usage metrics
    pub audio: Option<InferenceAudioUsage>,
}

/// Response wrapper for inference list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceListResponse {
    pub subscriptions: Vec<InferenceSubscription>,
}

/// Response wrapper for single inference subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub subscription: InferenceSubscription,
}

/// Response wrapper for inference usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceUsageResponse {
    pub usage: InferenceUsage,
}

/// Request to create inference subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInferenceRequest {
    /// User-supplied label
    pub label: String,
}

/// Request to update inference subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInferenceRequest {
    /// New label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inference_subscription_deserialize() {
        let json = r#"{
            "id": "inf-123",
            "date_created": "2024-01-01T00:00:00Z",
            "label": "prod",
            "api_key": "secret"
        }"#;
        let sub: InferenceSubscription = serde_json::from_str(json).unwrap();
        assert_eq!(sub.id.as_deref(), Some("inf-123"));
        assert_eq!(sub.label.as_deref(), Some("prod"));
    }

    #[test]
    fn test_inference_usage_deserialize() {
        let json = r#"{
            "chat": {
                "current_tokens": "1000",
                "monthly_allotment": "10000",
                "overage": "0"
            },
            "audio": {
                "tts_characters": "500",
                "tts_sm_characters": "1200"
            }
        }"#;
        let usage: InferenceUsage = serde_json::from_str(json).unwrap();
        assert_eq!(usage.chat.unwrap().current_tokens.as_deref(), Some("1000"));
        assert_eq!(
            usage.audio.unwrap().tts_sm_characters.as_deref(),
            Some("1200")
        );
    }
}
