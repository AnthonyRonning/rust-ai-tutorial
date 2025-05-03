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
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::{
    signal::ctrl_c,
    sync::mpsc,
    select,
};

// Signal to control application shutdown
static RUNNING: AtomicBool = AtomicBool::new(true);

// Function to handle clean-up operations
fn cleanup() {
    println!("\nGoodbye! Thank you for using Rust CLI Chat.");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up a channel for user input
    let (input_sender, mut input_receiver) = mpsc::channel::<String>(10);
    
    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // Spawn a task to handle Ctrl+C
    tokio::spawn(async move {
        if let Ok(()) = ctrl_c().await {
            println!("\nReceived shutdown signal...");
            running_clone.store(false, Ordering::SeqCst);
            RUNNING.store(false, Ordering::SeqCst);
        }
    });
    
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
    println!("Press Ctrl+C to quit at any time.");
    println!("===================================");
    println!();
    
    // Spawn a task to read user input
    let input_task = tokio::spawn(async move {
        // Display initial prompt
        print!("You> ");
        io::stdout().flush().unwrap();
        
        loop {
            if !RUNNING.load(Ordering::SeqCst) {
                break;
            }
            
            // Read user input (blocking operation)
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    // Trim whitespace
                    let input = input.trim().to_string();
                    
                    // Check for exit command
                    if input == "/exit" {
                        RUNNING.store(false, Ordering::SeqCst);
                        break;
                    }
                    
                    // Skip empty messages
                    if input.is_empty() {
                        // Show prompt again for empty input
                        print!("You> ");
                        io::stdout().flush().unwrap();
                        continue;
                    }
                    
                    // Send input to the main task
                    if let Err(_) = input_sender.send(input).await {
                        // Channel closed, time to exit
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    RUNNING.store(false, Ordering::SeqCst);
                    break;
                }
            }
        }
    });
    
    // Chat loop
    while running.load(Ordering::SeqCst) {
        // Wait for user input or Ctrl+C
        select! {
            Some(input) = input_receiver.recv() => {
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
                    // Check if we received an interrupt
                    if !running.load(Ordering::SeqCst) {
                        break;
                    }
                    
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
                
                // New line after the response and display next prompt
                println!();
                print!("You> ");
                io::stdout().flush()?;
            }
            else => {
                // Channel closed or Ctrl+C was pressed
                break;
            }
        }
    }
    
    // Wait for input task to complete
    let _ = input_task.await;
    
    // Clean up resources
    cleanup();
    
    Ok(())
}
