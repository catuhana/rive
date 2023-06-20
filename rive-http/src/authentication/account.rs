use crate::prelude::*;
use rive_models::{
    account::{AccountInfo, EmailVerification},
    data::{
        ChangeEmailData, ChangePasswordData, ConfirmAccountDeletionData, CreateAccountData,
        PasswordResetData, ResendVerificationData, SendPasswordResetData,
    },
};

impl Client {
    /// Create a new account.
    pub async fn create_account(&self, data: CreateAccountData) -> Result<()> {
        self.client
            .post(ep!(self, "/auth/account/create"))
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Resend account creation verification email.
    pub async fn resend_verification(&self, data: ResendVerificationData) -> Result<()> {
        self.client
            .post(ep!(self, "/auth/account/reverify"))
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Schedule an account for deletion by confirming the received token.
    pub async fn confirm_account_deletion(&self, data: ConfirmAccountDeletionData) -> Result<()> {
        self.client
            .put(ep!(self, "/auth/account/delete"))
            .json(&data)
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
    pub async fn change_password(&self, data: ChangePasswordData) -> Result<()> {
        self.client
            .patch(ep!(self, "/auth/account/change/password"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Change the associated account email.
    pub async fn change_email(&self, data: ChangeEmailData) -> Result<()> {
        self.client
            .patch(ep!(self, "/auth/account/change/email"))
            .json(&data)
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
    pub async fn send_password_reset(&self, data: SendPasswordResetData) -> Result<()> {
        self.client
            .post(ep!(self, "/auth/account/reset_password"))
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Confirm password reset and change the password.
    pub async fn password_reset(&self, data: PasswordResetData) -> Result<()> {
        self.client
            .patch(ep!(self, "/auth/account/reset_password"))
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
