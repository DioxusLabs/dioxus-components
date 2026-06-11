use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| "Click or drag an empty slot".to_string());
    let mut events = use_signal(sample_events);

    rsx! {
        div { style: "display: grid; gap: 12px; padding: 20px;",
            div { style: "font-size: 0.875rem;", "{selected}" }
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Week,
                events: events(),
                with_drag_slot_select: true,
                on_event_create: move |payload: ScheduleEventCreate| {
                    events.with_mut(|events| {
                        let index = events.len() + 1;
                        events.push(ScheduleEvent {
                            id: format!("created-{index}"),
                            title: match payload.source {
                                ScheduleEventCreateSource::TimeSlotClick => "Created from click".to_string(),
                                ScheduleEventCreateSource::TimeSlotDrag => "Created from drag".to_string(),
                                ScheduleEventCreateSource::AllDaySlotClick => "Created all-day".to_string(),
                                ScheduleEventCreateSource::DayClick => "Created from day".to_string(),
                            },
                            start: payload.start,
                            end: payload.end,
                            all_day: payload.all_day,
                            color: Some("blue".to_string()),
                            description: None,
                            recurrence: None,
                            drag_disabled: false,
                            resize_disabled: false,
                        });
                    });
                    selected.set(format!("Created {} to {}", payload.start, payload.end));
                },
            }
        }
    }
}
