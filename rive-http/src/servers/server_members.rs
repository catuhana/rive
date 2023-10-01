use crate::prelude::*;
use rive_models::{
    channel::PartialInvite,
    data::{BanUserData, EditMemberData, FetchMembersData},
    id::{
        marker::{ServerMarker, UserMarker},
        Id,
    },
    member::{Member, MemberList},
    server::{BanList, ServerBan},
};

impl Client {
    /// Fetch all server members.
    pub async fn fetch_members(
        &self,
        server_id: &Id<ServerMarker>,
        data: &FetchMembersData,
    ) -> Result<MemberList> {
        Ok(self
            .client
            .get(ep!(self, "/servers/{}/members", server_id.value_ref()))
            .query(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Retreive a member.
    pub async fn fetch_member(
        &self,
        server_id: &Id<ServerMarker>,
        member_id: &Id<UserMarker>,
    ) -> Result<Member> {
        Ok(self
            .client
            .get(ep!(
                self,
                "/servers/{}/members/{}",
                server_id.value_ref(),
                member_id.value_ref()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Remove a member from the server.
    pub async fn kick_member(
        &self,
        server_id: &Id<ServerMarker>,
        member_id: &Id<UserMarker>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/servers/{}/members/{}",
                server_id.value_ref(),
                member_id.value_ref()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Edit a member by their ID.
    pub async fn edit_member(
        &self,
        server_id: &Id<ServerMarker>,
        member_id: &Id<UserMarker>,
        data: &EditMemberData,
    ) -> Result<Member> {
        Ok(self
            .client
            .patch(ep!(
                self,
                "/servers/{}/members/{}",
                server_id.value_ref(),
                member_id.value_ref()
            ))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Ban a user by their ID.
    pub async fn ban_user(
        &self,
        server_id: &Id<ServerMarker>,
        user_id: &Id<UserMarker>,
        data: &BanUserData,
    ) -> Result<ServerBan> {
        Ok(self
            .client
            .put(ep!(
                self,
                "/servers/{}/bans/{}",
                server_id.value_ref(),
                user_id.value_ref()
            ))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Remove a user's ban.
    pub async fn unban_user(
        &self,
        server_id: &Id<ServerMarker>,
        user_id: &Id<UserMarker>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/servers/{}/bans/{}",
                server_id.value_ref(),
                user_id.value_ref()
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

    /// Fetch all bans on a server.
    pub async fn fetch_bans(&self, server_id: &Id<ServerMarker>) -> Result<BanList> {
        Ok(self
            .client
            .get(ep!(self, "/servers/{}/bans", server_id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch all server invites.
    pub async fn fetch_invites(&self, server_id: &Id<ServerMarker>) -> Result<PartialInvite> {
        Ok(self
            .client
            .get(ep!(self, "/servers/{}/invites", server_id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
