use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut current_value = use_signal(|| 50.0);

    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",

            // Display the current value
            div { style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;", "{current_value:.0}%" }

            Slider {
                label: "Demo Slider",
                horizontal: true,
                min: 0.0,
                max: 100.0,
                step: 1.0,
                default_value: 50.0,
                on_value_change: move |value: f64| {
                    current_value.set(value);
                },
            }
        }
    }
}
