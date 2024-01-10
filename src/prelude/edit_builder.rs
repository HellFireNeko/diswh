use crate::{EditMessagePacket, Embed};

#[derive(Clone)]
pub struct EditMessageBuilder {
    message: EditMessagePacket
}

impl EditMessageBuilder {
    
    pub fn new(
        content: impl Into<String>,
    ) -> Self {
        Self {
            message: EditMessagePacket {
                content: content.into(),
                embeds: Vec::new(),
            },
        }
    }

    pub fn add_embed(mut self, embed: Embed) -> Self {
        self.message.embeds.push(embed);
        self
    }

    pub fn build(self) -> EditMessagePacket {
        self.message
    }
}