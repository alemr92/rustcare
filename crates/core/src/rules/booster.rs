use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoosterConfig {
    pub booster: BoosterOptions,
    pub services: Vec<BoosterService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoosterOptions {
    #[serde(default)]
    pub enable_ram_clean: bool,
    #[serde(default)]
    pub enable_service_optimization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoosterService {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub safe_to_stop: bool,
    pub recommended_action: String,
}
