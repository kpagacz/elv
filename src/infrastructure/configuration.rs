use config::{Map, Source};

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
    pub output_width: u32,
}

impl Default for CliConfiguration {
    fn default() -> Self {
        CliConfiguration { output_width: 120 }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    #[error("Cannot create a configuration file")]
    CreateError(#[from] std::io::Error),
    #[error("Cannot serialize the configuration to toml format")]
    SerializationError(#[from] toml::ser::Error),
    #[error("Cannot update the configuration value")]
    UpdateError,
    #[error("Cannot build the configuration")]
    BuildError(#[from] config::ConfigError),
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
        let config_builder_from_file = match Self::builder_from_config_file() {
            Ok(builder) => builder,
            Err(_) => return config::ConfigBuilder::default(),
        };

        config_builder_from_file.add_source(
            config::Environment::with_prefix("AOC")
                .separator("_")
                .keep_prefix(true),
        )
    }

    pub fn get_project_directories() -> directories::ProjectDirs {
        directories::ProjectDirs::from("", "", "elv")
            .expect("Failed to get the project directories")
    }

    fn builder_from_config_file(
    ) -> Result<config::ConfigBuilder<config::builder::DefaultState>, ConfigurationError> {
        let project_dirs = Self::get_project_directories();
        if !project_dirs.config_dir().join(".config").exists() {
            Self::write_default_config()?;
        }

        Ok(config::Config::builder().add_source(
            config::File::with_name(project_dirs.config_dir().join(".config").to_str().unwrap())
                .format(config::FileFormat::Toml),
        ))
    }

    pub fn get_file_configuration_map() -> Result<Map<String, config::Value>, ConfigurationError> {
        Ok(config::File::with_name(
            Self::get_project_directories()
                .config_dir()
                .join(".config")
                .to_str()
                .unwrap(),
        )
        .format(config::FileFormat::Toml)
        .collect()?)
    }

    pub fn update_configuration_key<T>(key: &str, value: T) -> Result<(), ConfigurationError>
    where
        T: Into<config::Value>,
    {
        let mut file_config = Self::builder_from_config_file()?;
        file_config = file_config
            .set_override(key, value)
            .map_err(|_| ConfigurationError::UpdateError)?;

        file_config
            .build()?
            .try_deserialize::<Configuration>()?
            .write_to_file()?;
        Ok(())
    }

    fn write_default_config() -> Result<(), ConfigurationError> {
        Configuration::default().write_to_file()?;
        Ok(())
    }

    fn write_to_file(self) -> Result<(), ConfigurationError> {
        let toml_string = toml::to_string(&self)?;
        let project_dirs = Self::get_project_directories();
        let config_dir = project_dirs.config_dir();
        std::fs::create_dir_all(config_dir)?;
        std::fs::write(config_dir.join(".config"), toml_string)?;
        Ok(())
    }
}
