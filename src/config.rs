use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub target_directory: String,
    pub filename_format: String,
    pub entry_format: String,
    pub target_section: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_directory: "~/Documents/daily-notes".to_string(),
            filename_format: "YYYY-MM-DD.md".to_string(),
            entry_format: "[HH:mm:ss] {text}".to_string(),
            target_section: "### Tweets".to_string(),
        }
    }
}

impl Config {
    pub fn load_or_create() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            Self::load(&config_path)
        } else {
            let config = Self::default();
            config.save(&config_path)?;
            println!("Configuration file created: {}", config_path.display());
            Ok(config)
        }
    }

    fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        toml::from_str(&content).with_context(|| "Failed to parse config file")
    }

    fn save(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let content = toml::to_string_pretty(self)
            .with_context(|| "Failed to serialize config file")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Config directory not found")?;

        Ok(config_dir.join("tw").join("config.toml"))
    }

    pub fn expand_path(&self, path: &str) -> String {
        if path.starts_with("~/") {
            if let Some(home_dir) = dirs::home_dir() {
                return path.replacen("~", &home_dir.to_string_lossy(), 1);
            }
        }
        path.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.target_directory, "~/Documents/daily-notes");
        assert_eq!(config.filename_format, "YYYY-MM-DD.md");
        assert_eq!(config.entry_format, "[HH:mm:ss] {text}");
        assert_eq!(config.target_section, "### Tweets");
    }

    #[test]
    fn test_expand_path_with_tilde() {
        let config = Config::default();
        let path = "~/test/path";
        let expanded = config.expand_path(path);
        
        if let Some(home_dir) = dirs::home_dir() {
            let expected = format!("{}/test/path", home_dir.to_string_lossy());
            assert_eq!(expanded, expected);
        }
    }

    #[test]
    fn test_expand_path_without_tilde() {
        let config = Config::default();
        let path = "/absolute/path";
        let expanded = config.expand_path(path);
        assert_eq!(expanded, path);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(config.target_directory, deserialized.target_directory);
        assert_eq!(config.filename_format, deserialized.filename_format);
        assert_eq!(config.entry_format, deserialized.entry_format);
        assert_eq!(config.target_section, deserialized.target_section);
    }
}