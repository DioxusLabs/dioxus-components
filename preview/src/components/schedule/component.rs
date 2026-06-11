use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::date_picker::DatePicker;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
pub use dioxus_primitives::schedule::{
    add_months, shift_date, today, ScheduleAllDaySlotClick, ScheduleClassNames,
    ScheduleDateChange, ScheduleDayClick, ScheduleDayViewConfig, ScheduleDropDestination,
    ScheduleEvent, ScheduleEventClick, ScheduleEventCreate, ScheduleEventCreateSource,
    ScheduleEventDrag, ScheduleEventDrop, ScheduleEventRenderContext, ScheduleEventResize,
    ScheduleExternalDrop, ScheduleLabels, ScheduleLayout, ScheduleMobileMonthViewConfig,
    ScheduleMode, ScheduleMonthViewConfig, ScheduleRecurrence, ScheduleRecurrenceExpansionLimit,
    ScheduleRecurrenceFrequency, ScheduleSlotRangeSelection, ScheduleTimeGridConfig,
    ScheduleTimeSlotClick, ScheduleView, ScheduleViewChange, ScheduleWeekViewConfig,
    ScheduleYearViewConfig,
};
pub type ScheduleResizeEdge = dioxus_primitives::schedule::ScheduleResizeEdge;
use time::{macros::time, Date, Duration, PrimitiveDateTime};

#[css_module("/src/components/schedule/style.css")]
struct Styles;

/// A styled schedule surface for day, week, month, and year planning views.
#[derive(Props, Clone, PartialEq)]
pub struct ScheduleProps {
    /// Controlled active date.
    #[props(default)]
    pub date: ReadSignal<Option<Date>>,
    /// Default active date for uncontrolled usage.
    #[props(default = sample_date())]
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
    #[props(default = sample_events())]
    pub events: Vec<ScheduleEvent>,
    /// Recurrence expansion limit.
    #[props(default)]
    pub recurrence_expansion_limit: ScheduleRecurrenceExpansionLimit,
    /// Locale identifier exposed to the primitive.
    #[props(default = "en-US".to_string())]
    pub locale: String,
    /// Visible labels for navigation, views, and empty regions.
    #[props(default)]
    pub labels: ScheduleLabels,
    /// Day view configuration.
    #[props(default)]
    pub day_view: ScheduleDayViewConfig,
    /// Week view configuration.
    #[props(default = work_week_config())]
    pub week_view: ScheduleWeekViewConfig,
    /// Month view configuration.
    #[props(default)]
    pub month_view: ScheduleMonthViewConfig,
    /// Year view configuration.
    #[props(default)]
    pub year_view: ScheduleYearViewConfig,
    /// Mobile month view configuration used in responsive layout.
    #[props(default)]
    pub mobile_month_view: ScheduleMobileMonthViewConfig,
    /// Whether to render the default schedule header.
    #[props(default = true)]
    pub with_default_header: bool,
    /// Custom schedule header content.
    #[props(default)]
    pub header: Option<Element>,
    /// Radius token applied through the primitive style variable.
    #[props(default)]
    pub radius: Option<String>,
    /// Stable primitive class hooks for advanced styling.
    #[props(default)]
    pub class_names: ScheduleClassNames,
    /// Enable internal event drag/drop.
    #[props(default)]
    pub with_events_drag_and_drop: bool,
    /// Enable drag-to-select slots.
    #[props(default)]
    pub with_drag_slot_select: bool,
    /// Enable event resizing.
    #[props(default)]
    pub with_event_resize: bool,
    /// Event drag guard.
    #[props(default = Callback::new(|event: ScheduleEvent| !event.drag_disabled))]
    pub can_drag_event: Callback<ScheduleEvent, bool>,
    /// Event resize guard.
    #[props(default = Callback::new(|event: ScheduleEvent| !event.resize_disabled))]
    pub can_resize_event: Callback<ScheduleEvent, bool>,
    /// Custom event body renderer.
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
    /// Called when an event is dropped onto a schedule target.
    #[props(default)]
    pub on_event_drop: Callback<ScheduleEventDrop>,
    /// Called when external data is dropped onto a schedule target.
    #[props(default)]
    pub on_external_event_drop: Callback<ScheduleExternalDrop>,
    /// Called when drag slot selection completes.
    #[props(default)]
    pub on_slot_drag_end: Callback<ScheduleSlotRangeSelection>,
    /// Called when an event resize completes.
    #[props(default)]
    pub on_event_resize: Callback<ScheduleEventResize>,
    /// Additional attributes for the schedule root.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Schedule(props: ScheduleProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_schedule,
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let mut internal_date = use_signal(|| props.default_date);
    let mut internal_view = use_signal(|| props.default_view);
    let active_date = match (props.date)() {
        Some(date) => date,
        None => internal_date(),
    };
    let active_view = match (props.view)() {
        Some(view) => view,
        None => internal_view(),
    };
    let labels = props.labels.clone();
    let user_date_change = props.on_date_change;
    let user_view_change = props.on_view_change;
    let set_date = move |next: Date| {
        let previous = active_date;
        internal_date.set(next);
        user_date_change.call(ScheduleDateChange {
            previous,
            next,
            view: active_view,
        });
    };
    let set_view = move |next: ScheduleView| {
        let previous = active_view;
        internal_view.set(next);
        user_view_change.call(ScheduleViewChange {
            previous,
            next,
            date: active_date,
        });
    };
    let header = if let Some(header) = props.header {
        Some(header)
    } else if props.with_default_header {
        Some(rsx! {
            ScheduleHeader {
                date: active_date,
                view: active_view,
                labels: labels.clone(),
                on_date: set_date,
                on_view: set_view,
            }
        })
    } else {
        None
    };
    let on_date_change = move |payload: ScheduleDateChange| {
        internal_date.set(payload.next);
        user_date_change.call(payload);
    };
    let on_view_change = move |payload: ScheduleViewChange| {
        internal_view.set(payload.next);
        user_view_change.call(payload);
    };

    rsx! {
        dioxus_primitives::schedule::Schedule {
            date: Some(active_date),
            default_date: props.default_date,
            on_date_change,
            view: Some(active_view),
            default_view: props.default_view,
            on_view_change,
            mode: props.mode,
            layout: props.layout,
            events: props.events,
            recurrence_expansion_limit: props.recurrence_expansion_limit,
            locale: props.locale,
            labels: props.labels,
            day_view: props.day_view,
            week_view: props.week_view,
            month_view: props.month_view,
            year_view: props.year_view,
            mobile_month_view: props.mobile_month_view,
            with_default_header: false,
            header,
            radius: props.radius,
            class_names: props.class_names,
            with_events_drag_and_drop: props.with_events_drag_and_drop,
            with_drag_slot_select: props.with_drag_slot_select,
            with_event_resize: props.with_event_resize,
            can_drag_event: props.can_drag_event,
            can_resize_event: props.can_resize_event,
            render_event_body: props.render_event_body,
            on_time_slot_click: props.on_time_slot_click,
            on_all_day_slot_click: props.on_all_day_slot_click,
            on_day_click: props.on_day_click,
            on_event_create: props.on_event_create,
            on_event_click: props.on_event_click,
            on_event_drag_start: props.on_event_drag_start,
            on_event_drag_end: props.on_event_drag_end,
            on_event_drop: props.on_event_drop,
            on_external_event_drop: props.on_external_event_drop,
            on_slot_drag_end: props.on_slot_drag_end,
            on_event_resize: props.on_event_resize,
            attributes: merged,
        }
    }
}

#[component]
fn ScheduleHeader(
    date: Date,
    view: ScheduleView,
    labels: ScheduleLabels,
    on_date: Callback<Date>,
    on_view: Callback<ScheduleView>,
) -> Element {
    rsx! {
        header { "data-schedule-header": true,
            div { "data-schedule-header-navigation": true,
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    aria_label: labels.previous.clone(),
                    onclick: move |_| on_date.call(shift_date(date, view, -1)),
                    "‹"
                }
                DatePicker {
                    "data-schedule-date-picker": true,
                    selected_date: Some(date),
                    on_value_change: move |next| {
                        if let Some(next) = next {
                            on_date.call(next);
                        }
                    },
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    aria_label: labels.next.clone(),
                    onclick: move |_| on_date.call(shift_date(date, view, 1)),
                    "›"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| on_date.call(today()),
                    {labels.today.clone()}
                }
            }
            nav {
                "aria-label": "Schedule views",
                "data-schedule-view-controls": true,
                ScheduleViewButton {
                    target: ScheduleView::Day,
                    current: view,
                    label: labels.day,
                    on_view,
                }
                ScheduleViewButton {
                    target: ScheduleView::Week,
                    current: view,
                    label: labels.week,
                    on_view,
                }
                ScheduleViewButton {
                    target: ScheduleView::Month,
                    current: view,
                    label: labels.month,
                    on_view,
                }
                ScheduleViewButton {
                    target: ScheduleView::Year,
                    current: view,
                    label: labels.year,
                    on_view,
                }
            }
        }
    }
}

#[component]
fn ScheduleViewButton(
    target: ScheduleView,
    current: ScheduleView,
    label: String,
    on_view: Callback<ScheduleView>,
) -> Element {
    rsx! {
        Button {
            variant: if target == current { ButtonVariant::Primary } else { ButtonVariant::Outline },
            size: ButtonSize::Sm,
            "data-schedule-view-button": target.as_str(),
            "data-active": target == current,
            onclick: move |_| on_view.call(target),
            {label}
        }
    }
}

/// Returns the preview date used by the schedule examples.
pub fn sample_date() -> Date {
    today()
}

/// Returns realistic preview events covering all-day, timed, overlapping, colored, and recurring cases.
pub fn sample_events() -> Vec<ScheduleEvent> {
    let anchor = sample_date();
    let month_start = Date::from_calendar_date(anchor.year(), anchor.month(), 1).unwrap();
    let next_month_start = add_months(month_start, 1);
    let month_border_start = next_month_start - Duration::days(1);
    let days_until_sunday = (7 - anchor.weekday().number_days_from_sunday() as i64) % 7;
    let week_border_start = anchor + Duration::days(days_until_sunday - 1);

    vec![
        event(
            anchor,
            "launch",
            "Launch window",
            event_time(0, 9, 0, 10, 30),
            "blue",
        )
        .with_description("Timed launch planning window"),
        event(
            anchor,
            "design",
            "Design review",
            event_time(0, 9, 30, 11, 0),
            "violet",
        ),
        event(
            anchor,
            "support",
            "Support handoff",
            event_time(0, 10, 15, 12, 0),
            "green",
        ),
        event(
            anchor,
            "research",
            "Customer interviews",
            event_time(1, 13, 0, 15, 0),
            "orange",
        ),
        event(
            anchor,
            "sync",
            "Daily team sync",
            event_time(2, 9, 0, 9, 30),
            "teal",
        )
        .recurring(ScheduleRecurrenceFrequency::Daily, 1, Some(4)),
        event(
            anchor,
            "planning",
            "Sprint planning",
            event_time(3, 11, 0, 12, 30),
            "pink",
        ),
        event(
            anchor,
            "readout",
            "Executive readout",
            event_time(4, 15, 0, 16, 0),
            "gray",
        )
        .drag_disabled(),
        all_day_event(anchor, "onsite", "Team onsite", 0, 2, "amber"),
        all_day_event(anchor, "freeze", "Release freeze", 4, 4, "red").resize_disabled(),
        all_day_event_on_dates(
            "week-border",
            "Weekend handoff",
            week_border_start,
            week_border_start + Duration::days(2),
            "cyan",
        ),
        all_day_event_on_dates(
            "month-border",
            "Month-end rollout",
            month_border_start,
            next_month_start,
            "lime",
        ),
        event(anchor, "retro", "Retro", event_time(4, 10, 0, 11, 0), "indigo").recurring(
            ScheduleRecurrenceFrequency::Weekly,
            1,
            Some(3),
        ),
    ]
}

pub(crate) fn apply_demo_event_drop(
    events: &mut Vec<ScheduleEvent>,
    payload: &ScheduleEventDrop,
    limit: ScheduleRecurrenceExpansionLimit,
) {
    if let Some(event) = editable_demo_event(events, &payload.event_id, limit) {
        event.start = payload.new_start;
        event.end = payload.new_end;
        event.all_day = payload.destination == ScheduleDropDestination::AllDay;
    }
}

pub(crate) fn apply_demo_event_resize(
    events: &mut Vec<ScheduleEvent>,
    payload: &ScheduleEventResize,
    limit: ScheduleRecurrenceExpansionLimit,
) {
    if let Some(event) = editable_demo_event(events, &payload.event_id, limit) {
        event.start = payload.new_start;
        event.end = payload.new_end;
    }
}

/// Returns French labels used by the internationalized example.
pub fn french_labels() -> ScheduleLabels {
    ScheduleLabels {
        previous: "Precedent".to_string(),
        next: "Suivant".to_string(),
        today: "Aujourd'hui".to_string(),
        day: "Jour".to_string(),
        week: "Semaine".to_string(),
        month: "Mois".to_string(),
        year: "Annee".to_string(),
        all_day: "Journee".to_string(),
        empty_slot: "Aucun evenement".to_string(),
    }
}

/// Returns a compact workday time grid for examples.
pub fn workday_time_grid() -> ScheduleTimeGridConfig {
    ScheduleTimeGridConfig {
        with_default_header: true,
        start_hour: 7,
        end_hour: 18,
        slot_minutes: 60,
    }
}

fn work_week_config() -> ScheduleWeekViewConfig {
    ScheduleWeekViewConfig {
        time_grid: workday_time_grid(),
        ..ScheduleWeekViewConfig::default()
    }
}

fn event(anchor: Date, id: &str, title: &str, time: EventTime, color: &str) -> ScheduleEvent {
    ScheduleEvent {
        id: id.to_string(),
        title: title.to_string(),
        start: datetime(anchor + Duration::days(time.day_offset), time.start_hour, time.start_minute),
        end: datetime(anchor + Duration::days(time.day_offset), time.end_hour, time.end_minute),
        all_day: false,
        color: Some(color.to_string()),
        description: None,
        recurrence: None,
        drag_disabled: false,
        resize_disabled: false,
    }
}

fn event_time(
    day_offset: i64,
    start_hour: u8,
    start_minute: u8,
    end_hour: u8,
    end_minute: u8,
) -> EventTime {
    EventTime {
        day_offset,
        start_hour,
        start_minute,
        end_hour,
        end_minute,
    }
}

struct EventTime {
    day_offset: i64,
    start_hour: u8,
    start_minute: u8,
    end_hour: u8,
    end_minute: u8,
}

fn all_day_event(
    anchor: Date,
    id: &str,
    title: &str,
    start_day_offset: i64,
    end_day_offset: i64,
    color: &str,
) -> ScheduleEvent {
    all_day_event_on_dates(
        id,
        title,
        anchor + Duration::days(start_day_offset),
        anchor + Duration::days(end_day_offset),
        color,
    )
}

fn all_day_event_on_dates(
    id: &str,
    title: &str,
    start_date: Date,
    end_date: Date,
    color: &str,
) -> ScheduleEvent {
    ScheduleEvent {
        id: id.to_string(),
        title: title.to_string(),
        start: datetime(start_date, 0, 0),
        end: datetime(end_date, 23, 59) + Duration::minutes(1),
        all_day: true,
        color: Some(color.to_string()),
        description: None,
        recurrence: None,
        drag_disabled: false,
        resize_disabled: false,
    }
}

fn datetime(date: Date, hour: u8, minute: u8) -> PrimitiveDateTime {
    PrimitiveDateTime::new(
        date,
        time!(00:00) + Duration::hours(hour as i64) + Duration::minutes(minute as i64),
    )
}

trait ScheduleEventPreviewExt {
    fn with_description(self, description: &str) -> Self;
    fn recurring(
        self,
        frequency: ScheduleRecurrenceFrequency,
        interval: u32,
        count: Option<usize>,
    ) -> Self;
    fn drag_disabled(self) -> Self;
    fn resize_disabled(self) -> Self;
}

impl ScheduleEventPreviewExt for ScheduleEvent {
    fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    fn recurring(
        mut self,
        frequency: ScheduleRecurrenceFrequency,
        interval: u32,
        count: Option<usize>,
    ) -> Self {
        self.recurrence = Some(ScheduleRecurrence {
            frequency,
            interval,
            count,
            until: None,
        });
        self
    }

    fn drag_disabled(mut self) -> Self {
        self.drag_disabled = true;
        self
    }

    fn resize_disabled(mut self) -> Self {
        self.resize_disabled = true;
        self
    }
}

fn editable_demo_event<'a>(
    events: &'a mut Vec<ScheduleEvent>,
    event_id: &str,
    limit: ScheduleRecurrenceExpansionLimit,
) -> Option<&'a mut ScheduleEvent> {
    let editable_id = if let Some(index) = events
        .iter()
        .position(|event| event.id == event_id && event.recurrence.is_none())
    {
        events[index].id.clone()
    } else if detach_recurring_occurrence(events, event_id, limit).is_some() {
        event_id.to_string()
    } else {
        events
            .iter()
            .find(|event| event.id == event_id)
            .map(|event| event.id.clone())?
    };

    events.iter_mut().find(|event| event.id == editable_id)
}

fn detach_recurring_occurrence(
    events: &mut Vec<ScheduleEvent>,
    occurrence_id: &str,
    limit: ScheduleRecurrenceExpansionLimit,
) -> Option<usize> {
    let target = resolve_recurring_occurrence(events, occurrence_id, limit)?;
    let source = events.remove(target.source_index);

    let mut replacements = Vec::with_capacity(3);
    if target.occurrence_index > 0 {
        let mut leading = source.clone();
        leading.recurrence.as_mut().unwrap().count = Some(target.occurrence_index);
        replacements.push(leading);
    }

    let mut detached = target.occurrence.clone();
    detached.recurrence = None;
    replacements.push(detached);

    if let Some((next_start, next_end)) = target.next_occurrence_bounds {
        let mut trailing = source;
        let trailing_count = trailing
            .recurrence
            .as_ref()
            .and_then(|recurrence| recurrence.count)
            .map(|count| count - target.occurrence_index - 1);
        trailing.id = recurring_continuation_id(&trailing.id, target.occurrence_index + 1);
        trailing.start = next_start;
        trailing.end = next_end;
        trailing.recurrence.as_mut().unwrap().count = trailing_count;
        replacements.push(trailing);
    }

    let insert_at = target.source_index;
    events.splice(insert_at..insert_at, replacements);
    events
        .iter()
        .position(|event| event.id == occurrence_id && event.recurrence.is_none())
}

fn resolve_recurring_occurrence(
    events: &[ScheduleEvent],
    occurrence_id: &str,
    limit: ScheduleRecurrenceExpansionLimit,
) -> Option<RecurringOccurrenceTarget> {
    let (source_id, occurrence_index) = recurrence_identity(events, occurrence_id)?;
    let source_index = events
        .iter()
        .position(|event| event.id == source_id && event.recurrence.is_some())?;
    let source = &events[source_index];
    let occurrences = expand_preview_recurrence(source, limit);
    let occurrence = occurrences.get(occurrence_index)?.clone();
    let next_occurrence_bounds = recurrence_occurrence_at(source, occurrence_index + 1)
        .map(|occurrence| (occurrence.start, occurrence.end));

    Some(RecurringOccurrenceTarget {
        source_index,
        occurrence_index,
        occurrence,
        next_occurrence_bounds,
    })
}

fn recurrence_identity<'a>(
    events: &'a [ScheduleEvent],
    occurrence_id: &str,
) -> Option<(&'a str, usize)> {
    if let Some(event) = events
        .iter()
        .find(|event| event.id == occurrence_id && event.recurrence.is_some())
    {
        return Some((event.id.as_str(), 0));
    }

    let (source_id, index) = occurrence_id.rsplit_once(':')?;
    let occurrence_index = index.parse::<usize>().ok()?;
    events
        .iter()
        .find(|event| event.id == source_id && event.recurrence.is_some())
        .map(|event| (event.id.as_str(), occurrence_index))
}

fn recurring_continuation_id(source_id: &str, occurrence_index: usize) -> String {
    format!("__demo_recurrence_after__{source_id}__{occurrence_index}")
}

fn expand_preview_recurrence(
    event: &ScheduleEvent,
    limit: ScheduleRecurrenceExpansionLimit,
) -> Vec<ScheduleEvent> {
    let Some(recurrence) = &event.recurrence else {
        return vec![event.clone()];
    };
    let max_occurrences = recurrence
        .count
        .unwrap_or(limit.max_occurrences)
        .min(limit.max_occurrences);
    let mut occurrences = Vec::new();

    for index in 0..max_occurrences {
        let Some(occurrence) = recurrence_occurrence_at(event, index) else {
            break;
        };
        occurrences.push(occurrence);
    }

    occurrences
}

struct RecurringOccurrenceTarget {
    source_index: usize,
    occurrence_index: usize,
    occurrence: ScheduleEvent,
    next_occurrence_bounds: Option<(PrimitiveDateTime, PrimitiveDateTime)>,
}

fn recurrence_occurrence_at(
    event: &ScheduleEvent,
    occurrence_index: usize,
) -> Option<ScheduleEvent> {
    let recurrence = event.recurrence.as_ref()?;
    if recurrence
        .count
        .is_some_and(|count| occurrence_index >= count)
    {
        return None;
    }

    let duration = event.end - event.start;
    let start = advance_recurrence_start(event.start, recurrence, occurrence_index);
    if recurrence.until.is_some_and(|until| start > until) {
        return None;
    }

    let mut occurrence = event.clone();
    occurrence.id = if occurrence_index == 0 {
        event.id.clone()
    } else {
        format!("{}:{occurrence_index}", event.id)
    };
    occurrence.start = start;
    occurrence.end = start + duration;
    occurrence.recurrence = None;
    Some(occurrence)
}

fn advance_recurrence_start(
    start: PrimitiveDateTime,
    recurrence: &ScheduleRecurrence,
    occurrence_index: usize,
) -> PrimitiveDateTime {
    let interval = recurrence.interval.max(1) as i64;
    match recurrence.frequency {
        ScheduleRecurrenceFrequency::Daily => {
            start + Duration::days(interval * occurrence_index as i64)
        }
        ScheduleRecurrenceFrequency::Weekly => {
            start + Duration::weeks(interval * occurrence_index as i64)
        }
        ScheduleRecurrenceFrequency::Monthly => PrimitiveDateTime::new(
            add_months(start.date(), (interval * occurrence_index as i64) as i32),
            start.time(),
        ),
        ScheduleRecurrenceFrequency::Yearly => PrimitiveDateTime::new(
            add_months(
                start.date(),
                (interval * 12 * occurrence_index as i64) as i32,
            ),
            start.time(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_datetime(day_offset: i64, hour: u8, minute: u8) -> PrimitiveDateTime {
        datetime(sample_date() + Duration::days(day_offset), hour, minute)
    }

    fn drop_payload(
        event_id: &str,
        new_start: PrimitiveDateTime,
        new_end: PrimitiveDateTime,
        destination: ScheduleDropDestination,
    ) -> ScheduleEventDrop {
        ScheduleEventDrop {
            event_id: event_id.to_string(),
            event: ScheduleEvent {
                id: event_id.to_string(),
                title: "Edited".to_string(),
                start: new_start,
                end: new_end,
                all_day: destination == ScheduleDropDestination::AllDay,
                color: None,
                description: None,
                recurrence: None,
                drag_disabled: false,
                resize_disabled: false,
            },
            new_start,
            new_end,
            destination,
            date: new_start.date(),
            view: ScheduleView::Week,
        }
    }

    fn resize_payload(
        event_id: &str,
        new_start: PrimitiveDateTime,
        new_end: PrimitiveDateTime,
    ) -> ScheduleEventResize {
        ScheduleEventResize {
            event_id: event_id.to_string(),
            event: ScheduleEvent {
                id: event_id.to_string(),
                title: "Edited".to_string(),
                start: new_start,
                end: new_end,
                all_day: false,
                color: None,
                description: None,
                recurrence: None,
                drag_disabled: false,
                resize_disabled: false,
            },
            new_start,
            new_end,
            edge: dioxus_primitives::schedule::ScheduleResizeEdge::End,
            view: ScheduleView::Week,
        }
    }

    #[test]
    fn editing_one_recurring_occurrence_detaches_only_that_occurrence() {
        let mut events = sample_events();
        let new_start = sample_datetime(3, 7, 0);
        let new_end = sample_datetime(3, 7, 30);

        apply_demo_event_resize(
            &mut events,
            &resize_payload("sync:1", new_start, new_end),
            ScheduleRecurrenceExpansionLimit::default(),
        );

        let detached = events.iter().find(|event| event.id == "sync:1").unwrap();
        assert_eq!(detached.start, new_start);
        assert_eq!(detached.end, new_end);
        assert!(detached.recurrence.is_none());

        let source = events.iter().find(|event| event.id == "sync").unwrap();
        assert_eq!(source.start, sample_datetime(2, 9, 0));
        assert_eq!(source.recurrence.as_ref().unwrap().count, Some(1));

        let trailing = events
            .iter()
            .find(|event| event.id == recurring_continuation_id("sync", 2))
            .unwrap();
        assert!(trailing.recurrence.is_some());
        assert_eq!(trailing.start, sample_datetime(4, 9, 0));
        assert_eq!(trailing.recurrence.as_ref().unwrap().count, Some(2));
    }

    #[test]
    fn editing_first_recurring_occurrence_detaches_only_that_occurrence() {
        let mut events = sample_events();
        let new_start = sample_datetime(2, 7, 0);
        let new_end = sample_datetime(2, 7, 30);

        apply_demo_event_resize(
            &mut events,
            &resize_payload("sync", new_start, new_end),
            ScheduleRecurrenceExpansionLimit::default(),
        );

        let detached = events.iter().find(|event| event.id == "sync").unwrap();
        assert_eq!(detached.start, new_start);
        assert_eq!(detached.end, new_end);
        assert!(detached.recurrence.is_none());

        let trailing = events
            .iter()
            .find(|event| event.id == recurring_continuation_id("sync", 1))
            .unwrap();
        assert!(trailing.recurrence.is_some());
        assert_eq!(trailing.start, sample_datetime(3, 9, 0));
        assert_eq!(trailing.recurrence.as_ref().unwrap().count, Some(3));
    }

    #[test]
    fn dragging_recurring_occurrence_to_all_day_detaches_only_that_occurrence() {
        let mut events = sample_events();
        let new_start = sample_datetime(3, 0, 0);
        let new_end = sample_datetime(3, 23, 59) + Duration::minutes(1);

        apply_demo_event_drop(
            &mut events,
            &drop_payload(
                "sync:1",
                new_start,
                new_end,
                ScheduleDropDestination::AllDay,
            ),
            ScheduleRecurrenceExpansionLimit::default(),
        );

        let detached = events.iter().find(|event| event.id == "sync:1").unwrap();
        assert_eq!(detached.start, new_start);
        assert_eq!(detached.end, new_end);
        assert!(detached.all_day);
        assert!(detached.recurrence.is_none());

        let source = events.iter().find(|event| event.id == "sync").unwrap();
        assert_eq!(source.recurrence.as_ref().unwrap().count, Some(1));

        let trailing = events
            .iter()
            .find(|event| event.id == recurring_continuation_id("sync", 2))
            .unwrap();
        assert!(trailing.recurrence.is_some());
        assert_eq!(trailing.start, sample_datetime(4, 9, 0));
    }

    #[test]
    fn non_recurring_ids_with_numeric_suffix_stay_directly_editable() {
        let start = sample_datetime(0, 13, 0);
        let end = sample_datetime(0, 14, 0);
        let mut events = vec![ScheduleEvent {
            id: "design:1".to_string(),
            title: "Design review".to_string(),
            start,
            end,
            all_day: false,
            color: None,
            description: None,
            recurrence: None,
            drag_disabled: false,
            resize_disabled: false,
        }];
        let new_start = sample_datetime(0, 15, 0);
        let new_end = sample_datetime(0, 16, 0);

        apply_demo_event_resize(
            &mut events,
            &resize_payload("design:1", new_start, new_end),
            ScheduleRecurrenceExpansionLimit::default(),
        );

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, "design:1");
        assert_eq!(events[0].start, new_start);
        assert_eq!(events[0].end, new_end);
    }

    #[test]
    fn editing_last_visible_occurrence_preserves_unexpanded_recurring_tail() {
        let start = sample_datetime(0, 9, 0);
        let end = sample_datetime(0, 9, 30);
        let mut events = vec![ScheduleEvent {
            id: "series".to_string(),
            title: "Long series".to_string(),
            start,
            end,
            all_day: false,
            color: None,
            description: None,
            recurrence: Some(ScheduleRecurrence {
                frequency: ScheduleRecurrenceFrequency::Daily,
                interval: 1,
                count: Some(10),
                until: None,
            }),
            drag_disabled: false,
            resize_disabled: false,
        }];
        let new_start = sample_datetime(4, 7, 0);
        let new_end = sample_datetime(4, 7, 30);
        let limit = ScheduleRecurrenceExpansionLimit { max_occurrences: 5 };

        apply_demo_event_resize(
            &mut events,
            &resize_payload("series:4", new_start, new_end),
            limit,
        );

        let leading = events.iter().find(|event| event.id == "series").unwrap();
        assert_eq!(leading.recurrence.as_ref().unwrap().count, Some(4));

        let detached = events.iter().find(|event| event.id == "series:4").unwrap();
        assert_eq!(detached.start, new_start);
        assert_eq!(detached.end, new_end);
        assert!(detached.recurrence.is_none());

        let trailing = events
            .iter()
            .find(|event| event.id == recurring_continuation_id("series", 5))
            .unwrap();
        assert_eq!(trailing.start, sample_datetime(5, 9, 0));
        assert_eq!(trailing.end, sample_datetime(5, 9, 30));
        assert_eq!(trailing.recurrence.as_ref().unwrap().count, Some(5));
    }
}
