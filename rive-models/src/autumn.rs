use serde::Deserialize;

/// Uploaded attachment data
#[derive(Deserialize, Debug, Clone)]
pub struct UploadData {
    /// Attachment ID
    pub id: String,
}
