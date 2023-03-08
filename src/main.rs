use dotenvy::dotenv;
use std::env;
use teloxide::{prelude::*, utils::command::BotCommands, types::Me};

extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Simple commands")]
enum SimpleCommand {
    #[command(description = "shows this message.")]
    Help,
    #[command(description = "shows your ID.")]
    MyId,
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    pretty_env_logger::init_timed();

    run(env::var("TELEGRAM_TOKEN").unwrap()).await;
}

async fn run(token: String) {
    let bot = Bot::new(token);

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
            .filter_command::<SimpleCommand>()
            .endpoint(simple_commands_handler),
        )
        .branch(
            dptree::filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup())
            .endpoint(group_message_handler),
        )
        .endpoint(message_handler);

    Dispatcher::builder(bot, handler)
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

async fn simple_commands_handler(
    bot: Bot,
    _me: teloxide::types::Me,
    msg: Message,
    cmd: SimpleCommand,
) -> Result<(), teloxide::RequestError> {
    let text = match cmd {
        SimpleCommand::Help => {
            SimpleCommand::descriptions().to_string()
        }
        SimpleCommand::MyId => {
            format!("{}", msg.from().unwrap().id)
        }
    };

    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

async fn group_message_handler(
    _bot: Bot,
    me: Me,
    msg: Message,
) -> Result<(), teloxide::RequestError> {
    let text = msg.text().unwrap_or("");
    if text.contains(&me.mention()) {
        println!("mention");
    } else {
        println!("not mention");
    }
    Ok(())
}

async fn message_handler(bot: Bot, me: Me, msg: Message) -> Result<(), teloxide::RequestError> {
    bot.send_message(msg.chat.id, me.username()).await?;
    Ok(())
}
