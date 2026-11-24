//! Defines the [`VirtualList`] component for rendering large lists with virtualization.

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use dioxus::core::Task;
use dioxus::html::geometry::PixelsVector2D;
use dioxus::prelude::*;
use dioxus_sdk_time::sleep;

use crate::r#virtual::{
    cache_item_sizes, compute_measurements, get_total_size, get_virtual_items, scroll_anchor,
    set_scroll_offset, set_viewport_size, types::VirtualItem, VirtualizerState,
    VirtualizerStateStoreExt,
};

const SCROLL_IDLE: Duration = Duration::from_millis(600);

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
        stable_total_size: None,
        stable_measurement_count: None,
    });

    // Measurements as a memo — recomputes when count or item_size_cache change.
    // Read (not peeked) by the render body so the component re-renders when the
    // memo invalidates; peeking a dirty memo returns stale data (Memo::peek does
    // not check the dirty flag), which can yield out-of-bounds indices when
    // `count` shrinks between renders.
    let measurements: Memo<Vec<VirtualItem>> = use_memo(move || {
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

    let mut container = use_signal(|| None::<Rc<MountedData>>);
    let container_height = use_signal(|| 0);
    let window_height = use_signal(|| 0);
    let mut idle_task = use_signal(|| None::<Task>);

    use_effect(move || {
        set_viewport_size(&state, container_height().min(window_height()));
    });

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
            onmounted: move |event| {
                container.set(Some(event.data()));
                async move {
                    if let Ok(offset) = event.data().get_scroll_offset().await {
                        if !*state.is_scrolling().peek() {
                            state.scroll_offset().set(offset.y.round() as u32);
                        }
                    }
                }
            },
            onresize: move |event| {
                if let Ok(size) = event.data().get_content_box_size() {
                    set_height(container_height, size.height);
                }
            },
            onscroll: move |event| {
                let offset = event.data().scroll_top().round() as u32;
                if offset == *state.scroll_offset().peek() {
                    return;
                }
                set_height(container_height, f64::from(event.data().client_height()));
                set_scroll_offset(&state, &measurements.peek(), offset, true);
                let previous_task = idle_task.take();
                if let Some(previous_task) = previous_task {
                    previous_task.cancel();
                }
                idle_task.set(Some(spawn(async move {
                    sleep(SCROLL_IDLE).await;
                    idle_task.set(None);
                    let current = *state.scroll_offset().peek();
                    set_scroll_offset(&state, &measurements.peek(), current, false);
                })));
            },
            ..attributes,

            div {
                style: "position: relative; height:{canvas_height}px; width: 100%; overflow-anchor:none;",
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
                                onresize: move |event| async move {
                                    let Ok(size) = event.data().get_content_box_size() else {
                                        return;
                                    };
                                    let mounted = container.peek().clone();
                                    if let Some(mounted) = mounted {
                                        let size = size.height.max(1.0).round() as u32;
                                        measure_rows(state, measurements, &mounted, [(idx, size)]).await;
                                    }
                                },
                                {render_item(idx)}
                            }
                        }
                    })}
                }
            }
            div {
                aria_hidden: "true",
                style: "position:absolute; top:0; left:0; width:0; height:0; overflow:hidden;",
                div {
                    style: "height:100vh;",
                    onresize: move |event| {
                        if let Ok(size) = event.data().get_content_box_size() {
                            set_height(window_height, size.height);
                        }
                    },
                }
            }
        }
    }
}

fn set_height(mut height: Signal<u32>, value: f64) {
    let value = value.round() as u32;
    if *height.peek() != value {
        height.set(value);
    }
}

async fn measure_rows(
    state: Store<VirtualizerState>,
    measurements: Memo<Vec<VirtualItem>>,
    container: &MountedData,
    sizes: impl IntoIterator<Item = (usize, u32)>,
) {
    let current_y = *state.scroll_offset().peek();
    let anchor = {
        let before = measurements.read();
        let anchor = scroll_anchor(&before, current_y);
        if !cache_item_sizes(&state, &before, sizes) {
            return;
        }
        anchor
    };
    let Some((index, old_start)) = anchor else {
        return;
    };
    let Some(new_start) = measurements.read().get(index).map(VirtualItem::start) else {
        return;
    };
    if old_start == new_start {
        return;
    }
    if let Ok(offset) = container.get_scroll_offset().await {
        if offset.y.round() as u32 != current_y || *state.scroll_offset().peek() != current_y {
            return;
        }
        let target_y = offset.y + f64::from(new_start) - f64::from(old_start);
        if container
            .scroll(
                PixelsVector2D::new(offset.x, target_y.max(0.0)),
                ScrollBehavior::Instant,
            )
            .await
            .is_ok()
        {
            // Blitz does not dispatch a scroll event for programmatic scrolls. Read the
            // position back because the renderer may clamp the requested offset.
            if let Ok(actual) = container.get_scroll_offset().await {
                if *state.scroll_offset().peek() == current_y {
                    state.scroll_offset().set(actual.y.round() as u32);
                }
            }
        }
    }
}
