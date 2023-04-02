use crate::prelude::*;
use rive_models::{message::BulkMessageResponse, payload::FetchMessagesPayload, stats::Stats};

impl Client {
    /// Fetch various technical statistics.
    pub async fn query_stats(&self) -> Result<Stats> {
        Ok(self
            .client
            .get(ep!(self, "/admin/stats"))
            .auth(&self.authentication)
            .send()
            .await?
            .json()
            .await?)
    }

    /// This is a privileged route to globally fetch messages.
    pub async fn globally_fetch_messages(
        &self,
        payload: FetchMessagesPayload,
    ) -> Result<BulkMessageResponse> {
        Ok(self
            .client
            .get(ep!(self, "/admin/messages"))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .json()
            .await?)
    }
}
