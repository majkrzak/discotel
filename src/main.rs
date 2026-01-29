use discotel::config::Config;
use dotenv::dotenv;
use opentelemetry::logs::{LogRecord, Logger, LoggerProvider, Severity};
use opentelemetry_otlp::LogExporter;
use opentelemetry_sdk::logs::{SdkLogger, SdkLoggerProvider};
use serenity::{
    all::{ClientBuilder, Context, Event, GatewayIntents, RawEventHandler},
    async_trait,
};
use std::time::SystemTime;

struct Handler {
    logger: SdkLogger,
}

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, _ctx: Context, ev: Event) {
        let mut log_entry = self.logger.create_log_record();
        log_entry.set_severity_number(Severity::Trace);
        log_entry.set_observed_timestamp(SystemTime::now());
        log_entry.set_body(serde_json::to_string(&ev).unwrap().into());
        self.logger.emit(log_entry);
    }
}

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
        .raw_event_handler(Handler { logger })
        .await?;

    client.start().await?;

    Ok(())
}
