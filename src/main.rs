use once_cell::sync::Lazy;
use serde::Deserialize;
use telers::{
    Bot, Dispatcher, Router,
    enums::UpdateType,
    event::{EventReturn, telegram::HandlerResult},
    methods::ForwardMessage,
    types::Message,
};
use tracing::{Level, event};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    token: String,
    from_id: i64,
    to_id: i64,
}

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    envy::prefixed("FB_")
        .from_env::<AppConfig>()
        .expect("cannot load env")
});

async fn forward_message(bot: Bot, message: Message) -> HandlerResult {
    if message.chat().id() == CONFIG.from_id {
        bot.send(ForwardMessage::new(
            CONFIG.to_id,
            CONFIG.from_id,
            message.id(),
        ))
        .await?;
    }

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("INFO"))
        .init();

    let bot = Bot::new(&CONFIG.token);

    let router = {
        let mut r = Router::new("main");
        r.channel_post.register(forward_message);
        r
    };

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::ChannelPost)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => event!(Level::INFO, "Bot stopped"),
        Err(err) => event!(Level::ERROR, error = %err, "Bot stopped"),
    }
}
