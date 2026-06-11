# Schedule

The schedule component renders day, week, month, and year calendar views with timed events, all-day events, recurrence expansion, responsive mobile layout, and scheduling interactions.

## Usage

```rust
Schedule {
    default_date: sample_date(),
    default_view: ScheduleView::Week,
    events: sample_events(),
}
```

## Views

Use `default_view` for uncontrolled view state or `view` with `on_view_change` for controlled state. Supported views are `ScheduleView::Day`, `ScheduleView::Week`, `ScheduleView::Month`, and `ScheduleView::Year`. The header exposes navigation and view controls, and selecting a month in the year view moves to month view.

## Controlled State

Use `date` and `on_date_change` to control the visible date. Use `view` and `on_view_change` to control the visible view. The callback payloads include previous and next values plus the active view or date so application state can stay synchronized.

## Events And Recurrence

Events use stable ids, titles, start and end date-times, all-day state, optional colors, descriptions, and optional recurrence rules. `recurrence_expansion_limit` bounds repeated events. The preview data includes timed, all-day, overlapping, colored, daily recurring, and weekly recurring events.

## Interactions

Enable `with_events_drag_and_drop`, `with_drag_slot_select`, and `with_event_resize` to expose drag/drop, slot range selection, and resize behavior. Handlers receive typed payloads for time slots, all-day slots, day cells, event clicks, drag start/end, internal drops, external drops, slot selections, and resize completions. `can_drag_event` and `can_resize_event` can prevent specific events from moving or resizing.

Use `on_event_create` for built-in event creation. Timed slot clicks emit a `ScheduleEventCreate` with the slot `start` and `end`. Timed drag selection emits one create payload for the normalized selected range. All-day slot clicks and day-cell clicks emit full-day ranges. The payload includes `date`, `all_day`, `view`, and `source` (`TimeSlotClick`, `TimeSlotDrag`, `AllDaySlotClick`, or `DayClick`) so applications can append an event without stitching together the legacy click and selection callbacks.

## Responsive Layout

Set `layout: ScheduleLayout::Responsive` to render both desktop and mobile containers. CSS switches to the mobile month presentation at small widths while keeping the year view available.

## Custom Rendering And Header

Use `render_event_body` to replace the default event body. Use `with_default_header: false` to suppress the top-level schedule header, pass `header` to replace it with custom content, or compose with the exported primitive `ScheduleHeader` and `ScheduleViewButton` controls. Per-view config structs expose `with_default_header` toggles for the day, week, month, year, and mobile month view headers.

## Styling

The primitive exposes a `radius` style variable, stateful `data-*` attributes, and `ScheduleClassNames` hooks for desktop/mobile containers, day/week/month/year/mobile-month surfaces, slots, days, and events.

## Localization And Labels

Set `locale` and pass `ScheduleLabels` to localize visible navigation, view names, all-day labels, and empty slot text.

## Static Mode

Set `mode: ScheduleMode::Static` to keep navigation and selection available while disabling event drag/drop and event resize, even when those interactive props are enabled.

## Accessibility

The primitive renders buttons for navigation, view controls, dates, months, and slots. Provide meaningful event titles and descriptions, keep custom event bodies readable, and retain visible focus states when extending the styles.

## Component Structure

```rust
Schedule {
    default_date: sample_date(),
    default_view: ScheduleView::Week,
    layout: ScheduleLayout::Responsive,
    mode: ScheduleMode::Default,
    events: sample_events(),
    labels: ScheduleLabels::default(),
    with_events_drag_and_drop: true,
    with_drag_slot_select: true,
    with_event_resize: true,
}
```
