use dioxus::prelude::*;

use super::super::component::*;

#[component]
pub fn Demo() -> Element {
    let labels = ["bug", "feature", "core", "desktop", "example", "duplicate"];
    let tags = labels.iter().enumerate().map(|(index, &t)| {
        let disabled = matches!(t, "feature" | "example");
        rsx! {
            Tag {
                index,
                value: t,
                disabled,
                "{t}"
                RemoveButton {}
            }
        }
    });

    let mut values = use_signal(|| vec!["bug".to_string()]);
    let values_signal = use_memo(move || Some(values()));

    rsx! {
        TagGroupMulti {
            values: values_signal,
            on_values_change: move |v| values.set(v),
            allow_empty_selection: false,
            TagGroupLabel { "Labels" }
            TagList {
                TagGroupEmpty { "No tags" }
                {tags}
            }
        }
    }
}
