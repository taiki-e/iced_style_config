use crate::import::*;

widget_style_sheet_map!(Text);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    size: Option<u16>,
    color: Option<Color>,
    width: Option<Length>,
    height: Option<Length>,
    horizontal_alignment: Option<alignment::Horizontal>,
    vertical_alignment: Option<alignment::Vertical>,
}

impl StyleSheet {
    /// Create a new fragment of [`iced::Text`] with the given contents.
    pub fn new(&self, label: impl Into<String>) -> iced::Text {
        let mut this = iced::Text::new(label);
        if let Some(size) = self.size {
            this = this.size(size);
        }
        if let Some(color) = self.color {
            this = this.color(color);
        }
        if let Some(width) = self.width {
            this = this.width(width.into());
        }
        if let Some(height) = self.height {
            this = this.height(height.into());
        }
        if let Some(horizontal_alignment) = self.horizontal_alignment {
            this = this.horizontal_alignment(horizontal_alignment.into());
        }
        if let Some(vertical_alignment) = self.vertical_alignment {
            this = this.vertical_alignment(vertical_alignment.into());
        }
        this
    }
}
