use crate::prelude::*;
use rive_models::{
    id::{marker::ChannelMarker, Id},
    voice::VoiceAuthenticationData,
};

impl Client {
    /// Asks the voice server for a token to join the call
    pub async fn join_call(&self, id: &Id<ChannelMarker>) -> Result<VoiceAuthenticationData> {
        Ok(self
            .client
            .post(ep!(self, "/channels/{}/join_call", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
