use anyhow::Result;
use async_trait::async_trait;
use teloxide::prelude::Requester;
use teloxide::types::{ChatId, MessageId, UserId};

use crate::{bot::message_processor::MessageProcessor, shared::CommandHandler};

// --- Command Structs ---

pub struct GetMainMenuCommand {
    pub user_id: UserId,
    pub chat_id: ChatId,
    pub message_id: MessageId,
    pub edit_message: bool,
}

// --- Command Handlers ---

#[async_trait]
impl/*<TRepo1, TRepo2>*/ CommandHandler<GetMainMenuCommand>
    for MessageProcessor/*<TRepo1, TRepo2>*/
/*where
    TRepo1: Repo1 + Sync,
    TRepo2: Repo2 + Sync,*/
{
    async fn handle(&self, command: GetMainMenuCommand) -> Result<()> {
        self.bot_provider
            .bot
            .send_message(command.chat_id, "Основное меню:")
            .await?;
        Ok(())
    }
}
