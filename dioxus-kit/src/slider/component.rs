use dioxus::prelude::*;
use dioxus_kit_core::slider::{self, RangeSliderProps, SliderProps};

#[css_module("/src/slider/style.css")]
struct Styles;

#[component]
pub fn Slider(props: SliderProps) -> Element {
    rsx! {
        slider::Slider {
            class: Styles::dx_slider,
            value: props.value,
            default_value: props.default_value,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            on_value_change: props.on_value_change,
            label: props.label,
            attributes: props.attributes,
            slider::SliderTrack { class: Styles::dx_slider_track,
                slider::SliderRange { class: Styles::dx_slider_range }
                slider::SliderThumb { class: Styles::dx_slider_thumb }
            }
        }
    }
}

#[component]
pub fn RangeSlider(props: RangeSliderProps) -> Element {
    rsx! {
        slider::RangeSlider {
            class: Styles::dx_slider,
            value: props.value,
            default_value: props.default_value,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            on_value_change: props.on_value_change,
            label: props.label,
            attributes: props.attributes,
            slider::SliderTrack { class: Styles::dx_slider_track,
                slider::SliderRange { class: Styles::dx_slider_range }
                slider::SliderThumb { class: Styles::dx_slider_thumb, index: 0usize }
                slider::SliderThumb { class: Styles::dx_slider_thumb, index: 1usize }
            }
        }
    }
}
