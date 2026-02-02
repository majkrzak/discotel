use std::time::SystemTime;

use opentelemetry::logs::{LogRecord, Logger, Severity};
use opentelemetry_sdk::logs::SdkLogger;
use serenity::{
    all::{Context, Framework, FullEvent},
    async_trait,
};

macro_rules! handlers {
    ( $(
        $event_name:ident: $event_kind:ident => ($ctx:pat, $( $arg_name:ident $(: $arg_pat:pat)? ),*)
        $( if ($test:expr) )?
        let {
            $($init_p:pat = $init_e:expr;)*
        } in [
            $message:literal,
            $($attr_key:literal => $attr_val:expr,)*
        ];
    )* ) => {
        pub struct Handler {
            pub logger: SdkLogger,
        }

        impl Handler {
            $(
                async fn $event_name (&self, $ctx: Context, ev: FullEvent, now: SystemTime) {
                    if let FullEvent::$event_kind { $( $arg_name $(: $arg_pat)? ),* } = ev {
                        $( if $test )? {
                            $(let $init_p = $init_e;)*
                            let mut log_record = self.logger.create_log_record();
                            log_record.set_event_name(stringify!($name));
                            log_record.set_observed_timestamp(now);
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
        impl Framework for Handler {
            async fn dispatch(&self, ctx: Context, ev: FullEvent) {
                let now = SystemTime::now();
                $(self.$event_name (ctx.clone(),ev.clone(),now.clone()).await;)*
            }
        }
    };
}

handlers! {

    message_created: Message => (ctx, new_message)
    if (true) let {
        user_name = new_message.author.display_name();
        channel_id = new_message.channel_id;
        channel_name = channel_id.name(ctx).await.unwrap();
    } in [
        "@{user_name} sent message on #{channel_name}",
        "guild.id" => new_message.guild_id.unwrap().to_string(),
        "user.id" => new_message.author.id.to_string(),
        "user.name" => user_name.to_string(),
        "channel.id" => channel_id.to_string(),
        "channel.name" => channel_id.to_string(),
        "message.content" => new_message.content,
    ];

    voice_channel_joined: VoiceStateUpdate => (ctx, old, new)
    if (
        old.is_none()
        || new.channel_id.is_some() && new.channel_id.unwrap() != old.clone().unwrap().channel_id.unwrap()
    ) let {
        member = new.member.unwrap();
        user = member.user;
        user_name = user.display_name();
        channel_id = new.channel_id.unwrap();
        channel_name = channel_id.name(ctx).await.unwrap();
    } in [
        "@{user_name} joined `{channel_name}` voice channel",
        "guild.id" => new.guild_id.unwrap().to_string(),
        "user.id" => user.id.to_string(),
        "user.name" => user_name.to_string(),
        "channel.id" => channel_id.to_string(),
        "channel.name" => channel_name.to_string(),
    ];

    voice_channel_left: VoiceStateUpdate => (ctx, old, new)
    if (
        new.channel_id.is_none()
        || old.is_some() && new.channel_id.unwrap() != old.clone().unwrap().channel_id.unwrap()
    ) let {
        member = new.member.unwrap();
        user = member.user;
        user_name = user.display_name();
        channel_id = old.unwrap().channel_id.unwrap();
        channel_name = channel_id.name(ctx).await.unwrap();
    } in [
        "@{user_name} left `{channel_name}` voice channel",
        "guild.id" => new.guild_id.unwrap().to_string(),
        "user.id" => user.id.to_string(),
        "user.name" => user_name.to_string(),
        "channel.id" => channel_id.to_string(),
        "channel.name" => channel_name.to_string(),
    ];
}
