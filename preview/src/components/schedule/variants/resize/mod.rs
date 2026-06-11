use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut last_resize = use_signal(|| "Use an event resize handle".to_string());
    let mut events = use_signal(sample_events);

    rsx! {
        div { style: "display: grid; gap: 12px; padding: 20px;",
            div { style: "font-size: 0.875rem;", "{last_resize}" }
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Week,
                events: events(),
                with_event_resize: true,
                on_event_resize: move |payload: ScheduleEventResize| {
                    events.with_mut(|events| {
                        apply_demo_event_resize(
                            events,
                            &payload,
                            ScheduleRecurrenceExpansionLimit::default(),
                        );
                    });
                    last_resize.set(format!("Resized {} to {}", payload.event.title, payload.new_end));
                },
            }
        }
    }
}
