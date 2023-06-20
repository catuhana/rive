use rive_models::{data::CreateEmojiData, emoji::Emoji};

use crate::prelude::*;

impl Client {
    /// Fetch an emoji by its ID.
    pub async fn fetch_emoji(&self, id: impl Into<String>) -> Result<Emoji> {
        Ok(self
            .client
            .get(ep!(self, "/custom/emoji/{}", id.into()))
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
        id: impl Into<String>,
        data: CreateEmojiData,
    ) -> Result<Emoji> {
        Ok(self
            .client
            .put(ep!(self, "/custom/emoji/{}", id.into()))
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
    pub async fn delete_emoji(&self, id: impl Into<String>) -> Result<()> {
        self.client
            .delete(ep!(self, "/custom/emoji/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
