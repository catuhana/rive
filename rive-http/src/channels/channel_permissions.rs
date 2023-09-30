use crate::prelude::*;
use rive_models::{
    channel::Channel,
    data::{SetDefaultPermissionData, SetRolePermissionData},
    id::{
        marker::{ChannelMarker, RoleMarker},
        Id,
    },
};

impl Client {
    /// Sets permissions for the specified role in this channel.
    ///
    /// Channel must be a [Channel::TextChannel] or [Channel::VoiceChannel].
    pub async fn set_role_channel_permissions(
        &self,
        channel_id: &Id<ChannelMarker>,
        role_id: &Id<RoleMarker>,
        data: SetRolePermissionData,
    ) -> Result<Channel> {
        Ok(self
            .client
            .put(ep!(
                self,
                "/channels/{}/permissions/{}",
                channel_id.value_ref(),
                role_id.value_ref()
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

    /// Sets permissions for the specified role in this channel.
    ///
    /// Channel must be a [Channel::Group], [Channel::TextChannel] or [Channel::VoiceChannel].
    pub async fn set_default_channel_permissions(
        &self,
        channel_id: &Id<ChannelMarker>,
        data: SetDefaultPermissionData,
    ) -> Result<Channel> {
        Ok(self
            .client
            .put(ep!(
                self,
                "/channels/{}/permissions/default",
                channel_id.value_ref(),
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
}
