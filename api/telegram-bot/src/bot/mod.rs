use std::{error::Error, sync::OnceLock};

use commands::MainCommands;
use commands_handlers::main_commands_menu_handler;
use configurations::BotConfig;
use dialogue::State;
use teloxide::{
    Bot as TBot,
    adaptors::DefaultParseMode,
    dispatching::{UpdateFilterExt, UpdateHandler, dialogue::InMemStorage},
    dptree::{self, case},
    prelude::{Dialogue, Dispatcher, LoggingErrorHandler, RequesterExt},
    types::{ParseMode, Update},
};

mod commands;
mod commands_handlers;
pub mod cfg;
mod dialogue;
mod message_processor;

pub type Bot = DefaultParseMode<TBot>;
type BotDialogue = Dialogue<State, InMemStorage<State>>;

pub static INSTANCE: OnceLock<TgBotProvider> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct TgBotProvider {
    bot: Bot,
}

impl TgBotProvider {
    pub fn new(config: &BotConfig) -> Self {
        TgBotProvider {
            bot: TBot::new(&config.bot_token).parse_mode(ParseMode::MarkdownV2),
        }
    }

    pub fn global() -> &'static TgBotProvider {
        INSTANCE
            .get()
            .expect("Can't get TgBotProvider from global instance")
    }
}

impl TgBotProvider {
    pub async fn start_receive_messages(&self) {
        let bot_instance = self.bot.clone();
        Dispatcher::builder(bot_instance, self.schema())
            .dependencies(dptree::deps![InMemStorage::<State>::new()])
            .default_handler(|upd| async move {
                log::warn!("Unhandled update: {:?}", upd);
            })
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }

    fn schema(&self) -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
        let main_commands_handler = teloxide::filter_command::<MainCommands, _>()
            .branch(case![MainCommands::Menu].endpoint(main_commands_menu_handler));

        let message_handler = Update::filter_message().branch(main_commands_handler);

        teloxide::dispatching::dialogue::enter::<Update, InMemStorage<State>, State, _>()
            .branch(message_handler)
    }
}
