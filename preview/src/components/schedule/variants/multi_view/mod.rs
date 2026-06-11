use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let events = sample_events();

    rsx! {
        div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 16px; padding: 20px;",
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Day,
                events: events.clone(),
            }
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Month,
                events: events.clone(),
            }
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Year,
                events,
            }
        }
    }
}
