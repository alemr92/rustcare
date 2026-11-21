pub mod booster;
pub mod cleaner;
pub mod privacy;

pub use booster::BoosterConfig;
pub use cleaner::CleanerConfig;
pub use privacy::PrivacyConfig;

pub struct RulesEngine;

impl RulesEngine {
    pub const CLEANER_RULES_TOML: &'static str = include_str!("../../../rules/cleaner_rules.toml");
    pub const PRIVACY_RULES_TOML: &'static str = include_str!("../../../rules/privacy_rules.toml");
    pub const BOOSTER_RULES_TOML: &'static str = include_str!("../../../rules/booster_rules.toml");

    pub fn load_cleaner_rules() -> anyhow::Result<CleanerConfig> {
        let config: CleanerConfig = toml::from_str(Self::CLEANER_RULES_TOML)?;
        Ok(config)
    }

    pub fn load_privacy_rules() -> anyhow::Result<PrivacyConfig> {
        let config: PrivacyConfig = toml::from_str(Self::PRIVACY_RULES_TOML)?;
        Ok(config)
    }

    pub fn load_booster_rules() -> anyhow::Result<BoosterConfig> {
        let config: BoosterConfig = toml::from_str(Self::BOOSTER_RULES_TOML)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules_parse_cleaner() {
        let rules = RulesEngine::load_cleaner_rules().expect("Fallo al parsear cleaner_rules.toml");
        assert!(!rules.groups.is_empty());
    }

    #[test]
    fn test_rules_parse_privacy() {
        let rules = RulesEngine::load_privacy_rules().expect("Fallo al parsear privacy_rules.toml");
        assert!(!rules.settings.is_empty());
    }

    #[test]
    fn test_rules_parse_booster() {
        let rules = RulesEngine::load_booster_rules().expect("Fallo al parsear booster_rules.toml");
        assert!(!rules.services.is_empty());
    }
}
