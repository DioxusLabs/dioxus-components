use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};

#[css_module("/src/components/form/style.css")]
struct Styles;

#[component]
pub fn Demo() -> Element {
    rsx! {
        form {
            class: Styles::dx_form_example,
            onsubmit: move |e| {
                tracing::info!("{:?}", e.values());
            },
            Checkbox { id: "tos-check", class: Styles::dx_tos_check, name: "tos-check",
                CheckboxIndicator { "+" }
            }
            label { r#for: "tos-check", "I agree to the terms presented." }
            br {}
            button { r#type: "submit", "Submit" }
        }
    }
}
