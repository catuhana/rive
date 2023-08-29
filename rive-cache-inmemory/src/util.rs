use rive_models::channel::Channel;

// TODO: move it to rive_models
pub(crate) fn channel_id(channel: &Channel) -> &String {
    match channel {
        Channel::SavedMessages { id, .. } => id,
        Channel::DirectMessage { id, .. } => id,
        Channel::Group { id, .. } => id,
        Channel::TextChannel { id, .. } => id,
        Channel::VoiceChannel { id, .. } => id,
    }
}
