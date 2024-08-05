use std::{env, error::Error, sync::Arc};

use rive_models::{authentication::Authentication, event::ServerEvent};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let auth = Authentication::BotToken(env::var("TOKEN")?);

    let mut gateway = rive_gateway::Gateway::new(auth.clone());
    let http = Arc::new(rive_http_new::Client::new(auth.clone()));

    loop {
        match gateway.next_event().await {
            Ok(event) => {
                tokio::spawn(handle_event(event, http.clone()));
            }
            Err(err) => {
                tracing::warn!("{err:#?}");
                break;
            }
        }
    }

    Ok(())
}

async fn handle_event(
    event: ServerEvent,
    http: Arc<rive_http_new::Client>,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    if let ServerEvent::Message(message) = event {
        if message.content.is_some_and(|c| c.starts_with("!ping")) {
            http.send_message(&message.channel).content("Pong!").await?;
        };
    }

    Ok(())
}
