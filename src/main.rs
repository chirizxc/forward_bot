mod config;

use telers::{
    Bot, Dispatcher, Extension, Router,
    enums::UpdateType,
    event::{EventReturn, telegram::HandlerResult},
    methods::ForwardMessage,
    types::Message,
};
use tracing::{Level, event};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::config::{BotConfig, ChatConfig, Config, LoggingConfig};

async fn forward_message(
    bot: Bot,
    message: Message,
    Extension(chats): Extension<Vec<ChatConfig>>,
) -> HandlerResult {
    let chat_id = message.chat().id();

    for ChatConfig { from_id, to_id } in chats {
        if chat_id == from_id {
            for &to in to_id.as_slice() {
                let sent_message = bot
                    .send(ForwardMessage::new(to, from_id, message.id()))
                    .await?;
                event!(
                    Level::INFO,
                    "Message forwarded from {} to {} (message_id={})",
                    from_id,
                    to,
                    sent_message.id()
                );
            }
        }
    }
    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Config {
        bot: BotConfig { token },
        chats,
        logging: LoggingConfig { dirs },
    } = config::parse_from_fs(&*config::get_path()).unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::builder().parse_lossy(dirs))
        .init();

    let bot = Bot::new(token);

    let router = {
        let mut r = Router::new("main");
        r.channel_post.register(forward_message);
        r
    };

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::ChannelPost)
        .extension(chats)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => event!(Level::INFO, "Bot stopped"),
        Err(err) => event!(Level::ERROR, error = %err, "Bot stopped"),
    }
}
