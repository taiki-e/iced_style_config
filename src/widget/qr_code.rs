use crate::import::*;

widget_style_sheet_map!(QRCode);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    dark: Option<Color>,
    light: Option<Color>,
    cell_size: Option<u16>,
}

impl StyleSheet {
    /// Creates a new [`iced::QRCode`] with the provided [`iced::qr_code::State`].
    pub fn new<'a>(&self, state: &'a mut iced::qr_code::State) -> iced::QRCode<'a> {
        let mut this = iced::QRCode::new(state);
        if self.dark.is_some() || self.light.is_some() {
            this = this.color(
                self.dark.map_or(iced::Color::BLACK, Into::into),
                self.light.map_or(iced::Color::WHITE, Into::into),
            );
        }
        if let Some(cell_size) = self.cell_size {
            this = this.cell_size(cell_size);
        }
        this
    }
}
