use crate::prelude::embed::*;

#[derive(Clone)]
pub struct EmbedBuilder {
    embed: Embed,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self {
            embed: Embed {
                title: None,
                embed_type: "rich".to_string(),
                description: None,
                url: None,
                color: 0,
                footer: EmbedFooter {
                    text: None,
                    icon_url: None,
                },
                image: EmbedMultimedia {
                    url: None,
                    height: None,
                    width: None,
                },
                thumbnail: EmbedMultimedia {
                    url: None,
                    height: None,
                    width: None,
                },
                video: EmbedMultimedia {
                    url: None,
                    height: None,
                    width: None,
                },
                provider: EmbedProvider {
                    name: None,
                    url: None,
                },
                author: EmbedAuthor {
                    name: None,
                    url: None,
                    icon_url: None,
                },
                fields: Vec::new(),
            },
        }
    }

    pub fn with_title(&mut self, title: impl Into<String>) -> Self {
        self.embed.title = Some(title.into());
        self.clone()
    }

    pub fn with_description(&mut self, description: impl Into<String>) -> Self {
        self.embed.description = Some(description.into());
        self.clone()
    }

    pub fn with_url(&mut self, url: impl Into<String>) -> Self {
        self.embed.url = Some(url.into());
        self.clone()
    }

    pub fn with_footer_text(&mut self, text: impl Into<String>) -> Self {
        self.embed.footer.text = Some(text.into());
        self.clone()
    }

    pub fn with_footer_url(&mut self, url: impl Into<String>) -> Self {
        self.embed.footer.icon_url = Some(url.into());
        self.clone()
    }

    pub fn with_image(&mut self, url: impl Into<String>) -> Self {
        self.embed.image.url = Some(url.into());
        self.clone()
    }

    pub fn with_image_dims(&mut self, width: i32, height: i32) -> Self {
        self.embed.image.width = Some(width);
        self.embed.image.height = Some(height);
        self.clone()
    }

    pub fn with_thumbnail(&mut self, url: impl Into<String>) -> Self {
        self.embed.thumbnail.url = Some(url.into());
        self.clone()
    }

    pub fn with_thumbnail_dims(&mut self, width: i32, height: i32) -> Self {
        self.embed.thumbnail.width = Some(width);
        self.embed.thumbnail.height = Some(height);
        self.clone()
    }

    pub fn with_video(&mut self, url: impl Into<String>) -> Self {
        self.embed.video.url = Some(url.into());
        self.clone()
    }

    pub fn with_video_dims(&mut self, width: i32, height: i32) -> Self {
        self.embed.video.width = Some(width);
        self.embed.video.height = Some(height);
        self.clone()
    }

    pub fn with_provider(&mut self, name: impl Into<String>, url: impl Into<String>) -> Self {
        self.embed.provider.name = Some(name.into());
        self.embed.provider.url = Some(url.into());
        self.clone()
    }

    pub fn with_author(
        &mut self,
        name: impl Into<String>,
        url: Option<&str>,
        icon_url: Option<&str>,
    ) -> Self {
        self.embed.author.name = Some(name.into());
        if let Some(url) = url {
            self.embed.author.url = Some(url.to_string());
        }
        if let Some(icon_url) = icon_url {
            self.embed.author.icon_url = Some(icon_url.to_string());
        }
        self.clone()
    }

    pub fn with_color(&mut self, color: i32) -> Self {
        self.embed.color = color;
        self.clone()
    }

    pub fn add_field(&mut self, name: &str, value: &str, inline: bool) -> Self {
        self.embed.fields.push(EmbedField {
            name: name.to_string(),
            value: value.to_string(),
            inline,
        });
        self.clone()
    }

    pub fn build(&self) -> Embed {
        self.embed.clone()
    }
}
