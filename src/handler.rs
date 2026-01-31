use std::time::SystemTime;

use opentelemetry::logs::{LogRecord, Logger, Severity};
use opentelemetry_sdk::logs::SdkLogger;
use serenity::{
    all::{Context, Event, MessageCreateEvent, RawEventHandler},
    async_trait,
};

macro_rules! handler {
    ($name:ident, $type:ident, ($c:pat, $e:pat),
    {$(let $init_pat:pat = $init_val:expr;)*},
    $test:expr, $message:literal,
    $(($key:literal, $val:expr)),* ) => {
        async |logger: SdkLogger, $c: Context, event: Event| {
            if let Event::$type($e) = event {
                $(let $init_pat = $init_val;)*
                if $test {
                    let mut log_record = logger.create_log_record();
                    log_record.set_event_name(stringify!($name));
                    log_record.set_observed_timestamp(SystemTime::now());
                    log_record.set_severity_number(Severity::Trace);
                    log_record.set_body(format!($message).into());
                    $(log_record.add_attribute($key, $val);)*
                    logger.emit(log_record);
                }
            }
        }
    };
}

pub struct Handler {
    pub logger: SdkLogger,
}

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, ctx: Context, ev: Event) {
        let handlers = vec![handler!(
            mesage_create,
            MessageCreate,
            (ctx, MessageCreateEvent { message, .. }),
            {
                let author = message.author.display_name();
                let channel = message.channel(&ctx).await.unwrap().guild().unwrap().name;
            },
            true,
            "@{author} sent message on #{channel}",
            ("user.id", message.author.id.to_string()),
            ("message.content", message.content)
        )];
        for handler in handlers {
            handler(self.logger.clone(), ctx.clone(), ev.clone()).await;
        }
    }
}
