use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.75rem; width: 100%; max-width: 900px;",
            div {
                style: "font-size: 0.875rem; color: var(--primary-color-11);",
                "Three panes share one horizontal SplitPane with two independently focusable dividers."
            }
            div {
                style: "height: 300px;",
                SplitPane {
                    direction: SplitPaneDirection::Horizontal,
                    Pane {
                        default_size: SplitPaneSize::percent(22.0),
                        min_size: SplitPaneSize::px(140.0),
                        Panel {
                            title: "Navigator",
                            body: "A narrow primary pane for folders, filters, or project sections.",
                        }
                    }
                    SplitPaneDivider {}
                    Pane {
                        default_size: SplitPaneSize::percent(48.0),
                        min_size: SplitPaneSize::px(220.0),
                        Panel {
                            title: "Editor",
                            body: "The center pane keeps most of the workspace while both sides remain resizable.",
                        }
                    }
                    SplitPaneDivider {}
                    Pane {
                        min_size: SplitPaneSize::px(180.0),
                        Panel {
                            title: "Inspector",
                            body: "Additional panes work by alternating Pane and SplitPaneDivider children.",
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
            style: "height: 100%; box-sizing: border-box; padding: 1rem; display: flex; flex-direction: column; gap: 0.5rem;",
            h3 { style: "margin: 0; font-size: 1rem;", "{title}" }
            p { style: "margin: 0; color: var(--primary-color-11); line-height: 1.4;", "{body}" }
        }
    }
}
