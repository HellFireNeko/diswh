use serde::Serialize;

use crate::Embed;

#[derive(Clone, Serialize)]
pub struct EditMessagePacket {
    pub content: String,
    pub embeds: Vec<Embed>,
}
