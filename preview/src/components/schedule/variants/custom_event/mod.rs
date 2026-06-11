use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "padding: 20px;",
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Week,
                render_event_body: Callback::new(|context: ScheduleEventRenderContext| {
                    let event = context.event;
                    rsx! {
                        div { "data-schedule-custom-event": event.id.clone(),
                            strong { "{event.title}" }
                            span { " · custom body" }
                        }
                    }
                }),
            }
        }
    }
}
