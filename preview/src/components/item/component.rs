use crate::components::separator::Separator;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
#[css_module("/src/components/item/style.css")]
struct Styles;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ItemVariant::Default => "default",
            ItemVariant::Outline => "outline",
            ItemVariant::Muted => "muted",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

impl ItemSize {
    pub fn class(&self) -> &'static str {
        match self {
            ItemSize::Default => "default",
            ItemSize::Sm => "sm",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ItemMediaVariant::Default => "default",
            ItemMediaVariant::Icon => "icon",
            ItemMediaVariant::Image => "image",
        }
    }
}

#[component]
pub fn ItemGroup(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_item_group,
        role: "list",
        "data-slot": "item-group",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemSeparator(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_item_separator,
        "data-slot": "item-separator",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        Separator { horizontal: true, decorative: true, attributes: merged }
    }
}

#[component]
pub fn Item(
    #[props(default)] variant: ItemVariant,
    #[props(default)] size: ItemSize,
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_item,
        "data-slot": "item",
        "data-variant": variant.class(),
        "data-size": size.class(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        if let Some(dynamic) = r#as {
            {dynamic.call(merged)}
        } else {
            div {
                onclick: move |event| {
                    if let Some(f) = &onclick {
                        f.call(event);
                    }
                },
                onkeydown: move |event| {
                    if let Some(f) = &onkeydown {
                        f.call(event);
                    }
                },
                ..merged,
                {children}
            }
        }
    }
}

#[component]
pub fn ItemMedia(
    #[props(default)] variant: ItemMediaVariant,
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_item_media,
        "data-slot": "item-media",
        "data-variant": variant.class(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemContent(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_item_content,
        "data-slot": "item-content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemTitle(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_item_title,
        "data-slot": "item-title",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemDescription(
    #[props(extends=GlobalAttributes)]
    #[props(extends=p)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(p {
        class: Styles::dx_item_description,
        "data-slot": "item-description",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        p { ..merged,{children} }
    }
}

#[component]
pub fn ItemActions(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_item_actions,
        "data-slot": "item-actions",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemHeader(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_item_header,
        "data-slot": "item-header",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemFooter(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_item_footer,
        "data-slot": "item-footer",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}
