//! This is an example of basic use of an in-memory cache.

use std::{env, error::Error, sync::Arc};

use rive_cache_inmemory::InMemoryCache;
use rive_gateway::Gateway;
use rive_http::Client;
use rive_models::{authentication::Authentication, data::SendMessageData, event::ServerEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let auth = Authentication::BotToken(env::var("TOKEN")?);

    let http = Client::new(auth.clone());
    let mut gateway = Gateway::new(auth);

    // Create a cache with default configuration
    let cache = Arc::new(InMemoryCache::new());

    loop {
        match gateway.next_event().await {
            Ok(event) => {
                tokio::spawn(handle_event(event, http.clone(), cache.clone()));
            }
            Err(err) => {
                tracing::warn!("received an error: {:#?}", err);
                break;
            }
        }
    }

    Ok(())
}

async fn handle_event(
    event: ServerEvent,
    http: rive_http::Client,
    cache: Arc<InMemoryCache>,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    // Update the cache with the event
    cache.update(&event);

    if let ServerEvent::Message(message) = event {
        let Some(ref content) = message.content else {
            return Ok(());
        };

        // A command that send a number of messages in the cache.
        if content.starts_with("!messages") {
            let count = cache.stats().messages();

            let content = format!("Cached messages: {count}");

            let data = SendMessageData {
                content: Some(&content),
                ..Default::default()
            };
            http.send_message(&message.channel, &data).await?;
        }
        // A command that sends cached user info
        else if content.starts_with("!user") {
            let args = content.split_whitespace().collect::<Vec<&str>>();

            let response = match args.get(1) {
                Some(id) => match cache.user(&id.to_string().into()) {
                    Some(user) => format!("```rust\n{:#?}\n```", user.value()),
                    None => "User not found!".to_string(),
                },
                None => "No ID specified!".to_string(),
            };

            let data = SendMessageData {
                content: Some(&response),
                ..Default::default()
            };
            http.send_message(&message.channel, &data).await?;
        }
    }

    Ok(())
}
