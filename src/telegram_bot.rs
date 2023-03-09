use teloxide::prelude::*;

use crate::{command_handlers::{SimpleCommand, commands_handler, group_message_handler, message_handler}, openai::OpenAI};

pub struct TelegramBot {
    bot: Bot,
}

impl TelegramBot {
    pub fn new(token: String) -> TelegramBot {
        TelegramBot {
            bot: Bot::new(token),
        }
    }

    pub async fn run(self, _openai: OpenAI) {
        let handler = Update::filter_message()
            .branch(
                dptree::entry()
                .filter_command::<SimpleCommand>()
                .endpoint(commands_handler),
            )
            .branch(
                dptree::filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup())
                .endpoint(group_message_handler),
            )
            .endpoint(message_handler);

        Dispatcher::builder(self.bot, handler)
            // If no handler succeeded to handle an update, this closure will be called.
            .default_handler(|upd| async move {
                log::warn!("Unhandled update: {:?}", upd);
            })
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
