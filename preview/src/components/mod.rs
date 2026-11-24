use super::{ComponentDemoData, ComponentType, ComponentVariantDemoData, HighlightedCode};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ComponentCategory {
    Forms,
    Navigation,
    Overlays,
    Feedback,
    Disclosure,
    DataDisplay,
}

impl ComponentCategory {
    pub const ALL: &'static [Self] = &[
        Self::Forms,
        Self::Navigation,
        Self::Overlays,
        Self::Feedback,
        Self::Disclosure,
        Self::DataDisplay,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Forms => "Forms",
            Self::Navigation => "Navigation",
            Self::Overlays => "Overlays",
            Self::Feedback => "Feedback",
            Self::Disclosure => "Disclosure",
            Self::DataDisplay => "Data display",
        }
    }
}

pub fn category_of(name: &str) -> ComponentCategory {
    match name {
        "button" | "input" | "textarea" | "label" | "checkbox" | "switch" | "radio_group"
        | "toggle" | "toggle_group" | "select" | "slider" | "calendar" | "date_picker"
        | "color_picker" => ComponentCategory::Forms,
        "navbar" | "sidebar" | "tabs" | "pagination" | "menubar" | "toolbar" | "context_menu"
        | "dropdown_menu" => ComponentCategory::Navigation,
        "dialog" | "alert_dialog" | "sheet" | "popover" | "tooltip" | "hover_card" => {
            ComponentCategory::Overlays
        }
        "toast" | "progress" | "skeleton" | "badge" => ComponentCategory::Feedback,
        "accordion" | "collapsible" => ComponentCategory::Disclosure,
        "avatar" | "card" | "separator" | "aspect_ratio" | "item" | "drag_and_drop_list"
        | "virtual_list" | "scroll_area" | "tag_group" => ComponentCategory::DataDisplay,
        _ => ComponentCategory::DataDisplay,
    }
}

macro_rules! examples {
    ($($name:ident $(($kind:ident))? $([$($variant:ident),*])?),* $(,)?) => {
        $(
            pub(crate) mod $name {
                pub(crate) mod component;
                #[allow(unused)]
                pub use component::*;
                pub(crate) mod variants {
                    pub(crate) mod main;
                    $(
                        $(
                            pub(crate) mod $variant;
                        )*
                    )?
                }
            }
        )*
        pub(crate) static DEMOS: &[ComponentDemoData] = &[
            $(
                examples!(@demo $name $( $kind )? $([$($variant),*])?),
            )*
        ];
    };

    (@kind) => { ComponentType::Normal };
    (@kind normal) => { ComponentType::Normal };
    (@kind block) => { ComponentType::Block };

    // Normal components: no variant-level css_highlighted
    (@demo $name:ident $([$($variant:ident),*])?) => {
        ComponentDemoData {
            name: stringify!($name),
            r#type: ComponentType::Normal,
            description: include_str!(concat!(
                env!("OUT_DIR"),
                "/",
                stringify!($name),
                "/description.txt"
            )),
            docs: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/docs.html")),
            component: HighlightedCode {
                source: dioxus_code::code!(concat!("/src/components/", stringify!($name), "/component.rs")),
            },
            style: HighlightedCode {
                source: dioxus_code::code!(concat!("/src/components/", stringify!($name), "/style.css")),
            },
            variants: &[
                ComponentVariantDemoData {
                    name: "main",
                    rs_highlighted: HighlightedCode {
                        source: dioxus_code::code!(concat!("/src/components/", stringify!($name), "/variants/main/mod.rs")),
                    },
                    css_highlighted: None,
                    component: $name::variants::main::Demo,
                },
                $(
                    $(
                        ComponentVariantDemoData {
                            name: stringify!($variant),
                            rs_highlighted: HighlightedCode {
                                source: dioxus_code::code!(concat!("/src/components/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs")),
                            },
                            css_highlighted: None,
                            component: $name::variants::$variant::Demo,
                        },
                    )*
                )?
            ],
        }
    };

    // Block components: rendered in iframe, with shared demo.css
    (@demo $name:ident block $([$($variant:ident),*])?) => {
        ComponentDemoData {
            name: stringify!($name),
            r#type: ComponentType::Block,
            description: include_str!(concat!(
                env!("OUT_DIR"),
                "/",
                stringify!($name),
                "/description.txt"
            )),
            docs: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/docs.html")),
            component: HighlightedCode {
                source: dioxus_code::code!(concat!("/src/components/", stringify!($name), "/component.rs")),
            },
            style: HighlightedCode {
                source: dioxus_code::code!(concat!("/src/components/", stringify!($name), "/style.css")),
            },
            variants: &[
                ComponentVariantDemoData {
                    name: "main",
                    rs_highlighted: HighlightedCode {
                        source: dioxus_code::code!(concat!("/src/components/", stringify!($name), "/variants/main/mod.rs")),
                    },
                    css_highlighted: Some(HighlightedCode {
                        source: dioxus_code::code!(concat!("/src/components/", stringify!($name), "/variants/demo.css")),
                    }),
                    component: $name::variants::main::Demo,
                },
                $(
                    $(
                        ComponentVariantDemoData {
                            name: stringify!($variant),
                            rs_highlighted: HighlightedCode {
                                source: dioxus_code::code!(concat!("/src/components/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs")),
                            },
                            css_highlighted: Some(HighlightedCode {
                                source: dioxus_code::code!(concat!("/src/components/", stringify!($name), "/variants/demo.css")),
                            }),
                            component: $name::variants::$variant::Demo,
                        },
                    )*
                )?
            ],
        }
    };
}

examples!(
    accordion,
    alert_dialog,
    aspect_ratio,
    avatar,
    badge,
    button[size, icon],
    calendar[simple, internationalized, range, multi_month, unavailable_dates],
    card,
    checkbox,
    collapsible,
    color_picker,
    combobox[controlled, disabled, dynamic],
    context_menu,
    date_picker[internationalized, range, multi_month, unavailable_dates],
    dialog,
    drag_and_drop_list[removable],
    dropdown_menu,
    hover_card,
    input,
    item[variant, size, image, group],
    label,
    menubar,
    navbar,
    pagination,
    popover,
    progress,
    radio_group,
    scroll_area,
    select[multi],
    separator,
    sheet,
    sidebar(block)[floating, inset],
    skeleton,
    slider[dynamic_range, range],
    switch,
    tabs,
    tag_group[multi, states],
    textarea[outline, fade, ghost],
    toast,
    toggle,
    toggle_group,
    toolbar,
    tooltip,
    virtual_list[random_heights],
);
