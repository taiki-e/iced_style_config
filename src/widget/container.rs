use crate::import::*;

widget_style_sheet_map!(Container);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    padding: Option<u16>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<u32>,
    max_height: Option<u32>,
    align_x: Option<alignment::Horizontal>,
    align_y: Option<alignment::Vertical>,

    // style
    #[serde(flatten)]
    pub(crate) style: Option<Style>,
}

impl StyleSheet {
    /// Creates an empty [`iced::Container`].
    pub fn new<'a, Message>(
        &self,
        content: impl Into<iced::Element<'a, Message>>,
    ) -> iced::Container<'a, Message> {
        let mut this = iced::Container::new(content);
        if let Some(padding) = self.padding {
            this = this.padding(padding);
        }
        if let Some(width) = self.width {
            this = this.width(width.into());
        }
        if let Some(height) = self.height {
            this = this.height(height.into());
        }
        if let Some(max_width) = self.max_width {
            this = this.max_width(max_width);
        }
        if let Some(max_height) = self.max_height {
            this = this.max_height(max_height);
        }
        if let Some(align_x) = self.align_x {
            this = this.align_x(align_x.into());
        }
        if let Some(align_y) = self.align_y {
            this = this.align_y(align_y.into());
        }
        if let Some(style) = self.style {
            this = this.style(style);
        }
        this
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Style(iced::container::Style);

impl iced::container::StyleSheet for Style {
    fn style(&self) -> iced::container::Style {
        self.0
    }
}

mod de {
    use crate::import::*;

    #[derive(Debug, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    pub(super) struct Style {
        text_color: Option<Color>,
        background: Option<Background>,
        border_radius: Option<f32>,
        border_width: Option<f32>,
        border_color: Option<Color>,
    }

    impl<'de> Deserialize<'de> for super::Style {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let input = Style::deserialize(deserializer)?;
            let mut this = iced::container::Style::default();

            if let Some(text_color) = input.text_color {
                this.text_color = Some(text_color.into());
            }
            if let Some(background) = input.background {
                this.background = Some(background.into());
            }
            if let Some(border_radius) = input.border_radius {
                this.border_radius = border_radius;
            }
            if let Some(border_width) = input.border_width {
                this.border_width = border_width;
            }
            if let Some(border_color) = input.border_color {
                this.border_color = border_color.into();
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
            "ContainerStyle".into()
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
