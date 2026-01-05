use anyhow::{Context, Result};
use config::{Config as ConfigLoader, Environment, File};

use crate::app::Config;

pub const RUST_APP_ENVIRONMENT_VAR_NAME: &str = "RUST_APP_ENVIRONMENT";

pub fn get_configuration() -> Result<Config> {
    let rust_app_env = std::env::var(RUST_APP_ENVIRONMENT_VAR_NAME)?;

    let mut builder = ConfigLoader::builder()
        // Add default config file, e.g., `config.json` or config.ENV.json
        .add_source(File::with_name("config.json").required(false));
    if !rust_app_env.is_empty() {
        builder = builder
            .add_source(File::with_name(&format!("config.{}.json", rust_app_env)).required(false));
    }
    // Add environment variables with a prefix, e.g., `APP_BOT_CONF__BOT_TOKEN=...`
    builder = builder.add_source(Environment::with_prefix("APP").separator("__"));

    let config_loader = builder
        .build()
        .context("Failed to build configuration loader")?;

    // Deserialize the configuration into the main `Config` struct
    let config: Config = config_loader
        .try_deserialize()
        .context("Failed to deserialize configuration")?;

    Ok(config)
}
