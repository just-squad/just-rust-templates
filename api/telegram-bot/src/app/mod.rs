use serde::Deserialize;

use crate::{api::cfg::ApiConfiguration, bot::cfg::BotCfg};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub bot_conf: BotCfg,
    pub api_configuration: ApiConfiguration,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub config: Config,
}

impl Application {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
