use dioxus::prelude::*;
use dioxus_kit_core::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/skeleton/style.css")]
struct Styles;

#[component]
pub fn Skeleton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_skeleton,
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged }
    }
}
