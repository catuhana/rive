use crate::prelude::*;
use rive_models::{
    payload::{
        CreateRolePayload, EditRolePayload, SetDefaultRolePermissionPayload,
        SetServerRolePermissionPayload,
    },
    server::{NewRole, Role, Server},
};

impl Client {
    /// Creates a new server role.
    pub async fn create_role(
        &self,
        server_id: impl Into<String>,
        payload: CreateRolePayload,
    ) -> Result<NewRole> {
        Ok(self
            .client
            .post(ep!(self, "/servers/{}/roles", server_id.into()))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Delete a server role by its ID.
    pub async fn delete_role(
        &self,
        server_id: impl Into<String>,
        role_id: impl Into<String>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/servers/{}/roles/{}",
                server_id.into(),
                role_id.into()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Edit a role by its ID.
    pub async fn edit_role(
        &self,
        server_id: impl Into<String>,
        role_id: impl Into<String>,
        payload: EditRolePayload,
    ) -> Result<Role> {
        Ok(self
            .client
            .patch(ep!(
                self,
                "/servers/{}/roles/{}",
                server_id.into(),
                role_id.into()
            ))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Sets permissions for the specified role in the server.
    pub async fn set_role_permission(
        &self,
        server_id: impl Into<String>,
        role_id: impl Into<String>,
        payload: SetServerRolePermissionPayload,
    ) -> Result<Server> {
        Ok(self
            .client
            .put(ep!(
                self,
                "/servers/{}/permissions/{}",
                server_id.into(),
                role_id.into()
            ))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Sets permissions for the default role in the server.
    pub async fn set_default_permission(
        &self,
        server_id: impl Into<String>,
        payload: SetDefaultRolePermissionPayload,
    ) -> Result<Server> {
        Ok(self
            .client
            .put(ep!(
                self,
                "/servers/{}/permissions/default",
                server_id.into(),
            ))
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
