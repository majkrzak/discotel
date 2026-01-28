use dotenv::dotenv;
use serde::Deserialize;
use serenity::{
    all::{ClientBuilder, Context, Event, GatewayIntents, RawEventHandler},
    async_trait,
};
use std::sync::Arc;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub discord_token: Arc<str>,
}

struct Handler;

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, _ctx: Context, ev: Event) {
        println!("{ev:?}");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = envy::from_env::<Config>().unwrap();

    let mut client = ClientBuilder::new(config.discord_token, GatewayIntents::all())
        .raw_event_handler(Handler)
        .await
        .unwrap();

    client.start().await.unwrap();
}
