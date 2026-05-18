use super::super::component::*;
use dioxus::prelude::*;
use dioxus_kit_core::ContentSide;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "padding: 50px; display: flex; flex-direction: row; flex-wrap: wrap; gap: 40px; justify-content: center; align-items: center;",
            HoverCard {
                HoverCardTrigger {
                    i { "Dioxus" }
                }
                HoverCardContent { side: ContentSide::Bottom,
                    div { padding: "1rem",
                        "Dioxus is"
                        i { " the " }
                        "Rust framework for building fullstack web, desktop, and mobile apps. Iterate with live hotreloading, add server functions, and deploy in record time."
                    }
                }
            }
        }
    }
}
