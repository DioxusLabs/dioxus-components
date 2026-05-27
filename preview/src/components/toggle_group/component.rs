use dioxus::prelude::*;
use dioxus_primitives::toggle_group::{self, ToggleGroupProps, ToggleItemProps};

#[css_module("/src/components/toggle_group/style.css")]
struct Styles;

#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    rsx! {
        toggle_group::ToggleGroup {
            class: Styles::dx_toggle_group,
            default_pressed: props.default_pressed,
            pressed: props.pressed,
            on_pressed_change: props.on_pressed_change,
            disabled: props.disabled,
            allow_multiple_pressed: props.allow_multiple_pressed,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ToggleItem(props: ToggleItemProps) -> Element {
    rsx! {
        toggle_group::ToggleItem {
            class: Styles::dx_toggle_item,
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}
