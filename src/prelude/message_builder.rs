use crate::{MessagePacket, Embed};

/// A builder for constructing [MessagePacket] instances with optional fields such as username,
/// avatar URL, and embeds.
///
/// The [MessageBuilder] provides a fluent interface for setting various properties of a message
/// before constructing the final [MessagePacket]. This allows for flexible and readable message
/// creation.
///
/// # Examples
///
/// ```no_run
/// use crate::{MessageBuilder, Embed};
///
/// let embed = Embed::new("Title", "Description");
/// let message = MessageBuilder::new("Hello, world!", false)
///     .with_username("BotUser")
///     .with_avatar_url("https://example.com/avatar.png")
///     .add_embed(embed)
///     .build();
/// ```
#[derive(Clone)]
pub struct MessageBuilder {
    message: MessagePacket,
}

impl MessageBuilder {
    /// Creates a new `MessageBuilder` with the specified content and TTS (text-to-speech) flag.
    ///
    /// # Arguments
    ///
    /// * `content` - The message content.
    /// * `tts` - Whether the message should be sent as text-to-speech.
    ///
    /// # Returns
    ///
    /// A new instance of `MessageBuilder`.
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

    /// Sets the username for the message.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to display as the sender.
    ///
    /// # Returns
    ///
    /// The updated `MessageBuilder` with the specified username.
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.message.username = username.into();
        self
    }

    /// Sets the avatar URL for the message.
    ///
    /// # Arguments
    ///
    /// * `avatar_url` - The URL of the avatar to display as the sender.
    ///
    /// # Returns
    ///
    /// The updated `MessageBuilder` with the specified avatar URL.
    pub fn with_avatar_url(mut self, avatar_url: impl Into<String>) -> Self {
        self.message.avatar_url = avatar_url.into();
        self
    }

    /// Adds an embed to the message.
    ///
    /// # Arguments
    ///
    /// * `embed` - The `Embed` object to add to the message.
    ///
    /// # Returns
    ///
    /// The updated `MessageBuilder` with the new embed added.
    pub fn add_embed(mut self, embed: Embed) -> Self {
        self.message.embeds.push(embed);
        self
    }

    /// Consumes the builder and returns the constructed `MessagePacket`.
    ///
    /// # Returns
    ///
    /// The built `MessagePacket` containing all specified fields.
    pub fn build(self) -> MessagePacket {
        self.message
    }
}
