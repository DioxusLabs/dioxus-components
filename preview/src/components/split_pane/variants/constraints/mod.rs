use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "height: 260px; width: 100%; max-width: 760px;",
            SplitPane {
                direction: SplitPaneDirection::Horizontal,
                Pane {
                    default_size: SplitPaneSize::px(220.0),
                    min_size: SplitPaneSize::px(160.0),
                    max_size: SplitPaneSize::px(320.0),
                    Panel { title: "Fixed Range", body: "This pane stays between 160px and 320px." }
                }
                SplitPaneDivider {}
                Pane {
                    min_size: SplitPaneSize::percent(30.0),
                    Panel { title: "Flexible", body: "The neighboring pane absorbs the remaining width." }
                }
            }
        }
    }
}

#[component]
fn Panel(title: &'static str, body: &'static str) -> Element {
    rsx! {
        div {
            style: "height: 100%; box-sizing: border-box; padding: 1rem;",
            h3 { style: "margin: 0 0 0.5rem; font-size: 1rem;", "{title}" }
            p { style: "margin: 0; color: var(--primary-color-11);", "{body}" }
        }
    }
}
