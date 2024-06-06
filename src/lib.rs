extern crate log;

use dotenvy::dotenv;
use log::{debug, error, info, warn};
use openai::{chat, set_key};
use std::env;

const SYSTEM_PROMPT: &str = "You are a writer for a developer that works on software, open source, architecture and productivity. Your job is to write a blog post about a subject and develop it to completion. All output should be markdown based.";

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
                content: Some(SYSTEM_PROMPT.to_string()),
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
