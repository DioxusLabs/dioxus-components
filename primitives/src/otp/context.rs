//! Shared state for the one-time-password components.

use dioxus::prelude::*;
use std::ops::Range;

#[derive(Clone, Copy)]
pub(super) struct OtpCtx {
    pub value: Memo<String>,
    pub disabled: ReadSignal<bool>,
    pub active_index: Memo<Option<usize>>,
    pub selected_range: Memo<Option<Range<usize>>>,
    pub begin_slot_selection: Callback<usize>,
    pub extend_slot_selection: Callback<usize>,
    pub end_slot_selection: Callback<Option<usize>>,
}
