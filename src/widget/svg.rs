use std::path::PathBuf;

use crate::import::*;

widget_style_sheet_map!(Svg);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    width: Option<Length>,
    height: Option<Length>,
}

impl StyleSheet {
    /// Creates a new [`iced::Svg`] from the given [`iced::svg::Handle`].
    pub fn new(&self, handle: impl Into<iced::svg::Handle>) -> iced::Svg {
        let mut this = iced::Svg::new(handle);
        if let Some(width) = self.width {
            this = this.width(width.into());
        }
        if let Some(height) = self.height {
            this = this.height(height.into());
        }
        this
    }

    /// Creates a new [`iced::Svg`] that will display the contents of the file at the
    /// provided path.
    pub fn from_path(&self, path: impl Into<PathBuf>) -> iced::Svg {
        self.new(iced::svg::Handle::from_path(path))
    }
}
