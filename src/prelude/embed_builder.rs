use crate::prelude::embed::*;

/// A builder for constructing [`Embed`] objects with a fluent interface.
///
/// The `EmbedBuilder` struct provides a convenient way to create and customize
/// embed objects by chaining method calls. Each method sets a property of the
/// embed and returns the builder, allowing for expressive and readable code.
///
/// # Examples
///
/// ```no_run
/// use crate::prelude::embed::EmbedBuilder;
///
/// let embed = EmbedBuilder::new()
///     .with_title("Example Title")
///     .with_description("This is an example embed.")
///     .with_color(0x00ff00)
///     .add_field("Field Name", "Field Value", true)
///     .build();
/// ```
///
/// See individual methods for details on each property that can be set.
#[derive(Clone)]
pub struct EmbedBuilder {
    embed: Embed,
}

impl EmbedBuilder {
    /// Creates a new, empty `EmbedBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = EmbedBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            embed: Embed::new(),
        }
    }

    /// Sets the title of the embed.
    ///
    /// # Arguments
    ///
    /// * `title` - The title text.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.embed.title = Some(title.into());
        self
    }

    /// Sets the description of the embed.
    ///
    /// # Arguments
    ///
    /// * `description` - The description text.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.embed.description = Some(description.into());
        self
    }

    /// Sets the URL of the embed.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to associate with the embed.
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.embed.url = Some(url.into());
        self
    }

    /// Sets the footer text of the embed.
    ///
    /// # Arguments
    ///
    /// * `text` - The footer text.
    pub fn with_footer_text(mut self, text: impl Into<String>) -> Self {
        self.embed.footer.text = Some(text.into());
        self
    }

    /// Sets the footer icon URL of the embed.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the footer icon.
    pub fn with_footer_url(mut self, url: impl Into<String>) -> Self {
        self.embed.footer.icon_url = Some(url.into());
        self
    }

    /// Sets the image URL of the embed.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the image.
    pub fn with_image(mut self, url: impl Into<String>) -> Self {
        self.embed.image.url = Some(url.into());
        self
    }

    /// Sets the dimensions of the embed image.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    pub fn with_image_dims(mut self, width: i32, height: i32) -> Self {
        self.embed.image.width = Some(width);
        self.embed.image.height = Some(height);
        self
    }

    /// Sets the thumbnail URL of the embed.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the thumbnail.
    pub fn with_thumbnail(mut self, url: impl Into<String>) -> Self {
        self.embed.thumbnail.url = Some(url.into());
        self
    }

    /// Sets the dimensions of the embed thumbnail.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the thumbnail.
    /// * `height` - The height of the thumbnail.
    pub fn with_thumbnail_dims(mut self, width: i32, height: i32) -> Self {
        self.embed.thumbnail.width = Some(width);
        self.embed.thumbnail.height = Some(height);
        self
    }

    /// Sets the video URL of the embed.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video.
    pub fn with_video(mut self, url: impl Into<String>) -> Self {
        self.embed.video.url = Some(url.into());
        self
    }

    /// Sets the dimensions of the embed video.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the video.
    /// * `height` - The height of the video.
    pub fn with_video_dims(mut self, width: i32, height: i32) -> Self {
        self.embed.video.width = Some(width);
        self.embed.video.height = Some(height);
        self
    }

    /// Sets the provider name and URL of the embed.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the provider.
    /// * `url` - The URL of the provider.
    pub fn with_provider(mut self, name: impl Into<String>, url: impl Into<String>) -> Self {
        self.embed.provider.name = Some(name.into());
        self.embed.provider.url = Some(url.into());
        self
    }

    /// Sets the author of the embed.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the author.
    /// * `url` - The URL of the author (optional).
    /// * `icon_url` - The icon URL of the author (optional).
    pub fn with_author(
        mut self,
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
        self
    }

    /// Sets the color of the embed.
    ///
    /// # Arguments
    ///
    /// * `color` - The color as an integer.
    pub fn with_color(mut self, color: i32) -> Self {
        self.embed.color = color;
        self
    }

    /// Adds a field to the embed.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
    /// * `inline` - Whether the field should display inline.
    pub fn add_field(mut self, name: &str, value: &str, inline: bool) -> Self {
        self.embed.fields.push(EmbedField {
            name: name.to_string(),
            value: value.to_string(),
            inline,
        });
        self
    }

    /// Consumes the builder and returns the constructed `Embed`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let embed = EmbedBuilder::new().with_title("Title").build();
    /// ```
    pub fn build(self) -> Embed {
        self.embed
    }
}
