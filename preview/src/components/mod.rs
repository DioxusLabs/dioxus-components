use super::{ComponentDemoData, ComponentType, ComponentVariantDemoData, HighlightedCode};

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
            docs: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/docs.html")),
            component: HighlightedCode {
                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.base16-ocean.light.html")),
                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.base16-ocean.dark.html")),
            },
            style: HighlightedCode {
                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.light.html")),
                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.dark.html")),
            },
            variants: &[
                ComponentVariantDemoData {
                    name: "main",
                    rs_highlighted: HighlightedCode {
                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.base16-ocean.light.html")),
                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.base16-ocean.dark.html")),
                    },
                    css_highlighted: None,
                    component: $name::variants::main::Demo,
                },
                $(
                    $(
                        ComponentVariantDemoData {
                            name: stringify!($variant),
                            rs_highlighted: HighlightedCode {
                                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs.base16-ocean.light.html")),
                                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs.base16-ocean.dark.html")),
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
            docs: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/docs.html")),
            component: HighlightedCode {
                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.base16-ocean.light.html")),
                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.base16-ocean.dark.html")),
            },
            style: HighlightedCode {
                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.light.html")),
                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.dark.html")),
            },
            variants: &[
                ComponentVariantDemoData {
                    name: "main",
                    rs_highlighted: HighlightedCode {
                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.base16-ocean.light.html")),
                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.base16-ocean.dark.html")),
                    },
                    css_highlighted: Some(HighlightedCode {
                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/demo.css.base16-ocean.light.html")),
                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/demo.css.base16-ocean.dark.html")),
                    }),
                    component: $name::variants::main::Demo,
                },
                $(
                    $(
                        ComponentVariantDemoData {
                            name: stringify!($variant),
                            rs_highlighted: HighlightedCode {
                                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs.base16-ocean.light.html")),
                                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs.base16-ocean.dark.html")),
                            },
                            css_highlighted: Some(HighlightedCode {
                                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/demo.css.base16-ocean.light.html")),
                                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/demo.css.base16-ocean.dark.html")),
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
    button,
    calendar[simple, internationalized, range, multi_month, unavailable_dates],
    checkbox,
    collapsible,
    color_picker,
    context_menu,
    card,
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
    virtual_list[random_heights],
    scroll_area,
    select[multi],
    separator,
    sheet,
    sidebar(block)[floating, inset],
    skeleton,
    slider[dynamic_range, range],
    switch,
    tabs,
    textarea[outline, fade, ghost],
    toast,
    toggle_group,
    toggle,
    toolbar,
    tooltip,
    table,
);
