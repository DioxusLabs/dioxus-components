use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "padding: 20px;",
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Week,
                week_view: ScheduleWeekViewConfig {
                    time_grid: ScheduleTimeGridConfig {
                        start_hour: 7,
                        end_hour: 18,
                        slot_minutes: 30,
                        with_default_header: true,
                    },
                    ..ScheduleWeekViewConfig::default()
                },
            }
        }
    }
}
