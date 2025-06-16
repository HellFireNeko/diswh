use crate::{EditMessagePacket, Embed};

#[derive(Clone)]
pub struct EditMessageBuilder {
    message: EditMessagePacket
}

impl EditMessageBuilder {
    /// Creates a new `EditMessageBuilder` with the specified content.
    ///
    /// # Arguments
    ///
    /// * `content` - The new content for the message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let builder = EditMessageBuilder::new("Updated content");
    /// ```
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

    /// Adds an embed to the message.
    ///
    /// # Arguments
    ///
    /// * `embed` - The embed to add to the message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let builder = EditMessageBuilder::new("content").add_embed(embed);
    /// ```
    pub fn add_embed(mut self, embed: Embed) -> Self {
        self.message.embeds.push(embed);
        self
    }

    /// Consumes the builder and returns the constructed `EditMessagePacket`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let packet = EditMessageBuilder::new("content").build();
    /// ```
    pub fn build(self) -> EditMessagePacket {
        self.message
    }
}