use crate::prelude::*;
use rive_models::{
    payload::{EditReportPayload, ReportContentPayload},
    report::Report,
    snapshot::Snapshot,
};

impl Client {
    /// Edit a report.
    pub async fn edit_report(
        &self,
        report: impl Into<String>,
        payload: EditReportPayload,
    ) -> Result<Report> {
        Ok(self
            .client
            .patch(ep!(self, "/safety/reports/{}", report.into()))
            .auth(&self.authentication)
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch a report by its ID.
    pub async fn fetch_report(&self, id: impl Into<String>) -> Result<Report> {
        Ok(self
            .client
            .get(ep!(self, "/safety/report/{}", id.into()))
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
    pub async fn report_content(&self, payload: ReportContentPayload) -> Result<()> {
        self.client
            .post(ep!(self, "/safety/report"))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Fetch a snapshot for a given report.
    pub async fn fetch_snapshot(&self, report_id: impl Into<String>) -> Result<Snapshot> {
        Ok(self
            .client
            .get(ep!(self, "/safety/snapshot/{}", report_id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
