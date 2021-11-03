use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct NyaaConfig {
    pub url: String,
    pub interval: u64,
}

#[derive(Deserialize)]
pub struct DiscordConfig {
    pub webhook_url: String,
    pub avatar_url: String,
    pub author_name: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub nyaa: NyaaConfig,
    pub discord: DiscordConfig,
}
