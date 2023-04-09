use serde::Deserialize;

/// Onboarding status
#[derive(Deserialize, Debug, Clone)]
pub struct OnboardingStatus {
    /// Whether onboarding is required
    pub onboarding: bool,
}
