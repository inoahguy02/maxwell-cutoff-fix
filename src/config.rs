use std::fs::File;

use config::{Config, ConfigError, FileFormat};
use serde::{Deserialize, Serialize};

pub fn load() -> Result<DeviceConfig, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::new("config.json", FileFormat::Json))
        .build()?;

    settings.try_deserialize()
}

pub fn save(config: &DeviceConfig) -> anyhow::Result<()> {
    let file = File::create("config.json")?;
    serde_json::to_writer_pretty(file, config)?;
    Ok(())
}

pub fn create_default() -> anyhow::Result<()> {
    let file = File::create("config.json")?;
    serde_json::to_writer_pretty(file, &DeviceConfig::default())?;
    Ok(())
}

#[derive(Serialize, Deserialize, Default)]
pub struct DeviceConfig {
    pub devices: Vec<String>
}