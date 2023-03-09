use futures::StreamExt;
use revolt::{
    gateway::RevoltWs,
    http::RevoltHttp,
    models::{
        authentication::Authentication,
        event::{ClientToServerEvent, ServerToClientEvent},
        message::Message,
        payload::SendMessagePayload,
    },
    util::extensions::BuilderExt,
};
use std::{env, error::Error, sync::Arc, time::Duration};
use tokio::{spawn, sync::Mutex, time::sleep};

struct Context {
    http: RevoltHttp,
    ws: Arc<Mutex<RevoltWs>>,
}

type Result<T = ()> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result {
    let token = env::var("TOKEN")?;

    let http = RevoltHttp::new(Authentication::BotToken(token.clone()));
    let ws = RevoltWs::connect().await?;
    let ws = Arc::new(Mutex::new(ws));

    let ctx = Arc::new(Context { http, ws });

    authenticate(ctx.clone(), token.to_owned()).await?;
    spawn_heartbeat(ctx.clone()).await?;
    listen_to_events(ctx.clone()).await?;

    Ok(())
}

async fn authenticate(ctx: Arc<Context>, token: String) -> Result {
    ctx.ws
        .lock()
        .await
        .send(ClientToServerEvent::Authenticate { token })
        .await?;
    Ok(())
}

async fn spawn_heartbeat(ctx: Arc<Context>) -> Result {
    let heartbeat_ws = Arc::clone(&ctx.ws);
    spawn(async move {
        loop {
            heartbeat_ws
                .lock()
                .await
                .send(ClientToServerEvent::Ping { data: 0 })
                .await
                .unwrap();
            sleep(Duration::from_secs(15)).await;
        }
    });
    Ok(())
}

async fn listen_to_events(ctx: Arc<Context>) -> Result {
    while let Some(event) = ctx.ws.lock().await.next().await {
        let event = dbg!(event?);
        handle_event(event, Arc::clone(&ctx)).await?;
    }
    Ok(())
}

async fn handle_event(event: ServerToClientEvent, ctx: Arc<Context>) -> Result {
    match event {
        ServerToClientEvent::Message(message) => {
            handle_message(message, Arc::clone(&ctx)).await?;
        }
        ServerToClientEvent::Authenticated => {
            println!("Client is authenticated");
        }
        _ => {}
    };

    Ok(())
}

async fn handle_message(message: Message, ctx: Arc<Context>) -> Result {
    if let Some(ref content) = message.content {
        match content.as_str() {
            "!ping" => ping(message, Arc::clone(&ctx)).await?,
            "!pong" => pong(message, Arc::clone(&ctx)).await?,
            _ => {}
        };
    }

    Ok(())
}

async fn ping(message: Message, ctx: Arc<Context>) -> Result {
    let payload = SendMessagePayload::builder().content("Pong!").build();
    ctx.http.send_message(message.channel, payload).await?;

    Ok(())
}

async fn pong(message: Message, ctx: Arc<Context>) -> Result {
    let payload = SendMessagePayload::builder().content("Ping!").build();
    ctx.http.send_message(message.channel, payload).await?;

    Ok(())
}
