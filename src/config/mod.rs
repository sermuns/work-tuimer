use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Configuration for issue tracker integrations (JIRA, Linear, GitHub, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub integrations: IntegrationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationConfig {
    /// Default tracker name when auto-detection is ambiguous
    #[serde(default)]
    pub default_tracker: Option<String>,

    /// Map of tracker name to tracker configuration
    #[serde(default)]
    pub trackers: HashMap<String, TrackerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrackerConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub base_url: String,
    /// Regex patterns to match ticket IDs for this tracker
    #[serde(default)]
    pub ticket_patterns: Vec<String>,
    /// URL template for browsing tickets: {base_url}, {ticket}
    #[serde(default)]
    pub browse_url: String,
    /// URL template for worklog page: {base_url}, {ticket}
    #[serde(default)]
    pub worklog_url: String,
}

impl Config {
    /// Load config from file, or return defaults if file doesn't exist
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path();

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)
                .context(format!("Failed to read config file: {:?}", config_path))?;
            let config: Config =
                toml::from_str(&contents).context("Failed to parse config TOML")?;
            return Ok(config);
        } else {
            return Ok(Config::default());
        }
    }

    /// Get config file path (~/.config/work-tuimer/config.toml)
    /// Respects XDG_CONFIG_HOME environment variable on Unix systems
    fn get_config_path() -> PathBuf {
        // On Unix systems (Linux/macOS), respect XDG_CONFIG_HOME
        #[cfg(unix)]
        {
            if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
                return PathBuf::from(xdg_config)
                    .join("work-tuimer")
                    .join("config.toml");
            }
            // Fall back to ~/.config if XDG_CONFIG_HOME is not set
            if let Some(home) = std::env::var_os("HOME") {
                return PathBuf::from(home)
                    .join(".config")
                    .join("work-tuimer")
                    .join("config.toml");
            }
        }

        // On Windows, use dirs::config_dir() which returns AppData/Roaming
        #[cfg(windows)]
        {
            if let Some(config_dir) = dirs::config_dir() {
                return config_dir.join("work-tuimer").join("config.toml");
            }
        }

        // Final fallback for any platform
        return PathBuf::from("./config.toml");
    }

    /// Check if any tracker integration is properly configured
    pub fn has_integrations(&self) -> bool {
        return self.integrations
            .trackers
            .values()
            .any(|tracker| return tracker.enabled && !tracker.base_url.is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.integrations.default_tracker, None);
        assert!(config.integrations.trackers.is_empty());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize");
        assert!(toml_str.contains("integrations"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
[integrations]
default_tracker = "my-jira"

[integrations.trackers.my-jira]
enabled = true
base_url = "https://test.atlassian.net"
ticket_patterns = ["^PROJ-\\d+$"]
browse_url = "{base_url}/browse/{ticket}"
worklog_url = "{base_url}/browse/{ticket}?focusedWorklogId=-1"
        "#;

        let config: Config = toml::from_str(toml_str).expect("Failed to deserialize");
        assert_eq!(
            config.integrations.default_tracker,
            Some("my-jira".to_string())
        );
        let tracker = config.integrations.trackers.get("my-jira").unwrap();
        assert_eq!(tracker.base_url, "https://test.atlassian.net");
        assert_eq!(tracker.ticket_patterns[0], "^PROJ-\\d+$");
    }

    #[test]
    fn test_tracker_config_defaults() {
        let tracker = TrackerConfig::default();
        assert!(!tracker.enabled);
        assert!(tracker.base_url.is_empty());
        assert!(tracker.ticket_patterns.is_empty());
    }
}
