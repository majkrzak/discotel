use dotenv::dotenv;
use opentelemetry::logs::{LogRecord, Logger, LoggerProvider};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::logs::{SdkLogger, SdkLoggerProvider};
use serde::Deserialize;
use serenity::{
    all::{ClientBuilder, Context, Event, GatewayIntents, RawEventHandler},
    async_trait,
};

#[derive(Deserialize, Debug, Clone)]
struct Config {
    discord_token: String,
    otel_url: String,
}

struct Handler {
    logger: SdkLogger,
}

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, _ctx: Context, ev: Event) {
        let logger = self.logger.clone();
        let mut log_entry = logger.create_log_record();
        log_entry.set_body(serde_json::to_string(&ev).unwrap().into());
        tokio::task::spawn_blocking(move || {
            logger.emit(log_entry);
        })
        .await
        .unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let config = envy::from_env::<Config>()?;

    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpJson)
        .with_endpoint(config.otel_url)
        .build()?;

    let provider = SdkLoggerProvider::builder()
        .with_simple_exporter(exporter)
        .build();

    let logger: opentelemetry_sdk::logs::SdkLogger = provider.logger("discolog");

    let mut client = ClientBuilder::new(config.discord_token, GatewayIntents::all())
        .raw_event_handler(Handler { logger })
        .await?;

    client.start().await?;

    Ok(())
}
