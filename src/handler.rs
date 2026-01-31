use std::time::SystemTime;

use opentelemetry::logs::{LogRecord, Logger, Severity};
use opentelemetry_sdk::logs::SdkLogger;
use serenity::{
    all::{Context, Event, MessageCreateEvent, RawEventHandler},
    async_trait,
};

struct LogLine {
    msg: String,
}

impl LogLine {
    fn apply_to(self, log_record: &mut impl LogRecord) {
        log_record.set_body(self.msg.into());
        log_record.set_severity_number(Severity::Trace);
    }
}

macro_rules! handler {
    ($event_name:ident, $event_type:ident, ($c:pat, $e:pat), $b:block ) => {
        async fn $event_name($c: Context, raw_event: Event) -> Option<LogLine> {
            if let Event::$event_type($e) = raw_event {
                Some($b)
            } else {
                None
            }
        }
    };
}

handler!(
    mesage_create,
    MessageCreate,
    (ctx, MessageCreateEvent { message, .. }),
    {
        let author = message.author.display_name();
        let channel = message.channel(&ctx).await.unwrap().guild().unwrap().name;
        LogLine {
            msg: format!("@{author} sent message on #{channel}"),
        }
    }
);

pub struct Handler {
    pub logger: SdkLogger,
}

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, ctx: Context, ev: Event) {
        if let Some(log_line) = mesage_create(ctx, ev).await {
            let mut log_record = self.logger.create_log_record();
            log_record.set_observed_timestamp(SystemTime::now());
            log_line.apply_to(&mut log_record);
            self.logger.emit(log_record);
        }
    }
}
