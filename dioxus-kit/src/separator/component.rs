use dioxus::prelude::*;
use dioxus_kit_core::separator::{self, SeparatorProps};
use dioxus_kit_core::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/separator/style.css")]
struct Styles;

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_separator,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        separator::Separator {
            horizontal: props.horizontal,
            decorative: props.decorative,
            attributes: merged,
            {props.children}
        }
    }
}
