use rive_models::{
    data::CreateWebhookData,
    id::{marker::ChannelMarker, Id},
    webhook::Webhook,
};

use crate::prelude::*;

impl Client {
    /// Create a webhook which 3rd party platforms can use to send messages.
    pub async fn create_webhook(
        &self,
        channel_id: &Id<ChannelMarker>,
        data: CreateWebhookData,
    ) -> Result<Webhook> {
        Ok(self
            .client
            .post(ep!(self, "/channels/{}/webhooks", channel_id.value_ref()))
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
    pub async fn get_all_webhooks(&self, channel_id: &Id<ChannelMarker>) -> Result<Vec<Webhook>> {
        Ok(self
            .client
            .get(ep!(self, "/channels/{}/webhooks", channel_id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
