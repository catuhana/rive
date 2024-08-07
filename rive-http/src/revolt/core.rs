use rive_models::core::InstanceConfiguration;

use crate::prelude::*;

impl Client {
    pub async fn query_node(&self) -> Result<InstanceConfiguration> {
        Ok(self
            .client
            .get(ep!(self, "/"))
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
