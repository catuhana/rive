use crate::prelude::*;
use rive_models::{
    channel::Channel,
    data::CreateGroupData,
    id::{
        marker::{ChannelMarker, UserMarker},
        Id,
    },
    user::User,
};

impl Client {
    /// Retrieves all users who are part of this group.
    pub async fn fetch_group_members(&self, id: &Id<ChannelMarker>) -> Result<Vec<User>> {
        Ok(self
            .client
            .get(ep!(self, "/channels/{}/members", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Create a new group channel.
    pub async fn create_group(&self, data: CreateGroupData) -> Result<Channel> {
        Ok(self
            .client
            .post(ep!(self, "/channels/create"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Adds another user to the group.
    pub async fn add_member_to_group(
        &self,
        group_id: &Id<ChannelMarker>,
        member_id: &Id<UserMarker>,
    ) -> Result<()> {
        self.client
            .put(ep!(
                self,
                "/channels/{}/recipients/{}",
                group_id.value_ref(),
                member_id.value_ref()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?;
        Ok(())
    }

    /// Removes a user from the group.
    pub async fn remove_member_from_group(
        &self,
        group_id: &Id<ChannelMarker>,
        member_id: &Id<UserMarker>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/channels/{}/recipients/{}",
                group_id.value_ref(),
                member_id.value_ref()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?;
        Ok(())
    }
}
