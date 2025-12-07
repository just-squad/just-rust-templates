use anyhow::{Context, Result};
use app::Application;
use bot::TgBotProvider;
use std::{path::Path, sync::Arc};

mod app;
mod bot;
mod api;
mod shared;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // configure app config
    config::add_configuration()?;
    // configure logger
    env_logger::init();

    log::info!("Load application settings...");
    let app = Arc::new(Application::new());
    log::info!("tg token {}", app.config.bot_conf.bot_token);

    log::info!("Starting bot...");
    let bot_provider = TgBotProvider::new(&app.config.bot_conf);
    bot::INSTANCE
        .set(bot_provider.clone())
        .expect("Can't set static bot provider");

    tokio::spawn(start_bot());
    log::info!("Bot started...");

    log::info!("Start Api Server...");
    let api_provider = api::ApiProvider::new(&app.config.api_configuration);
    api_provider.start_server().await;
    Ok(())
}

async fn start_bot() {
    let bot_provider = bot::INSTANCE
        .get()
        .expect("Can't get instance of bot provider. Set instance before get");
    bot_provider.start_receive_messages().await;
}
