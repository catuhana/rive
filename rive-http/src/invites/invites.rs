use crate::prelude::*;
use rive_models::{
    id::{marker::InviteMarker, Id},
    invite::{Invite, InviteJoin},
};

impl Client {
    /// Fetch an invite by its ID.
    pub async fn fetch_invite(&self, id: &Id<InviteMarker>) -> Result<Invite> {
        Ok(self
            .client
            .get(ep!(self, "/invites/{}", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Join an invite by its ID.
    pub async fn join_invite(&self, id: &Id<InviteMarker>) -> Result<InviteJoin> {
        Ok(self
            .client
            .post(ep!(self, "/invites/{}", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Delete an invite by its ID.
    pub async fn delete_invite(&self, id: &Id<InviteMarker>) -> Result<()> {
        self.client
            .delete(ep!(self, "/invites/{}", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
