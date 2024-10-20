use serde::Deserialize;
use config::{Config, ConfigError, File};


#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn from_config_file(file_name: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(file_name))
            .build()?;

        config.try_deserialize()
    }
}

