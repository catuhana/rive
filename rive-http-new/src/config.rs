use rive_models::authentication::Authentication;

#[derive(Debug, Clone)]
pub struct Config {
    pub authentication: Authentication,
    pub base_url: String,
}
