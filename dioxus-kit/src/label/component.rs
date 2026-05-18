use dioxus::prelude::*;
use dioxus_kit_core::label::{self, LabelProps};
#[css_module("/src/label/style.css")]
struct Styles;

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label::Label {
            class: Styles::dx_label,
            html_for: props.html_for,
            attributes: props.attributes,
            {props.children}
        }
    }
}
