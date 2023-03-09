use std::{error::Error, collections::HashMap };

use async_openai::{Client, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessageArgs, ChatCompletionRequestMessage, Role}};
use chrono::{Utc, TimeZone};
use teloxide::{types::ChatId, prelude::Dialogue, dispatching::dialogue::ErasedStorage};

#[derive(Clone, Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageArgs {
    role: Role,
    content: String,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ChatState {
    pub conversations: HashMap<i64, Vec<MessageArgs>>,
    pub last_updated: HashMap<i64, i64>
}

pub type ChatDialogue = Dialogue<ChatState, ErasedStorage<ChatState>>;
pub type ChatStorage = std::sync::Arc<ErasedStorage<ChatState>>;

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

    pub async fn get_chat_response(&self, chat_id: ChatId, dialogue: ChatDialogue, query: String) -> Result<String, Box<dyn Error + Send + Sync>> {
        let chat_state = &mut dialogue.get().await.unwrap().unwrap();
        if !chat_state.conversations.contains_key(&chat_id.0) || self._max_age_reached(chat_id, chat_state) {
            self.reset_chat_history(chat_id, chat_state);
        }

        chat_state.last_updated.insert(chat_id.0, Utc::now().timestamp_millis());

        let history = chat_state.conversations.get(&chat_id.0).unwrap();

        if history.len() > self.config.max_history_size as usize {
            // TODO: should summary
            self.reset_chat_history(chat_id, chat_state);
        }

        self._add_to_history(chat_id, Role::User, query, chat_state);

        let messages: Vec<ChatCompletionRequestMessage> = chat_state.conversations.get(&chat_id.0).unwrap().iter().map(|x| ChatCompletionRequestMessageArgs::default()
            .role(x.role.clone())
            .content(x.content.clone())
            .build().unwrap()).collect();

        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(self.config.max_tokens)
            .model(&self.config.model)
            .messages(messages)
            .temperature(self.config.temperature)
            .n(self.config.n_choices)
            .presence_penalty(self.config.presence_penalty)
            .frequency_penalty(self.config.frequency_penalty)
            .build()?;

        dialogue.update(chat_state.to_owned()).await?;

        let chat = self.client.chat();
        let response = chat.create(request).await?;

        if response.choices.len() > 0 {
            let answer = response.choices[0].message.content.clone();
            self._add_to_history(chat_id, Role::Assistant, answer.clone(), chat_state);

            return Ok(answer);
        } else {
            return Ok(String::from(""));
        }
    }

    pub fn reset_chat_history(&self, chat_id: ChatId, chat_state: &mut ChatState) {
        chat_state.conversations.insert(chat_id.0, vec![
            MessageArgs {
                role: Role::System,
                content: self.config.assistant_prompt.clone(),
            }
        ]);
    }

    fn _add_to_history(&self, chat_id: ChatId, role: Role, content: String, chat_state: &mut ChatState) {
        let mut history = chat_state.conversations.get(&chat_id.0).unwrap().to_vec();
        history.push(
            MessageArgs { role, content }
        );
        chat_state.conversations.insert(chat_id.0, history);
    }

    fn _max_age_reached(&self, chat_id: ChatId, chat_state: &mut ChatState) -> bool {
        if !chat_state.last_updated.contains_key(&chat_id.0) {
            return false;
        }

        let last_updated = chat_state.last_updated.get(&chat_id.0).unwrap();
        let now = Utc::now();
        let max_age_minutes = self.config.max_conversation_age_minutes;
        let duration = now.signed_duration_since(Utc.timestamp_millis_opt(last_updated.to_owned()).unwrap());
        duration.num_minutes() > max_age_minutes as i64
    }
}
