# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

- Update `notify` to 5.0.0-pre.16.

## [0.2.0] - 2022-08-05

- Reduce dependencies by using `iced_*` subcrates instead of facade `iced` crate.

- Remove `image` and `svg` features. They are now enabled by default.

- Remove `From<iced::Error>` implementation on `iced_style_config::Error`.

## [0.1.0] - 2022-08-05

Initial release

[Unreleased]: https://github.com/taiki-e/iced_style_config/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/taiki-e/iced_style_config/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/taiki-e/iced_style_config/releases/tag/v0.1.0
