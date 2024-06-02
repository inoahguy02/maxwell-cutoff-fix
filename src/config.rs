use config::{Config, FileFormat};
use serde::{Deserialize, Serialize};
use std::fs::File;

use crate::MCF;

impl MCF {
    pub fn load(&self) -> anyhow::Result<MainConfig> {
        let path = self.extra_files_dir.join("config.json");
        let path_str = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to convert path to &str"))?;

        let settings = Config::builder()
            .add_source(config::File::new(path_str, FileFormat::Json))
            .build()?;

        Ok(settings.try_deserialize()?)
    }

    pub fn save(&self, config: &MainConfig) -> anyhow::Result<()> {
        let file = File::create(self.extra_files_dir.join("config.json"))?;
        serde_json::to_writer_pretty(file, config)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct MainConfig {
    pub devices: Vec<String>,

    #[serde(default = "default_num_of_kept_logs")]
    pub num_of_kept_logs: u16,
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
