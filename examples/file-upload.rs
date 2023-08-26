//! This example uploads a file on disk to the server and
//! sends a message which contains the attachment.

use rive_models::{authentication::Authentication, data::SendMessageData};
use std::{env::var, path::Path};
use tokio::fs::File;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path_var = var("FILE_PATH")?;
    let token = var("TOKEN")?;
    let channel_id = var("CHANNEL_ID")?;

    let autumn = rive_autumn::Client::new();
    let http = rive_http::Client::new(Authentication::BotToken(token));

    let file_path = Path::new(&file_path_var);
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file = File::open(&file_path).await?;

    let upload_data = autumn.upload("attachments", file_name, file).await?;

    let data = SendMessageData {
        attachments: Some(vec![upload_data.id]),
        ..Default::default()
    };
    http.send_message(channel_id, data).await?;

    Ok(())
}
