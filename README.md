# Rust AI Chat CLI

A minimal Rust CLI application that connects to OpenAI-compatible endpoints for streaming chat completions. This is a simple but complete example that demonstrates:

- Connecting to any OpenAI-compatible API endpoint
- Streaming responses from the API token-by-token
- Maintaining conversation history

## Building This Project Step by Step - Tutorial

Follow these steps with the provided prompts to recreate this project from scratch. Each prompt is designed to guide you through implementing a specific portion of the application.

### 1. Create a New Rust Project

```
I want to create a simple command-line chat application using Rust. Can you help me set up a new Rust project for this? I'd like to name it something like "cli-chat" and initialize it with Git so I can track my changes. Just a basic starter project that prints a hello message would be great to make sure everything's working.
```

### 2. Set Up Project Dependencies 

```
For my Rust chat application, I need to add some dependencies to make it work. I need an async runtime because we'll be making API calls, something to interact with OpenAI's API, a way to load environment variables from a file, something for working with asynchronous streams, and a crate that makes error handling easier. How would I add all these to my Cargo.toml file?
```

### 3. Load Environment Variables

```
I want my chat app to be configurable without changing the code. Let's set it up to load configuration from environment variables, with support for a .env file. I need to load an API key for OpenAI, a base URL for the API (with a sensible default), and let the user choose which AI model to use. If the required API key is missing, the app should show a helpful error message.
```

### 4. Set Up the OpenAI Client

```
Now I need to connect to the OpenAI API. How can I create a client using the credentials we loaded from environment variables? I want to configure it with the API key and base URL, then create a client instance. Also, let's add some welcome messages to let users know the app is ready and how to use it.
```

### 5. Add User Input Handling

```
Let's make the app interactive. I want to create a loop that shows a prompt (like "You> "), waits for the user to type a message, and then processes it. Users should be able to exit the app by typing a special command like "/exit". Make sure the prompt displays correctly before the user types.
```

### 6. Track Conversation History

```
To make the chat feel natural, I want to maintain a conversation history. Let's start with a system message that tells the AI to be helpful, then add each user message to the history as they chat. This way, the AI can remember the context of the conversation.
```

### 7. Create an API Request

```
Now I need to send the user's message to the AI. Let's create a request that includes the model name we loaded from environment variables and the conversation history we've built up. I want to enable streaming so we get responses token-by-token instead of waiting for the complete response.
```

### 8. Handle Streaming Responses

```
When the AI responds, I want to show its reply word by word as it comes in, just like a real conversation. After sending the request, let's set up a loop that processes the streaming response chunks as they arrive. For each piece of text, we should print it immediately and also save the complete response for later.
```

### 9. Save AI Responses to History

```
After the AI finishes responding, I need to save its complete message to our conversation history. This way, when the user sends their next message, the AI will have the full context of everything that's been said so far.
```

### 10. Add Graceful Shutdown

```
Let's make sure our app exits properly. We already have the "/exit" command, but users also expect Ctrl+C to work. How can we handle keyboard interrupts so the app shuts down gracefully in both cases?
```

## Future Tutorial Ideas

Here are potential future enhancements you can add to this basic chat application:

### Conversation Management
   - Save/load conversations to JSON or Markdown files
   - Continue previous conversations on startup
   - Conversation branching to explore different response paths
   - Chat summarization with a `/summary` command
   - Conversation templates with pre-defined flows for specific tasks

### User Experience
   - Command history with up/down arrow navigation (using rustyline)
   - Message editing with `/edit` command to fix your last message
   - Terminal UI with ANSI colors for different speakers
   - Loading spinners during API requests
   - Markdown rendering with syntax highlighting for code blocks

### Configuration & Customization
   - Command-line flags using clap (model, temperature, system prompt file)
   - Configuration file support beyond environment variables
   - Provider profiles to switch between OpenAI, Anthropic, Ollama, etc.
   - Multiple chat personas with different system prompts
   - Customizable message templates for common prompts

### Advanced Features
   - Function calling/tool mode to execute local commands
   - Local context injection with `/file` command for including file contents
   - Token usage tracking and cost estimation
   - Streaming rate meter with tokens/second display
   - Automatic error handling and retries for network issues
   - Prompt chaining for multi-step reasoning pipelines
   - LLM model comparison by running prompts against multiple models

### Analytics & Insights
   - Conversation analytics to identify patterns and topic frequencies
   - Response quality evaluation metrics
   - Token usage statistics and cost tracking
   - Performance benchmarks across different models

### Extensions
   - Embeddings + RAG: Add retrieval from local files to augment prompts
   - Interactive shell mode with command execution and explanation
   - Voice interface with text-to-speech output and speech recognition
   - Plugin architecture for custom extensions
