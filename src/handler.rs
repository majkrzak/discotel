use std::time::SystemTime;

use opentelemetry::logs::{LogRecord, Logger, Severity};
use opentelemetry_sdk::logs::SdkLogger;
use serenity::{
    all::{Context, Event, MessageCreateEvent, RawEventHandler},
    async_trait,
};

macro_rules! handlers {
    ( $(
        $event_name:ident: $event_kind:ident => ($ctx:pat, $mx:pat) {
            $(let $init_p:pat = $init_e:expr;)*
        } if ($test:expr) [
            $message:literal,
            $($attr_key:literal => $attr_val:expr,)*
        ];
    )* ) => {
        pub struct Handler {
            pub logger: SdkLogger,
        }

        impl Handler {
            $(
                async fn $event_name (&self, $ctx: Context, ev: Event) {
                    if let Event::$event_kind($mx) = ev {
                        $(let $init_p = $init_e;)*
                        if $test {
                            let mut log_record = self.logger.create_log_record();
                            log_record.set_event_name(stringify!($name));
                            log_record.set_observed_timestamp(SystemTime::now());
                            log_record.set_severity_number(Severity::Trace);
                            log_record.set_body(format!($message).into());
                            $(log_record.add_attribute($attr_key, $attr_val);)*
                            self.logger.emit(log_record);
                        }
                    }
                }
            )*
        }

        #[async_trait]
        impl RawEventHandler for Handler {
            async fn raw_event(&self, ctx: Context, ev: Event) {
                $(self.$event_name (ctx,ev).await;)*
            }
        }
    };
}

handlers! {
    message_created: MessageCreate => (ctx, MessageCreateEvent { message, .. }) {
        let author = message.author.display_name();
        let channel = message.channel(&ctx).await.unwrap().guild().unwrap().name;
    } if (true) [
        "@{author} sent message on #{channel}",
        "user.id" => message.author.id.to_string(),
        "message.content" => message.content,
    ];
}
