//! Virtual list implementation using Dioxus Store for fine-grained reactivity.
//!
//! This module provides the core algorithms needed for efficient list virtualization:
//!
//! - Computing item positions from measured or estimated sizes
//! - Calculating the visible range using binary search
//! - Handling scroll position corrections when items resize

pub(crate) mod types;
mod utils;
mod virtualizer;

pub(crate) use virtualizer::{
    cache_item_sizes, compute_measurements, get_total_size, get_virtual_items, scroll_anchor,
    set_scroll_offset, set_viewport_size, VirtualizerState, VirtualizerStateStoreExt,
};
