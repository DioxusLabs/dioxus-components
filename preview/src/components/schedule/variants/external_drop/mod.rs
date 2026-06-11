use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut message = use_signal(|| "Drag the external task into a time slot".to_string());

    rsx! {
        div { style: "display: grid; gap: 12px; padding: 20px;",
            div {
                draggable: true,
                "data-schedule-external-source": true,
                style: "width: max-content; border: 1px dashed currentColor; border-radius: 8px; padding: 8px 10px; cursor: grab;",
                ondragstart: move |event: Event<DragData>| {
                    event.data_transfer().set_effect_allowed("copy");
                    event.data_transfer().set_drop_effect("copy");
                    let _ = event.data_transfer().set_data("text/plain", "External planning task");
                },
                "External planning task"
            }
            div { "data-schedule-external-drop-status": true, style: "font-size: 0.875rem;", "{message}" }
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Week,
                with_events_drag_and_drop: true,
                on_external_event_drop: move |payload: ScheduleExternalDrop| {
                    let data = payload.data.unwrap_or_else(|| "external item".to_string());
                    message.set(format!("Dropped {data} at {}", payload.start));
                },
            }
        }
    }
}
