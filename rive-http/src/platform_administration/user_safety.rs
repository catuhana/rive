use crate::prelude::*;
use rive_models::{
    data::{CreateStrikeData, EditAccountStrikeData, EditReportData, ReportContentData},
    id::{
        marker::{ReportMarker, StrikeMarker, UserMarker},
        Id,
    },
    report::Report,
    snapshot::Snapshot,
    strike::AccountStrike,
};

impl Client {
    /// Edit a report.
    pub async fn edit_report(
        &self,
        report: &Id<ReportMarker>,
        data: EditReportData,
    ) -> Result<Report> {
        Ok(self
            .client
            .patch(ep!(self, "/safety/reports/{}", report.value_ref()))
            .auth(&self.authentication)
            .json(&data)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch a report by its ID.
    pub async fn fetch_report(&self, id: &Id<ReportMarker>) -> Result<Report> {
        Ok(self
            .client
            .get(ep!(self, "/safety/report/{}", id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch all available reports
    pub async fn fetch_reports(&self) -> Result<Vec<Report>> {
        Ok(self
            .client
            .get(ep!(self, "/safety/reports"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Report a piece of content to the moderation team.
    pub async fn report_content(&self, data: ReportContentData) -> Result<()> {
        self.client
            .post(ep!(self, "/safety/report"))
            .json(&data)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Fetch a snapshot for a given report.
    pub async fn fetch_snapshot(&self, report_id: &Id<ReportMarker>) -> Result<Snapshot> {
        Ok(self
            .client
            .get(ep!(self, "/safety/snapshot/{}", report_id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Create a new account strike.
    pub async fn create_strike(&self, data: CreateStrikeData) -> Result<AccountStrike> {
        Ok(self
            .client
            .post(ep!(self, "/safety/strikes"))
            .auth(&self.authentication)
            .json(&data)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Fetch strikes for a user by their ID.
    pub async fn fetch_strikes(&self, user_id: &Id<UserMarker>) -> Result<AccountStrike> {
        Ok(self
            .client
            .get(ep!(self, "/safety/strikes/{}", user_id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Edit a strike by its ID.
    pub async fn edit_strike(
        &self,
        strike_id: &Id<StrikeMarker>,
        data: EditAccountStrikeData,
    ) -> Result<()> {
        self.client
            .patch(ep!(self, "/safety/strikes/{}", strike_id.value_ref()))
            .auth(&self.authentication)
            .json(&data)
            .send()
            .await?
            .json()
            .await?;
        Ok(())
    }

    /// Edit a strike by its ID.
    pub async fn delete_strike(&self, strike_id: &Id<StrikeMarker>) -> Result<()> {
        self.client
            .delete(ep!(self, "/safety/strikes/{}", strike_id.value_ref()))
            .auth(&self.authentication)
            .send()
            .await?
            .json()
            .await?;
        Ok(())
    }
}
