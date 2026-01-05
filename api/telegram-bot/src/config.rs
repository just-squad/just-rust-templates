use std::path::Path;

use anyhow::Context;
use config::{Config, Environment};

pub fn add_configuration() -> anyhow::Result<Config> {
    load_env_file()?;

    Config::builder()
        .add_source(Environment::with_prefix(""))
        .add_source(config::Environment::with_prefix("RUST"))
        .add_source(config::Environment::with_prefix("APPENV").separator("__"))
        .build()
        .with_context("Error occurred while load configurations:")?
}

fn load_env_file() -> anyhow::Result<()> {
    let local_env_path = Path::new(".env.local");
    if local_env_path.exists() {
        dotenvy::from_path(local_env_path)?
    } else {
        dotenvy::dotenv().with_context("Error occurred while load configurations:")?
    }
}
