use crate::prelude::*;
use rive_models::{message::BulkMessageResponse, data::FetchMessagesData, stats::Stats};

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
        data: FetchMessagesData,
    ) -> Result<BulkMessageResponse> {
        Ok(self
            .client
            .get(ep!(self, "/admin/messages"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .json()
            .await?)
    }
}
