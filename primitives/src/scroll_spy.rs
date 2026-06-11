//! Scroll spy state and helpers for tracking active document headings.

use dioxus::prelude::*;
use serde::Deserialize;

/// Options used to configure [`use_scroll_spy`].
#[derive(Clone, PartialEq)]
pub struct ScrollSpyOptions {
    /// CSS selector used to find heading elements.
    pub selector: String,

    /// Viewport offset used when selecting the active heading.
    pub offset: f64,

    /// Scroll container that emits scroll updates.
    pub scroll_host: ScrollSpyScrollHost,

    /// Data to expose before browser-side heading discovery runs.
    pub initial_data: Vec<ScrollSpyData>,
}

impl Default for ScrollSpyOptions {
    fn default() -> Self {
        Self {
            selector: "h1, h2, h3, h4, h5, h6".to_string(),
            offset: 0.0,
            scroll_host: ScrollSpyScrollHost::Window,
            initial_data: Vec::new(),
        }
    }
}

/// Scroll container used by [`use_scroll_spy`].
#[derive(Clone, PartialEq, Eq)]
pub enum ScrollSpyScrollHost {
    /// Listen to the browser window.
    Window,

    /// Listen to the first element matching the CSS selector.
    Selector(String),
}

/// Data for one heading tracked by [`use_scroll_spy`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ScrollSpyData {
    /// Stable id used by table-of-contents controls.
    pub id: String,

    /// Human-readable heading text.
    pub value: String,

    /// Heading depth, where `h1` is `1` and `h6` is `6`.
    pub depth: u8,
}

/// State returned by [`use_scroll_spy`].
#[derive(Clone, Copy)]
pub struct ScrollSpyState {
    /// Index of the currently active heading.
    pub active: Signal<Option<usize>>,

    /// Heading data discovered by the hook.
    pub data: Signal<Vec<ScrollSpyData>>,

    /// Whether browser-side heading discovery has run.
    pub initialized: Signal<bool>,

    /// Re-query headings and reattach scroll listeners.
    pub reinitialize: Callback<()>,
}

/// Track headings in the current document and expose the active heading index.
pub fn use_scroll_spy(options: ScrollSpyOptions) -> ScrollSpyState {
    #[allow(unused_mut)]
    let mut active = use_signal(|| active_from_positions(&[]));
    #[allow(unused_mut)]
    let mut data = use_signal(|| options.initial_data.clone());
    #[allow(unused_mut)]
    let mut initialized = use_signal(|| false);
    let mut reinitialize_tick = use_signal(|| 0usize);

    let reinitialize = use_callback(move |()| {
        reinitialize_tick.with_mut(|tick| *tick += 1);
    });

    client! {
        let effect_options = options.clone();
        crate::use_effect_with_cleanup(move || {
            let _ = reinitialize_tick();
            let selector = effect_options.selector.clone();
            let offset = effect_options.offset;
            let host_selector = match &effect_options.scroll_host {
                ScrollSpyScrollHost::Window => None,
                ScrollSpyScrollHost::Selector(selector) => Some(selector.clone()),
            };

            let mut eval = document::eval(
                r#"
                const selector = await dioxus.recv();
                const offset = await dioxus.recv();
                const hostSelector = await dioxus.recv();
                const host = hostSelector ? document.querySelector(hostSelector) : window;
                if (!host) {
                    dioxus.send({ kind: "initialized", data: [], active: null });
                    return;
                }

                let headings = [];
                let frame = null;

                function headingDepth(element) {
                    const match = /^H([1-6])$/.exec(element.tagName || "");
                    return match ? Number(match[1]) : 0;
                }

                function readHeadings() {
                    const root = host === window ? document : host;
                    headings = Array.from(root.querySelectorAll(selector)).filter(
                        (element) => headingDepth(element) > 0
                    );
                    return headings.map((element, index) => ({
                        tag_name: element.tagName || "",
                        id: element.id || `dxc-scroll-spy-${index}`,
                        value: element.textContent || ""
                    }));
                }

                function activeIndex() {
                    if (headings.length === 0) {
                        return null;
                    }

                    const positions = [];
                    const hostTop = host === window ? 0 : host.getBoundingClientRect().top;
                    for (let index = 0; index < headings.length; index += 1) {
                        const position = headings[index].getBoundingClientRect().top - hostTop - offset;
                        positions.push(position);
                    }

                    return activeIndexFromPositions(positions);
                }

                function activeIndexFromPositions(positions) {
                    for (let index = positions.length - 1; index >= 0; index -= 1) {
                        if (positions[index] <= 0) {
                            return index;
                        }
                    }

                    return positions.length > 0 ? 0 : null;
                }

                function publish(kind) {
                    dioxus.send({
                        kind,
                        data: readHeadings(),
                        active: activeIndex()
                    });
                }

                publish("initialized");
                const onScroll = () => publish("scroll");
                host.addEventListener("scroll", onScroll, { passive: true });
                if (host !== window) {
                    window.addEventListener("resize", onScroll, { passive: true });
                }

                await dioxus.recv();
                host.removeEventListener("scroll", onScroll);
                if (host !== window) {
                    window.removeEventListener("resize", onScroll);
                }
                "#,
            );
            let _ = eval.send(selector);
            let _ = eval.send(offset);
            let _ = eval.send(host_selector);

            spawn(async move {
                while let Ok(message) = eval.recv::<ScrollSpyMessage>().await {
                    data.set(
                        message
                            .data
                            .into_iter()
                            .filter_map(scroll_spy_data_from_match)
                            .collect(),
                    );
                    active.set(message.active);
                    if message.kind == "initialized" {
                        initialized.set(true);
                    }
                }
            });

            move || {
                let _ = eval.send(true);
            }
        });
    }

    ScrollSpyState {
        active,
        data,
        initialized,
        reinitialize,
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ScrollSpyMessage {
    kind: String,
    data: Vec<ScrollSpyMatch>,
    active: Option<usize>,
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
#[derive(Deserialize)]
struct ScrollSpyMatch {
    tag_name: String,
    id: String,
    value: String,
}

pub(crate) fn active_from_positions(positions: &[f64]) -> Option<usize> {
    positions
        .iter()
        .rposition(|position| *position <= 0.0)
        .or(if positions.is_empty() { None } else { Some(0) })
}

#[cfg(test)]
pub(crate) fn positions_from_tops(heading_tops: &[f64], host_top: f64, offset: f64) -> Vec<f64> {
    heading_tops
        .iter()
        .map(|heading_top| heading_top - host_top - offset)
        .collect()
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
pub(crate) fn heading_depth(tag_name: &str) -> u8 {
    let mut chars = tag_name.chars();
    match (chars.next(), chars.next(), chars.next()) {
        (Some('h' | 'H'), Some(depth @ '1'..='6'), None) => depth as u8 - b'0',
        _ => 0,
    }
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
fn scroll_spy_data_from_match(matched: ScrollSpyMatch) -> Option<ScrollSpyData> {
    let depth = heading_depth(&matched.tag_name);
    (depth > 0).then_some(ScrollSpyData {
        id: matched.id,
        value: matched.value,
        depth,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_depth_reads_heading_tags() {
        assert_eq!(heading_depth("H1"), 1);
        assert_eq!(heading_depth("h4"), 4);
        assert_eq!(heading_depth("div"), 0);
        assert_eq!(heading_depth("H7"), 0);
    }

    #[test]
    fn selector_matches_can_filter_out_non_heading_tags() {
        let matches = [
            ScrollSpyMatch {
                tag_name: "div".to_string(),
                id: "ignored".to_string(),
                value: "Ignored".to_string(),
            },
            ScrollSpyMatch {
                tag_name: "H1".to_string(),
                id: "intro".to_string(),
                value: "Intro".to_string(),
            },
            ScrollSpyMatch {
                tag_name: "section".to_string(),
                id: "ignored-2".to_string(),
                value: "Ignored Again".to_string(),
            },
            ScrollSpyMatch {
                tag_name: "h3".to_string(),
                id: "details".to_string(),
                value: "Details".to_string(),
            },
        ];
        let data = matches
            .into_iter()
            .filter_map(scroll_spy_data_from_match)
            .collect::<Vec<_>>();

        assert_eq!(
            data,
            vec![
                ScrollSpyData {
                    id: "intro".to_string(),
                    value: "Intro".to_string(),
                    depth: 1,
                },
                ScrollSpyData {
                    id: "details".to_string(),
                    value: "Details".to_string(),
                    depth: 3,
                },
            ]
        );
    }

    #[test]
    fn non_heading_match_does_not_become_scroll_spy_data() {
        let matched = ScrollSpyMatch {
            tag_name: "nav".to_string(),
            id: "toc".to_string(),
            value: "Table of contents".to_string(),
        };

        assert_eq!(scroll_spy_data_from_match(matched), None);
    }

    #[test]
    fn active_from_positions_returns_none_for_empty_list() {
        assert_eq!(active_from_positions(&[]), None);
    }

    #[test]
    fn active_from_positions_returns_last_heading_at_or_above_offset_else_first_upcoming() {
        assert_eq!(active_from_positions(&[-120.0, -12.0, 80.0]), Some(1));
        assert_eq!(active_from_positions(&[20.0, -4.0, 6.0]), Some(1));
    }

    #[test]
    fn active_from_positions_prefers_latest_heading_at_or_above_offset() {
        assert_eq!(active_from_positions(&[-160.0, -20.0, 420.0]), Some(1));
    }

    #[test]
    fn active_from_positions_falls_back_to_first_upcoming_heading() {
        assert_eq!(active_from_positions(&[30.0, 180.0]), Some(0));
    }

    #[test]
    fn host_relative_positions_activate_later_heading_before_viewport_top() {
        let viewport_relative = positions_from_tops(&[40.0, 280.0, 500.0], 0.0, 88.0);
        let host_relative = positions_from_tops(&[40.0, 280.0, 500.0], 200.0, 88.0);

        assert_eq!(viewport_relative, vec![-48.0, 192.0, 412.0]);
        assert_eq!(host_relative, vec![-248.0, -8.0, 212.0]);
        assert_eq!(active_from_positions(&viewport_relative), Some(0));
        assert_eq!(active_from_positions(&host_relative), Some(1));
    }

    #[test]
    fn host_relative_positions_remove_first_heading_stall_during_scroll() {
        let heading_tops_by_frame = [
            [120.0, 360.0, 600.0],
            [40.0, 280.0, 520.0],
            [-20.0, 220.0, 460.0],
            [-80.0, 160.0, 400.0],
        ];

        let viewport_active = heading_tops_by_frame
            .iter()
            .map(|tops| active_from_positions(&positions_from_tops(tops, 0.0, 88.0)))
            .collect::<Vec<_>>();
        let host_active = heading_tops_by_frame
            .iter()
            .map(|tops| active_from_positions(&positions_from_tops(tops, 200.0, 88.0)))
            .collect::<Vec<_>>();

        assert_eq!(viewport_active, vec![Some(0), Some(0), Some(0), Some(0)]);
        assert_eq!(host_active, vec![Some(0), Some(1), Some(1), Some(1)]);
    }

    #[test]
    fn options_preserve_initial_data() {
        let data = vec![ScrollSpyData {
            id: "intro".to_string(),
            value: "Intro".to_string(),
            depth: 1,
        }];

        let options = ScrollSpyOptions {
            initial_data: data.clone(),
            ..Default::default()
        };

        assert_eq!(options.initial_data, data);
    }
}
