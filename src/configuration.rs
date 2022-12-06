use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::errors::*;
use config::{builder::DefaultState, Config, ConfigBuilder, Environment, File, FileFormat::Toml};

#[derive(Debug, Deserialize, Serialize)]
pub struct AocConfiguration {
    #[serde(default = "default_token")]
    pub token: String,
}

impl Default for AocConfiguration {
    fn default() -> Self {
        AocConfiguration {
            token: default_token(),
        }
    }
}

fn default_token() -> String {
    "".to_string()
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Configuration {
    #[serde(default)]
    pub aoc: AocConfiguration,
}

impl Configuration {
    pub fn new() -> Self {
        match Self::builder().build() {
            Ok(config) => config
                .try_deserialize::<Configuration>()
                .unwrap_or(Configuration::default()),
            Err(_) => Configuration::default(),
        }
    }

    pub fn builder() -> ConfigBuilder<DefaultState> {
        let project_dirs = Self::get_project_directories();
        if !project_dirs.config_dir().join(".config").exists() {
            if Self::write_default_config().is_err() {
                println!(
                    "Failed to write the default config to: {}",
                    project_dirs.config_dir().join(".config").display()
                );
                println!("Using default configuration");
                return ConfigBuilder::default();
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
        builder
    }

    pub fn get_project_directories() -> ProjectDirs {
        ProjectDirs::from("", "", "aoc-elf").expect("Failed to get the project directories")
    }

    fn write_default_config() -> Result<()> {
        let default_config = Configuration::default();
        let project_dirs = Self::get_project_directories();
        let config_path = project_dirs.config_dir().join(".config");
        let prefix = config_path.parent().unwrap();
        std::fs::create_dir_all(prefix)?;
        let toml_string = toml::to_string(&default_config)?;
        fs::write(config_path, toml_string)?;
        Ok(())
    }
}
