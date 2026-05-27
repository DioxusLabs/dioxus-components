use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tabs {
            default_value: "tab1".to_string(),
            horizontal: true,
            max_width: "16rem",
            TabList {
                TabTrigger { value: "tab1".to_string(), index: 0usize, "Tab 1" }
                TabTrigger { value: "tab2".to_string(), index: 1usize, "Tab 2" }
                TabTrigger { value: "tab3".to_string(), index: 2usize, "Tab 3" }
            }
            TabContent { index: 0usize, value: "tab1".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Tab 1 Content"
                }
            }
            TabContent {
                index: 1usize,
                value: "tab2".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Tab 2 Content"
                }
            }
            TabContent { index: 2usize, value: "tab3".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Tab 3 Content"
                }
            }
        }
    }
}
