use std::net::SocketAddr;

use config::Config;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BotCfg {
    pub bot_token: String,
    pub secret_token: String,
    pub server_port: u16,
    pub webhook_host: String,
    pub webhook_path: String,
}

impl BotCfg {
    pub fn get_addr(&self) -> SocketAddr {
        ([127, 0, 0, 1], self.server_port).into()
    }

    pub fn get_webhook_addr(&self) -> url::Url {
        format!("{}{}", self.webhook_host, self.webhook_path)
            .parse()
            .unwrap()
    }
}

impl From<Config> for BotCfg {
    fn from(value: Config) -> Self {
        let bot_cfg: BotCfg = value
            .try_deserialize()
            .expect("error while deserialize BotCfg from global configuration");
        return bot_cfg;
    }
}
