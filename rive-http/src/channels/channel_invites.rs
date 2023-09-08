use crate::prelude::*;
use rive_models::channel::PartialInvite;

impl Client {
    /// Creates an invite to this channel.
    ///
    /// Channel must be a [`Channel::TextChannel`].
    ///
    /// [`Channel::TextChannel`]: rive_models::channel::Channel::TextChannel
    pub async fn create_invite(&self, id: impl Into<String>) -> Result<PartialInvite> {
        Ok(self
            .client
            .post(ep!(self, "/channels/{}/invites", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
