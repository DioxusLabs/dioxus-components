use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronsUpDown;
use dioxus_primitives::collapsible::{
    self, CollapsibleContentProps, CollapsibleProps, CollapsibleTriggerProps,
};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/collapsible/style.css")]
struct Styles;

#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    rsx! {
        collapsible::Collapsible {
            keep_mounted: props.keep_mounted,
            default_open: props.default_open,
            disabled: props.disabled,
            open: props.open,
            on_open_change: props.on_open_change,
            as: props.r#as,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
    let base = attributes!(button {
        class: Styles::dx_collapsible_trigger,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    let show_icon = props.r#as.is_none();

    rsx! {
        collapsible::CollapsibleTrigger { as: props.r#as, attributes: merged,
            {props.children}
            if show_icon {
                ChevronsUpDown {
                    size: "1rem",
                    stroke: "var(--secondary-color-3)",
                }
            }
        }
    }
}

#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    rsx! {
        collapsible::CollapsibleContent {
            class: Styles::dx_collapsible_content,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CollapsibleItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            border: "1px solid var(--primary-color-6)",
            border_radius: "0.5rem",
            padding: "1rem",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CollapsibleList(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "0.5rem",
            max_width: "20rem",
            color: "var(--secondary-color-3)",
            ..attributes,
            {children}
        }
    }
}
