use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger { "Rich content" }
            TooltipContent { side: ContentSide::Left, style: "width: 200px;",
                h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
                p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
            }
        }
    }
}
