# rive-cache-inmemory

`rive-cache-inmemory` is an implementation of an in-memory cache for the [Rive](https://docs.rs/rive) ecosystem. It's intended to be used only within the current process.

It processes incoming events, and adds/modifies/removes resources depending on the event type and data.

There's also a simple API for iterating over resource entities and getting cache statistics (such as the number of stored users).

## Example

Update a cache with incoming events from the gateway:

```rust no_run
use std::{env, error::Error};

use rive_cache_inmemory::InMemoryCache;
use rive_gateway::Gateway;
use rive_models::authentication::Authentication;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let auth = Authentication::SessionToken(env::var("TOKEN")?);

    let mut gateway = Gateway::new(auth);

    // Create a cache with messages and emojis caching disabled:
    let cache = InMemoryCache::builder()
        .cache_messages(false)
        .cache_emojis(false)
        .build();

    loop {
        match gateway.next_event().await {
            Ok(event) => {
                // Update the cache with the event:
                cache.update(&event);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
```
