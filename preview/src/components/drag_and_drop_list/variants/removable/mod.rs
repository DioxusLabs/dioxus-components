use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let items = [
        ("cat", "Cat 🐱"),
        ("cow", "Cow 🐮"),
        ("dog", "Dog 🐶"),
        ("fox", "Fox 🦊"),
        ("pig", "Pig 🐷"),
    ]
    .map(|(key, label)| {
        rsx! {
            span { key: "{key}", "{label}" }
        }
    })
    .to_vec();

    rsx! {
        DragAndDropList {
            items,
            is_removable: true
        }
    }
}
