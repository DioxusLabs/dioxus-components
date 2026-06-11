use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut last_drop = use_signal(|| "Drag an event to a time slot".to_string());
    let mut events = use_signal(sample_events);

    rsx! {
        div { style: "display: grid; gap: 12px; padding: 20px;",
            div { style: "font-size: 0.875rem;", "{last_drop}" }
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Week,
                events: events(),
                with_events_drag_and_drop: true,
                on_event_drop: move |payload: ScheduleEventDrop| {
                    events.with_mut(|events| {
                        apply_demo_event_drop(
                            events,
                            &payload,
                            ScheduleRecurrenceExpansionLimit::default(),
                        );
                    });
                    last_drop.set(format!("Dropped {} on {}", payload.event.title, payload.new_start));
                },
                on_external_event_drop: move |payload: ScheduleExternalDrop| {
                    last_drop.set(format!("External item dropped at {}", payload.start));
                },
            }
        }
    }
}
