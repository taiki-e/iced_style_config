# iced_style_config

[![crates.io](https://img.shields.io/crates/v/iced_style_config?style=flat-square&logo=rust)](https://crates.io/crates/iced_style_config)
[![docs.rs](https://img.shields.io/badge/docs.rs-iced__style__config-blue?style=flat-square&logo=docs.rs)](https://docs.rs/iced_style_config)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-stable-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/iced_style_config/CI/main?style=flat-square&logo=github)](https://github.com/taiki-e/iced_style_config/actions)

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

*Compiler support: requires the latest stable rustc*

## Examples

On native targets:

```sh
cargo run --example hot_reloading
```

On WebAssembly:

```sh
cargo build --target wasm32-unknown-unknown --example hot_reloading
wasm-bindgen target/wasm32-unknown-unknown/debug/examples/hot_reloading.wasm --out-dir hot_reloading --web
echo '<!DOCTYPE html>
<html>
  <head>
    <meta http-equiv="Content-type" content="text/html; charset=utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Tour - Iced</title>
  </head>
  <body>
    <script type="module">
      import init from "./hot_reloading/hot_reloading.js";
      init("./hot_reloading/hot_reloading_bg.wasm");
    </script>
  </body>
</html>' > index.html
cargo run --example server
```

Note: Hot reloading on WebAssembly is not yet supported.

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

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
