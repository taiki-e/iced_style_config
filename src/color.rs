use crate::import::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color(iced::Color);

impl From<Color> for iced::Color {
    fn from(color: Color) -> Self {
        color.0
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
#[serde(transparent)]
pub(crate) struct Background(Color);

impl From<Background> for iced::Background {
    fn from(background: Background) -> Self {
        Self::Color(background.0.into())
    }
}

#[rustfmt::skip]
#[path = "gen/color_map.rs"]
mod default_color_aliases;

use default_color_aliases::DEFAULT_COLOR_ALIASES;

scoped_thread_local!(pub(crate) static COLOR_ALIASES: RefCell<ColorMap>);

#[derive(Debug, Default)]
pub(crate) struct ColorAliases(pub(crate) ColorMap);

pub use self::map::ColorMap;

mod map {
    use crate::import::*;

    #[derive(Debug, Clone, Default)]
    pub struct ColorMap(HashMap<String, iced::Color>);

    impl ColorMap {
        pub(crate) fn with_capacity(capacity: usize) -> Self {
            Self(HashMap::with_capacity(capacity))
        }

        pub(crate) fn insert(&mut self, k: impl Into<String>, v: iced::Color) {
            let mut k = k.into();
            k.make_ascii_lowercase();
            self.0.insert(k, v);
        }

        pub fn get<S>(&self, k: S) -> Option<&iced::Color>
        where
            S: Borrow<str> + Into<String>,
        {
            if let v @ Some(..) = self.0.get(k.borrow()) {
                v
            } else {
                let mut k = k.into();
                k.make_ascii_lowercase();
                self.0.get(&k)
            }
        }

        pub fn contains_key<S>(&self, k: S) -> bool
        where
            S: Borrow<str> + Into<String>,
        {
            self.get(k).is_some()
        }
    }

    impl<S> Index<S> for ColorMap
    where
        S: Borrow<str> + Into<String>,
    {
        type Output = iced::Color;

        #[track_caller]
        fn index(&self, index: S) -> &Self::Output {
            self.get(index).expect("fal")
        }
    }

    impl FromIterator<(String, Color)> for ColorMap {
        fn from_iter<T: IntoIterator<Item = (String, Color)>>(iter: T) -> Self {
            Self(
                iter.into_iter()
                    .map(|(mut k, v)| {
                        k.make_ascii_lowercase();
                        (k, v.into())
                    })
                    .collect(),
            )
        }
    }
}

mod de {
    use crate::import::*;

    #[derive(Debug, Deserialize)]
    #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
    #[serde(untagged)]
    pub(super) enum Color {
        Alias(String),
        RgbTuple(f32, f32, f32),
        RgbStruct {
            r: f32,
            g: f32,
            b: f32,
        },
        RgbaTuple(f32, f32, f32, f32),
        RgbaStruct {
            r: f32,
            g: f32,
            b: f32,
            a: f32,
        },
        RgbaStruct2 {
            rgb: String,
            a: f32,
        },
        Rgb8Tuple(
            #[serde(deserialize_with = "from_hex")] u8,
            #[serde(deserialize_with = "from_hex")] u8,
            #[serde(deserialize_with = "from_hex")] u8,
        ),
        Rgb8Struct {
            #[serde(deserialize_with = "from_hex")]
            r: u8,
            #[serde(deserialize_with = "from_hex")]
            g: u8,
            #[serde(deserialize_with = "from_hex")]
            b: u8,
        },
        Rgba8Tuple(
            #[serde(deserialize_with = "from_hex")] u8,
            #[serde(deserialize_with = "from_hex")] u8,
            #[serde(deserialize_with = "from_hex")] u8,
            #[serde(deserialize_with = "from_f32")] f32,
        ),
        Rgba8Struct {
            #[serde(deserialize_with = "from_hex")]
            r: u8,
            #[serde(deserialize_with = "from_hex")]
            g: u8,
            #[serde(deserialize_with = "from_hex")]
            b: u8,
            #[serde(deserialize_with = "from_f32")]
            a: f32,
        },
    }

    impl<'de> Deserialize<'de> for super::Color {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Self(match Color::deserialize(deserializer)? {
                Color::Alias(color) => resolve_color_alias(color).map_err(D::Error::custom)?,
                Color::RgbTuple(r, g, b) | Color::RgbStruct { r, g, b } => {
                    iced::Color::from_rgb(r, g, b)
                }
                Color::RgbaTuple(r, g, b, a) | Color::RgbaStruct { r, g, b, a } => {
                    iced::Color::from_rgba(r, g, b, a)
                }
                Color::RgbaStruct2 { rgb, a } => {
                    let rgb = resolve_color_alias(rgb).map_err(D::Error::custom)?;
                    iced::Color::from_rgba(rgb.r, rgb.g, rgb.b, a)
                }
                Color::Rgb8Tuple(r, g, b) | Color::Rgb8Struct { r, g, b } => {
                    iced::Color::from_rgb8(r, g, b)
                }
                Color::Rgba8Tuple(r, g, b, a) | Color::Rgba8Struct { r, g, b, a } => {
                    iced::Color::from_rgba8(r, g, b, a)
                }
            }))
        }
    }

    pub(super) fn resolve_color_alias(mut key: String) -> Result<iced::Color, String> {
        key.make_ascii_lowercase();
        if let Some(color) = super::DEFAULT_COLOR_ALIASES.get(&*key).copied() {
            return Ok(color);
        }
        if let Some(color) = super::COLOR_ALIASES.with(|map| map.borrow().get(&*key).copied()) {
            return Ok(color);
        }
        Err(format!("cannot found color alias `{key}`"))
    }

    impl<'de> Deserialize<'de> for super::ColorAliases {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if let Some(map) = <Option<HashMap<_, super::Color>>>::deserialize(deserializer)? {
                let map: super::ColorMap = map.into_iter().collect();
                super::COLOR_ALIASES.with(|a| *a.borrow_mut() = map.clone());
                Ok(Self(map))
            } else {
                Ok(Self::default())
            }
        }
    }
}

#[cfg(feature = "schema")]
mod schema {
    use super::*;

    impl JsonSchema for Color {
        fn schema_name() -> String {
            "Color".into()
        }

        fn json_schema(gen: &mut SchemaGenerator) -> Schema {
            de::Color::json_schema(gen)
        }
    }

    impl Serialize for Color {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let _ = serializer;
            unimplemented!()
        }
    }

    impl JsonSchema for ColorAliases {
        fn schema_name() -> String {
            "ColorAliases".into()
        }

        fn json_schema(gen: &mut SchemaGenerator) -> Schema {
            <HashMap<String, Color>>::json_schema(gen)
        }
    }

    impl Serialize for ColorAliases {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let _ = serializer;
            unimplemented!()
        }
    }
}
