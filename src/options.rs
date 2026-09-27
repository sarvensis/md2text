/// Configuration for [`crate::to_plain_text`].
///
/// Construct with [`Options::default`] and override the fields you care about:
///
/// ```
/// use md2text::Options;
///
/// let opts = Options { keep_link_urls: true, ..Options::default() };
/// ```
#[derive(Debug, Clone)]
pub struct Options {
    /// Append the destination URL after link text, e.g. `Google (https://…)`.
    ///
    /// When `false` (the default) only the visible link text is kept.
    pub keep_link_urls: bool,

    /// Keep the `alt` text of images. When `false`, images are dropped entirely.
    /// Defaults to `true`.
    pub keep_image_alt: bool,

    /// Marker used for unordered list items. Defaults to `"- "`.
    /// Set to `""` to drop bullets and keep only the item text.
    pub list_bullet: String,

    /// Repair obviously truncated input before parsing.
    ///
    /// The main repair is closing an unterminated fenced code block so its
    /// content is emitted as code rather than swallowed. Enabled by default,
    /// which is what you want when feeding partial streaming output.
    pub heal_truncated: bool,

    /// Enable GitHub-flavored extensions: tables, strikethrough and task lists.
    /// Defaults to `true`.
    pub gfm: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            keep_link_urls: false,
            keep_image_alt: true,
            list_bullet: "- ".to_string(),
            heal_truncated: true,
            gfm: true,
        }
    }
}
