use crate::prelude::*;
use rive_models::{
    channel::PartialInvite,
    member::{Member, MemberList},
    data::{BanUserData, EditMemberData, FetchMembersData},
    server::{BanList, ServerBan},
};

impl Client {
    /// Fetch all server members.
    pub async fn fetch_members(
        &self,
        server_id: impl Into<String>,
        data: FetchMembersData,
    ) -> Result<MemberList> {
        Ok(self
            .client
            .get(ep!(self, "/servers/{}/members", server_id.into()))
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
        server_id: impl Into<String>,
        member_id: impl Into<String>,
    ) -> Result<Member> {
        Ok(self
            .client
            .get(ep!(
                self,
                "/servers/{}/members/{}",
                server_id.into(),
                member_id.into()
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
        server_id: impl Into<String>,
        member_id: impl Into<String>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/servers/{}/members/{}",
                server_id.into(),
                member_id.into()
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
        server_id: impl Into<String>,
        member_id: impl Into<String>,
        data: EditMemberData,
    ) -> Result<Member> {
        Ok(self
            .client
            .patch(ep!(
                self,
                "/servers/{}/members/{}",
                server_id.into(),
                member_id.into()
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
        server_id: impl Into<String>,
        user_id: impl Into<String>,
        data: BanUserData,
    ) -> Result<ServerBan> {
        Ok(self
            .client
            .put(ep!(
                self,
                "/servers/{}/bans/{}",
                server_id.into(),
                user_id.into()
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
        server_id: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/servers/{}/bans/{}",
                server_id.into(),
                user_id.into()
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
    pub async fn fetch_bans(&self, server_id: impl Into<String>) -> Result<BanList> {
        Ok(self
            .client
            .get(ep!(self, "/servers/{}/bans", server_id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch all server invites.
    pub async fn fetch_invites(&self, server_id: impl Into<String>) -> Result<PartialInvite> {
        Ok(self
            .client
            .get(ep!(self, "/servers/{}/invites", server_id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
