use crate::prelude::*;
use rive_models::{
    channel::Channel,
    data::EditChannelData,
    id::{marker::ChannelMarker, Id},
};

impl Client {
    /// Fetch channel by its ID.
    pub async fn fetch_channel(&self, id: &Id<ChannelMarker>) -> Result<Channel> {
        Ok(self
            .client
            .get(ep!(self, "/channels/{}", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Deletes a server channel, leaves a group or closes a group.
    pub async fn close_channel(&self, id: &Id<ChannelMarker>) -> Result<()> {
        self.client
            .delete(ep!(self, "/channels/{}", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Edit a channel object by its id.
    pub async fn edit_channel(
        &self,
        id: &Id<ChannelMarker>,
        data: &EditChannelData,
    ) -> Result<Channel> {
        Ok(self
            .client
            .patch(ep!(self, "/channels/{}", id.value_ref()))
            .auth(&self.authentication)
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
