use crate::bot::TgBotProvider;

#[derive(Debug, Clone)]
pub struct MessageProcessor/*<TRepo1, TRepo2>*/ {
    pub bot_provider: TgBotProvider,
    // Add your repositories here
}

impl/*<TRepo1, TRepo2>*/ MessageProcessor/*<TRepo1, TRepo2>*/
/*where
    TRepo1: Repo1,
    TRepo2: Repo2,*/
{
    pub fn new(
        bot_provider: TgBotProvider,
        /*repo_1: TRepo1,
        repo_2: TRepo2,*/
    ) -> Self {
        Self {
            bot_provider,
            /*repo_1,
            repo_2,*/
        }
    }
}
