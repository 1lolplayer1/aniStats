use anyhow::Ok;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub monitor: MonitorConfig,
    pub sites: Vec<SiteConfig>,
}

#[derive(Deserialize, Clone)]
pub struct MonitorConfig {
    pub default_interval_secs: u64,
    pub request_timeout_secs: u64,
    pub history_limit: usize,
}

#[derive(Deserialize, Clone)]
pub struct SiteConfig {
    pub name: String,
    pub url: String,
    pub interval_secs: Option<u64>,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let text = std::fs::read_to_string("config.toml")?;
        let config = toml::from_str(&text)?;
        Ok(config)
    }
}