use crate::import::*;

/// An Iced style sheet.
#[derive(Debug, Default)]
pub struct Theme(ThemeInner);

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        crate::color::COLOR_ALIASES.set(&RefCell::default(), || {
            let mut this = ThemeInner::deserialize(deserializer)?;
            this.button.init();
            this.checkbox.init();
            this.container.init();
            this.image.init();
            this.image_viewer.init();
            this.pick_list.init();
            this.progress_bar.init();
            #[cfg(feature = "qr_code")]
            this.qr_code.init();
            this.radio.init();
            this.scrollable.init();
            this.slider.init();
            this.svg.init();
            this.text.init();
            this.text_input.init();
            Ok(Self(this))
        })
    }
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
struct ThemeInner {
    /// Colors defined in the style sheet.
    color: crate::color::ColorAliases,
    /// Style sheet for [`iced::Button`].
    #[serde(default)]
    button: crate::button::Button,
    /// Style sheet for [`iced::Checkbox`].
    #[serde(default)]
    checkbox: crate::checkbox::Checkbox,
    /// Style sheet for [`iced::Container`].
    #[serde(default)]
    container: crate::container::Container,
    /// Style sheet for [`iced::Image`].
    #[serde(default)]
    image: crate::image::Image,
    /// Style sheet for [`iced::image::Viewer`].
    #[serde(default)]
    image_viewer: crate::image_viewer::Viewer,
    /// Style sheet for [`iced::PickList`].
    #[serde(default)]
    pick_list: crate::pick_list::PickList,
    /// Style sheet for [`iced::ProgressBar`].
    #[serde(default)]
    progress_bar: crate::progress_bar::ProgressBar,
    /// Style sheet for [`iced::QRCode`].
    #[cfg(feature = "qr_code")]
    #[serde(default)]
    qr_code: crate::qr_code::QRCode,
    /// Style sheet for [`iced::Radio`].
    #[serde(default)]
    radio: crate::radio::Radio,
    // /// Style sheet for [`iced::Rule`].
    // #[serde(default)]
    // rule: crate::rule::Rule,
    /// Style sheet for [`iced::Scrollable`].
    #[serde(default)]
    scrollable: crate::scrollable::Scrollable,
    /// Style sheet for [`iced::Slider`].
    #[serde(default)]
    slider: crate::slider::Slider,
    /// Style sheet for [`iced::Svg`].
    #[serde(default)]
    svg: crate::svg::Svg,
    /// Style sheet for [`iced::Text`].
    #[serde(default)]
    text: crate::text::Text,
    /// Style sheets for [`iced::TextInput`].
    #[serde(default)]
    text_input: crate::text_input::TextInput,
}

impl FromStr for Theme {
    type Err = Error;

    /// Creates a new Iced style sheets from a string of TOML text.
    ///
    /// This function is equivalent to the following code:
    ///
    /// ```rust
    /// use iced_style_config::Theme;
    /// # fn from_str(s: &str) -> Result<Theme, Box<dyn std::error::Error>> {
    /// let theme: Theme = toml::from_str(s)?;
    /// Ok(theme)
    /// # }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(Error::new)
    }
}

impl Theme {
    /// Creates a new Iced style sheets from a TOML file.
    ///
    /// This function is equivalent to the following code:
    ///
    /// ```rust
    /// use iced_style_config::Theme;
    /// # use std::path::Path;
    /// # fn from_file(path: impl AsRef<Path>) -> Result<Theme, Box<dyn std::error::Error>> {
    /// let bytes = std::fs::read(path)?;
    /// let theme: Theme = toml::from_slice(&bytes)?;
    /// Ok(theme)
    /// # }
    /// ```
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let bytes = fs::read(path).map_err(Error::new)?;
        toml::from_slice(&bytes).map_err(Error::new)
    }

    /// Returns colors defined in the style sheet.
    pub fn color(&self) -> &crate::color::ColorMap {
        &self.0.color.0
    }

    /// Returns style sheet for [`iced::Button`].
    pub fn button(&self) -> &crate::button::Button {
        &self.0.button
    }

    /// Returns style sheet for [`iced::Checkbox`].
    pub fn checkbox(&self) -> &crate::checkbox::Checkbox {
        &self.0.checkbox
    }

    /// Returns style sheet for [`iced::Container`].
    pub fn container(&self) -> &crate::container::Container {
        &self.0.container
    }

    /// Returns style sheet for [`iced::Image`].
    pub fn image(&self) -> &crate::image::Image {
        &self.0.image
    }

    /// Returns style sheet for [`iced::image::Viewer`].
    pub fn image_viewer(&self) -> &crate::image_viewer::Viewer {
        &self.0.image_viewer
    }

    /// Returns style sheet for [`iced::PickList`].
    pub fn pick_list(&self) -> &crate::pick_list::PickList {
        &self.0.pick_list
    }

    /// Returns style sheet for [`iced::ProgressBar`].
    pub fn progress_bar(&self) -> &crate::progress_bar::ProgressBar {
        &self.0.progress_bar
    }

    /// Returns style sheet for [`iced::QRCode`].
    #[cfg(feature = "qr_code")]
    #[cfg_attr(docsrs, doc(cfg(feature = "qr_code")))]
    pub fn qr_code(&self) -> &crate::qr_code::QRCode {
        &self.0.qr_code
    }

    /// Returns style sheet for [`iced::Radio`].
    pub fn radio(&self) -> &crate::radio::Radio {
        &self.0.radio
    }

    // TODO: The current implementation is not very good as it requires us to
    // /// Returns style sheet for [`iced::Rule`].
    // pub fn rule(&self) -> &crate::rule::Rule {
    //     &self.0.rule
    // }

    /// Returns style sheet for [`iced::Scrollable`].
    pub fn scrollable(&self) -> &crate::scrollable::Scrollable {
        &self.0.scrollable
    }

    /// Returns style sheet for [`iced::Slider`].
    pub fn slider(&self) -> &crate::slider::Slider {
        &self.0.slider
    }

    /// Returns style sheet for [`iced::Svg`].
    pub fn svg(&self) -> &crate::svg::Svg {
        &self.0.svg
    }

    /// Returns style sheet for [`iced::Text`].
    pub fn text(&self) -> &crate::text::Text {
        &self.0.text
    }

    /// Returns style sheet for [`iced::TextInput`].
    pub fn text_input(&self) -> &crate::text_input::TextInput {
        &self.0.text_input
    }
}

#[cfg(feature = "schema")]
mod schema {
    use super::*;

    impl JsonSchema for Theme {
        fn schema_name() -> String {
            "Theme".into()
        }

        fn json_schema(gen: &mut SchemaGenerator) -> Schema {
            ThemeInner::json_schema(gen)
        }
    }

    impl Serialize for Theme {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let _ = serializer;
            unimplemented!()
        }
    }
}
