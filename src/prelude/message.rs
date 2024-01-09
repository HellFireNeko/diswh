use std::collections::VecDeque;

use serde::Serialize;
use serde_json::{json, Value};

use crate::Embed;

#[derive(Clone, Serialize)]
pub struct MessagePacket {
    pub content: String,
    pub username: String,
    pub avatar_url: String,
    pub tts: bool,
    pub embeds: Vec<Embed>,
}

impl MessagePacket {
    pub(crate) fn serialize_packet(&self) -> String {
        let mut j = json!({});
        if !self.content.is_empty() {
            j["content"] = json!(self.content);
        }
        if !self.username.is_empty() {
            j["username"] = json!(self.username);
        }
        if !self.avatar_url.is_empty() {
            j["avatar_url"] = json!(self.avatar_url);
        }
        j["tts"] = json!(self.tts);
        if !self.embeds.is_empty() {
            let mut embeds_json = VecDeque::new();
            for embed in &self.embeds {
                let e: Value = serde_json::to_value(embed).unwrap();
                embeds_json.push_back(e);
            }
            j["embeds"] = json!(embeds_json);
        }
        j.to_string()
    }
}