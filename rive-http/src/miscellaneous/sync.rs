use crate::prelude::*;
use rive_models::{channel::ChannelUnread, data::FetchMessagesData, user::UserSettings};
use std::collections::HashMap;

impl Client {
    /// Fetch settings from server filtered by keys.
    pub async fn fetch_settings(&self, data: &FetchMessagesData) -> Result<UserSettings> {
        Ok(self
            .client
            .post(ep!(self, "/sync/settings/fetch"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Upload data to save to settings.
    pub async fn set_settings(&self, data: &HashMap<String, String>) -> Result<()> {
        self.client
            .post(ep!(self, "/sync/settings/set"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Fetch information about unread state on channels.
    pub async fn fetch_unreads(&self) -> Result<Vec<ChannelUnread>> {
        Ok(self
            .client
            .get(ep!(self, "/sync/unreads"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
