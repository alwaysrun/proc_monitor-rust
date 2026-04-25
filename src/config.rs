use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
    Close,
    Start,
}

impl<'de> Deserialize<'de> for ActionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "close" => Ok(ActionType::Close),
            "start" => Ok(ActionType::Start),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown action type: {}, expected 'close' or 'start'",
                s
            ))),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProcessMonitored {
    pub monitor: MonitorConfig,
}

#[derive(Debug, Deserialize)]
pub struct MonitorConfig {
    pub process: Vec<ProcessConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProcessConfig {
    pub monitored: String,
    pub action: HashMap<String, ActionType>,
    pub check_interval: u64,
}

pub fn get_config_path() -> PathBuf {
    let mut config_path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    config_path.push("configure");
    config_path.push("config.toml");

    if !config_path.exists() {
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let mut exe_config_path = PathBuf::from(exe_dir);
                exe_config_path.push("configure");
                exe_config_path.push("config.toml");
                if exe_config_path.exists() {
                    return exe_config_path;
                }
            }
        }
    }

    config_path
}

pub fn load_config() -> Result<ProcessMonitored, Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    let mut file = File::open(&config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: ProcessMonitored = toml::from_str(&contents)?;
    Ok(config)
}
