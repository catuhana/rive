use rive_models::data::RemoveReactionToMessageData;

use crate::prelude::*;

impl Client {
    /// React to a given message.
    pub async fn add_reaction_to_message(
        &self,
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
        emoji: impl Into<String>,
    ) -> Result<()> {
        self.client
            .put(ep!(
                self,
                "/channels/{}/messages/{}/reactions/{}",
                channel_id.into(),
                message_id.into(),
                emoji.into()
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
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
        emoji: impl Into<String>,
        data: RemoveReactionToMessageData,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/channels/{}/messages/{}/reactions/{}",
                channel_id.into(),
                message_id.into(),
                emoji.into()
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
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/channels/{}/messages/{}/reactions",
                channel_id.into(),
                message_id.into(),
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
