use crate::import::*;

#[derive(Debug, Clone, Copy, Deserialize)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum Length {
    Fill,
    FillPortion(u16),
    Shrink,
    Units(u16),
}

impl From<Length> for iced_native::Length {
    fn from(length: Length) -> Self {
        match length {
            Length::Fill => Self::Fill,
            Length::FillPortion(factor) => Self::FillPortion(factor),
            Length::Shrink => Self::Shrink,
            Length::Units(units) => Self::Units(units),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum Alignment {
    Start,
    Center,
    End,
    Fill,
}

impl From<Alignment> for iced_native::Alignment {
    fn from(align: Alignment) -> Self {
        match align {
            Alignment::Start => Self::Start,
            Alignment::Center => Self::Center,
            Alignment::End => Self::End,
            Alignment::Fill => Self::Fill,
        }
    }
}

pub(crate) mod alignment {
    use super::*;

    #[derive(Debug, Clone, Copy, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    #[serde(rename_all = "snake_case")]
    pub(crate) enum Horizontal {
        Left,
        Center,
        Right,
    }

    impl From<Horizontal> for iced_native::alignment::Horizontal {
        fn from(align: Horizontal) -> Self {
            match align {
                Horizontal::Left => Self::Left,
                Horizontal::Center => Self::Center,
                Horizontal::Right => Self::Right,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    #[serde(rename_all = "snake_case")]
    pub(crate) enum Vertical {
        Top,
        Center,
        Bottom,
    }

    impl From<Vertical> for iced_native::alignment::Vertical {
        fn from(align: Vertical) -> Self {
            match align {
                Vertical::Top => Self::Top,
                Vertical::Center => Self::Center,
                Vertical::Bottom => Self::Bottom,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
#[serde(untagged)]
pub(crate) enum Vector {
    Tuple(f32, f32),
    Struct { x: f32, y: f32 },
}

impl From<Vector> for iced_native::Vector {
    fn from(vector: Vector) -> Self {
        match vector {
            Vector::Struct { x, y } | Vector::Tuple(x, y) => Self { x, y },
        }
    }
}
