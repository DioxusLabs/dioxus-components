use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "height: 260px; width: 100%; max-width: 760px;",
            SplitPane {
                direction: SplitPaneDirection::Horizontal,
                divider_size: 18.0,
                divider_class: "split-pane-demo-divider",
                divider_style: "background: color-mix(in oklab, var(--secondary-color-2) 18%, var(--primary-color-3));",
                Pane {
                    default_size: SplitPaneSize::percent(45.0),
                    min_size: SplitPaneSize::px(180.0),
                    Panel { title: "Custom Hook", body: "Root-level divider_class and divider_style style all dividers." }
                }
                SplitPaneDivider {
                    divider: rsx! {
                        span {
                            style: "width: 0.375rem; height: 3rem; border-radius: 9999px; background: var(--secondary-color-2); box-shadow: 0 0 0 1px var(--primary-color-1);"
                        }
                    },
                }
                Pane {
                    min_size: SplitPaneSize::px(180.0),
                    Panel { title: "Custom Content", body: "Divider content can be replaced per divider." }
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
