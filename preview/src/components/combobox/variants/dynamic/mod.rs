use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut show_svelte = use_signal(|| true);
    let mut show_solid = use_signal(|| true);

    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 20rem;",
            div { style: "display: flex; gap: 0.5rem;",
                button {
                    r#type: "button",
                    onpointerdown: move |event| event.prevent_default(),
                    onclick: move |_| show_svelte.toggle(),
                    "Toggle SvelteKit"
                }
                button {
                    r#type: "button",
                    onpointerdown: move |event| event.prevent_default(),
                    onclick: move |_| show_solid.toggle(),
                    "Toggle SolidStart"
                }
            }
            Combobox::<String> {
                placeholder: "Select framework...",
                aria_label: "Dynamic framework",
                list_aria_label: "Dynamic frameworks",
                ComboboxEmpty { "No framework found." }
                ComboboxOption::<String> {
                    index: 0usize,
                    value: "next".to_string(),
                    text_value: "Next.js",
                    "Next.js"
                }
                if show_svelte() {
                    ComboboxOption::<String> {
                        index: 1usize,
                        value: "svelte".to_string(),
                        text_value: "SvelteKit",
                        "SvelteKit"
                    }
                }
                if show_solid() {
                    ComboboxOption::<String> {
                        index: 2usize,
                        value: "solid".to_string(),
                        text_value: "SolidStart",
                        "SolidStart"
                    }
                }
                ComboboxOption::<String> {
                    index: 3usize,
                    value: "dioxus".to_string(),
                    text_value: "Dioxus",
                    "Dioxus"
                }
            }
        }
    }
}
