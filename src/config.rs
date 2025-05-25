use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::errors::CliError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub logger: LoggerConfig,
    pub templates: TemplatesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggerConfig {
    pub level: String,
    pub color: bool,
    pub timestamp: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplatesConfig {
    #[serde(flatten)]
    pub custom_templates: std::collections::HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            logger: LoggerConfig {
                level: "info".to_string(),
                color: true,
                timestamp: true,
            },
            templates: TemplatesConfig {
                custom_templates: std::collections::HashMap::new(),
            },
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, CliError> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| crate::errors::config_error(format!("設定ファイルのパース失敗: {}", e)))?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), CliError> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::errors::config_error(format!("設定のシリアライズ失敗: {}", e)))?;
        fs::write(path, content)?;
        Ok(())
    }
}