//! Defines the [`VirtualList`] component for rendering large lists with virtualization.

use std::collections::HashMap;

use dioxus::prelude::*;
use wasm_bindgen::JsCast;

use crate::{
    r#virtual::{
        compute_measurements, get_total_size, get_virtual_items, resize_item, set_scroll_offset,
        set_viewport_size, VirtualizerState, VirtualizerStateStoreExt,
    },
    use_effect_with_cleanup, use_task_spawner, EventListener, Timeout,
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

    let last_scroll_msg = use_hook(|| CopyValue::new(None::<ScrollMsg>));
    let mut scroll_end_timer = use_hook(|| CopyValue::new(None::<Timeout>));
    let task_spawner = use_task_spawner();

    // Subscribe to scroll events through web-sys so scroll state updates before the next render.
    use_effect_with_cleanup(move || {
        let id = container_id.peek().clone();
        let Some(container) = scroll_container(&id) else {
            return Box::new(|| {}) as Box<dyn FnOnce()>;
        };

        publish_scroll_state(&id, false, state, measurements, last_scroll_msg);

        let scroll_id = id.clone();
        let mut scroll_timer = scroll_end_timer;
        let scroll_task_spawner = task_spawner.clone();
        let scroll_listener = EventListener::new(&container, "scroll", move |_| {
            scroll_task_spawner.clone().run(|| {
                if let Some(timer) = scroll_timer.take() {
                    drop(timer);
                }

                publish_scroll_state(&scroll_id, true, state, measurements, last_scroll_msg);

                // Firefox in CI can take long enough between scroll events and
                // measurement reads that a shorter timeout unfreezes the scroll
                // canvas mid-scroll.
                let timeout_id = scroll_id.clone();
                scroll_timer.set(Some(Timeout::new(
                    scroll_task_spawner.clone(),
                    600,
                    move || {
                        publish_scroll_state(
                            &timeout_id,
                            false,
                            state,
                            measurements,
                            last_scroll_msg,
                        );
                    },
                )));
            });
        });

        let resize_listener = web_sys::window().map(|window| {
            let resize_id = id.clone();
            let resize_task_spawner = task_spawner.clone();
            EventListener::new(&window, "resize", move |_| {
                resize_task_spawner.clone().run(|| {
                    publish_scroll_state(&resize_id, false, state, measurements, last_scroll_msg);
                });
            })
        });

        Box::new(move || {
            drop(scroll_listener);
            drop(resize_listener);
            if let Some(timer) = scroll_end_timer.take() {
                drop(timer);
            }
        })
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
                let id = container_id.peek().clone();
                sync_container_scroll(&id, new_scroll);
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

/// Scroll message from the DOM listener bridge.
#[derive(Clone, Copy, PartialEq, Eq)]
struct ScrollMsg {
    offset: u32,
    viewport: u32,
    is_scrolling: bool,
}

fn scroll_container(container_id: &str) -> Option<web_sys::HtmlElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id(container_id)?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()
}

fn read_scroll_msg(container_id: &str, is_scrolling: bool) -> Option<ScrollMsg> {
    let container = scroll_container(container_id)?;
    let offset = container.scroll_top().max(0) as u32;
    let container_height = container.client_height().max(0) as u32;
    let window_height = web_sys::window()
        .and_then(|window| window.inner_height().ok())
        .and_then(|height| height.as_f64())
        .map(|height| height.max(0.0).round() as u32)
        .unwrap_or(container_height);
    let viewport = container_height.min(window_height);

    Some(ScrollMsg {
        offset,
        viewport: if viewport == 0 { 600 } else { viewport },
        is_scrolling,
    })
}

fn publish_scroll_state(
    container_id: &str,
    is_scrolling: bool,
    state: Store<VirtualizerState>,
    measurements: Memo<Vec<crate::r#virtual::types::VirtualItem>>,
    mut last_scroll_msg: CopyValue<Option<ScrollMsg>>,
) {
    let Some(scroll_msg) = read_scroll_msg(container_id, is_scrolling) else {
        return;
    };

    if last_scroll_msg.cloned() == Some(scroll_msg) {
        return;
    }
    last_scroll_msg.set(Some(scroll_msg));

    let correction = {
        let m = measurements.peek();
        set_scroll_offset(&state, &m, scroll_msg.offset, scroll_msg.is_scrolling)
    };
    set_viewport_size(&state, scroll_msg.viewport);

    if let Some(delta) = correction {
        let new_scroll = (scroll_msg.offset as i32 + delta).max(0) as u32;
        sync_container_scroll(container_id, new_scroll);
        state.scroll_offset().set(new_scroll);
    }
}

fn sync_container_scroll(container_id: &str, scroll_top: u32) {
    if let Some(container) = scroll_container(container_id) {
        container.set_scroll_top(scroll_top as i32);
    }
}
