use std::cmp::Ordering;

use dioxus::prelude::*;
use time::{Date, Duration, Month, PrimitiveDateTime, Time, Weekday};

use crate::LocalDateExt as _;

use super::state::{
    LaidOutEvent, ResizedEventTimes, ScheduleExternalData, ScheduleSlotRange,
    ScheduleSlotSelectionState,
};
use super::types::{
    ScheduleEvent, ScheduleRecurrenceExpansionLimit, ScheduleRecurrenceFrequency,
    ScheduleResizeEdge, ScheduleTimeGridConfig, ScheduleView,
};

/// Returns the current local date, falling back to UTC when local time is unavailable.
pub fn today() -> Date {
    time::OffsetDateTime::now_local_date()
}

pub(crate) fn now() -> PrimitiveDateTime {
    let now = time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc());
    PrimitiveDateTime::new(now.date(), now.time())
}

/// Shifts a schedule anchor date by the number of ranges represented by the active view.
pub fn shift_date(date: Date, view: ScheduleView, amount: i64) -> Date {
    match view {
        ScheduleView::Day => date + Duration::days(amount),
        ScheduleView::Week => date + Duration::weeks(amount),
        ScheduleView::Month => add_months(date, amount as i32),
        ScheduleView::Year => add_months(date, (amount * 12) as i32),
    }
}

pub(crate) fn week_dates(date: Date, first_day: Weekday) -> Vec<Date> {
    let offset = (7 + date.weekday().number_days_from_monday() as i64
        - first_day.number_days_from_monday() as i64)
        % 7;
    let start = date - Duration::days(offset);
    (0..7).map(|day| start + Duration::days(day)).collect()
}

pub(crate) fn month_grid_dates(date: Date, first_day: Weekday) -> Vec<Date> {
    let first = Date::from_calendar_date(date.year(), date.month(), 1).unwrap();
    let offset = (7 + first.weekday().number_days_from_monday() as i64
        - first_day.number_days_from_monday() as i64)
        % 7;
    let start = first - Duration::days(offset);
    (0..42).map(|day| start + Duration::days(day)).collect()
}

pub(crate) fn time_slots(config: ScheduleTimeGridConfig) -> Vec<Time> {
    let start = config.start_hour.min(23);
    let end = config.end_hour.min(23).max(start);
    let step = config.slot_minutes.max(1) as usize;
    (start..=end)
        .flat_map(|hour| (0..60).step_by(step).map(move |minute| (hour, minute)))
        .filter_map(|(hour, minute)| Time::from_hms(hour, minute, 0).ok())
        .collect()
}

pub(crate) fn format_time_range(start: PrimitiveDateTime, end: PrimitiveDateTime) -> String {
    format!(
        "{} - {}",
        format_time(start.time()),
        format_time(end.time())
    )
}

pub(crate) fn format_time(time: Time) -> String {
    let hour = time.hour();
    let minute = time.minute();
    let suffix = if hour < 12 { "AM" } else { "PM" };
    let display_hour = match hour % 12 {
        0 => 12,
        hour => hour,
    };
    if minute == 0 {
        format!("{display_hour} {suffix}")
    } else {
        format!("{display_hour}:{minute:02} {suffix}")
    }
}

pub(crate) fn format_date_label(date: Date) -> String {
    let weekday = match date.weekday() {
        Weekday::Monday => "Mon",
        Weekday::Tuesday => "Tue",
        Weekday::Wednesday => "Wed",
        Weekday::Thursday => "Thu",
        Weekday::Friday => "Fri",
        Weekday::Saturday => "Sat",
        Weekday::Sunday => "Sun",
    };
    format!("{weekday} {}", date.day())
}

pub(crate) fn format_day_of_month_label(date: Date) -> String {
    date.day().to_string()
}

pub(crate) fn month_weekday_labels(first_day: Weekday, locale: &str) -> Vec<&'static str> {
    let weekday_order = [
        Weekday::Sunday,
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
    ];
    let start = first_day.number_days_from_sunday() as usize;
    let language = locale
        .split(['-', '_'])
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();

    (0..7)
        .map(|offset| weekday_label(weekday_order[(start + offset) % 7], &language))
        .collect()
}

fn weekday_label(weekday: Weekday, language: &str) -> &'static str {
    match language {
        "fr" => match weekday {
            Weekday::Sunday => "dim.",
            Weekday::Monday => "lun.",
            Weekday::Tuesday => "mar.",
            Weekday::Wednesday => "mer.",
            Weekday::Thursday => "jeu.",
            Weekday::Friday => "ven.",
            Weekday::Saturday => "sam.",
        },
        "es" => match weekday {
            Weekday::Sunday => "dom",
            Weekday::Monday => "lun",
            Weekday::Tuesday => "mar",
            Weekday::Wednesday => "mie",
            Weekday::Thursday => "jue",
            Weekday::Friday => "vie",
            Weekday::Saturday => "sab",
        },
        "de" => match weekday {
            Weekday::Sunday => "So",
            Weekday::Monday => "Mo",
            Weekday::Tuesday => "Di",
            Weekday::Wednesday => "Mi",
            Weekday::Thursday => "Do",
            Weekday::Friday => "Fr",
            Weekday::Saturday => "Sa",
        },
        _ => match weekday {
            Weekday::Sunday => "Sun",
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
        },
    }
}

pub(crate) fn is_current_day(date: Date) -> bool {
    date == today()
}

pub(crate) fn current_time_line_offset(
    now: PrimitiveDateTime,
    config: ScheduleTimeGridConfig,
) -> Option<f32> {
    let start_hour = config.start_hour.min(23);
    let end_hour = config.end_hour.min(23).max(start_hour);
    let visible_start_minutes = start_hour as f32 * 60.0;
    let visible_end_minutes = (end_hour as f32 + 1.0) * 60.0;
    let now_time = now.time();
    let now_minutes =
        now_time.hour() as f32 * 60.0 + now_time.minute() as f32 + now_time.second() as f32 / 60.0;
    if now_minutes < visible_start_minutes || now_minutes >= visible_end_minutes {
        return None;
    }
    Some(
        ((now_minutes - visible_start_minutes) / (visible_end_minutes - visible_start_minutes))
            * 100.0,
    )
}

pub(crate) fn slot_selection_range(
    selection: ScheduleSlotSelectionState,
    slot_minutes: u8,
) -> ScheduleSlotRange {
    let slot_duration = Duration::minutes(slot_minutes.max(1) as i64);
    let (start, end_slot) = if selection.anchor <= selection.current {
        (selection.anchor, selection.current)
    } else {
        (selection.current, selection.anchor)
    };
    ScheduleSlotRange {
        start,
        end: end_slot + slot_duration,
    }
}

pub(crate) fn selection_contains(
    selection: ScheduleSlotSelectionState,
    slot: PrimitiveDateTime,
) -> bool {
    let (start, end) = if selection.anchor <= selection.current {
        (selection.anchor, selection.current)
    } else {
        (selection.current, selection.anchor)
    };
    slot >= start && slot <= end
}

pub(crate) fn resized_event_times(
    event: &ScheduleEvent,
    edge: ScheduleResizeEdge,
    target_slot: PrimitiveDateTime,
    slot_minutes: u8,
) -> ResizedEventTimes {
    let slot_duration = Duration::minutes(slot_minutes.max(1) as i64);
    match edge {
        ScheduleResizeEdge::Start => {
            let latest_start = event.end - slot_duration;
            ResizedEventTimes {
                new_start: target_slot.min(latest_start),
                new_end: event.end,
            }
        }
        ScheduleResizeEdge::End => {
            let earliest_end = event.start + slot_duration;
            ResizedEventTimes {
                new_start: event.start,
                new_end: (target_slot + slot_duration).max(earliest_end),
            }
        }
    }
}

pub(crate) fn external_drop_data(event: &Event<DragData>) -> Option<ScheduleExternalData> {
    external_drop_data_from_formats(&[
        (
            "application/json",
            event.data_transfer().get_data("application/json"),
        ),
        ("text/plain", event.data_transfer().get_data("text/plain")),
        (
            "text/uri-list",
            event.data_transfer().get_data("text/uri-list"),
        ),
        ("text/html", event.data_transfer().get_data("text/html")),
    ])
}

pub(crate) fn external_drop_data_from_formats(
    formats: &[(&str, Option<String>)],
) -> Option<ScheduleExternalData> {
    formats.iter().find_map(|(format, data)| {
        data.as_ref()
            .filter(|data| !data.is_empty())
            .map(|data| ScheduleExternalData {
                format: (*format).to_string(),
                data: data.clone(),
            })
    })
}

/// Adds months to a date while clamping the day to the last valid day in the target month.
pub fn add_months(date: Date, months: i32) -> Date {
    let month_index = date.year() * 12 + date.month() as i32 - 1 + months;
    let year = month_index.div_euclid(12);
    let month = Month::try_from(month_index.rem_euclid(12) as u8 + 1).unwrap();
    Date::from_calendar_date(year, month, date.day().min(month.length(year))).unwrap()
}

pub(crate) fn year_month_transition(date: Date, month_number: u8) -> Date {
    let month = Month::try_from(month_number.clamp(1, 12)).unwrap();
    Date::from_calendar_date(
        date.year(),
        month,
        date.day().min(month.length(date.year())),
    )
    .unwrap()
}

pub(crate) fn event_intersects_range(
    event: &ScheduleEvent,
    start: PrimitiveDateTime,
    end: PrimitiveDateTime,
) -> bool {
    event.start < end && event.end > start
}

pub(crate) fn filter_events_for_date(events: &[ScheduleEvent], date: Date) -> Vec<ScheduleEvent> {
    let start = PrimitiveDateTime::new(date, Time::MIDNIGHT);
    let end = start + Duration::days(1);
    events
        .iter()
        .filter(|event| event_intersects_range(event, start, end))
        .cloned()
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MonthEventSegment {
    pub(crate) event: ScheduleEvent,
    pub(crate) start_column: usize,
    pub(crate) column_span: usize,
    pub(crate) start_date: Date,
}

pub(crate) fn month_event_segments_for_week(
    events: &[ScheduleEvent],
    week: &[Date],
) -> Vec<MonthEventSegment> {
    let Some(week_start) = week.first().copied() else {
        return Vec::new();
    };
    let Some(week_end_day) = week.last().copied() else {
        return Vec::new();
    };
    let week_start_time = PrimitiveDateTime::new(week_start, Time::MIDNIGHT);
    let week_end_time = PrimitiveDateTime::new(week_end_day + Duration::days(1), Time::MIDNIGHT);

    events
        .iter()
        .filter_map(|event| {
            if !event_intersects_range(event, week_start_time, week_end_time) {
                return None;
            }

            let clipped_start = event.start.max(week_start_time);
            let clipped_end = event.end.min(week_end_time);
            if clipped_end <= clipped_start {
                return None;
            }

            let inclusive_end = clipped_end - Duration::nanoseconds(1);
            let start_column = week
                .iter()
                .position(|day| *day == clipped_start.date())
                .unwrap_or(0);
            let end_column = week
                .iter()
                .position(|day| *day == inclusive_end.date())
                .unwrap_or_else(|| week.len().saturating_sub(1));

            Some(MonthEventSegment {
                event: event.clone(),
                start_column,
                column_span: end_column.saturating_sub(start_column) + 1,
                start_date: clipped_start.date(),
            })
        })
        .collect()
}

pub(crate) fn expand_events(
    events: &[ScheduleEvent],
    limit: ScheduleRecurrenceExpansionLimit,
) -> Vec<ScheduleEvent> {
    let mut expanded = Vec::new();
    for event in events {
        expanded.extend(expand_event(event, limit));
    }
    expanded.sort_by(|a, b| a.start.cmp(&b.start).then_with(|| a.id.cmp(&b.id)));
    expanded
}

pub(crate) fn expand_event(
    event: &ScheduleEvent,
    limit: ScheduleRecurrenceExpansionLimit,
) -> Vec<ScheduleEvent> {
    let Some(recurrence) = &event.recurrence else {
        return vec![event.clone()];
    };
    let interval = recurrence.interval.max(1);
    let max = recurrence
        .count
        .unwrap_or(limit.max_occurrences)
        .min(limit.max_occurrences);
    let duration = event.end - event.start;
    let mut occurrences = Vec::new();
    let mut start = event.start;
    for index in 0..max {
        if recurrence.until.is_some_and(|until| start > until) {
            break;
        }
        let mut occurrence = event.clone();
        occurrence.start = start;
        occurrence.end = start + duration;
        if index > 0 {
            occurrence.id = format!("{}:{}", event.id, index);
            occurrence.recurrence = None;
        }
        occurrences.push(occurrence);
        start = next_occurrence_start(start, recurrence.frequency, interval);
    }
    occurrences
}

pub(crate) fn next_occurrence_start(
    start: PrimitiveDateTime,
    frequency: ScheduleRecurrenceFrequency,
    interval: u32,
) -> PrimitiveDateTime {
    match frequency {
        ScheduleRecurrenceFrequency::Daily => start + Duration::days(interval as i64),
        ScheduleRecurrenceFrequency::Weekly => start + Duration::weeks(interval as i64),
        ScheduleRecurrenceFrequency::Monthly => {
            PrimitiveDateTime::new(add_months(start.date(), interval as i32), start.time())
        }
        ScheduleRecurrenceFrequency::Yearly => PrimitiveDateTime::new(
            add_months(start.date(), (interval * 12) as i32),
            start.time(),
        ),
    }
}

pub(crate) fn layout_overlapping_events(mut events: Vec<ScheduleEvent>) -> Vec<LaidOutEvent> {
    events.sort_by(|a, b| match a.start.cmp(&b.start) {
        Ordering::Equal => a.end.cmp(&b.end),
        other => other,
    });
    let mut active: Vec<(usize, PrimitiveDateTime)> = Vec::new();
    let mut laid_out: Vec<LaidOutEvent> = Vec::new();
    let mut cluster_start = 0usize;
    let mut cluster_max_columns = 1usize;
    for event in events {
        active.retain(|(_, end)| *end > event.start);
        if active.is_empty() && cluster_start < laid_out.len() {
            for laid_out_event in &mut laid_out[cluster_start..] {
                laid_out_event.columns = cluster_max_columns;
            }
            cluster_start = laid_out.len();
            cluster_max_columns = 1;
        }
        let mut column = 0;
        while active.iter().any(|(used, _)| *used == column) {
            column += 1;
        }
        active.push((column, event.end));
        cluster_max_columns = cluster_max_columns.max(active.len());
        laid_out.push(LaidOutEvent {
            event,
            column,
            columns: 1,
        });
    }
    for laid_out_event in &mut laid_out[cluster_start..] {
        laid_out_event.columns = cluster_max_columns;
    }
    laid_out
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TimedEventGeometry {
    pub(crate) top_slots: f32,
    pub(crate) height_slots: f32,
}

pub(crate) fn timed_event_geometry(
    event: &ScheduleEvent,
    date: Date,
    config: ScheduleTimeGridConfig,
) -> Option<TimedEventGeometry> {
    let start_hour = config.start_hour.min(23);
    let end_hour = config.end_hour.min(23).max(start_hour);
    let visible_start = PrimitiveDateTime::new(date, Time::from_hms(start_hour, 0, 0).unwrap());
    let visible_end = visible_start + Duration::hours((end_hour - start_hour + 1) as i64);
    let clipped_start = event.start.max(visible_start);
    let clipped_end = event.end.min(visible_end);
    if clipped_end <= clipped_start {
        return None;
    }
    let slot_minutes = config.slot_minutes.max(1) as f32;
    let top_minutes = (clipped_start - visible_start).whole_minutes() as f32;
    let height_minutes = (clipped_end - clipped_start).whole_minutes().max(1) as f32;
    Some(TimedEventGeometry {
        top_slots: top_minutes / slot_minutes,
        height_slots: height_minutes / slot_minutes,
    })
}
