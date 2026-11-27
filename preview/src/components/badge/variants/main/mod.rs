use dioxus::prelude::*;

use super::super::component::*;

use super::super::component::Styles;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: Styles::dx_badge_example,

            Badge { "Primary" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
            Badge {
                variant: BadgeVariant::Secondary,
                style: "background-color: var(--focused-border-color)",
                VerifiedIcon {}
                "Verified"
            }
        }
    }
}
