use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "height: 360px; width: 100%; max-width: 820px;",
            SplitPane {
                direction: SplitPaneDirection::Horizontal,
                Pane {
                    default_size: SplitPaneSize::percent(32.0),
                    min_size: SplitPaneSize::px(180.0),
                    Panel { title: "Files", body: "A primary horizontal split." }
                }
                SplitPaneDivider {}
                Pane {
                    min_size: SplitPaneSize::px(260.0),
                    SplitPane {
                        direction: SplitPaneDirection::Vertical,
                        Pane {
                            default_size: SplitPaneSize::percent(62.0),
                            min_size: SplitPaneSize::px(150.0),
                            Panel { title: "Editor", body: "Nested panes can compose inside a parent pane." }
                        }
                        SplitPaneDivider {}
                        Pane {
                            min_size: SplitPaneSize::px(90.0),
                            Panel { title: "Console", body: "Each split owns its own direction, constraints, and dividers." }
                        }
                    }
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
