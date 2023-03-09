use std::error::Error;

use async_openai::{Client, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessageArgs}};
use log::info;
use teloxide::types::ChatId;

#[derive(Clone)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub show_usage: bool,
    pub max_history_size: u16,
    pub max_conversation_age_minutes: u16,
    pub assistant_prompt: String,
    pub max_tokens: u16,
    pub model: String,
    pub temperature: u8,
    pub n_choices: u8,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub image_size: String,
}

#[derive(Clone)]
pub struct OpenAI {
    pub client: Client,
    pub config: OpenAIConfig,
}

impl OpenAI {
    pub fn new(config: OpenAIConfig) -> OpenAI {
        OpenAI {
            client: Client::new(),
            config,
        }
    }

    pub async fn get_chat_response(self, query: String) -> Result<String, Box<dyn Error>> {
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(self.config.max_tokens)
            .model(self.config.model)
            .messages([
                ChatCompletionRequestMessageArgs::default()
                    .content(query)
                    .build()?,
            ])
            .temperature(self.config.temperature)
            .n(self.config.n_choices)
            .presence_penalty(self.config.presence_penalty)
            .frequency_penalty(self.config.frequency_penalty)
            .build()?;

        let chat = self.client.chat();
        let response = chat.create(request).await?;

        if response.choices.len() > 0 {
            let answer = response.choices[0].message.content.clone();

            return Ok(answer);
        } else {
            return Ok(String::from(""));
            // return Err(("An error has occurred, Please try again in a while."))
        }
    }
}
