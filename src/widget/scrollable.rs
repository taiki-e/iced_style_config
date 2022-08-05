use crate::import::*;

widget_style_sheet_map!(Scrollable);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    spacing: Option<u16>,
    padding: Option<u16>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<u32>,
    max_height: Option<u32>,
    align_items: Option<Alignment>,
    scrollbar_width: Option<u16>,
    scrollbar_margin: Option<u16>,
    scroller_width: Option<u16>,

    // style
    #[serde(flatten)]
    pub(crate) style: Option<Style>,
}

impl StyleSheet {
    /// Creates a new [`iced::Scrollable`] with the given [`iced::scrollable::State`].
    pub fn new<'a, Message, Renderer: iced_native::Renderer>(
        &self,
        state: &'a mut iced_native::widget::scrollable::State,
    ) -> iced_native::widget::Scrollable<'a, Message, Renderer> {
        let mut this = iced_native::widget::Scrollable::new(state);
        if let Some(spacing) = self.spacing {
            this = this.spacing(spacing);
        }
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
        if let Some(align_items) = self.align_items {
            this = this.align_items(align_items.into());
        }
        if let Some(scrollbar_width) = self.scrollbar_width {
            this = this.scrollbar_width(scrollbar_width);
        }
        if let Some(scrollbar_margin) = self.scrollbar_margin {
            this = this.scrollbar_margin(scrollbar_margin);
        }
        if let Some(scroller_width) = self.scroller_width {
            this = this.scroller_width(scroller_width);
        }
        if let Some(style) = self.style {
            this = this.style(style);
        }
        this
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Style {
    active: iced_style::scrollable::Scrollbar,
    hovered: iced_style::scrollable::Scrollbar,
    dragging: iced_style::scrollable::Scrollbar,
}

impl iced_style::scrollable::StyleSheet for Style {
    fn active(&self) -> iced_style::scrollable::Scrollbar {
        self.active
    }

    fn hovered(&self) -> iced_style::scrollable::Scrollbar {
        self.hovered
    }

    fn dragging(&self) -> iced_style::scrollable::Scrollbar {
        self.dragging
    }
}

mod de {
    use crate::import::*;

    #[derive(Debug, Clone, Copy, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    pub(super) struct Style {
        #[serde(default)]
        active: Scrollbar,
        #[serde(default)]
        hovered: Scrollbar,
        #[serde(default)]
        dragging: Scrollbar,
    }

    #[derive(Debug, Clone, Copy, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    struct Scrollbar {
        background: Option<Background>,
        border_radius: Option<f32>,
        border_width: Option<f32>,
        border_color: Option<Color>,
        scroller: Option<Scroller>,
    }

    impl Scrollbar {
        fn overwrite(&self, scrollbar: &mut iced_style::scrollable::Scrollbar) {
            if let Some(background) = self.background {
                scrollbar.background = Some(background.into());
            }
            if let Some(border_radius) = self.border_radius {
                scrollbar.border_radius = border_radius;
            }
            if let Some(border_width) = self.border_width {
                scrollbar.border_width = border_width;
            }
            if let Some(border_color) = self.border_color {
                scrollbar.border_color = border_color.into();
            }
            if let Some(scroller) = &self.scroller {
                if let Some(color) = scroller.color {
                    scrollbar.scroller.color = color.into();
                }
                if let Some(border_radius) = scroller.border_radius {
                    scrollbar.scroller.border_radius = border_radius;
                }
                if let Some(border_width) = scroller.border_width {
                    scrollbar.scroller.border_width = border_width;
                }
                if let Some(border_color) = scroller.border_color {
                    scrollbar.scroller.border_color = border_color.into();
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    struct Scroller {
        color: Option<Color>,
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

            let mut active = iced_style::scrollable::Scrollbar {
                background: None,
                border_radius: 5.0,
                border_width: 0.0,
                border_color: iced_native::Color::TRANSPARENT,
                scroller: iced_style::scrollable::Scroller {
                    color: [0.0, 0.0, 0.0, 0.7].into(),
                    border_radius: 5.0,
                    border_width: 0.0,
                    border_color: iced_native::Color::TRANSPARENT,
                },
            };
            input.active.overwrite(&mut active);

            let mut hovered = iced_style::scrollable::Scrollbar {
                background: Some(iced_native::Background::Color([0.0, 0.0, 0.0, 0.3].into())),
                ..active
            };
            input.hovered.overwrite(&mut hovered);

            let mut dragging = hovered;
            input.dragging.overwrite(&mut dragging);

            Ok(Self { active, hovered, dragging })
        }
    }
}

#[cfg(feature = "schema")]
mod schema {
    use super::*;

    impl JsonSchema for Style {
        fn schema_name() -> String {
            "ScrollableStyle".into()
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
