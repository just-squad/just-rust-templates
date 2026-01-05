use anyhow::{Context, Result};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use app::Application;
use bot::{message_processor::MessageProcessor, TgBotProvider};

mod api;
mod app;
mod bot;
mod shared;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    // configure logger
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower=off"));
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Load application settings...");
    let config = config::get_configuration().context("Failed to load configuration")?;
    let app = Arc::new(Application::new(config));

    info!("Starting bot...");
    let bot_provider = TgBotProvider::new(&app.config.bot_conf);

    let message_processor = Arc::new(MessageProcessor::new(
        bot_provider.clone(),
        /* repos here */
    ));

    tokio::spawn(start_bot(bot_provider, message_processor));
    info!("Bot started...");

    info!("Start Api Server...");
    let api_provider = api::ApiProvider::new(&app.config.api_configuration);
    api_provider.start_server().await?;
    Ok(())
}

async fn start_bot/*<TRepo1, TRepo2>*/(bot_provider: TgBotProvider, message_processor: Arc<MessageProcessor/*<TRepo1, TRepo2> */>)
/*where
    TRepo1: Repo1 + Send + Sync + 'static,
    TRepo2: Repo2 + Send + Sync + 'static,*/
{
    bot_provider
        .start_receive_messages(message_processor)
        .await;
}