use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "padding: 20px;",
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Week,
                mode: ScheduleMode::Static,
                with_events_drag_and_drop: true,
                with_event_resize: true,
                with_drag_slot_select: true,
            }
        }
    }
}
