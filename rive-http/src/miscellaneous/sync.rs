use crate::prelude::*;
use rive_models::{channel::ChannelUnread, payload::FetchMessagesPayload, user::UserSettings};
use std::collections::HashMap;

impl RevoltHttp {
    /// Fetch settings from server filtered by keys.
    pub async fn fetch_settings(&self, payload: FetchMessagesPayload) -> Result<UserSettings> {
        Ok(self
            .client
            .post(ep!(self, "/sync/settings/fetch"))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Upload data to save to settings.
    pub async fn set_settings(&self, payload: HashMap<String, String>) -> Result<()> {
        self.client
            .post(ep!(self, "/sync/settings/set"))
            .json(&payload)
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
