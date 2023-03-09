use teloxide::{prelude::*, utils::command::BotCommands, types::Me };

use crate::openai::OpenAI;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Simple commands")]
pub enum SimpleCommand {
    #[command(description = "shows this message.")]
    Help,
    #[command(description = "shows your ID.")]
    MyId,
}

pub async fn commands_handler(
    _openai: OpenAI,
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

pub async fn group_message_handler(
    _openai: OpenAI,
    _bot: Bot,
    me: Me,
    msg: Message,
    _cmd: SimpleCommand,
) -> Result<(), teloxide::RequestError> {
    let text = msg.text().unwrap_or("");
    if text.contains(&me.mention()) {
        println!("mention");
    } else {
        println!("not mention");
    }
    Ok(())
}

pub async fn message_handler(
    openai: OpenAI,
    bot: Bot,
    _me: Me,
    msg: Message,
) -> Result<(), teloxide::RequestError> {
    let response = openai.get_chat_response(msg.text().unwrap().to_string()).await.unwrap();
    
    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}
