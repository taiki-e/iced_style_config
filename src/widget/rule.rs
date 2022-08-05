use crate::import::*;

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
pub struct Rule {
    #[serde(flatten)]
    pub(crate) style: Option<Style>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Style(iced_style::rule::Style);

impl Rule {
    /// Creates a horizontal [`iced::Rule`] for dividing content by the given vertical spacing.
    pub fn horizontal(&self, spacing: u16) -> iced_native::widget::Rule<'static> {
        let mut this = iced_native::widget::Rule::horizontal(spacing);
        if let Some(style) = self.style {
            this = this.style(style);
        }
        this
    }

    /// Creates a vertical [`iced::Rule`] for dividing content by the given horizontal spacing.
    pub fn vertical(&self, spacing: u16) -> iced_native::widget::Rule<'static> {
        let mut this = iced_native::widget::Rule::vertical(spacing);
        if let Some(style) = self.style {
            this = this.style(style);
        }
        this
    }
}

impl iced_style::rule::StyleSheet for Style {
    fn style(&self) -> iced_style::rule::Style {
        self.0
    }
}

mod de {
    use crate::import::*;

    #[derive(Debug, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    pub(super) struct Style {
        color: Option<Color>,
        width: Option<u16>,
        radius: Option<f32>,
        fill_mode: Option<FillMode>,
    }

    #[derive(Debug, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    #[serde(rename_all = "snake_case")]
    enum FillMode {
        Full,
        Percent(f32),
        Padded(u16),
        AsymmetricPadding(u16, u16),
    }

    impl From<FillMode> for iced_style::rule::FillMode {
        fn from(fill_mode: FillMode) -> Self {
            match fill_mode {
                FillMode::Full => Self::Full,
                FillMode::Percent(x) => Self::Percent(x),
                FillMode::Padded(x) => Self::Padded(x),
                FillMode::AsymmetricPadding(x, y) => Self::AsymmetricPadding(x, y),
            }
        }
    }

    impl<'de> Deserialize<'de> for super::Style {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let input = Style::deserialize(deserializer)?;
            let mut this = iced_style::rule::Style {
                color: [0.6, 0.6, 0.6, 0.51].into(),
                width: 1,
                radius: 0.0,
                fill_mode: iced_style::rule::FillMode::Percent(90.0),
            };

            if let Some(color) = input.color {
                this.color = color.into();
            }
            if let Some(width) = input.width {
                this.width = width;
            }
            if let Some(radius) = input.radius {
                this.radius = radius;
            }
            if let Some(fill_mode) = input.fill_mode {
                this.fill_mode = fill_mode.into();
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
            "RuleStyle".into()
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
