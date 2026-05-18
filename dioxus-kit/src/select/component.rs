use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronDown};
use dioxus_kit_core::select::{
    self, SelectGroupLabelProps, SelectMultiProps, SelectOptionProps, SelectProps,
};
use dioxus_kit_core::{dioxus_attributes::attributes, merge_attributes};

pub use dioxus_kit_core::select::SelectGroup;

#[css_module("/src/select/style.css")]
#[allow(dead_code)]
struct Styles;

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let base = attributes!(div { class: "dx-select" });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::Select {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            select::SelectTrigger {
                class: "dx-select-trigger",
                select::SelectValue {}
                ChevronDown {
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList {
                class: "dx-select-list",
                {props.children}
            }
        }
    }
}

#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element {
    let base = attributes!(div { class: "dx-select" });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectMulti {
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            select::SelectTrigger {
                class: "dx-select-trigger",
                select::SelectValue {}
                ChevronDown {
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList {
                class: "dx-select-list",
                {props.children}
            }
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let base = attributes!(div {
        class: "dx-select-group-label"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectGroupLabel {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    let base = attributes!(div {
        class: "dx-select-option"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            select::SelectItemIndicator {
                Check {
                    size: "1rem",
                    stroke: "var(--secondary-color-5)",
                }
            }
        }
    }
}
