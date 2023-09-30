use crate::prelude::*;
use rive_models::{
    channel::PartialInvite,
    id::{marker::InviteMarker, Id},
};

impl Client {
    /// Creates an invite to this channel.
    ///
    /// Channel must be a [`Channel::TextChannel`].
    ///
    /// [`Channel::TextChannel`]: rive_models::channel::Channel::TextChannel
    pub async fn create_invite(&self, id: &Id<InviteMarker>) -> Result<PartialInvite> {
        Ok(self
            .client
            .post(ep!(self, "/channels/{}/invites", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
