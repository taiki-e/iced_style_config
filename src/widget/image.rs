use crate::import::*;

widget_style_sheet_map!(Image);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    width: Option<Length>,
    height: Option<Length>,
}

impl StyleSheet {
    /// Creates a new [`iced::Image`] with the given [`iced::image::Handle`].
    pub fn new(&self, handle: impl Into<iced::image::Handle>) -> iced::Image {
        let mut this = iced::Image::new(handle);
        if let Some(width) = self.width {
            this = this.width(width.into());
        }
        if let Some(height) = self.height {
            this = this.height(height.into());
        }
        this
    }
}
