use rive_models::{
    data::{CreateMFATicketData, EnableTOTP2FAData},
    mfa::{MFAMethod, MFARecoveryCode, MFAStatus, MFATicket, TOTPSecret},
};

use crate::prelude::*;

impl Client {
    /// Create a new MFA ticket or validate an existing one.
    pub async fn create_mfa_ticket(&self, data: CreateMFATicketData) -> Result<MFATicket> {
        Ok(self
            .client
            .put(ep!(self, "/auth/mfa/ticket"))
            .auth(&self.authentication)
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch MFA status of an account.
    pub async fn fetch_mfa_status(&self) -> Result<MFAStatus> {
        Ok(self
            .client
            .get(ep!(self, "/auth/mfa/"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch recovery codes for an account.
    pub async fn fetch_recovery_codes(&self) -> Result<Vec<MFARecoveryCode>> {
        Ok(self
            .client
            .post(ep!(self, "/auth/mfa/recovery"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Re-generate recovery codes for an account.
    pub async fn generate_recovery_codes(&self) -> Result<Vec<MFARecoveryCode>> {
        Ok(self
            .client
            .patch(ep!(self, "/auth/mfa/recovery"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch available MFA methods.
    pub async fn get_mfa_methods(&self) -> Result<Vec<MFAMethod>> {
        Ok(self
            .client
            .get(ep!(self, "/auth/mfa/methods"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Enable TOTP 2FA for an account.
    pub async fn enable_totp_2fa(&self, data: EnableTOTP2FAData) -> Result<()> {
        self.client
            .put(ep!(self, "/auth/mfa/totp"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Generate a new secret for TOTP.
    pub async fn generate_totp_secret(&self) -> Result<TOTPSecret> {
        Ok(self
            .client
            .post(ep!(self, "/auth/mfa/totp"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Disable TOTP 2FA for an account.
    pub async fn disable_totp_2fa(&self) -> Result<()> {
        self.client
            .delete(ep!(self, "/auth/mfa/totp"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
