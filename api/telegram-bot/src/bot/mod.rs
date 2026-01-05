use std::{error::Error, sync::Arc};

// public modules
pub mod cfg;
pub mod commands;
pub mod handlers;
pub mod message_processor;

// private modules
mod dialogue;
use commands::MainCommands;
use dialogue::State;
use message_processor::MessageProcessor;
use teloxide::{
    Bot as TBot,
    adaptors::DefaultParseMode,
    dispatching::{
        UpdateFilterExt, UpdateHandler,
        dialogue::{GetChatId, InMemStorage},
    },
    dptree::{self, case},
    prelude::{Dispatcher, LoggingErrorHandler, RequesterExt},
    types::{Message, ParseMode, Update},
};
use tracing::warn;

use crate::{
    bot::{cfg::BotCfg, handlers::user_handlers::GetMainMenuCommand},
    shared::CommandHandler,
};

type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub type Bot = DefaultParseMode<TBot>;

#[derive(Debug, Clone)]
pub struct TgBotProvider {
    bot: Bot,
}

impl TgBotProvider {
    pub fn new(config: &BotCfg) -> Self {
        TgBotProvider {
            bot: TBot::new(&config.bot_token).parse_mode(ParseMode::MarkdownV2),
        }
    }
}

impl TgBotProvider {
    pub async fn start_receive_messages(
        &self,
        message_processor: Arc<MessageProcessor /*<TRepo1, TRepo2>*/>,
    )
    /*where
    TRepo1: Repo1 + Send + Sync + 'static,
    TRepo2: Repo2 + Send + Sync + 'static,*/
    {
        let bot_instance = self.bot.clone();
        Dispatcher::builder(bot_instance, self.schema/*::<TRepo1, TRepo2>*/())
            .dependencies(dptree::deps![
                InMemStorage::<State>::new(),
                message_processor
            ])
            .default_handler(|upd| async move {
                warn!("Unhandled update: {:?}", upd);
            })
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }

    fn schema(&self) -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>>
/*where
        TRepo1: Repo1 + Send + Sync + 'static,
        TRepo2: Repo2 + Send + Sync + 'static,*/ {
        let main_commands_handler = teloxide::filter_command::<MainCommands, _>()
            .branch(case![MainCommands::Menu].endpoint(main_commands_menu_handler));

        let message_handler = Update::filter_message().branch(main_commands_handler);

        teloxide::dispatching::dialogue::enter::<Update, InMemStorage<State>, State, _>()
            .branch(message_handler)
    }
}

async fn main_commands_menu_handler(
    msg: Message,
    processor: Arc<MessageProcessor /*<TRepo1, TRepo2>*/>,
) -> HandlerResult
/*where
        TRepo1: Repo1 + Send + Sync + 'static,
        TRepo2: Repo2 + Send + Sync + 'static,*/
{
    let user_id = msg
        .from
        .clone()
        .expect("Can't get user info from telegram message")
        .id;
    let chat_id = msg
        .chat_id()
        .expect("Can't get chat id from telegram message");
    processor
        .handle(GetMainMenuCommand {
            user_id,
            chat_id,
            message_id: msg.id,
            edit_message: false,
        })
        .await?;
    Ok(())
}
