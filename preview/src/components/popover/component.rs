use dioxus::prelude::*;
use dioxus_primitives::popover::{
    self, PopoverContentProps, PopoverRootProps, PopoverTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/popover/style.css")]
struct Styles;

#[component]
pub fn PopoverRoot(props: PopoverRootProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_popover
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        popover::PopoverRoot {
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let base = attributes!(button {
        class: Styles::dx_popover_trigger
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        popover::PopoverTrigger { attributes: merged, {props.children} }
    }
}

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    let class = if let Some(class) = props.class {
        format!("{} {}", Styles::dx_popover_content, class)
    } else {
        Styles::dx_popover_content.to_string()
    };

    rsx! {
        popover::PopoverContent {
            class,
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
