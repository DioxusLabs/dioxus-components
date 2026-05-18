#![allow(unpredictable_function_pointer_comparisons)]

use crate::components::{
    avatar::{AvatarImageSize, ImageAvatar},
    badge::{Badge, BadgeVariant, VerifiedIcon},
    button::{Button, ButtonVariant},
    checkbox::Checkbox,
    color_picker::ColorPicker,
    combobox::{Combobox, ComboboxEmpty, ComboboxOption},
    drag_and_drop_list::DragAndDropList,
    input::Input,
    item::{
        Item, ItemContent, ItemDescription, ItemMedia, ItemMediaVariant, ItemTitle, ItemVariant,
    },
    label::Label,
    progress::Progress,
    radio_group::{RadioGroup, RadioItem},
    slider::Slider,
    switch::Switch,
    tabs::{TabContent, TabList, TabTrigger, Tabs, TabsVariant},
    textarea::{Textarea, TextareaVariant},
    toggle_group::{ToggleGroup, ToggleItem},
};
use core::panic;
use dioxus::prelude::{dioxus_router::LinkProps, *};
use dioxus_code::{advanced::HighlightedSource, Code, CodeTheme, Theme};
use dioxus_i18n::prelude::{use_init_i18n, I18nConfig};
use dioxus_icons::lucide::{
    ArrowRight, ArrowUpRight, Check, ChevronDown, ChevronLeft, Copy, ExternalLink, Mail, Menu,
    Pause, Play, SkipBack, SkipForward, X,
};
use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use unic_langid::{langid, LanguageIdentifier};

mod components;
mod dashboard;
mod theme;

#[derive(Copy, Clone, PartialEq)]
enum ComponentType {
    /// Normal component as default.
    Normal,
    /// Component that render the preview inside an iframe for isolation.
    Block,
}

#[derive(Clone, PartialEq)]
struct ComponentDemoData {
    name: &'static str,
    r#type: ComponentType,
    description: &'static str,
    docs: &'static str,
    component: HighlightedCode,
    style: HighlightedCode,
    variants: &'static [ComponentVariantDemoData],
}

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Clone, PartialEq)]
struct ComponentVariantDemoData {
    name: &'static str,
    rs_highlighted: HighlightedCode,
    css_highlighted: Option<HighlightedCode>,
    component: fn() -> Element,
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[cfg(feature = "server")]
fn main() {
    use dioxus::server::axum::{routing::post, Json, Router};
    use dioxus::server::{DioxusRouterExt, IncrementalRendererConfig, ServeConfig};

    dioxus::server::serve(|| async {
        let cfg = ServeConfig::builder()
            // Enable incremental rendering
            .incremental(
                IncrementalRendererConfig::new()
                    // Store static files in the public directory where other static assets like wasm are stored
                    .static_dir(
                        std::env::current_exe()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .join("public"),
                    )
                    // Don't clear the public folder on every build. The public folder has other files including the wasm
                    // binary and static assets required for the app to run
                    .clear_cache(false),
            )
            .enable_out_of_order_streaming();

        // Workaround for dioxus-cli 0.7.6: with `--base-path`, the `static_routes`
        // server function ends up under `/<base>/api/static_routes`, but the SSG
        // step POSTs to the unprefixed `/api/static_routes` and fails to parse
        // the empty body. Expose a shim at the root that returns the route list.
        let router = Router::new()
            .route(
                "/api/static_routes",
                post(|| async {
                    Json(
                        Route::static_routes()
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<String>>(),
                    )
                }),
            )
            .serve_dioxus_application(cfg, App);

        Ok(router)
    })
}

#[component]
pub fn App() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("i18n/en-US.ftl")))
            .with_locale((langid!("fr-FR"), include_str!("i18n/fr-FR.ftl")))
            .with_locale((langid!("es-ES"), include_str!("i18n/es-ES.ftl")))
            .with_locale((langid!("de-DE"), include_str!("i18n/de-DE.ftl")))
    });

    rsx! {
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(AppLayout)]
    #[layout(NavigationLayout)]
    #[route("/?:iframe&:dark_mode")]
    Home {
        iframe: Option<bool>,
        dark_mode: Option<bool>,
    },
    #[route("/docs?:dark_mode")]
    Docs { dark_mode: Option<bool> },
    #[route("/demos?:dark_mode")]
    Demos { dark_mode: Option<bool> },
    #[route("/component/?:name&:iframe&:dark_mode")]
    ComponentDemo {
        name: String,
        iframe: Option<bool>,
        dark_mode: Option<bool>,
    },
    #[end_layout]
    #[route("/component/block/?:name&:variant&:dark_mode")]
    ComponentBlockDemo {
        name: String,
        variant: Option<String>,
        dark_mode: Option<bool>,
    },
    #[route("/dashboard/email-client?:dark_mode")]
    EmailClientDashboard { dark_mode: Option<bool> },
}

impl Route {
    pub fn iframe(&self) -> Option<bool> {
        match self {
            Route::Home { iframe, .. } => *iframe,
            Route::Docs { .. } => None,
            Route::Demos { .. } => None,
            Route::ComponentDemo { iframe, .. } => *iframe,
            Route::ComponentBlockDemo { .. } => None,
            Route::EmailClientDashboard { .. } => None,
        }
    }

    pub fn in_iframe() -> Option<bool> {
        let route: Self = router().current();
        route.iframe()
    }

    pub fn dark_mode(&self) -> Option<bool> {
        match self {
            Route::Home { dark_mode, .. } => *dark_mode,
            Route::Docs { dark_mode, .. } => *dark_mode,
            Route::Demos { dark_mode, .. } => *dark_mode,
            Route::ComponentDemo { dark_mode, .. } => *dark_mode,
            Route::ComponentBlockDemo { dark_mode, .. } => *dark_mode,
            Route::EmailClientDashboard { dark_mode, .. } => *dark_mode,
        }
    }

    pub fn in_dark_mode() -> Option<bool> {
        let route: Self = router().current();
        route.dark_mode()
    }

    pub fn home() -> Self {
        let iframe = Self::in_iframe();
        let dark_mode = Self::in_dark_mode();
        Self::Home { iframe, dark_mode }
    }

    pub fn docs() -> Self {
        let dark_mode = Self::in_dark_mode();
        Self::Docs { dark_mode }
    }

    pub fn demos() -> Self {
        let dark_mode = Self::in_dark_mode();
        Self::Demos { dark_mode }
    }

    pub fn component(name: impl ToString) -> Self {
        let iframe = Self::in_iframe();
        let dark_mode = Self::in_dark_mode();
        Self::ComponentDemo {
            name: name.to_string(),
            iframe,
            dark_mode,
        }
    }
}

#[component]
fn AppLayout() -> Element {
    use_effect(move || {
        theme::theme_seed();
        if let Some(dark_mode) = Route::in_dark_mode() {
            theme::set_theme(dark_mode);
        }
    });

    rsx! {
        Outlet::<Route> {}
    }
}

#[component]
fn NavigationLayout() -> Element {
    // Send the route to the parent window if in an iframe
    let mut initial_route = use_hook(|| CopyValue::new(true));
    use_effect(move || {
        let route: Route = router().current();

        // Only send route changes, not the initial route
        if initial_route() || !Route::in_iframe().unwrap_or_default() {
            initial_route.set(false);
            return;
        }

        let eval = document::eval(
            "let route = await dioxus.recv();
            window.top.postMessage({ 'route': route }, '*');",
        );
        let _ = eval.send(route.to_string());
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link {
            rel: "stylesheet",
            href: dioxus_kit::THEME_CSS,
        }
        document::Link { rel: "stylesheet", href: asset!("/assets/hero.css") }
        Navbar {}
        Outlet::<Route> {}
        Footer {}
    }
}

#[component]
fn Navbar() -> Element {
    let in_iframe = Route::in_iframe().unwrap_or_default();
    let in_component = matches!(router().current(), Route::ComponentDemo { .. });
    if in_iframe {
        return rsx! {
            nav {
                class: "dx-preview-navbar",
                aria_label: "Primary",
                border: "none",
                padding: "1rem",
                justify_content: "flex-start",
                if in_component {
                    Link {
                        to: Route::home(),
                        class: "dx-navbar-brand",
                        aria_label: "Back",
                        ChevronLeft {
                            size: "2rem",
                            stroke: "var(--secondary-color-4)",
                        }
                    }
                }
            }
        };
    }

    rsx! {
        nav { class: "dx-preview-navbar", aria_label: "Primary",
            div { class: "dx-navbar-inner",
                div { class: "dx-navbar-primary",
                    Link { to: Route::home(), class: "dx-navbar-brand",
                        img {
                            src: asset!("/assets/dioxus_color.svg"),
                            alt: "Dioxus Logo",
                            width: "18",
                            height: "18",
                        }
                        span { "dioxus-components" }
                    }
                    Link { to: Route::docs(), class: "dx-navbar-link", "Docs" }
                    Link { to: Route::demos(), class: "dx-navbar-link", "Demos" }
                }
                div { class: "dx-navbar-utilities",
                    // TODO: restore once the primitives crate is published
                    // Link {
                    //     to: "https://crates.io/crates/dioxus-components",
                    //     class: "dx-navbar-link",
                    //     aria_label: "Dioxus-Components crates.io",
                    //     Icon {
                    //         width: "24px",
                    //         height: "24px",
                    //         viewBox: ViewBox::new(0, 0, 576, 512),
                    //         path {
                    //             d: "M290.8 48.6l78.4 29.7L288 109.5 206.8 78.3l78.4-29.7c1.8-.7 3.8-.7 5.7 0zM136 92.5l0 112.2c-1.3 .4-2.6 .8-3.9 1.3l-96 36.4C14.4 250.6 0 271.5 0 294.7L0 413.9c0 22.2 13.1 42.3 33.5 51.3l96 42.2c14.4 6.3 30.7 6.3 45.1 0L288 457.5l113.5 49.9c14.4 6.3 30.7 6.3 45.1 0l96-42.2c20.3-8.9 33.5-29.1 33.5-51.3l0-119.1c0-23.3-14.4-44.1-36.1-52.4l-96-36.4c-1.3-.5-2.6-.9-3.9-1.3l0-112.2c0-23.3-14.4-44.1-36.1-52.4l-96-36.4c-12.8-4.8-26.9-4.8-39.7 0l-96 36.4C150.4 48.4 136 69.3 136 92.5zM392 210.6l-82.4 31.2 0-89.2L392 121l0 89.6zM154.8 250.9l78.4 29.7L152 311.7 70.8 280.6l78.4-29.7c1.8-.7 3.8-.7 5.7 0zm18.8 204.4l0-100.5L256 323.2l0 95.9-82.4 36.2zM421.2 250.9c1.8-.7 3.8-.7 5.7 0l78.4 29.7L424 311.7l-81.2-31.1 78.4-29.7zM523.2 421.2l-77.6 34.1 0-100.5L528 323.2l0 90.7c0 3.2-1.9 6-4.8 7.3z",
                    //             fill: "currentColor",
                    //             fill_rule: "nonzero",
                    //         }
                    //     }
                    // }
                    Link {
                        to: "https://github.com/DioxusLabs/components",
                        class: "dx-navbar-link",
                        img {
                            class: "dx-light-mode-only",
                            src: asset!("/assets/github-mark/github-mark.svg"),
                            alt: "GitHub",
                            width: "22",
                            height: "22",
                        }
                        img {
                            class: "dx-dark-mode-only",
                            src: asset!("/assets/github-mark/github-mark-white.svg"),
                            alt: "GitHub",
                            width: "22",
                            height: "22",
                        }
                    }
                    theme::DarkModeToggle {}
                    LanguageSelect {}
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    if Route::in_iframe().unwrap_or_default() {
        return rsx! {};
    }

    rsx! {
        footer { class: "dx-preview-footer",
            div { class: "dx-footer-inner",
                div { class: "dx-footer-brand",
                    Link { to: Route::home(), class: "dx-footer-brand-link",
                        img {
                            src: asset!("/assets/dioxus_color.svg"),
                            alt: "Dioxus Logo",
                            width: "22",
                            height: "22",
                        }
                        span { "Dioxus Components" }
                    }
                    p { class: "dx-footer-tagline",
                        "Accessible, themeable interface pieces for Dioxus apps."
                    }
                }
                nav { class: "dx-footer-nav", aria_label: "Footer",
                    div { class: "dx-footer-nav-group",
                        span { class: "dx-footer-nav-heading", "Library" }
                        Link { to: Route::home(), class: "dx-footer-link", "Components" }
                        Link { to: Route::docs(), class: "dx-footer-link", "Docs" }
                        Link { to: Route::demos(), class: "dx-footer-link", "Demos" }
                    }
                    div { class: "dx-footer-nav-group",
                        span { class: "dx-footer-nav-heading", "Project" }
                        Link {
                            to: "https://github.com/DioxusLabs/dioxus-components",
                            class: "dx-footer-link",
                            "GitHub"
                        }
                        Link {
                            to: "https://dioxuslabs.com",
                            class: "dx-footer-link",
                            "Dioxus"
                        }
                    }
                }
            }
            div { class: "dx-footer-base",
                span { class: "dx-footer-copy", "Built with Dioxus." }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct HighlightedCode {
    pub source: HighlightedSource,
}

#[component]
fn CodeBlock(source: HighlightedCode) -> Element {
    rsx! {
        div {
            class: "dx-code-block",
            tabindex: "0",
            PreviewCode { source: source.source }
        }
        CopyButton { position: "absolute", top: "0.5em", right: "0.5em" }
    }
}

#[component]
fn PreviewCode(source: HighlightedSource) -> Element {
    rsx! {
        div {
            class: "dx-preview-code-theme",
            tabindex: "0",
            Code {
                src: source,
                theme: CodeTheme::system(Theme::GITHUB_LIGHT, Theme::GITHUB_DARK),
            }
        }
    }
}

#[component]
fn CopyButton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            class: "dx-copy-button",
            r#type: "button",
            aria_label: "Copy code",
            "data-copied": copied,
            "onclick": "const visiblePre = Array.from(this.parentNode.querySelectorAll('pre')).find((pre) => pre.offsetParent !== null); navigator.clipboard.writeText(visiblePre ? visiblePre.innerText : Array.from(this.parentNode.childNodes).filter((node) => node !== this).map((node) => node.textContent).join('').trim());",
            onclick: move |_| copied.set(true),
            ..attributes,
            if copied() {
                CheckIcon {}
            } else {
                CopyIcon {}
            }
        }
    }
}

#[component]
fn CopyIcon() -> Element {
    rsx! {
        Copy {
            width: "24px",
            height: "24px",
        }
    }
}

#[component]
fn CheckIcon() -> Element {
    rsx! {
        Check {
            width: "24px",
            height: "24px",
        }
    }
}

#[derive(PartialEq, Display, EnumIter, EnumString)]
enum Language {
    English,
    French,
    Spanish,
    German,
}

impl Language {
    const fn id(&self) -> LanguageIdentifier {
        match self {
            Language::English => langid!("en-US"),
            Language::French => langid!("fr-FR"),
            Language::Spanish => langid!("es-ES"),
            Language::German => langid!("de-DE"),
        }
    }

    const fn flag(&self) -> &'static str {
        match self {
            Language::English => "🇬🇧",
            Language::French => "🇫🇷",
            Language::Spanish => "🇪🇸",
            Language::German => "🇩🇪",
        }
    }

    fn display_name(&self) -> String {
        format!("{} {}", self.flag(), self.localize_name())
    }

    const fn localize_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Spanish => "Español",
            Language::German => "Deutsch",
        }
    }
}

#[component]
fn LanguageSelect() -> Element {
    let mut current_lang = use_signal(|| Language::English);

    rsx! {
        document::Stylesheet { href: asset!("/assets/language-select.css") }
        div { class: "dx-language-container",
            span { class: "dx-language-select-container",
                select {
                    class: "dx-language-select",
                    aria_label: "Language",
                    onchange: move |e| {
                        let name = e.value().parse().unwrap_or(current_lang.to_string());
                        if let Ok(lang) = Language::from_str(&name) {
                            current_lang.set(lang);
                        }
                        let id = current_lang.read().id();
                        tracing::info!("Current lang: {id}");
                        // i18n().set_language(id);
                    },
                    for lang in Language::iter() {
                        option {
                            value: lang.to_string(),
                            selected: lang == *current_lang.read(),
                            {lang.display_name()}
                        }
                    }
                }
                span { class: "dx-language-select-value",
                    {current_lang.read().flag()}
                    ChevronDown {
                        class: "dx-select-expand-icon",
                        size: "24px",
                        stroke: "var(--secondary-color-4)",
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentCode(
    rs_highlighted: HighlightedCode,
    css_highlighted: HighlightedCode,
    #[props(default = ComponentType::Normal)] component_type: ComponentType,
) -> Element {
    rsx! {
        Tabs {
            default_value: "main.rs",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            horizontal: true,
            width: "100%",
            TabList {
                TabTrigger { value: "main.rs", index: 0usize, "main.rs" }
                TabTrigger { value: "style.css", index: 1usize, "style.css" }
                if component_type != ComponentType::Block {
                    TabTrigger { value: "dx-components-theme.css", index: 2usize, "dx-components-theme.css" }
                }
            }
            div {
                width: "100%",
                height: "100%",
                display: "flex",
                flex_direction: "column",
                justify_content: "center",
                align_items: "center",
                TabContent {
                    index: 0usize,
                    padding: 0,
                    value: "main.rs",
                    width: "100%",
                    position: "relative",
                    CodeBlock { source: rs_highlighted }
                }
                TabContent {
                    index: 1usize,
                    padding: 0,
                    value: "style.css",
                    width: "100%",
                    position: "relative",
                    CodeBlock { source: css_highlighted }
                }
                if component_type != ComponentType::Block {
                    TabContent {
                        index: 2usize,
                        padding: 0,
                        value: "dx-components-theme.css",
                        width: "100%",
                        position: "relative",
                        CodeBlock { source: THEME_CSS }
                    }
                }
            }
        }
    }
}

#[component]
fn Docs(dark_mode: Option<bool>) -> Element {
    rsx! {
        main { class: "dx-docs-layout",
            DocsSidebar { active_component: None }
            article { class: "dx-docs-page dx-docs-prose",
                header { class: "dx-docs-page-header",
                    p { class: "dx-docs-eyebrow", "Docs" }
                    h1 { "Build with dioxus-components" }
                    p {
                        "dioxus-components is a collection of styled, accessible Dioxus components designed to be copied into your app. Use the CLI when you want the fastest path, or copy the source when you want complete ownership."
                    }
                }
                section { class: "dx-docs-section",
                    h2 { "How it works" }
                    p {
                        "dioxus-components is not yet on crates.io. For now, components ship from this Git repository — you point your app at the primitives library here, then pull individual styled components into your source tree with the Dioxus CLI."
                    }
                    p {
                        "Start by adding the underlying primitives library to your app's "
                        code { "Cargo.toml" }
                        " from the Git path:"
                    }
                    pre {
                        code { r#"dioxus-primitives = {{ git = "https://github.com/DioxusLabs/components" }}"# }
                    }
                    p {
                        "The styled components live in this same repository as a registry. The "
                        code { "dx components" }
                        " subcommand of the Dioxus CLI is what reads from it. To see everything that's available:"
                    }
                    div { class: "dx-docs-command",
                        code { "dx components list" }
                        CopyCommandButton { command: "dx components list".to_string() }
                    }
                    p {
                        "Then add a specific component to your app — swap "
                        code { "button" }
                        " for any name from the list:"
                    }
                    div { class: "dx-docs-command",
                        code { "dx components add button" }
                        CopyCommandButton { command: "dx components add button".to_string() }
                    }
                    p {
                        "Each "
                        code { "dx components add" }
                        " copies the component's Rust source and its stylesheet directly into your project. Once it's in your tree, the code is yours: keep the included CSS as-is, replace the class names with Tailwind utilities, or rewrite the styles from scratch. There is no runtime dependency on this registry after the copy."
                    }
                }
                section { class: "dx-docs-section",
                    h2 { "Add a component" }
                    p { "Run the add command from your Dioxus app. Swap the final name for any component in the sidebar." }
                    div { class: "dx-docs-command",
                        code { "dx components add button" }
                        CopyCommandButton { command: "dx components add button".to_string() }
                    }
                    p { class: "dx-docs-muted",
                        "If you do not have the Dioxus CLI yet, install it once with cargo install dioxus-cli."
                    }
                }
                section { class: "dx-docs-section",
                    h2 { "Recommended workflow" }
                    ol {
                        li { "Pick a component from the sidebar or catalog." }
                        li { "Preview the default example and variants." }
                        li { "Run the CLI command shown on the component page." }
                        li { "Customize the generated Rust and CSS to fit your app." }
                    }
                }
            }
        }
    }
}

#[component]
fn DocsSidebar(active_component: Option<&'static str>) -> Element {
    let mut open = use_signal(|| false);
    let close = move |_| open.set(false);
    rsx! {
        button {
            class: "dx-docs-sidebar-toggle",
            r#type: "button",
            aria_label: "Open navigation",
            aria_expanded: open(),
            aria_controls: "dx-docs-sidebar-nav",
            onclick: move |_| open.set(true),
            Menu { size: "18" }
            span { "Menu" }
        }
        div {
            class: if open() { "dx-docs-sidebar-backdrop dx-docs-sidebar-backdrop-open" } else { "dx-docs-sidebar-backdrop" },
            aria_hidden: "true",
            onclick: close,
        }
        aside {
            id: "dx-docs-sidebar-nav",
            class: if open() { "dx-docs-sidebar dx-docs-sidebar-open" } else { "dx-docs-sidebar" },
            aria_label: "Docs navigation",
            button {
                class: "dx-docs-sidebar-close",
                r#type: "button",
                aria_label: "Close navigation",
                onclick: close,
                X { size: "18" }
            }
            div { class: "dx-docs-sidebar-scroll",
                nav {
                    aria_label: "Components",
                    onclick: close,
                    div { class: "dx-docs-sidebar-section",
                        p { class: "dx-docs-sidebar-heading", "Start" }
                        Link {
                            to: Route::docs(),
                            class: if active_component.is_none() { "dx-docs-sidebar-link dx-docs-sidebar-link-active" } else { "dx-docs-sidebar-link" },
                            "Overview"
                        }
                    }
                    for cat in components::ComponentCategory::ALL.iter().copied() {
                        div { class: "dx-docs-sidebar-section",
                            p { class: "dx-docs-sidebar-heading", "{cat.label()}" }
                            for component in components::DEMOS.iter().filter(|c| components::category_of(c.name) == cat) {
                                Link {
                                    to: Route::component(component.name),
                                    class: if active_component == Some(component.name) { "dx-docs-sidebar-link dx-docs-sidebar-link-active" } else { "dx-docs-sidebar-link" },
                                    {component.name.replace("_", " ")}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

struct DemoEntry {
    tag: &'static str,
    title: &'static str,
    description: &'static str,
    route: fn() -> Route,
    thumb: fn() -> Element,
}

fn email_client_thumb() -> Element {
    rsx! {
        Mail { size: "56", stroke_width: "1.4" }
    }
}

const DEMO_ENTRIES: &[DemoEntry] = &[DemoEntry {
    tag: "Dashboard",
    title: "Email client",
    description:
        "Multi-pane mail app composed from the sidebar, item list, reading pane, and compose modal.",
    route: || Route::EmailClientDashboard {
        dark_mode: Route::in_dark_mode(),
    },
    thumb: email_client_thumb,
}];

#[component]
fn Demos(dark_mode: Option<bool>) -> Element {
    rsx! {
        main { class: "dx-home-page", role: "main",
            section { class: "dx-home-section",
                header { class: "dx-section-header",
                    span { class: "dx-section-eyebrow", "Demos" }
                    h1 { class: "dx-section-title", "Demo apps" }
                    p { class: "dx-section-summary",
                        "End-to-end app demos assembled from these primitives. Open one to explore the layout and try it live."
                    }
                }
                ul { class: "dx-demos-grid",
                    for entry in DEMO_ENTRIES {
                        li { class: "dx-demos-item",
                            Link {
                                to: (entry.route)(),
                                class: "dx-demos-card",
                                div { class: "dx-demos-card-thumb", {(entry.thumb)()} }
                                div { class: "dx-demos-card-meta",
                                    span { class: "dx-demos-card-tag", "{entry.tag}" }
                                    h2 { class: "dx-demos-card-title", "{entry.title}" }
                                    p { class: "dx-demos-card-description", "{entry.description}" }
                                    span { class: "dx-demos-card-cta",
                                        "Open demo"
                                        ArrowRight { size: "16", stroke_width: "1.6" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentDemo(iframe: Option<bool>, dark_mode: Option<bool>, name: String) -> Element {
    let route = router().current::<Route>();
    tracing::info!("route: {route}");
    let Some(demo) = components::DEMOS
        .iter()
        .find(|demo| demo.name == name)
        .cloned()
    else {
        return rsx! {
            main { class: "dx-component-demo-not-found",
                h3 { "Component not found" }
                p { "The requested component does not exist." }
            }
        };
    };
    rsx! {
        ComponentHighlight { demo }
    }
}

#[component]
fn ComponentHighlight(demo: ComponentDemoData) -> Element {
    let ComponentDemoData {
        name: raw_name,
        r#type,
        docs,
        description,
        variants,
        component,
        style,
    } = demo;
    let name = raw_name.replace("_", " ");
    let [main, variants @ ..] = variants else {
        unreachable!("Expected at least one variant for component: {}", name);
    };

    rsx! {
        main { class: "dx-docs-layout",
            DocsSidebar { active_component: Some(raw_name) }
            article { class: "dx-component-page",
                header { class: "dx-component-page-header",
                    p { class: "dx-docs-eyebrow", "Component" }
                    div { class: "dx-component-page-title-row",
                        h1 { "{name}" }
                        ComponentInstallCommand { name: raw_name }
                    }
                    p { "{description}" }
                }
                section { class: "dx-component-section",
                    match r#type {
                        ComponentType::Normal => rsx! {
                            ComponentVariantHighlight { variant: main.clone(), main_variant: true, component_name: None }
                        },
                        ComponentType::Block => rsx! {
                            BlockComponentVariantHighlight { variant: main.clone(), main_variant: true, component_name: raw_name, show_install: false }
                        },
                    }
                }
                section { class: "dx-component-section",
                    div { class: "dx-component-section-heading",
                        h2 { "Installation" }
                        p { "Use the CLI command for the common path, or copy the component files manually." }
                    }
                    details { class: "dx-component-manual-install dx-component-manual-install-code",
                        summary { "Manual installation files" }
                        ManualComponentInstallation { component, style }
                    }
                }
                section { class: "dx-component-section dx-docs-prose",
                    div { class: "dx-component-section-heading",
                        h2 { "Usage notes" }
                    }
                    div { class: "dx-component-description",
                        div { dangerous_inner_html: docs }
                    }
                }
                if !variants.is_empty() {
                    section { class: "dx-component-section",
                        div { class: "dx-component-section-heading",
                            h2 { "Variants" }
                            p { "Alternative examples for common configurations." }
                        }
                        for variant in variants {
                            div { class: "dx-component-variant",
                                match r#type {
                                    ComponentType::Normal => rsx! {
                                        ComponentVariantHighlight { variant: variant.clone(), main_variant: false, component_name: None }
                                    },
                                    ComponentType::Block => rsx! {
                                        BlockComponentVariantHighlight { variant: variant.clone(), main_variant: false, component_name: raw_name, show_install: false }
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentInstallCommand(name: &'static str) -> Element {
    let command = format!("dx components add {name}");

    rsx! {
        div { class: "dx-component-inline-command",
            code { "{command}" }
            CopyCommandButton { command: command.clone() }
        }
    }
}

#[component]
fn ManualComponentInstallation(component: HighlightedCode, style: HighlightedCode) -> Element {
    rsx! {
        div { class: "dx-component-manual-copy",
            p { class: "dx-docs-muted",
                "Copy the component source and CSS into your app. Import the shared theme CSS once near your app root."
            }
        }
        div { class: "dx-component-manual-code",
            ComponentCode {
                rs_highlighted: component,
                css_highlighted: style,
                component_type: ComponentType::Normal,
            }
        }
    }
}

#[component]
fn ComponentVariantHighlight(
    variant: ComponentVariantDemoData,
    main_variant: bool,
    component_name: Option<&'static str>,
) -> Element {
    let ComponentVariantDemoData {
        name,
        rs_highlighted: highlighted,
        css_highlighted: _,
        component: Comp,
    } = variant;
    rsx! {
        if !main_variant {
            h3 { class: "dx-component-variant-title", "{name}" }
        }
        Tabs {
            default_value: "Demo",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            horizontal: true,
            width: "100%",
            variant: TabsVariant::Ghost,
            div { class: "dx-component-tabs-header",
                TabList {
                    TabTrigger { value: "Demo", index: 0usize, "DEMO" }
                    TabTrigger { value: "Code", index: 1usize, "CODE" }
                }
                if let Some(component_name) = component_name {
                    ComponentInstallCommand { name: component_name }
                }
            }
            div {
                width: "100%",
                height: "100%",
                display: "flex",
                flex_direction: "column",
                justify_content: "center",
                align_items: "center",
                TabContent {
                    index: 0usize,
                    class: "dx-component-preview-frame",
                    id: "component-preview-frame",
                    value: "Demo",
                    width: "100%",
                    position: "relative",
                    Comp {}
                }
                TabContent {
                    index: 1usize,
                    class: "dx-component-preview-frame",
                    value: "Code",
                    width: "100%",
                    position: "relative",
                    CodeBlock { source: highlighted }
                }
            }
        }
    }
}

#[component]
fn BlockComponentVariantHighlight(
    component_name: &'static str,
    variant: ComponentVariantDemoData,
    main_variant: bool,
    show_install: bool,
) -> Element {
    let ComponentVariantDemoData {
        name,
        rs_highlighted: highlighted,
        css_highlighted,
        component: _,
    } = variant;

    let route_path = Route::ComponentBlockDemo {
        name: component_name.to_string(),
        variant: Some(name.to_string()),
        dark_mode: Route::in_dark_mode(),
    }
    .to_string();

    let iframe_src = match router().prefix() {
        Some(prefix) => format!("{prefix}{route_path}"),
        None => route_path,
    };

    rsx! {
        if !main_variant {
            h3 { class: "dx-component-variant-title", "{name}" }
        }
        Tabs {
            default_value: "Preview",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            horizontal: true,
            width: "100%",
            variant: TabsVariant::Ghost,
            div { class: "dx-component-tabs-header",
                TabList {
                    TabTrigger { value: "Preview", index: 0usize, "PREVIEW" }
                    TabTrigger { value: "Code", index: 1usize, "CODE" }
                }
                if show_install {
                    ComponentInstallCommand { name: component_name }
                }
            }
            div {
                width: "100%",
                height: "100%",
                display: "flex",
                flex_direction: "column",
                justify_content: "center",
                align_items: "center",
                TabContent {
                    index: 0usize,
                    id: "component-preview-frame",
                    value: "Preview",
                    width: "100%",
                    position: "relative",
                    iframe {
                        src: "{iframe_src}",
                        width: "100%",
                        height: "600px",
                        border: "1px solid var(--primary-color-6)",
                        border_radius: "0.5em",
                    }
                }
                TabContent {
                    index: 1usize,
                    value: "Code",
                    width: "100%",
                    position: "relative",
                    if let Some(css) = css_highlighted {
                        ComponentCode {
                            rs_highlighted: highlighted,
                            css_highlighted: css,
                            component_type: ComponentType::Block,
                        }
                    } else {
                        CodeBlock { source: highlighted }
                    }
                }
            }
        }
    }
}

#[component]
fn EmailClientDashboard(dark_mode: Option<bool>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: dioxus_kit::THEME_CSS }
        dashboard::views::email_client::EmailClient {}
    }
}

#[component]
fn ComponentBlockDemo(name: String, variant: Option<String>, dark_mode: Option<bool>) -> Element {
    let Some(demo) = components::DEMOS.iter().find(|d| d.name == name).cloned() else {
        return rsx! {
            div { "Block component not found" }
        };
    };

    let variant = match variant.as_deref() {
        Some(wanted) => match demo.variants.iter().find(|v| v.name == wanted) {
            Some(v) => v,
            None => {
                return rsx! {
                    div {
                        style: "min-height: 100vh; display: flex; align-items: center; justify-content: center; padding: 2rem;",
                        "Variant content not found: {wanted}"
                    }
                };
            }
        },
        None => &demo.variants[0],
    };

    let Comp = variant.component;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link {
            rel: "stylesheet",
            href: dioxus_kit::THEME_CSS,
        }
        div { style: "min-height: 100vh;", Comp {} }
    }
}

#[component]
fn Home(iframe: Option<bool>, dark_mode: Option<bool>) -> Element {
    rsx! {
        main { class: "dx-home-page", role: "main",
            div { id: "hero",
                div { class: "dx-hero-shell",
                    h1 { class: "dx-hero-heading",
                        span { class: "dx-hero-title", "dioxus-components" }
                        span { class: "dx-hero-subtitle",
                            "beautiful, accessible, responsive components for dioxus apps"
                        }
                    }
                    p { class: "dx-hero-summary",
                        "Dioxus components by the Dioxus team. Browse the catalog, copy the CLI command, and pull only what you need into your project. Thoughtfully designed with powerful accessibility features."
                    }
                    div { class: "dx-hero-cta",
                        Link { to: Route::docs(), class: "dx-hero-cta-primary",
                            "get started"
                            ArrowRight { size: "18", stroke_width: "1.8" }
                        }
                        div { class: "dx-hero-command",
                            span { class: "dx-hero-prompt", "$" }
                            code { "dx components list" }
                            CopyCommandButton { command: "dx components list".to_string() }
                        }
                    }
                }
            }
            WidgetMasonry {}
            section { class: "dx-home-section dx-catalog-section",
                header { class: "dx-section-header",
                    span { class: "dx-section-eyebrow", "Catalog" }
                    h2 { class: "dx-section-title", "All components" }
                    p { class: "dx-section-summary",
                        "Every primitive in the library, with live previews and a copy-paste install command for each one."
                    }
                }
                ComponentGallery {}
            }
        }
    }
}

struct MasonryEntry {
    component: fn() -> Element,
    popout: bool,
}

const BLOCKS: &[MasonryEntry] = &[
    MasonryEntry {
        component: BlockSignIn,
        popout: false,
    },
    MasonryEntry {
        component: BlockProfile,
        popout: false,
    },
    MasonryEntry {
        component: BlockStats,
        popout: false,
    },
    MasonryEntry {
        component: BlockInbox,
        popout: false,
    },
    MasonryEntry {
        component: BlockTasks,
        popout: false,
    },
    MasonryEntry {
        component: BlockNotifications,
        popout: false,
    },
    MasonryEntry {
        component: BlockPlayer,
        popout: false,
    },
    MasonryEntry {
        component: BlockCommand,
        popout: true,
    },
    MasonryEntry {
        component: BlockComposer,
        popout: false,
    },
    MasonryEntry {
        component: BlockPricing,
        popout: false,
    },
    MasonryEntry {
        component: BlockFilters,
        popout: false,
    },
    MasonryEntry {
        component: BlockColorPalette,
        popout: true,
    },
    MasonryEntry {
        component: BlockTabs,
        popout: false,
    },
    MasonryEntry {
        component: BlockSchedule,
        popout: false,
    },
];

#[component]
fn WidgetMasonry() -> Element {
    rsx! {
        section { class: "dx-home-section dx-masonry-section",
            header { class: "dx-section-header",
                span { class: "dx-section-eyebrow", "Showcase" }
                h2 { class: "dx-section-title", "Sample interfaces" }
                p { class: "dx-section-summary",
                    "Live, interactive UI blocks composed from the primitives below. Use your keyboard to test the accessibility interactions."
                }
            }
            div { class: "dx-widget-masonry",
                for entry in BLOCKS {
                    MasonryCard { component: entry.component, popout: entry.popout }
                }
            }
        }
    }
}

#[allow(unpredictable_function_pointer_comparisons)]
#[component]
fn MasonryCard(component: fn() -> Element, #[props(default)] popout: bool) -> Element {
    let Comp = component;
    let class = if popout {
        "dx-widget-card dx-widget-card-popout"
    } else {
        "dx-widget-card"
    };
    rsx! {
        div { class,
            Comp {}
        }
    }
}

#[component]
fn BlockSignIn() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.3rem; margin-bottom: 1.1rem;",
            h3 { style: "margin: 0; font-size: 1.05rem; font-weight: 660; color: var(--secondary-color-3);", "Welcome back" }
            p { style: "margin: 0; color: var(--secondary-color-5); font-size: 0.85rem;", "Sign in to your workspace." }
        }
        div { style: "display: grid; gap: 0.75rem; margin-bottom: 1rem;",
            div { style: "display: grid; gap: 0.35rem;",
                Label { html_for: "blk-signin-email", "Email" }
                Input { id: "blk-signin-email", r#type: "email", placeholder: "you@example.com" }
            }
            div { style: "display: grid; gap: 0.35rem;",
                div { style: "display: flex; align-items: center;",
                    Label { html_for: "blk-signin-pw", "Password" }
                    span { style: "margin-left: auto; font-size: 0.78rem; color: var(--secondary-color-5); text-decoration: underline; text-underline-offset: 3px;",
                        "Forgot?"
                    }
                }
                Input { id: "blk-signin-pw", r#type: "password", placeholder: "••••••••" }
            }
        }
        div { style: "display: grid; gap: 0.5rem;",
            Button { style: "width: 100%;", "Sign in" }
            Button { variant: ButtonVariant::Outline, style: "width: 100%;", "Continue with Google" }
        }
    }
}

#[component]
fn BlockProfile() -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; gap: 0.75rem;",
            ImageAvatar {
                size: AvatarImageSize::Medium,
                src: "https://avatar.vercel.sh/avery-lin",
                alt: "Avery Lin",
                aria_label: "Avatar",
                "AL"
            }
            div { style: "flex: 1; display: grid; gap: 0.1rem; min-width: 0;",
                div { style: "display: flex; align-items: center; gap: 0.4rem;",
                    span { style: "font-weight: 600; color: var(--secondary-color-3);", "Avery Lin" }
                    Badge {
                        variant: BadgeVariant::Secondary,
                        style: "padding: 0.15rem 0.3rem; background-color: var(--focused-border-color); color: white;",
                        VerifiedIcon {}
                    }
                }
                span { style: "color: var(--secondary-color-5); font-size: 0.85rem;", "@averylin" }
            }
            Button { variant: ButtonVariant::Outline, "Follow" }
        }
        p { style: "margin: 1.1rem 0 0; color: var(--secondary-color-5); font-size: 0.9rem; line-height: 1.55;",
            "Building UI primitives that ship to web, desktop, and mobile. Mostly Rust, mostly weekends."
        }
        div { style: "display: flex; gap: 0.35rem; margin-top: 0.85rem; flex-wrap: wrap;",
            Badge { variant: BadgeVariant::Outline, "Rust" }
            Badge { variant: BadgeVariant::Outline, "WebAssembly" }
            Badge { variant: BadgeVariant::Outline, "UI" }
        }
    }
}

#[component]
fn BlockStats() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.45rem;",
            p { style: "margin: 0; color: var(--secondary-color-5); font-size: 0.74rem; text-transform: uppercase; letter-spacing: 0.1em; font-weight: 600;",
                "Active users · 30d"
            }
            div { style: "display: flex; align-items: baseline; gap: 0.6rem;",
                span { style: "font-size: 2rem; font-weight: 720; color: var(--secondary-color-3); line-height: 1.1;",
                    "24,815"
                }
                Badge {
                    variant: BadgeVariant::Secondary,
                    style: "background-color: rgba(34, 197, 94, 0.18); color: rgb(21, 128, 61);",
                    "+12.4%"
                }
            }
        }
        div { style: "margin-top: 1rem;",
            Progress {
                value: 68.0,
                aria_label: "Toward Q2 target",
                style: "width: 100%;",
            }
        }
        p { style: "margin: 0.65rem 0 0; color: var(--secondary-color-5); font-size: 0.82rem;",
            "On track for the 36k Q2 target."
        }
    }
}

#[component]
fn BlockNotifications() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.3rem; margin-bottom: 1rem;",
            h3 { style: "margin: 0; font-size: 1rem; font-weight: 660; color: var(--secondary-color-3);", "Notifications" }
            p { style: "margin: 0; color: var(--secondary-color-5); font-size: 0.85rem;", "Pick what we ping you about." }
        }
        div { style: "display: grid; gap: 0.95rem;",
            NotificationRow { id: "blk-notif-comments", name: "Comments", description: "Replies on your posts", default_on: true }
            NotificationRow { id: "blk-notif-mentions", name: "Mentions", description: "When someone @'s you", default_on: true }
            NotificationRow { id: "blk-notif-weekly", name: "Weekly digest", description: "A Monday morning recap", default_on: false }
            NotificationRow { id: "blk-notif-updates", name: "Product updates", description: "New features and releases", default_on: false }
        }
    }
}

#[component]
fn NotificationRow(id: String, name: String, description: String, default_on: bool) -> Element {
    let mut checked = use_signal(|| default_on);
    rsx! {
        div { style: "display: flex; align-items: center; gap: 0.75rem;",
            div { style: "flex: 1; display: grid; gap: 0.1rem; min-width: 0;",
                span { style: "font-weight: 540; font-size: 0.92rem; color: var(--secondary-color-3);", "{name}" }
                span { style: "color: var(--secondary-color-5); font-size: 0.8rem;", "{description}" }
            }
            Switch {
                id: "{id}",
                checked: checked(),
                aria_label: "{name}",
                on_checked_change: move |v| checked.set(v),
            }
        }
    }
}

#[component]
fn BlockPlayer() -> Element {
    const TRACK_DURATION_SECONDS: f64 = 212.0;
    const TRACK_START_SECONDS: f64 = 84.0;

    let mut playing = use_signal(|| true);
    let mut progress_seconds = use_signal(|| Some(TRACK_START_SECONDS));
    let current_time = use_memo(move || format_track_time(progress_seconds().unwrap_or(0.0)));
    let duration_time = format_track_time(TRACK_DURATION_SECONDS);

    use_effect(move || {
        let mut timer = document::eval(
            "setInterval(() => {
                dioxus.send(performance.now());
            }, 100);",
        );

        spawn(async move {
            let mut last_tick_ms: Option<f64> = None;

            while let Ok(now_ms) = timer.recv::<f64>().await {
                let elapsed_seconds = last_tick_ms
                    .map(|last_ms| ((now_ms - last_ms) / 1000.0).clamp(0.0, 0.25))
                    .unwrap_or(0.0);
                last_tick_ms = Some(now_ms);

                if !playing() {
                    continue;
                }

                let current = progress_seconds().unwrap_or(0.0);
                let next = if current >= TRACK_DURATION_SECONDS {
                    0.0
                } else {
                    (current + elapsed_seconds).min(TRACK_DURATION_SECONDS)
                };
                progress_seconds.set(Some(next));
            }
        });
    });

    rsx! {
        div { style: "display: flex; gap: 0.85rem; align-items: center;",
            img {
                src: "https://avatar.vercel.sh/midnight-city",
                alt: "Midnight City album art",
                width: "64",
                height: "64",
                style: "width: 64px; height: 64px; border-radius: 0.45rem; object-fit: cover; flex-shrink: 0; box-shadow: 0 6px 18px -8px rgba(0,0,0,0.35);",
            }
            div { style: "flex: 1; min-width: 0;",
                p { style: "margin: 0; font-weight: 600; color: var(--secondary-color-3); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                    "Midnight City"
                }
                p { style: "margin: 0.15rem 0 0; color: var(--secondary-color-5); font-size: 0.85rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                    "M83 · Hurry Up, We're Dreaming"
                }
            }
        }
        div { style: "margin-top: 1.1rem;",
            Slider {
                horizontal: true,
                min: 0.0,
                max: TRACK_DURATION_SECONDS,
                step: 1.0,
                value: progress_seconds,
                on_value_change: move |value| progress_seconds.set(Some(value)),
                label: "Track progress",
            }
            div { style: "display: flex; justify-content: space-between; margin-top: 0.45rem; color: var(--secondary-color-5); font-size: 0.78rem;",
                span { "{current_time}" }
                span { "{duration_time}" }
            }
        }
        div { style: "display: flex; align-items: center; justify-content: center; gap: 0.5rem; margin-top: 0.6rem;",
            Button {
                variant: ButtonVariant::Ghost,
                aria_label: "Previous",
                onclick: move |_| progress_seconds.set(Some(0.0)),
                SkipBack { size: "18", fill: "currentColor", stroke_width: "1.5" }
            }
            Button {
                aria_label: "Play or pause",
                onclick: move |_| { let v = !playing(); playing.set(v); },
                if playing() {
                    Pause { size: "18", fill: "currentColor", stroke_width: "1.5" }
                } else {
                    Play { size: "18", fill: "currentColor", stroke_width: "1.5" }
                }
            }
            Button {
                variant: ButtonVariant::Ghost,
                aria_label: "Next",
                onclick: move |_| progress_seconds.set(Some(0.0)),
                SkipForward { size: "18", fill: "currentColor", stroke_width: "1.5" }
            }
        }
    }
}

fn format_track_time(seconds: f64) -> String {
    let seconds = if seconds.is_finite() { seconds } else { 0.0 };
    let seconds = seconds.max(0.0).floor() as u64;
    format!("{}:{:02}", seconds / 60, seconds % 60)
}

#[component]
fn BlockPricing() -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; gap: 0.5rem; margin-bottom: 0.6rem;",
            h3 { style: "margin: 0; font-size: 1rem; font-weight: 660; color: var(--secondary-color-3);", "Team" }
            Badge { variant: BadgeVariant::Secondary, "Most popular" }
        }
        div { style: "display: flex; align-items: baseline; gap: 0.3rem; margin-bottom: 0.55rem;",
            span { style: "font-size: 2.4rem; font-weight: 720; color: var(--secondary-color-3); line-height: 1;", "$12" }
            span { style: "color: var(--secondary-color-5);", "/ seat / mo" }
        }
        p { style: "margin: 0 0 1rem; color: var(--secondary-color-5); font-size: 0.86rem; line-height: 1.55;",
            "Everything in Pro, plus shared workspaces and audit logs."
        }
        ul { style: "list-style: none; padding: 0; margin: 0 0 1rem; display: grid; gap: 0.55rem; color: var(--secondary-color-4); font-size: 0.88rem;",
            for feature in ["Unlimited projects", "Role-based access", "SSO + SAML", "Priority support"] {
                li { style: "display: flex; align-items: center; gap: 0.55rem;",
                    svg {
                        width: "16",
                        height: "16",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "var(--highlight-color-tertiary)",
                        stroke_width: "2.5",
                        "aria-hidden": "true",
                        polyline { points: "20 6 9 17 4 12" }
                    }
                    "{feature}"
                }
            }
        }
        Button { style: "width: 100%;", "Start free trial" }
    }
}

#[component]
fn BlockFilters() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.3rem; margin-bottom: 1rem;",
            h3 { style: "margin: 0; font-size: 1rem; font-weight: 660; color: var(--secondary-color-3);", "Filter results" }
            p { style: "margin: 0; color: var(--secondary-color-5); font-size: 0.85rem;", "Narrow down what's shown below." }
        }
        div { style: "display: grid; gap: 1.1rem;",
            div { style: "display: grid; gap: 0.45rem;",
                span { style: "color: var(--secondary-color-5); font-size: 0.78rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.08em;",
                    "Status"
                }
                RadioGroup { default_value: "active".to_string(),
                    RadioItem { value: "active".to_string(), index: 0usize, "Active" }
                    RadioItem { value: "draft".to_string(), index: 1usize, "Drafts" }
                    RadioItem { value: "archived".to_string(), index: 2usize, "Archived" }
                }
            }
            div { style: "display: grid; gap: 0.45rem;",
                span { style: "color: var(--secondary-color-5); font-size: 0.78rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.08em;",
                    "Tags"
                }
                div { style: "display: grid; gap: 0.4rem;",
                    for tag in [("ft-design", "Design", true), ("ft-eng", "Engineering", false), ("ft-research", "Research", false)] {
                        div { style: "display: flex; align-items: center; gap: 0.55rem;",
                            Checkbox {
                                id: tag.0,
                                name: tag.0,
                                default_checked: if tag.2 { dioxus_kit_core::checkbox::CheckboxState::Checked } else { dioxus_kit_core::checkbox::CheckboxState::Unchecked },
                                aria_label: tag.1,
                            }
                            Label { html_for: tag.0, "{tag.1}" }
                        }
                    }
                }
            }
            Button { style: "width: 100%; margin-top: 0.2rem;", "Apply filters" }
        }
    }
}

#[component]
fn BlockColorPalette() -> Element {
    use dioxus_kit_core::color_picker::Color;
    use palette::{encoding, Hsv, IntoColor};

    let mut color = use_signal(|| -> Hsv<encoding::Srgb, f64> {
        Color::new(124, 58, 237).into_format::<f64>().into_color()
    });

    rsx! {
        div { style: "display: grid; gap: 0.3rem; margin-bottom: 1.1rem;",
            h3 { style: "margin: 0; font-size: 1rem; font-weight: 660; color: var(--secondary-color-3);", "Theme accent" }
            p { style: "margin: 0; color: var(--secondary-color-5); font-size: 0.85rem;", "Tune the accent that shows up across the workspace." }
        }
        ColorPicker {
            label: "Theme accent color",
            color: color(),
            on_color_change: move |c| color.set(c),
        }
    }
}

#[component]
fn BlockTabs() -> Element {
    let members: &[(&str, &str, &str, &str)] = &[
        ("Avery Lin", "Eng lead", "online", "AL"),
        ("Casey Park", "Design", "away", "CP"),
        ("Robin Hayes", "PM", "offline", "RH"),
    ];
    let activity: &[(&str, &str, &str)] = &[
        ("Casey", "shipped v2.4.1", "12m ago"),
        ("Avery", "opened PR #482", "1h ago"),
        ("Robin", "moved 4 tasks", "3h ago"),
    ];
    rsx! {
        div { style: "display: grid; gap: 0.3rem; margin-bottom: 1.1rem;",
            h3 { style: "margin: 0; font-size: 1rem; font-weight: 660; color: var(--secondary-color-3);", "Workspace" }
            p { style: "margin: 0; color: var(--secondary-color-5); font-size: 0.85rem;", "Team activity at a glance." }
        }
        Tabs {
            default_value: "members".to_string(),
            horizontal: true,
            width: "100%",
            TabList {
                TabTrigger { value: "members".to_string(), index: 0usize, "Members" }
                TabTrigger { value: "activity".to_string(), index: 1usize, "Activity" }
                TabTrigger { value: "files".to_string(), index: 2usize, "Files" }
            }
            TabContent { index: 0usize, value: "members".to_string(),
                div { style: "display: grid; gap: 0.85rem;",
                    for member in members.iter() {
                        div { style: "display: flex; align-items: center; gap: 0.7rem;",
                            ImageAvatar {
                                size: AvatarImageSize::Small,
                                src: "https://avatar.vercel.sh/{member.0}",
                                alt: "{member.0}",
                                aria_label: "{member.0}",
                                "{member.3}"
                            }
                            div { style: "flex: 1; min-width: 0;",
                                div { style: "font-weight: 540; color: var(--secondary-color-3); font-size: 0.9rem;", "{member.0}" }
                                div { style: "color: var(--secondary-color-5); font-size: 0.78rem;", "{member.1}" }
                            }
                            span {
                                style: match member.2 {
                                    "online" => "width: 0.55rem; height: 0.55rem; border-radius: 999px; background-color: rgb(34,197,94);",
                                    "away" => "width: 0.55rem; height: 0.55rem; border-radius: 999px; background-color: rgb(234,179,8);",
                                    _ => "width: 0.55rem; height: 0.55rem; border-radius: 999px; background-color: var(--primary-color-6);",
                                },
                            }
                        }
                    }
                }
            }
            TabContent { index: 1usize, value: "activity".to_string(),
                div { style: "display: grid; gap: 0.85rem;",
                    for entry in activity.iter() {
                        div { style: "display: flex; align-items: baseline; gap: 0.45rem; font-size: 0.88rem;",
                            span { style: "font-weight: 600; color: var(--secondary-color-3);", "{entry.0}" }
                            span { style: "color: var(--secondary-color-5);", "{entry.1}" }
                            span { style: "margin-left: auto; color: var(--secondary-color-5); font-size: 0.78rem; white-space: nowrap;", "{entry.2}" }
                        }
                    }
                }
            }
            TabContent { index: 2usize, value: "files".to_string(),
                div { style: "display: grid; gap: 0.6rem; color: var(--secondary-color-4); font-size: 0.88rem;",
                    div { style: "display: flex; align-items: center; gap: 0.5rem;",
                        span { style: "font-family: monospace; color: var(--secondary-color-5);", "/" }
                        span { "Roadmap Q2.md" }
                        Badge { variant: BadgeVariant::Outline, style: "margin-left: auto;", "Draft" }
                    }
                    div { style: "display: flex; align-items: center; gap: 0.5rem;",
                        span { style: "font-family: monospace; color: var(--secondary-color-5);", "/" }
                        span { "Brand guidelines.pdf" }
                    }
                    div { style: "display: flex; align-items: center; gap: 0.5rem;",
                        span { style: "font-family: monospace; color: var(--secondary-color-5);", "/" }
                        span { "Onboarding deck.key" }
                    }
                }
            }
        }
    }
}

#[component]
fn BlockSchedule() -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; gap: 0.6rem; margin-bottom: 0.85rem;",
            div { style: "flex: 1;",
                h3 { style: "margin: 0; font-size: 1rem; font-weight: 660; color: var(--secondary-color-3);", "Schedule" }
                p { style: "margin: 0; color: var(--secondary-color-5); font-size: 0.85rem;", "Pick a day for the standup." }
            }
            Badge { variant: BadgeVariant::Outline, "Mar 2026" }
        }
        div { style: "display: grid; justify-items: center;",
            components::calendar::variants::main::Demo {}
        }
    }
}

#[component]
fn BlockCommand() -> Element {
    let mut query = use_signal(String::new);
    let workspaces: &[(&str, &str)] = &[
        ("acme", "Acme Inc."),
        ("orbit", "Orbit Studio"),
        ("nimbus", "Nimbus Labs"),
        ("strata", "Strata Health"),
        ("vela", "Vela Robotics"),
        ("riverstone", "Riverstone Capital"),
    ];
    rsx! {
        div { style: "display: grid; gap: 0.3rem; margin-bottom: 1rem;",
            h3 { style: "margin: 0; font-size: 1rem; font-weight: 660; color: var(--secondary-color-3);", "Switch workspace" }
            p { style: "margin: 0; color: var(--secondary-color-5); font-size: 0.85rem;", "Jump between projects your team owns." }
        }
        Combobox::<String> {
            query: Some(query()),
            on_query_change: move |next| query.set(next),
            placeholder: "Search workspaces...",
            aria_label: "Switch workspace",
            list_aria_label: "Workspaces",
            ComboboxEmpty { "No workspaces match." }
            for (i , (value , label)) in workspaces.iter().enumerate() {
                ComboboxOption::<String> {
                    index: i,
                    value: value.to_string(),
                    text_value: label.to_string(),
                    "{label}"
                }
            }
        }
    }
}

#[component]
fn BlockInbox() -> Element {
    let messages: &[(&str, &str, &str)] = &[
        ("Sarah Chen", "Left 3 comments on the auth flow", "2m"),
        ("Marcus Wright", "Roadmap sync notes attached", "1h"),
        ("Lena Park", "Refactored the sidebar layout", "4h"),
    ];
    rsx! {
        div { style: "display: flex; align-items: center; gap: 0.55rem; margin-bottom: 0.85rem;",
            div { style: "flex: 1;",
                h3 { style: "margin: 0; font-size: 1rem; font-weight: 660; color: var(--secondary-color-3);", "Inbox" }
                p { style: "margin: 0; color: var(--secondary-color-5); font-size: 0.85rem;", "3 new conversations." }
            }
            Badge { variant: BadgeVariant::Secondary, "3" }
        }
        div { style: "display: grid; gap: 0.5rem;",
            for (sender , preview , time) in messages.iter() {
                Item { variant: ItemVariant::Outline,
                    ItemMedia { variant: ItemMediaVariant::Icon,
                        ImageAvatar {
                            size: AvatarImageSize::Small,
                            src: "https://avatar.vercel.sh/{sender}",
                            alt: "{sender}",
                            aria_label: "{sender}",
                            "{sender.chars().next().unwrap_or('?')}"
                        }
                    }
                    ItemContent {
                        ItemTitle { "{sender}" }
                        ItemDescription { "{preview}" }
                    }
                    ItemContent { flex: "none",
                        ItemDescription { "{time}" }
                    }
                }
            }
        }
    }
}

#[component]
fn BlockTasks() -> Element {
    let tasks: &[(&str, &str, &str, &str)] = &[
        ("LNC-128", "Ship Q2 product roadmap", "Today", "AL"),
        ("LNC-142", "Redesign onboarding flow", "Apr 24", "CP"),
        ("LNC-147", "Audit payment webhook logs", "Apr 29", "RH"),
        ("LNC-151", "Draft changelog for v2.4", "May 02", "AL"),
    ];
    let items: Vec<Element> = tasks
        .iter()
        .map(|t| {
            rsx! {
                div { key: "{t.0}", style: "display: flex; align-items: center; gap: 0.75rem; min-width: 0;",
                    div { style: "flex: 1; min-width: 0; display: grid; gap: 0.2rem;",
                        div { style: "color: var(--secondary-color-3); font-size: 0.9rem; font-weight: 540; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                            "{t.1}"
                        }
                        div { style: "display: flex; align-items: center; gap: 0.45rem; color: var(--secondary-color-5); font-size: 0.78rem;",
                            span { style: "font-family: monospace;", "{t.0}" }
                            span { style: "width: 3px; height: 3px; border-radius: 999px; background-color: var(--primary-color-7);" }
                            span { "{t.2}" }
                        }
                    }
                    ImageAvatar {
                        size: AvatarImageSize::Small,
                        src: "https://avatar.vercel.sh/{t.3}",
                        alt: "{t.3}",
                        aria_label: "Assignee {t.3}",
                        "{t.3}"
                    }
                }
            }
        })
        .collect();

    rsx! {
        div { style: "display: flex; align-items: center; gap: 0.55rem; margin-bottom: 1.1rem;",
            div { style: "flex: 1;",
                h3 { style: "margin: 0; font-size: 1rem; font-weight: 660; color: var(--secondary-color-3);", "Launch priorities" }
                p { style: "margin: 0; color: var(--secondary-color-5); font-size: 0.85rem;", "Drag to reorder — top is highest priority." }
            }
            Badge { variant: BadgeVariant::Outline, "4 active" }
        }
        DragAndDropList { items }
    }
}

#[component]
fn BlockComposer() -> Element {
    let mut draft = use_signal(|| {
        "Big thanks to the team for landing the new roadmap view — looks great!".to_string()
    });
    rsx! {
        div { style: "display: flex; align-items: center; gap: 0.65rem; margin-bottom: 1rem;",
            ImageAvatar {
                size: AvatarImageSize::Small,
                src: "https://avatar.vercel.sh/avery-lin",
                alt: "Avery Lin",
                aria_label: "Avery Lin",
                "AL"
            }
            div { style: "flex: 1; display: grid; gap: 0.1rem;",
                span { style: "font-weight: 600; color: var(--secondary-color-3); font-size: 0.9rem;", "Reply to roadmap thread" }
                span { style: "color: var(--secondary-color-5); font-size: 0.78rem;", "Posting as @averylin · #product" }
            }
        }
        Textarea {
            variant: TextareaVariant::Default,
            value: draft,
            oninput: move |e: FormEvent| draft.set(e.value()),
            placeholder: "Share an update…",
            style: "width: 100%; min-height: 5.5rem; resize: vertical;",
        }
        div { style: "display: flex; align-items: center; gap: 0.55rem; margin-top: 0.85rem;",
            ToggleGroup { horizontal: true, allow_multiple_pressed: true, aria_label: "Text formatting",
                ToggleItem { index: 0usize, aria_label: "Bold",
                    b { "B" }
                }
                ToggleItem { index: 1usize, aria_label: "Italic",
                    i { "I" }
                }
                ToggleItem { index: 2usize, aria_label: "Underline",
                    u { "U" }
                }
            }
            div { style: "margin-left: auto; display: flex; gap: 0.45rem;",
                Button { variant: ButtonVariant::Ghost, "Save draft" }
                Button { "Post" }
            }
        }
    }
}

#[component]
fn ComponentGallery() -> Element {
    rsx! {
        div { class: "dx-component-gallery",
            for component in components::DEMOS.iter().cloned() {
                ComponentGalleryPreview { component }
            }
        }
    }
}

#[component]
fn ComponentGalleryPreview(component: ComponentDemoData) -> Element {
    let ComponentDemoData {
        name,
        r#type,
        description,
        variants,
        ..
    } = component;

    let first_variant = &variants[0];
    let Comp = first_variant.component;
    let display_name = name.replace("_", " ");
    let install_command = format!("dx components add {name}");

    let preview = match r#type {
        ComponentType::Normal => rsx! {
            Comp {}
        },
        ComponentType::Block => rsx! {
            Link {
                to: Route::component(name),
                class: "dx-component-card-block-link",
                "Open full preview"
                ArrowUpRight { size: "18", stroke_width: "1.6" }
            }
        },
    };

    rsx! {
        article { class: "dx-component-card",
            div { class: "dx-component-card-meta",
                h3 { class: "dx-component-card-title",
                    Link {
                        to: Route::component(name),
                        class: "dx-component-card-title-link",
                        "{display_name}"
                        ArrowUpRight { size: "18", stroke_width: "1.6" }
                    }
                }
                p { class: "dx-component-card-description", "{description}" }
                div { class: "dx-component-card-actions",
                    div { class: "dx-component-card-command",
                        code { "{install_command}" }
                        CopyCommandButton { command: install_command.clone() }
                    }
                }
            }
            div { class: "dx-component-card-preview", {preview} }
        }
    }
}

#[component]
fn CopyCommandButton(command: String) -> Element {
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            class: "dx-copy-button dx-component-card-copy",
            r#type: "button",
            aria_label: "Copy install command",
            "data-command": "{command}",
            "data-copied": copied,
            "onclick": "navigator.clipboard.writeText(this.dataset.command);",
            onclick: move |_| copied.set(true),
            if copied() {
                CheckIcon {}
            } else {
                CopyIcon {}
            }
        }
    }
}

#[component]
fn GotoIcon(mut props: LinkProps) -> Element {
    props.children = rsx! {
        ExternalLink {
            size: "20px",
            stroke: "var(--secondary-color-4)",
        }
    };
    Link(props)
}

const THEME_CSS: HighlightedCode = HighlightedCode {
    source: dioxus_code::code!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../dioxus-kit/assets/dx-components-theme.css"
    )),
};
