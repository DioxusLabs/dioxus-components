use super::super::component::*;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowUpRight, GitMerge};

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            flex_wrap: "wrap",
            align_items: "flex-start",
            gap: "0.75rem",

            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Icon,
                ArrowUpRight { size: "16px" }
            }

            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Icon,
                border_radius: "50%",
                ArrowUpRight { size: "16px" }
            }

            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Sm,
                GitMerge { size: "16px" }
                "Merge"
            }
        }
    }
}
