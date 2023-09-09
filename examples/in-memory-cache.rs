use futures::StreamExt;
use std::{env, error::Error};

use rive_cache_inmemory::InMemoryCache;
use rive_gateway::Gateway;
use rive_http::Client;
use rive_models::{authentication::Authentication, data::SendMessageData, event::ServerEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let auth = Authentication::BotToken(env::var("TOKEN")?);

    let http = Client::new(auth.clone());
    let mut gateway = Gateway::connect(auth).await?;

    // Create a cache with default configuration
    let cache = InMemoryCache::new();

    while let Some(event) = gateway.next().await {
        let event = event?;

        // Update the cache with the event
        cache.update(&event);

        if let ServerEvent::Message(message) = event {
            let Some(content) = message.content else {
                continue;
            };

            // A command that send a number of messages in the cache.
            if content.starts_with("!messages") {
                let count = cache.stats().messages();

                let data = SendMessageData {
                    content: Some(format!("Cached messages: {count}")),
                    ..Default::default()
                };
                http.send_message(message.channel, data).await?;
            }
            // A command that sends cached user info
            else if content.starts_with("!user") {
                let args = content.split_whitespace().collect::<Vec<&str>>();

                let response = match args.get(1) {
                    Some(id) => match cache.user(id) {
                        Some(user) => format!("```rust\n{:#?}\n```", user.value()),
                        None => "User not found!".to_string(),
                    },
                    None => "No ID specified!".to_string(),
                };

                let data = SendMessageData {
                    content: Some(response),
                    ..Default::default()
                };
                http.send_message(message.channel, data).await?;
            }
        }
    }

    Ok(())
}
