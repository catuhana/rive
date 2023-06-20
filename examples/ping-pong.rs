use futures::StreamExt;
use rive::{
    gateway::Gateway,
    http::Client,
    models::{
        authentication::Authentication,
        event::{ClientEvent, ServerEvent},
        message::Message,
        data::SendMessageData,
    },
};
use std::{env, error::Error, time::Duration};
use tokio::{spawn, time::sleep};

#[derive(Clone)]
struct Context {
    client: Client,
    gateway: Gateway,
}

type Result<T = ()> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result {
    let token = env::var("TOKEN")?;

    let client = Client::new(Authentication::BotToken(token.clone()));
    let gateway = Gateway::connect().await?;

    let ctx = Context { client, gateway };

    authenticate(&ctx, token.to_owned()).await?;
    spawn_heartbeat(ctx.clone()).await?;
    listen_to_events(ctx.clone()).await?;

    Ok(())
}

async fn authenticate(ctx: &Context, token: String) -> Result {
    ctx.gateway
        .send(ClientEvent::Authenticate { token })
        .await?;
    Ok(())
}

async fn spawn_heartbeat(ctx: Context) -> Result {
    spawn(async move {
        loop {
            ctx.gateway
                .send(ClientEvent::Ping { data: 0 })
                .await
                .unwrap();
            sleep(Duration::from_secs(15)).await;
        }
    });
    Ok(())
}

async fn listen_to_events(ctx: Context) -> Result {
    while let Some(event) = ctx.gateway.clone().next().await {
        let event = event?;
        handle_event(event, ctx.clone()).await?;
    }
    Ok(())
}

async fn handle_event(event: ServerEvent, ctx: Context) -> Result {
    match event {
        ServerEvent::Message(message) => {
            handle_message(message, ctx).await?;
        }
        ServerEvent::Authenticated => {
            println!("Client is authenticated");
        }
        _ => {}
    };

    Ok(())
}

async fn handle_message(message: Message, ctx: Context) -> Result {
    if let Some(ref content) = message.content {
        match content.as_str() {
            "!ping" => ping(message, ctx).await?,
            _ => {}
        };
    }

    Ok(())
}

async fn ping(message: Message, ctx: Context) -> Result {
    let data = SendMessageData {
        content: Some("Pong!".to_owned()),
        ..Default::default()
    };
    ctx.client.send_message(message.channel, data).await?;

    Ok(())
}
