//! Defines the [`DatePicker`] and [`DateRangePicker`] components and its subcomponents, which allowing users to enter or select a date value

use crate::{
    calendar::{
        weekday_abbreviation, AvailableRanges, CalendarProps, DateRange, RangeCalendarProps,
    },
    dioxus_core::Properties,
    focus::{use_focus_controlled_item_disabled, use_focus_provider, FocusState},
    popover::*,
    use_unique_id, LocalDateExt as _,
};

use dioxus::prelude::*;
use num_integer::Integer;
use std::{fmt::Display, str::FromStr};
use time::{macros::date, Date, Month, OffsetDateTime, Weekday};

/// The context provided by the [`DatePicker`] component to its children.
#[derive(Copy, Clone)]
struct BaseDatePickerContext {
    // State
    open: Signal<bool>,
    read_only: ReadSignal<bool>,

    // Configuration
    disabled: ReadSignal<bool>,
    focus: FocusState,
    enabled_date_range: DateRange,
    available_ranges: Memo<AvailableRanges>,
}

/// The context provided by the [`DatePicker`] component to its children.
#[derive(Copy, Clone)]
struct DatePickerContext {
    on_value_change: Callback<Option<Date>>,
    selected_date: ReadSignal<Option<Date>>,
}

impl DatePickerContext {
    fn set_date(&mut self, date: Option<Date>) {
        let value = { self.selected_date.peek().cloned() };
        if value != date {
            self.on_value_change.call(date);
        }
    }
}

/// The props for the [`DatePicker`] component.
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
    #[props(default = date!(1925-01-01))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = date!(2050-12-31))]
    pub max_date: Date,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<DateRange>>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to extend the date picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the date picker element
    pub children: Element,
}

/// # DatePicker
///
/// The [`DatePicker`] component provides an accessible date input interface.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{calendar::Calendar, date_picker::*, popover::*, ContentAlign};
/// use time::Date;
/// #[component]
/// fn Demo() -> Element {
///    let mut selected_date = use_signal(|| None::<Date>);
///    rsx! {
///        div {
///            DatePicker {
///                selected_date: selected_date(),
///                on_value_change: move |date| {
///                    tracing::info!("Date changed to: {date:?}");
///                    selected_date.set(date);
///               },
///                DatePickerPopover {
///                    DatePickerInput {
///                        PopoverTrigger {
///                            "Select date"
///                        }
///                        PopoverContent {
///                            align: ContentAlign::End,
///                            DatePickerCalendar {
///                                calendar: Calendar,
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
///
/// # Styling
///
/// The [`DatePicker`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the DatePicker is disabled. Possible values are `true` or `false`.
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let open = use_signal(|| false);
    let focus = use_focus_provider(props.roving_loop);
    let available_ranges = use_memo(move || AvailableRanges::new(&props.disabled_ranges.read()));

    // Create context provider for child components
    use_context_provider(|| BaseDatePickerContext {
        open,
        read_only: props.read_only,
        disabled: props.disabled,
        focus,
        enabled_date_range: DateRange::new(props.min_date, props.max_date),
        available_ranges,
    });

    use_context_provider(|| DatePickerContext {
        on_value_change: props.on_value_change,
        selected_date: props.selected_date,
    });

    rsx! {
        div {
            role: "group",
            aria_label: "Date",
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The context provided by the [`DateRangePicker`] component to its children.
#[derive(Copy, Clone)]
pub struct DateRangePickerContext {
    // Currently selected date range
    date_range: ReadSignal<Option<DateRange>>,
    set_selected_range: Callback<Option<DateRange>>,
}

impl DateRangePickerContext {
    /// Set the selected date
    pub fn set_range(&mut self, range: Option<DateRange>) {
        if (self.date_range)() != range {
            self.set_selected_range.call(range);
        }
    }
}

/// The props for the [`DatePicker`] component.
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
    #[props(default = date!(1925-01-01))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = date!(2050-12-31))]
    pub max_date: Date,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<DateRange>>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to extend the date picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the date picker element
    pub children: Element,
}

/// # DateRangePicker
///
/// The [`DateRangePicker`] component provides an accessible date range input interface.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{calendar::{DateRange, RangeCalendar}, date_picker::*, popover::*, ContentAlign};
/// #[component]
/// fn Demo() -> Element {
///    let mut selected_range = use_signal(|| None::<DateRange>);
///    rsx! {
///        div {
///            DateRangePicker {
///                selected_range: selected_range(),
///                on_range_change: move |range| {
///                    tracing::info!("Selected range: {:?}", range);
///                    selected_range.set(range);
///               },
///                DatePickerPopover {
///                    DatePickerInput {
///                        PopoverTrigger {
///                            "Select date"
///                        }
///                        PopoverContent {
///                            align: ContentAlign::End,
///                            DateRangePickerCalendar {
///                                calendar: RangeCalendar,
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
///
/// # Styling
///
/// The [`DateRangePicker`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the DateRangePicker is disabled. Possible values are `true` or `false`.
#[component]
pub fn DateRangePicker(props: DateRangePickerProps) -> Element {
    let open = use_signal(|| false);
    let focus = use_focus_provider(props.roving_loop);

    let available_ranges = use_memo(move || AvailableRanges::new(&props.disabled_ranges.read()));

    // Create context provider for child components
    use_context_provider(|| BaseDatePickerContext {
        open,
        read_only: props.read_only,
        disabled: props.disabled,
        focus,
        enabled_date_range: DateRange::new(props.min_date, props.max_date),
        available_ranges,
    });

    use_context_provider(|| DateRangePickerContext {
        date_range: props.selected_range,
        set_selected_range: props.on_range_change,
    });

    rsx! {
        div {
            role: "group",
            aria_label: "Date Range",
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DatePickerPopover`] component.
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerPopoverProps {
    /// Whether the popover is a modal and should capture focus.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled open state of the popover.
    pub open: ReadSignal<Option<bool>>,

    /// The default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback fired when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes to apply to the popover root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the popover root component.
    pub children: Element,

    /// The popover root component to use.
    #[props(default = PopoverRoot)]
    pub popover_root: fn(PopoverRootProps) -> Element,
}

/// # DatePickerPopover
///
/// The `DatePickerPopover` component wraps all the popover components and manages the state.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{calendar::Calendar, date_picker::*, popover::*, ContentAlign};
/// use time::Date;
/// #[component]
/// fn Demo() -> Element {
///    let mut selected_date = use_signal(|| None::<Date>);
///    rsx! {
///        div {
///            DatePicker {
///                selected_date: selected_date(),
///                on_value_change: move |date| {
///                    tracing::info!("Date changed to: {date:?}");
///                    selected_date.set(date);
///               },
///                DatePickerPopover {
///                    DatePickerInput {
///                        PopoverTrigger {
///                            "Select date"
///                        }
///                        PopoverContent {
///                            align: ContentAlign::End,
///                            DatePickerCalendar {
///                                calendar: Calendar,
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
#[component]
pub fn DatePickerPopover(props: DatePickerPopoverProps) -> Element {
    let ctx = use_context::<BaseDatePickerContext>();
    let mut open = ctx.open;

    let PopoverRoot = props.popover_root;

    rsx! {
        PopoverRoot {
            open: open(),
            on_open_change: move |v| open.set(v),
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[doc(hidden)]
/// A trait for types that can provide default calendar rendering.
pub trait DefaultCalendarProps {
    /// Provide a default calendar rendering function.
    fn default_calendar(self) -> Element;
}

/// The props for the Calendar component.
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerCalendarProps<T: DefaultCalendarProps + Properties + PartialEq> {
    /// Callback when display weekday
    #[props(default = Callback::new(|weekday: Weekday| weekday_abbreviation(weekday).to_string()))]
    pub on_format_weekday: Callback<Weekday, String>,

    /// Callback when display month
    #[props(default = Callback::new(|month: Month| month.to_string()))]
    pub on_format_month: Callback<Month, String>,

    /// The month being viewed
    #[props(default = ReadSignal::new(Signal::new(OffsetDateTime::now_local_date())))]
    pub view_date: ReadSignal<Date>,

    /// The current date (used for highlighting today)
    #[props(default = OffsetDateTime::now_local_date())]
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
    #[props(default = date!(1925-01-01))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = date!(2050-12-31))]
    pub max_date: Date,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<DateRange>>,

    /// Additional attributes to extend the calendar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the calendar element
    pub children: Element,

    /// The calendar to render with
    #[props(default = T::default_calendar)]
    pub calendar: fn(T) -> Element,
}

/// # DatePickerCalendar
///
/// The [`DatePickerCalendar`] component provides an accessible calendar interface with arrow key navigation, month switching, and date selection.
/// Used as date picker popover component
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{calendar::Calendar, date_picker::*, popover::*, ContentAlign};
/// use time::Date;
/// #[component]
/// fn Demo() -> Element {
///    let mut selected_date = use_signal(|| None::<Date>);
///    rsx! {
///        div {
///            DatePicker {
///                selected_date: selected_date(),
///                on_value_change: move |date| {
///                    tracing::info!("Date changed to: {date:?}");
///                    selected_date.set(date);
///               },
///                DatePickerPopover {
///                    DatePickerInput {
///                        PopoverTrigger {
///                            "Select date"
///                        }
///                        PopoverContent {
///                            align: ContentAlign::End,
///                            DatePickerCalendar {}
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
#[component]
pub fn DatePickerCalendar(props: DatePickerCalendarProps<CalendarProps>) -> Element {
    let mut base_ctx = use_context::<BaseDatePickerContext>();
    let mut ctx = use_context::<DatePickerContext>();

    #[allow(non_snake_case)]
    let Calendar = props.calendar;
    let mut view_date = use_signal(|| props.today);
    use_effect(move || {
        if let Some(date) = (ctx.selected_date)() {
            view_date.set(date);
        }
    });

    let min_date = base_ctx.enabled_date_range.start();
    let max_date = base_ctx.enabled_date_range.end();

    rsx! {
        Calendar {
            selected_date: ctx.selected_date,
            on_date_change: move |date| {
                ctx.set_date(date);
                base_ctx.open.set(false);
            },
            disabled_ranges: base_ctx.available_ranges.read().to_disabled_ranges(),
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: view_date(),
            on_view_change: move |date| view_date.set(date),
            today: props.today,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date,
            max_date,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// # DateRangePickerCalendar
///
/// The [`DateRangePickerCalendar`] component provides an accessible calendar interface with arrow key navigation, month switching, and date range selection.
/// Used as date picker popover component
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{calendar::{DateRange, RangeCalendar}, date_picker::*, popover::*, ContentAlign};
/// #[component]
/// fn Demo() -> Element {
///    let mut selected_range = use_signal(|| None::<DateRange>);
///    rsx! {
///        div {
///            DateRangePicker {
///                selected_range: selected_range(),
///                on_range_change: move |range| {
///                    tracing::info!("Selected range: {:?}", range);
///                    selected_range.set(range);
///               },
///                DatePickerPopover {
///                    DateRangePickerInput {
///                        PopoverTrigger {
///                            "Select date"
///                        }
///                        PopoverContent {
///                            align: ContentAlign::End,
///                            DateRangePickerCalendar {}
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
#[component]
pub fn DateRangePickerCalendar(props: DatePickerCalendarProps<RangeCalendarProps>) -> Element {
    let mut base_ctx = use_context::<BaseDatePickerContext>();
    let mut ctx = use_context::<DateRangePickerContext>();

    #[allow(non_snake_case)]
    let RangeCalendar = props.calendar;
    let mut view_date = use_signal(|| props.today);
    use_effect(move || {
        if let Some(r) = (ctx.date_range)() {
            view_date.set(r.start());
        }
    });

    let min_date = base_ctx.enabled_date_range.start();
    let max_date = base_ctx.enabled_date_range.end();

    rsx! {
        RangeCalendar {
            selected_range: ctx.date_range,
            on_range_change: move |range| {
                ctx.set_range(range);
                base_ctx.open.set(false);
            },
            disabled_ranges: base_ctx.available_ranges.read().to_disabled_ranges(),
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: view_date(),
            on_view_change: move |date| view_date.set(date),
            today: props.today,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date,
            max_date,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// The props for the [`DateSegment`] component
#[derive(Props, Clone, PartialEq)]
struct DateSegmentProps<T: Clone + Integer + 'static> {
    // The index of the segment
    pub index: ReadSignal<usize>,

    // The controlled value of the date picker
    pub value: ReadSignal<Option<T>>,

    // Default value
    pub default: T,

    // Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    // The minimum value
    pub min: ReadSignal<T>,

    // The maximum value
    pub max: ReadSignal<T>,

    // Max field length
    pub max_length: usize,

    // Callback when display placeholder
    pub on_format_placeholder: Callback<(), String>,

    // Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
fn DateSegment<T: Clone + Copy + Integer + FromStr + Display + 'static>(
    props: DateSegmentProps<T>,
) -> Element {
    let mut text_value = use_signal(|| "".to_string());
    use_effect(move || {
        let text = match (props.value)() {
            Some(value) => value.to_string(),
            None => String::default(),
        };
        text_value.set(text);
    });

    let mut reset_value = use_signal(|| false);

    // The formatted text for the segment
    let display_value = use_memo(move || {
        let value = (props.value)();
        match value {
            Some(value) => format!("{:0>width$}", value, width = props.max_length),
            None => props
                .on_format_placeholder
                .call(())
                .repeat(props.max_length),
        }
    });

    let now_value = use_memo(move || (props.value)().unwrap_or(props.default));

    let mut ctx = use_context::<BaseDatePickerContext>();

    let mut set_value = move |text: String| {
        if text.is_empty() {
            props.on_value_change.call(None);
            ctx.focus.focus_prev();
            return;
        }
        let min = props.min.cloned();
        let max = props.max.cloned();

        let value = text.parse::<T>().map(|v| v.min(max)).ok();
        if let Some(value) = value {
            let inRange = value >= min && value <= max;

            // If adding a new digit would exceed max, move to next segment
            let newValue = (text + "0").parse::<T>().unwrap_or(value);
            if inRange && newValue > max {
                ctx.focus.focus_next();
            }
        };

        props.on_value_change.call(value);
    };
    use_effect(move || {
        // If this item is not focused, always keep the value clamped
        if !ctx.focus.is_focused(props.index.cloned()) {
            if let Some(value) = (props.value)() {
                let clamped_value = value.clamp(props.min.cloned(), props.max.cloned());
                if clamped_value != value {
                    props.on_value_change.call(Some(clamped_value));
                }
            }
        }
    });

    let roll_value = move |value: T| {
        let min = props.min.cloned();
        let max = props.max.cloned();
        if value < min {
            max
        } else if value > max {
            min
        } else {
            value
        }
    };

    let handle_keydown = move |event: Event<KeyboardData>| {
        let key = event.key();
        match key {
            Key::Character(actual_char) => {
                // Don't block keyboard shortcuts
                if event.modifiers().ctrl() || event.modifiers().meta() || event.modifiers().alt() {
                    return;
                }
                if actual_char.parse::<T>().is_ok() {
                    let mut text = text_value();
                    if text.len() == props.max_length || reset_value() {
                        text = String::default();
                        reset_value.set(false);
                    };
                    text.push_str(&actual_char);
                    set_value(text);
                }
                event.prevent_default();
                event.stop_propagation();
            }
            Key::Backspace => {
                let mut text = text_value();
                if event.modifiers().ctrl() || event.modifiers().meta() {
                    text.clear();
                } else {
                    text.pop();
                }
                set_value(text);
            }
            Key::Delete => {
                let mut text = text_value();
                text.remove(0);
                set_value(text);
            }
            Key::ArrowLeft => {
                ctx.focus.focus_prev();
            }
            Key::ArrowRight => {
                ctx.focus.focus_next();
            }
            Key::Enter => {
                ctx.focus.focus_next();
                event.prevent_default();
                event.stop_propagation();
            }
            Key::ArrowUp => {
                let value = match (props.value)() {
                    Some(mut value) => {
                        value.inc();
                        roll_value(value)
                    }
                    None => props.default,
                };
                props.on_value_change.call(Some(value));
            }
            Key::ArrowDown => {
                let value = match (props.value)() {
                    Some(mut value) => {
                        value.dec();
                        roll_value(value)
                    }
                    None => props.default,
                };
                props.on_value_change.call(Some(value));
            }
            _ => (),
        }
    };

    let disabled = move || (ctx.disabled)();
    let onmounted = use_focus_controlled_item_disabled(props.index, disabled);

    let span_id = use_unique_id();
    let id = use_memo(move || format!("span-{span_id}"));
    let label_id = format!("{id}-label");

    rsx! {
        span {
            id,
            role: "spinbutton",
            aria_valuemin: props.min.to_string(),
            aria_valuemax: props.max.to_string(),
            aria_valuenow: now_value.to_string(),
            aria_labelledby: "{label_id}",
            inputmode: "numeric",
            contenteditable: !(ctx.read_only)(),
            spellcheck: false,
            tabindex: "0",
            enterkeyhint: "next",
            onkeydown: handle_keydown,
            onmounted,
            onfocus: move |_| {
                reset_value.set(true);
                ctx.focus.set_focus(Some(props.index.cloned()));
                if (ctx.open)() {
                    ctx.open.set(false);
                }
            },
            "no-date": (props.value)().is_none(),
            "data-disabled": (ctx.disabled)(),
            ..props.attributes,
            {display_value}
        }
    }
}

#[derive(Clone, Copy)]
struct DateElementContext {
    start_index: usize,
    year_value: Signal<Option<i32>>,
    month_value: Signal<Option<u8>>,
    day_value: Signal<Option<u8>>,
    on_format_day_placeholder: Callback<(), String>,
    on_format_month_placeholder: Callback<(), String>,
    on_format_year_placeholder: Callback<(), String>,
}

/// The props for the [`DatePickerYearSegment`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerYearSegmentProps {
    /// Additional attributes for the year segment element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The props for the [`DatePickerMonthSegment`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerMonthSegmentProps {
    /// Additional attributes for the month segment element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The props for the [`DatePickerDaySegment`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerDaySegmentProps {
    /// Additional attributes for the day segment element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The props for the [`DatePickerSeparator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerSeparatorProps {
    /// The separator symbol.
    #[props(default = '-')]
    pub symbol: char,

    /// Additional attributes for the separator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A year segment in a date input.
#[component]
pub fn DatePickerYearSegment(props: DatePickerYearSegmentProps) -> Element {
    let mut ctx = use_context::<DateElementContext>();
    let base_ctx = use_context::<BaseDatePickerContext>();
    let today = OffsetDateTime::now_local_date();
    let min_year = base_ctx.enabled_date_range.start().year();
    let max_year = base_ctx.enabled_date_range.end().year();

    rsx! {
        DateSegment {
            aria_label: "year",
            index: ctx.start_index,
            value: ctx.year_value,
            default: today.year(),
            on_value_change: move |value: Option<i32>| ctx.year_value.set(value),
            min: min_year,
            max: max_year,
            max_length: 4,
            on_format_placeholder: ctx.on_format_year_placeholder,
            attributes: props.attributes,
        }
    }
}

/// A month segment in a date input.
#[component]
pub fn DatePickerMonthSegment(props: DatePickerMonthSegmentProps) -> Element {
    let mut ctx = use_context::<DateElementContext>();
    let base_ctx = use_context::<BaseDatePickerContext>();
    let today = OffsetDateTime::now_local_date();
    let min_date = base_ctx.enabled_date_range.start();
    let max_date = base_ctx.enabled_date_range.end();
    let min_year = min_date.year();
    let max_year = max_date.year();
    let min_month = match (ctx.year_value)() {
        Some(year) if year == min_year => min_date.month(),
        _ => Month::January,
    };
    let max_month = match (ctx.year_value)() {
        Some(year) if year == max_year => max_date.month(),
        _ => Month::December,
    };

    rsx! {
        DateSegment {
            aria_label: "month",
            index: ctx.start_index + 1usize,
            value: ctx.month_value,
            default: today.month() as u8,
            on_value_change: move |value: Option<u8>| ctx.month_value.set(value),
            min: min_month as u8,
            max: max_month as u8,
            max_length: 2,
            on_format_placeholder: ctx.on_format_month_placeholder,
            attributes: props.attributes,
        }
    }
}

/// A day segment in a date input.
#[component]
pub fn DatePickerDaySegment(props: DatePickerDaySegmentProps) -> Element {
    let mut ctx = use_context::<DateElementContext>();
    let base_ctx = use_context::<BaseDatePickerContext>();
    let today = OffsetDateTime::now_local_date();
    let min_date = base_ctx.enabled_date_range.start();
    let max_date = base_ctx.enabled_date_range.end();
    let min_year = min_date.year();
    let max_year = max_date.year();
    let min_day = match ((ctx.year_value)(), (ctx.month_value)()) {
        (Some(year), Some(month)) if year == min_year && month == min_date.month() as u8 => {
            min_date.day()
        }
        _ => 1,
    };
    let max_day = match ((ctx.year_value)(), (ctx.month_value)()) {
        (Some(year), Some(month)) if year == max_year && month == max_date.month() as u8 => {
            max_date.day()
        }
        (Some(year), Some(month)) => {
            if let Ok(month) = Month::try_from(month) {
                month.length(year)
            } else {
                31
            }
        }
        _ => 31,
    };

    rsx! {
        DateSegment {
            aria_label: "day",
            index: ctx.start_index + 2usize,
            value: ctx.day_value,
            default: today.day(),
            on_value_change: move |value: Option<u8>| ctx.day_value.set(value),
            min: min_day,
            max: max_day,
            max_length: 2,
            on_format_placeholder: ctx.on_format_day_placeholder,
            attributes: props.attributes,
        }
    }
}

/// A separator in a date input.
#[component]
pub fn DatePickerSeparator(props: DatePickerSeparatorProps) -> Element {
    rsx! {
        span {
            aria_hidden: "true",
            tabindex: "-1",
            "is-separator": true,
            "no-date": true,
            ..props.attributes,
            "{props.symbol}"
        }
    }
}

/// The props for the [`DatePickerInputValue`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerInputValueProps {
    /// Callback when display day placeholder
    #[props(default = Callback::new(|_| "D".to_string()))]
    pub on_format_day_placeholder: Callback<(), String>,

    /// Callback when display month placeholder
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_month_placeholder: Callback<(), String>,

    /// Callback when display year placeholder
    #[props(default = Callback::new(|_| "Y".to_string()))]
    pub on_format_year_placeholder: Callback<(), String>,

    /// The children of the date value.
    #[props(default)]
    pub children: Option<Element>,
}

/// The props for the [`DateRangePickerInputValue`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DateRangePickerInputValueProps {
    /// Callback when display day placeholder
    #[props(default = Callback::new(|_| "D".to_string()))]
    pub on_format_day_placeholder: Callback<(), String>,

    /// Callback when display month placeholder
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_month_placeholder: Callback<(), String>,

    /// Callback when display year placeholder
    #[props(default = Callback::new(|_| "Y".to_string()))]
    pub on_format_year_placeholder: Callback<(), String>,

    /// The children of the date range value.
    #[props(default)]
    pub children: Option<Element>,
}

/// The props for the [`DateRangePickerStartValue`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DateRangePickerStartValueProps {
    /// The children of the start date value.
    #[props(default)]
    pub children: Option<Element>,
}

/// The props for the [`DateRangePickerEndValue`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DateRangePickerEndValueProps {
    /// The children of the end date value.
    #[props(default)]
    pub children: Option<Element>,
}

#[derive(Clone, Copy)]
struct DateRangeInputContext {
    start_date: Signal<Option<Date>>,
    end_date: Signal<Option<Date>>,
    on_format_day_placeholder: Callback<(), String>,
    on_format_month_placeholder: Callback<(), String>,
    on_format_year_placeholder: Callback<(), String>,
}

#[derive(Props, Clone, PartialEq)]
struct DateElementProps {
    /// The start index (used for focus)
    #[props(default = 0)]
    pub start_index: usize,

    /// The selected date
    pub selected_date: ReadSignal<Option<Date>>,

    /// Callback when selected date changes
    #[props(default)]
    pub on_date_change: Callback<Option<Date>>,

    /// Callback when display day placeholder
    #[props(default = Callback::new(|_| "D".to_string()))]
    pub on_format_day_placeholder: Callback<(), String>,

    /// Callback when display month placeholder
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_month_placeholder: Callback<(), String>,

    /// Callback when display year placeholder
    #[props(default = Callback::new(|_| "Y".to_string()))]
    pub on_format_year_placeholder: Callback<(), String>,

    /// The children of the date element.
    #[props(default)]
    pub children: Option<Element>,
}

#[component]
fn DateElement(props: DateElementProps) -> Element {
    let ctx = use_context::<BaseDatePickerContext>();
    let selected_date = props.selected_date.peek().cloned();

    let mut day_value = use_signal(move || selected_date.map(|date| date.day()));
    let mut month_value = use_signal(move || selected_date.map(|date| date.month() as u8));
    let mut year_value = use_signal(move || selected_date.map(|date| date.year()));

    use_effect(move || {
        let date = (props.selected_date)();
        year_value.set(date.map(|d| d.year()));
        month_value.set(date.map(|d| d.month() as u8));
        day_value.set(date.map(|d| d.day()));
    });

    use_effect(move || {
        if let (Some(year), Some(month), Some(day)) = (
            year_value(),
            month_value().and_then(|m| Month::try_from(m).ok()),
            day_value(),
        ) {
            if let Some(date) = Date::from_calendar_date(year, month, day)
                .ok()
                .filter(|date| ctx.enabled_date_range.contains(*date))
                .filter(|date| ctx.available_ranges.read().valid_interval(*date))
            {
                props.on_date_change.call(Some(date));
            }
        }
    });

    use_context_provider(|| DateElementContext {
        start_index: props.start_index,
        year_value,
        month_value,
        day_value,
        on_format_day_placeholder: props.on_format_day_placeholder,
        on_format_month_placeholder: props.on_format_month_placeholder,
        on_format_year_placeholder: props.on_format_year_placeholder,
    });

    let children = props.children.unwrap_or_else(|| {
        rsx! {
            DatePickerYearSegment {}
            DatePickerSeparator {}
            DatePickerMonthSegment {}
            DatePickerSeparator {}
            DatePickerDaySegment {}
        }
    });

    rsx! {
        {children}
    }
}

/// The editable date value for a single date picker input.
#[component]
pub fn DatePickerInputValue(props: DatePickerInputValueProps) -> Element {
    let mut base_ctx = use_context::<BaseDatePickerContext>();
    let mut ctx = use_context::<DatePickerContext>();

    rsx! {
        DateElement {
            selected_date: ctx.selected_date,
            on_date_change: move |date| {
                ctx.set_date(date);
                base_ctx.open.set(false);
            },
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            children: props.children,
        }
    }
}

/// The editable date range value for a date range picker input.
#[component]
pub fn DateRangePickerInputValue(props: DateRangePickerInputValueProps) -> Element {
    let base_ctx = use_context::<BaseDatePickerContext>();
    let mut ctx = use_context::<DateRangePickerContext>();
    let selected_range = ctx.date_range.peek().cloned();

    let mut start_date = use_signal(move || selected_range.map(|range| range.start()));
    let mut end_date = use_signal(move || selected_range.map(|range| range.end()));

    use_effect(move || {
        let date_range = ctx.date_range.cloned();
        start_date.set(date_range.map(|r| r.start()));
        end_date.set(date_range.map(|r| r.end()));
    });

    use_effect(move || {
        if let (Some(start), Some(end)) = (start_date(), end_date()) {
            // force auto validation for input range
            if end < start {
                return;
            }

            // checking non-contiguous ranges
            if base_ctx
                .available_ranges
                .read()
                .available_range(start, base_ctx.enabled_date_range)
                .is_some_and(|r| r.contains(end))
            {
                let range = Some(DateRange::new(start, end));
                ctx.set_range(range);
            }
        };
    });

    use_context_provider(|| DateRangeInputContext {
        start_date,
        end_date,
        on_format_day_placeholder: props.on_format_day_placeholder,
        on_format_month_placeholder: props.on_format_month_placeholder,
        on_format_year_placeholder: props.on_format_year_placeholder,
    });

    let children = props.children.unwrap_or_else(|| {
        rsx! {
            DateRangePickerStartValue {}
            DatePickerSeparator {
                symbol: '—',
            }
            DateRangePickerEndValue {}
        }
    });

    rsx! {
        {children}
    }
}

/// The editable start date value in a range picker input.
#[component]
pub fn DateRangePickerStartValue(props: DateRangePickerStartValueProps) -> Element {
    let mut ctx = use_context::<DateRangeInputContext>();

    rsx! {
        DateElement {
            selected_date: ctx.start_date,
            on_date_change: move |date| ctx.start_date.set(date),
            on_format_day_placeholder: ctx.on_format_day_placeholder,
            on_format_month_placeholder: ctx.on_format_month_placeholder,
            on_format_year_placeholder: ctx.on_format_year_placeholder,
            children: props.children,
        }
    }
}

/// The editable end date value in a range picker input.
#[component]
pub fn DateRangePickerEndValue(props: DateRangePickerEndValueProps) -> Element {
    let mut ctx = use_context::<DateRangeInputContext>();

    rsx! {
        DateElement {
            start_index: 3,
            selected_date: ctx.end_date,
            on_date_change: move |date| ctx.end_date.set(date),
            on_format_day_placeholder: ctx.on_format_day_placeholder,
            on_format_month_placeholder: ctx.on_format_month_placeholder,
            on_format_year_placeholder: ctx.on_format_year_placeholder,
            children: props.children,
        }
    }
}

/// The props for the [`DatePickerInput`] component
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerInputProps {
    /// Callback when display day placeholder
    #[props(default = Callback::new(|_| "D".to_string()))]
    pub on_format_day_placeholder: Callback<(), String>,

    /// Callback when display month placeholder
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_month_placeholder: Callback<(), String>,

    /// Callback when display year placeholder
    #[props(default = Callback::new(|_| "Y".to_string()))]
    pub on_format_year_placeholder: Callback<(), String>,

    /// Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the date picker element
    #[props(default)]
    pub children: Option<Element>,
}

/// # DatePickerInput
///
/// The input element for the [`DatePicker`] component which allow users to enter a date value.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{calendar::Calendar, date_picker::*, popover::*, ContentAlign};
/// use time::Date;
/// #[component]
/// fn Demo() -> Element {
///    let mut selected_date = use_signal(|| None::<Date>);
///    rsx! {
///        div {
///            DatePicker {
///                selected_date: selected_date(),
///                on_value_change: move |date| {
///                    tracing::info!("Date changed to: {date:?}");
///                    selected_date.set(date);
///               },
///                DatePickerPopover {
///                    DatePickerInput {
///                        PopoverTrigger {
///                            "Select date"
///                        }
///                        PopoverContent {
///                            align: ContentAlign::End,
///                            DatePickerCalendar {
///                                calendar: Calendar,
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
#[component]
pub fn DatePickerInput(props: DatePickerInputProps) -> Element {
    let children = props.children.unwrap_or_else(|| {
        rsx! {
            DatePickerInputValue {
                on_format_day_placeholder: props.on_format_day_placeholder,
                on_format_month_placeholder: props.on_format_month_placeholder,
                on_format_year_placeholder: props.on_format_year_placeholder,
            }
        }
    });

    rsx! {
        div { ..props.attributes,
            {children}
        }
    }
}

/// # DateRangePickerInput
///
/// The input element for the [`DateRangePicker`] component which allow users to enter a date range.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{calendar::{DateRange, RangeCalendar}, date_picker::*, popover::*, ContentAlign};
/// #[component]
/// fn Demo() -> Element {
///    let mut selected_range = use_signal(|| None::<DateRange>);
///    rsx! {
///        div {
///            DateRangePicker {
///                selected_range: selected_range(),
///                on_range_change: move |range| {
///                    tracing::info!("Selected range: {:?}", range);
///                    selected_range.set(range);
///               },
///                DatePickerPopover {
///                    DateRangePickerInput {
///                        PopoverTrigger {
///                            "Select date"
///                        }
///                        PopoverContent {
///                            align: ContentAlign::End,
///                            DateRangePickerCalendar {
///                                calendar: RangeCalendar,
///                            }
///                        }
///                    }
///                }
///            }
///        }
///    }
///}
/// ```
#[component]
pub fn DateRangePickerInput(props: DatePickerInputProps) -> Element {
    let children = props.children.unwrap_or_else(|| {
        rsx! {
            DateRangePickerInputValue {
                on_format_day_placeholder: props.on_format_day_placeholder,
                on_format_month_placeholder: props.on_format_month_placeholder,
                on_format_year_placeholder: props.on_format_year_placeholder,
            }
        }
    });

    rsx! {
        div { ..props.attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[component]
    fn ControlledDatePicker() -> Element {
        rsx! {
            DatePicker {
                selected_date: Some(date!(2026 - 05 - 07)),
                DatePickerInput {}
            }
        }
    }

    #[component]
    fn ControlledDateRangePicker() -> Element {
        rsx! {
            DateRangePicker {
                selected_range: Some(DateRange::new(date!(2026 - 05 - 07), date!(2026 - 05 - 11))),
                DateRangePickerInput {}
            }
        }
    }

    #[test]
    fn date_picker_input_renders_controlled_date_on_first_render() {
        let mut dom = VirtualDom::new(ControlledDatePicker);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("2026"));
        assert!(html.contains("05"));
        assert!(html.contains("07"));
        assert!(!html.contains("YYYY"));
        assert!(!html.contains("MM"));
        assert!(!html.contains("DD"));
    }

    #[test]
    fn date_range_picker_input_renders_controlled_range_on_first_render() {
        let mut dom = VirtualDom::new(ControlledDateRangePicker);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("2026"));
        assert!(html.contains("05"));
        assert!(html.contains("07"));
        assert!(html.contains("11"));
        assert!(!html.contains("YYYY"));
        assert!(!html.contains("MM"));
        assert!(!html.contains("DD"));
    }
}
