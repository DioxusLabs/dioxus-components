use dioxus::prelude::*;
use time::{Date, PrimitiveDateTime, Weekday};

use crate::schedule::utils::today;

/// The visible schedule view.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScheduleView {
    /// A single-day time grid.
    Day,
    /// A seven-day time grid.
    #[default]
    Week,
    /// A month grid.
    Month,
    /// A year grid of months.
    Year,
}

impl ScheduleView {
    /// Returns the stable lowercase identifier used by schedule view controls and data attributes.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Year => "year",
        }
    }
}

/// The interaction mode for the schedule.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScheduleMode {
    /// Enables configured interactive behavior.
    #[default]
    Default,
    /// Disables drag/drop and resize behavior even when those flags are enabled.
    Static,
}

/// The schedule layout strategy.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScheduleLayout {
    /// Render a single desktop-oriented view container.
    #[default]
    Default,
    /// Render desktop and mobile containers with data attributes for responsive CSS.
    Responsive,
}

/// Recurrence frequency for a scheduled event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleRecurrenceFrequency {
    /// Repeat every day.
    Daily,
    /// Repeat every week.
    Weekly,
    /// Repeat every month, clamping invalid day-of-month values.
    Monthly,
    /// Repeat every year, clamping invalid day-of-month values.
    Yearly,
}

/// A recurrence rule attached to a [`ScheduleEvent`].
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleRecurrence {
    /// The recurrence frequency.
    pub frequency: ScheduleRecurrenceFrequency,
    /// The number of frequency units between occurrences. Values below one are treated as one.
    pub interval: u32,
    /// Stop expanding after this many occurrences, including the original event.
    pub count: Option<usize>,
    /// Stop expanding after occurrences that start after this date/time.
    pub until: Option<PrimitiveDateTime>,
}

/// A bounded recurrence expansion shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduleRecurrenceExpansionLimit {
    /// Maximum number of occurrences to create per recurring event.
    pub max_occurrences: usize,
}

impl Default for ScheduleRecurrenceExpansionLimit {
    fn default() -> Self {
        Self {
            max_occurrences: 128,
        }
    }
}

/// A schedule event.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleEvent {
    /// Stable event id.
    pub id: String,
    /// Event title displayed by the default renderer.
    pub title: String,
    /// Event start date/time.
    pub start: PrimitiveDateTime,
    /// Event end date/time.
    pub end: PrimitiveDateTime,
    /// Whether the event is an all-day event.
    pub all_day: bool,
    /// Optional color token exposed as `data-color`.
    pub color: Option<String>,
    /// Optional description exposed as an accessible title.
    pub description: Option<String>,
    /// Optional recurrence rule.
    pub recurrence: Option<ScheduleRecurrence>,
    /// Whether the event should reject drag/drop behavior.
    pub drag_disabled: bool,
    /// Whether the event should reject resize behavior.
    pub resize_disabled: bool,
}

/// Text labels used by the schedule.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleLabels {
    /// Previous date-range navigation label.
    pub previous: String,
    /// Next date-range navigation label.
    pub next: String,
    /// Today navigation label.
    pub today: String,
    /// Day view label.
    pub day: String,
    /// Week view label.
    pub week: String,
    /// Month view label.
    pub month: String,
    /// Year view label.
    pub year: String,
    /// All-day region label.
    pub all_day: String,
    /// Empty slot label.
    pub empty_slot: String,
}

impl Default for ScheduleLabels {
    fn default() -> Self {
        Self {
            previous: "Previous".to_string(),
            next: "Next".to_string(),
            today: "Today".to_string(),
            day: "Day".to_string(),
            week: "Week".to_string(),
            month: "Month".to_string(),
            year: "Year".to_string(),
            all_day: "All day".to_string(),
            empty_slot: "No events".to_string(),
        }
    }
}

/// Configuration shared by time-grid views.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleTimeGridConfig {
    /// Whether to render the default header.
    pub with_default_header: bool,
    /// First visible hour.
    pub start_hour: u8,
    /// Last visible hour.
    pub end_hour: u8,
    /// Slot size in minutes.
    pub slot_minutes: u8,
}

impl Default for ScheduleTimeGridConfig {
    fn default() -> Self {
        Self {
            with_default_header: true,
            start_hour: 0,
            end_hour: 23,
            slot_minutes: 60,
        }
    }
}

/// Day view configuration.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScheduleDayViewConfig {
    /// Time-grid configuration for the view.
    pub time_grid: ScheduleTimeGridConfig,
}

/// Week view configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleWeekViewConfig {
    /// Time-grid configuration for the view.
    pub time_grid: ScheduleTimeGridConfig,
    /// First day of the week.
    pub first_day_of_week: Weekday,
}

impl Default for ScheduleWeekViewConfig {
    fn default() -> Self {
        Self {
            time_grid: ScheduleTimeGridConfig::default(),
            first_day_of_week: Weekday::Sunday,
        }
    }
}

/// Month view configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleMonthViewConfig {
    /// Whether to render the default header.
    pub with_default_header: bool,
    /// First day of the week.
    pub first_day_of_week: Weekday,
}

impl Default for ScheduleMonthViewConfig {
    fn default() -> Self {
        Self {
            with_default_header: true,
            first_day_of_week: Weekday::Sunday,
        }
    }
}

/// Year view configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleYearViewConfig {
    /// Whether to render the default header.
    pub with_default_header: bool,
}

impl Default for ScheduleYearViewConfig {
    fn default() -> Self {
        Self {
            with_default_header: true,
        }
    }
}

/// Mobile month view configuration used by responsive layouts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleMobileMonthViewConfig {
    /// Whether to render the default header.
    pub with_default_header: bool,
}

impl Default for ScheduleMobileMonthViewConfig {
    fn default() -> Self {
        Self {
            with_default_header: true,
        }
    }
}

/// Event render context passed to custom render callbacks.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleEventRenderContext {
    /// The expanded event occurrence to render.
    pub event: ScheduleEvent,
    /// The active schedule view.
    pub view: ScheduleView,
    /// The date represented by the containing slot or cell.
    pub date: Date,
    /// Whether drag affordances are active for the event.
    pub draggable: bool,
    /// Whether resize affordances are active for the event.
    pub resizable: bool,
}

/// Date change callback payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleDateChange {
    /// Previous date.
    pub previous: Date,
    /// Next date.
    pub next: Date,
    /// Active view when the date changed.
    pub view: ScheduleView,
}

/// View change callback payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleViewChange {
    /// Previous view.
    pub previous: ScheduleView,
    /// Next view.
    pub next: ScheduleView,
    /// Active date when the view changed.
    pub date: Date,
}

/// Time slot click callback payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleTimeSlotClick {
    /// Slot start.
    pub start: PrimitiveDateTime,
    /// Slot end.
    pub end: PrimitiveDateTime,
    /// Containing date.
    pub date: Date,
    /// Active view.
    pub view: ScheduleView,
}

/// All-day slot click callback payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleAllDaySlotClick {
    /// Slot date.
    pub date: Date,
    /// Active view.
    pub view: ScheduleView,
}

/// Day click callback payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleDayClick {
    /// Clicked date.
    pub date: Date,
    /// Active view.
    pub view: ScheduleView,
}

/// Interaction source that requested a new schedule event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleEventCreateSource {
    /// A timed slot was clicked.
    TimeSlotClick,
    /// A timed slot range was selected by dragging.
    TimeSlotDrag,
    /// An all-day slot was clicked.
    AllDaySlotClick,
    /// A day cell or day header was clicked.
    DayClick,
}

/// Event creation callback payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleEventCreate {
    /// Requested event start.
    pub start: PrimitiveDateTime,
    /// Requested event end.
    pub end: PrimitiveDateTime,
    /// Containing date.
    pub date: Date,
    /// Whether the requested event is all-day.
    pub all_day: bool,
    /// Active view.
    pub view: ScheduleView,
    /// Interaction source that requested creation.
    pub source: ScheduleEventCreateSource,
}

/// Event click callback payload.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleEventClick {
    /// Clicked event occurrence.
    pub event: ScheduleEvent,
    /// Active view.
    pub view: ScheduleView,
}

/// Event drag lifecycle callback payload.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleEventDrag {
    /// Dragged event occurrence.
    pub event: ScheduleEvent,
    /// Active view.
    pub view: ScheduleView,
}

/// Destination surface for an event drop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleDropDestination {
    /// The event was dropped into the all-day row.
    AllDay,
    /// The event was dropped into a timed slot or other non-all-day surface.
    Timed,
}

/// Event drop callback payload.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleEventDrop {
    /// Dropped event id.
    pub event_id: String,
    /// Original event occurrence.
    pub event: ScheduleEvent,
    /// Destination surface that accepted the drop.
    pub destination: ScheduleDropDestination,
    /// New start.
    pub new_start: PrimitiveDateTime,
    /// New end.
    pub new_end: PrimitiveDateTime,
    /// Target date.
    pub date: Date,
    /// Active view.
    pub view: ScheduleView,
}

/// External item drop callback payload.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleExternalDrop {
    /// Browser-provided data transfer payload when available.
    ///
    /// Dioxus exposes the browser `DataTransfer` strings that the drag source
    /// allows for the current drop. The schedule checks common formats in this
    /// order: `application/json`, `text/plain`, `text/uri-list`, and `text/html`.
    /// Browser security rules and non-web renderers can still make external
    /// drops unavailable; in that case this field is `None`.
    pub data: Option<String>,
    /// The MIME type that produced [`ScheduleExternalDrop::data`].
    pub data_format: Option<String>,
    /// Target start.
    pub start: PrimitiveDateTime,
    /// Target end.
    pub end: PrimitiveDateTime,
    /// Target date.
    pub date: Date,
    /// Active view.
    pub view: ScheduleView,
}

/// Slot range selection callback payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleSlotRangeSelection {
    /// Selected range start.
    pub start: PrimitiveDateTime,
    /// Selected range end.
    pub end: PrimitiveDateTime,
    /// Active view.
    pub view: ScheduleView,
}

/// Edge used during event resize.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleResizeEdge {
    /// Resize from the event start.
    Start,
    /// Resize from the event end.
    End,
}

/// Event resize callback payload.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleEventResize {
    /// Resized event id.
    pub event_id: String,
    /// Original event occurrence.
    pub event: ScheduleEvent,
    /// New start.
    pub new_start: PrimitiveDateTime,
    /// New end.
    pub new_end: PrimitiveDateTime,
    /// Resized edge.
    pub edge: ScheduleResizeEdge,
    /// Active view.
    pub view: ScheduleView,
}

/// Class names applied to schedule surfaces.
///
/// The primitive also exposes state through `data-*` attributes. These classes
/// provide an API-level styling surface for consumers that prefer stable class
/// hooks over attribute selectors.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScheduleClassNames {
    /// Class applied to the desktop view container.
    pub desktop_view: String,
    /// Class applied to the responsive mobile view container.
    pub mobile_view: String,
    /// Class applied to day view sections.
    pub day_view: String,
    /// Class applied to week view sections.
    pub week_view: String,
    /// Class applied to month view sections.
    pub month_view: String,
    /// Class applied to year view sections.
    pub year_view: String,
    /// Class applied to mobile month view sections.
    pub mobile_month_view: String,
    /// Class applied to time slot buttons.
    pub time_slot: String,
    /// Class applied to all-day slot buttons.
    pub all_day_slot: String,
    /// Class applied to month day cells.
    pub month_day: String,
    /// Class applied to event nodes.
    pub event: String,
}

/// The props for the [`Schedule`](crate::schedule::components::Schedule) component.
#[derive(Props, Clone, PartialEq)]
pub struct ScheduleProps {
    /// Controlled active date.
    #[props(default)]
    pub date: ReadSignal<Option<Date>>,
    /// Default active date for uncontrolled usage.
    #[props(default = today())]
    pub default_date: Date,
    /// Callback fired after the active date changes.
    #[props(default)]
    pub on_date_change: Callback<ScheduleDateChange>,
    /// Controlled active view.
    #[props(default)]
    pub view: ReadSignal<Option<ScheduleView>>,
    /// Default active view for uncontrolled usage.
    #[props(default)]
    pub default_view: ScheduleView,
    /// Callback fired after the active view changes.
    #[props(default)]
    pub on_view_change: Callback<ScheduleViewChange>,
    /// Interaction mode.
    #[props(default)]
    pub mode: ScheduleMode,
    /// Layout strategy.
    #[props(default)]
    pub layout: ScheduleLayout,
    /// Events to render.
    #[props(default)]
    pub events: Vec<ScheduleEvent>,
    /// Recurrence expansion limit.
    #[props(default)]
    pub recurrence_expansion_limit: ScheduleRecurrenceExpansionLimit,
    /// Locale identifier exposed as `data-locale`.
    #[props(default = "en-US".to_string())]
    pub locale: String,
    /// Text labels.
    #[props(default)]
    pub labels: ScheduleLabels,
    /// Day view configuration.
    #[props(default)]
    pub day_view: ScheduleDayViewConfig,
    /// Week view configuration.
    #[props(default)]
    pub week_view: ScheduleWeekViewConfig,
    /// Month view configuration.
    #[props(default)]
    pub month_view: ScheduleMonthViewConfig,
    /// Year view configuration.
    #[props(default)]
    pub year_view: ScheduleYearViewConfig,
    /// Mobile month view configuration.
    #[props(default)]
    pub mobile_month_view: ScheduleMobileMonthViewConfig,
    /// Whether to render the top-level default schedule header.
    #[props(default = true)]
    pub with_default_header: bool,
    /// Custom top-level header content. When supplied, this replaces the default
    /// schedule header regardless of [`ScheduleProps::with_default_header`].
    #[props(default)]
    pub header: Option<Element>,
    /// Radius token exposed through `--dxc-schedule-radius` for style layers.
    #[props(default)]
    pub radius: Option<String>,
    /// Class names for stable schedule styling hooks.
    #[props(default)]
    pub class_names: ScheduleClassNames,
    /// Enable event drag/drop. Ignored in [`ScheduleMode::Static`].
    #[props(default)]
    pub with_events_drag_and_drop: bool,
    /// Enable drag slot selection.
    #[props(default)]
    pub with_drag_slot_select: bool,
    /// Enable event resize. Ignored in [`ScheduleMode::Static`].
    #[props(default)]
    pub with_event_resize: bool,
    /// Event drag guard.
    #[props(default = Callback::new(|event: ScheduleEvent| !event.drag_disabled))]
    pub can_drag_event: Callback<ScheduleEvent, bool>,
    /// Event resize guard.
    #[props(default = Callback::new(|event: ScheduleEvent| !event.resize_disabled))]
    pub can_resize_event: Callback<ScheduleEvent, bool>,
    /// Custom event body renderer. The default renderer shows title and time.
    #[props(default)]
    pub render_event_body: Option<Callback<ScheduleEventRenderContext, Element>>,
    /// Called when a time slot is clicked.
    #[props(default)]
    pub on_time_slot_click: Callback<ScheduleTimeSlotClick>,
    /// Called when an all-day slot is clicked.
    #[props(default)]
    pub on_all_day_slot_click: Callback<ScheduleAllDaySlotClick>,
    /// Called when a day cell is clicked.
    #[props(default)]
    pub on_day_click: Callback<ScheduleDayClick>,
    /// Called when an empty schedule slot requests event creation.
    #[props(default)]
    pub on_event_create: Callback<ScheduleEventCreate>,
    /// Called when an event is clicked.
    #[props(default)]
    pub on_event_click: Callback<ScheduleEventClick>,
    /// Called when event dragging starts.
    #[props(default)]
    pub on_event_drag_start: Callback<ScheduleEventDrag>,
    /// Called when event dragging ends.
    #[props(default)]
    pub on_event_drag_end: Callback<ScheduleEventDrag>,
    /// Called when an event is dropped on a schedule target.
    #[props(default)]
    pub on_event_drop: Callback<ScheduleEventDrop>,
    /// Called when external data is dropped on a schedule target.
    #[props(default)]
    pub on_external_event_drop: Callback<ScheduleExternalDrop>,
    /// Called when drag slot selection completes.
    #[props(default)]
    pub on_slot_drag_end: Callback<ScheduleSlotRangeSelection>,
    /// Called when an event resize completes.
    #[props(default)]
    pub on_event_resize: Callback<ScheduleEventResize>,
    /// Additional attributes to apply to the schedule root.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
