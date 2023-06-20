use rive_models::{data::CreateWebhookData, webhook::Webhook};

use crate::prelude::*;

impl Client {
    /// Create a webhook which 3rd party platforms can use to send messages.
    pub async fn create_webhook(
        &self,
        channel_id: impl Into<String>,
        data: CreateWebhookData,
    ) -> Result<Webhook> {
        Ok(self
            .client
            .post(ep!(self, "/channels/{}/webhooks", channel_id.into()))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Get all webhooks inside the channel.
    pub async fn get_all_webhooks(&self, channel_id: impl Into<String>) -> Result<Vec<Webhook>> {
        Ok(self
            .client
            .get(ep!(self, "/channels/{}/webhooks", channel_id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
