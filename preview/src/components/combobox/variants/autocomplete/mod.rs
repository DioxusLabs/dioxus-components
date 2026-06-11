use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| None::<String>);
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
        div { class: "dx-combobox-demo-stack",
            Autocomplete {
                value: Some(value.into()),
                on_value_change: move |next| value.set(next),
                placeholder: "Type a framework...",
                ComboboxEmpty { "No framework found." }
                for (index, (value, label)) in frameworks.iter().enumerate() {
                    ComboboxOption::<String> {
                        index,
                        value: value.to_string(),
                        text_value: label.to_string(),
                        {*label}
                    }
                }
            }
            p { class: "dx-combobox-demo-value",
                "Selected: {value().unwrap_or_else(|| \"none\".to_string())}"
            }
        }
    }
}
