//! Low-level virtualizer primitives using Dioxus Store for fine-grained reactivity.
//!
//! This module provides the core algorithms needed for efficient list virtualization:
//!
//! - Computing item positions from measured or estimated sizes
//! - Calculating the visible range using binary search
//! - Handling scroll position corrections when items resize
//!
//! These APIs intentionally do not render any accessibility roles, keyboard behavior, focus
//! management, or scroll container DOM. Higher-level components such as virtualized listboxes must
//! provide those semantics themselves.

pub mod types;
mod utils;
mod virtualizer;

pub use virtualizer::{
    compute_measurements, get_total_size, get_virtual_items, resize_item, set_scroll_offset,
    set_viewport_size, VirtualizerState, VirtualizerStateStoreExt,
};
