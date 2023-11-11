# rive-standby

`rive-standby` is an implementation of a special structure that processes incoming items and allows tasks to wait for some specific event that satisfies some given condition.

Suppose we want to wait in our client for a ✅ or ❌ response from a certain user to confirm an action. To do this, we can use application-level state or create a special stream that filters incoming events from the main event loop, but this is not very convenient and may not always be suitable. And that's what this crate is for.

It is used for the Rive crates ecosystem and is built on top of it. See the [`rive`](https://docs.rs/rive) documentation for more information.

## Examples

### Briefly

Wait for a message in channel with ID `AAA` from channel with id `BBB` with content "test":

```rust no_run
use rive_models::message::Message;
use rive_standby::Standby;

#[tokio::main]
async fn main() {
    let standby = Standby::new();

    let event = standby
        .wait_for::<Message>(|event| {
            event.channel.value_ref() == "AAA"
                && event.author.value_ref() == "BBB"
                && event.content.as_ref().is_some_and(|c| c == "test")
        })
        .await;

    println!("{event:#?}");
}
```

### Full example

Connect to the gateway, process events and wait for reaction:

```rust no_run
use std::{env, sync::Arc};

use rive_gateway::Gateway;
use rive_models::{
    authentication::Authentication,
    event::{MessageReactEvent, ServerEvent},
    message::Message,
};
use rive_standby::Standby;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let auth = Authentication::BotToken(env::var("TOKEN")?);

    let mut gateway = Gateway::new(auth.clone());
    let standby = Arc::new(Standby::new());

    loop {
        let event = match gateway.next_event().await {
            Ok(event) => event,
            Err(err) => {
                tracing::warn!(?err);
                break;
            }
        };

        // Feed a bystander with an event, which will fulfill any futures
        // that are waiting for the specific event.
        standby.process(event.clone());

        match event {
            ServerEvent::Message(msg) => {
                if msg.content.as_ref().is_some_and(|c| c == "!react") {
                    tokio::spawn(react(msg, Arc::clone(&standby)));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

// Wait for a reaction from the message author, and print it once they react.
async fn react(msg: Message, standby: Arc<Standby>) -> Result<(), anyhow::Error> {
    let event = standby
        .wait_for::<MessageReactEvent>(|event| event.user_id == msg.author && event.id == msg.id)
        .await;

    println!("User @{} reacted with :{}:", event.user_id, event.emoji_id);

    Ok(())
}
```

### Timeout

`rive-standby` does not provide a built-in timeout mechanism. Instead, it is recommended to use [`timeout`] from `tokio`.

In this example, we wait for any message from user with ID `AAA` in channel `BBB` for 10 seconds:

```rust no_run
use std::time::Duration;

use rive_models::message::Message;
use rive_standby::Standby;
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let standby = Standby::new();

    let event_future = standby.wait_for::<Message>(|event| {
        event.channel.value_ref() == "AAA" && event.author.value_ref() == "BBB"
    });

    match timeout(Duration::from_secs(10), event_future).await {
        Ok(event) => println!("{event:#?}"),
        Err(_) => eprintln!("did not receive the event within 10 seconds"),
    }
}
```

[`timeout`]: https://docs.rs/tokio/latest/tokio/time/fn.timeout.html
