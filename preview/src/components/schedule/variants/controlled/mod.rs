use super::super::component::*;
use dioxus::prelude::*;
use time::Duration;

#[component]
pub fn Demo() -> Element {
    let mut date = use_signal(sample_date);
    let mut view = use_signal(|| ScheduleView::Week);
    let mut status = use_signal(|| "Controlled schedule".to_string());

    rsx! {
        div { style: "display: grid; gap: 12px; padding: 20px;",
            div { "data-schedule-controlled-status": true, style: "font-size: 0.875rem;", "{status}" }
            div { style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center;",
                button { onclick: move |_| date.set(date() - Duration::days(1)), "Previous day" }
                button { onclick: move |_| date.set(sample_date()), "Reset date" }
                button { onclick: move |_| date.set(date() + Duration::days(1)), "Next day" }
                button { onclick: move |_| view.set(ScheduleView::Day), "Day" }
                button { onclick: move |_| view.set(ScheduleView::Week), "Week" }
                button { onclick: move |_| view.set(ScheduleView::Month), "Month" }
                button { onclick: move |_| view.set(ScheduleView::Year), "Year" }
            }
            Schedule {
                date: Some(date()),
                view: Some(view()),
                on_date_change: move |payload: ScheduleDateChange| {
                    date.set(payload.next);
                    status.set(format!("Date changed to {}", payload.next));
                },
                on_view_change: move |payload: ScheduleViewChange| {
                    view.set(payload.next);
                    status.set(format!("View changed to {:?}", payload.next));
                },
            }
        }
    }
}
