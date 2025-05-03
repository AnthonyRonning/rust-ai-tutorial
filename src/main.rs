mod config;

use anyhow::Result;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = match config::Config::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    // Create OpenAI client
    let client = config.create_openai_client();

    // Display welcome message
    println!("===================================");
    println!("🤖 Welcome to Rust CLI Chat!");
    println!("===================================");
    println!("Using model: {}", config.model);
    println!("Connected to API: {}", config.openai_base_url);
    println!();
    println!("Type your message and press Enter to chat.");
    println!("Type 'exit' or 'quit' to end the conversation.");
    println!("===================================");
    println!();
    
    // Here we'll add the chat loop functionality later
    
    Ok(())
}
