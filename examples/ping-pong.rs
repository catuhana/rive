use std::{env, error::Error};

use rive_models::{authentication::Authentication, data::SendMessageData, event::ServerEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    let auth = Authentication::BotToken(env::var("TOKEN")?);

    let mut gateway = rive_gateway::Gateway::new(auth.clone());
    let http = rive_http::Client::new(auth.clone());

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
    http: rive_http::Client,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    if let ServerEvent::Message(message) = event {
        if message.content.is_some_and(|c| c.starts_with("!ping")) {
            let data = SendMessageData {
                content: Some("Pong!".to_owned()),
                ..Default::default()
            };
            http.send_message(&message.channel, &data).await?;
        };
    }

    Ok(())
}
