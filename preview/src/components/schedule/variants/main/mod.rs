use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut status = use_signal(|| "Interact with the schedule".to_string());
    let mut events = use_signal(sample_events);

    rsx! {
        div { style: "display: grid; gap: 12px; padding: 20px;",
            div { "data-schedule-main-status": true, style: "font-size: 0.875rem;", "{status}" }
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Week,
                events: events(),
                with_events_drag_and_drop: true,
                with_drag_slot_select: true,
                with_event_resize: true,
                on_event_click: move |payload: ScheduleEventClick| {
                    status.set(format!("Clicked event {}", payload.event.title));
                },
                on_time_slot_click: move |payload: ScheduleTimeSlotClick| {
                    status.set(format!("Clicked time slot {}", payload.start));
                },
                on_all_day_slot_click: move |payload: ScheduleAllDaySlotClick| {
                    status.set(format!("Clicked all-day slot {}", payload.date));
                },
                on_day_click: move |payload: ScheduleDayClick| {
                    status.set(format!("Clicked day {}", payload.date));
                },
                on_event_drop: move |payload: ScheduleEventDrop| {
                    events.with_mut(|events| {
                        apply_demo_event_drop(
                            events,
                            &payload,
                            ScheduleRecurrenceExpansionLimit::default(),
                        );
                    });
                    status.set(format!("Dropped {} on {}", payload.event.title, payload.new_start));
                },
                on_event_resize: move |payload: ScheduleEventResize| {
                    events.with_mut(|events| {
                        apply_demo_event_resize(
                            events,
                            &payload,
                            ScheduleRecurrenceExpansionLimit::default(),
                        );
                    });
                    status.set(format!("Resized {} to {}", payload.event.title, payload.new_end));
                },
            }
        }
    }
}
