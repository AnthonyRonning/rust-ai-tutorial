mod config;

use anyhow::Result;

fn main() -> Result<()> {
    // Load configuration
    let config = match config::Config::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    println!("CLI Chat initialized successfully!");
    println!("Using model: {}", config.model);
    println!("API URL: {}", config.openai_base_url);
    
    Ok(())
}
