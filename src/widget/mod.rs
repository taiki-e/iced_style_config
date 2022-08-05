macro_rules! widget_style_sheet_map {
    ($name:ident) => {
        #[derive(Debug, Default, Deserialize)]
        #[cfg_attr(feature = "schema", derive(Serialize, JsonSchema))]
        #[serde(transparent)]
        pub struct $name(BTreeMap<String, StyleSheet>);

        impl $name {
            pub(crate) fn init(&mut self) {
                if !self.0.contains_key("default") {
                    self.0.insert("default".to_string(), StyleSheet::default());
                }
            }
        }

        impl std::ops::Deref for $name {
            type Target = StyleSheet;

            fn deref(&self) -> &Self::Target {
                &self.0["default"]
            }
        }

        impl<Q: ?Sized> std::ops::Index<&Q> for $name
        where
            String: std::borrow::Borrow<Q>,
            Q: Ord,
        {
            type Output = StyleSheet;

            fn index(&self, index: &Q) -> &Self::Output {
                &self.0[index]
            }
        }
    };
}

pub mod button;
pub mod checkbox;
pub mod container;
#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
pub mod image;
#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
pub mod image_viewer;
pub(crate) mod menu;
pub mod pick_list;
pub mod progress_bar;
#[cfg(feature = "qr_code")]
#[cfg_attr(docsrs, doc(cfg(feature = "qr_code")))]
pub mod qr_code;
pub mod radio;
pub mod scrollable;
pub mod slider;
#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
pub mod svg;
pub mod text;
pub mod text_input;

// TODO: The current implementation is not very good as it requires us to
// specify spacing in Rust code.
// pub mod rule;

// TODO: implement
// pub mod canvas;
// pub mod column;
// pub mod pane_grid;
// pub mod row;
// pub mod tooltip;
// pub mod space;
