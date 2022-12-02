use crate::import::*;

widget_style_sheet_map!(Checkbox);

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
    /// Creates a new [`iced::Checkbox`].
    ///
    /// It expects:
    ///   - a boolean describing whether the [`iced::Checkbox`] is checked or not
    ///   - the label of the [`iced::Checkbox`]
    ///   - a function that will be called when the [`iced::Checkbox`] is toggled. It
    ///     will receive the new state of the [`iced::Checkbox`] and must produce a
    ///     `Message`.
    pub fn new<Message, Renderer: iced_native::text::Renderer>(
        &self,
        is_checked: bool,
        label: impl Into<String>,
        f: impl Fn(bool) -> Message + 'static,
    ) -> iced_native::widget::Checkbox<'static, Message, Renderer>
    where
        Renderer::Theme: iced_style::checkbox::StyleSheet + iced_style::text::StyleSheet,
    {
        let mut this = iced_native::widget::Checkbox::new(is_checked, label, f);
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
    active: iced_style::checkbox::Appearance,
    active_checked: iced_style::checkbox::Appearance,
    hovered: iced_style::checkbox::Appearance,
    hovered_checked: iced_style::checkbox::Appearance,
}

impl iced_style::checkbox::StyleSheet for Style {
    fn active(&self, is_checked: bool) -> iced_style::checkbox::Appearance {
        if is_checked {
            self.active_checked
        } else {
            self.active
        }
    }

    fn hovered(&self, is_checked: bool) -> iced_style::checkbox::Appearance {
        if is_checked {
            self.hovered_checked
        } else {
            self.hovered
        }
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
        #[serde(flatten)]
        default: Inner2,
        #[serde(default)]
        checked: Inner2,
    }

    #[derive(Debug, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    struct Inner2 {
        background: Option<Background>,
        checkmark_color: Option<Color>,
        border_radius: Option<f32>,
        border_width: Option<f32>,
        border_color: Option<Color>,
        text_color: Option<Color>,
    }

    impl Inner2 {
        fn overwrite(&self, style: &mut iced_style::checkbox::Appearance) {
            if let Some(background) = self.background {
                style.background = background.into();
            }
            if let Some(checkmark_color) = self.checkmark_color {
                style.checkmark_color = checkmark_color.into();
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

            let mut active = iced_style::checkbox::Appearance {
                background: iced_native::Background::Color(iced_native::Color::from_rgb(
                    0.95, 0.95, 0.95,
                )),
                checkmark_color: iced_native::Color::from_rgb(0.3, 0.3, 0.3),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: iced_native::Color::from_rgb(0.6, 0.6, 0.6),
                text_color: None,
            };
            input.active.default.overwrite(&mut active);
            let mut active_checked = active;
            input.active.checked.overwrite(&mut active_checked);

            let mut hovered = iced_style::checkbox::Appearance {
                background: iced_native::Background::Color(iced_native::Color::from_rgb(
                    0.90, 0.90, 0.90,
                )),
                ..active
            };
            let mut hovered_checked = iced_style::checkbox::Appearance {
                background: iced_native::Background::Color(iced_native::Color::from_rgb(
                    0.90, 0.90, 0.90,
                )),
                ..active_checked
            };
            input.hovered.default.overwrite(&mut hovered);
            input.hovered.default.overwrite(&mut hovered_checked);
            input.hovered.checked.overwrite(&mut hovered_checked);

            Ok(Self { active, active_checked, hovered, hovered_checked })
        }
    }
}

#[cfg(feature = "schema")]
mod schema {
    use super::*;

    impl JsonSchema for Style {
        fn schema_name() -> String {
            "CheckboxStyle".into()
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
