use crate::prelude::*;
use rive_models::{
    account::{AccountInfo, EmailVerification},
    payload::{
        ChangeEmailPayload, ChangePasswordPayload, ConfirmAccountDeletionPayload,
        CreateAccountPayload, PasswordResetPayload, ResendVerificationPayload,
        SendPasswordResetPayload,
    },
};

impl Client {
    /// Create a new account.
    pub async fn create_account(&self, payload: CreateAccountPayload) -> Result<()> {
        self.client
            .post(ep!(self, "/auth/account/create"))
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Resend account creation verification email.
    pub async fn resend_verification(&self, payload: ResendVerificationPayload) -> Result<()> {
        self.client
            .post(ep!(self, "/auth/account/reverify"))
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Schedule an account for deletion by confirming the received token.
    pub async fn confirm_account_deletion(
        &self,
        payload: ConfirmAccountDeletionPayload,
    ) -> Result<()> {
        self.client
            .put(ep!(self, "/auth/account/delete"))
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Request to have an account deleted.
    pub async fn delete_account(&self) -> Result<()> {
        self.client
            .post(ep!(self, "/auth/account/delete"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Fetch account information from the current session.
    pub async fn fetch_account(&self) -> Result<AccountInfo> {
        Ok(self
            .client
            .get(ep!(self, "/auth/account/"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Disable an account.
    pub async fn disable_account(&self) -> Result<()> {
        self.client
            .post(ep!(self, "/auth/account/disable"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Change the current account password.
    pub async fn change_password(&self, payload: ChangePasswordPayload) -> Result<()> {
        self.client
            .patch(ep!(self, "/auth/account/change/password"))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Change the associated account email.
    pub async fn change_email(&self, payload: ChangeEmailPayload) -> Result<()> {
        self.client
            .patch(ep!(self, "/auth/account/change/email"))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Change the associated account email.
    pub async fn verify_email(&self, code: impl Into<String>) -> Result<EmailVerification> {
        Ok(self
            .client
            .post(ep!(self, "/auth/account/verify/{}", code.into()))
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Send an email to reset account password.
    pub async fn send_password_reset(&self, payload: SendPasswordResetPayload) -> Result<()> {
        self.client
            .post(ep!(self, "/auth/account/reset_password"))
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Confirm password reset and change the password.
    pub async fn password_reset(&self, payload: PasswordResetPayload) -> Result<()> {
        self.client
            .patch(ep!(self, "/auth/account/reset_password"))
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
