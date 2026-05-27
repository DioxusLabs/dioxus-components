use super::super::component::*;
use crate::components::label::Label;
use crate::components::switch::Switch;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut percentage_mode = use_signal(|| true);
    let mut current_value = use_signal(|| Some(50.0_f64));

    let max = use_memo(move || if percentage_mode() { 100.0 } else { 1000.0 });
    let step = use_memo(move || if percentage_mode() { 1.0 } else { 10.0 });
    let formatted_value = use_memo(move || {
        current_value()
            .map(|v| {
                if percentage_mode() {
                    format!("{v:.0}%")
                } else {
                    format!("{v:.0}/1000")
                }
            })
            .unwrap_or_default()
    });

    rsx! {
        div {
            style: "margin-bottom: 15px; display: flex; gap: 10px;",
            Switch {
                id: "mode-switch",
                checked: percentage_mode(),
                on_checked_change: move |new_checked| {
                    percentage_mode.set(new_checked);
                    if new_checked {
                        if let Some(v) = current_value() {
                            current_value.set(Some(v.min(100.0)));
                        }
                    }
                },
            }
            Label {
                html_for: "mode-switch",
                "Percentage"
            }
        }

        div {
            style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;",
            "{formatted_value}"
        }

        Slider {
            label: "Dynamic Range Slider",
            horizontal: true,
            min: 0.0,
            max,
            step,
            value: current_value,
            on_value_change: move |new_value: f64| {
                current_value.set(Some(new_value));
            },
        }
    }
}
