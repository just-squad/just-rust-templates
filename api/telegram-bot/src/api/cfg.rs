use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ApiConfiguration {
    #[serde(default = "default_http_port")]
    pub http_port: u16,
    #[serde(default = "default_debug_port")]
    pub debug_port: u16,
}

fn default_http_port() -> u16 {
    5100
}

fn default_debug_port() -> u16 {
    5104
}
