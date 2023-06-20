use crate::prelude::*;
use rive_models::{
    message::{BulkMessageResponse, Message},
    data::{
        BulkDeleteMessagesData, EditMessageData, FetchMessagesData,
        SearchForMessagesData, SendMessageData,
    },
};

impl Client {
    /// Lets the server and all other clients know that we've seen this message id in this channel.
    pub async fn acknowledge_message(
        &self,
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Result<()> {
        self.client
            .put(ep!(
                self,
                "/channels/{}/ack/{}",
                channel_id.into(),
                message_id.into()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Fetch multiple messages.
    pub async fn fetch_messages(
        &self,
        channel_id: impl Into<String>,
        data: FetchMessagesData,
    ) -> Result<BulkMessageResponse> {
        Ok(self
            .client
            .get(ep!(self, "/channels/{}/messages", channel_id.into()))
            .auth(&self.authentication)
            .query(&data)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Send a message to a given channel.
    pub async fn send_message(
        &self,
        channel_id: impl Into<String>,
        data: SendMessageData,
    ) -> Result<Message> {
        Ok(self
            .client
            .post(ep!(self, "/channels/{}/messages", channel_id.into()))
            .auth(&self.authentication)
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Search for messages within the given parameters.
    pub async fn search_for_messages(
        &self,
        channel_id: impl Into<String>,
        data: SearchForMessagesData,
    ) -> Result<Message> {
        Ok(self
            .client
            .post(ep!(self, "/channels/{}/messages/search", channel_id.into()))
            .auth(&self.authentication)
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Retrieves a message by its ID.
    pub async fn fetch_message(
        &self,
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Result<Message> {
        Ok(self
            .client
            .get(ep!(
                self,
                "/channels/{}/messages/{}",
                channel_id.into(),
                message_id.into()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Delete a message you've sent or one you have permission to delete.
    pub async fn delete_message(
        &self,
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/channels/{}/messages/{}",
                channel_id.into(),
                message_id.into()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    pub async fn edit_message(
        &self,
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
        data: EditMessageData,
    ) -> Result<Message> {
        Ok(self
            .client
            .patch(ep!(
                self,
                "/channels/{}/messages/{}",
                channel_id.into(),
                message_id.into()
            ))
            .auth(&self.authentication)
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Delete multiple messages you've sent or one you have permission to delete.
    ///
    /// This will always require ManageMessages permission regardless of whether you own the message or not.
    ///
    /// Messages must have been sent within the past 1 week.
    pub async fn bulk_delete_messages(
        &self,
        channel_id: impl Into<String>,
        data: BulkDeleteMessagesData,
    ) -> Result<()> {
        self.client
            .delete(ep!(self, "/channels/{}/messages/bulk", channel_id.into(),))
            .auth(&self.authentication)
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
