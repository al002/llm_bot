use std::sync::Arc;

use log::info;
use teloxide::{prelude::*, utils::command::BotCommands, types::Me };
use tokio::sync::Mutex;

use crate::{openai::{OpenAI, ChatDialogue }, llm_rpc_client::LLMRpcClient, llm::LlmRequest};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Simple commands")]
pub enum SimpleCommand {
    #[command(description = "shows this message.")]
    Help,
    #[command(description = "reset chat history")]
    Reset,
    #[command(description = "shows your ID.")]
    MyId,
}

pub async fn commands_handler(
    openai: Arc<OpenAI>,
    dialogue: ChatDialogue,
    bot: Bot,
    _me: teloxide::types::Me,
    msg: Message,
    cmd: SimpleCommand,
) -> Result<(), teloxide::RequestError> {
    let text = match cmd {
        SimpleCommand::Help => {
            SimpleCommand::descriptions().to_string()
        }
        SimpleCommand::Reset => {
            info!("Resetting the conversation history for user {}", msg.chat.username().unwrap());
            reset(openai, msg.chat.id, dialogue).await
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
    // llm_rpc_client: Arc<Mutex<LLMRpcClient>>,
    dialogue: ChatDialogue,
    bot: Bot,
    me: Me,
    msg: Message,
) -> Result<(), teloxide::RequestError> {
    let text = msg.text().unwrap_or("");
    if (msg.chat.is_group() || msg.chat.is_supergroup()) && !text.contains(&me.mention()) {
        return Ok(());
    }

    // let resposne = llm_rpc_client.lock().await.client.query(LlmRequest {
    //     query: text.to_string(),
    // }).await.unwrap();
    // bot.send_message(msg.chat.id, resposne.get_ref().response.clone()).await?;

    // uncomment below to use builtin openai
    let response = openai.get_chat_response(msg.chat.id, dialogue, text.to_string()).await.unwrap();

    bot.send_message(msg.chat.id, response.clone()).await?;
    Ok(())
}

pub async fn reset(
    openai: Arc<OpenAI>,
    chat_id: ChatId,
    dialogue: ChatDialogue,
) -> String {
    let state = &mut dialogue.get().await.unwrap().unwrap();
    openai.reset_chat_history(chat_id, state);
    let update_result = dialogue.update(state.to_owned()).await;

    if update_result.is_err() {
        return String::from("Reset error");
    }

    String::from("Done")
}

pub async fn transcribe(
    _openai: &OpenAI,
    _bot: Bot,
    _me: Me,
    _msg: Message,
) -> Result<(), teloxide::RequestError> {
    Ok(())
}
