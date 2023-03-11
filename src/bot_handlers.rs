use std::sync::Arc;

use teloxide::{prelude::*, utils::command::BotCommands, types::Me };
use tokio::sync::Mutex;

use crate::{openai::{OpenAI, ChatDialogue }, llm_rpc_client::LLMRpcClient, llm::LlmRequest};

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
    _openai: Arc<OpenAI>,
    llm_rpc_client: Arc<Mutex<LLMRpcClient>>,
    _dialogue: ChatDialogue,
    bot: Bot,
    me: Me,
    msg: Message,
) -> Result<(), teloxide::RequestError> {
    let text = msg.text().unwrap_or("");
    if (msg.chat.is_group() || msg.chat.is_supergroup()) && !text.contains(&me.mention()) {
        return Ok(());
    }

    let resposne = llm_rpc_client.lock().await.client.query(LlmRequest {
        query: text.to_string(),
    }).await.unwrap();
    bot.send_message(msg.chat.id, resposne.get_ref().response.clone()).await?;

    // uncomment below to use builtin openai
    // let response = openai.get_chat_response(msg.chat.id, dialogue, text.to_string()).await.unwrap();
    //
    // bot.send_message(msg.chat.id, response.clone()).await?;
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
