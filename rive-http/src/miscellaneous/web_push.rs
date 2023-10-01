use crate::prelude::*;
use rive_models::data::PushSubscribeData;

impl Client {
    /// Create a new Web Push subscription.
    ///
    /// If an existing subscription exists on this session, it will be removed.
    pub async fn push_subscribe(&self, data: &PushSubscribeData) -> Result<()> {
        self.client
            .post(ep!(self, "/push/subscribe"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Remove the Web Push subscription associated with the current session.
    pub async fn push_unsubscribe(&self) -> Result<()> {
        self.client
            .post(ep!(self, "/push/unsubscribe"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
