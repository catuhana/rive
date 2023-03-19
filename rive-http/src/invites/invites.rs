use crate::prelude::*;
use rive_models::invite::{Invite, InviteJoin};

impl RevoltHttp {
    /// Fetch an invite by its ID.
    pub async fn fetch_invite(&self, id: impl Into<String>) -> Result<Invite> {
        Ok(self
            .client
            .get(ep!(self, "/invites/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Join an invite by its ID.
    pub async fn join_invite(&self, id: impl Into<String>) -> Result<InviteJoin> {
        Ok(self
            .client
            .post(ep!(self, "/invites/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Delete an invite by its ID.
    pub async fn delete_invite(&self, id: impl Into<String>) -> Result<()> {
        self.client
            .delete(ep!(self, "/invites/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
