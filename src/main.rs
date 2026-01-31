use discotel::{config::Config, handler::Handler};
use dotenv::dotenv;
use opentelemetry::logs::LoggerProvider;
use opentelemetry_otlp::LogExporter;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use serenity::all::{ClientBuilder, GatewayIntents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let config = envy::from_env::<Config>()?;

    let logger = SdkLoggerProvider::builder()
        .with_batch_exporter(LogExporter::builder().with_http().build()?)
        .build()
        .logger("discolog");

    let mut client = ClientBuilder::new(config.discord_token, GatewayIntents::all())
        .raw_event_handler(Handler{logger})
        .await?;

    client.start().await?;

    Ok(())
}
