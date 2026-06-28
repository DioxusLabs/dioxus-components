use dioxus::prelude::*;
use dioxus_primitives::otp::{
    self, OneTimePasswordGroupProps, OneTimePasswordInputProps, OneTimePasswordSeparatorProps,
    OneTimePasswordSlotProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/otp/style.css")]
struct Styles;

#[component]
pub fn OneTimePasswordInput(props: OneTimePasswordInputProps) -> Element {
    let base = attributes!(div { class: Styles::dx_otp });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        otp::OneTimePasswordInput {
            value: props.value,
            default_value: props.default_value,
            maxlength: props.maxlength,
            inputmode: props.inputmode,
            autocomplete: props.autocomplete,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            id: props.id,
            aria_label: props.aria_label,
            aria_labelledby: props.aria_labelledby,
            validate: props.validate,
            on_value_change: props.on_value_change,
            on_complete: props.on_complete,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn OneTimePasswordGroup(props: OneTimePasswordGroupProps) -> Element {
    let base = attributes!(div { class: Styles::dx_otp_group });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        otp::OneTimePasswordGroup {
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn OneTimePasswordSlot(props: OneTimePasswordSlotProps) -> Element {
    let base = attributes!(div { class: Styles::dx_otp_slot });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        otp::OneTimePasswordSlot {
            index: props.index,
            attributes: merged,
            span { class: Styles::dx_otp_caret, aria_hidden: "true" }
            {props.children}
        }
    }
}

#[component]
pub fn OneTimePasswordSeparator(props: OneTimePasswordSeparatorProps) -> Element {
    let base = attributes!(div { class: Styles::dx_otp_separator });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        otp::OneTimePasswordSeparator {
            attributes: merged,
            svg {
                width: "10",
                height: "10",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                line { x1: "5", y1: "12", x2: "19", y2: "12" }
            }
            {props.children}
        }
    }
}
