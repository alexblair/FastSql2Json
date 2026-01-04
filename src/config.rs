use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::from_str;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub app: AppConfig,
    pub file_intervals: Option<HashMap<String, u64>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub start_dir: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = from_str(&content)?;
        Ok(config)
    }
    
    pub fn get_interval(&self, file_path: &str) -> Option<u64> {
        if let Some(intervals) = &self.file_intervals {
            intervals.get(file_path).cloned()
        } else {
            None
        }
    }
}
