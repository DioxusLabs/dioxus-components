//! Core Virtualizer implementation using Dioxus Store for fine-grained reactivity.

use std::collections::HashMap;
use std::ops::Range;

use dioxus::prelude::*;

use super::types::{Key, VirtualItem};
use super::utils::{default_range_extractor, find_nearest_binary_search};

/// Reactive virtualizer state.
///
/// Only holds mutable state shared between event handlers and the render body.
/// Prop-derived values live outside the Store and are read directly from signals.
#[derive(Clone, PartialEq, Store)]
pub struct VirtualizerState {
    // --- Reactive (`.read()` in render body → triggers re-renders) ---
    /// Current scroll offset from the container's `scrollTop`.
    pub scroll_offset: u32,
    /// Current viewport height.
    pub viewport_size: u32,
    /// Whether the user is actively scrolling.
    pub is_scrolling: bool,

    // --- Cache (`.peek()` only → never triggers re-renders) ---
    /// Measured sizes keyed by item key, populated by resize callbacks.
    pub item_size_cache: HashMap<Key, u32>,

    /// Frozen total size during active scrolling to prevent scrollbar drift.
    pub stable_total_size: Option<u32>,
    /// Item count the frozen total size was calculated for.
    pub stable_measurement_count: Option<usize>,
}

// ---------------------------------------------------------------------------
// Public API – free functions operating on Store<VirtualizerState>
// ---------------------------------------------------------------------------

/// Compute item measurements from current state.
///
/// This is a pure function suitable for use inside a `use_memo`.
/// Adaptive estimation (average of measured sizes) is derived directly
/// from `item_size_cache` rather than tracked separately.
///
/// If `estimate` is provided, it is always used for unmeasured items.
/// Otherwise, falls back to the adaptive average of measured sizes, or 100px.
pub fn compute_measurements(
    count: usize,
    item_size_cache: &HashMap<Key, u32>,
    estimate: Option<&dyn Fn(usize) -> u32>,
) -> Vec<VirtualItem> {
    let adaptive_size = if estimate.is_none() && !item_size_cache.is_empty() {
        let sum: u64 = item_size_cache.values().map(|&v| v as u64).sum();
        Some((sum / item_size_cache.len() as u64) as u32)
    } else {
        None
    };

    let mut measurements = Vec::with_capacity(count);
    for i in 0..count {
        let key = i;
        let size = item_size_cache.get(&key).copied().unwrap_or_else(|| {
            if let Some(est) = estimate {
                est(i)
            } else {
                adaptive_size.unwrap_or(100)
            }
        });

        let start = measurements
            .last()
            .map(|m: &VirtualItem| m.end())
            .unwrap_or(0);
        measurements.push(VirtualItem::new(key, i, start, size));
    }
    measurements
}

/// Handle a scroll event.  Writes reactive fields (`scroll_offset`,
/// `is_scrolling`) which trigger component re-renders.
///
pub fn set_scroll_offset(
    state: &Store<VirtualizerState>,
    measurements: &[VirtualItem],
    offset: u32,
    is_scrolling: bool,
) {
    let was_scrolling = *state.is_scrolling().peek();

    if is_scrolling && !was_scrolling {
        // Freeze total size when scrolling starts to prevent scrollbar drift
        let total = calculate_total_size(measurements);
        state.stable_total_size().set(Some(total));
        state
            .stable_measurement_count()
            .set(Some(measurements.len()));
    }

    if !is_scrolling && was_scrolling {
        state.stable_total_size().set(None);
        state.stable_measurement_count().set(None);
    }

    // Reactive writes – these trigger re-renders.
    state.scroll_offset().set(offset);
    state.is_scrolling().set(is_scrolling);
}

/// Set the viewport size. Reactive write.
pub fn set_viewport_size(state: &Store<VirtualizerState>, size: u32) {
    if *state.viewport_size().peek() != size {
        state.viewport_size().set(size);
    }
}

/// Cache measured sizes, returning whether the measurements changed.
pub fn cache_item_sizes(
    state: &Store<VirtualizerState>,
    measurements: &[VirtualItem],
    sizes: impl IntoIterator<Item = (usize, u32)>,
) -> bool {
    let mut cache = state.item_size_cache();
    let changed: Vec<_> = {
        let cached = cache.peek();
        sizes
            .into_iter()
            .filter_map(|(index, size)| {
                let key = measurements.get(index)?.key();
                if cached.get(&key).is_some_and(|old| old.abs_diff(size) <= 2) {
                    return None;
                }
                Some((key, size))
            })
            .collect()
    };
    if changed.is_empty() {
        return false;
    }
    cache.write().extend(changed);
    true
}

pub fn scroll_anchor(measurements: &[VirtualItem], offset: u32) -> Option<(usize, u32)> {
    let index = find_nearest_binary_search(measurements, offset);
    measurements.get(index).map(|item| (index, item.start()))
}

/// Return the virtual items to render.
///
/// **This is meant to be called in the render body.**  It `.read()`s
/// `scroll_offset` and `viewport_size` to subscribe the component.
pub fn get_virtual_items(
    state: &Store<VirtualizerState>,
    measurements: &[VirtualItem],
    overscan: usize,
) -> Vec<VirtualItem> {
    let range = match calculate_range(state, measurements) {
        Some(r) => r,
        None => return Vec::new(),
    };

    let count = measurements.len();
    let indexes = default_range_extractor(range, overscan, count);

    indexes
        .into_iter()
        .filter_map(|i| measurements.get(i).cloned())
        .collect()
}

/// Return the total scrollable size.
///
/// During active scrolling returns a frozen value to prevent scrollbar drift.
pub fn get_total_size(state: &Store<VirtualizerState>, measurements: &[VirtualItem]) -> u32 {
    let stable_measurement_count = *state.stable_measurement_count().peek();
    if stable_measurement_count == Some(measurements.len()) {
        if let Some(stable) = *state.stable_total_size().peek() {
            return stable;
        }
    }
    calculate_total_size(measurements)
}

fn get_scroll_offset_for_measurements(
    scroll_offset: u32,
    viewport_size: u32,
    measurements: &[VirtualItem],
) -> u32 {
    let max_scroll_offset = calculate_total_size(measurements).saturating_sub(viewport_size);
    scroll_offset.min(max_scroll_offset)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Calculate the visible range.
///
/// `.read()`s `scroll_offset` and `viewport_size` (reactive subscription).
fn calculate_range(
    state: &Store<VirtualizerState>,
    measurements: &[VirtualItem],
) -> Option<Range<usize>> {
    // Reactive reads – subscribes the calling component.
    let scroll_offset = *state.scroll_offset().read();
    let viewport_size = *state.viewport_size().read();

    if measurements.is_empty() || viewport_size == 0 {
        return None;
    }

    if measurements.len() <= 1 {
        return Some(0..measurements.len());
    }

    let scroll_offset =
        get_scroll_offset_for_measurements(scroll_offset, viewport_size, measurements);
    let start_index = find_nearest_binary_search(measurements, scroll_offset);
    let mut end_index = start_index;
    let last_index = measurements.len() - 1;

    while end_index < last_index && measurements[end_index].end() < scroll_offset + viewport_size {
        end_index += 1;
    }

    Some(start_index..(end_index + 1))
}

/// Calculate total size from measurements.
fn calculate_total_size(measurements: &[VirtualItem]) -> u32 {
    measurements.last().map(|m| m.end()).unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::rc::Rc;

    /// Run a closure inside a Dioxus runtime context so that Store/CopyValue
    /// APIs are available.  The closure runs inside a component render.
    fn with_runtime(f: impl Fn() + 'static) {
        let result = Rc::new(Cell::new(false));
        let result2 = result.clone();
        let test_fn = Rc::new(f);
        let mut dom = VirtualDom::new_with_props(
            |props: TestHarnessProps| {
                (props.test_fn)();
                props.result.set(true);
                rsx! { div {} }
            },
            TestHarnessProps {
                test_fn,
                result: result2,
            },
        );
        dom.rebuild_in_place();
        assert!(result.get(), "Test component did not run");
    }

    #[derive(Clone, Props)]
    struct TestHarnessProps {
        test_fn: Rc<dyn Fn()>,
        result: Rc<Cell<bool>>,
    }

    impl PartialEq for TestHarnessProps {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    fn create_test_state() -> Store<VirtualizerState> {
        Store::new(VirtualizerState {
            scroll_offset: 0,
            viewport_size: 600,
            is_scrolling: false,
            item_size_cache: HashMap::new(),
            stable_total_size: None,
            stable_measurement_count: None,
        })
    }

    fn make_measurements(state: &Store<VirtualizerState>) -> Vec<VirtualItem> {
        let isc = state.item_size_cache();
        let cache = isc.peek();
        compute_measurements(100, &cache, Some(&|_| 50))
    }

    #[test]
    fn test_stable_total_size_ignored_when_count_changes() {
        with_runtime(|| {
            let state = create_test_state();
            let m = make_measurements(&state);
            set_scroll_offset(&state, &m, 1000, true);

            assert_eq!(get_total_size(&state, &m), 5000);

            let smaller = compute_measurements(4, &HashMap::new(), Some(&|_| 50));
            assert_eq!(get_total_size(&state, &smaller), 200);
        });
    }

    #[test]
    fn test_range_clamps_stale_scroll_offset_after_count_shrinks() {
        with_runtime(|| {
            let state = create_test_state();
            let smaller = compute_measurements(4, &HashMap::new(), Some(&|_| 50));
            state.scroll_offset().set(10_000);
            state.viewport_size().set(600);

            let virtual_items = get_virtual_items(&state, &smaller, 0);
            let indexes: Vec<_> = virtual_items.iter().map(VirtualItem::index).collect();

            assert_eq!(indexes, vec![0, 1, 2, 3]);
        });
    }

    #[test]
    fn test_resize_item_below_viewport() {
        with_runtime(|| {
            let state = create_test_state();
            let m = make_measurements(&state);
            set_scroll_offset(&state, &m, 0, false);

            let m = make_measurements(&state);
            assert!(cache_item_sizes(&state, &m, [(50, 100)]));
            assert_eq!(
                scroll_anchor(&m, 0),
                scroll_anchor(&make_measurements(&state), 0)
            );
        });
    }

    #[test]
    fn test_resize_item_above_viewport() {
        with_runtime(|| {
            let state = create_test_state();
            let m = make_measurements(&state);
            set_scroll_offset(&state, &m, 1000, false);

            let m = make_measurements(&state);
            let (index, start) = scroll_anchor(&m, 1000).unwrap();
            assert!(cache_item_sizes(&state, &m, [(5, 100)]));
            assert_eq!(make_measurements(&state)[index].start() - start, 50);
        });
    }

    #[test]
    fn test_measurement_batch_keeps_the_current_anchor_during_scrolling() {
        with_runtime(|| {
            let state = create_test_state();

            let m = make_measurements(&state);
            set_scroll_offset(&state, &m, 1000, true);

            let m = make_measurements(&state);
            let (index, start) = scroll_anchor(&m, 1000).unwrap();
            assert!(cache_item_sizes(&state, &m, [(5, 100), (3, 80)]));
            let m = make_measurements(&state);
            assert_eq!(
                m[index].start() - start,
                80,
                "Both sizes changed above the current anchor"
            );
            set_scroll_offset(&state, &m, 1080, true);
            set_scroll_offset(&state, &m, 200, true);
            set_scroll_offset(&state, &m, 200, false);
            assert_eq!(*state.scroll_offset().peek(), 200);
        });
    }

    #[test]
    fn test_adaptive_measurement_accounts_for_unmeasured_rows_above_anchor() {
        with_runtime(|| {
            let state = create_test_state();

            let m = compute_measurements(100, &HashMap::new(), None);
            set_scroll_offset(&state, &m, 2010, true);
            let (index, start) = scroll_anchor(&m, 2010).unwrap();
            assert!(cache_item_sizes(&state, &m, [(20, 120), (50, 60)]));
            let updated = compute_measurements(100, &state.item_size_cache().peek(), None);
            assert_eq!((index, start), (20, 2000));
            assert_eq!(updated[index].start(), 1800);
            assert_eq!(scroll_anchor(&updated, 1810), Some((20, 1800)));
        });
    }
}
