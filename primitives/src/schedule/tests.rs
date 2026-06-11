use dioxus::prelude::*;
use time::macros::{date, time};
use time::{Date, Duration, PrimitiveDateTime};

use super::state::ScheduleCapabilities;
use super::state::ScheduleSlotSelectionState;
use super::types::{
    ScheduleDropDestination, ScheduleEvent, ScheduleEventCreateSource, ScheduleEventRenderContext,
    ScheduleLayout, ScheduleMode, ScheduleRecurrence, ScheduleRecurrenceExpansionLimit,
    ScheduleRecurrenceFrequency, ScheduleResizeEdge, ScheduleTimeGridConfig,
    ScheduleWeekViewConfig,
};
use super::utils::{
    current_time_line_offset, expand_event, external_drop_data_from_formats,
    filter_events_for_date, format_day_of_month_label, is_current_day, layout_overlapping_events,
    month_event_segments_for_week, month_weekday_labels, resized_event_times, selection_contains,
    slot_selection_range, timed_event_geometry, year_month_transition,
};
use super::{add_months, shift_date, today};

fn event(id: &str, start: PrimitiveDateTime, end: PrimitiveDateTime) -> ScheduleEvent {
    ScheduleEvent {
        id: id.to_string(),
        title: id.to_string(),
        start,
        end,
        all_day: false,
        color: None,
        description: None,
        recurrence: None,
        drag_disabled: false,
        resize_disabled: false,
    }
}

#[component]
fn ClearDraggingEventHarness() -> Element {
    let dragging = use_signal(|| {
        Some(event(
            "dragged",
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:00)),
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:00)),
        ))
    });

    use_hook(move || {
        super::components::clear_dragging_event(dragging);
    });

    rsx! {
        div { "{dragging().is_some()}" }
    }
}

#[component]
fn ResponsiveMobileMonthMultiDayHarness() -> Element {
    rsx! {
        super::components::Schedule {
            default_date: date!(2026 - 05 - 01),
            default_view: super::types::ScheduleView::Month,
            layout: ScheduleLayout::Responsive,
            with_default_header: false,
            events: vec![event(
                "conference",
                PrimitiveDateTime::new(date!(2026 - 05 - 04), time!(09:00)),
                PrimitiveDateTime::new(date!(2026 - 05 - 07), time!(17:00)),
            )],
            render_event_body: Some(Callback::new(|context: ScheduleEventRenderContext| {
                rsx! {
                    span { "data-render-date": context.date.to_string(), "{context.date}" }
                }
            })),
        }
    }
}

#[component]
fn WeekTimedMultiDayHarness() -> Element {
    rsx! {
        super::components::Schedule {
            default_date: date!(2026 - 05 - 05),
            default_view: super::types::ScheduleView::Week,
            layout: ScheduleLayout::Default,
            with_default_header: false,
            week_view: ScheduleWeekViewConfig {
                time_grid: ScheduleTimeGridConfig {
                    start_hour: 8,
                    end_hour: 18,
                    slot_minutes: 60,
                    with_default_header: true,
                },
                ..Default::default()
            },
            events: vec![
                event(
                    "conference",
                    PrimitiveDateTime::new(date!(2026 - 05 - 04), time!(10:00)),
                    PrimitiveDateTime::new(date!(2026 - 05 - 07), time!(12:00)),
                ),
                event(
                    "standup",
                    PrimitiveDateTime::new(date!(2026 - 05 - 05), time!(09:00)),
                    PrimitiveDateTime::new(date!(2026 - 05 - 05), time!(10:00)),
                ),
            ],
            render_event_body: Some(Callback::new(|context: ScheduleEventRenderContext| {
                rsx! {
                    span { "data-render-date": context.date.to_string(), "{context.event.id}" }
                }
            })),
        }
    }
}

#[test]
fn recurrence_expansion_respects_limit() {
    let mut event = event(
        "daily",
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:00)),
    );
    event.recurrence = Some(ScheduleRecurrence {
        frequency: ScheduleRecurrenceFrequency::Daily,
        interval: 1,
        count: Some(10),
        until: None,
    });

    let expanded = expand_event(
        &event,
        ScheduleRecurrenceExpansionLimit { max_occurrences: 3 },
    );

    assert_eq!(expanded.len(), 3);
    assert_eq!(expanded[2].start.date(), date!(2026 - 05 - 03));
}

#[test]
fn event_filtering_includes_intersecting_events() {
    let events = vec![
        event(
            "overnight",
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(23:00)),
            PrimitiveDateTime::new(date!(2026 - 05 - 02), time!(01:00)),
        ),
        event(
            "other",
            PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(09:00)),
            PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(10:00)),
        ),
    ];

    let filtered = filter_events_for_date(&events, date!(2026 - 05 - 02));

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id, "overnight");
}

#[test]
fn month_event_segments_span_each_intersecting_week_once() {
    let events = vec![event(
        "conference",
        PrimitiveDateTime::new(date!(2026 - 05 - 04), time!(09:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 07), time!(17:00)),
    )];
    let week = (4..=10)
        .map(|day| date!(2026 - 05 - 01).replace_day(day).unwrap())
        .collect::<Vec<_>>();

    let segments = month_event_segments_for_week(&events, &week);

    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].event.id, "conference");
    assert_eq!(segments[0].start_column, 0);
    assert_eq!(segments[0].column_span, 4);
    assert_eq!(segments[0].start_date, date!(2026 - 05 - 04));
}

#[test]
fn month_event_segments_split_at_week_boundaries() {
    let events = vec![event(
        "trip",
        PrimitiveDateTime::new(date!(2026 - 05 - 08), time!(09:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 12), time!(17:00)),
    )];
    let first_week = (4..=10)
        .map(|day| date!(2026 - 05 - 01).replace_day(day).unwrap())
        .collect::<Vec<_>>();
    let second_week = (11..=17)
        .map(|day| date!(2026 - 05 - 01).replace_day(day).unwrap())
        .collect::<Vec<_>>();

    let first_segments = month_event_segments_for_week(&events, &first_week);
    let second_segments = month_event_segments_for_week(&events, &second_week);

    assert_eq!(first_segments.len(), 1);
    assert_eq!(first_segments[0].start_column, 4);
    assert_eq!(first_segments[0].column_span, 3);
    assert_eq!(first_segments[0].start_date, date!(2026 - 05 - 08));
    assert_eq!(second_segments.len(), 1);
    assert_eq!(second_segments[0].start_column, 0);
    assert_eq!(second_segments[0].column_span, 2);
    assert_eq!(second_segments[0].start_date, date!(2026 - 05 - 11));
}

#[test]
fn month_event_segments_treat_midnight_end_as_exclusive() {
    let events = vec![event(
        "overnight",
        PrimitiveDateTime::new(date!(2026 - 05 - 04), time!(09:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 06), time!(00:00)),
    )];
    let week = (4..=10)
        .map(|day| date!(2026 - 05 - 01).replace_day(day).unwrap())
        .collect::<Vec<_>>();

    let segments = month_event_segments_for_week(&events, &week);

    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].start_column, 0);
    assert_eq!(segments[0].column_span, 2);
}

#[test]
fn static_mode_gates_drag_and_resize() {
    let capabilities = ScheduleCapabilities::new(ScheduleMode::Static, true, true, true);

    assert!(!capabilities.events_drag_and_drop);
    assert!(capabilities.drag_slot_select);
    assert!(!capabilities.event_resize);
}

#[test]
fn slot_selection_range_tracks_forward_and_reverse_drags() {
    let nine = PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:00));
    let eleven = PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(11:00));

    let forward = slot_selection_range(
        ScheduleSlotSelectionState {
            anchor: nine,
            current: eleven,
        },
        30,
    );
    let reverse = slot_selection_range(
        ScheduleSlotSelectionState {
            anchor: eleven,
            current: nine,
        },
        30,
    );

    assert_eq!(forward.start, nine);
    assert_eq!(
        forward.end,
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(11:30))
    );
    assert_eq!(reverse, forward);
    assert!(selection_contains(
        ScheduleSlotSelectionState {
            anchor: eleven,
            current: nine,
        },
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:00))
    ));
}

#[test]
fn all_day_create_payload_uses_full_day_range() {
    let payload = super::components::all_day_event_create(
        date!(2026 - 05 - 01),
        super::types::ScheduleView::Month,
        ScheduleEventCreateSource::DayClick,
    );

    assert_eq!(
        payload.start,
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(00:00))
    );
    assert_eq!(
        payload.end,
        PrimitiveDateTime::new(date!(2026 - 05 - 02), time!(00:00))
    );
    assert_eq!(payload.date, date!(2026 - 05 - 01));
    assert!(payload.all_day);
    assert_eq!(payload.source, ScheduleEventCreateSource::DayClick);
}

#[test]
fn resize_computation_uses_edge_and_target_slot() {
    let original = event(
        "resize",
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(11:00)),
    );

    let start_resize = resized_event_times(
        &original,
        ScheduleResizeEdge::Start,
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(08:30)),
        30,
    );
    let end_resize = resized_event_times(
        &original,
        ScheduleResizeEdge::End,
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(12:00)),
        30,
    );

    assert_eq!(
        start_resize.new_start,
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(08:30))
    );
    assert_eq!(start_resize.new_end, original.end);
    assert_eq!(end_resize.new_start, original.start);
    assert_eq!(
        end_resize.new_end,
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(12:30))
    );
}

#[test]
fn resize_computation_preserves_minimum_duration() {
    let original = event(
        "resize",
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(11:00)),
    );

    let start_resize = resized_event_times(
        &original,
        ScheduleResizeEdge::Start,
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(11:00)),
        30,
    );
    let end_resize = resized_event_times(
        &original,
        ScheduleResizeEdge::End,
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(08:00)),
        30,
    );

    assert_eq!(
        start_resize.new_start,
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:30))
    );
    assert_eq!(
        end_resize.new_end,
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:30))
    );
}

#[test]
fn external_drop_data_prefers_useful_formats() {
    let data = external_drop_data_from_formats(&[
        ("application/json", None),
        ("text/plain", Some("plain".to_string())),
        ("text/html", Some("<b>html</b>".to_string())),
    ])
    .unwrap();

    assert_eq!(data.format, "text/plain");
    assert_eq!(data.data, "plain");

    assert_eq!(
        external_drop_data_from_formats(&[("text/plain", Some(String::new()))]),
        None
    );
}

#[test]
fn build_event_drop_preserves_duration_and_destination() {
    let original = event(
        "dragged",
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:30)),
    );
    let new_start = PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(13:00));

    let drop = super::components::build_event_drop(
        original.clone(),
        new_start,
        date!(2026 - 05 - 03),
        ScheduleDropDestination::Timed,
        super::types::ScheduleView::Week,
        None,
    );

    assert_eq!(drop.event_id, original.id);
    assert_eq!(drop.event, original);
    assert_eq!(drop.destination, ScheduleDropDestination::Timed);
    assert_eq!(drop.new_start, new_start);
    assert_eq!(
        drop.new_end,
        PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(14:30))
    );
    assert_eq!(drop.date, date!(2026 - 05 - 03));
    assert_eq!(drop.view, super::types::ScheduleView::Week);
}

#[test]
fn build_event_drop_uses_slot_duration_for_all_day_to_timed_move() {
    let mut original = event(
        "all-day",
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(00:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 02), time!(00:00)),
    );
    original.all_day = true;
    let new_start = PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(07:00));

    let drop = super::components::build_event_drop(
        original.clone(),
        new_start,
        date!(2026 - 05 - 03),
        ScheduleDropDestination::Timed,
        super::types::ScheduleView::Week,
        Some(60),
    );

    assert_eq!(drop.new_start, new_start);
    assert_eq!(
        drop.new_end,
        PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(08:00))
    );
}

#[test]
fn build_event_drop_without_slot_duration_preserves_all_day_duration() {
    let mut original = event(
        "all-day",
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(00:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 02), time!(00:00)),
    );
    original.all_day = true;
    let new_start = PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(00:00));

    let drop = super::components::build_event_drop(
        original.clone(),
        new_start,
        date!(2026 - 05 - 03),
        ScheduleDropDestination::Timed,
        super::types::ScheduleView::Month,
        None,
    );

    assert_eq!(drop.new_start, new_start);
    assert_eq!(
        drop.new_end,
        PrimitiveDateTime::new(date!(2026 - 05 - 04), time!(00:00))
    );
}

#[test]
fn time_slot_drop_active_matches_only_exact_timed_drop_target() {
    let dragged = event(
        "dragged",
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(12:00)),
    );
    let drop_target = Some(format!(
        "time-{}",
        PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(11:00))
    ));

    assert!(super::components::time_slot_drop_active(
        drop_target.clone(),
        PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(11:00)),
        60,
        Some(&dragged),
    ));
    assert!(!super::components::time_slot_drop_active(
        drop_target.clone(),
        PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(12:00)),
        60,
        Some(&dragged),
    ));
    assert!(!super::components::time_slot_drop_active(
        drop_target,
        PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(13:00)),
        60,
        Some(&dragged),
    ));
}

#[test]
fn timed_drop_preview_style_spans_dragged_event_duration_once() {
    let dragged = event(
        "dragged",
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(12:30)),
    );
    let days: Vec<Date> = (0..7)
        .map(|offset| date!(2026 - 05 - 03) + Duration::days(offset))
        .collect();

    let style = super::components::timed_drop_preview_style(
        Some(format!(
            "time-{}",
            PrimitiveDateTime::new(date!(2026 - 05 - 05), time!(11:00))
        )),
        &days,
        ScheduleTimeGridConfig {
            start_hour: 8,
            end_hour: 18,
            slot_minutes: 30,
            with_default_header: true,
        },
        Some(&dragged),
    )
    .expect("active timed drop preview");

    assert!(style.contains("top: calc(var(--schedule-time-slot-size) * 6.0000 + 2px);"));
    assert!(style.contains("height: calc(var(--schedule-time-slot-size) * 5.0000 - 2px);"));
    assert!(style.contains("left: calc(28.5714% + 4px);"));
    assert!(style.contains("width: calc(14.2857% - 8px);"));
}

#[test]
fn timed_drop_preview_style_spans_multi_day_dragged_events_once() {
    let dragged = event(
        "dragged",
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 03), time!(12:00)),
    );
    let days: Vec<Date> = (0..7)
        .map(|offset| date!(2026 - 05 - 03) + Duration::days(offset))
        .collect();

    let style = super::components::timed_drop_preview_style(
        Some(format!(
            "time-{}",
            PrimitiveDateTime::new(date!(2026 - 05 - 05), time!(11:00))
        )),
        &days,
        ScheduleTimeGridConfig {
            start_hour: 8,
            end_hour: 18,
            slot_minutes: 60,
            with_default_header: true,
        },
        Some(&dragged),
    )
    .expect("active timed drop preview");

    assert!(style.contains("top: calc(var(--schedule-time-slot-size) * 0.0000 + 2px);"));
    assert!(style.contains("height: calc(var(--schedule-time-slot-size) * 11.0000 - 2px);"));
    assert!(style.contains("left: calc(28.5714% + 4px);"));
    assert!(style.contains("width: calc(42.8571% - 8px);"));
    assert!(style.contains("pointer-events: auto;"));
}

#[test]
fn timed_spanning_event_geometry_spans_visible_columns_once() {
    let event = event(
        "conference",
        PrimitiveDateTime::new(date!(2026 - 05 - 04), time!(10:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 07), time!(12:00)),
    );
    let days: Vec<Date> = (0..7)
        .map(|offset| date!(2026 - 05 - 03) + Duration::days(offset))
        .collect();

    let geometry = super::components::timed_spanning_event_geometry(
        &event,
        &days,
        ScheduleTimeGridConfig {
            start_hour: 8,
            end_hour: 18,
            slot_minutes: 60,
            with_default_header: true,
        },
    )
    .expect("visible timed spanning event");

    assert_eq!(geometry.start_column, 1);
    assert_eq!(geometry.column_span, 4);
    assert_eq!(geometry.day_count, 7);
    assert_eq!(geometry.start_date, date!(2026 - 05 - 04));
    assert_eq!(geometry.top_slots, 0.0);
    assert_eq!(geometry.height_slots, 11.0);
}

#[test]
fn timed_spanning_event_geometry_clips_to_visible_day_range() {
    let event = event(
        "conference",
        PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 06), time!(12:00)),
    );
    let days: Vec<Date> = (0..3)
        .map(|offset| date!(2026 - 05 - 04) + Duration::days(offset))
        .collect();

    let geometry = super::components::timed_spanning_event_geometry(
        &event,
        &days,
        ScheduleTimeGridConfig {
            start_hour: 8,
            end_hour: 18,
            slot_minutes: 60,
            with_default_header: true,
        },
    )
    .expect("visible clipped timed spanning event");

    assert_eq!(geometry.start_column, 0);
    assert_eq!(geometry.column_span, 3);
    assert_eq!(geometry.day_count, 3);
    assert_eq!(geometry.start_date, date!(2026 - 05 - 04));
    assert_eq!(geometry.top_slots, 0.0);
    assert_eq!(geometry.height_slots, 11.0);
}

#[test]
fn timed_spanning_event_geometry_ignores_single_day_events() {
    let event = event(
        "standup",
        PrimitiveDateTime::new(date!(2026 - 05 - 05), time!(09:00)),
        PrimitiveDateTime::new(date!(2026 - 05 - 05), time!(10:00)),
    );
    let days: Vec<Date> = (0..7)
        .map(|offset| date!(2026 - 05 - 03) + Duration::days(offset))
        .collect();

    assert!(super::components::timed_spanning_event_geometry(
        &event,
        &days,
        ScheduleTimeGridConfig::default(),
    )
    .is_none());
}

#[test]
fn week_view_renders_timed_multi_day_event_as_single_spanning_node() {
    let mut dom = VirtualDom::new(WeekTimedMultiDayHarness);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert_eq!(
        html.matches("data-schedule-event=\"conference\"").count(),
        1
    );
    assert_eq!(html.matches("data-schedule-event=\"standup\"").count(), 1);
    assert!(html.contains("data-schedule-timed-spanning-events"));
    assert!(html.contains("data-render-date=\"2026-05-04\""));
    assert!(html.contains("left: calc(14.2857% + 4px);"));
    assert!(html.contains("width: calc(57.1429% - 8px);"));
    assert!(html.contains("data-schedule-timed-events"));
}

#[test]
fn clear_dragging_event_resets_stale_drag_state() {
    let mut dom = VirtualDom::new(ClearDraggingEventHarness);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains(">false<"));
}

#[test]
fn responsive_mobile_month_renders_multi_day_events_on_each_intersecting_day() {
    let mut dom = VirtualDom::new(ResponsiveMobileMonthMultiDayHarness);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert_eq!(
        html.matches("data-schedule-event=\"conference\"").count(),
        5
    );
    assert_eq!(html.matches("data-render-date=\"2026-05-04\"").count(), 2);
    assert_eq!(html.matches("data-render-date=\"2026-05-05\"").count(), 1);
    assert_eq!(html.matches("data-render-date=\"2026-05-06\"").count(), 1);
    assert_eq!(html.matches("data-render-date=\"2026-05-07\"").count(), 1);
}

#[test]
fn current_day_helper_compares_dates() {
    assert!(is_current_day(today()));
    assert!(!is_current_day(today() - Duration::days(1)));
}

#[test]
fn current_time_line_offset_is_within_visible_hours() {
    let now = PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:30));
    let offset = current_time_line_offset(
        now,
        super::types::ScheduleTimeGridConfig {
            with_default_header: true,
            start_hour: 8,
            end_hour: 10,
            slot_minutes: 30,
        },
    );

    assert_eq!(offset, Some(50.0));
}

#[test]
fn current_time_line_offset_excludes_times_outside_visible_hours() {
    let config = super::types::ScheduleTimeGridConfig {
        with_default_header: true,
        start_hour: 8,
        end_hour: 10,
        slot_minutes: 30,
    };

    assert_eq!(
        current_time_line_offset(
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(07:59)),
            config
        ),
        None
    );
    assert_eq!(
        current_time_line_offset(
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(11:00)),
            config
        ),
        None
    );
}

#[test]
fn year_month_transition_clamps_day_and_keeps_year() {
    let next = year_month_transition(date!(2024 - 01 - 31), 2);

    assert_eq!(next, date!(2024 - 02 - 29));
}

#[test]
fn add_months_clamps_to_last_day_of_target_month() {
    assert_eq!(add_months(date!(2024 - 01 - 31), 1), date!(2024 - 02 - 29));
    assert_eq!(add_months(date!(2025 - 01 - 31), 1), date!(2025 - 02 - 28));
}

#[test]
fn shift_date_uses_schedule_view_granularity() {
    let anchor = date!(2026 - 05 - 15);

    assert_eq!(shift_date(anchor, super::types::ScheduleView::Day, 2), date!(2026 - 05 - 17));
    assert_eq!(shift_date(anchor, super::types::ScheduleView::Week, -1), date!(2026 - 05 - 08));
    assert_eq!(shift_date(anchor, super::types::ScheduleView::Month, 1), date!(2026 - 06 - 15));
    assert_eq!(shift_date(anchor, super::types::ScheduleView::Year, 1), date!(2027 - 05 - 15));
}

#[test]
fn overlap_layout_assigns_distinct_columns() {
    let laid_out = layout_overlapping_events(vec![
        event(
            "a",
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:00)),
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:00)),
        ),
        event(
            "b",
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:30)),
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:30)),
        ),
    ]);

    assert_eq!(laid_out.len(), 2);
    assert_ne!(laid_out[0].column, laid_out[1].column);
    assert_eq!(laid_out[0].columns, 2);
}

#[test]
fn overlap_layout_uses_local_columns_per_collision_group() {
    let laid_out = layout_overlapping_events(vec![
        event(
            "a",
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:00)),
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:00)),
        ),
        event(
            "b",
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:30)),
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:30)),
        ),
        event(
            "c",
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(12:00)),
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(13:00)),
        ),
    ]);

    assert_eq!(laid_out.len(), 3);
    assert_eq!(laid_out[0].columns, 2);
    assert_eq!(laid_out[1].columns, 2);
    assert_eq!(laid_out[2].column, 0);
    assert_eq!(laid_out[2].columns, 1);
}

#[test]
fn timed_event_geometry_tracks_duration_and_clips_to_visible_range() {
    let geometry = timed_event_geometry(
        &event(
            "visible",
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(09:00)),
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(10:30)),
        ),
        date!(2026 - 05 - 01),
        super::types::ScheduleTimeGridConfig {
            with_default_header: true,
            start_hour: 8,
            end_hour: 10,
            slot_minutes: 30,
        },
    )
    .unwrap();

    assert!((geometry.top_slots - 2.0).abs() < 0.001);
    assert!((geometry.height_slots - 3.0).abs() < 0.001);

    let clipped = timed_event_geometry(
        &event(
            "clipped",
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(07:00)),
            PrimitiveDateTime::new(date!(2026 - 05 - 01), time!(08:30)),
        ),
        date!(2026 - 05 - 01),
        super::types::ScheduleTimeGridConfig {
            with_default_header: true,
            start_hour: 8,
            end_hour: 10,
            slot_minutes: 30,
        },
    )
    .unwrap();

    assert_eq!(clipped.top_slots, 0.0);
    assert!((clipped.height_slots - 1.0).abs() < 0.001);
}

#[test]
fn month_weekday_labels_follow_first_day_and_locale() {
    let monday_first = month_weekday_labels(time::Weekday::Monday, "fr-FR");
    let sunday_first = month_weekday_labels(time::Weekday::Sunday, "en-US");

    assert_eq!(
        monday_first,
        vec!["lun.", "mar.", "mer.", "jeu.", "ven.", "sam.", "dim."]
    );
    assert_eq!(
        sunday_first,
        vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
    );
}

#[test]
fn month_weekday_labels_fallback_for_unknown_locale() {
    assert_eq!(
        month_weekday_labels(time::Weekday::Monday, "pt-BR"),
        vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
    );
}

#[test]
fn month_day_labels_use_day_numbers_only() {
    assert_eq!(format_day_of_month_label(date!(2026 - 05 - 18)), "18");
}
