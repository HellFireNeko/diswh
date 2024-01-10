use crate::{MessagePacket, Embed};

#[derive(Clone)]
pub struct MessageBuilder {
    message: MessagePacket,
}

impl MessageBuilder {
    pub fn new(
        content: impl Into<String>,
        tts: bool,
    ) -> Self {
        Self {
            message: MessagePacket {
                content: content.into(),
                username: "".into(),
                avatar_url: "".into(),
                tts,
                embeds: Vec::new(),
            },
        }
    }

    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.message.username = username.into();
        self
    }

    pub fn with_avatar_url(mut self, avatar_url: impl Into<String>) -> Self {
        self.message.avatar_url = avatar_url.into();
        self
    }

    pub fn add_embed(mut self, embed: Embed) -> Self {
        self.message.embeds.push(embed);
        self
    }

    pub fn build(self) -> MessagePacket {
        self.message
    }
}
