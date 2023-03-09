use std::sync::Arc;

use teloxide::{prelude::*, utils::command::BotCommands, types::Me };

use crate::openai::{OpenAI, ChatDialogue };

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Simple commands")]
pub enum SimpleCommand {
    #[command(description = "shows this message.")]
    Help,
    #[command(description = "shows your ID.")]
    MyId,
}

pub async fn commands_handler(
    _openai: &OpenAI,
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

pub async fn prompt(
    openai: Arc<OpenAI>,
    dialogue: ChatDialogue,
    bot: Bot,
    me: Me,
    msg: Message,
) -> Result<(), teloxide::RequestError> {
    let text = msg.text().unwrap_or("");
    if (msg.chat.is_group() || msg.chat.is_supergroup()) && !text.contains(&me.mention()) {
        return Ok(());
    }

    let response = openai.get_chat_response(msg.chat.id, dialogue, text.to_string()).await.unwrap();
    
    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}

pub async fn transcribe(
    _openai: &OpenAI,
    _bot: Bot,
    _me: Me,
    _msg: Message,
) -> Result<(), teloxide::RequestError> {
    Ok(())
}
