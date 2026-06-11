use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut status = use_signal(|| "Snap points: 25%, 50%, 75%".to_string());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.75rem; width: 100%; max-width: 760px;",
            div { style: "font-size: 0.875rem; color: var(--primary-color-11);", "{status}" }
            div {
                style: "height: 260px;",
                SplitPane {
                    direction: SplitPaneDirection::Horizontal,
                    snap_points: vec![
                        SplitPaneSize::percent(25.0),
                        SplitPaneSize::percent(50.0),
                        SplitPaneSize::percent(75.0),
                    ],
                    snap_tolerance: 24.0,
                    step: 12.0,
                    on_resize_end: move |event: SplitPaneResizeEvent| {
                        if let Some(size) = event.sizes.first() {
                            status.set(format!("Released at {}", format_size(size)));
                        }
                    },
                    Pane {
                        default_size: SplitPaneSize::percent(50.0),
                        min_size: SplitPaneSize::percent(15.0),
                        Panel { title: "Snapping", body: "Resize near common percentages to snap into place." }
                    }
                    SplitPaneDivider {}
                    Pane {
                        min_size: SplitPaneSize::percent(15.0),
                        Panel { title: "Workspace", body: "Snap behavior applies to pointer and keyboard resizing." }
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

fn format_size(size: &SplitPaneSize) -> String {
    match size {
        SplitPaneSize::Px(px) => format!("{px:.0}px"),
        SplitPaneSize::Percent(percent) => format!("{percent:.0}%"),
    }
}
