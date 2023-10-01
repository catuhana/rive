use rive_models::{
    data::CreateEmojiData,
    emoji::Emoji,
    id::{
        marker::{AttachmentMarker, EmojiMarker},
        Id,
    },
};

use crate::prelude::*;

impl Client {
    /// Fetch an emoji by its ID.
    pub async fn fetch_emoji(&self, id: &Id<EmojiMarker>) -> Result<Emoji> {
        Ok(self
            .client
            .get(ep!(self, "/custom/emoji/{}", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Create an emoji by its Autumn upload id.
    pub async fn create_new_emoji(
        &self,
        id: &Id<AttachmentMarker>,
        data: &CreateEmojiData,
    ) -> Result<Emoji> {
        Ok(self
            .client
            .put(ep!(self, "/custom/emoji/{}", id.value_ref()))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Delete an emoji by its id
    pub async fn delete_emoji(&self, id: &Id<EmojiMarker>) -> Result<()> {
        self.client
            .delete(ep!(self, "/custom/emoji/{}", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
