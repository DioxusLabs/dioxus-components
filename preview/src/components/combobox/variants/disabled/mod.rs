use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let frameworks: &[(&str, &str, bool)] = &[
        ("next", "Next.js", false),
        ("svelte", "SvelteKit", true),
        ("nuxt", "Nuxt.js", false),
        ("remix", "Remix", false),
    ];

    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 20rem;",
            Combobox::<String> {
                placeholder: "Select framework...",
                aria_label: "Framework with disabled option",
                list_aria_label: "Frameworks with disabled option",
                ComboboxEmpty { "No framework found." }
                for (i , (value , label , disabled)) in frameworks.iter().enumerate() {
                    ComboboxOption::<String> {
                        index: i,
                        value: value.to_string(),
                        text_value: label.to_string(),
                        disabled: *disabled,
                        {*label}
                    }
                }
            }
            Combobox::<String> {
                disabled: true,
                placeholder: "Disabled combobox",
                aria_label: "Disabled combobox",
                list_aria_label: "Disabled list",
                ComboboxOption::<String> {
                    index: 0usize,
                    value: "disabled".to_string(),
                    text_value: "Disabled option",
                    "Disabled option"
                }
            }
        }
    }
}
