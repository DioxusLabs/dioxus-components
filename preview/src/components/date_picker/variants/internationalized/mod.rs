use super::super::component::*;
use dioxus::prelude::*;

use dioxus_i18n::tid;
use time::Date;

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);

    rsx! {
        div {
            DatePicker {
                selected_date: selected_date(),
                on_value_change: move |v| {
                    tracing::info!("Selected date changed: {:?}", v);
                    selected_date.set(v);
                },
                on_format_day_placeholder: || tid!("D_Abbr"),
                on_format_month_placeholder: || tid!("M_Abbr"),
                on_format_year_placeholder: || tid!("Y_Abbr"),
            }
        }
    }
}
