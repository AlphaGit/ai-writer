use dotenvy::dotenv;
use std::env;
use log::debug;

pub enum Config {
    OpenAiApiKey,
    EditorProgram,
    EditorArgs,
}

impl Config {
    pub fn get_value(&self) -> &str {
        match self {
            Config::OpenAiApiKey => "OPENAI_API_KEY",
            Config::EditorProgram => "EDITOR_PROGRAM",
            Config::EditorArgs => "EDITOR_ARGS",
        }
    }
}

pub fn read_config(config: Config) -> Option<String> {
    debug!("Loading dotenv file");
    dotenv().expect("Failed to read .env file");

    let config_name = config.get_value();

    debug!("Config name: {:?}", config_name);
    env::var(config_name).ok()
}