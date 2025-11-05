use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration for issue tracker integrations (JIRA, Linear, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub integrations: IntegrationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    /// Default tracker type when auto-detection is ambiguous
    #[serde(default = "default_tracker")]
    pub default_tracker: String,

    /// JIRA configuration
    #[serde(default)]
    pub jira: TrackerConfig,

    /// Linear configuration
    #[serde(default)]
    pub linear: TrackerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerConfig {
    pub enabled: bool,
    pub base_url: String,
    /// Regex patterns to match ticket IDs for this tracker
    #[serde(default)]
    pub ticket_patterns: Vec<String>,
    /// URL template for browsing tickets: {base_url}, {ticket}
    pub browse_url: String,
    /// URL template for worklog page: {base_url}, {ticket}
    #[serde(default)]
    pub worklog_url: String,
}

impl Default for TrackerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            base_url: String::new(),
            ticket_patterns: vec![],
            browse_url: String::new(),
            worklog_url: String::new(),
        }
    }
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            default_tracker: "jira".to_string(),
            jira: TrackerConfig {
                enabled: true,
                base_url: "https://mccomprojects.atlassian.net".to_string(),
                ticket_patterns: vec!["^[A-Z]+-\\d+$".to_string()],
                browse_url: "{base_url}/browse/{ticket}".to_string(),
                worklog_url: "{base_url}/browse/{ticket}?focusedWorklogId=-1".to_string(),
            },
            linear: TrackerConfig::default(),
        }
    }
}

fn default_tracker() -> String {
    "jira".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            integrations: IntegrationConfig::default(),
        }
    }
}

impl Config {
    /// Load config from file, or return defaults if file doesn't exist
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path();

        if config_path.exists() {
            let contents =
                fs::read_to_string(&config_path)
                    .context(format!("Failed to read config file: {:?}", config_path))?;
            let config: Config =
                toml::from_str(&contents).context("Failed to parse config TOML")?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Get config file path (~/.config/work-tuimer/config.toml)
    fn get_config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("work-tuimer").join("config.toml")
        } else {
            PathBuf::from("./config.toml")
        }
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create config directory: {:?}", parent))?;
        }

        let toml_str = toml::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(&config_path, toml_str)
            .context(format!("Failed to write config file: {:?}", config_path))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.integrations.default_tracker, "jira");
        assert!(config.integrations.jira.enabled);
        assert_eq!(
            config.integrations.jira.base_url,
            "https://mccomprojects.atlassian.net"
        );
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize");
        assert!(toml_str.contains("jira"));
        assert!(toml_str.contains("mccomprojects.atlassian.net"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
[integrations]
default_tracker = "jira"

[integrations.jira]
enabled = true
base_url = "https://test.atlassian.net"
ticket_patterns = ["^PROJ-\\d+$"]
browse_url = "{base_url}/browse/{ticket}"
worklog_url = "{base_url}/browse/{ticket}?focusedWorklogId=-1"
        "#;

        let config: Config = toml::from_str(toml_str).expect("Failed to deserialize");
        assert_eq!(config.integrations.jira.base_url, "https://test.atlassian.net");
        assert_eq!(config.integrations.jira.ticket_patterns[0], "^PROJ-\\d+$");
    }

    #[test]
    fn test_tracker_config_defaults() {
        let tracker = TrackerConfig::default();
        assert!(!tracker.enabled);
        assert!(tracker.base_url.is_empty());
        assert!(tracker.ticket_patterns.is_empty());
    }
}
