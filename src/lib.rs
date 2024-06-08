extern crate log;

pub mod prompt_builder;

use dotenvy::dotenv;
use log::{debug, error, info, warn};
use openai::{chat, set_key};
use std::env;

pub async fn get_completion(prompt: &str) -> String {
    dotenv().unwrap();
    set_key(env::var("OPENAI_API_KEY").unwrap());

    debug!("Prompt:\n{}", prompt);

    let completion = chat::ChatCompletionBuilder::default()
        .model("gpt-4o")
        .messages(vec![
            chat::ChatCompletionMessage {
                role: chat::ChatCompletionMessageRole::System,
                name: None,
                function_call: None,
                content: Some(prompt_builder::SYSTEM_PROMPT.to_string()),
            },
            chat::ChatCompletionMessage {
                role: chat::ChatCompletionMessageRole::User,
                name: None,
                function_call: None,
                content: Some(prompt.to_string()),
            },
        ])
        .create()
        .await
        .unwrap();

    let first_choice = completion.choices.first().unwrap();
    let response = first_choice.message.content.as_ref().unwrap();

    debug!("Completion Response:\n{}", response);

    response.to_string()
}
