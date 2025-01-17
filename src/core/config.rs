use crate::gui::views::settings::Settings;
use serde::{Deserialize, Serialize};
use static_init::dynamic;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme: String,
}

#[dynamic]
static CONFIG_FILE: PathBuf = config_dir();

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "Lupin".to_string(),
        }
    }
}

impl Config {
    pub fn save_changes(settings: &Settings) {
        let new = Config {
            theme: settings.theme.name.clone(),
        };
        let config = toml::to_string(&new).unwrap();
        fs::write(&*CONFIG_FILE, config).expect("Could not write config file to disk!");
    }

    pub fn load_configuration_file() -> Self {
        match fs::read_to_string(&*CONFIG_FILE) {
            Ok(s) => toml::from_str(&s).unwrap_or_else(|e| panic!("Invalid config file: `{}`", e)),
            Err(e) => {
                println!("{}", e);
                let default_conf = toml::to_string(&Config::default()).unwrap();
                fs::write(&*CONFIG_FILE, default_conf)
                    .expect("Could not write config file to disk!");
                Config::default()
            }
        }
    }
}

fn config_dir() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap().join("uad");
    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }
    config_dir.join("config.toml")
}
