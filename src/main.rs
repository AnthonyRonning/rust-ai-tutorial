mod config;

use anyhow::Result;
use std::io::{self, Write};
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
    println!("Type '/exit' to end the conversation.");
    println!("===================================");
    println!();
    
    // Chat loop
    loop {
        // Display prompt and flush to ensure it appears before user input
        print!("You> ");
        io::stdout().flush()?;
        
        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        // Trim whitespace
        let input = input.trim();
        
        // Check for exit command
        if input == "/exit" {
            println!("Goodbye! Thank you for using Rust CLI Chat.");
            break;
        }
        
        // Skip empty messages
        if input.is_empty() {
            continue;
        }
        
        // Process the user message (we'll add the AI response in the next step)
        println!("AI> Processing your message: {}", input);
    }
    
    Ok(())
}
