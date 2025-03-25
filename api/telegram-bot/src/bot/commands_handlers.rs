use std::error::Error;

use teloxide::{dispatching::dialogue::GetChatId, types::Message};

use crate::shared::CommandHandler;

use super::{
    Bot, BotDialogue,
    message_processor::{GetMainMenuCommand, MessageProcessor},
};

type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

/// .
///
/// # Errors
///
/// This function will return an error if .
pub async fn main_commands_menu_handler(
    msg: Message,
    _bot: Bot,
    _dialogue: BotDialogue,
) -> HandlerResult {
    let processor = MessageProcessor::new().await?;
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
            _user_id: user_id,
            chat_id,
            _message_id: msg.id,
            _edit_message: false,
        })
        .await?;
    Ok(())
}
