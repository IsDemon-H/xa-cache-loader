use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub custom_path: Option<String>,
}

impl Config {
    fn config_path(exe_dir: &PathBuf) -> PathBuf {
        exe_dir.join("config.json")
    }

    pub fn load(exe_dir: &PathBuf) -> Self {
        let path = Self::config_path(exe_dir);
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(cfg) = serde_json::from_str::<Config>(&content) {
                    return cfg;
                }
            }
        }
        Config { custom_path: None }
    }

    pub fn save(&self, exe_dir: &PathBuf) {
        let path = Self::config_path(exe_dir);
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, json);
        }
    }

    pub fn get_target_path(&self, exe_dir: &PathBuf) -> PathBuf {
        if let Some(ref custom) = self.custom_path {
            PathBuf::from(custom)
        } else {
            exe_dir.join("league of legends")
        }
    }
}
