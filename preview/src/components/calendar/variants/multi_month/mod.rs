use super::super::component::*;
use dioxus::prelude::*;
use time::{macros::date, Date};

use dioxus_primitives::calendar::DateRange;

#[component]
pub fn Demo() -> Element {
    let mut selected_range = use_signal(|| None::<DateRange>);
    let mut view_date = use_signal(|| date!(2026 - 05 - 15));
    rsx! {
        div { style: "padding: 20px;",
            RangeCalendar {
                selected_range: selected_range(),
                on_range_change: move |range| {
                    tracing::info!("Selected range: {:?}", range);
                    selected_range.set(range);
                },
                view_date: view_date(),
                on_view_change: move |new_view: Date| {
                    tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
                    view_date.set(new_view);
                },
                min_date: date!(1995 - 07 - 21),
                max_date: date!(2035 - 09 - 11),
                month_count: 3,
            }
        }
    }
}
