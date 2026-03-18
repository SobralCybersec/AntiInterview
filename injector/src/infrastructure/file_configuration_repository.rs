use crate::application::ConfigurationRepository;
use crate::domain::Configuration;
use anyhow::{Context, Result};
use std::{fs, path::PathBuf};
use tracing::{debug, error};

pub struct FileConfigurationRepository;

impl FileConfigurationRepository {
    pub fn new() -> Self {
        Self
    }

    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("AntiInterview");
        path.push("config.toml");
        path
    }
}

impl ConfigurationRepository for FileConfigurationRepository {
    fn load(&self) -> Result<Configuration> {
        let config_path = Self::config_path();

        if !config_path.exists() {
            let default_config = Configuration::default();
            if let Err(e) = self.save(&default_config) {
                error!("Failed to save default config: {:?}", e);
            }
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_path)
            .context("Failed to read configuration file")?;

        let config: Configuration = toml::from_str(&content)
            .context("Failed to parse configuration")?;

        debug!("Configuration loaded from {:?}", config_path);
        Ok(config)
    }

    fn save(&self, config: &Configuration) -> Result<()> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create configuration directory")?;
        }

        let content = toml::to_string_pretty(config)
            .context("Failed to serialize configuration")?;

        fs::write(&config_path, content)
            .context("Failed to write configuration file")?;

        debug!("Configuration saved to {:?}", config_path);
        Ok(())
    }
}
