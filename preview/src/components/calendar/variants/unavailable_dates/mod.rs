use super::super::component::*;
use dioxus::prelude::*;
use time::{ext::NumericalDuration, macros::date, Date};

use dioxus_primitives::calendar::DateRange;

#[component]
pub fn Demo() -> Element {
    let mut selected_range = use_signal(|| None::<DateRange>);

    let start = date!(2026 - 05 - 15);
    let mut view_date = use_signal(|| start);

    let disabled_ranges = use_signal(|| {
        vec![
            DateRange::new(start, start.saturating_add(3.days())),
            DateRange::new(start.saturating_add(15.days()), start.saturating_add(18.days())),
            DateRange::new(start.saturating_add(22.days()), start.saturating_add(23.days())),
        ]
    });

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
                disabled_ranges: disabled_ranges(),
            }
        }
    }
}
