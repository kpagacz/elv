use crate::domain::errors::*;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct CliConfiguration {
    pub output_width: usize,
}

impl Default for CliConfiguration {
    fn default() -> Self {
        CliConfiguration { output_width: 120 }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Configuration {
    #[serde(default)]
    pub aoc: AocConfiguration,
    pub cli: CliConfiguration,
}

impl Configuration {
    pub fn new() -> Self {
        match Self::builder().build() {
            Ok(config) => config
                .try_deserialize::<Configuration>()
                .unwrap_or_default(),
            Err(_) => Configuration::default(),
        }
    }

    pub fn builder() -> config::ConfigBuilder<config::builder::DefaultState> {
        let project_dirs = Self::get_project_directories();
        if !project_dirs.config_dir().join(".config").exists()
            && Self::write_default_config().is_err()
        {
            println!(
                "Failed to write the default config to: {}",
                project_dirs.config_dir().join(".config").display()
            );
            println!("Using default configuration");
            return config::ConfigBuilder::default();
        }

        let builder = config::Config::builder()
            .add_source(
                config::File::with_name(
                    project_dirs.config_dir().join(".config").to_str().unwrap(),
                )
                .format(config::FileFormat::Toml),
            )
            .add_source(
                config::Environment::with_prefix("AOC")
                    .separator("_")
                    .keep_prefix(true),
            );
        builder
    }

    pub fn get_project_directories() -> directories::ProjectDirs {
        directories::ProjectDirs::from("", "", "elv")
            .expect("Failed to get the project directories")
    }

    fn write_default_config() -> Result<()> {
        let default_config = Configuration::default();
        let project_dirs = Self::get_project_directories();
        let config_path = project_dirs.config_dir().join(".config");
        let prefix = config_path.parent().unwrap();
        std::fs::create_dir_all(prefix)?;
        let toml_string = toml::to_string(&default_config)?;
        std::fs::write(config_path, toml_string)?;
        Ok(())
    }
}
