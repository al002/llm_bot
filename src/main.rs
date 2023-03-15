use dotenvy::dotenv;
use std::env;
use telegram_bot::TelegramBot;
use openai::{OpenAI, OpenAIConfig};

extern crate pretty_env_logger;
extern crate log;

pub mod llm {
    tonic::include_proto!("llm");
}

mod bot_handlers;
mod telegram_bot;
mod openai;
mod llm_rpc_client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");
    pretty_env_logger::init_timed();

    let openai_config = OpenAIConfig {
        api_key: env::var("OPENAI_API_KEY").unwrap(),
        show_usage: env::var("SHOW_USAGE").unwrap_or(String::from("false")) == "true",
        max_history_size: env::var("MAX_HISTORY_SIZE").unwrap_or(String::from("10")).parse::<u16>().unwrap_or(10),
        max_conversation_age_minutes: env::var("MAX_CONVERSATION_AGE_MINUTES").unwrap_or(String::from("180")).parse::<u16>().unwrap_or(180),
        assistant_prompt: env::var("ASSISTANT_PROMPT").unwrap_or(String::from("You are a helpful assistant.")),
        max_tokens: env::var("MAX_TOKENS").unwrap_or(String::from("1200")).parse::<u16>().unwrap_or(1200),
        model: String::from("gpt-3.5-turbo"),
        temperature: 1,
        n_choices: 1,
        presence_penalty: 0.0,
        frequency_penalty: 0.0,
        image_size: String::from("512x512"),
    };

    // let rpc_client = LLMRpcClient::new(String::from("http://[::1]:50051")).await;
    let openai = OpenAI::new(openai_config);
    let bot = TelegramBot::new(env::var("TELEGRAM_BOT_TOKEN").unwrap(), openai);
    bot.run().await;

    Ok(())
}

