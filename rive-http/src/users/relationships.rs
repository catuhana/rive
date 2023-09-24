use crate::prelude::*;
use rive_models::{
    data::SendFriendRequestData,
    id::{marker::UserMarker, Id},
    user::{Mutuals, User},
};

impl Client {
    /// This fetches your direct messages, including any DM and group DM conversations.
    pub async fn fetch_mutual_friends_and_servers(&self, id: &Id<UserMarker>) -> Result<Mutuals> {
        Ok(self
            .client
            .get(ep!(self, "/users/{}/mutual", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Accept another user's friend request
    pub async fn accept_friend_request(&self, id: &Id<UserMarker>) -> Result<User> {
        Ok(self
            .client
            .put(ep!(self, "/users/{}/friend", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Denies another user's friend request or removes an existing friend.
    pub async fn remove_or_deny_friend(&self, id: &Id<UserMarker>) -> Result<User> {
        Ok(self
            .client
            .delete(ep!(self, "/users/{}/friend", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Block another user by their id.
    pub async fn block_user(&self, id: &Id<UserMarker>) -> Result<User> {
        Ok(self
            .client
            .put(ep!(self, "/users/{}/block", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Unblock another user by their id.
    pub async fn unblock_user(&self, id: &Id<UserMarker>) -> Result<User> {
        Ok(self
            .client
            .delete(ep!(self, "/users/{}/block", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Send a friend request to another user.
    pub async fn send_friend_request(&self, data: SendFriendRequestData) -> Result<User> {
        Ok(self
            .client
            .post(ep!(self, "/users/friend"))
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
