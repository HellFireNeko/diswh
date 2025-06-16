use reqwest::header;

use super::{EditMessagePacket, MessagePacket};

/// A builder for sending and editing messages via a webhook.
///
/// The [Webhook] struct provides methods to send and edit messages
/// using a specified webhook URL. It supports both asynchronous and synchronous
/// operations, depending on the enabled feature flag (`async`).
///
/// # Examples
///
/// Creating a new [Webhook]:
///
/// ```no_run
/// let webhook = Webhook::new("your_webhook_here");
/// ```
///
/// Sending a message (asynchronously, with the `async` feature enabled):
///
/// ```no_run
/// webhook.send_message(message_packet).await?;
/// ```
///
/// Editing a message (asynchronously, with the `async` feature enabled):
///
/// ```no_run
/// webhook.edit_message(edit_packet, message_id).await?;
/// ```
///
/// Sending a message (synchronously, with the `async` feature disabled):
///
/// ```no_run
/// webhook.send_message(message_packet)?;
/// ```
///
/// Editing a message (synchronously, with the `async` feature disabled):
///
/// ```no_run
/// webhook.edit_message(edit_packet, message_id)?;
/// ```
#[derive(Clone)]
pub struct Webhook {
    url: String,
}

impl Webhook {
    /// Creates a new [Webhook] with the given webhook URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The webhook URL as a string or any type that can be converted into a string.
    ///
    /// # Examples
    ///
    /// ```
    /// let webhook = Webhook::new("your_webhook_here");
    /// ```
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

#[cfg(feature = "async")]
impl Webhook {
    /// Sends a message using the webhook asynchronously.
    ///
    /// # Arguments
    ///
    /// * `packet` - The [MessagePacket] to send.
    ///
    /// # Errors
    ///
    /// Returns a [reqwest::Error] if the HTTP request fails.
    pub async fn send_message(&self, packet: MessagePacket) -> Result<(), reqwest::Error> {
        Webhook::send_packet(&self.url.clone(), false, &packet.serialize_packet()).await?;
        Ok(())
    }

    /// Edits an existing message using the webhook asynchronously.
    ///
    /// # Arguments
    ///
    /// * `packet` - The [EditMessagePacket] to send.
    /// * `id` - The ID of the message to edit.
    ///
    /// # Errors
    ///
    /// Returns a [reqwest::Error] if the HTTP request fails.
    pub async fn edit_message(
        &self,
        packet: EditMessagePacket,
        id: usize,
    ) -> Result<(), reqwest::Error> {
        Webhook::send_packet(
            &(self.url.clone() + &format!("/messages/{}", id)),
            true,
            &packet.serialize_packet(),
        ).await?;
        Ok(())
    }

    async fn send_packet(url: &str, patch: bool, packet: &str) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();

        let packet = packet.to_string();

        if patch {
            client
                .patch(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(packet)
                .send().await?;
        } else {
            client
                .post(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(packet)
                .send().await?;
        }

        Ok(())
    }
}

#[cfg(not(feature = "async"))]
impl Webhook {
    /// Sends a message using the webhook synchronously.
    ///
    /// # Arguments
    ///
    /// * `packet` - The [MessagePacket] to send.
    ///
    /// # Errors
    ///
    /// Returns a [reqwest::Error] if the HTTP request fails.
    pub fn send_message(&self, packet: MessagePacket) -> Result<(), reqwest::Error> {
        Webhook::send_packet(&self.url.clone(), false, &packet.serialize_packet())?;
        Ok(())
    }

    /// Edits an existing message using the webhook synchronously.
    ///
    /// # Arguments
    ///
    /// * `packet` - The [EditMessagePacket] to send.
    /// * `id` - The ID of the message to edit.
    ///
    /// # Errors
    ///
    /// Returns a [reqwest::Error] if the HTTP request fails.
    pub fn edit_message(
        &self,
        packet: EditMessagePacket,
        id: usize,
    ) -> Result<(), reqwest::Error> {
        Webhook::send_packet(
            &(self.url.clone() + &format!("/messages/{}", id)),
            true,
            &packet.serialize_packet(),
        )?;
        Ok(())
    }

    fn send_packet(url: &str, patch: bool, packet: &str) -> Result<(), reqwest::Error> {
        let client = reqwest::blocking::Client::new();

        let packet = packet.to_string();

        if patch {
            client
                .patch(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(packet)
                .send()?;
        } else {
            client
                .post(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(packet)
                .send()?;
        }

        Ok(())
    }
}
