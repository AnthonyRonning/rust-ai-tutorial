use anyhow::{Context, Result};
use std::env;

pub struct Config {
    pub openai_api_key: String,
    pub openai_base_url: String,
    pub model: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        // Get API key with validation
        let openai_api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable is required. Please add it to your .env file or set it in your environment")?;

        // Get base URL with default
        let openai_base_url = env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com".to_string());

        // Get model with default
        let model = env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

        Ok(Config {
            openai_api_key,
            openai_base_url,
            model,
        })
    }
}