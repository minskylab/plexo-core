use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};

// use crate::config::LLM_MODEL_NAME;

#[derive(Clone)]
pub struct LLMEngine {
    client: Client,
}

impl LLMEngine {
    pub fn new() -> Self {
        let client = Client::new();
        Self { client }
    }

    pub async fn chat_completion(&self, system_message: String, user_message: String) -> String {
        todo!();
        // let request = CreateChatCompletionRequestArgs::default()
        //     .max_tokens(512u16)
        //     .model(LLM_MODEL_NAME.to_string())
        //     .messages([
        //         ChatCompletionRequestMessageArgs::default()
        //             .role(Role::System)
        //             .content(system_message)
        //             .build()
        //             .unwrap(),
        //         ChatCompletionRequestMessageArgs::default()
        //             .role(Role::User)
        //             .content(user_message)
        //             .build()
        //             .unwrap(),
        //     ])
        //     .build()
        //     .unwrap();

        // let response = self.client.chat().create(request).await.unwrap();

        // response.choices.first().unwrap().message.content.clone()
    }
}
