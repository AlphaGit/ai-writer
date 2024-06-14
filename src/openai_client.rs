use crate::{config::{self, Config}, prompt_builder};
use log::debug;
use openai;

pub async fn get_completion(prompt: &str) -> Option<String> {
    let openai_api_key = config::read_config(Config::OpenAiApiKey);
    openai::set_key(openai_api_key?);

    debug!("Prompt:\n{}", prompt);

    let completion = openai::chat::ChatCompletionBuilder::default()
        .model("gpt-4o")
        .messages(vec![
            openai::chat::ChatCompletionMessage {
                role: openai::chat::ChatCompletionMessageRole::System,
                name: None,
                function_call: None,
                content: Some(prompt_builder::SYSTEM_PROMPT.to_string()),
            },
            openai::chat::ChatCompletionMessage {
                role: openai::chat::ChatCompletionMessageRole::User,
                name: None,
                function_call: None,
                content: Some(prompt.to_string()),
            },
        ])
        .create()
        .await
        .ok()?;

    let first_choice = completion.choices.first()?;
    let response = first_choice.message.content.as_ref()?;

    debug!("Completion Response:\n{}", response);

    Some(response.to_string())
}
