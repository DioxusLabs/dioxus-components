//! Defines the [`VirtualList`] component for rendering large lists with virtualization.

use std::collections::HashMap;

use dioxus::prelude::*;
use serde::Deserialize;

use crate::r#virtual::{
    compute_measurements, get_total_size, get_virtual_items, resize_item, set_scroll_offset,
    set_viewport_size, VirtualizerState, VirtualizerStateStoreExt,
};

/// The props for the [`VirtualList`] component.
#[derive(Props, Clone, PartialEq)]
pub struct VirtualListProps {
    /// The total number of items in the list.
    pub count: ReadSignal<usize>,
    /// The amount of render buffer (in estimated row counts) above and below the viewport.
    #[props(default = ReadSignal::new(Signal::new(8)))]
    pub buffer: ReadSignal<usize>,
    /// Estimates the height of an item by index (used before measurement).
    /// For best scrollbar stability, return values close to actual heights.
    /// If not provided, uses adaptive estimation based on measured items.
    pub estimate_size: Option<Callback<usize, u32>>,
    /// Renders a single item by its absolute index.
    pub render_item: Callback<usize, Element>,
    /// Additional attributes to apply to the container element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # VirtualList
///
/// The `VirtualList` component virtualizes a large list by rendering only the visible slice plus a
/// configurable buffer. It supports dynamic row heights and keeps total scroll height with a
/// virtual canvas.
///
/// Each rendered item receives `aria-setsize` and `aria-posinset` attributes for accessibility,
/// allowing screen readers to announce the total list size even though only a subset of items
/// is present in the DOM.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::virtual_list::VirtualList;
///
/// #[derive(Clone, PartialEq)]
/// struct Row {
///     title: String,
/// }
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         VirtualList {
///             count: 100usize,
///             buffer: 8usize,
///             // Optional: estimate height per item for smoother scrolling
///             // If omitted, uses adaptive estimation based on measured items
///             estimate_size: |_idx| 48,
///             render_item: move |idx: usize| rsx! {
///                 article { key: "{idx}", "Row {idx}" }
///             },
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`VirtualList`] component renders a container `div` with the class `dx-virtual-list-container`.
/// All user-provided `attributes` are spread onto the container element.
#[component]
pub fn VirtualList(props: VirtualListProps) -> Element {
    let VirtualListProps {
        count,
        buffer,
        estimate_size,
        render_item,
        attributes,
    } = props;

    let container_id = crate::use_unique_id();

    // Create the Store — only holds mutable shared state
    let state: Store<VirtualizerState> = use_store(|| VirtualizerState {
        scroll_offset: 0,
        viewport_size: 0,
        is_scrolling: false,
        item_size_cache: HashMap::new(),
        scroll_adjustments: 0,
        stable_total_size: None,
        stable_measurement_count: None,
        deferred_adjustments: 0,
    });

    // Measurements as a memo — recomputes when count or item_size_cache change.
    // Read (not peeked) by the render body so the component re-renders when the
    // memo invalidates; peeking a dirty memo returns stale data (Memo::peek does
    // not check the dirty flag), which can yield out-of-bounds indices when
    // `count` shrinks between renders.
    let measurements: Memo<Vec<crate::r#virtual::types::VirtualItem>> = use_memo(move || {
        let count = count();
        let isc = state.item_size_cache();
        let item_size_cache = isc.read();
        let estimate_cb = estimate_size.as_ref().map(|c| move |i: usize| c(i));
        compute_measurements(
            count,
            &item_size_cache,
            estimate_cb.as_ref().map(|f| f as &dyn Fn(usize) -> u32),
        )
    });

    // Subscribe to scroll events via JS bridge
    use_effect(move || {
        let script = r#"
            const container = document.getElementById(await dioxus.recv());
            if (!container) return;

            let scrollEndTimer = null;
            let lastOffset = null;
            let lastViewport = null;
            let lastIsScrolling = null;

            function publish(isScrolling) {
                const scroll = Math.round(container.scrollTop);
                const viewport = Math.min(container.clientHeight, window.innerHeight) || 600;
                // Deduplicate only if the full scroll state is unchanged.
                if (
                    scroll === lastOffset &&
                    viewport === lastViewport &&
                    isScrolling === lastIsScrolling
                ) {
                    return;
                }
                lastOffset = scroll;
                lastViewport = viewport;
                lastIsScrolling = isScrolling;
                dioxus.send({
                    offset: scroll,
                    viewport: viewport,
                    isScrolling: isScrolling
                });
            }

            function onScroll() {
                // Clear any pending scroll-end detection
                if (scrollEndTimer !== null) {
                    clearTimeout(scrollEndTimer);
                }

                // Send scroll event immediately (no RAF batching)
                // This ensures Rust receives the event before the next render
                publish(true);

                // Debounce scroll-end detection. Firefox in CI can take long
                // enough between scroll events and measurement reads that a
                // shorter timeout unfreezes the scroll canvas mid-scroll.
                scrollEndTimer = setTimeout(() => {
                    scrollEndTimer = null;
                    publish(false);
                }, 600);
            }

            // Initial publish
            publish(false);

            container.addEventListener("scroll", onScroll, { passive: true });
            window.addEventListener("resize", () => publish(false), { passive: true });

            await dioxus.recv();
            if (scrollEndTimer !== null) clearTimeout(scrollEndTimer);
            container.removeEventListener("scroll", onScroll);
        "#;
        let mut eval = document::eval(script);
        let _ = eval.send(container_id.peek().clone());

        spawn(async move {
            while let Ok(scroll_msg) = eval.recv::<ScrollMsg>().await {
                let scrolling = scroll_msg.is_scrolling;

                let correction = {
                    let m = measurements.peek();
                    set_scroll_offset(&state, &m, scroll_msg.offset, scrolling)
                };
                set_viewport_size(&state, scroll_msg.viewport);

                if let Some(delta) = correction {
                    let new_scroll = (scroll_msg.offset as i32 + delta).max(0) as u32;
                    sync_container_scroll(container_id.peek().clone(), new_scroll).await;
                    state.scroll_offset().set(new_scroll);
                }
            }
        });
    });

    let onresize = move |idx| {
        move |event: Event<ResizeData>| {
            let rect = event.data().get_content_box_size().unwrap_or_default();
            let measured = rect.height.max(1.0).round() as u32;

            let m = measurements.peek();
            let adjustment = resize_item(&state, &m, idx, measured);
            drop(m);

            if let Some(delta) = adjustment {
                let current = *state.scroll_offset().peek();
                let new_scroll = (current as i32 + delta).max(0) as u32;
                spawn(async move {
                    sync_container_scroll(container_id.peek().clone(), new_scroll).await;
                });
            }
        }
    };

    let m = measurements.read();
    let virtual_items = get_virtual_items(&state, &m, buffer());
    let total_height = get_total_size(&state, &m);

    let top_offset = virtual_items.first().map(|item| item.start()).unwrap_or(0);
    let canvas_height = total_height.max(*state.viewport_size().peek());
    let set_size = count.to_string();

    rsx! {
        div {
            id: container_id,
            role: "list",
            tabindex: "0",
            ..attributes,

            div {
                style: "position: relative; height:{canvas_height}px; width: 100%;",
                div {
                    style: "position: absolute; inset: 0 auto auto 0; width: 100%; transform: translateY({top_offset}px); will-change: transform;",
                    {virtual_items.iter().map(move |item| {
                        let idx = item.index();

                        rsx! {
                            div {
                                key: "{item.key()}",
                                role: "listitem",
                                "data-virtual-index": "{idx}",
                                "aria-setsize": "{set_size}",
                                "aria-posinset": "{idx + 1}",
                                onresize: onresize(idx),
                                {render_item(idx)}
                            }
                        }
                    })}
                }
            }
        }
    }
}

/// Parsed scroll message from JS bridge.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScrollMsg {
    offset: u32,
    viewport: u32,
    is_scrolling: bool,
}

async fn sync_container_scroll(container_id: String, scroll_top: u32) {
    let eval = document::eval(
        r#"
        const id = await dioxus.recv();
        const targetScroll = await dioxus.recv();
        const container = document.getElementById(id);
        if (container) {
            container.scrollTop = targetScroll;
        }
        "#,
    );
    let _ = eval.send(container_id);
    let _ = eval.send(scroll_top);
}
