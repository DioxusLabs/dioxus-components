use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronDown, ChevronLeft, ChevronRight};
use dioxus_primitives::calendar::{
    self, CalendarDayProps, CalendarGridBodyProps, CalendarGridCellProps,
    CalendarGridDayHeaderProps, CalendarGridHeadProps, CalendarGridHeaderRowProps,
    CalendarGridRootProps, CalendarGridWeekProps, CalendarHeaderProps, CalendarNavigationProps,
    CalendarSelectMonthProps, CalendarSelectMonthSelectProps, CalendarSelectMonthValueProps,
    CalendarSelectYearProps, CalendarSelectYearSelectProps, CalendarSelectYearValueProps,
    CalendarViewProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use time::{Date, Month, UtcDateTime, Weekday};

#[css_module("/src/components/calendar/style.css")]
struct Styles;

fn fixed_date(year: i32, month: Month, day: u8) -> Date {
    Date::from_calendar_date(year, month, day).expect("valid fixed date")
}

fn weekday_abbreviation(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "Mo",
        Weekday::Tuesday => "Tu",
        Weekday::Wednesday => "We",
        Weekday::Thursday => "Th",
        Weekday::Friday => "Fr",
        Weekday::Saturday => "Sa",
        Weekday::Sunday => "Su",
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// The selected date
    #[props(default)]
    pub selected_date: ReadSignal<Option<Date>>,

    /// Callback when selected date changes
    #[props(default)]
    pub on_date_change: Callback<Option<Date>>,

    /// Callback when display weekday
    #[props(default = Callback::new(|weekday: Weekday| weekday_abbreviation(weekday).to_string()))]
    pub on_format_weekday: Callback<Weekday, String>,

    /// Callback when display month
    #[props(default = Callback::new(|month: Month| month.to_string()))]
    pub on_format_month: Callback<Month, String>,

    /// The month being viewed
    #[props(default = ReadSignal::new(Signal::new(UtcDateTime::now().date())))]
    pub view_date: ReadSignal<Date>,

    /// The current date (used for highlighting today)
    #[props(default = UtcDateTime::now().date())]
    pub today: Date,

    /// Callback when view date changes
    #[props(default)]
    pub on_view_change: Callback<Date>,

    /// Whether the calendar is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// First day of the week
    #[props(default = Weekday::Sunday)]
    pub first_day_of_week: Weekday,

    /// Lower limit of the range of available dates
    #[props(default = fixed_date(1925, Month::January, 1))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = fixed_date(2050, Month::December, 31))]
    pub max_date: Date,

    /// Specify how many months are visible at once
    #[props(default = 1)]
    pub month_count: u8,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<calendar::DateRange>>,

    /// Additional attributes to extend the calendar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[derive(Props, Clone, PartialEq)]
pub struct RangeCalendarProps {
    /// The selected range
    #[props(default)]
    pub selected_range: ReadSignal<Option<calendar::DateRange>>,

    /// Callback when selected date range changes
    #[props(default)]
    pub on_range_change: Callback<Option<calendar::DateRange>>,

    /// Callback when display weekday
    #[props(default = Callback::new(|weekday: Weekday| weekday_abbreviation(weekday).to_string()))]
    pub on_format_weekday: Callback<Weekday, String>,

    /// Callback when display month
    #[props(default = Callback::new(|month: Month| month.to_string()))]
    pub on_format_month: Callback<Month, String>,

    /// The month being viewed
    #[props(default = ReadSignal::new(Signal::new(UtcDateTime::now().date())))]
    pub view_date: ReadSignal<Date>,

    /// The current date (used for highlighting today)
    #[props(default = UtcDateTime::now().date())]
    pub today: Date,

    /// Callback when view date changes
    #[props(default)]
    pub on_view_change: Callback<Date>,

    /// Whether the calendar is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// First day of the week
    #[props(default = Weekday::Sunday)]
    pub first_day_of_week: Weekday,

    /// Lower limit of the range of available dates
    #[props(default = fixed_date(1925, Month::January, 1))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = fixed_date(2050, Month::December, 31))]
    pub max_date: Date,

    /// Specify how many months are visible at once
    #[props(default = 1)]
    pub month_count: u8,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<calendar::DateRange>>,

    /// Additional attributes to extend the calendar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let month_count = props.month_count.max(1);

    rsx! {
        CalendarRoot {
            selected_date: props.selected_date,
            on_date_change: props.on_date_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            disabled_ranges: props.disabled_ranges,
            attributes: props.attributes,
            for offset in 0..month_count {
                CalendarMonthView { key: "{offset}", offset, month_count }
            }
        }
    }
}

#[component]
pub fn RangeCalendar(props: RangeCalendarProps) -> Element {
    let month_count = props.month_count.max(1);

    rsx! {
        RangeCalendarRoot {
            selected_range: props.selected_range,
            on_range_change: props.on_range_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            disabled_ranges: props.disabled_ranges,
            attributes: props.attributes,
            for offset in 0..month_count {
                CalendarMonthView { key: "{offset}", offset, month_count }
            }
        }
    }
}

#[component]
pub(crate) fn CalendarRoot(props: calendar::CalendarProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_calendar
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        calendar::Calendar {
            selected_date: props.selected_date,
            on_date_change: props.on_date_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            disabled_ranges: props.disabled_ranges,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub(crate) fn RangeCalendarRoot(props: calendar::RangeCalendarProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_calendar
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        calendar::RangeCalendar {
            selected_range: props.selected_range,
            on_range_change: props.on_range_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            disabled_ranges: props.disabled_ranges,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub(crate) fn CalendarMonthView(offset: u8, month_count: u8) -> Element {
    let show_previous = offset == 0;
    let show_next = offset.saturating_add(1) == month_count;

    rsx! {
        CalendarView { offset,
            CalendarHeader {
                CalendarNavigation {
                    if show_previous {
                        CalendarPreviousMonthButton {}
                    }
                    CalendarSelectMonth {}
                    CalendarSelectYear {}
                    if show_next {
                        CalendarNextMonthButton {}
                    }
                }
            }
            CalendarGrid {}
        }
    }
}

#[component]
fn CalendarView(props: CalendarViewProps) -> Element {
    rsx! {
        calendar::CalendarView {
            class: Styles::dx_calendar_view,
            offset: props.offset,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn CalendarHeader(props: CalendarHeaderProps) -> Element {
    rsx! {
        calendar::CalendarHeader { id: props.id, attributes: props.attributes, {props.children} }
    }
}

#[component]
fn CalendarNavigation(props: CalendarNavigationProps) -> Element {
    rsx! {
        calendar::CalendarNavigation { class: Styles::dx_calendar_navigation, attributes: props.attributes, {props.children} }
    }
}

#[component]
fn CalendarPreviousMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarPreviousMonthButton { class: Styles::dx_calendar_nav_prev, attributes,
            ChevronLeft { size: "20px" }
        }
    }
}

#[component]
fn CalendarNextMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarNextMonthButton { class: Styles::dx_calendar_nav_next, attributes,
            ChevronRight { size: "20px" }
        }
    }
}

#[component]
fn CalendarSelectMonth(props: CalendarSelectMonthProps) -> Element {
    rsx! {
        calendar::CalendarSelectMonth {
            attributes: props.attributes,
            class: Styles::dx_calendar_month_select_container,
            CalendarSelectMonthSelect {}
            CalendarSelectMonthValue {
                DropDownIcon { }
                {props.children}
            }
        }
    }
}

#[component]
fn CalendarSelectMonthSelect(props: CalendarSelectMonthSelectProps) -> Element {
    rsx! {
        calendar::CalendarSelectMonthSelect {
            class: Styles::dx_calendar_month_select,
            attributes: props.attributes,
        }
    }
}

#[component]
fn CalendarSelectMonthValue(props: CalendarSelectMonthValueProps) -> Element {
    rsx! {
        calendar::CalendarSelectMonthValue {
            class: Styles::dx_calendar_month_select_value,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn CalendarSelectYear(props: CalendarSelectYearProps) -> Element {
    rsx! {
        calendar::CalendarSelectYear {
            attributes: props.attributes,
            class: Styles::dx_calendar_year_select_container,
            CalendarSelectYearSelect {}
            CalendarSelectYearValue {
                DropDownIcon { }
                {props.children}
            }
        }
    }
}

#[component]
fn CalendarSelectYearSelect(props: CalendarSelectYearSelectProps) -> Element {
    rsx! {
        calendar::CalendarSelectYearSelect {
            class: Styles::dx_calendar_year_select,
            attributes: props.attributes,
        }
    }
}

#[component]
fn CalendarSelectYearValue(props: CalendarSelectYearValueProps) -> Element {
    rsx! {
        calendar::CalendarSelectYearValue {
            class: Styles::dx_calendar_year_select_value,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn CalendarGrid(
    #[props(default)] id: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let grid = calendar::use_calendar_grid();

    rsx! {
        CalendarGridRoot { id, attributes,
            CalendarGridHead {
                CalendarGridHeaderRow {
                    for weekday in grid.weekdays().iter().cloned() {
                        CalendarGridDayHeader {
                            key: "{weekday.weekday():?}",
                            weekday: weekday.weekday(),
                            {weekday.label().to_string()}
                        }
                    }
                }
            }
            CalendarGridBody {
                for week in grid.weeks() {
                    CalendarGridWeek {
                        for date in week.iter().copied() {
                            CalendarGridCell {
                                key: "{date}",
                                CalendarDay { date }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalendarGridRoot(props: CalendarGridRootProps) -> Element {
    rsx! {
        calendar::CalendarGridRoot {
            class: Styles::dx_calendar_grid,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn CalendarGridHead(props: CalendarGridHeadProps) -> Element {
    rsx! {
        calendar::CalendarGridHead {
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn CalendarGridHeaderRow(props: CalendarGridHeaderRowProps) -> Element {
    rsx! {
        calendar::CalendarGridHeaderRow {
            class: Styles::dx_calendar_grid_header,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn CalendarGridDayHeader(props: CalendarGridDayHeaderProps) -> Element {
    rsx! {
        calendar::CalendarGridDayHeader {
            class: Styles::dx_calendar_grid_day_header,
            weekday: props.weekday,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn CalendarGridBody(props: CalendarGridBodyProps) -> Element {
    rsx! {
        calendar::CalendarGridBody {
            class: Styles::dx_calendar_grid_body,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn CalendarGridWeek(props: CalendarGridWeekProps) -> Element {
    rsx! {
        calendar::CalendarGridWeek {
            class: Styles::dx_calendar_grid_week,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn CalendarGridCell(props: CalendarGridCellProps) -> Element {
    rsx! {
        calendar::CalendarGridCell {
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn CalendarDay(props: CalendarDayProps) -> Element {
    rsx! {
        calendar::CalendarDay {
            class: Styles::dx_calendar_grid_cell,
            date: props.date,
            attributes: props.attributes,
            children: props.children,
        }
    }
}

#[component]
fn DropDownIcon() -> Element {
    rsx! {
        ChevronDown {
            size: "20px",
            stroke: "var(--secondary-color-4)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[component]
    fn CalendarWithDefaultView() -> Element {
        rsx! {
            Calendar {
                view_date: fixed_date(2026, Month::May, 15),
            }
        }
    }

    #[component]
    fn CalendarWithDefaultMonthCount() -> Element {
        rsx! {
            Calendar {
                view_date: fixed_date(2026, Month::May, 15),
                month_count: 3,
            }
        }
    }

    #[test]
    fn calendar_renders_default_view_when_children_are_omitted() {
        let mut dom = VirtualDom::new(CalendarWithDefaultView);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("Calendar"));
        assert_eq!(html.matches("role=\"grid\"").count(), 1);
    }

    #[test]
    fn calendar_month_count_renders_multiple_default_views() {
        let mut dom = VirtualDom::new(CalendarWithDefaultMonthCount);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert_eq!(html.matches("role=\"grid\"").count(), 3);
    }
}
