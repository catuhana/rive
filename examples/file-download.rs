//! This example downloads message attachment to a disk.

use rive_models::{attachment::Attachment, authentication::Authentication};
use std::env::var;
use tokio::{fs::File, io};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = var("TOKEN")?;
    let channel_id = var("CHANNEL_ID")?;
    let message_id = var("MESSAGE_ID")?;

    let autumn = rive_autumn::Client::new();
    let http = rive_http::Client::new(Authentication::BotToken(token));

    let message = http.fetch_message(channel_id, message_id).await?;

    for attachment in message.attachments.unwrap() {
        let Attachment {
            id, tag, filename, ..
        } = &attachment;

        let mut contents = autumn.download(tag, id).await?;

        let mut file = File::create(&filename).await?;
        io::copy(&mut contents, &mut file).await?;
    }

    Ok(())
}
