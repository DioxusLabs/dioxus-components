use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "height: 340px; width: 100%; max-width: 720px;",
            SplitPane {
                direction: SplitPaneDirection::Vertical,
                Pane {
                    default_size: SplitPaneSize::percent(45.0),
                    min_size: SplitPaneSize::px(110.0),
                    PaneContent { title: "Timeline", content: "Stack panes vertically for editors, inspectors, and logs." }
                }
                SplitPaneDivider {}
                Pane {
                    min_size: SplitPaneSize::px(120.0),
                    PaneContent { title: "Details", content: "The divider uses row-resize behavior and horizontal separator semantics." }
                }
            }
        }
    }
}

#[component]
fn PaneContent(title: &'static str, content: &'static str) -> Element {
    rsx! {
        div {
            style: "height: 100%; box-sizing: border-box; padding: 1rem;",
            h3 { style: "margin: 0 0 0.5rem; font-size: 1rem;", "{title}" }
            p { style: "margin: 0; color: var(--primary-color-11);", "{content}" }
        }
    }
}
