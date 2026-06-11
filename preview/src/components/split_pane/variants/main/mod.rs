use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut last_size = use_signal(|| "Drag the divider or focus it and use arrow keys".to_string());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.75rem; width: 100%; max-width: 760px;",
            div { style: "font-size: 0.875rem; color: var(--primary-color-11);", "{last_size}" }
            div {
                style: "height: 260px;",
                SplitPane {
                    direction: SplitPaneDirection::Horizontal,
                    step: 24.0,
                    on_resize: move |event: SplitPaneResizeEvent| {
                        if let Some(size) = event.sizes.first() {
                            last_size.set(format!("Left pane: {}", format_size(size)));
                        }
                    },
                    Pane {
                        default_size: SplitPaneSize::percent(35.0),
                        min_size: SplitPaneSize::px(160.0),
                        DemoPanel { title: "Navigator", body: "Project tree, filters, and quick actions." }
                    }
                    SplitPaneDivider {}
                    Pane {
                        min_size: SplitPaneSize::px(220.0),
                        DemoPanel { title: "Preview", body: "Resizable content region using pointer and keyboard input." }
                    }
                }
            }
        }
    }
}

#[component]
fn DemoPanel(title: &'static str, body: &'static str) -> Element {
    rsx! {
        section {
            style: "height: 100%; box-sizing: border-box; padding: 1rem; display: flex; flex-direction: column; gap: 0.5rem;",
            h3 { style: "margin: 0; font-size: 1rem;", "{title}" }
            p { style: "margin: 0; color: var(--primary-color-11); line-height: 1.4;", "{body}" }
        }
    }
}

fn format_size(size: &SplitPaneSize) -> String {
    match size {
        SplitPaneSize::Px(px) => format!("{px:.0}px"),
        SplitPaneSize::Percent(percent) => format!("{percent:.0}%"),
    }
}
