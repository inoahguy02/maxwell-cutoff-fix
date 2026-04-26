use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashSet, env, fs::File, io, path::PathBuf};

pub struct ConfigManager {
    pub cfg: RefCell<MainConfig>,
    pub cfg_file_path: PathBuf,
}

impl ConfigManager {
    pub fn init() -> anyhow::Result<Self> {
        // generate cfg path
        let cfg_file_path = env::current_exe()
            .context("Failed to get path to executable")?
            .parent()
            .context("missing exe parent dir")?
            .join("maxwell-fix-config.json");

        Ok(Self {
            cfg: RefCell::new(MainConfig::from_path(&cfg_file_path)?),
            cfg_file_path,
        })
    }

    pub fn reload_config(&self) -> anyhow::Result<()> {
        *self.cfg.borrow_mut() = MainConfig::from_path(&self.cfg_file_path)?;
        Ok(())
    }

    pub fn save_config(&self) -> anyhow::Result<()> {
        let file = File::create(&self.cfg_file_path)?;
        serde_json::to_writer_pretty(file, &*self.cfg.borrow())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct MainConfig {
    pub devices: HashSet<String>,
    pub num_of_kept_logs: u16,
    pub autostart: bool
}

impl MainConfig {
    fn from_path(path: &PathBuf) -> anyhow::Result<Self> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                File::create(path).context("Failed to create file")?
            }
            Err(e) => return Err(anyhow!("Failed to open file: {}", e)),
        };

        let cfg: Self = match serde_json::from_reader(file) {
            Ok(cfg) => cfg,
            Err(_) => Self::default(),
        };

        Ok(cfg)
    }
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            devices: Default::default(),
            num_of_kept_logs: 10,
            autostart: true
        }
    }
}
