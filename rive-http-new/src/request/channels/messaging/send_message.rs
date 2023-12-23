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

    pub fn content(mut self, content: &'a str) -> SendMessageRequest<'a> {
        self.data.content = Some(content);
        self
    }

    pub fn attachments(
        mut self,
        attachments: &'a [Id<AttachmentMarker>],
    ) -> SendMessageRequest<'a> {
        self.data.attachments = Some(attachments);
        self
    }

    pub fn replies(mut self, replies: &'a [ReplyBorrowed<'a>]) -> SendMessageRequest<'a> {
        self.data.replies = Some(replies);
        self
    }

    pub fn embeds(mut self, embeds: &'a [SendableEmbedBorrowed<'a>]) -> SendMessageRequest<'a> {
        self.data.embeds = Some(embeds);
        self
    }

    pub fn masquerade(mut self, masquerade: &'a MasqueradeBorrowed) -> SendMessageRequest<'a> {
        self.data.masquerade = Some(masquerade);
        self
    }

    pub fn interactions(
        mut self,
        interactions: &'a InteractionsBorrowed,
    ) -> SendMessageRequest<'a> {
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
