use rive::prelude::*;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let auth = Authentication::BotToken(env::var("TOKEN")?);

    let mut rive = Rive::new(auth).await?;

    while let Some(event) = rive.gateway.next().await {
        let event = event?;

        rive.update(&event);

        if let ServerEvent::Message(message) = event {
            if message.content.is_some_and(|c| c.starts_with("!ping")) {
                let data = SendMessageData {
                    content: Some("Pong!".to_owned()),
                    ..Default::default()
                };
                rive.http.send_message(&message.channel, data).await?;
            };
        }
    }

    Ok(())
}
