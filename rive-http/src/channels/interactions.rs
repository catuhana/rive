use rive_models::{
    data::RemoveReactionToMessageData,
    id::{
        marker::{ChannelMarker, EmojiMarker, MessageMarker},
        Id,
    },
};

use crate::prelude::*;

impl Client {
    /// React to a given message.
    pub async fn add_reaction_to_message(
        &self,
        channel_id: &Id<ChannelMarker>,
        message_id: &Id<MessageMarker>,
        emoji: &Id<EmojiMarker>,
    ) -> Result<()> {
        self.client
            .put(ep!(
                self,
                "/channels/{}/messages/{}/reactions/{}",
                channel_id.value_ref(),
                message_id.value_ref(),
                emoji.value_ref()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Remove your own, someone else's or all of a given reaction.
    ///
    /// Requires [`Permission::ManageMessages`] if changing others' reactions.
    ///
    /// [`Permission::ManageMessages`]: rive_models::permission::Permission::ManageMessages
    pub async fn remove_reaction_to_message(
        &self,
        channel_id: &Id<ChannelMarker>,
        message_id: &Id<MessageMarker>,
        emoji: &Id<EmojiMarker>,
        data: RemoveReactionToMessageData,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/channels/{}/messages/{}/reactions/{}",
                channel_id.value_ref(),
                message_id.value_ref(),
                emoji.value_ref()
            ))
            .query(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Remove your own, someone else's or all of a given reaction.
    ///
    /// Requires [`Permission::ManageMessages`].
    ///
    /// [`Permission::ManageMessages`]: rive_models::permission::Permission::ManageMessages
    pub async fn remove_all_reactions_from_message(
        &self,
        channel_id: &Id<ChannelMarker>,
        message_id: &Id<MessageMarker>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/channels/{}/messages/{}/reactions",
                channel_id.value_ref(),
                message_id.value_ref(),
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
