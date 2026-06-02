use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub base_url: String,
    pub token: String,
}

impl Config {
    /// Load config from `~/.config/news-cli/config.toml`.
    pub fn load() -> anyhow::Result<Self> {
        let path = config_path();

        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&contents)
            .with_context(|| format!("failed to parse config file: {}", path.display()))?;

        // Strip trailing slash from base_url for consistency
        let base_url = config.base_url.trim_end_matches('/').to_string();

        Ok(Self {
            base_url,
            token: config.token,
        })
    }
}

pub fn config_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/home/user"));
    std::path::PathBuf::from(home)
        .join(".config")
        .join("news-cli")
        .join("config.toml")
}
