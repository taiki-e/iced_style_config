#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Style(iced_style::menu::Style);

impl From<Style> for iced_style::menu::Style {
    fn from(style: Style) -> Self {
        style.0
    }
}

mod de {
    use crate::import::*;

    #[derive(Debug, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    pub(super) struct Style {
        text_color: Option<Color>,
        background: Option<Background>,
        border_width: Option<f32>,
        border_color: Option<Color>,
        selected_text_color: Option<Color>,
        selected_background: Option<Background>,
    }

    impl<'de> Deserialize<'de> for super::Style {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let style = Style::deserialize(deserializer)?;
            let mut this = Self::default();

            if let Some(text_color) = style.text_color {
                this.0.text_color = text_color.into();
            }
            if let Some(background) = style.background {
                this.0.background = background.into();
            }
            if let Some(border_width) = style.border_width {
                this.0.border_width = border_width;
            }
            if let Some(border_color) = style.border_color {
                this.0.border_color = border_color.into();
            }
            if let Some(selected_text_color) = style.selected_text_color {
                this.0.selected_text_color = selected_text_color.into();
            }
            if let Some(selected_background) = style.selected_background {
                this.0.selected_background = selected_background.into();
            }

            Ok(this)
        }
    }
}

#[cfg(feature = "schema")]
mod schema {
    use super::*;
    use crate::import::*;

    impl JsonSchema for Style {
        fn schema_name() -> String {
            "MenuStyle".into()
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
