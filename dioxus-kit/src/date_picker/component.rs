use dioxus::prelude::*;

use dioxus_icons::lucide::ChevronDown;
use dioxus_kit_core::{
    calendar::DateRange,
    date_picker::{
        self, DatePickerDaySegmentProps, DatePickerInputProps, DatePickerMonthSegmentProps,
        DatePickerSeparatorProps, DatePickerYearSegmentProps, DateRangePickerEndValueProps,
        DateRangePickerStartValueProps,
    },
    dioxus_attributes::attributes,
    merge_attributes,
    popover::{PopoverContentProps, PopoverTriggerProps},
    ContentAlign,
};
use time::{Date, Month};

use super::super::calendar::*;
use super::super::popover::*;

#[css_module("/src/date_picker/style.css")]
struct Styles;

fn fixed_date(year: i32, month: Month, day: u8) -> Date {
    Date::from_calendar_date(year, month, day).expect("valid fixed date")
}

#[derive(Clone, Copy)]
struct StyledDatePickerContext {
    month_count: u8,
}

fn styled_month_count() -> u8 {
    try_use_context::<StyledDatePickerContext>()
        .map(|ctx| ctx.month_count)
        .unwrap_or(1)
        .max(1)
}

#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<Option<Date>>,

    /// The selected date
    #[props(default)]
    pub selected_date: ReadSignal<Option<Date>>,

    /// Whether the date picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the date picker is enable user input
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub read_only: ReadSignal<bool>,

    /// Lower limit of the range of available dates
    #[props(default = fixed_date(1925, Month::January, 1))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = fixed_date(2050, Month::December, 31))]
    pub max_date: Date,

    /// Callback when display day placeholder
    #[props(default = Callback::new(|_| "D".to_string()))]
    pub on_format_day_placeholder: Callback<(), String>,

    /// Callback when display month placeholder
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_month_placeholder: Callback<(), String>,

    /// Callback when display year placeholder
    #[props(default = Callback::new(|_| "Y".to_string()))]
    pub on_format_year_placeholder: Callback<(), String>,

    /// Specify how many months are visible at once
    #[props(default = 1)]
    pub month_count: u8,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<DateRange>>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to extend the date picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[derive(Props, Clone, PartialEq)]
pub struct DateRangePickerProps {
    /// Callback when value changes
    #[props(default)]
    pub on_range_change: Callback<Option<DateRange>>,

    /// The selected date
    #[props(default)]
    pub selected_range: ReadSignal<Option<DateRange>>,

    /// Whether the date picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the date picker is enable user input
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub read_only: ReadSignal<bool>,

    /// Lower limit of the range of available dates
    #[props(default = fixed_date(1925, Month::January, 1))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = fixed_date(2050, Month::December, 31))]
    pub max_date: Date,

    /// Callback when display day placeholder
    #[props(default = Callback::new(|_| "D".to_string()))]
    pub on_format_day_placeholder: Callback<(), String>,

    /// Callback when display month placeholder
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_month_placeholder: Callback<(), String>,

    /// Callback when display year placeholder
    #[props(default = Callback::new(|_| "Y".to_string()))]
    pub on_format_year_placeholder: Callback<(), String>,

    /// Specify how many months are visible at once
    #[props(default = 1)]
    pub month_count: u8,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<DateRange>>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to extend the date picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_date_picker
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let month_count = props.month_count.max(1);
    use_context_provider(|| StyledDatePickerContext { month_count });

    rsx! {
        div {
            date_picker::DatePicker {
                on_value_change: props.on_value_change,
                selected_date: props.selected_date,
                disabled: props.disabled,
                read_only: props.read_only,
                min_date: props.min_date,
                max_date: props.max_date,
                disabled_ranges: props.disabled_ranges,
                roving_loop: props.roving_loop,
                attributes: merged,
                date_picker::DatePickerPopover {
                    popover_root: PopoverRoot,
                    DatePickerInput {
                        on_format_day_placeholder: props.on_format_day_placeholder,
                        on_format_month_placeholder: props.on_format_month_placeholder,
                        on_format_year_placeholder: props.on_format_year_placeholder,
                    }
                }
            }
        }
    }
}

#[component]
pub fn DateRangePicker(props: DateRangePickerProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_date_picker
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let month_count = props.month_count.max(1);
    use_context_provider(|| StyledDatePickerContext { month_count });

    rsx! {
        div {
            date_picker::DateRangePicker {
                on_range_change: props.on_range_change,
                selected_range: props.selected_range,
                disabled: props.disabled,
                read_only: props.read_only,
                min_date: props.min_date,
                max_date: props.max_date,
                disabled_ranges: props.disabled_ranges,
                roving_loop: props.roving_loop,
                attributes: merged,
                date_picker::DatePickerPopover {
                    popover_root: PopoverRoot,
                    DateRangePickerInput {
                        on_format_day_placeholder: props.on_format_day_placeholder,
                        on_format_month_placeholder: props.on_format_month_placeholder,
                        on_format_year_placeholder: props.on_format_year_placeholder,
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn DatePickerInput(props: DatePickerInputProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_date_picker_group
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let extra_children = props.children;
    let month_count = styled_month_count();

    rsx! {
        date_picker::DatePickerInput {
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: merged,
            date_picker::DatePickerInputValue {
                on_format_day_placeholder: props.on_format_day_placeholder,
                on_format_month_placeholder: props.on_format_month_placeholder,
                on_format_year_placeholder: props.on_format_year_placeholder,
                DatePickerYearSegment {}
                DatePickerSeparator {}
                DatePickerMonthSegment {}
                DatePickerSeparator {}
                DatePickerDaySegment {}
            }
            if let Some(extra_children) = extra_children {
                {extra_children}
            }
            DatePickerPopoverTrigger {}
            DatePickerPopoverContent { align: ContentAlign::Center,
                date_picker::DatePickerCalendar { calendar: CalendarRoot,
                    for offset in 0..month_count {
                        CalendarMonthView { key: "{offset}", offset, month_count }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn DateRangePickerInput(props: DatePickerInputProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_date_picker_group
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let extra_children = props.children;
    let month_count = styled_month_count();

    rsx! {
        date_picker::DateRangePickerInput {
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: merged,
            date_picker::DateRangePickerInputValue {
                on_format_day_placeholder: props.on_format_day_placeholder,
                on_format_month_placeholder: props.on_format_month_placeholder,
                on_format_year_placeholder: props.on_format_year_placeholder,
                DateRangePickerStartValue {
                    DatePickerYearSegment {}
                    DatePickerSeparator {}
                    DatePickerMonthSegment {}
                    DatePickerSeparator {}
                    DatePickerDaySegment {}
                }
                DatePickerSeparator { symbol: '—' }
                DateRangePickerEndValue {
                    DatePickerYearSegment {}
                    DatePickerSeparator {}
                    DatePickerMonthSegment {}
                    DatePickerSeparator {}
                    DatePickerDaySegment {}
                }
            }
            if let Some(extra_children) = extra_children {
                {extra_children}
            }
            DatePickerPopoverTrigger {}
            DatePickerPopoverContent {
                align: ContentAlign::Center,
                date_picker::DateRangePickerCalendar {
                    calendar: RangeCalendarRoot,
                    for offset in 0..month_count {
                        CalendarMonthView { key: "{offset}", offset, month_count }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn DatePickerYearSegment(props: DatePickerYearSegmentProps) -> Element {
    rsx! {
        date_picker::DatePickerYearSegment {
            class: Styles::dx_date_segment,
            attributes: props.attributes,
        }
    }
}

#[component]
pub(crate) fn DatePickerMonthSegment(props: DatePickerMonthSegmentProps) -> Element {
    rsx! {
        date_picker::DatePickerMonthSegment {
            class: Styles::dx_date_segment,
            attributes: props.attributes,
        }
    }
}

#[component]
pub(crate) fn DatePickerDaySegment(props: DatePickerDaySegmentProps) -> Element {
    rsx! {
        date_picker::DatePickerDaySegment {
            class: Styles::dx_date_segment,
            attributes: props.attributes,
        }
    }
}

#[component]
pub(crate) fn DatePickerSeparator(props: DatePickerSeparatorProps) -> Element {
    rsx! {
        date_picker::DatePickerSeparator {
            class: Styles::dx_date_segment,
            symbol: props.symbol,
            attributes: props.attributes,
        }
    }
}

#[component]
pub(crate) fn DateRangePickerStartValue(props: DateRangePickerStartValueProps) -> Element {
    rsx! {
        date_picker::DateRangePickerStartValue {
            {props.children}
        }
    }
}

#[component]
pub(crate) fn DateRangePickerEndValue(props: DateRangePickerEndValueProps) -> Element {
    rsx! {
        date_picker::DateRangePickerEndValue {
            {props.children}
        }
    }
}

#[component]
pub(crate) fn DatePickerPopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        PopoverTrigger {
            class: Styles::dx_date_picker_popover_trigger,
            aria_label: "Show Calendar",
            attributes: props.attributes,
            ChevronDown {
                class: Styles::dx_date_picker_trigger,
                size: "20px",
                stroke: "var(--primary-color-7)",
            }
        }
    }
}

#[component]
pub(crate) fn DatePickerPopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        PopoverContent {
            class: Styles::dx_date_picker_popover_content.to_string(),
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_kit_core::calendar::DateRange;

    #[component]
    fn DatePickerWithDefaultInput() -> Element {
        rsx! {
            DatePicker {
                selected_date: Some(fixed_date(2026, Month::May, 7)),
            }
        }
    }

    #[component]
    fn DateRangePickerWithDefaultInput() -> Element {
        rsx! {
            DateRangePicker {
                selected_range: Some(DateRange::new(
                    fixed_date(2026, Month::May, 7),
                    fixed_date(2026, Month::May, 11),
                )),
            }
        }
    }

    #[test]
    fn date_picker_renders_default_input_when_children_are_omitted() {
        let mut dom = VirtualDom::new(DatePickerWithDefaultInput);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("2026"));
        assert!(html.contains("05"));
        assert!(html.contains("07"));
        assert!(html.contains("Show Calendar"));
    }

    #[test]
    fn date_range_picker_renders_default_input_when_children_are_omitted() {
        let mut dom = VirtualDom::new(DateRangePickerWithDefaultInput);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("2026"));
        assert!(html.contains("05"));
        assert!(html.contains("07"));
        assert!(html.contains("11"));
        assert!(html.contains("Show Calendar"));
    }
}
