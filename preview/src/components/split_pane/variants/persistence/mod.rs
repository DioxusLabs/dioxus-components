use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let (stored_sizes, persist_sizes) =
        use_split_pane_persistence("dioxus-preview-split-pane", SplitPaneStorage::Local);
    let first_size = use_memo(move || stored_sizes().and_then(|sizes| sizes.first().cloned()));
    let second_size = use_memo(move || stored_sizes().and_then(|sizes| sizes.get(1).cloned()));

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.75rem; width: 100%; max-width: 760px;",
            div { style: "font-size: 0.875rem; color: var(--primary-color-11);", "Resize, release, and reload the preview to restore the layout." }
            div {
                style: "height: 260px;",
                SplitPane {
                    direction: SplitPaneDirection::Horizontal,
                    on_resize_end: persist_sizes,
                    Pane {
                        default_size: first_size().or(Some(SplitPaneSize::percent(33.0))),
                        min_size: SplitPaneSize::px(160.0),
                        Panel { title: "Persisted A", body: "The persistence hook restores saved browser storage sizes." }
                    }
                    SplitPaneDivider {}
                    Pane {
                        default_size: second_size().or(Some(SplitPaneSize::percent(67.0))),
                        min_size: SplitPaneSize::px(180.0),
                        Panel { title: "Persisted B", body: "The resize-end callback stores final pane sizes." }
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
