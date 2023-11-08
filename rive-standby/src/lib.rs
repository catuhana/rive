mod event;
pub use event::StandbyEvent;

use rive_models::event::ServerEvent;
use tokio::sync::broadcast::{channel, error::RecvError, Sender};

#[derive(Debug)]
pub struct Standby {
    tx: Sender<ServerEvent>,
}

impl Standby {
    pub fn new() -> Self {
        let (tx, _) = channel(1);
        Standby { tx }
    }

    pub fn update(&self, event: ServerEvent) {
        if self.tx.receiver_count() > 0 {
            self.tx.send(event).expect("non-zero amount of receivers");
        }
    }

    pub async fn wait_for<T>(&self, predictate: impl Fn(&T) -> bool) -> T
    where
        T: StandbyEvent,
    {
        let mut rx = self.tx.subscribe();

        loop {
            match rx.recv().await {
                Ok(event) => match T::from_server_event(event) {
                    Some(event) if predictate(&event) => return event,
                    _ => continue,
                },
                Err(RecvError::Lagged(_)) => continue,
                Err(RecvError::Closed) => unreachable!("channel sender must not close"),
            };
        }
    }
}
