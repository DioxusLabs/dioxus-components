use dioxus::prelude::*;

use super::super::component::*;

#[component]
pub fn Demo() -> Element {
    let labels = ["bug", "feature", "core", "desktop", "example", "duplicate"];
    let tags = labels.iter().enumerate().map(|(index, &t)| {
        rsx! {
            Tag {
                index,
                value: t,
                "{t}"
                RemoveButton {}
            }
        }
    });

    let mut value = use_signal(|| Some("core".to_string()));

    rsx! {
        TagGroup {
            value: Some(value.into()),
            on_value_change: move |v| value.set(v),
            allow_empty_selection: false,
            TagGroupLabel { "Labels" }
            TagList {
                TagGroupEmpty { "No tags" }
                {tags}
            }
        }
    }
}
