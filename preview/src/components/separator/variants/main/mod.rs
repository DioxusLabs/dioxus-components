use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            "One thing"
            Separator {
                style: "margin: 25px 10px; width: 50%;",
                horizontal: true,
                decorative: true,
            }
            "Another thing"
        }
    }
}
