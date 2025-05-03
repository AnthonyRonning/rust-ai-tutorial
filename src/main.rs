mod config;

use anyhow::Result;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs,
        ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestMessage,
        CreateChatCompletionRequestArgs,
    },
};
use futures::StreamExt;
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
    
    // Track conversation history with the correct type
    let mut conversation_history: Vec<ChatCompletionRequestMessage> = Vec::new();
    
    // Add system message to set the AI's behavior
    let system_message = ChatCompletionRequestSystemMessageArgs::default()
        .content("You are a helpful AI assistant. Respond concisely and accurately to questions.")
        .build()?;
    conversation_history.push(system_message.into());

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
        
        // Add user message to history
        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(input)
            .build()?;
        conversation_history.push(user_message.into());
        
        // Create the chat completion request
        let request = CreateChatCompletionRequestArgs::default()
            .model(&config.model)
            .messages(conversation_history.clone())
            .stream(true)
            .build()?;
        
        // Send the request and get streaming response
        let mut stream = client.chat().create_stream(request).await?;
        
        // Print AI prefix
        print!("AI> ");
        io::stdout().flush()?;
        
        // Variable to collect the complete response
        let mut full_response = String::new();
        
        // Process streaming response
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    // Extract the content from the response
                    for choice in response.choices {
                        if let Some(content) = choice.delta.content {
                            // Print chunk immediately
                            print!("{}", content);
                            io::stdout().flush()?;
                            
                            // Add to full response
                            full_response.push_str(&content);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("\nError during streaming: {}", err);
                    break;
                }
            }
        }
        
        // Add AI response to conversation history
        if !full_response.is_empty() {
            let assistant_message = ChatCompletionRequestAssistantMessageArgs::default()
                .content(full_response)
                .build()?;
            conversation_history.push(assistant_message.into());
        }
        
        // New line after the response
        println!();
    }
    
    Ok(())
}
