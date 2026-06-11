use super::super::component::*;
use crate::components::slider::Slider;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut pane_size = use_signal(|| Some(SplitPaneSize::percent(40.0)));
    let mut slider_value = use_signal(|| Some(40.0_f64));
    let formatted_value = use_memo(move || slider_value().unwrap_or(40.0));

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 760px;",
            div {
                style: "display: flex; align-items: center; gap: 1rem;",
                span { style: "min-width: 8rem; font-weight: 600;", "Sidebar {formatted_value:.0}%" }
                Slider {
                    label: "Controlled pane size",
                    horizontal: true,
                    min: 20.0,
                    max: 70.0,
                    step: 1.0,
                    value: slider_value,
                    on_value_change: move |value: f64| {
                        slider_value.set(Some(value));
                        pane_size.set(Some(SplitPaneSize::percent(value)));
                    },
                }
            }
            div {
                style: "height: 260px;",
                SplitPane {
                    direction: SplitPaneDirection::Horizontal,
                    on_resize: move |event: SplitPaneResizeEvent| {
                        if let Some(size) = event.sizes.first().cloned() {
                            if let Some(value) = first_pane_percent(&event.sizes) {
                                let clamped = value.clamp(20.0, 70.0);
                                slider_value.set(Some(clamped));
                                pane_size.set(Some(SplitPaneSize::percent(clamped)));
                            } else {
                                pane_size.set(Some(size));
                            }
                        }
                    },
                    Pane {
                        size: pane_size,
                        min_size: SplitPaneSize::percent(20.0),
                        max_size: SplitPaneSize::percent(70.0),
                        Panel { title: "Controlled", body: "The first pane is driven by a signal shared with the slider." }
                    }
                    SplitPaneDivider {}
                    Pane {
                        min_size: SplitPaneSize::px(180.0),
                        Panel { title: "Remaining", body: "Dragging the divider updates the same controlled signal." }
                    }
                }
            }
        }
    }
}

fn first_pane_percent(sizes: &[SplitPaneSize]) -> Option<f64> {
    let total = sizes.iter().map(size_value).sum::<f64>();
    let first = size_value(sizes.first()?);
    (total > 0.0).then_some(first / total * 100.0)
}

fn size_value(size: &SplitPaneSize) -> f64 {
    match size {
        SplitPaneSize::Px(value) | SplitPaneSize::Percent(value) => *value,
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
