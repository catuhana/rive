use crate::prelude::*;
use rive_models::{
    payload::{DeleteAllSessionsPayload, EditSessionPayload, LoginPayload},
    session::{LoginResponse, SessionInfo},
};

impl Client {
    /// Login to an account.
    pub async fn login(&self, payload: LoginPayload) -> Result<LoginResponse> {
        Ok(self
            .client
            .post(ep!(self, "/auth/session/login"))
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Delete current session.
    pub async fn logout(&self) -> Result<()> {
        self.client
            .post(ep!(self, "/auth/session/logout"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Fetch all sessions associated with this account.
    pub async fn fetch_sessions(&self) -> Result<Vec<SessionInfo>> {
        Ok(self
            .client
            .get(ep!(self, "/auth/session/all"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Delete all active sessions, optionally including current one.
    pub async fn delete_all_sessions(&self, payload: DeleteAllSessionsPayload) -> Result<()> {
        self.client
            .delete(ep!(self, "/auth/session/all"))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Delete a specific active session.
    pub async fn revoke_session(&self, id: impl Into<String>) -> Result<()> {
        self.client
            .delete(ep!(self, "/auth/session/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Edit specific session information.
    pub async fn edit_session(
        &self,
        id: impl Into<String>,
        payload: EditSessionPayload,
    ) -> Result<SessionInfo> {
        Ok(self
            .client
            .patch(ep!(self, "/auth/session/{}", id.into()))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
