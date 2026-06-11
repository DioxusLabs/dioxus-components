use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::combobox::default_combobox_filter;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| None::<String>);
    let mut query = use_signal(String::new);
    let visible_indices = use_memo(move || {
        let query = query.read().clone();
        (0..1000)
            .filter(|index| default_combobox_filter(&query, &format!("Option {index}")))
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "dx-combobox-demo-stack",
            VirtualizedCombobox::<String> {
                value: Some(value.into()),
                on_value_change: move |next| value.set(next),
                query: Some(query()),
                on_query_change: move |next| query.set(next),
                placeholder: "Search 1,000 options...",
                aria_label: "Virtualized option picker",
                list_aria_label: "Virtualized options",
                count: 1000usize,
                visible_indices: Some(visible_indices.into()),
                estimate_size: |_: usize| 36,
                render_option: |index: usize| rsx! {
                    ComboboxOption::<String> {
                        index,
                        value: format!("option-{index}"),
                        text_value: format!("Option {index}"),
                        "Option {index}"
                    }
                },
            }
            p { class: "dx-combobox-demo-value",
                "Selected: {value().unwrap_or_else(|| \"none\".to_string())}"
            }
        }
    }
}
