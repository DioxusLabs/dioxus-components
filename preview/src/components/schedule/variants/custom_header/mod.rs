use super::super::component::*;
use dioxus::prelude::*;
use time::Duration;

#[component]
pub fn Demo() -> Element {
    let mut date = use_signal(sample_date);
    let mut view = use_signal(|| ScheduleView::Week);

    rsx! {
        div { style: "display: grid; gap: 12px; padding: 20px;",
            div { style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center;",
                button { onclick: move |_| date.set(date() - Duration::weeks(1)), "Previous week" }
                button { onclick: move |_| date.set(sample_date()), "Today" }
                button { onclick: move |_| date.set(date() + Duration::weeks(1)), "Next week" }
                button { onclick: move |_| view.set(ScheduleView::Day), "Day" }
                button { onclick: move |_| view.set(ScheduleView::Week), "Week" }
                button { onclick: move |_| view.set(ScheduleView::Month), "Month" }
            }
            Schedule {
                date: Some(date()),
                view: Some(view()),
                with_default_header: false,
                header: rsx! {
                    div {
                        "data-schedule-custom-header": true,
                        style: "display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 8px; padding: 12px; border-bottom: 1px solid var(--schedule-border); background: var(--schedule-surface-muted);",
                        strong { "Custom planning header" }
                        span { "{date} · {view:?}" }
                    }
                },
                on_date_change: move |payload: ScheduleDateChange| date.set(payload.next),
                on_view_change: move |payload: ScheduleViewChange| view.set(payload.next),
                day_view: ScheduleDayViewConfig {
                    time_grid: ScheduleTimeGridConfig {
                        with_default_header: false,
                        ..workday_time_grid()
                    },
                },
                week_view: ScheduleWeekViewConfig {
                    time_grid: ScheduleTimeGridConfig {
                        with_default_header: false,
                        ..workday_time_grid()
                    },
                    ..ScheduleWeekViewConfig::default()
                },
                month_view: ScheduleMonthViewConfig {
                    with_default_header: false,
                    ..ScheduleMonthViewConfig::default()
                },
            }
        }
    }
}
