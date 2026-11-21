use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub settings: Vec<PrivacySetting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySetting {
    pub id: String,
    pub category: String,
    pub name: String,
    pub description: String,
    pub hive: String,
    pub path: String,
    pub key: String,
    pub value_type: String,
    pub recommended_value: u32,
    pub default_windows_value: u32,
}
