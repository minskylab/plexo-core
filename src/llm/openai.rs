use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

use crate::config::LLM_MODEL_NAME;

#[derive(Clone)]
pub struct LLMEngine {
    client: Client<OpenAIConfig>,
}

impl LLMEngine {
    pub fn new() -> Self {
        let client = Client::new();
        Self { client }
    }

    pub async fn chat_completion(&self, system_message: String, user_message: String) -> String {
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model(LLM_MODEL_NAME.to_string())
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_message)
                    .build()
                    .unwrap()
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_message)
                    .build()
                    .unwrap()
                    .into(),
            ])
            .build()
            .unwrap();

        let response = self.client.chat().create(request).await.unwrap();

        response
            .choices
            .first()
            .unwrap()
            .message
            .content
            .clone()
            .unwrap()
    }
}
