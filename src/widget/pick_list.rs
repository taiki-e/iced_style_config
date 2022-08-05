use crate::import::*;

widget_style_sheet_map!(PickList);

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct StyleSheet {
    // layout
    padding: Option<u16>,
    width: Option<Length>,
    text_size: Option<u16>,

    // style
    #[serde(flatten)]
    pub(crate) style: Option<Style>,
}

impl StyleSheet {
    /// Creates a new [`iced::PickList`] with the given [`iced::pick_list::State`], a list of options,
    /// the current selected value, and the message to produce when an option is
    /// selected.
    pub fn new<'a, T, Message, Renderer: iced_native::text::Renderer>(
        &self,
        state: &'a mut iced_native::widget::pick_list::State<T>,
        options: impl Into<Cow<'a, [T]>>,
        selected: Option<T>,
        on_selected: impl Fn(T) -> Message + 'static,
    ) -> iced_native::widget::PickList<'a, T, Message, Renderer>
    where
        T: ToString + Eq,
        [T]: ToOwned<Owned = Vec<T>>,
    {
        let mut this = iced_native::widget::PickList::new(state, options, selected, on_selected);
        if let Some(padding) = self.padding {
            this = this.padding(padding);
        }
        if let Some(width) = self.width {
            this = this.width(width.into());
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
    menu: iced_style::menu::Style,
    active: iced_style::pick_list::Style,
    hovered: iced_style::pick_list::Style,
}

impl iced_style::pick_list::StyleSheet for Style {
    fn menu(&self) -> iced_style::menu::Style {
        self.menu
    }

    fn active(&self) -> iced_style::pick_list::Style {
        self.active
    }

    fn hovered(&self) -> iced_style::pick_list::Style {
        self.hovered
    }
}

mod de {
    use crate::import::*;

    #[derive(Debug, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    pub(super) struct Style {
        #[serde(default)]
        menu: crate::menu::Style,
        #[serde(default)]
        active: Inner,
        #[serde(default)]
        hovered: Inner,
    }

    #[derive(Debug, Default, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    struct Inner {
        text_color: Option<Color>,
        placeholder_color: Option<Color>,
        background: Option<Background>,
        border_radius: Option<f32>,
        border_width: Option<f32>,
        border_color: Option<Color>,
        icon_size: Option<f32>,
    }

    impl Inner {
        fn overwrite(&self, style: &mut iced_style::pick_list::Style) {
            if let Some(text_color) = self.text_color {
                style.text_color = text_color.into();
            }
            if let Some(placeholder_color) = self.placeholder_color {
                style.placeholder_color = placeholder_color.into();
            }
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
            if let Some(icon_size) = self.icon_size {
                style.icon_size = icon_size;
            }
        }
    }

    impl<'de> Deserialize<'de> for super::Style {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let input = Style::deserialize(deserializer)?;

            let menu = input.menu.into();

            let mut active = iced_style::pick_list::Style {
                text_color: iced_native::Color::BLACK,
                placeholder_color: [0.4, 0.4, 0.4].into(),
                background: iced_native::Background::Color([0.87, 0.87, 0.87].into()),
                border_radius: 0.0,
                border_width: 1.0,
                border_color: [0.7, 0.7, 0.7].into(),
                icon_size: 0.7,
            };
            input.active.overwrite(&mut active);

            let mut hovered =
                iced_style::pick_list::Style { border_color: iced_native::Color::BLACK, ..active };
            input.hovered.overwrite(&mut hovered);

            Ok(Self { menu, active, hovered })
        }
    }
}

#[cfg(feature = "schema")]
mod schema {
    use super::*;

    impl JsonSchema for Style {
        fn schema_name() -> String {
            "PickListStyle".into()
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
