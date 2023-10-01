use crate::prelude::*;
use rive_models::{
    bot::{Bot, OwnedBot, OwnedBots, PublicBot},
    data::{CreateBotData, EditBotData, InviteBotData},
    id::{marker::UserMarker, Id},
};

impl Client {
    /// Create a new Revolt bot.
    pub async fn create_bot(&self, data: &CreateBotData) -> Result<Bot> {
        Ok(self
            .client
            .post(ep!(self, "/bots/create"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch details of a public (or owned) bot by its id.
    pub async fn fetch_public_bot(&self, id: Id<UserMarker>) -> Result<PublicBot> {
        Ok(self
            .client
            .get(ep!(self, "/bots/{}/invite", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Invite a bot to a server or group by its id.
    pub async fn invite_bot(&self, bot_id: Id<UserMarker>, data: &InviteBotData) -> Result<()> {
        self.client
            .post(ep!(self, "/bots/{}/invite", bot_id.value_ref()))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Fetch details of a bot you own by its id.
    pub async fn fetch_bot(&self, id: &Id<UserMarker>) -> Result<OwnedBot> {
        Ok(self
            .client
            .get(ep!(self, "/bots/{}/invite", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Delete a bot by its id.
    pub async fn delete_bot(&self, id: &Id<UserMarker>) -> Result<()> {
        self.client
            .delete(ep!(self, "/bots/{}", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Edit bot details by its id.
    pub async fn edit_bot(&self, id: &Id<UserMarker>, data: &EditBotData) -> Result<Bot> {
        Ok(self
            .client
            .patch(ep!(self, "/bots/{}", id.value_ref()))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch all of the bots that you have control over.
    pub async fn fetch_owned_bots(&self) -> Result<OwnedBots> {
        Ok(self
            .client
            .get(ep!(self, "/bots/@me"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
