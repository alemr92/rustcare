use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanerConfig {
    pub groups: Vec<CleanerRuleGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanerRuleGroup {
    pub id: String,
    pub name: String,
    pub description: String,
    pub targets: Vec<CleanerTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanerTarget {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(default = "default_true")]
    pub recursive: bool,
    #[serde(default = "default_patterns")]
    pub patterns: Vec<String>,
    pub description: String,
    #[serde(default)]
    pub requires_admin: bool,
}

fn default_true() -> bool {
    true
}

fn default_patterns() -> Vec<String> {
    vec!["*".to_string()]
}
