use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| Some("svelte".to_string()));
    let mut open = use_signal(|| Some(false));

    let frameworks: &[(&str, &str)] = &[
        ("next", "Next.js"),
        ("svelte", "SvelteKit"),
        ("nuxt", "Nuxt.js"),
        ("remix", "Remix"),
        ("astro", "Astro"),
        ("solid", "SolidStart"),
        ("dioxus", "Dioxus"),
    ];

    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 20rem;",
            div { style: "display: flex; gap: 0.5rem;",
                button {
                    r#type: "button",
                    onclick: move |_| value.set(Some("astro".to_string())),
                    "Set Astro"
                }
                button {
                    r#type: "button",
                    onclick: move |_| open.set(Some(true)),
                    "Open"
                }
            }
            div {
                "data-testid": "combobox-controlled-value",
                "{value().unwrap_or_else(|| \"none\".to_string())}"
            }
            Combobox::<String> {
                value: Some(value.into()),
                open,
                on_value_change: move |next| value.set(next),
                on_open_change: move |next| open.set(Some(next)),
                placeholder: "Select framework...",
                aria_label: "Controlled framework",
                list_aria_label: "Controlled frameworks",
                ComboboxEmpty { "No framework found." }
                for (i , (value , label)) in frameworks.iter().enumerate() {
                    ComboboxOption::<String> {
                        index: i,
                        value: value.to_string(),
                        text_value: label.to_string(),
                        {*label}
                    }
                }
            }
        }
    }
}
