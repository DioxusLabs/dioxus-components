use super::super::component::*;
use dioxus::prelude::*;
use time::Weekday;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "padding: 20px;",
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Month,
                locale: "fr-FR",
                labels: french_labels(),
                week_view: ScheduleWeekViewConfig {
                    first_day_of_week: Weekday::Monday,
                    time_grid: workday_time_grid(),
                },
                month_view: ScheduleMonthViewConfig {
                    first_day_of_week: Weekday::Monday,
                    ..ScheduleMonthViewConfig::default()
                },
            }
        }
    }
}
