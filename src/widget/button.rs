use crate::import::*;

widget_style_sheet_map!(Button);

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<u16>,

    // style
    #[serde(flatten)]
    pub(crate) style: Option<Style>,
}

impl StyleSheet {
    /// Creates a new [`iced::Button`] with some local [`iced::button::State`] and the given
    /// content.
    pub fn new<'a, Message, Renderer: iced_native::Renderer>(
        &self,
        state: &'a mut iced_native::widget::button::State,
        content: impl Into<iced_native::Element<'a, Message, Renderer>>,
    ) -> iced_native::widget::Button<'a, Message, Renderer>
    where
        Message: Clone,
    {
        let mut this = iced_native::widget::Button::new(state, content);
        if let Some(width) = self.width {
            this = this.width(width.into());
        }
        if let Some(height) = self.height {
            this = this.height(height.into());
        }
        if let Some(padding) = self.padding {
            this = this.padding(padding);
        }
        if let Some(style) = self.style {
            this = this.style(style);
        }
        this
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Style {
    active: iced_style::button::Style,
    hovered: iced_style::button::Style,
    pressed: iced_style::button::Style,
    disabled: iced_style::button::Style,
}

impl iced_style::button::StyleSheet for Style {
    fn active(&self) -> iced_style::button::Style {
        self.active
    }

    fn hovered(&self) -> iced_style::button::Style {
        self.hovered
    }

    fn pressed(&self) -> iced_style::button::Style {
        self.pressed
    }

    fn disabled(&self) -> iced_style::button::Style {
        self.disabled
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
        #[serde(default)]
        pressed: Inner,
        #[serde(default)]
        disabled: Inner,
    }

    #[derive(Debug, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    struct Inner {
        shadow_offset: Option<Vector>,
        background: Option<Background>,
        border_radius: Option<f32>,
        border_width: Option<f32>,
        border_color: Option<Color>,
        text_color: Option<Color>,
    }

    impl Inner {
        fn overwrite(&self, style: &mut iced_style::button::Style) {
            if let Some(shadow_offset) = self.shadow_offset {
                style.shadow_offset = shadow_offset.into();
            }
            if let Some(background) = self.background {
                style.background = Some(background.into());
            }
            if let Some(border_radius) = self.border_radius {
                style.border_radius = border_radius;
            }
            if let Some(border_width) = self.border_width {
                style.border_width = border_width;
            }
            if let Some(border_color) = self.border_color {
                style.border_color = border_color.into();
            }
            if let Some(text_color) = self.text_color {
                style.text_color = text_color.into();
            }
        }
    }

    impl<'de> Deserialize<'de> for super::Style {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let input = Style::deserialize(deserializer)?;

            let mut active = iced_style::button::Style::default();
            input.active.overwrite(&mut active);

            let mut hovered = iced_style::button::Style {
                shadow_offset: active.shadow_offset + iced_native::Vector::new(0.0, 1.0),
                ..active
            };
            input.hovered.overwrite(&mut hovered);

            let mut pressed = iced_style::button::Style {
                shadow_offset: iced_native::Vector::default(),
                ..active
            };
            input.pressed.overwrite(&mut pressed);

            let mut disabled = iced_style::button::Style {
                shadow_offset: iced_native::Vector::default(),
                background: active.background.map(|background| match background {
                    iced_native::Background::Color(color) => {
                        iced_native::Background::Color(iced_native::Color {
                            a: color.a * 0.5,
                            ..color
                        })
                    }
                }),
                text_color: iced_native::Color {
                    a: active.text_color.a * 0.5,
                    ..active.text_color
                },
                ..active
            };
            input.disabled.overwrite(&mut disabled);

            Ok(Self { active, hovered, pressed, disabled })
        }
    }
}

#[cfg(feature = "schema")]
mod schema {
    use super::*;

    impl JsonSchema for Style {
        fn schema_name() -> String {
            "ButtonStyle".into()
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
