#![doc = include_str!("../README.md")]

use dioxus::prelude::*;

mod components;
pub use components::*;

/// The shared theme stylesheet every component's styles build on.
///
/// Include it once at the root of your app:
///
/// ```rust, ignore
/// document::Stylesheet { href: dioxus_components::THEME }
/// ```
pub const THEME: Asset = asset!("/assets/dx-components-theme.css");
