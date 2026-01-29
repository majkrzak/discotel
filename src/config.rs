use serde::Deserialize;

/// Application specific configuration
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub discord_token: String,
}
