use config::{Config, ConfigError, FileFormat};
use serde::{Deserialize, Serialize};
use std::fs::File;

pub fn load() -> Result<MainConfig, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::new("config.json", FileFormat::Json)) // Looks for file in env::current_dir()
        .build()?;
    settings.try_deserialize()
}

pub fn save(config: &MainConfig) -> anyhow::Result<()> {
    let file = File::create("config.json")?; // Places file beside exe, not in env::current_dir()
    serde_json::to_writer_pretty(file, config)?;
    Ok(())
}

pub fn create_default() -> anyhow::Result<()> {
    let file = File::create("config.json")?;
    serde_json::to_writer_pretty(file, &MainConfig::default())?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct MainConfig {
    pub devices: Vec<String>,

    #[serde(default = "default_num_of_kept_logs")]
    num_of_kept_logs: u16,
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            devices: Default::default(),
            num_of_kept_logs: default_num_of_kept_logs(),
        }
    }
}

fn default_num_of_kept_logs() -> u16 {
    10
}
