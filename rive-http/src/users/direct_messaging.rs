use crate::prelude::*;
use rive_models::{
    channel::Channel,
    id::{marker::UserMarker, Id},
};

impl Client {
    /// This fetches your direct messages, including any DM and group DM conversations.
    pub async fn fetch_direct_message_channels(&self) -> Result<Vec<Channel>> {
        Ok(self
            .client
            .get(ep!(self, "/users/dms"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Open a DM with another user.
    ///
    /// If the target is oneself, a saved messages channel is returned.
    pub async fn open_direct_message(&self, id: &Id<UserMarker>) -> Result<Channel> {
        Ok(self
            .client
            .get(ep!(self, "/users/{}/dm", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
