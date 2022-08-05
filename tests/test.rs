#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::path::Path;

use iced_style_config::Theme;

#[test]
fn parse() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    Theme::from_file(path.join("examples/dark_theme.toml")).unwrap();
    Theme::from_file(path.join("examples/light_theme.toml")).unwrap();
}
