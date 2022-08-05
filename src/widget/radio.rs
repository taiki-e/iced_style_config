use crate::import::*;

widget_style_sheet_map!(Radio);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    width: Option<Length>,
    size: Option<u16>,
    spacing: Option<u16>,
    text_size: Option<u16>,

    // style
    #[serde(flatten)]
    pub(crate) style: Option<Style>,
}

impl StyleSheet {
    /// Creates a new [`Radio`] button.
    ///
    /// It expects:
    ///   - the value related to the [`Radio`] button
    ///   - the label of the [`Radio`] button
    ///   - the current selected value
    ///   - a function that will be called when the [`Radio`] is selected. It
    ///   receives the value of the radio and must produce a `Message`.
    pub fn new<V, Message, Renderer: iced_native::text::Renderer>(
        &self,
        value: V,
        label: impl Into<String>,
        selected: Option<V>,
        f: impl Fn(V) -> Message + 'static,
    ) -> iced_native::widget::Radio<'static, Message, Renderer>
    where
        V: Eq + Copy,
        Message: Clone,
    {
        let mut this = iced_native::widget::Radio::new(value, label, selected, f);
        if let Some(width) = self.width {
            this = this.width(width.into());
        }
        if let Some(size) = self.size {
            this = this.size(size);
        }
        if let Some(spacing) = self.spacing {
            this = this.spacing(spacing);
        }
        if let Some(text_size) = self.text_size {
            this = this.text_size(text_size);
        }
        if let Some(style) = self.style {
            this = this.style(style);
        }
        this
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Style {
    active: iced_style::radio::Style,
    hovered: iced_style::radio::Style,
}

impl iced_style::radio::StyleSheet for Style {
    fn active(&self) -> iced_style::radio::Style {
        self.active
    }

    fn hovered(&self) -> iced_style::radio::Style {
        self.hovered
    }
}

mod de {
    use crate::import::*;

    #[derive(Debug, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    pub(super) struct Style {
        #[serde(default)]
        active: Inner,
        #[serde(default)]
        hovered: Inner,
    }

    #[derive(Debug, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    struct Inner {
        background: Option<Background>,
        dot_color: Option<Color>,
        border_width: Option<f32>,
        border_color: Option<Color>,
        text_color: Option<Color>,
    }

    impl Inner {
        fn overwrite(&self, style: &mut iced_style::radio::Style) {
            if let Some(background) = self.background {
                style.background = background.into();
            }
            if let Some(dot_color) = self.dot_color {
                style.dot_color = dot_color.into();
            }
            if let Some(border_width) = self.border_width {
                style.border_width = border_width;
            }
            if let Some(border_color) = self.border_color {
                style.border_color = border_color.into();
            }
            if let Some(text_color) = self.text_color {
                style.text_color = Some(text_color.into());
            }
        }
    }

    impl<'de> Deserialize<'de> for super::Style {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let input = Style::deserialize(deserializer)?;

            let mut active = iced_style::radio::Style {
                background: iced_native::Background::Color(iced_native::Color::from_rgb(
                    0.95, 0.95, 0.95,
                )),
                dot_color: iced_native::Color::from_rgb(0.3, 0.3, 0.3),
                border_width: 1.0,
                border_color: iced_native::Color::from_rgb(0.6, 0.6, 0.6),
                text_color: None,
            };
            input.active.overwrite(&mut active);

            let mut hovered = iced_style::radio::Style {
                background: iced_native::Background::Color(iced_native::Color::from_rgb(
                    0.90, 0.90, 0.90,
                )),
                ..active
            };
            input.hovered.overwrite(&mut hovered);

            Ok(Self { active, hovered })
        }
    }
}

#[cfg(feature = "schema")]
mod schema {
    use super::*;

    impl JsonSchema for Style {
        fn schema_name() -> String {
            "RadioStyle".into()
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
