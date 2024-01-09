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

impl WebhookBuilder {
    pub fn send_message(&self, packet: MessagePacket) -> Self {
        WebhookBuilder::send_packet(&self.url.clone(), false, &packet.serialize_packet());
        self.clone()
    }

    pub fn edit_message(&self, packet: EditMessagePacket, id: usize) -> Self {
        WebhookBuilder::send_packet(&(self.url.clone() + &format!("/messages/{}", id)), true, &packet.serialize_packet());
        self.clone()
    }

    fn send_packet(url: &str, patch: bool, packet: &str) {
        let mut easy = Easy::new();
        easy.url(url).unwrap();

        let mut headers = curl::easy::List::new();
        headers.append("Content-Type: application/json").unwrap();
        easy.http_headers(headers).unwrap();

        if patch {
            easy.custom_request("PATCH").unwrap();
        }

        easy.post_fields_copy(packet.as_bytes()).unwrap();

        easy.perform().unwrap();
    }
}