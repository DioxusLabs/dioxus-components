use super::super::component::*;
use crate::components::button::{Button, ButtonVariant};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "1.5rem",
            width: "100%",
            max_width: "28rem",

            Item {
                ItemContent {
                    ItemTitle { "Default Variant" }
                    ItemDescription {
                        "Standard styling with subtle background and borders."
                    }
                }
                ItemActions {
                    Button { variant: ButtonVariant::Outline, "Open" }
                }
            }

            Item { variant: ItemVariant::Outline,
                ItemContent {
                    ItemTitle { "Outline Variant" }
                    ItemDescription {
                        "Outlined style with clear borders and transparent background."
                    }
                }
                ItemActions {
                    Button { variant: ButtonVariant::Outline, "Open" }
                }
            }

            Item { variant: ItemVariant::Muted,
                ItemContent {
                    ItemTitle { "Muted Variant" }
                    ItemDescription {
                        "Subdued appearance with muted colors for secondary content."
                    }
                }
                ItemActions {
                    Button { variant: ButtonVariant::Outline, "Open" }
                }
            }
        }
    }
}
