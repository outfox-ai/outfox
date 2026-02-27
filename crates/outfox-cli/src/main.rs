//! ⚠️ DEMO ONLY - Do not use or waste time on this!
//!
//! This is a simple chat demo to show how to use outfox SDKs.
//! It's not meant for production use.

use std::io::{self, Write};
use std::pin::pin;

use futures_util::StreamExt;

const BANNER: &str = r#"
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║   ⚠️  WARNING: THIS IS JUST A DEMO - DON'T WASTE TIME ON IT!  ║
║                                                               ║
║   A simple chat demo for outfox SDKs.                          ║
║   Type 'quit' or 'exit' to exit.                              ║
║   Type 'openai' or 'zhipu' to switch provider.                ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
"#;

#[derive(Debug, Clone, Copy)]
enum Provider {
    OpenAI,
    Zhipu,
}

#[tokio::main]
async fn main() {
    println!("{}", BANNER);

    let mut provider = Provider::OpenAI;
    let mut history: Vec<(String, String)> = Vec::new();

    loop {
        print!("\n[{:?}] You: ", provider);
        io::stdout().flush().ok();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("Bye! (Remember: this was just a demo!)");
                break;
            }
            "openai" => {
                provider = Provider::OpenAI;
                println!("Switched to OpenAI. Set OPENAI_API_KEY env var.");
                continue;
            }
            "zhipu" => {
                provider = Provider::Zhipu;
                println!("Switched to Zhipu. Set ZHIPUAI_API_KEY env var.");
                continue;
            }
            "clear" => {
                history.clear();
                println!("History cleared.");
                continue;
            }
            _ => {}
        }

        print!("\nAssistant: ");
        io::stdout().flush().ok();

        let response = match provider {
            Provider::OpenAI => chat_openai(input).await,
            Provider::Zhipu => chat_zhipu(input).await,
        };

        match response {
            Ok(text) => {
                history.push((input.to_string(), text));
            }
            Err(e) => {
                println!("\n[Error: {}]", e);
            }
        }
    }
}

async fn chat_openai(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    use outfox_openai::Client;
    use outfox_openai::spec::chat::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestBuilder,
    };

    let client = Client::new();

    let request = CreateChatCompletionRequestBuilder::default()
        .model("gpt-4o-mini")
        .messages(vec![ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage::new(input),
        )])
        .stream(true)
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;
    let mut full_response = String::new();

    while let Some(result) = stream.next().await {
        match result {
            Ok(chunk) => {
                if let Some(choice) = chunk.choices.first() {
                    if let Some(content) = &choice.delta.content {
                        print!("{}", content);
                        io::stdout().flush().ok();
                        full_response.push_str(content);
                    }
                }
            }
            Err(e) => {
                return Err(format!("Stream error: {}", e).into());
            }
        }
    }

    println!();
    Ok(full_response)
}

async fn chat_zhipu(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    use outfox_zhipu::Client;
    use outfox_zhipu::spec::chat::{ChatMessage, CreateChatCompletionRequestArgs};

    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .model("glm-4-flash")
        .messages(vec![ChatMessage::user(input)])
        .build()?;

    let chat = client.chat();
    let stream = chat.create_stream(request).await?;
    let mut stream = pin!(stream);
    let mut full_response = String::new();

    while let Some(result) = stream.next().await {
        match result {
            Ok(chunk) => {
                if let Some(choice) = chunk.choices.first() {
                    if let Some(content) = &choice.delta.content {
                        print!("{}", content);
                        io::stdout().flush().ok();
                        full_response.push_str(content);
                    }
                }
            }
            Err(e) => {
                return Err(format!("Stream error: {}", e).into());
            }
        }
    }

    println!();
    Ok(full_response)
}
