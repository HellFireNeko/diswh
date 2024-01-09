use serde::Serialize;

use crate::Embed;

#[derive(Clone, Serialize)]
pub struct MessagePacket {
    pub content: String,
    pub username: String,
    pub avatar_url: String,
    pub tts: bool,
    pub embeds: Vec<Embed>,
}