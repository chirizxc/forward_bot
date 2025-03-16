use telers::{
    Bot, Dispatcher, Router,
    enums::UpdateType,
    event::{EventReturn, telegram::HandlerResult},
    methods::ForwardMessage,
    types::Message,
};
use tracing::{Level, event};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};

const CHANNEL_ID: i64 = -1002193370548; // @opensource_findings_python
const CHAT_ID: i64 = -1002216851397; // @opensource_findings_chat

const TOKEN: &str = "...";

async fn forward_message(bot: Bot, message: Message) -> HandlerResult {
    if message.chat().id() == CHANNEL_ID {
        bot.send(ForwardMessage::new(CHAT_ID, CHANNEL_ID, message.id()))
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

    let bot = Bot::new(TOKEN);

    let mut router = Router::new("main");
    router.channel_post.register(forward_message);

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
