use reqwest::header;

use super::{EditMessagePacket, MessagePacket};

#[derive(Clone)]
pub struct WebhookBuilder {
    url: String,
}

impl WebhookBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

#[cfg(feature = "async")]
impl WebhookBuilder {
    pub async fn send_message_async(self, packet: MessagePacket) -> Result<Self, reqwest::Error> {
        WebhookBuilder::send_packet_async(&self.url.clone(), false, &packet.serialize_packet()).await?;
        Ok(self)
    }

    pub async fn edit_message_async(
        self,
        packet: EditMessagePacket,
        id: usize,
    ) -> Result<Self, reqwest::Error> {
        WebhookBuilder::send_packet_async(
            &(self.url.clone() + &format!("/messages/{}", id)),
            true,
            &packet.serialize_packet(),
        ).await?;
        Ok(self)
    }

    async fn send_packet_async(url: &str, patch: bool, packet: &str) -> Result<(), reqwest::Error> {
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

impl WebhookBuilder {
    pub fn send_message(self, packet: MessagePacket) -> Result<Self, reqwest::Error> {
        WebhookBuilder::send_packet(&self.url.clone(), false, &packet.serialize_packet())?;
        Ok(self)
    }

    pub fn edit_message(
        self,
        packet: EditMessagePacket,
        id: usize,
    ) -> Result<Self, reqwest::Error> {
        WebhookBuilder::send_packet(
            &(self.url.clone() + &format!("/messages/{}", id)),
            true,
            &packet.serialize_packet(),
        )?;
        Ok(self)
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
