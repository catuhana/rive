use crate::prelude::*;
use rive_models::{
    channel::Channel,
    data::{CreateChannelData, CreateServerData, EditServerData},
    server::Server,
};

impl Client {
    /// Create a new server.
    pub async fn create_server(&self, data: CreateServerData) -> Result<Server> {
        Ok(self
            .client
            .post(ep!(self, "/servers/create"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Deletes a server if owner otherwise leaves.
    pub async fn fetch_server(&self, id: impl Into<String>) -> Result<Server> {
        Ok(self
            .client
            .get(ep!(self, "/servers/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Deletes a server if owner otherwise leaves.
    pub async fn delete_or_leave_server(&self, id: impl Into<String>) -> Result<()> {
        self.client
            .delete(ep!(self, "/servers/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?;
        Ok(())
    }

    /// Edit a server by its id.
    pub async fn edit_server(
        &self,
        id: impl Into<String>,
        data: EditServerData,
    ) -> Result<Server> {
        Ok(self
            .client
            .patch(ep!(self, "/servers/{}", id.into()))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Mark all channels in a server as read.
    pub async fn mark_server_as_read(&self, id: impl Into<String>) -> Result<()> {
        self.client
            .put(ep!(self, "/servers/{}/ack", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?;
        Ok(())
    }

    /// Create a new Text or Voice channel
    pub async fn create_channel(
        &self,
        server_id: impl Into<String>,
        data: CreateChannelData,
    ) -> Result<Channel> {
        Ok(self
            .client
            .post(ep!(self, "/servers/{}/channels", server_id.into()))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
