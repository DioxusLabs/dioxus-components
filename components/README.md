# dioxus-components

Styled, accessible components for [Dioxus](https://dioxuslabs.com), built on the unstyled [`dioxus-primitives`](https://crates.io/crates/dioxus-primitives) crate.

Each component lives in `src/components/<name>/` as a `component.rs`, a scoped `style.css` and a `component.json` manifest. You can use them two ways:

- **As a library**: depend on this crate and import components directly.
- **As source you own**: copy a component into your app with `dx components add <name>` and edit it freely.

## Using the library

```toml
[dependencies]
dioxus-components = { git = "https://github.com/DioxusLabs/components", branch = "v0.8" }
```

Include the theme stylesheet once at the root of your app, then use any component:

```rust, ignore
use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: dioxus_components::THEME }
        Button { variant: ButtonVariant::Secondary, "Click me" }
    }
}
```

Enable the `web` feature when targeting the web.

Browse every component and its variants in the [component gallery](https://dioxuslabs.github.io/dioxus-components/).
