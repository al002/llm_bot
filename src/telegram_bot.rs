use std::sync::Arc;

use teloxide::{prelude::*, dispatching::dialogue::{ErasedStorage, RedisStorage, SqliteStorage, Storage, serializer::{Json, Bincode}}};
use tokio::sync::Mutex;

use crate::{bot_handlers::{SimpleCommand, commands_handler, prompt, transcribe }, openai::{OpenAI, ChatState, ChatStorage}, llm_rpc_client::LLMRpcClient};

pub struct TelegramBot {
    bot: Bot,
    openai: OpenAI,
    llm_rpc_client: LLMRpcClient,
}

impl TelegramBot {
    pub fn new(token: String, openai: OpenAI, rpc_client: LLMRpcClient) -> TelegramBot {
        TelegramBot {
            bot: Bot::new(token),
            openai,
            llm_rpc_client: rpc_client,
        }
    }

    pub async fn run(self) {
        let redis_url = std::env::var("DB_REDIS");
        let storage: ChatStorage = if redis_url.is_ok() {
            RedisStorage::open(redis_url.unwrap(), Bincode).await.unwrap().erase()
        } else {
            SqliteStorage::open("db.sqlite", Json).await.unwrap().erase()
        };

        let handler = Update::filter_message()
            .enter_dialogue::<Message, ErasedStorage<ChatState>, ChatState>()
            .branch(
                dptree::entry()
                    .filter_command::<SimpleCommand>()
                    .endpoint(commands_handler),
            )
            .branch(
                Message::filter_text().endpoint(prompt),
            )
            .branch(
                Message::filter_audio().endpoint(transcribe),
            );

        let openai = Arc::new(self.openai);
        let llm_rpc_client = Arc::new(Mutex::new(self.llm_rpc_client));

        Dispatcher::builder(self.bot, handler)
            // If no handler succeeded to handle an update, this closure will be called.
            .default_handler(|upd| async move {
                log::warn!("Unhandled update: {:?}", upd);
            })
            .dependencies(dptree::deps![openai, llm_rpc_client, storage])
            // If the dispatcher fails for some reason, execute this handler.
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}
