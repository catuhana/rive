use futures::StreamExt;
use rive::{
    gateway::Gateway,
    http::Client,
    models::{
        authentication::Authentication, data::SendMessageData, event::ServerEvent, message::Message,
    },
};
use std::{env, error::Error};

#[derive(Clone)]
struct Context {
    client: Client,
    gateway: Gateway,
}

type Result<T = ()> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result {
    let auth = Authentication::BotToken(env::var("TOKEN")?);

    let client = Client::new(auth.clone());
    let gateway = Gateway::connect(auth.clone()).await?;

    let ctx = Context { client, gateway };

    while let Some(event) = ctx.gateway.clone().next().await {
        handle_event(&event?, &ctx).await?;
    }

    Ok(())
}

async fn handle_event(event: &ServerEvent, ctx: &Context) -> Result {
    match event {
        ServerEvent::Message(message) => {
            handle_message(&message, ctx).await?;
        }
        ServerEvent::Authenticated => {
            println!("Client is authenticated");
        }
        _ => {}
    };

    Ok(())
}

async fn handle_message(message: &Message, ctx: &Context) -> Result {
    if let Some(ref content) = message.content {
        match content.as_str() {
            "!ping" => ping(message, ctx).await?,
            _ => {}
        };
    }

    Ok(())
}

async fn ping(message: &Message, ctx: &Context) -> Result {
    let data = SendMessageData {
        content: Some("Pong!".to_owned()),
        ..Default::default()
    };
    ctx.client.send_message(&message.channel, data).await?;

    Ok(())
}
