use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut values = use_signal(|| Some(vec!["dioxus".to_string(), "components".to_string()]));

    rsx! {
        div { class: "dx-combobox-demo-stack",
            TagsInput {
                values,
                on_values_change: move |next| values.set(Some(next)),
                placeholder: "Add tag and press Enter...",
            }
            p { class: "dx-combobox-demo-value",
                "Tags: {values().unwrap_or_default().join(\", \")}"
            }
        }
    }
}
