use crate::import::*;

widget_style_sheet_map!(Slider);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    width: Option<Length>,
    height: Option<u16>,

    // style
    #[serde(flatten)]
    pub(crate) style: Option<Style>,
}

impl StyleSheet {
    /// Creates a new [`iced::Slider`].
    ///
    /// It expects:
    ///   - the local [`iced::slider::State`] of the [`iced::Slider`]
    ///   - an inclusive range of possible values
    ///   - the current value of the [`iced::Slider`]
    ///   - a function that will be called when the [`iced::Slider`] is dragged.
    ///   It receives the new value of the [`iced::Slider`] and must produce a
    ///   `Message`.
    pub fn new<'a, T, Message>(
        &self,
        state: &'a mut iced::slider::State,
        range: RangeInclusive<T>,
        value: T,
        on_change: impl Fn(T) -> Message + 'static,
    ) -> iced::Slider<'a, T, Message>
    where
        T: Copy + From<u8> + PartialOrd,
        Message: Clone,
    {
        let mut this = iced::Slider::new(state, range, value, on_change);
        if let Some(width) = self.width {
            this = this.width(width.into());
        }
        if let Some(height) = self.height {
            this = this.height(height);
        }
        if let Some(style) = self.style {
            this = this.style(style);
        }
        this
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Style {
    active: iced::slider::Style,
    hovered: iced::slider::Style,
    dragging: iced::slider::Style,
}

impl iced::slider::StyleSheet for Style {
    fn active(&self) -> iced::slider::Style {
        self.active
    }

    fn hovered(&self) -> iced::slider::Style {
        self.hovered
    }

    fn dragging(&self) -> iced::slider::Style {
        self.dragging
    }
}

mod de {
    use crate::import::*;

    #[derive(Debug, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    pub(super) struct Style {
        #[serde(default)]
        active: Inner,
        #[serde(default)]
        hovered: Inner,
        #[serde(default)]
        dragging: Inner,
    }

    #[derive(Debug, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    struct Inner {
        rail_colors: Option<(Color, Color)>,
        handle: Option<Handle>,
    }

    impl Inner {
        fn overwrite(&self, style: &mut iced::slider::Style) {
            if let Some(rail_colors) = self.rail_colors {
                style.rail_colors = (rail_colors.0.into(), rail_colors.1.into());
            }
            if let Some(handle) = &self.handle {
                if let Some(shape) = handle.shape {
                    style.handle.shape = shape.into();
                }
                if let Some(color) = handle.color {
                    style.handle.color = color.into();
                }
                if let Some(border_color) = handle.border_color {
                    style.handle.border_color = border_color.into();
                }
                if let Some(border_width) = handle.border_width {
                    style.handle.border_width = border_width;
                }
            }
        }
    }

    #[derive(Debug, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    struct Handle {
        shape: Option<HandleShape>,
        color: Option<Color>,
        border_width: Option<f32>,
        border_color: Option<Color>,
    }

    #[derive(Debug, Clone, Copy, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    #[serde(rename_all = "snake_case")]
    enum HandleShape {
        Circle { radius: f32 },
        Rectangle { width: u16, border_radius: f32 },
    }

    impl From<HandleShape> for iced::slider::HandleShape {
        fn from(shape: HandleShape) -> Self {
            match shape {
                HandleShape::Circle { radius } => Self::Circle { radius },
                HandleShape::Rectangle { width, border_radius } => {
                    Self::Rectangle { width, border_radius }
                }
            }
        }
    }

    impl<'de> Deserialize<'de> for super::Style {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let input = Style::deserialize(deserializer)?;

            let mut active = iced::slider::Style {
                rail_colors: ([0.6, 0.6, 0.6, 0.5].into(), iced::Color::WHITE),
                handle: iced::slider::Handle {
                    shape: iced::slider::HandleShape::Rectangle { width: 8, border_radius: 4.0 },
                    color: iced::Color::from_rgb(0.95, 0.95, 0.95),
                    border_color: iced::Color::from_rgb(0.6, 0.6, 0.6),
                    border_width: 1.0,
                },
            };
            input.active.overwrite(&mut active);

            let mut hovered = iced::slider::Style {
                handle: iced::slider::Handle {
                    color: iced::Color::from_rgb(0.90, 0.90, 0.90),
                    ..active.handle
                },
                ..active
            };
            input.hovered.overwrite(&mut hovered);

            let mut dragging = iced::slider::Style {
                handle: iced::slider::Handle {
                    color: iced::Color::from_rgb(0.85, 0.85, 0.85),
                    ..active.handle
                },
                ..active
            };
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
            "SliderStyle".into()
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
