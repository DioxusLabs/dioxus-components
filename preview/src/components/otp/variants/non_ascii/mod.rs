use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(String::new);
    let mut last_complete = use_signal(String::new);

    rsx! {
        div {
            style: "display: grid; gap: 0.75rem; justify-items: center;",
            p {
                style: "margin: 0; color: var(--secondary-color-2); font-size: 0.875rem; font-weight: 500;",
                "Emoji code"
            }
            OneTimePasswordInput {
                maxlength: 4usize,
                value: value(),
                inputmode: "text",
                autocomplete: "off",
                validate: |s: String| s.chars().all(|c| !c.is_ascii()),
                on_value_change: move |v| value.set(v),
                on_complete: move |v| last_complete.set(v),
                aria_label: "Emoji code",
                OneTimePasswordGroup {
                    OneTimePasswordSlot { index: 0usize }
                    OneTimePasswordSlot { index: 1usize }
                    OneTimePasswordSlot { index: 2usize }
                    OneTimePasswordSlot { index: 3usize }
                }
            }
            output {
                id: "otp-non-ascii-value",
                aria_live: "polite",
                style: "min-height: 1.25rem; color: var(--secondary-color-1); font-size: 1rem;",
                "{value}"
            }
            div { style: "display: none;",
                span { id: "otp-non-ascii-complete", "{last_complete}" }
            }
        }
    }
}
