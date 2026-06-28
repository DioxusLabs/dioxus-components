use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(String::new);
    let mut last_complete = use_signal(String::new);
    let mut disabled = use_signal(|| false);

    rsx! {
        div { style: "position: relative; display: inline-flex; align-items: center; justify-content: center;",
            OneTimePasswordInput {
                maxlength: 6usize,
                value: value(),
                disabled: disabled(),
                validate: |s: String| s.chars().all(|c| c.is_ascii_digit()),
                on_value_change: move |v| value.set(v),
                on_complete: move |v| last_complete.set(v),
                aria_label: "One-time password",
                OneTimePasswordGroup {
                    OneTimePasswordSlot { index: 0usize }
                    OneTimePasswordSlot { index: 1usize }
                    OneTimePasswordSlot { index: 2usize }
                }
                OneTimePasswordSeparator {}
                OneTimePasswordGroup {
                    OneTimePasswordSlot { index: 3usize }
                    OneTimePasswordSlot { index: 4usize }
                    OneTimePasswordSlot { index: 5usize }
                }
            }
            div { style: "display: none;",
                span { id: "otp-value", "{value}" }
                span { id: "otp-complete", "{last_complete}" }
                button {
                    id: "otp-toggle-disabled",
                    r#type: "button",
                    onclick: move |_| disabled.toggle(),
                    "toggle disabled"
                }
            }
        }
    }
}
