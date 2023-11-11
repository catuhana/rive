#![doc = include_str!("../README.md")]

mod event;
pub use event::StandbyEvent;

use rive_models::event::ServerEvent;
use tokio::sync::broadcast::{channel, error::RecvError, Sender};

/// A struct used by the main event loop to process incoming events and by tasks
/// to wait for specific events.
///
/// To use a bystander in multiple tasks, consider wrapping it in an
/// [`std::sync::Arc`].
///
/// See the [crate] documentation for more information.
#[derive(Debug)]
pub struct Standby {
    /// Event broadcaster.
    tx: Sender<ServerEvent>,
}

impl Standby {
    /// Create a new [`Standby`].
    pub fn new() -> Self {
        let (tx, _) = channel(1);
        Standby { tx }
    }

    /// Update bystander state by processing an incoming event.
    ///
    /// The method is called in the main event loop.
    pub fn process(&self, event: ServerEvent) {
        if self.tx.receiver_count() > 0 {
            self.tx.send(event).expect("non-zero amount of receivers");
        }
    }

    /// Wait for specific [event].
    ///
    /// [event]: rive_models::event
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

impl Default for Standby {
    fn default() -> Self {
        Self::new()
    }
}
