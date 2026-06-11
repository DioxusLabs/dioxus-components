use dioxus::prelude::*;
use time::{Date, Duration, Month, PrimitiveDateTime, Time};

use crate::use_controlled;

use super::state::{ScheduleCapabilities, ScheduleResizeState, ScheduleSlotSelectionState};
use super::types::*;
use super::utils::{
    current_time_line_offset, expand_events, external_drop_data, filter_events_for_date,
    format_date_label, format_day_of_month_label, format_time, format_time_range, is_current_day,
    layout_overlapping_events, month_event_segments_for_week, month_grid_dates,
    month_weekday_labels, now, resized_event_times, selection_contains, shift_date,
    slot_selection_range, time_slots, timed_event_geometry, today, week_dates,
    year_month_transition,
};

/// # Schedule
///
/// The `Schedule` component provides primitive scheduling behavior for day, week,
/// month, and year views. Custom event bodies can be supplied with
/// [`ScheduleProps::render_event_body`], which receives a [`ScheduleEventRenderContext`].
#[component]
pub fn Schedule(props: ScheduleProps) -> Element {
    let date_change = props.on_date_change;
    let view_change = props.on_view_change;
    let initial_date = props.default_date;
    let initial_view = props.default_view;
    let (date, raw_set_date) = use_controlled(props.date, initial_date, Callback::new(|_| {}));
    let (view, raw_set_view) = use_controlled(props.view, initial_view, Callback::new(|_| {}));
    let expanded_events = expand_events(&props.events, props.recurrence_expansion_limit);
    let current_date = date();
    let current_view = view();
    let dragging_event = use_signal(|| None::<ScheduleEvent>);
    let drop_target = use_signal(|| None::<String>);
    let slot_selection = use_signal(|| None::<ScheduleSlotSelectionState>);
    let slot_selection_suppressed_click = use_signal(|| None::<PrimitiveDateTime>);
    let resizing_event = use_signal(|| None::<ScheduleResizeState>);
    let resize_target = use_signal(|| None::<PrimitiveDateTime>);
    let capabilities = ScheduleCapabilities::new(
        props.mode,
        props.with_events_drag_and_drop,
        props.with_drag_slot_select,
        props.with_event_resize,
    );

    let set_date = move |next: Date| {
        let previous = date();
        raw_set_date.call(next);
        date_change.call(ScheduleDateChange {
            previous,
            next,
            view: view(),
        });
    };
    let set_view = move |next: ScheduleView| {
        let previous = view();
        raw_set_view.call(next);
        view_change.call(ScheduleViewChange {
            previous,
            next,
            date: date(),
        });
    };

    rsx! {
        div {
            "data-schedule-root": true,
            "data-view": current_view.as_str(),
            "data-mode": match props.mode {
                ScheduleMode::Default => "default",
                ScheduleMode::Static => "static",
            },
            "data-layout": match props.layout {
                ScheduleLayout::Default => "default",
                ScheduleLayout::Responsive => "responsive",
            },
            "data-locale": props.locale,
            "data-dragging": dragging_event().is_some(),
            "data-resizing": resizing_event().is_some(),
            style: props.radius.as_ref().map(|radius| format!("--dxc-schedule-radius: {radius};")),
            ..props.attributes,

            if let Some(header) = props.header {
                {header}
            } else if props.with_default_header {
                ScheduleHeader {
                    date: current_date,
                    view: current_view,
                    labels: props.labels.clone(),
                    on_previous: move |_| set_date(shift_date(current_date, current_view, -1)),
                    on_next: move |_| set_date(shift_date(current_date, current_view, 1)),
                    on_today: move |_| set_date(today()),
                    on_view: set_view,
                }
            }

            div {
                "data-schedule-desktop": true,
                class: props.class_names.desktop_view.clone(),
                ScheduleViewBody {
                    date: current_date,
                    view: current_view,
                    events: expanded_events.clone(),
                    labels: props.labels.clone(),
                    day_view: props.day_view,
                    week_view: props.week_view,
                    month_view: props.month_view,
                    year_view: props.year_view,
                    locale: props.locale.clone(),
                    class_names: props.class_names.clone(),
                    mobile: false,
                    capabilities,
                    dragging_event,
                    drop_target,
                    slot_selection,
                    slot_selection_suppressed_click,
                    resizing_event,
                    resize_target,
                    can_drag_event: props.can_drag_event,
                    can_resize_event: props.can_resize_event,
                    render_event_body: props.render_event_body,
                    on_date: Callback::new(set_date),
                    on_view: Callback::new(set_view),
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
                }
            }
            if props.layout == ScheduleLayout::Responsive {
                div {
                    "data-schedule-mobile": true,
                    "data-mobile-month-header": props.mobile_month_view.with_default_header,
                    class: props.class_names.mobile_view.clone(),
                    ScheduleViewBody {
                        date: current_date,
                        view: if current_view == ScheduleView::Year { ScheduleView::Year } else { ScheduleView::Month },
                        events: expanded_events,
                        labels: props.labels,
                        day_view: props.day_view,
                        week_view: props.week_view,
                        month_view: props.month_view,
                        year_view: props.year_view,
                        locale: props.locale.clone(),
                        class_names: props.class_names.clone(),
                        mobile: true,
                        mobile_month_view: props.mobile_month_view,
                        capabilities,
                        dragging_event,
                        drop_target,
                        slot_selection,
                        slot_selection_suppressed_click,
                        resizing_event,
                        resize_target,
                        can_drag_event: props.can_drag_event,
                        can_resize_event: props.can_resize_event,
                        render_event_body: props.render_event_body,
                        on_date: Callback::new(set_date),
                        on_view: Callback::new(set_view),
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
                    }
                }
            }
        }
    }
}

/// Props for [`ScheduleHeader`].
#[derive(Props, Clone, PartialEq)]
pub struct ScheduleHeaderProps {
    /// Date used by the header title and navigation callbacks.
    pub date: Date,
    /// Active schedule view.
    pub view: ScheduleView,
    /// Labels used by navigation and view controls.
    pub labels: ScheduleLabels,
    /// Called when the previous range button is clicked.
    pub on_previous: Callback<MouseEvent>,
    /// Called when the next range button is clicked.
    pub on_next: Callback<MouseEvent>,
    /// Called when the today button is clicked.
    pub on_today: Callback<MouseEvent>,
    /// Called when a view button is selected.
    pub on_view: Callback<ScheduleView>,
}

/// Default schedule header with navigation and view controls.
#[component]
pub fn ScheduleHeader(props: ScheduleHeaderProps) -> Element {
    let title = format!("{:?} {}", props.date.month(), props.date.year());
    rsx! {
        header { "data-schedule-header": true,
            button {
                "type": "button",
                "aria-label": props.labels.previous.clone(),
                onclick: move |event| props.on_previous.call(event),
                {props.labels.previous.clone()}
            }
            button {
                "type": "button",
                "aria-label": props.labels.today.clone(),
                onclick: move |event| props.on_today.call(event),
                {props.labels.today.clone()}
            }
            button {
                "type": "button",
                "aria-label": props.labels.next.clone(),
                onclick: move |event| props.on_next.call(event),
                {props.labels.next.clone()}
            }
            div { "data-schedule-title": true, "{title}" }
            nav {
                "aria-label": "Schedule views",
                "data-schedule-view-controls": true,
                ScheduleViewButton {
                    target: ScheduleView::Day,
                    current: props.view,
                    label: props.labels.day,
                    on_view: props.on_view,
                }
                ScheduleViewButton {
                    target: ScheduleView::Week,
                    current: props.view,
                    label: props.labels.week,
                    on_view: props.on_view,
                }
                ScheduleViewButton {
                    target: ScheduleView::Month,
                    current: props.view,
                    label: props.labels.month,
                    on_view: props.on_view,
                }
                ScheduleViewButton {
                    target: ScheduleView::Year,
                    current: props.view,
                    label: props.labels.year,
                    on_view: props.on_view,
                }
            }
        }
    }
}

/// Props for [`ScheduleViewButton`].
#[derive(Props, Clone, PartialEq)]
pub struct ScheduleViewButtonProps {
    /// View selected by the button.
    pub target: ScheduleView,
    /// Currently active view.
    pub current: ScheduleView,
    /// Visible button label.
    pub label: String,
    /// Called with [`ScheduleViewButtonProps::target`] when clicked.
    pub on_view: Callback<ScheduleView>,
}

/// Default schedule view switch button used by [`ScheduleHeader`].
#[component]
pub fn ScheduleViewButton(props: ScheduleViewButtonProps) -> Element {
    rsx! {
        button {
            "type": "button",
            "data-schedule-view-button": props.target.as_str(),
            "data-active": props.current == props.target,
            onclick: move |_| props.on_view.call(props.target),
            {props.label}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ScheduleViewBodyProps {
    date: Date,
    view: ScheduleView,
    events: Vec<ScheduleEvent>,
    labels: ScheduleLabels,
    day_view: ScheduleDayViewConfig,
    week_view: ScheduleWeekViewConfig,
    month_view: ScheduleMonthViewConfig,
    year_view: ScheduleYearViewConfig,
    locale: String,
    class_names: ScheduleClassNames,
    #[props(default)]
    mobile_month_view: ScheduleMobileMonthViewConfig,
    mobile: bool,
    capabilities: ScheduleCapabilities,
    dragging_event: Signal<Option<ScheduleEvent>>,
    drop_target: Signal<Option<String>>,
    slot_selection: Signal<Option<ScheduleSlotSelectionState>>,
    slot_selection_suppressed_click: Signal<Option<PrimitiveDateTime>>,
    resizing_event: Signal<Option<ScheduleResizeState>>,
    resize_target: Signal<Option<PrimitiveDateTime>>,
    can_drag_event: Callback<ScheduleEvent, bool>,
    can_resize_event: Callback<ScheduleEvent, bool>,
    render_event_body: Option<Callback<ScheduleEventRenderContext, Element>>,
    on_date: Callback<Date>,
    on_view: Callback<ScheduleView>,
    on_time_slot_click: Callback<ScheduleTimeSlotClick>,
    on_all_day_slot_click: Callback<ScheduleAllDaySlotClick>,
    on_day_click: Callback<ScheduleDayClick>,
    on_event_create: Callback<ScheduleEventCreate>,
    on_event_click: Callback<ScheduleEventClick>,
    on_event_drag_start: Callback<ScheduleEventDrag>,
    on_event_drag_end: Callback<ScheduleEventDrag>,
    on_event_drop: Callback<ScheduleEventDrop>,
    on_external_event_drop: Callback<ScheduleExternalDrop>,
    on_slot_drag_end: Callback<ScheduleSlotRangeSelection>,
    on_event_resize: Callback<ScheduleEventResize>,
}

#[component]
fn ScheduleViewBody(props: ScheduleViewBodyProps) -> Element {
    if props.mobile && props.view != ScheduleView::Year {
        return rsx! {
            MobileMonthView { ..props }
        };
    }
    match props.view {
        ScheduleView::Day => rsx! {
            TimeGridView {
                body: props.clone(),
                days: vec![props.date],
                config: props.day_view.time_grid,
            }
        },
        ScheduleView::Week => rsx! {
            TimeGridView {
                body: props.clone(),
                days: week_dates(props.date, props.week_view.first_day_of_week),
                config: props.week_view.time_grid,
            }
        },
        ScheduleView::Month => rsx! {
            MonthView { ..props }
        },
        ScheduleView::Year => rsx! {
            YearView { ..props }
        },
    }
}

#[derive(Props, Clone, PartialEq)]
struct TimeGridViewProps {
    body: ScheduleViewBodyProps,
    days: Vec<Date>,
    config: ScheduleTimeGridConfig,
}

#[component]
fn TimeGridView(mut props: TimeGridViewProps) -> Element {
    let view = props.body.view;
    let slots = time_slots(props.config);
    let column_template = format!("repeat({}, minmax(0, 1fr))", props.days.len().max(1));
    let current_date_time = now();
    let current_time_line_offset = current_time_line_offset(current_date_time, props.config);
    let current_day_index = props
        .days
        .iter()
        .position(|day| *day == current_date_time.date());
    let current_day_count = props.days.len().max(1) as f32;
    let current_time_label = format_time(current_date_time.time());
    let all_day_events: Vec<_> = props
        .body
        .events
        .iter()
        .filter(|event| event.all_day)
        .cloned()
        .collect();
    let timed_multi_day_events: Vec<_> = props
        .body
        .events
        .iter()
        .filter(|event| !event.all_day && timed_event_spans_multiple_days(event))
        .cloned()
        .collect();
    rsx! {
        section {
            "data-schedule-view": view.as_str(),
            "data-mobile": props.body.mobile,
            "data-default-header": props.config.with_default_header,
            class: match view {
                ScheduleView::Day => props.body.class_names.day_view.clone(),
                ScheduleView::Week => props.body.class_names.week_view.clone(),
                ScheduleView::Month => props.body.class_names.month_view.clone(),
                ScheduleView::Year => props.body.class_names.year_view.clone(),
            },
            div {
                "data-schedule-day-header-row": true,
                style: "grid-template-columns: {column_template};",
                for day in props.days.iter().copied() {
                    button {
                        "type": "button",
                        "data-schedule-day-header": day.to_string(),
                        onclick: move |_| {
                            props
                                .body
                                .on_day_click
                                .call(ScheduleDayClick {
                                    date: day,
                                    view,
                                });
                        },
                        {format_date_label(day)}
                    }
                }
            }
            div {
                "data-schedule-all-day-row": true,
                style: "grid-template-columns: {column_template};",
                for (day_index, day) in props.days.iter().copied().enumerate() {
                    {
                        let target_id = format!("all-day-{day}");
                        let all_day_label =
                            format!("{} {}", props.body.labels.all_day, format_date_label(day));
                        rsx! {
                            div { "data-schedule-all-day-column": day.to_string(),
                                button {
                                    "type": "button",
                                    "data-schedule-all-day-slot": day.to_string(),
                                    "aria-label": all_day_label,
                                    "data-drop-enabled": props.body.capabilities.events_drag_and_drop,
                                    "data-drop-accepted": props.body.capabilities.events_drag_and_drop
                                        && (props.body.dragging_event)().is_some(),
                                    "data-drop-active": (props.body.drop_target)() == Some(target_id.clone()),
                                    "data-drop-denied": (props.body.dragging_event)().is_some()
                                        && !props.body.capabilities.events_drag_and_drop,
                                    class: props.body.class_names.all_day_slot.clone(),
                                    onclick: move |_| {
                                        props
                                            .body
                                            .on_all_day_slot_click
                                            .call(ScheduleAllDaySlotClick {
                                                date: day,
                                                view,
                                            });
                                        props
                                            .body
                                            .on_event_create
                                            .call(
                                                all_day_event_create(
                                                    day,
                                                    view,
                                                    ScheduleEventCreateSource::AllDaySlotClick,
                                                ),
                                            );
                                    },
                                    onmouseup: move |_| {
                                        let start = PrimitiveDateTime::new(day, Time::MIDNIGHT);
                                        move_dragged_event(
                                            props.body.dragging_event,
                                            start,
                                            day,
                                            ScheduleDropDestination::AllDay,
                                            view,
                                            props.body.on_event_drag_end,
                                            props.body.on_event_drop,
                                        );
                                    },
                                    onpointerup: move |_| {
                                        let start = PrimitiveDateTime::new(day, Time::MIDNIGHT);
                                        move_dragged_event(
                                            props.body.dragging_event,
                                            start,
                                            day,
                                            ScheduleDropDestination::AllDay,
                                            view,
                                            props.body.on_event_drag_end,
                                            props.body.on_event_drop,
                                        );
                                    },
                                    ondragover: move |event| {
                                        if props.body.capabilities.events_drag_and_drop {
                                            event.prevent_default();
                                        }
                                    },
                                    ondragenter: {
                                        let target_id = target_id.clone();
                                        move |event| {
                                            if props.body.capabilities.events_drag_and_drop {
                                                event.prevent_default();
                                                props.body.drop_target.set(Some(target_id.clone()));
                                            }
                                        }
                                    },
                                    ondragleave: {
                                        let target_id = target_id.clone();
                                        move |_| {
                                            if (props.body.drop_target)() == Some(target_id.clone()) {
                                                props.body.drop_target.set(None);
                                            }
                                        }
                                    },
                                    ondrop: {
                                        let events = props.body.events.clone();
                                        let target_id = target_id.clone();
                                        move |event: Event<DragData>| {
                                            if props.body.capabilities.events_drag_and_drop {
                                                event.prevent_default();
                                                if (props.body.drop_target)() == Some(target_id.clone()) {
                                                    props.body.drop_target.set(None);
                                                }
                                                let start = PrimitiveDateTime::new(day, Time::MIDNIGHT);
                                                let end = start + Duration::days(1);
                                                if !move_dragged_event_from_drop(
                                                    props.body.dragging_event,
                                                    &event,
                                                    &events,
                                                    ScheduleDropContext {
                                                        new_start: start,
                                                        date: day,
                                                        destination: ScheduleDropDestination::AllDay,
                                                        view,
                                                        slot_minutes: None,
                                                    },
                                                    props.body.on_event_drop,
                                                ) {
                                                    let external = external_drop_data(&event);
                                                    props
                                                        .body
                                                        .on_external_event_drop
                                                        .call(ScheduleExternalDrop {
                                                            data: external.as_ref().map(|data| data.data.clone()),
                                                            data_format: external.map(|data| data.format),
                                                            start,
                                                            end,
                                                            date: day,
                                                            view,
                                                        });
                                                }
                                            }
                                        }
                                    },
                                    if day_index == 0 {
                                        {props.body.labels.all_day.clone()}
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    "data-schedule-all-day-events": true,
                    style: "grid-column: 1 / -1; grid-template-columns: {column_template};",
                    for segment in month_event_segments_for_week(&all_day_events, &props.days) {
                        ScheduleEventNode {
                            event: segment.event,
                            view,
                            date: segment.start_date,
                            capabilities: props.body.capabilities,
                            dragging_event: props.body.dragging_event,
                            resizing_event: props.body.resizing_event,
                            resize_target: props.body.resize_target,
                            can_drag_event: props.body.can_drag_event,
                            can_resize_event: props.body.can_resize_event,
                            render_event_body: props.body.render_event_body,
                            on_event_click: props.body.on_event_click,
                            on_event_drag_start: props.body.on_event_drag_start,
                            on_event_drag_end: props.body.on_event_drag_end,
                            on_event_resize: props.body.on_event_resize,
                            class_name: props.body.class_names.event.clone(),
                            layout_style: format!(
                                "grid-column: {} / span {};",
                                segment.start_column + 1,
                                segment.column_span,
                            ),
                        }
                    }
                }
            }
            div {
                "data-schedule-time-grid": true,
                style: "grid-template-columns: {column_template}; position: relative;",
                if let (Some(offset), Some(day_index)) = (current_time_line_offset, current_day_index) {
                    {
                        let day_width = 100.0 / current_day_count;
                        let day_left = day_index as f32 * day_width;
                        rsx! {
                            div {
                                "data-schedule-current-time-line": true,
                                "aria-label": "Current time {current_time_label}",
                                style: format!(
                                    "position: absolute; left: 0; right: 0; top: calc({offset:.4}% - 1px); height: 2px; background: color-mix(in srgb, #ef4444 36%, transparent); pointer-events: none; z-index: 2;",
                                ),
                                span {
                                    "data-schedule-current-time-label": true,
                                    style: "position: absolute; left: 0; top: 50%; transform: translateY(-50%); border-radius: 999px; background: #ef4444; color: white; padding: 2px 8px; font-size: 0.75rem; font-weight: 700; line-height: 1.2; box-shadow: 0 1px 2px rgba(0, 0, 0, 0.22);",
                                    "{current_time_label}"
                                }
                                span {
                                    "data-schedule-current-time-segment": true,
                                    style: format!(
                                        "position: absolute; left: {day_left:.4}%; width: {day_width:.4}%; top: 0; height: 2px; background: #ef4444;",
                                    ),
                                }
                                span {
                                    "data-schedule-current-time-marker": true,
                                    style: format!(
                                        "position: absolute; left: {day_left:.4}%; top: 50%; width: 10px; height: 10px; border-radius: 999px; background: #ef4444; transform: translate(-50%, -50%);",
                                    ),
                                }
                            }
                        }
                    }
                }
                if let Some(style) = timed_drop_preview_style(
                    (props.body.drop_target)(),
                    &props.days,
                    props.config,
                    (props.body.dragging_event)().as_ref(),
                ) {
                    div {
                        "data-schedule-drop-preview": true,
                        "data-drop-active": true,
                        style,
                    }
                }
                for day in props.days.iter().copied() {
                    div { "data-schedule-day-column": day.to_string(),
                        div { "data-schedule-day-slots": true,
                            for slot in slots.iter().copied() {
                                {
                                    rsx! {
                                        TimeSlot {
                                            date: day,
                                            slot,
                                            view,
                                            slot_minutes: props.config.slot_minutes,
                                            all_events: props.body.events.clone(),
                                            class_name: props.body.class_names.time_slot.clone(),
                                            capabilities: props.body.capabilities,
                                            dragging_event: props.body.dragging_event,
                                            drop_target: props.body.drop_target,
                                            slot_selection: props.body.slot_selection,
                                            slot_selection_suppressed_click: props.body.slot_selection_suppressed_click,
                                            resizing_event: props.body.resizing_event,
                                            resize_target: props.body.resize_target,
                                            on_time_slot_click: props.body.on_time_slot_click,
                                            on_event_create: props.body.on_event_create,
                                            on_event_drop: props.body.on_event_drop,
                                            on_external_event_drop: props.body.on_external_event_drop,
                                            on_slot_drag_end: props.body.on_slot_drag_end,
                                            on_event_resize: props.body.on_event_resize,
                                            can_drag_event: props.body.can_drag_event,
                                            can_resize_event: props.body.can_resize_event,
                                            render_event_body: props.body.render_event_body,
                                            on_event_click: props.body.on_event_click,
                                            on_event_drag_start: props.body.on_event_drag_start,
                                            on_event_drag_end: props.body.on_event_drag_end,
                                        }
                                    }
                                }
                            }
                        }
                        div { "data-schedule-timed-events": true,
                            for (event, geometry) in layout_overlapping_events(
                                filter_events_for_date(&props.body.events, day)
                                    .into_iter()
                                    .filter(|event| {
                                        !event.all_day && !timed_event_spans_multiple_days(event)
                                    })
                                    .collect(),
                            )
                            .into_iter()
                            .filter_map(|event| {
                                timed_event_geometry(&event.event, day, props.config)
                                    .map(|geometry| (event, geometry))
                            }) {
                                ScheduleEventNode {
                                    event: event.event,
                                    view: props.body.view,
                                    date: day,
                                    slot_minutes: props.config.slot_minutes,
                                    layout_column: event.column,
                                    layout_columns: event.columns,
                                    layout_style: timed_event_style(geometry, event.column, event.columns),
                                    capabilities: props.body.capabilities,
                                    dragging_event: props.body.dragging_event,
                                    resizing_event: props.body.resizing_event,
                                    resize_target: props.body.resize_target,
                                    can_drag_event: props.body.can_drag_event,
                                    can_resize_event: props.body.can_resize_event,
                                    render_event_body: props.body.render_event_body,
                                    on_event_click: props.body.on_event_click,
                                    on_event_drag_start: props.body.on_event_drag_start,
                                    on_event_drag_end: props.body.on_event_drag_end,
                                    on_event_resize: props.body.on_event_resize,
                                    class_name: props.body.class_names.event.clone(),
                                }
                            }
                        }
                    }
                }
                div {
                    "data-schedule-timed-spanning-events": true,
                    style: "grid-column: 1 / -1; position: absolute; inset: 0; pointer-events: none;",
                    for (event, geometry) in timed_multi_day_events
                        .iter()
                        .filter_map(|event| {
                            timed_spanning_event_geometry(event, &props.days, props.config)
                                .map(|geometry| (event.clone(), geometry))
                        }) {
                        ScheduleEventNode {
                            event,
                            view: props.body.view,
                            date: geometry.start_date,
                            slot_minutes: props.config.slot_minutes,
                            layout_style: timed_spanning_event_style(geometry),
                            capabilities: props.body.capabilities,
                            dragging_event: props.body.dragging_event,
                            resizing_event: props.body.resizing_event,
                            resize_target: props.body.resize_target,
                            can_drag_event: props.body.can_drag_event,
                            can_resize_event: props.body.can_resize_event,
                            render_event_body: props.body.render_event_body,
                            on_event_click: props.body.on_event_click,
                            on_event_drag_start: props.body.on_event_drag_start,
                            on_event_drag_end: props.body.on_event_drag_end,
                            on_event_resize: props.body.on_event_resize,
                            class_name: props.body.class_names.event.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TimeSlotProps {
    date: Date,
    slot: Time,
    view: ScheduleView,
    slot_minutes: u8,
    all_events: Vec<ScheduleEvent>,
    class_name: String,
    capabilities: ScheduleCapabilities,
    dragging_event: Signal<Option<ScheduleEvent>>,
    drop_target: Signal<Option<String>>,
    slot_selection: Signal<Option<ScheduleSlotSelectionState>>,
    slot_selection_suppressed_click: Signal<Option<PrimitiveDateTime>>,
    resizing_event: Signal<Option<ScheduleResizeState>>,
    resize_target: Signal<Option<PrimitiveDateTime>>,
    on_time_slot_click: Callback<ScheduleTimeSlotClick>,
    on_event_create: Callback<ScheduleEventCreate>,
    on_event_drop: Callback<ScheduleEventDrop>,
    on_external_event_drop: Callback<ScheduleExternalDrop>,
    on_slot_drag_end: Callback<ScheduleSlotRangeSelection>,
    on_event_resize: Callback<ScheduleEventResize>,
    can_drag_event: Callback<ScheduleEvent, bool>,
    can_resize_event: Callback<ScheduleEvent, bool>,
    render_event_body: Option<Callback<ScheduleEventRenderContext, Element>>,
    on_event_click: Callback<ScheduleEventClick>,
    on_event_drag_start: Callback<ScheduleEventDrag>,
    on_event_drag_end: Callback<ScheduleEventDrag>,
}

#[component]
fn TimeSlot(props: TimeSlotProps) -> Element {
    let start = PrimitiveDateTime::new(props.date, props.slot);
    let end = start + Duration::minutes(props.slot_minutes.max(1) as i64);
    let slot_label = format_time(props.slot);
    let dragging_event = props.dragging_event;
    let mut drop_target = props.drop_target;
    let mut slot_selection = props.slot_selection;
    let mut slot_selection_suppressed_click = props.slot_selection_suppressed_click;
    let mut resizing_event = props.resizing_event;
    let mut resize_target = props.resize_target;
    let accepts_schedule_drop =
        props.capabilities.events_drag_and_drop || props.capabilities.event_resize;
    let drop_interaction_active = dragging_event().is_some() || resizing_event().is_some();
    let target_id = format!("time-{start}");
    let active_target_id = target_id.clone();
    let drop_active = resizing_event().is_some() && drop_target() == Some(target_id.clone());
    let selection = slot_selection();
    let selection_range = selection.and_then(|selection| {
        if selection_contains(selection, start) {
            let range = slot_selection_range(selection, props.slot_minutes);
            Some((range.start.to_string(), range.end.to_string()))
        } else {
            None
        }
    });
    let selected = selection_range.is_some();
    let selected_start = selection_range.as_ref().map(|range| range.0.clone());
    let selected_end = selection_range.as_ref().map(|range| range.1.clone());
    let resize_preview = resizing_event().and_then(|resize| {
        resize_target().and_then(|target| {
            let resized =
                resized_event_times(&resize.event, resize.edge, target, props.slot_minutes);
            if resized.new_start == start {
                let mut event = resize.event.clone();
                event.start = resized.new_start;
                event.end = resized.new_end;
                Some(event)
            } else {
                None
            }
        })
    });
    let resize_preview_style = resize_preview.as_ref().map(|event| {
        let slot_minutes = props.slot_minutes.max(1) as f32;
        let duration_minutes = (event.end - event.start).whole_minutes().max(1) as f32;
        let slots = (duration_minutes / slot_minutes).max(1.0);
        format!("--schedule-resize-preview-slots: {slots:.4};")
    });
    rsx! {
        div {
            role: "button",
            tabindex: "0",
            "data-schedule-time-slot": start.to_string(),
            "data-slot-select-enabled": props.capabilities.drag_slot_select,
            "data-drop-enabled": accepts_schedule_drop,
            "data-drop-accepted": accepts_schedule_drop && drop_interaction_active,
            "data-drop-active": drop_active,
            "data-drop-denied": drop_interaction_active && !accepts_schedule_drop,
            "data-selected-range": selected,
            "data-selected-range-start": selected_start,
            "data-selected-range-end": selected_end,
            class: props.class_name,
            onclick: move |_| {
                if let Some(suppressed_start) = slot_selection_suppressed_click() {
                    slot_selection_suppressed_click.set(None);
                    if suppressed_start == start {
                        return;
                    }
                }
                props
                    .on_time_slot_click
                    .call(ScheduleTimeSlotClick {
                        start,
                        end,
                        date: props.date,
                        view: props.view,
                    });
                props
                    .on_event_create
                    .call(ScheduleEventCreate {
                        start,
                        end,
                        date: props.date,
                        all_day: false,
                        view: props.view,
                        source: ScheduleEventCreateSource::TimeSlotClick,
                    });
            },
            onmousedown: move |_| {
                if props.capabilities.drag_slot_select {
                    slot_selection
                        .set(
                            Some(ScheduleSlotSelectionState {
                                anchor: start,
                                current: start,
                            }),
                        );
                }
            },
            onmouseenter: move |_| {
                if resizing_event().is_some() {
                    resize_target.set(Some(start));
                    return;
                }
                if props.capabilities.drag_slot_select {
                    if let Some(mut selection) = slot_selection() {
                        selection.current = start;
                        slot_selection.set(Some(selection));
                    }
                }
            },
            onmouseup: move |_| {
                if props.capabilities.event_resize && resizing_event().is_some() {
                    let resize = resizing_event.take().unwrap();
                    let resized =
                        resized_event_times(&resize.event, resize.edge, start, props.slot_minutes);
                    props
                        .on_event_resize
                        .call(ScheduleEventResize {
                            event_id: resize.event.id.clone(),
                            event: resize.event,
                            new_start: resized.new_start,
                            new_end: resized.new_end,
                            edge: resize.edge,
                            view: props.view,
                        });
                    resize_target.set(None);
                    slot_selection_suppressed_click.set(Some(start));
                    return;
                }
                if props.capabilities.drag_slot_select {
                    if let Some(mut selection) = slot_selection.take() {
                        selection.current = start;
                        let range = slot_selection_range(selection, props.slot_minutes);
                        props
                            .on_slot_drag_end
                            .call(ScheduleSlotRangeSelection {
                                start: range.start,
                                end: range.end,
                                view: props.view,
                            });
                        if selection.anchor != selection.current {
                            props
                                .on_event_create
                                .call(ScheduleEventCreate {
                                    start: range.start,
                                    end: range.end,
                                    date: range.start.date(),
                                    all_day: false,
                                    view: props.view,
                                    source: ScheduleEventCreateSource::TimeSlotDrag,
                                });
                            slot_selection_suppressed_click.set(Some(start));
                        }
                    }
                }
            },
            onpointerup: move |_| {
                if props.capabilities.event_resize && resizing_event().is_some() {
                    let resize = resizing_event.take().unwrap();
                    let resized =
                        resized_event_times(&resize.event, resize.edge, start, props.slot_minutes);
                    props
                        .on_event_resize
                        .call(ScheduleEventResize {
                            event_id: resize.event.id.clone(),
                            event: resize.event,
                            new_start: resized.new_start,
                            new_end: resized.new_end,
                            edge: resize.edge,
                            view: props.view,
                        });
                    resize_target.set(None);
                    slot_selection_suppressed_click.set(Some(start));
                }
            },
            ondragover: move |event| {
                if accepts_schedule_drop {
                    event.prevent_default();
                }
                if resizing_event().is_some() {
                    resize_target.set(Some(start));
                }
            },
            ondragenter: {
                let target_id = target_id.clone();
                move |event| {
                    if accepts_schedule_drop {
                        event.prevent_default();
                    }
                    drop_target.set(Some(target_id.clone()));
                    if resizing_event().is_some() {
                        resize_target.set(Some(start));
                    }
                }
            },
            ondragleave: {
                let target_id = target_id.clone();
                move |_| {
                    if drop_target() == Some(target_id.clone()) {
                        drop_target.set(None);
                    }
                    if resize_target() == Some(start) {
                        resize_target.set(None);
                    }
                }
            },
            ondrop: move |event: Event<DragData>| {
                event.prevent_default();
                if drop_target() == Some(active_target_id.clone()) {
                    drop_target.set(None);
                }
                if props.capabilities.event_resize && resizing_event().is_some() {
                    let resize = resizing_event.take().unwrap();
                    let resized =
                        resized_event_times(&resize.event, resize.edge, start, props.slot_minutes);
                    props
                        .on_event_resize
                        .call(ScheduleEventResize {
                            event_id: resize.event.id.clone(),
                            event: resize.event,
                            new_start: resized.new_start,
                            new_end: resized.new_end,
                            edge: resize.edge,
                            view: props.view,
                        });
                    resize_target.set(None);
                    slot_selection_suppressed_click.set(Some(start));
                } else if props.capabilities.events_drag_and_drop
                    && move_dragged_event_from_drop(
                        dragging_event,
                        &event,
                        &props.all_events,
                        ScheduleDropContext {
                            new_start: start,
                            date: props.date,
                            destination: ScheduleDropDestination::Timed,
                            view: props.view,
                            slot_minutes: Some(props.slot_minutes),
                        },
                        props.on_event_drop,
                    )
                {} else if props.capabilities.events_drag_and_drop {
                    let external = external_drop_data(&event);
                    props
                        .on_external_event_drop
                        .call(ScheduleExternalDrop {
                            data: external.as_ref().map(|data| data.data.clone()),
                            data_format: external.map(|data| data.format),
                            start,
                            end,
                            date: props.date,
                            view: props.view,
                        });
                }
            },
            span { "data-schedule-time-slot-label": true, "{slot_label}" }
            if let Some(preview) = resize_preview {
                article {
                    "data-schedule-event": preview.id.clone(),
                    "data-schedule-resize-preview": true,
                    "data-color": preview.color.clone().unwrap_or_default(),
                    "data-all-day": false,
                    "data-draggable": false,
                    "data-resizable": false,
                    "style": resize_preview_style,
                    strong { "{preview.title}" }
                    span { "data-schedule-event-time": true,
                        " {format_time_range(preview.start, preview.end)}"
                    }
                }
            }
        }
    }
}

#[component]
fn MonthView(mut props: ScheduleViewBodyProps) -> Element {
    let days = month_grid_dates(props.date, props.month_view.first_day_of_week);
    let weekday_labels = month_weekday_labels(props.month_view.first_day_of_week, &props.locale);
    rsx! {
        section {
            "data-schedule-view": "month",
            "data-mobile": props.mobile,
            "data-default-header": props.month_view.with_default_header,
            class: props.class_names.month_view.clone(),
            div { "data-schedule-month-weekdays": true,
                for label in weekday_labels {
                    span { "data-schedule-month-weekday": true, "{label}" }
                }
            }
            for week in days.chunks(7) {
                div { "data-schedule-month-week": true,
                    div { "data-schedule-month-week-days": true,
                        for day in week.iter().copied() {
                            div {
                                "data-schedule-month-day": day.to_string(),
                                "data-outside-month": day.month() != props.date.month(),
                                "data-current-day": is_current_day(day),
                                "data-drop-enabled": props.capabilities.events_drag_and_drop,
                                "data-drop-accepted": props.capabilities.events_drag_and_drop && (props.dragging_event)().is_some(),
                                "data-drop-active": (props.drop_target)() == Some(format!("month-{day}")),
                                "data-drop-denied": (props.dragging_event)().is_some() && !props.capabilities.events_drag_and_drop,
                                class: props.class_names.month_day.clone(),
                                onmouseup: move |_| {
                                    if props.capabilities.events_drag_and_drop {
                                        let start = PrimitiveDateTime::new(day, Time::MIDNIGHT);
                                        move_dragged_event(
                                            props.dragging_event,
                                            start,
                                            day,
                                            ScheduleDropDestination::Timed,
                                            props.view,
                                            props.on_event_drag_end,
                                            props.on_event_drop,
                                        );
                                    }
                                },
                                onpointerup: move |_| {
                                    if props.capabilities.events_drag_and_drop {
                                        let start = PrimitiveDateTime::new(day, Time::MIDNIGHT);
                                        move_dragged_event(
                                            props.dragging_event,
                                            start,
                                            day,
                                            ScheduleDropDestination::Timed,
                                            props.view,
                                            props.on_event_drag_end,
                                            props.on_event_drop,
                                        );
                                    }
                                },
                                ondragover: move |event| {
                                    if props.capabilities.events_drag_and_drop {
                                        event.prevent_default();
                                    }
                                },
                                ondragenter: move |event| {
                                    if props.capabilities.events_drag_and_drop {
                                        event.prevent_default();
                                        props.drop_target.set(Some(format!("month-{day}")));
                                    }
                                },
                                ondragleave: move |_| {
                                    if (props.drop_target)() == Some(format!("month-{day}")) {
                                        props.drop_target.set(None);
                                    }
                                },
                                ondrop: {
                                    let events = props.events.clone();
                                    move |event: Event<DragData>| {
                                        if props.capabilities.events_drag_and_drop {
                                            event.prevent_default();
                                            if (props.drop_target)() == Some(format!("month-{day}")) {
                                                props.drop_target.set(None);
                                            }
                                            let start = PrimitiveDateTime::new(day, Time::MIDNIGHT);
                                            let end = start + Duration::days(1);
                                            if !move_dragged_event_from_drop(
                                                props.dragging_event,
                                                &event,
                                                &events,
                                                ScheduleDropContext {
                                                    new_start: start,
                                                    date: day,
                                                    destination: ScheduleDropDestination::Timed,
                                                    view: props.view,
                                                    slot_minutes: None,
                                                },
                                                props.on_event_drop,
                                            ) {
                                                let external = external_drop_data(&event);
                                                props
                                                    .on_external_event_drop
                                                    .call(ScheduleExternalDrop {
                                                        data: external.as_ref().map(|data| data.data.clone()),
                                                        data_format: external.map(|data| data.format),
                                                        start,
                                                        end,
                                                        date: day,
                                                        view: props.view,
                                                    });
                                            }
                                        }
                                    }
                                },
                                button {
                                    "type": "button",
                                    "data-schedule-month-day-button": day.to_string(),
                                    onclick: move |_| {
                                        props
                                            .on_day_click
                                            .call(ScheduleDayClick {
                                                date: day,
                                                view: props.view,
                                            });
                                    },
                                    {format_day_of_month_label(day)}
                                }
                            }
                        }
                    }
                    div { "data-schedule-month-week-events": true,
                        for segment in month_event_segments_for_week(&props.events, week) {
                            ScheduleEventNode {
                                event: segment.event,
                                view: props.view,
                                date: segment.start_date,
                                capabilities: props.capabilities,
                                dragging_event: props.dragging_event,
                                resizing_event: props.resizing_event,
                                resize_target: props.resize_target,
                                can_drag_event: props.can_drag_event,
                                can_resize_event: props.can_resize_event,
                                render_event_body: props.render_event_body,
                                on_event_click: props.on_event_click,
                                on_event_drag_start: props.on_event_drag_start,
                                on_event_drag_end: props.on_event_drag_end,
                                on_event_resize: props.on_event_resize,
                                class_name: props.class_names.event.clone(),
                                layout_style: format!(
                                    "grid-column: {} / span {};",
                                    segment.start_column + 1,
                                    segment.column_span,
                                ),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MobileMonthView(mut props: ScheduleViewBodyProps) -> Element {
    let days = month_grid_dates(props.date, props.month_view.first_day_of_week);
    let mobile_events_by_day: Vec<_> = days
        .iter()
        .copied()
        .map(|day| (day, filter_events_for_date(&props.events, day)))
        .collect();
    rsx! {
        section {
            "data-schedule-view": "mobile-month",
            "data-mobile": true,
            "data-mobile-month-view": true,
            "data-default-header": props.mobile_month_view.with_default_header,
            class: props.class_names.mobile_month_view.clone(),
            for (day, events) in mobile_events_by_day {
                div {
                    "data-schedule-mobile-month-day": day.to_string(),
                    "data-current-day": is_current_day(day),
                    "data-outside-month": day.month() != props.date.month(),
                    "data-drop-enabled": props.capabilities.events_drag_and_drop,
                    "data-drop-accepted": props.capabilities.events_drag_and_drop && (props.dragging_event)().is_some(),
                    "data-drop-active": (props.drop_target)() == Some(format!("mobile-month-{day}")),
                    "data-drop-denied": (props.dragging_event)().is_some() && !props.capabilities.events_drag_and_drop,
                    class: props.class_names.month_day.clone(),
                    onmouseup: move |_| {
                        if props.capabilities.events_drag_and_drop {
                            let start = PrimitiveDateTime::new(day, Time::MIDNIGHT);
                            move_dragged_event(
                                props.dragging_event,
                                start,
                                day,
                                ScheduleDropDestination::Timed,
                                props.view,
                                props.on_event_drag_end,
                                props.on_event_drop,
                            );
                        }
                    },
                    onpointerup: move |_| {
                        if props.capabilities.events_drag_and_drop {
                            let start = PrimitiveDateTime::new(day, Time::MIDNIGHT);
                            move_dragged_event(
                                props.dragging_event,
                                start,
                                day,
                                ScheduleDropDestination::Timed,
                                props.view,
                                props.on_event_drag_end,
                                props.on_event_drop,
                            );
                        }
                    },
                    ondragover: move |event| {
                        if props.capabilities.events_drag_and_drop {
                            event.prevent_default();
                        }
                    },
                    ondragenter: move |event| {
                        if props.capabilities.events_drag_and_drop {
                            event.prevent_default();
                            props.drop_target.set(Some(format!("mobile-month-{day}")));
                        }
                    },
                    ondragleave: move |_| {
                        if (props.drop_target)() == Some(format!("mobile-month-{day}")) {
                            props.drop_target.set(None);
                        }
                    },
                    ondrop: {
                        let events = props.events.clone();
                        move |event: Event<DragData>| {
                            if props.capabilities.events_drag_and_drop {
                                event.prevent_default();
                                if (props.drop_target)() == Some(format!("mobile-month-{day}")) {
                                    props.drop_target.set(None);
                                }
                                let start = PrimitiveDateTime::new(day, Time::MIDNIGHT);
                                let end = start + Duration::days(1);
                                if !move_dragged_event_from_drop(
                                    props.dragging_event,
                                    &event,
                                    &events,
                                    ScheduleDropContext {
                                        new_start: start,
                                        date: day,
                                        destination: ScheduleDropDestination::Timed,
                                        view: props.view,
                                        slot_minutes: None,
                                    },
                                    props.on_event_drop,
                                ) {
                                    let external = external_drop_data(&event);
                                    props
                                        .on_external_event_drop
                                        .call(ScheduleExternalDrop {
                                            data: external.as_ref().map(|data| data.data.clone()),
                                            data_format: external.map(|data| data.format),
                                            start,
                                            end,
                                            date: day,
                                            view: props.view,
                                        });
                                }
                            }
                        }
                    },
                    button {
                        "type": "button",
                        "data-schedule-mobile-month-day-button": day.to_string(),
                        onclick: move |_| {
                            props
                                .on_day_click
                                .call(ScheduleDayClick {
                                    date: day,
                                    view: props.view,
                                });
                        },
                        {format_day_of_month_label(day)}
                    }
                    for event in events {
                        ScheduleEventNode {
                            event,
                            view: props.view,
                            date: day,
                            capabilities: props.capabilities,
                            dragging_event: props.dragging_event,
                            resizing_event: props.resizing_event,
                            resize_target: props.resize_target,
                            can_drag_event: props.can_drag_event,
                            can_resize_event: props.can_resize_event,
                            render_event_body: props.render_event_body,
                            on_event_click: props.on_event_click,
                            on_event_drag_start: props.on_event_drag_start,
                            on_event_drag_end: props.on_event_drag_end,
                            on_event_resize: props.on_event_resize,
                            class_name: props.class_names.event.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn YearView(mut props: ScheduleViewBodyProps) -> Element {
    let weekday_labels = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    rsx! {
        section {
            "data-schedule-view": "year",
            "data-default-header": props.year_view.with_default_header,
            "data-mobile": props.mobile,
            class: props.class_names.year_view.clone(),
            for month_number in 1u8..=12 {
                {
                    let month = Month::try_from(month_number).unwrap();
                    let month_label = format!("{month:?}");
                    let month_date =
                        Date::from_calendar_date(props.date.year(), month, 1).unwrap();
                    let days = month_grid_dates(month_date, props.month_view.first_day_of_week);
                    rsx! {
                        button {
                            "type": "button",
                            "data-schedule-year-month": month_number,
                            "data-current-month": props.date.month() == month,
                            "data-drop-enabled": props.capabilities.events_drag_and_drop,
                            "data-drop-accepted": props.capabilities.events_drag_and_drop && (props.dragging_event)().is_some(),
                            "data-drop-active": (props.drop_target)() == Some(format!("year-{month_number}")),
                            "data-drop-denied": (props.dragging_event)().is_some() && !props.capabilities.events_drag_and_drop,
                            onmouseup: move |_| {
                                if props.capabilities.events_drag_and_drop {
                                    let date = year_month_transition(props.date, month_number);
                                    let start = PrimitiveDateTime::new(date, Time::MIDNIGHT);
                                    move_dragged_event(
                                        props.dragging_event,
                                        start,
                                        date,
                                        ScheduleDropDestination::Timed,
                                        props.view,
                                        props.on_event_drag_end,
                                        props.on_event_drop,
                                    );
                                }
                            },
                            onpointerup: move |_| {
                                if props.capabilities.events_drag_and_drop {
                                    let date = year_month_transition(props.date, month_number);
                                    let start = PrimitiveDateTime::new(date, Time::MIDNIGHT);
                                    move_dragged_event(
                                        props.dragging_event,
                                        start,
                                        date,
                                        ScheduleDropDestination::Timed,
                                        props.view,
                                        props.on_event_drag_end,
                                        props.on_event_drop,
                                    );
                                }
                            },
                            ondragover: move |event| {
                                if props.capabilities.events_drag_and_drop {
                                    event.prevent_default();
                                }
                            },
                            ondragenter: move |event| {
                                if props.capabilities.events_drag_and_drop {
                                    event.prevent_default();
                                    props.drop_target.set(Some(format!("year-{month_number}")));
                                }
                            },
                            ondragleave: move |_| {
                                if (props.drop_target)() == Some(format!("year-{month_number}")) {
                                    props.drop_target.set(None);
                                }
                            },
                            ondrop: {
                                let events = props.events.clone();
                                move |event: Event<DragData>| {
                                    if props.capabilities.events_drag_and_drop {
                                        event.prevent_default();
                                        if (props.drop_target)() == Some(format!("year-{month_number}")) {
                                            props.drop_target.set(None);
                                        }
                                        let date = year_month_transition(props.date, month_number);
                                        let start = PrimitiveDateTime::new(date, Time::MIDNIGHT);
                                        let end = start + Duration::days(1);
                                        if !move_dragged_event_from_drop(
                                            props.dragging_event,
                                            &event,
                                            &events,
                                            ScheduleDropContext {
                                                new_start: start,
                                                date,
                                                destination: ScheduleDropDestination::Timed,
                                                view: props.view,
                                                slot_minutes: None,
                                            },
                                            props.on_event_drop,
                                        ) {
                                            let external = external_drop_data(&event);
                                            props
                                                .on_external_event_drop
                                                .call(ScheduleExternalDrop {
                                                    data: external.as_ref().map(|data| data.data.clone()),
                                                    data_format: external.map(|data| data.format),
                                                    start,
                                                    end,
                                                    date,
                                                    view: props.view,
                                                });
                                        }
                                    }
                                }
                            },
                            onclick: move |_| {
                                let next = year_month_transition(props.date, month_number);
                                props.on_date.call(next);
                                props.on_view.call(ScheduleView::Month);
                            },
                            div { "data-schedule-year-month-header": true,
                                span { "data-schedule-year-month-label": true, "{month_label}" }
                                span { "data-schedule-year-month-meta": true, "{props.date.year()}" }
                            }
                            div { "data-schedule-year-weekdays": true,
                                for offset in 0..7 {
                                    {
                                        let index = (props.month_view.first_day_of_week.number_days_from_sunday()
                                            as usize + offset) % 7;
                                        rsx! {
                                            span { "data-schedule-year-weekday": true, "{weekday_labels[index]}" }
                                        }
                                    }
                                }
                            }
                            div { "data-schedule-year-days": true,
                                for day in days {
                                    {
                                        let day_events = filter_events_for_date(&props.events, day);
                                        let visible_dots = day_events.iter().take(3).cloned().collect::<Vec<_>>();
                                        let overflow_count = day_events.len().saturating_sub(visible_dots.len());
                                        let is_selected_day = day == props.date;
                                        rsx! {
                                            div {
                                                "data-schedule-year-day": true,
                                                "data-outside-month": day.month() != month,
                                                "data-current-day": is_current_day(day),
                                                "data-selected-day": is_selected_day,
                                                "data-weekend": matches!(day.weekday(), time::Weekday::Saturday | time::Weekday::Sunday),
                                                span { "data-schedule-year-day-number": true, "{day.day()}" }
                                                div { "data-schedule-year-day-dots": true,
                                                    for event in visible_dots {
                                                        span {
                                                            "data-schedule-event-dot": true,
                                                            "data-color": event.color.clone().unwrap_or_default(),
                                                            title: event.title,
                                                        }
                                                    }
                                                    if overflow_count > 0 {
                                                        span { "data-schedule-year-day-overflow": true, "+{overflow_count}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ScheduleEventNodeProps {
    event: ScheduleEvent,
    view: ScheduleView,
    date: Date,
    capabilities: ScheduleCapabilities,
    dragging_event: Signal<Option<ScheduleEvent>>,
    resizing_event: Signal<Option<ScheduleResizeState>>,
    resize_target: Signal<Option<PrimitiveDateTime>>,
    can_drag_event: Callback<ScheduleEvent, bool>,
    can_resize_event: Callback<ScheduleEvent, bool>,
    render_event_body: Option<Callback<ScheduleEventRenderContext, Element>>,
    on_event_click: Callback<ScheduleEventClick>,
    on_event_drag_start: Callback<ScheduleEventDrag>,
    on_event_drag_end: Callback<ScheduleEventDrag>,
    on_event_resize: Callback<ScheduleEventResize>,
    #[props(default = 60)]
    slot_minutes: u8,
    #[props(default)]
    class_name: String,
    #[props(default)]
    layout_column: usize,
    #[props(default = 1)]
    layout_columns: usize,
    #[props(default)]
    layout_style: String,
}

#[component]
fn ScheduleEventNode(props: ScheduleEventNodeProps) -> Element {
    let draggable =
        props.capabilities.events_drag_and_drop && props.can_drag_event.call(props.event.clone());
    let resizable = !props.event.all_day
        && props.capabilities.event_resize
        && props.can_resize_event.call(props.event.clone());
    let event_id = props.event.id.clone();
    let event_title = props.event.title.clone();
    let event_color = props.event.color.clone().unwrap_or_default();
    let event_label = props
        .event
        .description
        .clone()
        .unwrap_or_else(|| props.event.title.clone());
    let event_time = if props.event.all_day {
        "All day".to_string()
    } else {
        format_time_range(props.event.start, props.event.end)
    };
    let click_event = props.event.clone();
    let drag_start_event = props.event.clone();
    let drag_end_event = props.event.clone();
    let mut drag_start_signal = props.dragging_event;
    let drag_end_signal = props.dragging_event;
    let mut resize_start_signal = props.resizing_event;
    let mut resize_end_signal = props.resizing_event;
    let mut resize_start_target = props.resize_target;
    let mut resize_end_target = props.resize_target;
    let mut resize_start_drag_signal = props.dragging_event;
    let mut resize_end_drag_signal = props.dragging_event;
    let resize_slot_duration = Duration::minutes(props.slot_minutes.max(1) as i64);
    let resize_start_pointer_event = props.event.clone();
    let resize_end_pointer_event = props.event.clone();
    let context = ScheduleEventRenderContext {
        event: props.event.clone(),
        view: props.view,
        date: props.date,
        draggable,
        resizable,
    };
    let is_resize_source = (props.resizing_event)()
        .as_ref()
        .is_some_and(|resize| resize.event.id == event_id.as_str());
    let event_draggable = draggable && !is_resize_source;
    rsx! {
        article {
            "data-schedule-event": event_id.clone(),
            "data-color": event_color,
            "data-all-day": props.event.all_day,
            "data-draggable": event_draggable,
            "data-resizable": resizable,
            "data-drag-disabled": !draggable,
            "data-resize-disabled": !resizable,
            "data-resize-source": is_resize_source,
            "data-disabled": !draggable && !resizable,
            "data-layout-column": props.layout_column,
            "data-layout-columns": props.layout_columns,
            "title": event_label,
            class: props.class_name,
            style: (!props.layout_style.is_empty()).then_some(props.layout_style.clone()),
            draggable: event_draggable,
            onclick: move |event| {
                event.stop_propagation();
                props
                    .on_event_click
                    .call(ScheduleEventClick {
                        event: click_event.clone(),
                        view: props.view,
                    });
            },
            onmousedown: move |event| {
                event.stop_propagation();
            },
            onpointerdown: move |event: Event<PointerData>| {
                event.stop_propagation();
            },
            ondragstart: move |event: Event<DragData>| {
                if (props.resizing_event)().is_some() {
                    event.prevent_default();
                    return;
                }
                if event_draggable {
                    event.data_transfer().set_effect_allowed("move");
                    event.data_transfer().set_drop_effect("move");
                    let _ = event.data_transfer().set_data("text/plain", &drag_start_event.id);
                    let _ = event
                        .data_transfer()
                        .set_data("application/x-dioxus-schedule-event", &drag_start_event.id);
                    drag_start_signal.set(Some(drag_start_event.clone()));
                    props
                        .on_event_drag_start
                        .call(ScheduleEventDrag {
                            event: drag_start_event.clone(),
                            view: props.view,
                        });
                }
            },
            ondragend: move |_| {
                if event_draggable {
                    clear_dragging_event(drag_end_signal);
                    props
                        .on_event_drag_end
                        .call(ScheduleEventDrag {
                            event: drag_end_event.clone(),
                            view: props.view,
                        });
                }
            },
            if let Some(render) = props.render_event_body {
                {render.call(context)}
            } else {
                strong { "{event_title}" }
                span { "data-schedule-event-time": true, " {event_time}" }
            }
            if resizable {
                button {
                    "type": "button",
                    "data-schedule-resize-handle": "start",
                    draggable: false,
                    onmousedown: move |event| {
                        event.stop_propagation();
                        event.prevent_default();
                    },
                    onpointerdown: move |event| {
                        event.stop_propagation();
                        event.prevent_default();
                        resize_start_drag_signal.set(None);
                        resize_start_target.set(Some(resize_start_pointer_event.start));
                        resize_start_signal
                            .set(
                                Some(ScheduleResizeState {
                                    event: resize_start_pointer_event.clone(),
                                    edge: ScheduleResizeEdge::Start,
                                }),
                            );
                    },
                    onclick: move |event| {
                        event.stop_propagation();
                    },
                    ondragstart: move |event: Event<DragData>| {
                        event.stop_propagation();
                        event.prevent_default();
                    },
                    ondragend: move |_| {
                        resize_start_drag_signal.set(None);
                        resize_start_signal.set(None);
                        resize_start_target.set(None);
                    },
                    "Resize start"
                }
                button {
                    "type": "button",
                    "data-schedule-resize-handle": "end",
                    draggable: false,
                    onmousedown: move |event| {
                        event.stop_propagation();
                        event.prevent_default();
                    },
                    onpointerdown: move |event| {
                        event.stop_propagation();
                        event.prevent_default();
                        resize_end_drag_signal.set(None);
                        resize_end_target.set(Some(
                            resize_end_pointer_event.end - resize_slot_duration
                        ));
                        resize_end_signal
                            .set(
                                Some(ScheduleResizeState {
                                    event: resize_end_pointer_event.clone(),
                                    edge: ScheduleResizeEdge::End,
                                }),
                            );
                    },
                    onclick: move |event| {
                        event.stop_propagation();
                    },
                    ondragstart: move |event: Event<DragData>| {
                        event.stop_propagation();
                        event.prevent_default();
                    },
                    ondragend: move |_| {
                        resize_end_drag_signal.set(None);
                        resize_end_signal.set(None);
                        resize_end_target.set(None);
                    },
                    "Resize end"
                }
            }
        }
    }
}

pub(crate) fn all_day_event_create(
    date: Date,
    view: ScheduleView,
    source: ScheduleEventCreateSource,
) -> ScheduleEventCreate {
    let start = PrimitiveDateTime::new(date, Time::MIDNIGHT);
    ScheduleEventCreate {
        start,
        end: start + Duration::days(1),
        date,
        all_day: true,
        view,
        source,
    }
}

pub(crate) fn clear_dragging_event(mut dragging_event: Signal<Option<ScheduleEvent>>) {
    dragging_event.set(None);
}

fn move_dragged_event(
    mut dragging_event: Signal<Option<ScheduleEvent>>,
    new_start: PrimitiveDateTime,
    date: Date,
    destination: ScheduleDropDestination,
    view: ScheduleView,
    on_event_drag_end: Callback<ScheduleEventDrag>,
    on_event_drop: Callback<ScheduleEventDrop>,
) -> bool {
    let Some(event) = dragging_event.take() else {
        return false;
    };
    let drag = ScheduleEventDrag {
        event: event.clone(),
        view,
    };
    on_event_drop.call(build_event_drop(
        event,
        new_start,
        date,
        destination,
        view,
        None,
    ));
    on_event_drag_end.call(drag);
    true
}

fn move_dragged_event_from_drop(
    mut dragging_event: Signal<Option<ScheduleEvent>>,
    drop_event: &Event<DragData>,
    events: &[ScheduleEvent],
    context: ScheduleDropContext,
    on_event_drop: Callback<ScheduleEventDrop>,
) -> bool {
    let dragged_event = dragging_event
        .take()
        .or_else(|| event_from_drop_data(drop_event, events));
    let Some(event) = dragged_event else {
        return false;
    };
    on_event_drop.call(build_event_drop(
        event,
        context.new_start,
        context.date,
        context.destination,
        context.view,
        context.slot_minutes,
    ));
    true
}

#[derive(Clone, Copy)]
struct ScheduleDropContext {
    new_start: PrimitiveDateTime,
    date: Date,
    destination: ScheduleDropDestination,
    view: ScheduleView,
    slot_minutes: Option<u8>,
}

pub(crate) fn build_event_drop(
    event: ScheduleEvent,
    new_start: PrimitiveDateTime,
    date: Date,
    destination: ScheduleDropDestination,
    view: ScheduleView,
    slot_minutes: Option<u8>,
) -> ScheduleEventDrop {
    let duration = match (destination, event.all_day, slot_minutes) {
        (ScheduleDropDestination::Timed, true, Some(slot_minutes)) => {
            Duration::minutes(slot_minutes.max(1) as i64)
        }
        _ => event.end - event.start,
    };
    ScheduleEventDrop {
        event_id: event.id.clone(),
        event,
        destination,
        new_start,
        new_end: new_start + duration,
        date,
        view,
    }
}

fn timed_event_style(
    geometry: super::utils::TimedEventGeometry,
    column: usize,
    columns: usize,
) -> String {
    let columns = columns.max(1) as f32;
    let width = 100.0 / columns;
    let left = column as f32 * width;
    format!(
        "top: calc(var(--schedule-time-slot-size) * {:.4} + 2px); height: calc(var(--schedule-time-slot-size) * {:.4} - 2px); left: calc({:.4}% + 4px); width: calc({:.4}% - 8px);",
        geometry.top_slots, geometry.height_slots, left, width,
    )
}

pub(crate) fn timed_event_spans_multiple_days(event: &ScheduleEvent) -> bool {
    let inclusive_end = event.end - Duration::nanoseconds(1);
    event.end > event.start && event.start.date() != inclusive_end.date()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TimedSpanningEventGeometry {
    pub(crate) start_column: usize,
    pub(crate) column_span: usize,
    pub(crate) day_count: usize,
    pub(crate) start_date: Date,
    pub(crate) top_slots: f32,
    pub(crate) height_slots: f32,
}

pub(crate) fn timed_spanning_event_geometry(
    event: &ScheduleEvent,
    days: &[Date],
    config: ScheduleTimeGridConfig,
) -> Option<TimedSpanningEventGeometry> {
    if event.all_day || !timed_event_spans_multiple_days(event) {
        return None;
    }

    let mut start_column = None::<usize>;
    let mut end_column = None::<usize>;
    let mut top_slots = None::<f32>;
    let mut bottom_slots = None::<f32>;

    for (index, day) in days.iter().copied().enumerate() {
        let Some(geometry) = timed_event_geometry(event, day, config) else {
            continue;
        };

        start_column.get_or_insert(index);
        end_column = Some(index);
        top_slots = Some(top_slots.map_or(geometry.top_slots, |top| top.min(geometry.top_slots)));
        bottom_slots = Some(
            bottom_slots.map_or(geometry.top_slots + geometry.height_slots, |bottom| {
                bottom.max(geometry.top_slots + geometry.height_slots)
            }),
        );
    }

    let start_column = start_column?;
    let end_column = end_column?;
    let top_slots = top_slots?;
    let bottom_slots = bottom_slots?;
    if bottom_slots <= top_slots {
        return None;
    }

    Some(TimedSpanningEventGeometry {
        start_column,
        column_span: end_column.saturating_sub(start_column) + 1,
        day_count: days.len().max(1),
        start_date: days[start_column],
        top_slots,
        height_slots: bottom_slots - top_slots,
    })
}

fn timed_spanning_event_style(geometry: TimedSpanningEventGeometry) -> String {
    let day_count = geometry.day_count.max(1) as f32;
    let day_width = 100.0 / day_count;
    let left = geometry.start_column as f32 * day_width;
    let width = geometry.column_span as f32 * day_width;
    format!(
        "top: calc(var(--schedule-time-slot-size) * {:.4} + 2px); height: calc(var(--schedule-time-slot-size) * {:.4} - 2px); left: calc({:.4}% + 4px); width: calc({:.4}% - 8px); pointer-events: auto;",
        geometry.top_slots, geometry.height_slots, left, width,
    )
}

pub(crate) fn timed_drop_preview_style(
    drop_target: Option<String>,
    days: &[Date],
    config: ScheduleTimeGridConfig,
    dragging_event: Option<&ScheduleEvent>,
) -> Option<String> {
    let dragging_event = dragging_event?;
    let target = drop_target?;
    let target_start = time_slots(config)
        .into_iter()
        .flat_map(|slot| {
            days.iter()
                .copied()
                .map(move |day| PrimitiveDateTime::new(day, slot))
        })
        .find(|start| target == format!("time-{start}"))?;
    let day_index = days.iter().position(|day| *day == target_start.date())?;
    let duration = if dragging_event.all_day {
        Duration::minutes(config.slot_minutes.max(1) as i64)
    } else {
        (dragging_event.end - dragging_event.start)
            .max(Duration::minutes(config.slot_minutes.max(1) as i64))
    };
    let mut preview = dragging_event.clone();
    preview.start = target_start;
    preview.end = target_start + duration;
    preview.all_day = false;
    if let Some(geometry) = timed_spanning_event_geometry(&preview, days, config) {
        return Some(timed_spanning_event_style(geometry));
    }
    let geometry = timed_event_geometry(&preview, target_start.date(), config)?;
    let day_count = days.len().max(1) as f32;
    let width = 100.0 / day_count;
    let left = day_index as f32 * width;
    Some(format!(
        "top: calc(var(--schedule-time-slot-size) * {:.4} + 2px); height: calc(var(--schedule-time-slot-size) * {:.4} - 2px); left: calc({:.4}% + 4px); width: calc({:.4}% - 8px);",
        geometry.top_slots, geometry.height_slots, left, width,
    ))
}

pub(crate) fn time_slot_drop_active(
    drop_target: Option<String>,
    slot_start: PrimitiveDateTime,
    slot_minutes: u8,
    dragging_event: Option<&ScheduleEvent>,
) -> bool {
    let Some(drop_target) = drop_target else {
        return false;
    };
    let _ = (slot_minutes, dragging_event);
    drop_target == format!("time-{slot_start}")
}

fn event_from_drop_data(
    drop_event: &Event<DragData>,
    events: &[ScheduleEvent],
) -> Option<ScheduleEvent> {
    let event_id = drop_event
        .data_transfer()
        .get_data("application/x-dioxus-schedule-event")
        .or_else(|| drop_event.data_transfer().get_data("text/plain"))?;

    events.iter().find(|event| event.id == event_id).cloned()
}
