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
            println!("設定ファイルを作成しました: {}", config_path.display());
            Ok(config)
        }
    }

    fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("設定ファイルの読み込みに失敗しました: {}", path.display()))?;

        toml::from_str(&content).with_context(|| "設定ファイルの解析に失敗しました")
    }

    fn save(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("設定ディレクトリの作成に失敗しました: {}", parent.display())
            })?;
        }

        let content = toml::to_string_pretty(self)
            .with_context(|| "設定ファイルのシリアライズに失敗しました")?;

        fs::write(path, content)
            .with_context(|| format!("設定ファイルの書き込みに失敗しました: {}", path.display()))?;

        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("設定ディレクトリが見つかりません")?;

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
