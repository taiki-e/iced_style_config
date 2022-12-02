use crate::import::*;

widget_style_sheet_map!(TextInput);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    width: Option<Length>,
    padding: Option<u16>,
    size: Option<u16>,

    // style
    #[serde(flatten)]
    pub(crate) style: Option<Style>,
}

impl StyleSheet {
    /// Creates a new [`iced::TextInput`].
    ///
    /// It expects:
    /// - some [`iced::text_input::State`]
    /// - a placeholder
    /// - the current value
    /// - a function that produces a message when the [`iced::TextInput`] changes
    pub fn new<'a, Message, Renderer: iced_native::text::Renderer>(
        &self,
        state: &'a mut iced_native::widget::text_input::State,
        placeholder: &str,
        value: &str,
        on_change: impl Fn(String) -> Message + 'static,
    ) -> iced_native::widget::TextInput<'a, Message, Renderer>
    where
        Message: Clone,
        Renderer::Theme: iced_style::text_input::StyleSheet,
    {
        let mut this = iced_native::widget::TextInput::new(state, placeholder, value, on_change);
        if let Some(width) = self.width {
            this = this.width(width.into());
        }
        if let Some(padding) = self.padding {
            this = this.padding(padding);
        }
        if let Some(size) = self.size {
            this = this.size(size);
        }
        if let Some(style) = self.style {
            this = this.style(style);
        }
        this
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Style {
    active: iced_style::text_input::Appearance,
    focused: iced_style::text_input::Appearance,
    hovered: iced_style::text_input::Appearance,
    placeholder_color: iced_native::Color,
    value_color: iced_native::Color,
    selection_color: iced_native::Color,
}

impl iced_style::text_input::StyleSheet for Style {
    fn active(&self) -> iced_style::text_input::Appearance {
        self.active
    }

    fn focused(&self) -> iced_style::text_input::Appearance {
        self.focused
    }

    fn hovered(&self) -> iced_style::text_input::Appearance {
        self.hovered
    }

    fn placeholder_color(&self) -> iced_native::Color {
        self.placeholder_color
    }

    fn value_color(&self) -> iced_native::Color {
        self.value_color
    }

    fn selection_color(&self) -> iced_native::Color {
        self.selection_color
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
        focused: Inner,
        #[serde(default)]
        hovered: Inner,
        placeholder_color: Option<Color>,
        value_color: Option<Color>,
        selection_color: Option<Color>,
    }

    #[derive(Debug, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    struct Inner {
        background: Option<Background>,
        border_radius: Option<f32>,
        border_width: Option<f32>,
        border_color: Option<Color>,
    }

    impl Inner {
        fn overwrite(&self, style: &mut iced_style::text_input::Appearance) {
            if let Some(background) = self.background {
                style.background = background.into();
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
        }
    }

    impl<'de> Deserialize<'de> for super::Style {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let input = Style::deserialize(deserializer)?;

            let mut active = iced_style::text_input::Appearance {
                background: iced_native::Background::Color(iced_native::Color::WHITE),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: iced_native::Color::from_rgb(0.7, 0.7, 0.7),
            };
            input.active.overwrite(&mut active);

            let mut focused = iced_style::text_input::Appearance {
                border_color: iced_native::Color::from_rgb(0.5, 0.5, 0.5),
                ..active
            };
            input.focused.overwrite(&mut focused);

            let mut hovered = focused;
            input.hovered.overwrite(&mut hovered);

            let placeholder_color = input
                .placeholder_color
                .map_or(iced_native::Color::from_rgb(0.7, 0.7, 0.7), Into::into);
            let value_color =
                input.value_color.map_or(iced_native::Color::from_rgb(0.3, 0.3, 0.3), Into::into);
            let selection_color = input
                .selection_color
                .map_or(iced_native::Color::from_rgb(0.8, 0.8, 1.0), Into::into);

            Ok(Self { active, focused, hovered, placeholder_color, value_color, selection_color })
        }
    }
}

#[cfg(feature = "schema")]
mod schema {
    use super::*;

    impl JsonSchema for Style {
        fn schema_name() -> String {
            "TextInputStyle".into()
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
