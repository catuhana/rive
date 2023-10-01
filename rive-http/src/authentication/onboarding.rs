use rive_models::{data::CompleteOnboardingData, onboarding::OnboardingStatus};

use crate::prelude::*;

impl Client {
    /// This will tell you whether the current account requires onboarding or whether you
    /// can continue to send requests as usual. You may skip calling this if you're restoring
    /// an existing session.
    pub async fn check_onboarding_status(&self) -> Result<OnboardingStatus> {
        Ok(self
            .client
            .get(ep!(self, "/onboarding/hello"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// This sets a new username, completes onboarding and allows a user to start using Revolt.
    pub async fn complete_onboarding(&self, data: &CompleteOnboardingData) -> Result<()> {
        self.client
            .post(ep!(self, "/onboarding/complete"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
