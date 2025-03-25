use envconfig::Envconfig;

use crate::{api::configurations::ApiConfiguration};

#[derive(Envconfig, Debug, Clone)]
pub struct Config {
    #[envconfig(nested)]
    pub api_configuration: ApiConfiguration,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub config: Config,
}

impl Application {
    pub fn new() -> Self {
        let config = Config::init_from_env().expect("Can't load config from environment");

        Self { config }
    }
}
