use crate::error::Error;
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::types::{MessageId, MessageReactionUpdated, ThreadId};
use teloxide::utils::command::BotCommands;

use crate::secrets::Secrets;
use redis::AsyncCommands;

pub struct TgBot {
    pub bot: Bot,
    pub redis_client: redis::Client,
}

impl TgBot {
    pub async fn new(secret: Secrets) -> Result<Self, Error> {
        Ok(TgBot {
            bot: Bot::new(secret.tg_token),
            redis_client: redis::Client::open(secret.redis_uri)?,
        })
    }

    pub async fn start(&self) {
        let handler = dptree::entry()
            .branch(
                Update::filter_message()
                    .branch(
                        dptree::entry()
                            .filter_command::<Commands>()
                            .endpoint(start_commands_handler),
                    )
                    .branch(
                        dptree::filter(|msg: Message| {
                            // TODO make thread id configurable
                            msg.chat.is_supergroup()
                                && msg.thread_id == Some(ThreadId(MessageId(2)))
                        })
                        .endpoint(group_chat_handler),
                    ),
            )
            .branch(
                Update::filter_message_reaction_updated().branch(
                    dptree::filter(|reaction: MessageReactionUpdated| reaction.user.is_some())
                        .endpoint(reaction_handler),
                ),
            );

        Dispatcher::builder(self.bot.clone(), handler)
            .dependencies(dptree::deps![self.redis_client.clone()])
            .default_handler(|upd| async move {
                tracing::warn!("Unhandled update: {:?}", upd);
            })
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Commands {
    Start,
}

async fn group_chat_handler(bot: Bot, msg: Message, client: redis::Client) -> Result<(), Error> {
    tracing::debug!("Received a message from a group chat: {:?}", msg);

    if let Some(text) = msg.text() {
        let lines = text.split('\n');
        bot.delete_message(msg.chat.id, msg.id).await?;

        for line in lines {
            let formatted_line = format!("➖          {}", line);
            let mut message = bot.send_message(msg.chat.id, formatted_line.clone());
            message.message_thread_id = msg.thread_id;
            let msg_id = message.await?.id;

            let mut con = client.get_multiplexed_tokio_connection().await?;
            con.set(msg_id.0.to_string(), formatted_line).await?;
        }
    }

    Ok(())
}

async fn start_commands_handler(bot: Bot, msg: Message, cmd: Commands) -> Result<(), Error> {
    let text = match cmd {
        Commands::Start => {
            format!(
                "👋 Hello, {}!\nI am a private bot 🔒\nYou cannot use me 📵",
                msg.from.unwrap().first_name
            )
        }
    };

    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

async fn reaction_handler(
    bot: Bot,
    reaction: MessageReactionUpdated,
    client: redis::Client,
) -> Result<(), Error> {
    tracing::info!("MessageReactionUpdated: {:?}", reaction);

    let text = get_msg(reaction.message_id, client)
        .await?
        .replace("➖", "✅");
    bot.edit_message_text(reaction.chat.id, reaction.message_id, text)
        .await?;

    Ok(())
}

async fn get_msg(id: MessageId, client: redis::Client) -> Result<String, Error> {
    let mut con = client.get_multiplexed_tokio_connection().await?;
    let value: String = con.get(id.0.to_string()).await?;
    Ok(value)
}
