/*!
Create [Iced] style sheets from configuration files.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
iced = "0.4"
iced_style_config = "0.2"
```

To disable hot reloading support:

```toml
[dependencies]
iced = "0.4"
iced_style_config = { version = "0.2", default-features = false }
```

Note: Hot reloading on WebAssembly is not yet supported.

*Compiler support: requires the latest stable rustc*

## Examples

```sh
cargo run --example hot_reloading
```

## Schemas for configuration files

The `schema.json` is the JSON schemas for the configuration files, and when combined with an extension of the editor that supports completion using the JSON schema, completion can be enabled.

### Visual Studio Code

In VS Code, you can enable completion and validation by installing the [Even Better TOML] extension and using the `evenBetterToml.schema.associations` configuration object in `settings.json`.

For example:

```json
{
  "evenBetterToml.schema.associations": {
    ".*_theme\\.toml": "https://raw.githubusercontent.com/taiki-e/iced_style_config/main/schema.json",
  }
}
```

[Even Better TOML]: https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml
[Iced]: https://github.com/hecrj/iced
*/

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![warn(missing_debug_implementations, rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(
    clippy::pedantic,
    // lints for public library
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
)]
#![allow(
    clippy::default_trait_access,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_ret_no_self,
    clippy::wildcard_imports
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod widget;
pub use widget::*;

pub mod color;
mod colors;
mod core;
mod error;
mod theme;

#[cfg(feature = "hot-reloading")]
#[cfg_attr(docsrs, doc(cfg(feature = "hot-reloading")))]
pub mod reloadable;
#[cfg(feature = "hot-reloading")]
pub use reloadable::ReloadableTheme;

pub use crate::{
    error::{Error, Result},
    theme::Theme,
};

mod import {
    #[allow(unused_imports)]
    pub(crate) use std::{
        borrow::{Borrow, Cow},
        cell::RefCell,
        collections::{BTreeMap, HashMap},
        fs,
        ops::{self, Index, RangeInclusive},
        path::Path,
        str::FromStr,
    };

    #[cfg(feature = "schema")]
    pub(crate) use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
    pub(crate) use scoped_tls::scoped_thread_local;
    #[cfg(feature = "schema")]
    pub(crate) use serde::{Serialize, Serializer};

    pub(crate) use crate::{
        color::{Background, Color},
        core::*,
        de::*,
        Error,
    };
}

mod de {
    //! Helpers for deserializing.

    #![allow(single_use_lifetimes)] // https://github.com/rust-lang/rust/issues/55058

    pub(crate) use serde::{de::Error as _, Deserialize, Deserializer};

    pub(crate) fn from_hex<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        parse_hex::<D>(s)
    }

    // https://stackoverflow.com/questions/46753955/how-to-transform-fields-during-deserialization-using-serde
    pub(crate) fn parse_hex<'de, D>(s: &str) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Some(s) = s.strip_prefix("0x") {
            u8::from_str_radix(s, 16).map_err(D::Error::custom)
        } else {
            Err(D::Error::custom(format!("invalid hex: {s}")))
        }
    }

    pub(crate) fn from_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}
