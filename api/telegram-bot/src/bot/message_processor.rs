use async_trait::async_trait;
use teloxide::{prelude::Requester, types::{ChatId, MessageId, UserId}};
use anyhow::Result;

use crate::shared::CommandHandler;

use super::TgBotProvider;

#[derive(Debug, Clone)]
pub struct MessageProcessor {
    bot_provider: TgBotProvider,
}

impl MessageProcessor {
    /// .
    pub async fn new() -> Result<MessageProcessor> {
        let bt_prvdr = TgBotProvider::global().clone();

        Ok(MessageProcessor {
            bot_provider: bt_prvdr
        })
    }
}

pub struct GetMainMenuCommand {
    pub _user_id: UserId,
    pub chat_id: ChatId,
    pub _message_id: MessageId,
    pub _edit_message: bool,
}
#[async_trait]
impl CommandHandler<GetMainMenuCommand>
    for MessageProcessor
{
    async fn handle(&self, command: GetMainMenuCommand) -> Result<()> {
        self.bot_provider.bot.send_message(
                command.chat_id,
                "TEST START MESSAGE. PUT YOUR MESSAGE HERE",
            );

        Ok(())
    }
}