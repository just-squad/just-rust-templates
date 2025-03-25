use anyhow::{Context, Result};
use app::Application;
use bot::TgBotProvider;
use std::{path::Path, sync::Arc};

mod app;
mod api;

#[tokio::main]
async fn main() -> Result<()> {
    // configure app config
    load_app_cfg()?;

    // configure logger
    pretty_env_logger::init();

    log::info!("Load application settings...");
    let app = Arc::new(Application::new());

    log::info!("Start Api Server...");
    let api_provider = api::ApiProvider::new(&app.config.api_configuration);
    api_provider.start_server().await;
    Ok(())
}

fn load_app_cfg() -> Result<()> {
    let local_env_path = Path::new("local.env");
    if local_env_path.exists() {
        //dotenvy::dotenv().context("env file not found!")?;
        dotenvy::from_path(local_env_path).context(format!(
            "{} file not found",
            local_env_path.to_str().unwrap()
        ))?;
        Ok(())
    } else {
        dotenvy::dotenv().ok();
        Ok(())
    }
}