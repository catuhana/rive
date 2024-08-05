use std::future::IntoFuture;

use rive_models::{
    data::SendMessageData,
    embed::SendableEmbedBorrowed,
    id::{
        marker::{AttachmentMarker, ChannelMarker},
        Id,
    },
    message::{InteractionsBorrowed, MasqueradeBorrowed, Message, ReplyBorrowed},
};

use crate::{
    base::request::{Request, Route, TryIntoRequest},
    Client, ResponseFuture, Result,
};

#[must_use = "requests must be configured and executed"]
pub struct SendMessageRequest<'a> {
    client: &'a Client,
    channel_id: &'a Id<ChannelMarker>,
    data: SendMessageData<'a>,
}

impl<'a> SendMessageRequest<'a> {
    pub(crate) const fn new(client: &'a Client, channel_id: &'a Id<ChannelMarker>) -> Self {
        Self {
            client,
            channel_id,
            data: SendMessageData {
                content: None,
                attachments: None,
                replies: None,
                embeds: None,
                masquerade: None,
                interactions: None,
            },
        }
    }

    pub const fn content(mut self, content: &'a str) -> Self {
        self.data.content = Some(content);
        self
    }

    pub const fn attachments(mut self, attachments: &'a [Id<AttachmentMarker>]) -> Self {
        self.data.attachments = Some(attachments);
        self
    }

    pub const fn replies(mut self, replies: &'a [ReplyBorrowed<'a>]) -> Self {
        self.data.replies = Some(replies);
        self
    }

    pub const fn embeds(mut self, embeds: &'a [SendableEmbedBorrowed<'a>]) -> Self {
        self.data.embeds = Some(embeds);
        self
    }

    pub const fn masquerade(mut self, masquerade: &'a MasqueradeBorrowed) -> Self {
        self.data.masquerade = Some(masquerade);
        self
    }

    pub const fn interactions(mut self, interactions: &'a InteractionsBorrowed) -> Self {
        self.data.interactions = Some(interactions);
        self
    }
}

impl TryIntoRequest for SendMessageRequest<'_> {
    fn try_into_request(self) -> Result<Request> {
        Request::builder()
            .route(Route::SendMessage {
                channel_id: self.channel_id.value_ref(),
            })
            .json(&self.data)
            .build()
    }
}

impl<'a> IntoFuture for SendMessageRequest<'a> {
    type Output = Result<Message>;
    type IntoFuture = ResponseFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.client.fire(self))
    }
}
