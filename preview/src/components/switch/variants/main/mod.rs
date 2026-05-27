use super::super::component::*;
use dioxus::prelude::*;

#[css_module("/src/components/switch/style.css")]
struct Styles;

#[component]
pub fn Demo() -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        div { class: Styles::dx_switch_example,
            Switch {
                checked: checked(),
                aria_label: "Switch Demo",
                on_checked_change: move |new_checked| {
                    checked.set(new_checked);
                },
            }
        }
    }
}
