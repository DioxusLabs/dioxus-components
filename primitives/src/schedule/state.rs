use time::PrimitiveDateTime;

use super::types::{ScheduleEvent, ScheduleMode, ScheduleResizeEdge};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ScheduleCapabilities {
    pub(crate) events_drag_and_drop: bool,
    pub(crate) drag_slot_select: bool,
    pub(crate) event_resize: bool,
}

impl ScheduleCapabilities {
    pub(crate) fn new(
        mode: ScheduleMode,
        drag_drop: bool,
        slot_select: bool,
        resize: bool,
    ) -> Self {
        Self {
            events_drag_and_drop: mode != ScheduleMode::Static && drag_drop,
            drag_slot_select: slot_select,
            event_resize: mode != ScheduleMode::Static && resize,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ScheduleSlotSelectionState {
    pub(crate) anchor: PrimitiveDateTime,
    pub(crate) current: PrimitiveDateTime,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ScheduleResizeState {
    pub(crate) event: ScheduleEvent,
    pub(crate) edge: ScheduleResizeEdge,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ScheduleSlotRange {
    pub(crate) start: PrimitiveDateTime,
    pub(crate) end: PrimitiveDateTime,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ScheduleExternalData {
    pub(crate) format: String,
    pub(crate) data: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ResizedEventTimes {
    pub(crate) new_start: PrimitiveDateTime,
    pub(crate) new_end: PrimitiveDateTime,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LaidOutEvent {
    pub(crate) event: ScheduleEvent,
    pub(crate) column: usize,
    pub(crate) columns: usize,
}
