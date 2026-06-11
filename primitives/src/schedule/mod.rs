//! Defines the [`Schedule`] component and supporting scheduling data types.

mod components;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod utils;

pub use components::{
    Schedule, ScheduleHeader, ScheduleHeaderProps, ScheduleViewButton, ScheduleViewButtonProps,
};
pub use types::{
    ScheduleAllDaySlotClick, ScheduleClassNames, ScheduleDateChange, ScheduleDayClick,
    ScheduleDayViewConfig, ScheduleDropDestination, ScheduleEvent, ScheduleEventClick,
    ScheduleEventCreate, ScheduleEventCreateSource, ScheduleEventDrag, ScheduleEventDrop,
    ScheduleEventRenderContext, ScheduleEventResize, ScheduleExternalDrop, ScheduleLabels,
    ScheduleLayout, ScheduleMobileMonthViewConfig, ScheduleMode, ScheduleMonthViewConfig,
    ScheduleProps, ScheduleRecurrence, ScheduleRecurrenceExpansionLimit,
    ScheduleRecurrenceFrequency, ScheduleResizeEdge, ScheduleSlotRangeSelection,
    ScheduleTimeGridConfig, ScheduleTimeSlotClick, ScheduleView, ScheduleViewChange,
    ScheduleWeekViewConfig, ScheduleYearViewConfig,
};
pub use utils::{add_months, shift_date, today};
