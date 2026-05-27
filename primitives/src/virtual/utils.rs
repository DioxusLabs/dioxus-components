//! Utility functions for the virtual list implementation.

use std::ops::RangeInclusive;

use super::types::VirtualItem;

/// Binary search to find the nearest item at or before the given offset.
///
/// Returns the index of the item whose `start` position is closest to
/// (but not exceeding) the given offset.
pub(crate) fn find_nearest_binary_search(measurements: &[VirtualItem], offset: u32) -> usize {
    measurements
        .binary_search_by(|item| item.start().cmp(&offset))
        .unwrap_or_else(|idx| idx.saturating_sub(1))
}

/// Extract indices from a range with overscan applied.
pub(crate) fn default_range_extractor(
    range: std::ops::Range<usize>,
    overscan: usize,
    count: usize,
) -> RangeInclusive<usize> {
    if count == 0 {
        return 0..=0;
    }

    let start = range.start.saturating_sub(overscan);
    let end = (range.end + overscan).min(count - 1);

    start..=end
}
