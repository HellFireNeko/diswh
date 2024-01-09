use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct EmbedFooter {
    pub text: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct EmbedMultimedia {
    pub url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

#[derive(Clone, Serialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct EmbedAuthor {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Clone, Serialize)]
pub struct Embed {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub embed_type: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: i32,
    pub footer: EmbedFooter,
    pub image: EmbedMultimedia,
    pub thumbnail: EmbedMultimedia,
    pub video: EmbedMultimedia,
    pub provider: EmbedProvider,
    pub author: EmbedAuthor,
    pub fields: Vec<EmbedField>,
}