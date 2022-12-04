use serde::{Deserialize, Serialize};
use std::fs;

use crate::errors::*;
use config::{Config, Environment, File, FileFormat::Toml};
use directories::ProjectDirs;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Configuration {
    #[serde(default)]
    pub aoc: AocConfiguration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AocConfiguration {
    #[serde(default = "default_token")]
    pub token: String,
}

fn default_token() -> String {
    "".to_string()
}

impl Default for AocConfiguration {
    fn default() -> Self {
        AocConfiguration {
            token: default_token(),
        }
    }
}

pub fn get_config() -> Configuration {
    let project_dirs = get_project_directories();
    if !project_dirs.config_dir().join(".config").exists() {
        if write_default_config().is_err() {
            return Configuration::default();
        }
    }
    let builder = Config::builder()
        .add_source(
            File::with_name(project_dirs.config_dir().join(".config").to_str().unwrap())
                .format(Toml),
        )
        .add_source(
            Environment::with_prefix("AOC")
                .separator("_")
                .keep_prefix(true),
        );
    match builder.build() {
        Ok(config) => config
            .try_deserialize::<Configuration>()
            .unwrap_or(Configuration::default()),
        Err(_) => Configuration::default(),
    }
}

pub fn get_project_directories() -> ProjectDirs {
    ProjectDirs::from("", "", "aoc-elf").expect("Failed to get the project directories")
}

fn write_default_config() -> Result<()> {
    let default_config = Configuration::default();
    let project_dirs = get_project_directories();
    let config_path = project_dirs.config_dir().join(".config");
    let prefix = config_path.parent().unwrap();
    std::fs::create_dir_all(prefix)?;
    let toml_string = toml::to_string(&default_config)?;
    fs::write(config_path, toml_string)?;
    Ok(())
}
