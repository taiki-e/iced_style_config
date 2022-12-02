use crate::import::*;

widget_style_sheet_map!(Viewer);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    padding: Option<u16>,
    width: Option<Length>,
    height: Option<Length>,
    min_scale: Option<f32>,
    max_scale: Option<f32>,
    scale_step: Option<f32>,
}

impl StyleSheet {
    /// Creates a new [`iced::image::Viewer`] with the given
    /// [`iced::image::viewer::State`] and [`iced::image::Handle`].
    pub fn new<'a>(
        &self,
        handle: impl Into<iced_native::image::Handle>,
    ) -> iced_native::widget::image::Viewer<iced_native::image::Handle> {
        let mut this = iced_native::widget::image::Viewer::new(handle.into());
        if let Some(padding) = self.padding {
            this = this.padding(padding);
        }
        if let Some(width) = self.width {
            this = this.width(width.into());
        }
        if let Some(height) = self.height {
            this = this.height(height.into());
        }
        if let Some(min_scale) = self.min_scale {
            this = this.min_scale(min_scale);
        }
        if let Some(max_scale) = self.max_scale {
            this = this.max_scale(max_scale);
        }
        if let Some(scale_step) = self.scale_step {
            this = this.scale_step(scale_step);
        }
        this
    }
}
