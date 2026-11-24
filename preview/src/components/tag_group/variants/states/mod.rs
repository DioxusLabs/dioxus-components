use dioxus::prelude::*;

use super::super::component::*;

#[component]
pub fn Demo() -> Element {
    let mut nonselectable_value = use_signal(|| Some("alpha".to_string()));

    let mut mixed_values = use_signal(|| vec!["bug".to_string(), "desktop".to_string()]);
    let mixed_values_signal = use_memo(move || Some(mixed_values()));

    rsx! {
        div {
            TagGroup {
                "data-testid": "tag-group-disabled",
                disabled: true,
                TagGroupLabel { "Group disabled" }
                TagList {
                    TagGroupEmpty { "No tags" }
                    Tag { index: 0usize, value: "locked", "locked" RemoveButton {} }
                    Tag { index: 1usize, value: "archived", "archived" RemoveButton {} }
                }
            }

            TagGroup {
                "data-testid": "tag-group-nonselectable",
                value: Some(nonselectable_value.into()),
                on_value_change: move |value| nonselectable_value.set(value),
                selectable: false,
                TagGroupLabel { "Non-selectable removable" }
                TagList {
                    TagGroupEmpty { "No tags" }
                    Tag { index: 0usize, value: "alpha", "alpha" RemoveButton {} }
                    Tag { index: 1usize, value: "beta", "beta" RemoveButton {} }
                    Tag { index: 2usize, value: "gamma", "gamma" RemoveButton {} }
                }
            }

            TagGroupMulti {
                "data-testid": "tag-group-mixed-removable",
                values: mixed_values_signal,
                on_values_change: move |values| mixed_values.set(values),
                TagGroupLabel { "Mixed removable" }
                TagList {
                    TagGroupEmpty { "No tags" }
                    Tag { index: 0usize, value: "bug", "bug" RemoveButton {} }
                    Tag { index: 1usize, value: "core", "core" RemoveButton {} }
                    Tag { index: 2usize, value: "desktop", "desktop" }
                    Tag { index: 3usize, value: "feature", disabled: true, "feature" RemoveButton {} }
                }
            }
        }
    }
}
