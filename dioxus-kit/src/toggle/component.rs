use dioxus::prelude::*;
use dioxus_kit_core::toggle::{self, ToggleProps};

#[css_module("/src/toggle/style.css")]
struct Styles;

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    rsx! {
        toggle::Toggle {
            class: Styles::dx_toggle,
            pressed: props.pressed,
            default_pressed: props.default_pressed,
            disabled: props.disabled,
            on_pressed_change: props.on_pressed_change,
            onmounted: props.onmounted,
            onfocus: props.onfocus,
            onkeydown: props.onkeydown,
            attributes: props.attributes,
            {props.children}
        }
    }
}
