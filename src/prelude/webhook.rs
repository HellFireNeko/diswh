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
    pub async fn send_message_async(&self, packet: MessagePacket) -> Self {
        if let Ok(packet) = serde_json::to_string(&packet) {
            WebhookBuilder::send_packet_async(&self.url.clone(), false, &packet).await;
        }
        self.clone()
    }

    pub async fn edit_message_async(&self, packet: EditMessagePacket, id: usize) -> Self {
        if let Ok(packet) = serde_json::to_string(&packet) {
            WebhookBuilder::send_packet_async(&(self.url.clone() + &format!("/messages/{}", id)), true, &packet).await;
        }
        self.clone()
    }

    async fn send_packet_async(url: &str, patch: bool, packet: &str) {
        use reqwest::Client;

        let client = Client::new();
        let response = if patch {
            client.patch(url).body(packet.to_owned()).send().await
        } else {
            client.post(url).body(packet.to_owned()).send().await
        };

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    println!("Sent message to discord!")
                } else {
                    eprintln!("Status was not success! {}", res.status());
                }
            }
            Err(err) => {
                eprintln!("Error sending packet: {:?}", err);
            }
        }
    }
}

impl WebhookBuilder {
    pub fn send_message(&self, packet: MessagePacket) -> Self {
        if let Ok(packet) = serde_json::to_string(&packet) {
            WebhookBuilder::send_packet(&self.url.clone(), false, &packet);
        }
        self.clone()
    }

    pub fn edit_message(&self, packet: EditMessagePacket, id: usize) -> Self {
        if let Ok(packet) = serde_json::to_string(&packet) {
            WebhookBuilder::send_packet(&(self.url.clone() + &format!("/messages/{}", id)), true, &packet);
        }
        self.clone()
    }

    fn send_packet(url: &str, patch: bool, packet: &str) {
        use reqwest::blocking::Client;

        let client = Client::new();
        let response = if patch {
            client.patch(url).body(packet.to_owned()).send()
        } else {
            client.post(url).body(packet.to_owned()).send()
        };

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    println!("Sent message to discord!")
                } else {
                    eprintln!("Status was not success! {}", res.status());
                }
            }
            Err(err) => {
                eprintln!("Error sending packet: {:?}", err);
            }
        }
    }
}