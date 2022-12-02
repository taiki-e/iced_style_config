use crate::import::*;

widget_style_sheet_map!(ProgressBar);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    width: Option<Length>,
    height: Option<Length>,

    // style
    #[serde(flatten)]
    pub(crate) style: Option<Style>,
}

impl StyleSheet {
    /// Creates a new [`iced::ProgressBar`].
    ///
    /// It expects:
    ///   - an inclusive range of possible values
    ///   - the current value of the [`iced::ProgressBar`]
    pub fn new<Renderer: iced_native::Renderer>(
        &self,
        range: RangeInclusive<f32>,
        value: f32,
    ) -> iced_native::widget::ProgressBar<Renderer>
    where
        Renderer::Theme: iced_style::progress_bar::StyleSheet,
    {
        let mut this = iced_native::widget::ProgressBar::new(range, value);
        if let Some(width) = self.width {
            this = this.width(width.into());
        }
        if let Some(height) = self.height {
            this = this.height(height.into());
        }
        if let Some(style) = self.style {
            this = this.style(style);
        }
        this
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Style(iced_style::progress_bar::Appearance);

impl iced_style::progress_bar::StyleSheet for Style {
    fn appearance(&self) -> iced_style::progress_bar::Appearance {
        self.0
    }
}

mod de {
    use crate::import::*;

    #[derive(Debug, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    pub(super) struct Style {
        background: Option<Background>,
        bar: Option<Background>,
        border_radius: Option<f32>,
    }

    impl<'de> Deserialize<'de> for super::Style {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let input = Style::deserialize(deserializer)?;
            let mut this = iced_style::progress_bar::Appearance {
                background: iced_native::Background::Color(iced_native::Color::from_rgb(
                    0.6, 0.6, 0.6,
                )),
                bar: iced_native::Background::Color(iced_native::Color::from_rgb(0.3, 0.9, 0.3)),
                border_radius: 5.0,
            };

            if let Some(background) = input.background {
                this.background = background.into();
            }
            if let Some(bar) = input.bar {
                this.bar = bar.into();
            }
            if let Some(border_radius) = input.border_radius {
                this.border_radius = border_radius;
            }

            Ok(Self(this))
        }
    }
}

#[cfg(feature = "schema")]
mod schema {
    use super::*;

    impl JsonSchema for Style {
        fn schema_name() -> String {
            "ProgressBarStyle".into()
        }

        fn json_schema(gen: &mut SchemaGenerator) -> Schema {
            de::Style::json_schema(gen)
        }
    }

    impl Serialize for Style {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let _ = serializer;
            unimplemented!()
        }
    }
}
